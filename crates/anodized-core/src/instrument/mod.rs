use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Meta};

pub mod fns;
pub mod traits;

pub struct Backend {
    pub emit_print: bool,
    pub emit_panic: bool,
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
