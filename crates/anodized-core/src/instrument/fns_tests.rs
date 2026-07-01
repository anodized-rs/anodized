use crate::test_util::assert_tokens_eq;

use super::*;
use proc_macro2::TokenStream;
use syn::{Block, ItemFn, Type, parse_quote};

fn make_complex_spec() -> Spec {
    parse_quote! {
        requires: COND_1,
        #[cfg(META_1)]
        requires: [COND_2, COND_3],
        maintains: [COND_4, COND_5],
        #[cfg(META_2)]
        maintains: COND_6,
        captures: [
            EXPR_1 as ALIAS_1,
            EXPR_2 as (ALIAS_2, ALIAS_3),
        ],
        binds: PAT_1,
        ensures: COND_7,
        #[cfg(META_3)]
        ensures: [
            COND_8,
            |PAT_2: TYPE| COND_9,
        ],
    }
}

#[test]
fn embed_spec_item_fn() {
    let fn_spec = make_complex_spec();
    let item_fn: ItemFn = parse_quote! {
        fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
            BODY
        }
    };

    let qualifier_bits = FnQualifiers::empty().bits();
    let expected: TokenStream = parse_quote! {
        #[doc(hidden)]
        #[allow(warnings)]
        const __anodized_fn_qualifiers_FUNC: u32 = #qualifier_bits;

        #[doc(hidden)]
        #[allow(warnings)]
        fn __anodized_fn_requires_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> bool {
            let __anodized_clause_1 = (| | -> bool { COND_1 })();
            let __anodized_clause_2 = (| | -> bool { COND_2 })();
            let __anodized_clause_3 = (| | -> bool { COND_3 })();
            let __anodized_clause_4 = (| | -> bool { COND_4 })();
            let __anodized_clause_5 = (| | -> bool { COND_5 })();
            let __anodized_clause_6 = (| | -> bool { COND_6 })();
            __anodized_clause_1 && __anodized_clause_2 && __anodized_clause_3
                && __anodized_clause_4 && __anodized_clause_5 && __anodized_clause_6
        }

        #[doc(hidden)]
        #[allow(warnings)]
        fn __anodized_fn_ensures_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2, __anodized_output: &RET_TYPE) -> bool {
            let __anodized_clause_1 = (| | -> bool { COND_4 })();
            let __anodized_clause_2 = (| | -> bool { COND_5 })();
            let __anodized_clause_3 = (| | -> bool { COND_6 })();
            let (ALIAS_1, (ALIAS_2, ALIAS_3)) = ((| | EXPR_1)(), (| | EXPR_2)());
            let __anodized_clause_4 = (|PAT_1: &RET_TYPE| -> bool { COND_7 })(__anodized_output);
            let __anodized_clause_5 = (|PAT_1: &RET_TYPE| -> bool { COND_8 })(__anodized_output);
            let __anodized_clause_6 = (|PAT_2: TYPE| -> bool { COND_9 })(__anodized_output);
            __anodized_clause_1 && __anodized_clause_2 && __anodized_clause_3
                && __anodized_clause_4 && __anodized_clause_5 && __anodized_clause_6
        }

        fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
            BODY
        }
    };

    let observed = Config::Static.instrument_item_fn(fn_spec, item_fn).unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn default_instrument_item_fn() {
    let fn_spec = make_complex_spec();
    let item_fn: ItemFn = parse_quote! {
        fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
            BODY
        }
    };

    let expected: TokenStream = parse_quote! {
        fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
            if false {
                let mut __anodized_errors = String::new();
                let __anodized_precond = (|| -> bool { COND_1 })()
                    & (|| -> bool { COND_2 })()
                    & (|| -> bool { COND_3 })()
                    & (|| -> bool { COND_4 })()
                    & (|| -> bool { COND_5 })()
                    & (|| -> bool { COND_6 })();
                if !__anodized_precond {}
            }
            let (ALIAS_1, (ALIAS_2, ALIAS_3), __anodized_output): (_, _, RET_TYPE) = (
                (|| EXPR_1)(),
                (|| EXPR_2)(),
                (|| {
                    BODY
                })(),
            );
            if false {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = (|| -> bool { COND_4 })()
                    & (|| -> bool { COND_5 })()
                    & (|| -> bool { COND_6 })()
                    & (|PAT_1: &RET_TYPE| -> bool { COND_7 })(&__anodized_output)
                    & (|PAT_1: &RET_TYPE| -> bool { COND_8 })(&__anodized_output)
                    & (|PAT_2: TYPE| -> bool { COND_9 })(&__anodized_output);
                if !__anodized_postcond {}
            }
            __anodized_output
        }
    };

    let observed = Config::DEFAULT
        .instrument_item_fn(fn_spec, item_fn)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

