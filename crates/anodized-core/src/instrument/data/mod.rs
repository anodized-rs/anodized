#[cfg(test)]
mod tests;

use proc_macro2::Span;
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, GenericParam, Generics, Ident,
    ImplItem, ImplItemFn, ItemImpl, ReturnType, Signature, Type, parse_quote,
    punctuated::Punctuated,
};

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

        let spec_maintains_fn = ImplItemFn {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            sig: Self::build_type_spec_fn_sig("__anodized_data_maintains"),
            block: body,
            defaultness: None,
        };

        let attrs: [Attribute; 2] = [
            parse_quote!(#[doc(hidden)]),
            parse_quote!(#[allow(warnings)]),
        ];

        let generic_args = Self::build_generic_args_from_params(generics);
        let self_type: Type = parse_quote!(#ident #generic_args);
        ItemImpl {
            attrs: attrs.to_vec(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: generics.clone(),
            trait_: None,
            self_ty: Box::new(self_type),
            brace_token: Default::default(),
            items: vec![ImplItem::Fn(spec_maintains_fn)],
        }
    }

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
