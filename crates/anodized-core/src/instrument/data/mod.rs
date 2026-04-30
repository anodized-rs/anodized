#[cfg(test)]
mod tests;

use syn::{Generics, Ident, ItemImpl, parse_quote};

use crate::{DataSpec, instrument::Backend};

impl Backend {
    pub fn instrument_data_type(
        spec: DataSpec,
        ident: &Ident,
        generics: &Generics,
        is_enum: bool,
    ) -> ItemImpl {
        let mut body = Self::build_precondition_fn_body(&spec.maintains);
        if is_enum {
            // Bring all variants into scope for convenience.
            body.stmts.insert(0, parse_quote!(use #ident::*;));
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        parse_quote! {
            #[doc(hidden)]
            #[allow(warnings)]
            impl #impl_generics #ident #ty_generics #where_clause {
                fn __anodized_data_maintains(&self) #body
            }
        }
    }
}
