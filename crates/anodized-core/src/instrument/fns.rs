#[cfg(test)]
#[path = "fns_tests.rs"]
mod fns_tests;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Block, Expr, Ident, Pat, PatIdent, Path, ReturnType, Signature, Stmt, Type,
    parse::{Parse, Result},
    parse_quote,
};

use crate::{
    Capture, PostCondition, PreCondition, Spec, instrument::Config, qualifiers::FnQualifiers,
};

impl Config {
    pub fn instrument_fn(&self, spec: &Spec, sig: &Signature, body: &mut Block) -> syn::Result<()> {
        self.instrument_loops_in_fn_body(body)?;

        let is_async = sig.asyncness.is_some();

        // Extract the return type from the function signature
        let return_type = match &sig.output {
            syn::ReturnType::Default => syn::parse_quote!(()),
            syn::ReturnType::Type(_, ty) => ty.as_ref().clone(),
        };

        // Generate the new, instrumented function body.
        let new_body = self.instrument_fn_body(spec, body, is_async, &return_type)?;

        // Replace the old function body with the new one.
        *body = new_body;

        Ok(())
    }

    pub fn build_spec_fn_sig(prefix: &str, sig: &Signature) -> Signature {
        Signature {
            constness: sig.constness,
            asyncness: sig.asyncness,
            unsafety: sig.unsafety,
            abi: sig.abi.clone(),
            fn_token: sig.fn_token,
            ident: syn::Ident::new(&format!("{prefix}_{}", sig.ident), sig.ident.span()),
            generics: sig.generics.clone(),
            paren_token: sig.paren_token,
            inputs: sig.inputs.clone(),
            variadic: sig.variadic.clone(),
            output: syn::ReturnType::Default,
        }
    }

    pub fn build_qualifier_const_item<SomeConstItem: Parse>(
        attrs: &[Attribute],
        prefix: &str,
        qualifiers: FnQualifiers,
        fn_ident: &Ident,
    ) -> SomeConstItem {
        let qualifier_bits = qualifiers.bits();
        let name: Ident = syn::Ident::new(&format!("{}_{}", prefix, fn_ident), fn_ident.span());
        parse_quote! {
            #(#attrs)*
            const #name: u32 = #qualifier_bits;
        }
    }

    pub fn build_qualifier_check_stmt(
        fn_ident: &Ident,
        impl_type: &Type,
        trait_path: &Path,
    ) -> Stmt {
        let impl_const_name = Ident::new(
            &format!("__anodized_fn_qualifiers_{}", fn_ident),
            fn_ident.span(),
        );

        let trait_const_name = Ident::new(
            &format!("__anodized_fn_qualifiers_trait_{}", fn_ident),
            fn_ident.span(),
        );

        let message = format!(
            "the qualifiers on the impl `{}::{fn_ident}` cannot be weaker than the qualifiers on the trait `{}::{fn_ident}`",
            impl_type.to_token_stream(),
            trait_path.to_token_stream(),
        );

        parse_quote! {
            const {
                assert!(
                    Self::#impl_const_name == Self::#trait_const_name | Self::#impl_const_name,
                    #message,
                );
            };
        }
    }

