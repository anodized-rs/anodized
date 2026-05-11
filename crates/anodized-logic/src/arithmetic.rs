use ibig::IBig;

#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct int(IBig);

pub trait Integral {}

use crate::impl_integral;

impl_integral!(i8);
impl_integral!(i16);
impl_integral!(i32);
impl_integral!(i64);
impl_integral!(i128);
impl_integral!(isize);

impl_integral!(u8);
impl_integral!(u16);
impl_integral!(u32);
impl_integral!(u64);
impl_integral!(u128);
impl_integral!(usize);

/// Implements `From<T> for int` by delegating to `IBig`'s own `From` impl.
///
/// This is an internal macro — not exported — since it depends on the concrete
/// backing type and should not be part of the public API.
///
/// # Example (crate-internal)
///
/// ```rust,ignore
/// impl_from_integral!(i8);
/// impl_from_integral!(i16);
/// ```
macro_rules! impl_from_integral {
    ($t:ty) => {
        impl From<$t> for int {
            #[inline]
            fn from(val: $t) -> Self {
                int(IBig::from(val))
            }
        }
    };
}

impl_from_integral!(i8);
impl_from_integral!(i16);
impl_from_integral!(i32);
impl_from_integral!(i64);
impl_from_integral!(i128);
impl_from_integral!(isize);

impl_from_integral!(u8);
impl_from_integral!(u16);
impl_from_integral!(u32);
impl_from_integral!(u64);
impl_from_integral!(u128);
impl_from_integral!(usize);

/// Implements the `Integral` marker trait and all arithmetic interop with `int` for a given type.
///
/// The type must implement `From<T> for int`.
///
/// # Example
///
/// ```rust
/// use anodized_logic::arithmetic::int;
/// use anodized_logic::impl_integral;
///
/// impl_integral!(i32);
/// impl_integral!(u64);
/// ```
#[macro_export]
macro_rules! impl_integral {
    ($t:ty) => {
        impl $crate::arithmetic::Integral for $t {}

        // ------------------------------------------------------------
        // Arithmetic: T op int
        // ------------------------------------------------------------

        impl ::core::ops::Add<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(self) + rhs
            }
        }

        impl ::core::ops::Sub<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(self) - rhs
            }
        }

        impl ::core::ops::Mul<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(self) * rhs
            }
        }

        impl ::core::ops::Div<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(self) / rhs
            }
        }

        impl ::core::ops::Rem<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(self) % rhs
            }
        }

        // ------------------------------------------------------------
        // Arithmetic: int op T
        // ------------------------------------------------------------

        impl ::core::ops::Add<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $t) -> $crate::arithmetic::int {
                self + <$crate::arithmetic::int as ::core::convert::From<$t>>::from(rhs)
            }
        }

        impl ::core::ops::Sub<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $t) -> $crate::arithmetic::int {
                self - <$crate::arithmetic::int as ::core::convert::From<$t>>::from(rhs)
            }
        }

        impl ::core::ops::Mul<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $t) -> $crate::arithmetic::int {
                self * <$crate::arithmetic::int as ::core::convert::From<$t>>::from(rhs)
            }
        }

        impl ::core::ops::Div<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $t) -> $crate::arithmetic::int {
                self / <$crate::arithmetic::int as ::core::convert::From<$t>>::from(rhs)
            }
        }

        impl ::core::ops::Rem<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $t) -> $crate::arithmetic::int {
                self % <$crate::arithmetic::int as ::core::convert::From<$t>>::from(rhs)
            }
        }

        // ------------------------------------------------------------
        // Arithmetic: &int op T and T op &int
        // ------------------------------------------------------------

        impl ::core::ops::Add<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $t) -> $crate::arithmetic::int {
                self.clone() + rhs
            }
        }

        impl ::core::ops::Sub<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $t) -> $crate::arithmetic::int {
                self.clone() - rhs
            }
        }

        impl ::core::ops::Mul<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $t) -> $crate::arithmetic::int {
                self.clone() * rhs
            }
        }

        impl ::core::ops::Div<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $t) -> $crate::arithmetic::int {
                self.clone() / rhs
            }
        }

        impl ::core::ops::Rem<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $t) -> $crate::arithmetic::int {
                self.clone() % rhs
            }
        }

        impl ::core::ops::Add<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self + rhs.clone()
            }
        }

        impl ::core::ops::Sub<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self - rhs.clone()
            }
        }

        impl ::core::ops::Mul<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self * rhs.clone()
            }
        }

        impl ::core::ops::Div<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self / rhs.clone()
            }
        }

        impl ::core::ops::Rem<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self % rhs.clone()
            }
        }

        // ------------------------------------------------------------
        // Comparison: T with int (both directions)
        // ------------------------------------------------------------

        impl ::core::cmp::PartialEq<$crate::arithmetic::int> for $t {
            #[inline]
            fn eq(&self, other: &$crate::arithmetic::int) -> bool {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(*self) == *other
            }
        }

        impl ::core::cmp::PartialEq<$t> for $crate::arithmetic::int {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                *self == <$crate::arithmetic::int as ::core::convert::From<$t>>::from(*other)
            }
        }

        impl ::core::cmp::PartialOrd<$crate::arithmetic::int> for $t {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$crate::arithmetic::int,
            ) -> Option<::core::cmp::Ordering> {
                let lhs = <$crate::arithmetic::int as ::core::convert::From<$t>>::from(*self);
                lhs.partial_cmp(other)
            }
        }

        impl ::core::cmp::PartialOrd<$t> for $crate::arithmetic::int {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<::core::cmp::Ordering> {
                let rhs = <$crate::arithmetic::int as ::core::convert::From<$t>>::from(*other);
                self.partial_cmp(&rhs)
            }
        }

        // ------------------------------------------------------------
        // Compound assignment: int op= T
        // ------------------------------------------------------------

        impl ::core::ops::AddAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                *self = self.clone() + rhs;
            }
        }

        impl ::core::ops::SubAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                *self = self.clone() - rhs;
            }
        }

        impl ::core::ops::MulAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                *self = self.clone() * rhs;
            }
        }

        impl ::core::ops::DivAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                *self = self.clone() / rhs;
            }
        }

        impl ::core::ops::RemAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                *self = self.clone() % rhs;
            }
        }
    };
}

/// Implements `From<T> for int` by delegating to `IBig`'s own `From` impl.
///
/// This is an internal macro — not exported — since it depends on the concrete
/// backing type and should not be part of the public API.
///
/// # Example (crate-internal)
///
/// ```rust,ignore
/// impl_from_integral!(i8);
/// impl_from_integral!(i16);
/// ```
macro_rules! impl_from_integral {
    ($t:ty) => {
        impl From<$t> for int {
            #[inline]
            fn from(val: $t) -> Self {
                int(IBig::from(val))
            }
        }
    };
}
