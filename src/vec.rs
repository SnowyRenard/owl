use crate::num::prelude::*;
use core::ops::*;

macro_rules! reduce_fn {
    ($fn:expr, $a:expr, $b:expr) => { $fn($a, $b) };
    ($fn:expr, $a:expr, $b:expr, $($v:expr),+) => { reduce_fn!($fn, reduce_fn!($fn, $a, $b), $($v),+) };
}
macro_rules! reduce_op {
    ($op:tt, $a:expr, $b:expr) => { $a $op $b };
    ($op:tt, $a:expr, $b:expr, $($v:expr),+) => { reduce_op!($op, reduce_op!($op, $a, $b), $($v),+) };
}

/// Implements a single operation for a vector.
macro_rules! impl_op {
    (impl $Op:ident for $Vec:ident { $op:ident } ($($get:tt),+)) => {
        // NOTE: Reminder that scalars T: Copy also implement Into<$Vec<T>>.
        impl<V, T> $Op<V> for $Vec<T> where V: Into<$Vec<T>>, T: $Op<T, Output = T> {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: V) -> Self::Output {
                let rhs = rhs.into();
                $Vec::new($(self.$get.$op(rhs.$get)),+)
            }
        }
        impl<'a, T> $Op<&'a $Vec<T>> for $Vec<T> where T: $Op<&'a T, Output = T> {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: &'a $Vec<T>) -> Self::Output {
                $Vec::new($(self.$get.$op(&rhs.$get)),+)
            }
        }
        impl<'a, T> $Op<$Vec<T>> for &'a $Vec<T> where &'a T: $Op<T, Output = T> {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: $Vec<T>) -> Self::Output {
                $Vec::new($(self.$get.$op(rhs.$get)),+)
            }
        }
        impl<'a, 'b, T> $Op<&'a $Vec<T>> for &'b $Vec<T> where &'b T: $Op<&'a T, Output = T> {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: &'a $Vec<T>) -> Self::Output {
                Self::Output { $($get: self.$get.$op(&rhs.$get)),+ }
            }
        }

        impl<'a, T> $Op<T> for &'a $Vec<T> where &'a T: $Op<T, Output = T>, T: Copy {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: T) -> Self::Output {
                Self::Output { $($get: self.$get.$op(rhs)),+ }
            }
        }
        impl<'a, 'b, T> $Op<&'a T> for &'b $Vec<T> where &'b T: $Op<&'a T, Output = T> {
            type Output = $Vec<T>;

            #[inline]
            fn $op(self, rhs: &'a T) -> Self::Output {
                Self::Output { $($get: self.$get.$op(rhs)),+ }
            }
        }
    };
}

/// Implements a single assign operation for a vector.
macro_rules! impl_assign_op {
    (impl $Op:ident for $Vec:ident { $op:ident } ($($get:tt),+)) => {
        // NOTE: Reminder that scalars T: Copy also implement Into<$Vec<T>>.
        impl<V, T> $Op<V> for $Vec<T> where V: Into<$Vec<T>>, T: $Op<T> {
            #[inline]
            fn $op(&mut self, rhs: V) {
                let rhs = rhs.into();
                $(self.$get.$op(rhs.$get);)+
            }
        }
    };
}

