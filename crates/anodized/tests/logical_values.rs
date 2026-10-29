#![cfg_attr(anodized_charon, feature(register_tool))]
#![cfg_attr(anodized_charon, register_tool(charon))]

use anodized::logic::values::logical;
use anodized::spec;

#[spec(
    captures: old_opt = opt.as_ref().map(logical),
    ensures: |ret| [
        opt.is_none(),
        ret.as_ref().map(logical) == old_opt,
    ],
)]
pub fn take<T>(opt: &mut Option<T>) -> Option<T> {
    todo!()
}

#[spec(
    captures: [
        old_a = logical(a),
        old_b = logical(b),
    ],
    ensures: [
        logical(a) == old_a,
        logical(b) == old_b,
    ],
)]
pub fn swap<T>(a: &mut T, b: &mut T) {
    todo!()
}

#[spec]
pub trait Stack<T> {
    fn top(&self) -> Option<&T>;

    #[spec(
        captures: old_x = logical(&x),
        ensures: self.top().map(logical) == Some(old_x),
    )]
    fn push(&mut self, x: T);

    #[spec(
        captures: old_top = self.top().map(logical),
        ensures: |ret| ret.as_ref().map(logical) == old_top,
    )]
    fn pop(&mut self) -> Option<T>;
}
