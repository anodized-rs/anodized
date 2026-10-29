use core::marker::PhantomData;

/// Lifts a Rust value into logic.
pub fn logical<T: ?Sized>(_: &T) -> Logical<T> {
    Logical(PhantomData)
}

/// Zero-sized type that represents a Rust value lifted into logic.
///
/// `Logical<T>` is `Copy` and `Eq` for *any* `T`. This allows using `==` and duplicating a value
/// of a fully generic type `T` in specifications. The intended use case is static analysis.
///
/// A logical value has no runtime representation, so operations such as `==` panic.
#[repr(transparent)]
pub struct Logical<T: ?Sized>(
    // `fn() -> *const T` makes `Logical<T>` covariant in `T`, always `Send` and `Sync`,
    // and without drop-check effects.
    // `*const T` permits unsized `T`.
    PhantomData<fn() -> *const T>,
);

impl<T: ?Sized> From<&T> for Logical<T> {
    fn from(value: &T) -> Self {
        logical(value)
    }
}

impl<T: ?Sized> Clone for Logical<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Logical<T> {}

impl<T: ?Sized> PartialEq for Logical<T> {
    #[allow(unused_variables)]
    fn eq(&self, other: &Self) -> bool {
        unimplemented!("equality of logical values cannot be evaluated at runtime")
    }
}

impl<T: ?Sized> Eq for Logical<T> {}