macro_rules! impl_vec {
    ($Vec:ident, $size:tt, ($($get:tt),+), ($($index:tt),+), $tuple:tt) => {
        #[derive(Debug, Default, Clone, Hash, PartialEq)]
        pub struct $Vec<T> {
            $(pub $get: T),+
        }

        impl<T: Copy> Copy for $Vec<T> {}

        impl<T: Eq> Eq for $Vec<T> {}

        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $Vec<T> {}
        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $Vec<T> {}

        impl_op!(impl Add for $Vec { add } ($($get),+));
        impl_op!(impl Sub for $Vec { sub } ($($get),+));
        impl_op!(impl Mul for $Vec { mul } ($($get),+));
        impl_op!(impl Div for $Vec { div } ($($get),+));
        impl_op!(impl Rem for $Vec { rem } ($($get),+));
        impl_assign_op!(impl AddAssign for $Vec { add_assign } ($($get),+));
        impl_assign_op!(impl SubAssign for $Vec { sub_assign } ($($get),+));
        impl_assign_op!(impl MulAssign for $Vec { mul_assign } ($($get),+));
        impl_assign_op!(impl DivAssign for $Vec { div_assign } ($($get),+));
        impl_assign_op!(impl RemAssign for $Vec { rem_assign } ($($get),+));

        impl<T> $Vec<T> {
            #[inline]
            pub const fn new($($get: T),+) -> Self {
                Self { $($get),+ }
            }
            #[inline]
            pub const fn splat(v: T) -> Self where T: Copy {
                Self { $($get: v),+ }
            }

            /// Constant representing the number of elements for this vector type.
            pub const ELEM_COUNT:usize = $size;

            /// Converts this into a tuple with the same number of elements by consuming.
            pub fn into_tuple(self) -> $tuple {
                ($(self.$get),+)
            }
            /// Converts this into a fixed-size array.
            pub fn into_array(self) -> [T; $size] {
                [$(self.$get),+]
            }

            /// Returns a member wise-converted copy of this vector, using the given conversion
            /// closure.
            ///
            /// ```
            /// # use owl::Vec4;
            /// let v = Vec4::new(0_f32, 1., 1.8, 3.14);
            /// let i = v.map(|x| x.round() as i32);
            /// assert_eq!(i, Vec4::new(0, 1, 2, 3));
            /// ```
            #[inline]
            pub fn map<D,F>(self, mut f: F) -> $Vec<D> where F: FnMut(T) -> D {
                $Vec::new($(f(self.$get)),+)
            }

            /// Applies the function f to each element of this vector, in-place.
            ///
            /// ```
            /// # use owl::Vec4;
            /// let mut v = Vec4::new(0_u32, 1, 2, 3);
            /// v.apply(|x| x.count_ones());
            /// assert_eq!(v, Vec4::new(0, 1, 1, 2));
            /// ```
            #[inline]
            pub fn apply<F>(&mut self, mut f: F) where T: Copy, F: FnMut(T) -> T {
                $(self.$get = f(self.$get);)+
            }

            /// Returns the sum of all elements of `self`.
            ///
            /// In other words, this computes `self.x + self.y + ...`.
            #[inline]
            pub fn element_sum(self) -> T where T: Add<Output = T> {
                reduce_op!(+, $(self.$get),+)
            }
            /// Returns the product of all elements of `self`.
            ///
            /// In other words, this computes `self.x * self.y * ...`.
            #[inline]
            pub fn element_product(self) -> T where T: Mul<Output = T> {
                reduce_op!(*, $(self.$get),+)
            }

            /// Computes the dot product of `self` and `rhs`.
            #[inline]
            pub fn dot(self, rhs: Self) -> T where T: Add<Output = T> + Mul<Output = T> {
                (self * rhs).element_sum()
            }
            /// The squared length of a vector in its spatial length.
            /// It is slightly cheaper to compute then `length` because it avoids a square root.
            #[inline]
            pub fn length_squared(self) -> T where T: Add<Output = T> + Mul<Output = T> + Copy {
                self.dot(self)
            }
        }

        impl<T: Zero> $Vec<T> {
            pub const ZERO: Self = Self { $($get: T::ZERO),+ };
        }
        impl<T: One> $Vec<T> {
            pub const ONE: Self = Self { $($get: T::ONE),+ };
        }
        impl<T: NegOne> $Vec<T> {
            pub const NEG_ONE: Self = Self { $($get: T::NEG_ONE),+ };
        }

        impl<T: PartialOrd> $Vec<T> {
            /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
            ///
            /// In other words, this computes `[min(self.x, rhs.x), min(self.y, rhs.y), ...]`.
            #[inline]
            pub fn min(self, rhs: Self) -> Self {
                Self { $($get: if self.$get < rhs.$get {self.$get} else {rhs.$get}),+ }
            }
            /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
            ///
            /// In other words, this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ...]`.
            #[inline]
            pub fn max(self, rhs: Self) -> Self {
                Self { $($get: if self.$get > rhs.$get {self.$get} else {rhs.$get}),+ }
            }
            /// Component-wise clamping of values similar to [`f32::clamp`].
            ///
            /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                self.min(max).max(min)
            }

            /// Returns the horizontal minimum of `self`.
            ///
            /// In other words, this computes `min(x, y, ...)`.
            #[inline]
            pub fn min_element(self) -> T {
                let min = |a, b| if a < b { a } else { b };
                reduce_fn!(min, $(self.$get),+)
            }
            /// Returns the horizontal maximum of `self`.
            ///
            /// In other words, this computes `max(x, y, ...)`.
            #[inline]
            pub fn max_element(self) -> T {
                let max = |a, b| if a > b { a } else { b };
                reduce_fn!(max, $(self.$get),+)
            }

        }


        impl<T> $Vec<T>
            where T: Float + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy
        {
            #[inline]
            #[must_use]
            pub fn length(self) -> T {
                self.length_squared().sqrt()
            }

            #[inline]
            #[must_use]
            pub fn normalize(self) -> Self {
                self / self.length()
            }


        }

        impl<T: Float> $Vec<T> {
            pub fn floor(self) -> Self {
                Self { $($get: self.$get.floor()),+ }
            }
            pub fn ceil(self) -> Self {
                Self { $($get: self.$get.ceil()),+ }
            }
            pub fn round(self) -> Self {
                Self { $($get: self.$get.round()),+ }
            }
            pub fn trunc(self) -> Self {
                Self { $($get: self.$get.trunc()),+ }
            }
            pub fn fract(self) -> Self {
                Self { $($get: self.$get.fract()),+ }
            }
        }

        impl<T: Neg<Output = T>> Neg for $Vec<T> {
            type Output = $Vec<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output { $($get: -self.$get),+ }
            }
        }

        impl<T> Index<usize> for $Vec<T> {
            type Output = T;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($index => &self.$get),+,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl<T> IndexMut<usize> for $Vec<T> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($index => &mut self.$get),+,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl<T: Copy> From<T> for $Vec<T> {
            #[inline]
            fn from(value: T) -> Self {
                Self::splat(value)
            }
        }

        impl<T: Copy> From<[T; $size]> for $Vec<T> {
            #[inline]
            fn from(value: [T; $size]) -> Self {
                Self::new($(value[$index]),+)
            }
        }
        impl<T> From<$tuple> for $Vec<T> {
            #[inline]
            fn from(value: $tuple) -> Self {
                Self::new($(value.$index),+)
            }
        }

        impl<T> From<$Vec<T>> for [T; $size] {
            #[inline]
            fn from(value: $Vec<T>) -> Self {
                value.into_array()
            }
        }
        impl<T> From<$Vec<T>> for $tuple {
            #[inline]
            fn from(value: $Vec<T>) -> Self {
                value.into_tuple()
            }
        }
    };
}

