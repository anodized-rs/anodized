#[diagnostic::on_unimplemented(
    label = "type at the boundary of a `#[spec]`",
    message = "\
type spec enforcement needs `{Self}` to implement trait `anodized::types::Spec`",
    note = "\
if `{Self}` is a concrete local type, place a `#[spec]` attribute on its definition",
    note = "\
if `{Self}` is a concrete foreign type, wrap it in a local type such as `struct NewType({Self})`",
    note = "\
if `{Self}` is a type parameter, restrict it with the trait `{Self}: Spec`",
    note = "\
*UNSAFE*: alternatively, use `#[uncheck]` to locally disable type spec enforcement here"
)]
/// Defines a type refinement.
pub trait Spec {
    /// Returns `true` for a valid value and `false` otherwise.
    fn predicate(&self) -> bool;
}

impl<T: Spec + ?Sized> Spec for &T {
    fn predicate(&self) -> bool {
        <T as Spec>::predicate(self)
    }
}

impl<T: Spec + ?Sized> Spec for &mut T {
    fn predicate(&self) -> bool {
        <T as Spec>::predicate(self)
    }
}

impl<T: Spec> Spec for [T] {
    fn predicate(&self) -> bool {
        self.iter().all(<T as Spec>::predicate)
    }
}

impl<T: Spec, const N: usize> Spec for [T; N] {
    fn predicate(&self) -> bool {
        self.iter().all(<T as Spec>::predicate)
    }
}

impl<T: Spec> Spec for Option<T> {
    fn predicate(&self) -> bool {
        match self {
            Some(inner) => inner.predicate(),
            None => true,
        }
    }
}

impl<T: Spec, E: Spec> Spec for Result<T, E> {
    fn predicate(&self) -> bool {
        match self {
            Ok(okay) => okay.predicate(),
            Err(err) => err.predicate(),
        }
    }
}

impl<T: Spec + ?Sized> Spec for Box<T> {
    fn predicate(&self) -> bool {
        self.as_ref().predicate()
    }
}

impl<T: Spec> Spec for Vec<T> {
    fn predicate(&self) -> bool {
        <[T] as Spec>::predicate(self.as_slice())
    }
}

impl Spec for () {
    fn predicate(&self) -> bool {
        true
    }
}

impl<T1: Spec + ?Sized> Spec for (T1,) {
    fn predicate(&self) -> bool {
        self.0.predicate()
    }
}

impl<T1: Spec, T2: Spec + ?Sized> Spec for (T1, T2) {
    fn predicate(&self) -> bool {
        self.0.predicate() && self.1.predicate()
    }
}

impl<T1: Spec, T2: Spec, T3: Spec + ?Sized> Spec for (T1, T2, T3) {
    fn predicate(&self) -> bool {
        self.0.predicate() && self.1.predicate() && self.2.predicate()
    }
}
