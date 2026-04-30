use proc_macro2::TokenStream;
use syn::{ItemEnum, ItemStruct, Result, parse_quote};

use crate::{DataSpec, instrument::Backend};

#[cfg(test)]
mod tests;

impl Backend {
    pub fn instrument_item_struct(
        &self,
        spec: DataSpec,
        item_struct: ItemStruct,
    ) -> Result<TokenStream> {
        let ident = &item_struct.ident;
        let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
        let statements = Self::build_precondition_fn_body(&spec.maintains).stmts;

        Ok(parse_quote! {
            #item_struct

            #[doc(hidden)]
            #[allow(warnings)]
            impl #impl_generics #ident #ty_generics #where_clause {
                fn __anodized_data_maintains(&self) {
                    #(#statements)*
                }
            }
        })
    }

    pub fn instrument_item_enum(&self, spec: DataSpec, item_enum: ItemEnum) -> Result<TokenStream> {
        let ident = &item_enum.ident;
        let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
        let statements = Self::build_precondition_fn_body(&spec.maintains).stmts;

        Ok(parse_quote! {
            #item_enum

            #[doc(hidden)]
            #[allow(warnings)]
            impl #impl_generics #ident #ty_generics #where_clause {
                fn __anodized_data_maintains(&self) {
                    // Bring all variants into scope for convenience.
                    use #ident::*;
                    #(#statements)*
                }
            }
        })
    }
}
