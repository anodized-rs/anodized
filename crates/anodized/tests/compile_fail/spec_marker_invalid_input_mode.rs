#![no_main]
#![allow(unused_imports)]

use anodized::spec;

#[spec]
fn f(x: Spec!(i32, in)) {}
