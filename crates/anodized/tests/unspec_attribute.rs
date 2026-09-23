#![cfg_attr(anodized_charon, feature(register_tool))]
#![cfg_attr(anodized_charon, register_tool(charon))]
#![allow(unused)]

use anodized::{spec, types::Spec};

/// A `struct` with a type spec, a.k.a. refinement.
#[spec(
    maintains: [
        // Something something...
        todo!()
    ],
)]
struct T;

/// Default: inputs and outputs are not checked against their type specs.
#[spec]
fn one(x: T, y: &mut T) -> T {
    todo!()
}

/// Input `x` is checked on entry.
#[spec]
fn two(x: spec!(T), y: &mut T) -> T {
    todo!()
}

/// Input `y` is checked on exit.
#[spec]
fn three(x: T, y: spec!(&mut T, out)) -> T {
    todo!()
}

/// Input `y` is checked on entry and exit.
#[spec]
fn four(x: T, y: spec!(&mut T, inout)) -> T {
    todo!()
}

/// The output is checked on exit.
#[spec]
fn five(x: T, y: &mut T) -> spec!(T) {
    todo!()
}

/// Default: fields are not checked against their type specs.
#[spec]
struct Six {
    pub a: T,
    pub b: T,
}

/// The field `b` is checked against its type spec.
#[spec]
struct Seven {
    pub a: T,
    pub b: spec!(T),
}
