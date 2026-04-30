use crate::{DataSpec, instrument::Backend, test_util::assert_tokens_eq};

use proc_macro2::TokenStream;
use syn::{ItemEnum, ItemStruct, parse_quote};

#[test]
fn embed_spec_item_struct() {
    let struct_spec: DataSpec = parse_quote! {
        maintains: [
            COND_1,
            COND_2,
        ],
    };
    let item_struct: ItemStruct = parse_quote! {
        struct STRUCT<'LT_1, TYPE_1: BOUND_1>
        where
            'LT_1: 'LT_2,
        {
            FIELD_1: &'LT_1 TYPE_2,
            FIELD_2: TYPE_1,
        }
    };

    let expected: TokenStream = parse_quote! {
        struct STRUCT<'LT_1, TYPE_1: BOUND_1>
        where
            'LT_1: 'LT_2,
        {
            FIELD_1: &'LT_1 TYPE_2,
            FIELD_2: TYPE_1,
        }

        #[doc(hidden)]
        #[allow(warnings)]
        impl<'LT_1, TYPE_1: BOUND_1> STRUCT<'LT_1, TYPE_1>
        where
            'LT_1: 'LT_2,
        {
            fn __anodized_data_maintains(&self) {
                let _ = | | COND_1;
                let _ = | | COND_2;
            }
        }
    };

    let observed = Backend::NOTHING
        .instrument_item_struct(struct_spec, item_struct)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn embed_spec_item_enum() {
    let struct_spec: DataSpec = parse_quote! {
        maintains: [
            COND_1,
            COND_2,
        ],
    };
    let item_enum: ItemEnum = parse_quote! {
        enum ENUM<'LT_1, TYPE_1: BOUND_1>
        where
            'LT_1: 'LT_2,
        {
            VARIANT_1(&'LT_1 TYPE_2),
            VARIANT_2 { FIELD_2: TYPE_1 },
            VARIANT_3,
        }
    };

    let expected: TokenStream = parse_quote! {
        enum ENUM<'LT_1, TYPE_1: BOUND_1>
        where
            'LT_1: 'LT_2,
        {
            VARIANT_1(&'LT_1 TYPE_2),
            VARIANT_2 { FIELD_2: TYPE_1 },
            VARIANT_3,
        }

        #[doc(hidden)]
        #[allow(warnings)]
        impl<'LT_1, TYPE_1: BOUND_1> ENUM<'LT_1, TYPE_1>
        where
            'LT_1: 'LT_2,
        {
            fn __anodized_data_maintains(&self) {
                use ENUM::*;
                let _ = | | COND_1;
                let _ = | | COND_2;
            }
        }
    };

    let observed = Backend::NOTHING
        .instrument_item_enum(struct_spec, item_enum)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}
