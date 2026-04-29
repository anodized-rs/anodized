#[cfg(test)]
mod tests;

use proc_macro2::Span;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, GenericParam, Generics, Ident, ReturnType,
    Signature, parse_quote, punctuated::Punctuated,
};

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

    pub fn build_generic_args_from_params(
        params: &Generics,
    ) -> Option<AngleBracketedGenericArguments> {
        let mut args = Punctuated::new();
        for pair in params.params.pairs() {
            let arg = Self::build_generic_arg_from_param(pair.value());
            args.push_value(arg);
            if let Some(punct) = pair.punct() {
                args.push_punct(**punct);
            }
        }
        Some(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: params.lt_token?,
            args,
            gt_token: params.gt_token?,
        })
    }

    pub fn build_generic_arg_from_param(param: &GenericParam) -> GenericArgument {
        match param {
            GenericParam::Lifetime(lifetime_param) => {
                GenericArgument::Lifetime(lifetime_param.lifetime.clone())
            }
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                GenericArgument::Type(parse_quote!(#ident))
            }
            GenericParam::Const(const_param) => {
                let ident = &const_param.ident;
                GenericArgument::Const(parse_quote!(#ident))
            }
        }
    }
}
