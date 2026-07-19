#[cfg(test)]
#[path = "fns_tests.rs"]
mod fns_tests;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Block, Expr, Ident, Meta, Pat, PatIdent, Path, ReturnType, Signature, Stmt, Type,
    parse::{Parse, Result},
    parse_quote,
};

use crate::{
    Capture, PostCondition, PreCondition, Spec,
    instrument::{CheckSettings, Mode},
    qualifiers::FnQualifiers,
};

impl Mode {
    pub fn instrument_fn(&self, spec: &Spec, sig: &Signature, body: &mut Block) -> syn::Result<()> {
        self.instrument_loops_in_fn_body(body)?;

        let Mode::InjectChecks(check_config) = self else {
            return Ok(());
        };

        let is_async = sig.asyncness.is_some();

        // Extract the return type from the function signature
        let return_type = match &sig.output {
            syn::ReturnType::Default => syn::parse_quote!(()),
            syn::ReturnType::Type(_, ty) => ty.as_ref().clone(),
        };

        // Generate the new, instrumented function body.
        let new_body = check_config.instrument_fn_body(spec, body, is_async, &return_type)?;

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
}

impl CheckSettings {
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
            let expr = parse_quote! { __anodized_eval_pre(#closure) };
            let repr = condition.closure.body.to_token_stream().to_string();
            let clause = self.build_clause_eval(condition.cfg.as_ref(), &expr, &repr);
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
            let expr = parse_quote! { __anodized_eval_inv(#closure) };
            let repr = condition.closure.body.to_token_stream().to_string();
            let clause = self.build_clause_eval(condition.cfg.as_ref(), &expr, &repr);
            postcondition_clauses.push(clause);
        }
        for postcondition in &spec.ensures {
            let closure = &postcondition.closure;
            let input = &closure.inputs.first().unwrap();
            let output = &closure.output;
            let body = &closure.body;
            let closure_expr = match input {
                Pat::Type(_) => closure.clone(),
                _ => parse_quote! {
                    // If the closure's input doesn't have a type ascription, add one.
                    |#input: &#return_type| #output { #body }
                },
            };
            let expr = parse_quote! { __anodized_eval_post(#closure_expr, &#output_ident) };
            // Omit the closure's return type for brevity.
            let repr = quote! { |#input| #body }.to_string();
            let clause = self.build_clause_eval(postcondition.cfg.as_ref(), &expr, &repr);
            postcondition_clauses.push(clause);
        }
        if postcondition_clauses.is_empty() {
            postcondition_clauses.push(parse_quote!(true));
        }

        let do_run_checks = self.does_print || self.does_panic.is_some();

        let (output_expr, precond_fail_action, postcond_fail_action) =
            if let Some(ref panic_settings) = self.does_panic
                && panic_settings.has_try_fn
            {
                (
                    quote! { Ok(#output_ident) },
                    Some(parse_quote! {
                        return ::anodized::result::err_pre(__anodized_errors);
                    }),
                    Some(parse_quote! {
                        return ::anodized::result::err_post(#output_ident, __anodized_errors);
                    }),
                )
            } else {
                (
                    quote! { #output_ident },
                    self.build_fail_action("precondition failed"),
                    self.build_fail_action("postcondition failed"),
                )
            };

        Ok(parse_quote! {
            {
                if #do_run_checks {
                    fn __anodized_eval_pre(c: impl Fn() -> bool) -> bool { c() }
                    let mut __anodized_errors = ::std::string::String::new();
                    let __anodized_precond = #(#precondition_clauses)&*;
                    if !__anodized_precond {
                        #precond_fail_action
                    }
                }
                #body_and_captures
                if #do_run_checks {
                    fn __anodized_eval_inv(c: impl Fn() -> bool) -> bool { c() }
                    fn __anodized_eval_post<R>(c: impl Fn(&R) -> bool, r: &R) -> bool { c(r) }
                    let mut __anodized_errors = ::std::string::String::new();
                    let __anodized_postcond = #(#postcondition_clauses)&*;
                    if !__anodized_postcond {
                        #postcond_fail_action
                    }
                }
                #output_expr
            }
        })
    }

    fn build_clause_eval(&self, cfg: Option<&Meta>, expr: &Expr, repr: &str) -> Expr {
        if self.does_print {
            let br_and_repr = format!("\n    {repr}");
            let cfg_guard = match cfg {
                Some(meta) => quote! { !cfg!(#meta) || },
                None => quote!(),
            };
            parse_quote! { ( #cfg_guard #expr || __anodized_errors.push_str(#br_and_repr) != () ) }
        } else {
            expr.clone()
        }
    }

    fn build_fail_action(&self, message: &str) -> Option<Stmt> {
        let message_and_errors = format!("{message}:{{__anodized_errors}}");
        match (self.does_print, self.does_panic.is_some()) {
            (true, true) => Some(parse_quote! { panic!(#message_and_errors); }),
            (true, false) => Some(parse_quote! { eprintln!(#message_and_errors); }),
            (false, true) => Some(parse_quote! { panic!(#message); }),
            (false, false) => None,
        }
    }
}

pub(crate) fn make_try_fn_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("__anodized_fn_try_{ident}"), ident.span())
}

pub fn make_try_call(mut expr: Expr) -> Result<Expr> {
    match &mut expr {
        Expr::Call(fn_call) => {
            if let Expr::Path(path) = fn_call.func.as_mut()
                && (path.qself.is_some() || path.path.segments.len() > 1)
            {
                let last_segment = path.path.segments.last_mut().expect("last segment");
                last_segment.ident = make_try_fn_ident(&last_segment.ident);
                return Ok(expr);
            }
        }
        Expr::MethodCall(method_call) => {
            method_call.method = make_try_fn_ident(&method_call.method);
            return Ok(expr);
        }
        _ => {}
    }

    Err(syn::Error::new_spanned(
        expr,
        "must be a method call or a qualified function call",
    ))
}
