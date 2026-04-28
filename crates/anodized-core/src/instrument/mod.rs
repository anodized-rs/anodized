use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, ItemFn, ItemImpl, ItemTrait, Meta, Result, parse_quote};

use crate::Spec;

pub mod fns;
pub mod traits;

pub struct Backend {
    pub emit_print: bool,
    pub emit_panic: bool,
}

impl Backend {
    pub fn instrument_item_fn(&self, spec: Spec, mut item_fn: ItemFn) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        let attrs: [Attribute; 2] = [
            parse_quote!(#[doc(hidden)]),
            parse_quote!(#[allow(unused_variables)]),
        ];

        // Embed `spec` elements as `__anodized_fn_*` functions.
        let spec_requires_fn = ItemFn {
            attrs: attrs.to_vec(),
            vis: syn::Visibility::Inherited,
            sig: Self::build_spec_fn_sig("__anodized_fn_requires", &item_fn.sig),
            block: Box::new(Self::build_precondition_fn_body(&spec.requires)),
        };
        let spec_maintains_fn = ItemFn {
            attrs: attrs.to_vec(),
            vis: syn::Visibility::Inherited,
            sig: Self::build_spec_fn_sig("__anodized_fn_maintains", &item_fn.sig),
            block: Box::new(Self::build_precondition_fn_body(&spec.maintains)),
        };
        let spec_ensures_fn = ItemFn {
            attrs: attrs.to_vec(),
            vis: syn::Visibility::Inherited,
            sig: Self::build_spec_fn_sig("__anodized_fn_ensures", &item_fn.sig),
            block: Box::new(Self::build_poscondition_fn_body(
                &spec.captures,
                &spec.ensures,
                &item_fn.sig.output,
            )?),
        };

        // Instrument function body.
        self.instrument_fn(spec, &item_fn.sig, &mut item_fn.block)?;

        item_fn.to_tokens(&mut tokens);
        spec_requires_fn.to_tokens(&mut tokens);
        spec_maintains_fn.to_tokens(&mut tokens);
        spec_ensures_fn.to_tokens(&mut tokens);

        Ok(tokens)
    }

    pub fn instrument_item_trait(
        &self,
        spec: Spec,
        item_trait: ItemTrait,
    ) -> syn::Result<TokenStream> {
        let new_trait = self.instrument_trait(spec, item_trait)?;
        Ok(new_trait.to_token_stream())
    }

    pub fn instrument_item_trait_impl(
        &self,
        spec: Spec,
        item_impl: ItemImpl,
    ) -> syn::Result<TokenStream> {
        let new_trait_impl = self.instrument_trait_impl(spec, item_impl)?;
        Ok(new_trait_impl.to_token_stream())
    }
}

#[cfg(test)]
impl Backend {
    pub(crate) const NOTHING: Backend = Backend {
        emit_print: false,
        emit_panic: false,
    };

    pub(crate) const PRINT: Backend = Backend {
        emit_print: true,
        emit_panic: false,
    };

    pub(crate) const PANIC: Backend = Backend {
        emit_print: true,
        emit_panic: true,
    };
}

/// Make an error message to say that some item is unsupported.
pub fn make_item_error<T: ToTokens>(tokens: &T, item_descr: &str) -> syn::Error {
    let msg = format!(
        r#"The #[spec] attribute doesn't yet support this item: {}.
If this is a problem for your use case, please open a feature
request at https://github.com/mkovaxx/anodized/issues/new"#,
        item_descr
    );
    syn::Error::new_spanned(tokens, msg)
}

/// Finds the `[spec]` attrib in an attribute list.
///
/// Returns the spec [Attribute] and the remaining attributes.
fn find_spec_attr(attrs: Vec<Attribute>) -> syn::Result<(Option<Attribute>, Vec<Attribute>)> {
    let mut spec_attr = None;
    let mut other_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("spec") {
            if spec_attr.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "multiple `#[spec]` attributes on a single item are not supported",
                ));
            }
            spec_attr = Some(attr);
        } else {
            other_attrs.push(attr);
        }
    }

    Ok((spec_attr, other_attrs))
}

fn build_assert(
    cfg: Option<&Meta>,
    expr: &TokenStream,
    message: &str,
    repr: &TokenStream,
) -> TokenStream {
    let repr_str = repr.to_string();
    let check = quote! { assert!(#expr, #message, #repr_str); };
    guard_check(cfg, check)
}

fn build_eprint(
    cfg: Option<&Meta>,
    expr: &TokenStream,
    message: &str,
    repr: &TokenStream,
) -> TokenStream {
    let repr_str = repr.to_string();
    let check = quote! {
        if !(#expr) {
            eprintln!(#message, #repr_str);
        }
    };
    guard_check(cfg, check)
}

fn build_inert(
    // The check will not be present at runtime regardless of the `#[cfg]` setting.
    _cfg: Option<&Meta>,
    expr: &TokenStream,
    message: &str,
    repr: &TokenStream,
) -> TokenStream {
    let repr_str = repr.to_string();
    quote! {
        if false {
            assert!(#expr, #message, #repr_str);
        }
    }
}

fn guard_check(cfg: Option<&Meta>, check: TokenStream) -> TokenStream {
    if let Some(cfg) = cfg {
        quote! { if cfg!(#cfg) { #check } }
    } else {
        check
    }
}
