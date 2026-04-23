#[cfg(test)]
mod tests;

use crate::{
    Capture, PostCondition, PreCondition, Spec,
    instrument::{Config, build_assert, build_eprint},
};

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Block, Expr, Ident, Pat, PatIdent, ReturnType, Signature, Stmt, parse::Result, parse_quote,
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
                match &return_type {
                    ReturnType::Default => {
                        parse_quote! { let _ = |#output_binder: &()| #body; }
                    }
                    ReturnType::Type(_, ty) => {
                        parse_quote! { let _ = |#output_binder: &#ty| #body; }
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
                build_check(
                    postcondition.cfg.as_ref(),
                    &expr,
                    "Postcondition failed: {}",
                    &postcondition.closure.to_token_stream(),
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
