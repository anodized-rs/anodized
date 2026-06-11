#![no_main]

use std::sync::OnceLock;

use anodized_fmt::{Config, format_file};
use libfuzzer_sys::fuzz_target;
use test_util::fmt::{Template, Variant};

static TEMPLATE: OnceLock<Template> = OnceLock::new();

/// Build a template for the following fragment:
///
///     #[spec(
///         requires: x > 0,
///         ensures: *output > 0,
///     )]
///     fn func(x: i32) -> i32 { todo!() }
///
#[rustfmt::skip]
fn make_template() -> Template {
    Template::new()
        .code("#[spec( requires : x > 0 , ensures : * output > 0 , )]").fixed("\n")
        .fixed("fn func(x: i32) -> i32 { todo!() }\n")
}

fuzz_target!(
    init: {
        TEMPLATE.set(make_template()).unwrap();
    },
    |variant: Variant| {
        let config = Config::default();

        let template = TEMPLATE.get().unwrap();
        let default_input = template.generate(Variant::default());
        let variant_input = template.generate(variant);

        let fmt_default = format_file(&default_input, &config).expect("formatting default");
        let fmt_variant = format_file(&variant_input, &config).expect("formatting variant");

        assert_eq!(fmt_variant, fmt_default);
    }
);
