/// Implements all integration points for primitive integral types:
/// - `From<T>` for [`int`]
/// - the [`Primitive`] marker trait
/// - arithmetic/comparison interop between `T` and [`int`]
///
/// This macro is intentionally internal and not part of the public API.
macro_rules! impl_primitive_interop {
    ($t:ty) => {
        impl From<$t> for int {
            #[inline]
            fn from(val: $t) -> Self {
                int(BigInt::from(val))
            }
        }

        impl $crate::arithmetic::Primitive for $t {}

        // ------------------------------------------------------------
        // Arithmetic: T <op> int
        // ------------------------------------------------------------

        impl ::core::ops::Add<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                $crate::arithmetic::int::from(self) + rhs
            }
        }

        impl ::core::ops::Sub<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                $crate::arithmetic::int::from(self) - rhs
            }
        }

        impl ::core::ops::Mul<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                $crate::arithmetic::int::from(self) * rhs
            }
        }

        impl ::core::ops::Div<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                $crate::arithmetic::int::from(self) / rhs
            }
        }

        impl ::core::ops::Rem<$crate::arithmetic::int> for $t {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $crate::arithmetic::int) -> $crate::arithmetic::int {
                $crate::arithmetic::int::from(self) % rhs
            }
        }

        // ------------------------------------------------------------
        // Arithmetic: int <op> T
        // ------------------------------------------------------------

        impl ::core::ops::Add<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn add(self, rhs: $t) -> $crate::arithmetic::int {
                self + $crate::arithmetic::int::from(rhs)
            }
        }

        impl ::core::ops::Sub<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn sub(self, rhs: $t) -> $crate::arithmetic::int {
                self - $crate::arithmetic::int::from(rhs)
            }
        }

        impl ::core::ops::Mul<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn mul(self, rhs: $t) -> $crate::arithmetic::int {
                self * $crate::arithmetic::int::from(rhs)
            }
        }

        impl ::core::ops::Div<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn div(self, rhs: $t) -> $crate::arithmetic::int {
                self / $crate::arithmetic::int::from(rhs)
            }
        }

        impl ::core::ops::Rem<$t> for $crate::arithmetic::int {
            type Output = $crate::arithmetic::int;

            #[inline]
            fn rem(self, rhs: $t) -> $crate::arithmetic::int {
                self % $crate::arithmetic::int::from(rhs)
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
        // Comparison: T with int (both directions)
        // ------------------------------------------------------------

        impl ::core::cmp::PartialEq<$crate::arithmetic::int> for $t {
            #[inline]
            fn eq(&self, other: &$crate::arithmetic::int) -> bool {
                other == &$crate::arithmetic::int::from(*self)
            }
        }

        impl ::core::cmp::PartialEq<$t> for $crate::arithmetic::int {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self == &$crate::arithmetic::int::from(*other)
            }
        }

        impl ::core::cmp::PartialOrd<$crate::arithmetic::int> for $t {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$crate::arithmetic::int,
            ) -> Option<::core::cmp::Ordering> {
                let lhs = $crate::arithmetic::int::from(*self);
                lhs.partial_cmp(other)
            }
        }

        impl ::core::cmp::PartialOrd<$t> for $crate::arithmetic::int {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<::core::cmp::Ordering> {
                let rhs = $crate::arithmetic::int::from(*other);
                self.partial_cmp(&rhs)
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

pub(crate) use impl_primitive_interop;
