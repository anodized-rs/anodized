#![no_main]
#![allow(unused_imports)]

use anodized::spec;

#[spec]
struct S {
    field: Spec!(i32, inout),
}
