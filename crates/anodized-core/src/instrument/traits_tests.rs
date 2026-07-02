use crate::{qualifiers::FnQualifiers, test_util::assert_tokens_eq};

use super::*;
use proc_macro2::TokenStream;
use syn::{ItemImpl, ItemTrait, parse_quote};

#[test]
fn embed_spec_item_trait() {
    let trait_spec = DataSpec::empty();
    let item_trait: ItemTrait = parse_quote! {
        trait TRAIT {
            #[spec(
                requires: COND_1,
                maintains: COND_2,
                binds: PAT_1,
                ensures: COND_3,
            )]
            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let qualifier_bits = FnQualifiers::empty().bits();
    let expected: TokenStream = parse_quote! {
        trait TRAIT {
            #[doc(hidden)]
            #[allow(warnings)]
            fn __anodized_fn_requires_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> bool {
                let __anodized_clause_1 = (| | -> bool { COND_1 })();
                let __anodized_clause_2 = (| | -> bool { COND_2 })();
                __anodized_clause_1 && __anodized_clause_2
            }

            #[doc(hidden)]
            #[allow(warnings)]
            fn __anodized_fn_ensures_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2, __anodized_output: &RET_TYPE) -> bool {
                let __anodized_clause_1 = (| | -> bool { COND_2 })();
                let () = ();
                let __anodized_clause_2 = (|PAT_1: &RET_TYPE| -> bool { COND_3 })(__anodized_output);
                __anodized_clause_1 && __anodized_clause_2
            }

            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_trait_FUNC: u32 = #qualifier_bits;

            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_FUNC: u32 = #qualifier_bits;

            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let observed = Mode::EmbedSpecs
        .instrument_item_trait(trait_spec, item_trait)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn default_instrument_item_trait() {
    let trait_spec = DataSpec::empty();
    let item_trait: ItemTrait = parse_quote! {
        trait TRAIT {
            #[spec(
                requires: COND_1,
                maintains: COND_2,
                binds: PAT_1,
                ensures: COND_3,
            )]
            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let qualifier_bits = FnQualifiers::empty().bits();
    let expected: TokenStream = parse_quote! {
        trait TRAIT {
            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_trait_FUNC: u32 = #qualifier_bits;

            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_FUNC: u32 = #qualifier_bits;

            #[doc(hidden)]
            fn __anodized_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }

            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                if false {
                    let mut __anodized_errors = String::new();
                    let __anodized_precond = (|| -> bool { COND_1 })()
                        & (|| -> bool { COND_2 })();
                    if !__anodized_precond {}
                }
                let (__anodized_output): (RET_TYPE) = ((|| {
                    Self::__anodized_FUNC(self, PARAM_1, PARAM_2)
                })());
                if false {
                    let mut __anodized_errors = String::new();
                    let __anodized_postcond = (|| -> bool { COND_2 })()
                        & (|PAT_1: &RET_TYPE| -> bool { COND_3 })(&__anodized_output);
                    if !__anodized_postcond {}
                }
                __anodized_output
            }
        }
    };

    let observed = Mode::DEFAULT
        .instrument_item_trait(trait_spec, item_trait)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn embed_spec_item_impl_trait() {
    let trait_spec = DataSpec::empty();
    let item_impl: ItemImpl = parse_quote! {
        impl TRAIT for IMPL_TYPE {
            #[spec(
                requires: COND_1,
                maintains: COND_2,
                binds: PAT_1,
                ensures: COND_3,
            )]
            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let qualifier_bits = FnQualifiers::empty().bits();
    let expected: TokenStream = parse_quote! {
        impl TRAIT for IMPL_TYPE {
            #[doc(hidden)]
            #[allow(warnings)]
            fn __anodized_fn_requires_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> bool {
                let __anodized_clause_1 = (| | -> bool { COND_1 })();
                let __anodized_clause_2 = (| | -> bool { COND_2 })();
                __anodized_clause_1 && __anodized_clause_2
            }

            #[doc(hidden)]
            #[allow(warnings)]
            fn __anodized_fn_ensures_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2, __anodized_output: &RET_TYPE) -> bool {
                let __anodized_clause_1 = (| | -> bool { COND_2 })();
                let () = ();
                let __anodized_clause_2 = (|PAT_1: &RET_TYPE| -> bool { COND_3 })(__anodized_output);
                __anodized_clause_1 && __anodized_clause_2
            }

            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_FUNC: u32 = #qualifier_bits;

            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let observed = Mode::EmbedSpecs
        .instrument_item_trait_impl(trait_spec, item_impl)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}

#[test]
fn default_instrument_item_impl_trait() {
    let trait_spec = DataSpec::empty();
    let item_impl: ItemImpl = parse_quote! {
        impl TRAIT for IMPL_TYPE {
            #[spec(
                requires: COND_1,
                maintains: COND_2,
                binds: PAT_1,
                ensures: COND_3,
            )]
            fn FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                BODY
            }
        }
    };

    let qualifier_bits = FnQualifiers::empty().bits();
    let expected: TokenStream = parse_quote! {
        impl TRAIT for IMPL_TYPE {
            #[doc(hidden)]
            #[allow(warnings)]
            const __anodized_fn_qualifiers_FUNC: u32 = #qualifier_bits;

            #[inline]
            fn __anodized_FUNC(&self, PARAM_1: TYPE_1, PARAM_2: TYPE_2) -> RET_TYPE {
                const {
                    assert!(
                        Self::__anodized_fn_qualifiers_FUNC ==
                            Self::__anodized_fn_qualifiers_trait_FUNC |
                            Self::__anodized_fn_qualifiers_FUNC,
                        "the qualifiers on the impl `IMPL_TYPE::FUNC` cannot be weaker than the qualifiers on the trait `TRAIT::FUNC`",
                    );
                };
                if false {
                    let mut __anodized_errors = String::new();
                    let __anodized_precond = (|| -> bool { COND_1 })()
                        & (|| -> bool { COND_2 })();
                    if !__anodized_precond {}
                }
                let (__anodized_output): (RET_TYPE) = ((|| {
                    BODY
                })());
                if false {
                    let mut __anodized_errors = String::new();
                    let __anodized_postcond = (|| -> bool { COND_2 })()
                        & (|PAT_1: &RET_TYPE| -> bool { COND_3 })(&__anodized_output);
                    if !__anodized_postcond {}
                }
                __anodized_output
            }
        }
    };

    let observed = Mode::DEFAULT
        .instrument_item_trait_impl(trait_spec, item_impl)
        .unwrap();
    assert_tokens_eq(&observed, &expected);
}