    pub fn build_precondition_fn_body(conditions: &[PreCondition]) -> Block {
        let statements = conditions.iter().map(|condition| -> Stmt {
            let closure = &condition.closure;
            parse_quote! { let _ = #closure; }
        });
        parse_quote! {
            {
                #(#statements)*
            }
        }
    }

    pub fn build_poscondition_fn_body(
        captures: &[Capture],
        conditions: &[PostCondition],
        return_type: &ReturnType,
    ) -> Result<Block> {
        let aliases = captures.iter().map(|capture| &capture.pat);
        let capture_exprs = captures.iter().map(|capture| -> Expr {
            let expr = &capture.expr;
            // Wrap in closure to guard against `return`.
            parse_quote! { (|| #expr)() }
        });

        let mut statements = vec![];

        for condition in conditions {
            let closure = &condition.closure;
            // TODO: This sort of validation should happen during parsing.
            let output_binder = match closure.inputs.first() {
                Some(output_binder) if closure.inputs.len() == 1 => output_binder,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &closure.inputs,
                        "Postcondition closure must have exactly one parameter.",
                    ));
                }
            };
            let statement: Stmt = if let Pat::Type(_) = output_binder {
                // If the output binder has a type annotation, use as-is.
                parse_quote! { let _ = #closure; }
            } else {
                // Otherwise add a type annotation.
                let body = &closure.body;
                let output = &closure.output;
                match &return_type {
                    ReturnType::Default => {
                        parse_quote! { let _ = |#output_binder: &()| #output { #body }; }
                    }
                    ReturnType::Type(_, ty) => {
                        parse_quote! { let _ = |#output_binder: &#ty| #output { #body }; }
                    }
                }
            };
            statements.push(statement);
        }

        Ok(parse_quote! {
            {
                let (#(#aliases),*) = (#(#capture_exprs),*);
                #(#statements)*
            }
        })
    }

    fn instrument_fn_body(
        &self,
        spec: &Spec,
        original_body: &Block,
        is_async: bool,
        return_type: &syn::Type,
    ) -> Result<Block> {
        // The identifier for the return value binding.
        let output_ident = Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new("__anodized_output", Span::mixed_site()),
            subpat: None,
        });

        // --- Generate Precondition Clauses ---
        let mut precondition_clauses: Vec<Expr> = vec![];
        for condition in spec.requires.iter().chain(&spec.maintains) {
            let closure = &condition.closure;
            let expr = parse_quote! { (#closure)() };
            let repr = condition.closure.body.to_token_stream().to_string();
            let clause =
                self.build_clause_eval(&expr, &format!("Precondition clause failed: {repr}"));
            precondition_clauses.push(clause);
        }
        if precondition_clauses.is_empty() {
            precondition_clauses.push(parse_quote!(true));
        }

        // --- Generate Combined Body and Capture Statement ---
        // Capture values and execute body in a single tuple assignment
        // This ensures captured values aren't accessible to the body itself

        // Chain capture aliases with output binding
        let aliases = spec
            .captures
            .iter()
            .map(|cb| &cb.pat)
            .chain(std::iter::once(&output_ident));

        // Chain capture expressions with body expression
        let capture_exprs = spec.captures.iter().map(|cb| {
            let expr = &cb.expr;
            // Evaluate expression in a closure to prevent early return.
            quote! { (|| #expr)() }
        });

        // Chain underscore types with return type for tuple type annotation
        let types = spec
            .captures
            .iter()
            .map(|_| quote! { _ })
            .chain(std::iter::once(quote! { #return_type }));

        let body_expr = if is_async {
            quote! { (async || #original_body)().await }
        } else {
            quote! { (|| #original_body)() }
        };

        let exprs = capture_exprs.chain(std::iter::once(body_expr));

        // Build tuple assignment with type annotation on the tuple
        let body_and_captures = quote! {
            let (#(#aliases),*): (#(#types),*) = (#(#exprs),*);
        };

        // --- Generate Postcondition Clauses ---
        let mut postcondition_clauses: Vec<Expr> = vec![];
        for condition in &spec.maintains {
            let closure = condition.closure.to_token_stream();
            let expr = parse_quote! { (#closure)() };
            let repr = condition.closure.body.to_token_stream().to_string();
            let clause = self.build_clause_eval(&expr, &format!("Postcondition failed: {repr}"));
            postcondition_clauses.push(clause);
        }
        for postcondition in &spec.ensures {
            let closure = annotate_postcondition_closure_argument(
                postcondition.closure.clone(),
                return_type.clone(),
            );
            let expr = parse_quote! { (#closure)(&#output_ident) };
            let inputs = &postcondition.closure.inputs;
            let body = &postcondition.closure.body;
            // Omit the closure's return type for brevity.
            let repr = quote! { |#inputs| #body }.to_string();
            let clause = self.build_clause_eval(&expr, &format!("Postcondition failed: {repr}"));
            postcondition_clauses.push(clause);
        }
        if postcondition_clauses.is_empty() {
            postcondition_clauses.push(parse_quote!(true));
        }

        let do_run_checks = self.emit_print || self.emit_panic;
        let precond_check =
            self.build_condition_check(parse_quote!(__anodized_precond), "Precondition failed");
        let postcond_check =
            self.build_condition_check(parse_quote!(__anodized_postcond), "Postcondition failed");

        Ok(parse_quote! {
            {
                if #do_run_checks {
                    let __anodized_precond = #(#precondition_clauses)&*;
                    #precond_check
                }
                #body_and_captures
                if #do_run_checks {
                    let __anodized_postcond = #(#postcondition_clauses)&*;
                    #postcond_check
                }
                #output_ident
            }
        })
    }

    fn build_clause_eval(&self, expr: &Expr, message: &str) -> Expr {
        if self.emit_print {
            parse_quote!(if #expr { true } else { eprintln!("{}", #message); false })
        } else {
            expr.clone()
        }
    }

    fn build_condition_check(&self, ident: Ident, message: &str) -> Option<Expr> {
        if self.emit_panic {
            Some(parse_quote!(if !#ident { panic!("{}", #message); }))
        } else {
            None
        }
    }
}

fn annotate_postcondition_closure_argument(
    mut closure: syn::ExprClosure,
    return_type: syn::Type,
) -> syn::ExprClosure {
    // Add type annotation: convert |param| to |param: &ReturnType|.
    if let Some(first_input) = closure.inputs.first_mut() {
        // Wrap the pattern with a type annotation
        let pattern = first_input.clone();
        *first_input = syn::Pat::Type(syn::PatType {
            attrs: vec![],
            pat: Box::new(pattern),
            colon_token: Default::default(),
            ty: Box::new(syn::Type::Reference(syn::TypeReference {
                and_token: Default::default(),
                lifetime: None,
                mutability: None,
                elem: Box::new(return_type),
            })),
        });
    }
    closure
}
