use ibig::IBig;

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct int(IBig);

impl ::core::ops::Add<int> for int {
    type Output = int;

    #[inline]
    fn add(self, rhs: int) -> int {
        int(self.0 + rhs.0)
    }
}

impl ::core::ops::Sub<int> for int {
    type Output = int;

    #[inline]
    fn sub(self, rhs: int) -> int {
        int(self.0 - rhs.0)
    }
}

impl ::core::ops::Mul<int> for int {
    type Output = int;

    #[inline]
    fn mul(self, rhs: int) -> int {
        int(self.0 * rhs.0)
    }
}

impl ::core::ops::Div<int> for int {
    type Output = int;

    #[inline]
    fn div(self, rhs: int) -> int {
        int(self.0 / rhs.0)
    }
}

impl ::core::ops::Rem<int> for int {
    type Output = int;

    #[inline]
    fn rem(self, rhs: int) -> int {
        int(self.0 % rhs.0)
    }
}

pub trait Integral {}

pub use crate::impl_integral;

/// Implements `From<&T> for int` by delegating to `IBig`'s own `From` impl.
///
/// This is an internal macro — not exported — since it depends on the concrete
/// backing type and should not be part of the public API.
macro_rules! impl_from_integral {
    ($t:ty) => {
        impl From<$t> for int {
            #[inline]
            fn from(val: $t) -> Self {
                int(IBig::from(val))
            }
        }

        impl From<&$t> for int {
            #[inline]
            fn from(val: &$t) -> Self {
                int(IBig::from(*val))
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
#[macro_export]
macro_rules! impl_integral {
    ($t:ty) => {
        impl $crate::arithmetic::Integral for $t {}

        // ------------------------------------------------------------
        // Arithmetic: T <op> int
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
        // Arithmetic: int <op> T
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
        // Arithmetic: &int <op> T and T <op> &int
        // ------------------------------------------------------------

        impl ::core::ops::Add<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $t) -> $crate::arithmetic::int {
                (*self).clone() + rhs
            }
        }

        impl ::core::ops::Sub<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $t) -> $crate::arithmetic::int {
                (*self).clone() - rhs
            }
        }

        impl ::core::ops::Mul<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $t) -> $crate::arithmetic::int {
                (*self).clone() * rhs
            }
        }

        impl ::core::ops::Div<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $t) -> $crate::arithmetic::int {
                (*self).clone() / rhs
            }
        }

        impl ::core::ops::Rem<$t> for &$crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $t) -> $crate::arithmetic::int {
                (*self).clone() % rhs
            }
        }

        impl ::core::ops::Add<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self + (*rhs).clone()
            }
        }

        impl ::core::ops::Sub<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self - (*rhs).clone()
            }
        }

        impl ::core::ops::Mul<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self * (*rhs).clone()
            }
        }

        impl ::core::ops::Div<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self / (*rhs).clone()
            }
        }

        impl ::core::ops::Rem<&$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: &$crate::arithmetic::int) -> $crate::arithmetic::int {
                self % (*rhs).clone()
            }
        }

        // ------------------------------------------------------------
        // Compound assignment: int <assign_op> T
        // ------------------------------------------------------------

        impl ::core::ops::AddAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                *self = (*self).clone() + rhs;
            }
        }

        impl ::core::ops::SubAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                *self = (*self).clone() - rhs;
            }
        }

        impl ::core::ops::MulAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                *self = (*self).clone() * rhs;
            }
        }

        impl ::core::ops::DivAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                *self = (*self).clone() / rhs;
            }
        }

        impl ::core::ops::RemAssign<$t> for $crate::arithmetic::int {
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                *self = (*self).clone() % rhs;
            }
        }
    };
}

macro_rules! impl_integral_cmp_copy {
    ($t:ty) => {
        impl ::core::cmp::PartialEq<$crate::arithmetic::int> for $t {
            #[inline]
            fn eq(&self, other: &$crate::arithmetic::int) -> bool {
                <$crate::arithmetic::int as ::core::convert::From<$t>>::from(*self) == other.clone()
            }
        }

        impl ::core::cmp::PartialEq<$t> for $crate::arithmetic::int {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self == &<$crate::arithmetic::int as ::core::convert::From<$t>>::from(*other)
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
    };
}

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

impl_integral_cmp_copy!(i8);
impl_integral_cmp_copy!(i16);
impl_integral_cmp_copy!(i32);
impl_integral_cmp_copy!(i64);
impl_integral_cmp_copy!(i128);
impl_integral_cmp_copy!(isize);

impl_integral_cmp_copy!(u8);
impl_integral_cmp_copy!(u16);
impl_integral_cmp_copy!(u32);
impl_integral_cmp_copy!(u64);
impl_integral_cmp_copy!(u128);
impl_integral_cmp_copy!(usize);