impl_vec!(Vec2, 2, (x, y), (0, 1), (T, T));
impl_vec!(Vec3, 3, (x, y, z), (0, 1, 2), (T, T, T));
impl_vec!(Vec4, 4, (x, y, z, w), (0, 1, 2, 3), (T, T, T, T));

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Vec3<T> {
    /// Computes the cross product of `self` and `rhs`.
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
}

impl<T: Cast<u8> + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Vec3<T> {
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * T::cast(2) * self.dot(normal)
    }
}

impl<T: Float + Zero + One + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Vec3<T> {
    pub fn refract(self, normal: Self, eta: T) -> Self {
        let n_dot_i = normal.dot(self);
        let k = T::ONE - eta * eta * (T::ONE - n_dot_i * n_dot_i);
        if k >= T::ZERO {
            self * eta - normal * (eta * n_dot_i + T::sqrt(k))
        } else {
            Self::ZERO
        }
    }
}

// ====== //
// CONSTS //
// ====== //
impl<T: One + Zero> Vec2<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE);
}
impl<T: NegOne + Zero> Vec2<T> {
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO);
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE);
}
impl<T: One + Zero> Vec3<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}
impl<T: NegOne + Zero> Vec3<T> {
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO);
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO);
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE);
}
impl<T: One + Zero> Vec4<T> {
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}
impl<T: NegOne + Zero> Vec4<T> {
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO);
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO);
    pub const NEG_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE);
}