fn make_fn_body() -> Block {
    parse_quote! {
        {
            this_is_the_body()
        }
    }
}

fn make_return_type() -> Type {
    parse_quote! { SomeType }
}

#[test]
fn simple_requires() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = true;
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn requires_disable_runtime_checks() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let observed = RuntimeConfig::DEFAULT
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    let expected: Block = parse_quote! {
        {
            if false {
                let mut __anodized_errors = String::new();
                let __anodized_precond = (|| -> bool { CONDITION_1 })();
                if !__anodized_precond {}
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if false {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = true;
                if !__anodized_postcond {}
            }
            __anodized_output
        }
    };
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn requires_no_panic_runtime() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    eprintln!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = true;
                if !__anodized_postcond {
                    eprintln!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_maintains() {
    let spec: Spec = parse_quote! {
        maintains: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_ensures() {
    let spec: Spec = parse_quote! {
        ensures: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = true;
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|output: &#ret_type| -> bool { CONDITION_1 })(&__anodized_output)
                    || __anodized_errors.push_str("\n    | output | CONDITION_1") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_requires_and_maintains() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        maintains: CONDITION_2,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_2 })()
                    || __anodized_errors.push_str("\n    CONDITION_2") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_requires_and_ensures() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        ensures: CONDITION_2,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|output: &#ret_type| -> bool { CONDITION_2 })(&__anodized_output)
                    || __anodized_errors.push_str("\n    | output | CONDITION_2") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_maintains_and_ensures() {
    let spec: Spec = parse_quote! {
        maintains: CONDITION_1,
        ensures: CONDITION_2,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_2 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_2") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_requires_maintains_and_ensures() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        maintains: CONDITION_2,
        ensures: CONDITION_3,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_2 })()
                    || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_3 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_3") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn simple_async_requires_maintains_and_ensures() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        maintains: CONDITION_2,
        ensures: CONDITION_3,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = true;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((async || #body)().await);
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_2 })()
                    || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_3 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_3") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn multiple_conditions_in_clauses() {
    let spec: Spec = parse_quote! {
        requires: [CONDITION_1, CONDITION_2],
        maintains: [CONDITION_3, CONDITION_4],
        ensures: [CONDITION_5, CONDITION_6],
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & ((|| -> bool { CONDITION_3 })()
                        || __anodized_errors.push_str("\n    CONDITION_3") != ())
                    & ((|| -> bool { CONDITION_4 })()
                        || __anodized_errors.push_str("\n    CONDITION_4") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_3 })()
                    || __anodized_errors.push_str("\n    CONDITION_3") != ())
                    & ((|| -> bool { CONDITION_4 })()
                        || __anodized_errors.push_str("\n    CONDITION_4") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_5 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_5") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_6 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_6") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn binds_parameter() {
    let spec: Spec = parse_quote! {
        binds: OUTPUT_PATTERN,
        ensures: CONDITION_1,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = true;
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|OUTPUT_PATTERN: &#ret_type| -> bool { CONDITION_1 })(&__anodized_output)
                    || __anodized_errors.push_str("\n    | OUTPUT_PATTERN | CONDITION_1") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn ensures_with_mixed_conditions() {
    let spec: Spec = parse_quote! {
        ensures: [
            CONDITION_1,
            |PATTERN_1| CONDITION_2,
            CONDITION_3,
            |PATTERN_2| CONDITION_4
        ],
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = true;
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|output: &#ret_type| -> bool { CONDITION_1 })(&__anodized_output)
                    || __anodized_errors.push_str("\n    | output | CONDITION_1") != ())
                    & ((|PATTERN_1: &#ret_type| -> bool { CONDITION_2 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | PATTERN_1 | CONDITION_2") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_3 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_3") != ())
                    & ((|PATTERN_2: &#ret_type| -> bool { CONDITION_4 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | PATTERN_2 | CONDITION_4") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn cfg_attributes() {
    let spec: Spec = parse_quote! {
        #[cfg(SETTING_1)]
        requires: CONDITION_1,
        #[cfg(SETTING_2)]
        maintains: CONDITION_2,
        #[cfg(SETTING_3)]
        ensures: CONDITION_3,
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = (!cfg!(SETTING_1) || (|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & (!cfg!(SETTING_2) || (|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = (!cfg!(SETTING_2) || (|| -> bool { CONDITION_2 })()
                    || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & (!cfg!(SETTING_3)
                        || (|output: &#ret_type| -> bool { CONDITION_3 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_3") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn cfg_on_single_and_list_conditions() {
    let spec: Spec = parse_quote! {
        #[cfg(SETTING_1)]
        requires: CONDITION_1,
        maintains: [CONDITION_2, CONDITION_3],
        #[cfg(SETTING_2)]
        ensures: [CONDITION_4, CONDITION_5],
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = (!cfg!(SETTING_1) || (|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & ((|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & ((|| -> bool { CONDITION_3 })()
                        || __anodized_errors.push_str("\n    CONDITION_3") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_2 })()
                    || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & ((|| -> bool { CONDITION_3 })()
                        || __anodized_errors.push_str("\n    CONDITION_3") != ())
                    & (!cfg!(SETTING_2)
                        || (|output: &#ret_type| -> bool { CONDITION_4 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_4") != ())
                    & (!cfg!(SETTING_2)
                        || (|output: &#ret_type| -> bool { CONDITION_5 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_5") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn complex_mixed_conditions() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        #[cfg(SETTING_1)]
        requires: [CONDITION_2, CONDITION_3],
        maintains: [CONDITION_4, CONDITION_5],
        #[cfg(SETTING_2)]
        maintains: CONDITION_6,
        ensures: CONDITION_7,
        #[cfg(SETTING_3)]
        ensures: [CONDITION_8, |PATTERN_1| CONDITION_9],
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ())
                    & (!cfg!(SETTING_1) || (|| -> bool { CONDITION_2 })()
                        || __anodized_errors.push_str("\n    CONDITION_2") != ())
                    & (!cfg!(SETTING_1) || (|| -> bool { CONDITION_3 })()
                        || __anodized_errors.push_str("\n    CONDITION_3") != ())
                    & ((|| -> bool { CONDITION_4 })()
                        || __anodized_errors.push_str("\n    CONDITION_4") != ())
                    & ((|| -> bool { CONDITION_5 })()
                        || __anodized_errors.push_str("\n    CONDITION_5") != ())
                    & (!cfg!(SETTING_2) || (|| -> bool { CONDITION_6 })()
                        || __anodized_errors.push_str("\n    CONDITION_6") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (__anodized_output): (#ret_type) = ((|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|| -> bool { CONDITION_4 })()
                    || __anodized_errors.push_str("\n    CONDITION_4") != ())
                    & ((|| -> bool { CONDITION_5 })()
                        || __anodized_errors.push_str("\n    CONDITION_5") != ())
                    & (!cfg!(SETTING_2) || (|| -> bool { CONDITION_6 })()
                        || __anodized_errors.push_str("\n    CONDITION_6") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_7 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_7") != ())
                    & (!cfg!(SETTING_3)
                        || (|output: &#ret_type| -> bool { CONDITION_8 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_8") != ())
                    & (!cfg!(SETTING_3)
                        || (|PATTERN_1: &#ret_type| -> bool { CONDITION_9 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | PATTERN_1 | CONDITION_9") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn captures() {
    let spec: Spec = parse_quote! {
        requires: CONDITION_1,
        captures: [
            EXPR_1 as ALIAS_1,
            EXPR_2 as ALIAS_2,
        ],
        ensures: [
            CONDITION_2,
            CONDITION_3,
        ],
    };
    let body = make_fn_body();
    let ret_type = make_return_type();
    let is_async = false;

    let expected: Block = parse_quote! {
        {
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_precond = ((|| -> bool { CONDITION_1 })()
                    || __anodized_errors.push_str("\n    CONDITION_1") != ());
                if !__anodized_precond {
                    panic!("precondition failed:{__anodized_errors}");
                }
            }
            let (ALIAS_1, ALIAS_2, __anodized_output): (_, _, #ret_type) = ((| | EXPR_1) (), (| | EXPR_2) (), (|| #body)());
            if true {
                let mut __anodized_errors = String::new();
                let __anodized_postcond = ((|output: &#ret_type| -> bool { CONDITION_2 })(&__anodized_output)
                    || __anodized_errors.push_str("\n    | output | CONDITION_2") != ())
                    & ((|output: &#ret_type| -> bool { CONDITION_3 })(&__anodized_output)
                        || __anodized_errors.push_str("\n    | output | CONDITION_3") != ());
                if !__anodized_postcond {
                    panic!("postcondition failed:{__anodized_errors}");
                }
            }
            __anodized_output
        }
    };

    let observed = RuntimeConfig::PRINT_AND_PANIC
        .instrument_fn_body(&spec, &body, is_async, &ret_type)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}
