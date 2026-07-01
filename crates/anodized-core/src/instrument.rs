use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemConst, ItemFn, ItemImpl, ItemTrait, Result, parse_quote};

use crate::{DataSpec, Spec};

pub mod data;
pub mod fns;
pub mod loops;
pub mod traits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Config {
    Nothing,
    Dynamic(RuntimeConfig),
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub does_print: bool,
    pub does_panic: bool,
}

impl Config {
    pub fn instrument_item_fn(&self, spec: Spec, mut item_fn: ItemFn) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        if let Self::Static = self {
            // Embed `spec` elements as `__anodized_fn_*` items.
            let attrs: [Attribute; 2] = [
                parse_quote!(#[doc(hidden)]),
                parse_quote!(#[allow(warnings)]),
            ];

            let spec_qualifiers_const: ItemConst = Self::build_qualifier_const_item(
                &attrs,
                "__anodized_fn_qualifiers",
                spec.qualifiers,
                &item_fn.sig.ident,
            );
            let spec_requires_fn = ItemFn {
                attrs: attrs.to_vec(),
                vis: syn::Visibility::Inherited,
                sig: Self::build_precondition_fn_sig("__anodized_fn_requires", &item_fn.sig),
                block: Box::new(Self::build_precondition_fn_body(
                    &spec.requires,
                    &spec.maintains,
                )),
            };
            let spec_ensures_fn = ItemFn {
                attrs: attrs.to_vec(),
                vis: syn::Visibility::Inherited,
                sig: Self::build_postcondition_fn_sig("__anodized_fn_ensures", &item_fn.sig),
                block: Box::new(Self::build_postcondition_fn_body(
                    &spec.maintains,
                    &spec.captures,
                    &spec.ensures,
                    &item_fn.sig.output,
                )?),
            };

            spec_qualifiers_const.to_tokens(&mut tokens);
            spec_requires_fn.to_tokens(&mut tokens);
            spec_ensures_fn.to_tokens(&mut tokens);
        }

        // Instrument function body.
        self.instrument_fn(&spec, &item_fn.sig, &mut item_fn.block)?;

        item_fn.to_tokens(&mut tokens);
        Ok(tokens)
    }

    pub fn instrument_item_trait(
        &self,
        spec: DataSpec,
        item_trait: ItemTrait,
    ) -> Result<TokenStream> {
        let new_trait = self.instrument_trait(spec, item_trait)?;
        Ok(new_trait.to_token_stream())
    }

    pub fn instrument_item_trait_impl(
        &self,
        spec: DataSpec,
        item_impl: ItemImpl,
    ) -> Result<TokenStream> {
        let new_trait_impl = self.instrument_trait_impl(spec, item_impl)?;
        Ok(new_trait_impl.to_token_stream())
    }
}

#[cfg(test)]
impl Config {
    pub(crate) const DEFAULT: Self = Self::Dynamic(RuntimeConfig::DEFAULT);
}

#[cfg(test)]
impl RuntimeConfig {
    pub(crate) const DEFAULT: Self = Self {
        does_print: false,
        does_panic: false,
    };

    pub(crate) const PRINT: Self = Self {
        does_print: true,
        does_panic: false,
    };

    pub(crate) const PRINT_AND_PANIC: Self = Self {
        does_print: true,
        does_panic: true,
    };
}

/// Make an error message to say that some item is unsupported.
pub fn make_item_error<T: ToTokens>(tokens: &T, item_descr: &str) -> syn::Error {
    let msg = format!(
        r#"The #[spec] attribute doesn't yet support this item: {}.
If this is a problem for your use case, please open a feature
request at https://github.com/anodized-rs/anodized/issues/new"#,
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
