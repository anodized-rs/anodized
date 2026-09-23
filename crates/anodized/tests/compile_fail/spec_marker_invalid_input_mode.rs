#![no_main]
#![allow(unused_imports)]

use anodized::spec;

#[spec]
fn f(x: spec!(i32, in)) {}
