use crate::num::prelude::*;
use crate::vec::*;
use core::ops::*;

macro_rules! impl_op {
    (impl $Op:ident for $Mat:ident $Vec:ident { $op:ident } ($($get:tt),+)) => {
        impl<T> $Op<T> for $Mat<T>
            where T: $Op<Output = T> + Copy
        {
            type Output = $Mat<T>;
            fn $op(self, rhs: T) -> Self::Output {
                Self::Output {
                    $($get: self.$get.$op(rhs)),+
                }
            }
        }

        impl<T> $Op<$Vec<T>> for $Mat<T>
            where T: $Op<Output = T> + Copy
        {
            type Output = $Mat<T>;
            fn $op(self, rhs: $Vec<T>) -> Self::Output {
                Self::Output {
                    $($get: self.$get.$op(rhs)),+
                }
            }
        }


        impl<T> $Op<$Mat<T>> for $Mat<T>
            where T: $Op<Output = T> + Copy
        {
            type Output = $Mat<T>;
            fn $op(self, rhs: $Mat<T>) -> Self::Output {
                Self::Output {
                    $($get: self.$get.$op(rhs.$get)),+
                }
            }
        }
    };
}

macro_rules! impl_assign_op {
    (impl $Op:ident for $Mat:ident $Vec:ident { $op:ident } ($($get:tt),+)) => {
        // NOTE: Reminder that scalars T: Copy also implement Into<$Vec<T>>.
        impl<V, T> $Op<V> for $Mat<T> where V: Into<$Mat<T>>, T: $Op<T> {
            #[inline]
            fn $op(&mut self, rhs: V) {
                let rhs = rhs.into();
                $(self.$get.$op(rhs.$get);)+
            }
        }
    };
}

macro_rules! impl_mat {
    ($Mat:ident $Vec:ident ($($get:tt),+)) => {
        #[derive(Debug, Default, Clone, Hash, PartialEq)]
        pub struct $Mat<T> {
            $(pub $get: $Vec<T>),+
        }

        impl<T: Copy> Copy for $Mat<T> {}

        impl<T: Eq> Eq for $Mat<T> {}

        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $Mat<T> {}
        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $Mat<T> {}

        impl<T: Zero> $Mat<T> {
            pub const ZERO: Self = Self { $($get: $Vec::ZERO),+ };
        }
        impl<T: One> $Mat<T> {
            pub const ONE: Self = Self { $($get: $Vec::ONE),+ };
        }
        impl<T: NegOne> $Mat<T> {
            pub const NEG_ONE: Self = Self { $($get: $Vec::NEG_ONE),+ };
        }
        impl<T: Zero + One + Copy> $Mat<T> {
            pub const IDENTITY: Self = Self::from_diagonal($Vec::ONE);
        }

        impl_op!(impl Add for $Mat $Vec { add } ($($get),+));
        impl_op!(impl Sub for $Mat $Vec { sub } ($($get),+));
        impl_op!(impl Mul for $Mat $Vec { mul } ($($get),+));
        impl_op!(impl Div for $Mat $Vec { div } ($($get),+));
        impl_op!(impl Rem for $Mat $Vec { rem } ($($get),+));
        impl_assign_op!(impl AddAssign for $Mat $Vec { add_assign } ($($get),+));
        impl_assign_op!(impl SubAssign for $Mat $Vec { sub_assign } ($($get),+));
        impl_assign_op!(impl MulAssign for $Mat $Vec { mul_assign } ($($get),+));
        impl_assign_op!(impl DivAssign for $Mat $Vec { div_assign } ($($get),+));
        impl_assign_op!(impl RemAssign for $Mat $Vec { rem_assign } ($($get),+));

        impl<T> $Mat<T> {
            pub const fn from_cols($($get: $Vec<T>),+) -> Self {
                Self {
                    $($get),+
                }
            }

            pub const fn from_diagonal(diagonal: $Vec<T>) -> Self where T: Zero + Copy{
                let mut out = Self::ZERO;
                $(out.$get.$get = diagonal.$get;)+
                out
            }

            pub fn map<D,F>(self, mut f: F) -> $Mat<D> where F: FnMut(T) -> D {
                $Mat {
                    $($get: self.$get.map(&mut f)),+
                }
            }

            pub fn apply<F>(&mut self, f: F) where T: Copy, F: FnMut(T) -> T {
                *self = self.map(f);
            }
        }
    };
}

impl_mat!(Mat2 Vec2 (x, y));
impl_mat!(Mat3 Vec3 (x, y, z));
impl_mat!(Mat4 Vec4 (x, y, z, w));

impl<T> Mat2<T> {
    pub const fn new(m00: T, m01: T, m10: T, m11: T) -> Self {
        Self {
            x: Vec2::new(m00, m01),
            y: Vec2::new(m10, m11),
        }
    }
}
impl<T> Mat3<T> {
    pub const fn new(
        m00: T,
        m01: T,
        m02: T,
        m10: T,
        m11: T,
        m12: T,
        m20: T,
        m21: T,
        m22: T,
    ) -> Self {
        Self {
            x: Vec3::new(m00, m01, m02),
            y: Vec3::new(m10, m11, m12),
            z: Vec3::new(m20, m21, m22),
        }
    }
}
impl<T> Mat4<T> {
    pub const fn new(
        m00: T,
        m01: T,
        m02: T,
        m03: T,
        m10: T,
        m11: T,
        m12: T,
        m13: T,
        m20: T,
        m21: T,
        m22: T,
        m23: T,
        m30: T,
        m31: T,
        m32: T,
        m33: T,
    ) -> Self {
        Self {
            x: Vec4::new(m00, m01, m02, m03),
            y: Vec4::new(m10, m11, m12, m13),
            z: Vec4::new(m20, m21, m22, m23),
            w: Vec4::new(m30, m31, m32, m33),
        }
    }
}
