#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Expr, Item, TraitItemFn, parse_macro_input};

use anodized_core::{
    annotate::Specified as _,
    instrument::{
        CheckSettings, Mode, PanicSettings, SpecEmbedding, fns::make_try_call, make_item_error,
    },
    syntax::SpecFields,
};

const CONFIG: Mode = validate_config();

const fn validate_config() -> Mode {
    let erase = cfg!(anodized_discard_specs);
    let static_cfg = StaticCfg {
        is_enabled: cfg!(anodized_embed_specs),
        with_charon: cfg!(anodized_charon),
    };
    let runtime_cfg = RuntimeCfg {
        with_panic: cfg!(anodized_panic),
        with_print: cfg!(anodized_print),
        with_try: cfg!(anodized_try),
    };

    if erase {
        if static_cfg.any() || runtime_cfg.any() {
            panic!("`anodized_discard_specs` is incompatible with all other `anodized_*` settings");
        }
        Mode::ChangeNothing
    } else if static_cfg.any() {
        if runtime_cfg.any() {
            panic!("`anodized_embed_specs` is incompatible with all `anodized_runtime_*` settings");
        }
        Mode::EmbedSpecs(SpecEmbedding {
            uses_charon: static_cfg.with_charon,
        })
    } else {
        Mode::InjectChecks(CheckSettings {
            does_print: runtime_cfg.with_print,
            does_panic: if runtime_cfg.with_panic {
                Some(PanicSettings {
                    has_try_fn: runtime_cfg.with_try,
                })
            } else {
                None
            },
        })
    }
}

struct StaticCfg {
    is_enabled: bool,
    with_charon: bool,
}

impl StaticCfg {
    const fn any(&self) -> bool {
        let StaticCfg {
            is_enabled,
            with_charon,
        } = *self;
        is_enabled || with_charon
    }
}

struct RuntimeCfg {
    with_panic: bool,
    with_print: bool,
    with_try: bool,
}

impl RuntimeCfg {
    const fn any(&self) -> bool {
        let RuntimeCfg {
            with_panic,
            with_print,
            with_try,
        } = *self;
        with_panic || with_print || with_try
    }
}

/// **Must** be inside a `#[spec]` attribute's item. May be applied to a `fn`, its inputs, and
/// fields of a `struct` or `enum`.
///
/// - `#[unspec]`: The input or field may *not* satisfy its type spec.
/// - `#[unspec(in)]`: The input may *not* satisfy its type spec at entry.
/// - `#[unspec(out)]`: The input or output may *not* satisfy its type spec at exit.
///
/// This macro exists *only* to carry this documentation. It will never be invoked, because
/// `anodized-core` removes it as part of processing `#[spec]` attributes. Note that Rust does
/// *not* support macro attributes on inputs of a `fn` or fields of a `struct` or `enum`.
#[proc_macro_attribute]
pub fn unspec(_: TokenStream, _: TokenStream) -> TokenStream {
    syn::Error::new(
        Span::call_site(),
        "must be on or inside the item of a `#[spec]` attribute",
    )
    .to_compile_error()
    .into()
}

/// Attaches a specification to supported program elements.
///
/// This macro parses the spec and transforms the item's code to provide the following features:
/// - compile-time validation: the spec's syntax, scope, and types
/// - runtime checks: for supported items, configured by `cfg` settings
#[proc_macro_attribute]
pub fn spec(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the `#[spec]` attribute's arguments.
    let spec_fields = parse_macro_input!(args as SpecFields);

    // Parse the item to which the attribute is attached.
    let item = parse_macro_input!(input as Item);

    let result = match item {
        Item::Fn(mut func) => func
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_fn(spec, func)),
        Item::Trait(mut the_trait) => the_trait
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_trait(spec, the_trait)),
        Item::Impl(mut the_impl) if the_impl.trait_.is_some() => the_impl
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_trait_impl(spec, the_impl)),
        Item::Impl(mut the_impl) if the_impl.trait_.is_none() => the_impl
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_impl(spec, the_impl)),
        Item::Const(_) => Err(make_item_error(&item, "const")),
        Item::Enum(mut the_enum) => the_enum
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_enum(spec, the_enum)),
        Item::ExternCrate(_) => Err(make_item_error(&item, "extern crate")),
        Item::ForeignMod(_) => Err(make_item_error(&item, "extern block")),
        Item::Macro(_) => Err(make_item_error(&item, "macro")),
        Item::Mod(_) => Err(make_item_error(&item, "mod")),
        Item::Static(_) => Err(make_item_error(&item, "static")),
        Item::Struct(mut the_struct) => the_struct
            .parse_spec_from_fields(spec_fields)
            .and_then(|spec| CONFIG.instrument_item_struct(spec, the_struct)),
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

#[proc_macro]
pub fn try_call(args: TokenStream) -> TokenStream {
    if !CONFIG.emits_try_fn() {
        return syn::Error::new(
            Span::call_site(),
            "`try_call` needs the `anodized_try` build `cfg` to be enabled",
        )
        .to_compile_error()
        .into();
    }

    let expr = parse_macro_input!(args as Expr);

    match make_try_call(expr) {
        Ok(call) => call.into_token_stream().into(),
        Err(error) => error.to_compile_error().into(),
    }
}
