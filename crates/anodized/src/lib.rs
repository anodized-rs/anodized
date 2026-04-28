#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Item, TraitItemFn, parse_macro_input};

use anodized_core::{
    Spec,
    instrument::{Backend, make_item_error},
};

const BACKEND: Backend = Backend {
    emit_print: cfg!(anodized_print),
    emit_panic: cfg!(anodized_panic),
};

/// Attaches a specification to a fn, or enables specs inside a trait and its impls.
///
/// This macro parses spec elements and transforms the item's code to provide
/// compile-time syntax validation, and depending on settings, runtime checks.
#[proc_macro_attribute]
pub fn spec(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the item to which the attribute is attached.
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Fn(mut func) => {
            let spec = parse_macro_input!(args as Spec);
            BACKEND
                .instrument_fn(spec, &func.sig, &mut func.block)
                .map(|_| func.into_token_stream())
        }
        Item::Trait(the_trait) => {
            let spec = parse_macro_input!(args as Spec);
            BACKEND
                .instrument_trait(spec, the_trait)
                .map(|tokens| tokens.into_token_stream())
        }
        Item::Impl(the_impl) if the_impl.trait_.is_some() => {
            let spec = parse_macro_input!(args as Spec);
            BACKEND
                .instrument_trait_impl(spec, the_impl)
                .map(|tokens| tokens.into_token_stream())
        }
        Item::Impl(ref the_impl) if the_impl.trait_.is_none() => {
            Err(make_item_error(&item, "inherent impl"))
        }
        Item::Const(_) => Err(make_item_error(&item, "const")),
        Item::Enum(_) => Err(make_item_error(&item, "enum")),
        Item::ExternCrate(_) => Err(make_item_error(&item, "extern crate")),
        Item::ForeignMod(_) => Err(make_item_error(&item, "extern block")),
        Item::Macro(_) => Err(make_item_error(&item, "macro")),
        Item::Mod(_) => Err(make_item_error(&item, "mod")),
        Item::Static(_) => Err(make_item_error(&item, "static")),
        Item::Struct(_) => Err(make_item_error(&item, "struct")),
        Item::TraitAlias(_) => Err(make_item_error(&item, "trait alias")),
        Item::Type(_) => Err(make_item_error(&item, "type")),
        Item::Union(_) => Err(make_item_error(&item, "union")),
        Item::Use(_) => Err(make_item_error(&item, "use")),
        Item::Verbatim(ref tokens) => {
            // Try to parse as a trait fn
            if let Ok(trait_fn) = syn::parse2::<TraitItemFn>(tokens.clone()) {
                Err(syn::Error::new_spanned(
                    &trait_fn,
                    r#"The enclosing trait must have a `#[spec]` annotation."#,
                ))
            } else {
                Err(make_item_error(&item, "<unexpected>"))
            }
        }
        _ => Err(make_item_error(&item, "<unknown>")),
    };

    match result {
        Ok(item) => item.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