macro_rules! impl_prim_ops {
    ($Vec:ident<$type:ident>, $get:tt, $($Op:ident $op:ident),+) => { $(impl_prim_op!($Vec<$type>, $get, $Op $op);)+ };
}
macro_rules! impl_prim_op {
    ($Vec:ident<$type:ident>, ($($get:tt),+), $Op:ident $op:ident) => {
        impl $Op<$Vec<$type>> for $type {
            type Output = $Vec<$type>;

            #[inline]
            fn $op(self, rhs: $Vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$op(rhs.$get)),+ }
            }
        }
        impl $Op<&$Vec<$type>> for $type {
            type Output = $Vec<$type>;

            #[inline]
            fn $op(self, rhs: &$Vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$op(rhs.$get)),+ }
            }
        }
        impl $Op<$Vec<$type>> for &$type {
            type Output = $Vec<$type>;

            #[inline]
            fn $op(self, rhs: $Vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$op(rhs.$get)),+ }
            }
        }
        impl $Op<&$Vec<$type>> for &$type {
            type Output = $Vec<$type>;

            #[inline]
            fn $op(self, rhs: &$Vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$op(rhs.$get)),+ }
            }
        }
    };
}

macro_rules! impl_prim {
    ($($type:ident)+) => {
        impl_prim!(Vec2, (x, y), $($type)+);
        impl_prim!(Vec3, (x, y, z), $($type)+);
        impl_prim!(Vec4, (x, y, z, w), $($type)+);
    };
    ($Vec:ident, $get:tt, $($type:ident)+) => {
        $(
            impl_prim_ops!($Vec<$type>, $get, Add add, Sub sub, Mul mul, Div div, Rem rem);
        )+
    };
}

impl_prim!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);

#[macro_export]
macro_rules! swizzle {
    ($Vec:expr => $($get:tt),+ => $Ret:ty) => {
        <$Ret>::new($($Vec.$get),+)
    };
}

#[cfg(test)]
mod swizzle_test {
    use super::*;

    // Self Swizzle
    #[test]
    fn vec2_swizzle() {
        let result = swizzle!(Vec2::new(1,2) => y,x => Vec2<i32>);
        assert_eq!(result, Vec2::new(2, 1));
    }
    #[test]
    fn vec3_swizzle() {
        let result = swizzle!(Vec3::new(1,2,3) => z,y,x => Vec3<i32>);
        assert_eq!(result, Vec3::new(3, 2, 1));
    }
    #[test]
    fn vec4_swizzle() {
        let result = swizzle!(Vec4::new(1,2,3,4) => w,z,y,x => Vec4<i32>);
        assert_eq!(result, Vec4::new(4, 3, 2, 1));
    }

    // Up Swizzle
    #[test]
    fn vec2_up_swizzle() {
        let result = swizzle!(Vec2::new(1,2) => x,y,x => Vec3<i32>);
        assert_eq!(result, Vec3::new(1, 2, 1))
    }
    #[test]
    fn vec3_up_swizzle() {
        let result = swizzle!(Vec3::new(1,2,3) => x,y,z,x => Vec4<i32>);
        assert_eq!(result, Vec4::new(1, 2, 3, 1))
    }
    #[test]
    fn full_up_swizzle() {
        let result = swizzle!(Vec2::new(1, 2) => x,y,x,y => Vec4<i32>);
        assert_eq!(result, Vec4::new(1, 2, 1, 2));
    }

    // Down Swizzle
    #[test]
    fn vec3_down_swizzle() {
        let result = swizzle!(Vec3::new(1, 2, 3) => x,y => Vec2<i32>);
        assert_eq!(result, Vec2::new(1, 2))
    }
    #[test]
    fn vec4_down_swizzle() {
        let result = swizzle!(Vec4::new(1, 2, 3, 4) => x,y,z => Vec3<i32>);
        assert_eq!(result, Vec3::new(1, 2, 3))
    }
    #[test]
    fn full_down_swizzle() {
        let result = swizzle!(Vec4::new(1, 2, 3, 4) => x,y => Vec2<i32>);
        assert_eq!(result, Vec2::new(1, 2))
    }
}
