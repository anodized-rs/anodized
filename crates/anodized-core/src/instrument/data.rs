#[cfg(test)]
#[path = "data_tests.rs"]
mod data_tests;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Arm, Expr, Fields, Ident, ItemEnum, ItemImpl, ItemStruct, Result, Stmt, Variant, parse_quote,
};

use crate::{
    Condition, DataSpec,
    instrument::{Mode, build_cond_eval},
};

impl Mode {
    pub fn instrument_item_struct(
        &self,
        spec: DataSpec,
        item_struct: ItemStruct,
    ) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        item_struct.to_tokens(&mut tokens);

        if self.changes_anything() {
            let inductive_predicate = Self::build_enum_inductive_predicate(
                std::iter::once((&item_struct.ident, &item_struct.fields)),
                &spec.field_spec_flags,
            );
            let statements = Self::build_refinement_stmts(&inductive_predicate, &spec.maintains);

            let ident = &item_struct.ident;
            let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

            let spec_impl: ItemImpl = parse_quote! {
                #[doc(hidden)]
                #[allow(warnings)]
                impl #impl_generics ::anodized::logic::Spec for #ident #ty_generics #where_clause {
                    fn predicate(&self) -> bool {
                        #(#statements)*
                    }
                }
            };
            spec_impl.to_tokens(&mut tokens);
        }

        Ok(tokens)
    }

    pub fn instrument_item_enum(&self, spec: DataSpec, item_enum: ItemEnum) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        item_enum.to_tokens(&mut tokens);

        if let Mode::EmbedSpecs(_) = self {
            let inductive_predicate = Self::build_enum_inductive_predicate(
                item_enum
                    .variants
                    .iter()
                    .map(|variant| (&variant.ident, &variant.fields)),
                &spec.field_spec_flags,
            );
            let statements = Self::build_refinement_stmts(&inductive_predicate, &spec.maintains);

            let ident = &item_enum.ident;
            let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();

            let spec_impl: ItemImpl = parse_quote! {
                #[doc(hidden)]
                #[allow(warnings)]
                impl #impl_generics ::anodized::logic::Spec for #ident #ty_generics #where_clause {
                    fn predicate(&self) -> bool {
                        // Bring all variants into scope for convenience.
                        use #ident::*;
                        #(#statements)*
                    }
                }
            };
            spec_impl.to_tokens(&mut tokens);
        }

        Ok(tokens)
    }

    fn build_refinement_stmts(inductive_predicate: &Expr, maintains: &[Condition]) -> Vec<Stmt> {
        let mut statements = vec![parse_quote! {
            let __anodized_inv = #inductive_predicate;
        }];

        for invariant in maintains {
            let eval = build_cond_eval(&invariant.expr);
            let check = parse_quote! {
                let __anodized_inv = __anodized_inv & #eval;
            };
            statements.push(check);
        }

        statements
    }

    fn build_enum_inductive_predicate<'a>(
        variants: impl Iterator<Item = (&'a Ident, &'a Fields)>,
        field_spec_flags: &[Vec<bool>],
    ) -> Expr {
        let arms = variants
            .zip(field_spec_flags)
            .map(|((ident, fields), field_spec_flags)| {
                Self::build_variant_inductive_predicate(ident, fields, field_spec_flags)
            });

        parse_quote! {
            match self {
                #(#arms),*
            }
        }
    }

    fn build_variant_inductive_predicate(
        ident: &Ident,
        fields: &Fields,
        field_spec_flags: &[bool],
    ) -> Arm {
        let mut predicate_calls = Vec::new();

        let pattern = match fields {
            Fields::Named(fields) => {
                let included_fields = fields
                    .named
                    .iter()
                    .zip(field_spec_flags)
                    .filter_map(|(field, field_spec_flag)| {
                        field_spec_flag.then(|| field.ident.as_ref().unwrap())
                    })
                    .collect::<Vec<_>>();

                for field in &included_fields {
                    predicate_calls.push(quote!(::anodized::logic::Spec::predicate(#field)));
                }

                if included_fields.len() == fields.named.len() {
                    quote!({ #(#included_fields),* })
                } else if included_fields.is_empty() {
                    quote!({ .. })
                } else {
                    quote!({ #(#included_fields),*, .. })
                }
            }
            Fields::Unnamed(fields) => {
                let patterns = fields.unnamed.iter().zip(field_spec_flags).enumerate().map(
                    |(index, (_, field_spec_flag))| {
                        if *field_spec_flag {
                            let ident = Ident::new(
                                &format!("field_{index}"),
                                proc_macro2::Span::call_site(),
                            );
                            predicate_calls
                                .push(quote!(::anodized::logic::Spec::predicate(#ident)));
                            quote!(#ident)
                        } else {
                            quote!(_)
                        }
                    },
                );
                quote!((#(#patterns),*))
            }
            Fields::Unit => quote!(),
        };

        let predicate: TokenStream = match predicate_calls.as_slice() {
            [] => parse_quote!(true),
            [predicate] => syn::parse2(predicate.clone()).unwrap(),
            [first, rest @ ..] => {
                let predicate = rest
                    .iter()
                    .fold(first.clone(), |predicate, next| quote!(#predicate & #next));
                syn::parse2(predicate).unwrap()
            }
        };

        parse_quote! { #ident #pattern => #predicate }
    }
}
