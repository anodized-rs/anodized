#![no_main]

use std::sync::OnceLock;

use anodized_fmt::{Config, format_file};
use libfuzzer_sys::fuzz_target;
use test_util::fmt::{Template, Variant};

static TEMPLATE: OnceLock<Template> = OnceLock::new();

/// Build a template for the following fragment:
///
///     #[spec(
///         // the precondition
///         requires: x > 0,
///         // the postcondition
///         ensures: *output > 0,
///     )]
///     fn func(x: i32) -> i32 {
///         todo!()
///     }
///
#[rustfmt::skip]
fn make_template() -> Template {
    Template::new()
        .z().code("# [ spec (")
        .z().line_comment(" the precondition")
        .z().code("requires : x > 0 ,")
        .z().line_comment(" the postcondition")
        .z().code("ensures : * output > 0 ,")
        .z().code(") ]")
        .z().code("fn").p().code("func ( x : i32 ) -> i32 {")
        .z().code("todo ! ( )")
        .z().code("}")
        .z()
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

        dbg!(&default_input);
        dbg!(&variant_input);

        let fmt_default = format_file(&default_input, &config).expect("formatting default");
        let fmt_variant = format_file(&variant_input, &config).expect("formatting variant");

        assert_eq!(fmt_variant, fmt_default);
    }
);
