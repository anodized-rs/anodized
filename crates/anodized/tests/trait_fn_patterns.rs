#![allow(unused)]

use anodized::spec;

#[spec]
trait TraitA {
    #[spec(requires: left <= right)]
    fn func(input @ (left, right): (i32, i32)) {
        let _ = (left, right);
    }
}

pub struct Bounds {
    lower: i32,
    upper: i32,
}

#[spec]
trait TraitB {
    #[spec(requires: lower <= upper)]
    fn func(Bounds { lower, upper }: Bounds) {
        let _ = (lower, upper);
    }
}
