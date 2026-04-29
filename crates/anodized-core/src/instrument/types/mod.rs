#[cfg(test)]
mod tests;

use proc_macro2::Span;
use syn::{Ident, ReturnType, Signature, parse_quote};

use crate::instrument::Backend;

impl Backend {
    pub fn build_type_spec_fn_sig(name: &str) -> Signature {
        Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new(name, Span::mixed_site()),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: parse_quote!(&self),
            variadic: None,
            output: ReturnType::Default,
        }
    }
}
