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

/// Calls impl_op multiple times with different operations.
macro_rules! impl_ops {
    ($vec:ident, $get:tt, $($op:ident $fn:ident),+) => { $(impl_op!($vec, $get, $op $fn);)+ };
}
/// Implements a single operation for a vector.
macro_rules! impl_op {
    ($vec:ident, ($($get:tt),+), $op:ident $fn:ident) => {
        impl<T: $op<Output = T> + Copy> $op<$vec<T>> for $vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs.$get)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<&$vec<T>> for $vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: &$vec<T>) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs.$get)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<$vec<T>> for &$vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: $vec<T>) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs.$get)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<&$vec<T>> for &$vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: &$vec<T>) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs.$get)),+ }
            }
        }

        impl<T: $op<Output = T>+ Copy> $op<T> for $vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: T) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<&T> for $vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: &T) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(*rhs)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<T> for &$vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: T) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(rhs)),+ }
            }
        }
        impl<T: $op<Output = T> + Copy> $op<&T> for &$vec<T> {
            type Output = $vec<T>;

            #[inline(always)]
            fn $fn(self, rhs: &T) -> Self::Output {
                Self::Output { $($get: self.$get.$fn(*rhs)),+ }
            }
        }
    };
}

/// Calls impl_assign_op multiple times with different assign operations.
macro_rules! impl_assign_ops {
    ($vec:ident, $get:tt, $($op:ident $fn:ident),+) => { $(impl_assign_op!($vec, $get, $op $fn);)+ };
}
/// Implements a single assign operation for a vector.
macro_rules! impl_assign_op {
    ($vec:ident, ($($get:tt),+), $op:ident $fn:ident) => {
        impl<T: $op + Copy> $op<$vec<T>> for $vec<T> {
            #[inline(always)]
            fn $fn(&mut self, rhs: Self) {
                $(self.$get.$fn(rhs.$get);)+
            }
        }
        impl<T: $op + Copy> $op<&$vec<T>> for $vec<T> {
            #[inline(always)]
            fn $fn(&mut self, rhs: &Self) {
                $(self.$get.$fn(rhs.$get);)+
            }
        }

        impl<T: $op + Copy> $op<T> for $vec<T> {
            #[inline(always)]
            fn $fn(&mut self, rhs: T)  {
                $(self.$get.$fn(rhs);)+
            }
        }
        impl<T: $op + Copy> $op<&T> for $vec<T> {
            #[inline(always)]
            fn $fn(&mut self, rhs: &T)  {
                $(self.$get.$fn(*rhs);)+
            }
        }
    };
}

