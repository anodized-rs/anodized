use anodized::logic::values::logical;
use anodized::spec;

#[spec(
    captures: old_opt = logical(opt),
    ensures: |ret| [
        opt.is_none(),
        logical(&ret) == old_opt,
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
        captures: some_x = logical(&Some(&x)),
        ensures: logical(&self.top()) == some_x,
    )]
    fn push(&mut self, x: T);

    #[spec(
        captures: old_top = logical(&self.top()),
        ensures: |ret| logical(&ret.as_ref()) == old_top,
    )]
    fn pop(&mut self) -> Option<T>;
}
