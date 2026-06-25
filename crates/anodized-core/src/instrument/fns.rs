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
    Capture, PostCondition, PreCondition, Spec,
    instrument::{Config, build_assert, build_eprint},
    qualifiers::FnQualifiers,
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

    pub fn build_precondition_fn_sig(prefix: &str, sig: &Signature) -> Signature {
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
            output: parse_quote!(-> bool),
        }
    }

    pub fn build_postcondition_fn_sig(prefix: &str, sig: &Signature) -> Signature {
        let mut inputs = sig.inputs.clone();
        let output_binder = match &sig.output {
            ReturnType::Type(_, return_type) => parse_quote!(__anodized_output: &#return_type),
            ReturnType::Default => parse_quote!(__anodized_output: &()),
        };
        inputs.push(output_binder);

        Signature {
            constness: sig.constness,
            asyncness: sig.asyncness,
            unsafety: sig.unsafety,
            abi: sig.abi.clone(),
            fn_token: sig.fn_token,
            ident: syn::Ident::new(&format!("{prefix}_{}", sig.ident), sig.ident.span()),
            generics: sig.generics.clone(),
            paren_token: sig.paren_token,
            inputs,
            variadic: sig.variadic.clone(),
            output: parse_quote!(-> bool),
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

    pub fn build_precondition_fn_body(
        requires: &[PreCondition],
        maintains: &[PreCondition],
    ) -> Block {
        let mut statements: Vec<Stmt> = vec![];
        let mut clauses: Vec<Expr> = vec![];

        for condition in requires.iter().chain(maintains) {
            let i = clauses.len();
            let name = Ident::new(&format!("__anodized_clause_{}", i + 1), Span::mixed_site());
            let closure = &condition.closure;
            statements.push(parse_quote! { let #name = (#closure)(); });
            clauses.push(parse_quote! { #name });
        }

        if clauses.is_empty() {
            clauses.push(parse_quote!(true));
        }

        parse_quote! {
            {
                #(#statements)*
                #(#clauses)&&*
            }
        }
    }

    pub fn build_postcondition_fn_body(
        maintains: &[PreCondition],
        captures: &[Capture],
        ensures: &[PostCondition],
        return_type: &ReturnType,
    ) -> Result<Block> {
        let mut statements: Vec<Stmt> = vec![];
        let mut clauses: Vec<Expr> = vec![];

        for condition in maintains {
            let i = clauses.len();
            let name = Ident::new(&format!("__anodized_clause_{}", i + 1), Span::mixed_site());
            let closure = &condition.closure;
            statements.push(parse_quote! { let #name = (#closure)(); });
            clauses.push(parse_quote! { #name });
        }

        {
            let aliases = captures.iter().map(|capture| &capture.pat);
            let capture_exprs = captures.iter().map(|capture| -> Expr {
                let expr = &capture.expr;
                // Wrap in closure to guard against `return`.
                parse_quote! { (|| #expr)() }
            });
            statements.push(parse_quote! { let (#(#aliases),*) = (#(#capture_exprs),*); });
        }

        let output_type = match return_type {
            ReturnType::Type(_, return_type) => return_type.as_ref().clone(),
            ReturnType::Default => parse_quote!(()),
        };
        for condition in ensures {
            let i = clauses.len();
            let name = Ident::new(&format!("__anodized_clause_{}", i + 1), Span::mixed_site());
            let closure = &condition.closure;
            let input = closure.inputs.first().expect("valid postcondition");
            if let Pat::Type(_) = input {
                statements.push(parse_quote! { let #name = (#closure)(__anodized_output); });
            } else {
                let body = &closure.body;
                statements.push(parse_quote! {
                    let #name = (| #input: &#output_type | -> bool { #body })(__anodized_output);
                });
            }
            clauses.push(parse_quote! { #name });
        }

        if clauses.is_empty() {
            clauses.push(parse_quote!(true));
        }

        Ok(parse_quote! {
            {
                #(#statements)*
                #(#clauses)&&*
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
        let build_check = match (self.emit_print, self.emit_panic) {
            (true, true) => build_assert,
            (true, false) => build_eprint,
            (false, true) => build_assert,
            (false, false) => return Ok(original_body.clone()),
        };

        // The identifier for the return value binding.
        let output_ident = Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new("__anodized_output", Span::mixed_site()),
            subpat: None,
        });

        // --- Generate Precondition Checks ---
        let precondition_checks = spec
            .requires
            .iter()
            .map(|condition| {
                let closure = condition.closure.to_token_stream();
                let expr = quote! { (#closure)() };
                let repr = condition.closure.body.to_token_stream();
                build_check(
                    condition.cfg.as_ref(),
                    &expr,
                    "Precondition failed: {}",
                    &repr,
                )
            })
            .chain(spec.maintains.iter().map(|condition| {
                let closure = condition.closure.to_token_stream();
                let expr = quote! { (#closure)() };
                let repr = condition.closure.body.to_token_stream();
                build_check(
                    condition.cfg.as_ref(),
                    &expr,
                    "Pre-invariant failed: {}",
                    &repr,
                )
            }));

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

        // --- Generate Postcondition Checks ---
        let postcondition_checks = spec
            .maintains
            .iter()
            .map(|condition| {
                let closure = condition.closure.to_token_stream();
                let expr = quote! { (#closure)() };
                let repr = condition.closure.body.to_token_stream();
                build_check(
                    condition.cfg.as_ref(),
                    &expr,
                    "Post-invariant failed: {}",
                    &repr,
                )
            })
            .chain(spec.ensures.iter().map(|postcondition| {
                let closure = annotate_postcondition_closure_argument(
                    postcondition.closure.clone(),
                    return_type.clone(),
                );

                let expr = quote! { (#closure)(&#output_ident) };
                let inputs = &postcondition.closure.inputs;
                let body = &postcondition.closure.body;
                // Omit the closure's return type for brevity.
                let repr = quote! { |#inputs| #body };
                build_check(
                    postcondition.cfg.as_ref(),
                    &expr,
                    "Postcondition failed: {}",
                    &repr,
                )
            }));

        Ok(parse_quote! {
            {
                #(#precondition_checks)*
                #body_and_captures
                #(#postcondition_checks)*
                #output_ident
            }
        })
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