macro_rules! impl_vec {
    ($vec:ident, $size:tt, ($($get:tt),+), ($($index:tt),+), $tuple:tt) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $vec<T> {
            $(pub $get: T),+
        }

        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $vec<T> {}
        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $vec<T> {}

        impl_ops!($vec, ($($get),+), Add add, Sub sub, Mul mul, Div div, Rem rem);
        impl_assign_ops!($vec, ($($get),+), AddAssign add_assign, SubAssign sub_assign, MulAssign mul_assign, DivAssign div_assign, RemAssign rem_assign);

        impl<T> $vec<T> {
            #[inline(always)]
            pub const fn new($($get: T),+) -> Self {
                Self { $($get),+ }
            }
        }

        impl<T: Copy> $vec<T> {
            #[inline(always)]
            pub const fn splat(v: T) -> Self {
                Self { $($get: v),+ }
            }

            #[inline]
            pub fn map<D,F>(self, mut f: F) -> $vec<D> where F: FnMut(T) -> D {
                $vec::new($(f(self.$get)),+)
            }

            pub const fn to_array(&self) -> [T; $size] {
                [$(self.$get),+]
            }
        }


        impl<T: Zero + Copy> $vec<T> {
            pub const ZERO: Self = Self::splat(T::ZERO);
        }
        impl<T: One + Copy> $vec<T> {
            pub const ONE: Self = Self::splat(T::ONE);
        }
        impl<T: NegOne + Copy> $vec<T> {
            pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
        }

        impl<T: PartialOrd> $vec<T> {
            #[inline]
            pub fn min(self, rhs: Self) -> Self {
                Self { $($get: if self.$get < rhs.$get {self.$get} else {rhs.$get}),+ }
            }
            #[inline]
            pub fn max(self, rhs: Self) -> Self {
                Self { $($get: if self.$get > rhs.$get {self.$get} else {rhs.$get}),+ }
            }
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                self.min(max).max(min)
            }

            #[inline]
            pub fn min_element(self) -> T {
                let min = |a, b| if a < b { a } else { b };
                reduce_fn!(min, $(self.$get),+)
            }
            #[inline]
            pub fn max_element(self) -> T {
                let max = |a, b| if a > b { a } else { b };
                reduce_fn!(max, $(self.$get),+)
            }

        }

        impl<T: Mul<Output = T> + Add<Output = T> + Copy> $vec<T> {
            #[inline]
            pub fn dot(self, rhs: Self) -> T {
                (self * rhs).element_sum()
            }

            #[inline]
            pub fn element_sum(self) -> T {
                reduce_op!(+, $(self.$get),+)
            }

            #[inline]
            pub fn element_product(self) -> T {
                reduce_op!(*, $(self.$get),+)
            }

            #[inline]
            pub fn length_squared(self) -> T {
                self.dot(self)
            }
        }

        impl<T> $vec<T>
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

        impl<T: Neg<Output = T>> Neg for $vec<T> {
            type Output = $vec<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output { $($get: -self.$get),+ }
            }
        }
        impl<T: Neg<Output = T> + Copy> Neg for &$vec<T> {
            type Output = $vec<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::Output { $($get: -self.$get),+ }
            }
        }

        impl<T> Index<usize> for $vec<T> {
            type Output = T;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $($index => &self.$get),+,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl<T> IndexMut<usize> for $vec<T> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $($index => &mut self.$get),+,
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl<T: Copy> From<T> for $vec<T> {
            #[inline(always)]
            fn from(value: T) -> Self {
                Self::splat(value)
            }
        }

        impl<T: Copy> From<[T; $size]> for $vec<T> {
            #[inline(always)]
            fn from(value: [T; $size]) -> Self {
                Self::new($(value[$index]),+)
            }
        }

        impl<T> From<$tuple> for $vec<T> {
            #[inline(always)]
            fn from(value: $tuple) -> Self {
                Self::new($(value.$index),+)
            }
        }
        impl<T> From<$vec<T>> for [T; $size] {
            #[inline(always)]
            fn from(value: $vec<T>) -> Self {
                [$(value.$get),+]
            }
        }
    };
}

impl_vec!(Vec2, 2, (x, y), (0, 1), (T, T));
impl_vec!(Vec3, 3, (x, y, z), (0, 1, 2), (T, T, T));
impl_vec!(Vec4, 4, (x, y, z, w), (0, 1, 2, 3), (T, T, T, T));

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Vec3<T> {
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
    ($vec:ident<$type:ident>, $get:tt, $($op:ident $fn:ident),+) => { $(impl_prim_op!($vec<$type>, $get, $op $fn);)+ };
}
macro_rules! impl_prim_op {
    ($vec:ident<$type:ident>, ($($get:tt),+), $op:ident $fn:ident) => {
        impl $op<$vec<$type>> for $type {
            type Output = $vec<$type>;

            #[inline(always)]
            fn $fn(self, rhs: $vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$fn(rhs.$get)),+ }
            }
        }
        impl $op<&$vec<$type>> for $type {
            type Output = $vec<$type>;

            #[inline(always)]
            fn $fn(self, rhs: &$vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$fn(rhs.$get)),+ }
            }
        }
        impl $op<$vec<$type>> for &$type {
            type Output = $vec<$type>;

            #[inline(always)]
            fn $fn(self, rhs: $vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$fn(rhs.$get)),+ }
            }
        }
        impl $op<&$vec<$type>> for &$type {
            type Output = $vec<$type>;

            #[inline(always)]
            fn $fn(self, rhs: &$vec<$type>) -> Self::Output {
                Self::Output { $($get: self.$fn(rhs.$get)),+ }
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
    ($vec:ident, $get:tt, $($type:ident)+) => {
        $(
            impl_prim_ops!($vec<$type>, $get, Add add, Sub sub, Mul mul, Div div, Rem rem);
        )+
    };
}

impl_prim!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);
