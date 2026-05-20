#![allow(unused_variables, unused_imports)]

use anodized::{
    logic::{implies, opaque, quantifiers::forall},
    spec,
};
use hax_lib::fstar;

// Specs on functions

#[spec(requires: x > 0)]
pub fn f1(x: u8) {}

#[spec(ensures: *output == 1)]
pub fn f2() -> u8 {
    1
}

// Specs on traits

#[spec]
pub trait T {
    #[spec(
        requires: x > 0,
        ensures: *output > 0,
    )]
    fn f3(x: u8) -> u8;
}

#[spec]
impl T for u8 {
    #[spec(
        requires: true,
        ensures: *output > 1,
    )]
    fn f3(x: u8) -> u8 {
        2
    }
}

// Loop invariants

#[spec(
    // TODO: Would also need a spec that max is an element of x
    ensures: forall(|j: usize| implies!(j < x.len(), x[j] <= *output)),
)]
pub fn f_loop(x: &[u8]) -> u8 {
    let mut max = 0;
    #[spec(
        maintains: forall(|j: usize| implies!(j < i, x[j] <= max)),
    )]
    for i in 0..x.len() {
        if x[i] > max {
            max = x[i]
        }
    }
    max
}

// While loop

#[spec(
    // TODO: Would also need a spec that max is an element of x
    ensures: forall(|j: usize| implies!(j < x.len(), x[j] <= *output),
))]
pub fn f_while(x: &[u8]) -> u8 {
    let mut max = 0;
    let mut i = 0;
    #[spec(
        maintains: [
            i <= x.len(),
            forall(|j: usize| implies!(j < i, x[j] <= max)),
        ],
        decreases: x.len() - i,
    )]
    while i < x.len() {
        if x[i] > max {
            max = x[i]
        }
        i += 1;
    }
    max
}

// Raw F*
#[spec(
    #[cfg(hax)]
    requires: opaque!(fstar!("x >. mk_u8 0")),
)]
pub fn f_blob(x: u8) {}

// Raw F* with reference
#[spec(
    #[cfg(hax)]
    requires: opaque!(fstar!("${x} >. ${f2()}")),
)]
pub fn f_ref(x: u8) {}

// Refinement types
#[spec(
    maintains: self.x > 0,
)]
struct S {
    x: u8,
}
