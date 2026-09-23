#![no_main]
#![allow(unused_imports)]

use anodized::spec;

#[spec]
struct S {
    field: spec!(i32, inout),
}
