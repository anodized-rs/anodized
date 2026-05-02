use proc_macro2::TokenStream;
use syn::{ExprWhile, parse_quote};

use crate::{LoopSpec, instrument::Backend, test_util::assert_tokens_eq};

#[test]
fn embed_spec_item_struct() {
    let while_spec: LoopSpec = parse_quote! {
        maintains: [
            INVAR_1,
            INVAR_2,
        ],
        decreases: DECREASES_1,
    };
    let expr_while: ExprWhile = parse_quote! {
        while WHILE_COND {
            LOOP_BODY
        }
    };

    let expected: TokenStream = parse_quote! {
        while WHILE_COND {
            let __anodized_loop_maintains = {
                let _ = | | INVAR_1;
                let _ = | | INVAR_2;
            };
            let __anodized_loop_decreases = {
                let _ = | | DECREASES_1;
            };
            LOOP_BODY
        }
    };

    let observed = Backend::NOTHING.instrument_expr_while(while_spec, expr_while);

    assert_tokens_eq(&observed, &expected);
}
