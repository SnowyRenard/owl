use crate::math::Math;

use core::ops::*;

pub type Rg = Vec2;
pub type Uv = Vec2;
pub type Point2 = Vec2;

macro_rules! impl_signed {
    ($(($vec2: ident => $type: ident)),+) => {
        $(
            impl $vec2 {
                pub const NEG_ONE: Self = Self::splat(-1 as $type);

                pub const NEG_X: Self = Self::new(-1 as $type, 0 as $type);
                pub const NEG_Y: Self = Self::new(0 as $type, -1 as $type);

                #[inline]
                #[must_use]
                pub fn abs(self) -> Self {
                    Self {
                        x: self.x.abs(),
                        y: self.y.abs(),
                    }
                }
            }

            impl Neg for $vec2 {
                type Output = $vec2;

                #[inline]
                fn neg(self) -> Self::Output {
                    Self::Output {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
            impl Neg for &$vec2 {
                type Output = $vec2;

                #[inline]
                fn neg(self) -> Self::Output {
                    Self::Output {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
        )+
    };
}

macro_rules! impl_float {
    ($(($vec2: ident => $type: ident)), +) => {
        $(
            impl_signed!(($vec2 => $type));
            impl $vec2 {
                pub const INFINITY: Self = Self::splat(<$type>::INFINITY);
                pub const NEG_INFINITY: Self = Self::splat(<$type>::NEG_INFINITY);

                pub const NAN: Self = Self::splat(<$type>::NAN);

                #[inline]
                #[must_use]
                pub fn round(self) -> Self {
                    Self {
                        x: Math::round(self.x),
                        y: Math::round(self.y),
                    }
                }
                #[inline]
                #[must_use]
                pub fn floor(self) -> Self {
                    Self {
                        x: Math::floor(self.x),
                        y: Math::floor(self.y),
                    }
                }
                #[inline]
                #[must_use]
                pub fn ceil(self) -> Self {
                    Self {
                        x: Math::ceil(self.x),
                        y: Math::ceil(self.y),
                    }
                }
                #[inline]
                #[must_use]
                pub fn trunc(self) -> Self {
                    Self {
                        x: Math::trunc(self.x),
                        y: Math::trunc(self.y),
                    }
                }
                #[inline]
                #[must_use]
                pub fn fract(self) -> Self {
                    Self {
                        x: Math::fract(self.x),
                        y: Math::fract(self.y),
                    }
                }

                #[inline]
                #[must_use]
                pub fn length(self) -> $type {
                    Math::sqrt(self.length_squared())
                }
                #[inline]
                #[must_use]
                pub fn length_squared(self) -> $type {
                    self.dot(self)
                }

                #[inline]
                #[must_use]
                pub fn normalize(self) -> Self {
                    self / self.length()
                }

                #[inline]
                #[must_use]
                pub fn reflect(self, normal: Self) -> Self {
                    self - 2 as $type * self.dot(normal) * normal
                }
                #[inline]
                #[must_use]
                pub fn refract(self, normal: Self, eta: $type) -> Self {
                    let n_dot_i = normal.dot(self);
                    let k = 1 as $type - eta * eta * (1 as $type - n_dot_i * n_dot_i);
                    if k >= 0 as $type {
                        eta * self - (eta * n_dot_i + Math::sqrt(k)) * normal
                    } else {
                        Self::ZERO
                    }
                }
            }
        )+
    };
}

macro_rules! impl_op {
    ($op: ident, $fn: ident, $vec2: ident, $type: ident) => {
        // Vec2 x Vec2
        // ---------
        impl $op<$vec2> for $vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $vec2) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                }
            }
        }
        impl $op<&$vec2> for $vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$vec2) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                }
            }
        }
        impl $op<$vec2> for &$vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $vec2) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                }
            }
        }
        impl $op<&$vec2> for &$vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$vec2) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs.x),
                    y: self.y.$fn(rhs.y),
                }
            }
        }

        // Vec2 x type
        // ----------
        impl $op<$type> for $vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $type) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                }
            }
        }
        impl $op<&$type> for $vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$type) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                }
            }
        }
        impl $op<$type> for &$vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $type) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                }
            }
        }
        impl $op<&$type> for &$vec2 {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$type) -> Self::Output {
                Self::Output {
                    x: self.x.$fn(rhs),
                    y: self.y.$fn(rhs),
                }
            }
        }

        // type x Vec2
        // ----------
        impl $op<$vec2> for $type {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $vec2) -> Self::Output {
                Self::Output {
                    x: self.$fn(rhs.x),
                    y: self.$fn(rhs.y),
                }
            }
        }
        impl $op<&$vec2> for $type {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$vec2) -> Self::Output {
                Self::Output {
                    x: self.$fn(rhs.x),
                    y: self.$fn(rhs.y),
                }
            }
        }
        impl $op<$vec2> for &$type {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: $vec2) -> Self::Output {
                Self::Output {
                    x: self.$fn(rhs.x),
                    y: self.$fn(rhs.y),
                }
            }
        }
        impl $op<&$vec2> for &$type {
            type Output = $vec2;

            #[inline]
            fn $fn(self, rhs: &$vec2) -> Self::Output {
                Self::Output {
                    x: self.$fn(rhs.x),
                    y: self.$fn(rhs.y),
                }
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($op: ident, $fn: ident, $vec2: ident, $type: ident) => {
        // Vec2 x Vec2
        // ---------
        impl $op<$vec2> for $vec2 {
            #[inline]
            fn $fn(&mut self, rhs: $vec2) {
                self.x.$fn(rhs.x);
                self.y.$fn(rhs.y);
            }
        }
        impl $op<&$vec2> for $vec2 {
            #[inline]
            fn $fn(&mut self, rhs: &$vec2) {
                self.x.$fn(rhs.x);
                self.y.$fn(rhs.y);
            }
        }

        // Vec2 x type
        // ----------
        impl $op<$type> for $vec2 {
            #[inline]
            fn $fn(&mut self, rhs: $type) {
                self.x.$fn(rhs);
                self.y.$fn(rhs);
            }
        }
        impl $op<&$type> for $vec2 {
            #[inline]
            fn $fn(&mut self, rhs: &$type) {
                self.x.$fn(rhs);
                self.y.$fn(rhs);
            }
        }
    };
}

macro_rules! vec2s {
    ($(($name:ident => $type:ident)), +) => {
        $(
            impl_op!(Add, add, $name, $type);
            impl_op!(Sub, sub, $name, $type);
            impl_op!(Mul, mul, $name, $type);
            impl_op!(Div, div, $name, $type);

            impl_op_assign!(AddAssign, add_assign, $name, $type);
            impl_op_assign!(SubAssign, sub_assign, $name, $type);
            impl_op_assign!(MulAssign, mul_assign, $name, $type);
            impl_op_assign!(DivAssign, div_assign, $name, $type);

            #[cfg(feature = "bytemuck")]
            unsafe impl bytemuck::Pod for $name {}
            #[cfg(feature = "bytemuck")]
            unsafe impl bytemuck::Zeroable for $name {}
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct $name {
                pub x: $type,
                pub y: $type,
            }


            impl $name {
                pub const ZERO: Self = Self::splat(0 as $type);
                pub const ONE: Self = Self::splat(1 as $type);

                pub const X: Self = Self::new(1 as $type, 0 as $type);
                pub const Y: Self = Self::new(0 as $type, 1 as $type);

                pub const MIN: Self = Self::splat(<$type>::MIN);
                pub const MAX: Self = Self::splat(<$type>::MAX);

                #[inline]
                #[must_use]
                pub const fn new(x: $type, y: $type,) -> Self {
                    Self {x, y}
                }

                #[inline]
                #[must_use]
                pub const fn splat(value: $type) -> Self {
                    Self {x: value, y: value}
                }

                #[inline]
                #[must_use]
                pub fn map<F: Fn($type) -> $type>(self, f:F) -> Self {
                    Self {
                        x: f(self.x),
                        y: f(self.y),
                    }
                }

                #[inline]
                #[must_use]
                pub fn dot(self, other: Self) -> $type {
                    (self.x * other.x) + (self.y * other.y)
                }
                // #[inline]
                // #[must_use]
                // pub fn cross(self, other: Self) -> Self {
                //     Self {
                //         x: self.y * other.z - other.y * self.z,
                //         y: self.z * other.x - other.z * self.x,
                //         z: self.x * other.y - other.x * self.y,
                //     }
                // }

                #[inline]
                #[must_use]
                pub fn min(self, other: Self) -> Self {
                    Self {
                        x: if self.x < other.x { self.x } else { other.x },
                        y: if self.y < other.y { self.y } else { other.y },
                    }
                }
                #[inline]
                #[must_use]
                pub fn max(self, other: Self) -> Self {
                    Self {
                        x: if self.x > other.x { self.x } else { other.x },
                        y: if self.y > other.y { self.y } else { other.y },
                    }
                }

                #[inline]
                #[must_use]
                pub fn clamp(self, min: Self, max: Self) -> Self {
                    self.min(max).max(min)
                }

                #[inline]
                #[must_use]
                pub fn min_element(self) -> $type {
                    let min = |a, b| if a < b { a } else { b };
                    min(self.x, self.y)
                }
                #[inline]
                #[must_use]
                pub fn max_element(self) -> $type {
                    let max = |a, b| if a > b { a } else { b };
                    max(self.x, self.y)
                }

                #[inline]
                #[must_use]
                pub fn element_sum(self) -> $type {
                    self.x + self.y
                }
                #[inline]
                #[must_use]
                pub fn element_product(self) -> $type {
                    self.x * self.y
                }
            }

            impl Index<usize> for $name {
                type Output = $type;

                #[inline]
                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        _ => panic!("index out of bounds"),
                    }
                }
            }
            impl IndexMut<usize> for $name {
                #[inline]
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        _ => panic!("index out of bounds"),
                    }
                }
            }

            impl From<$type> for $name {
                #[inline]
                fn from(value: $type) -> Self {
                    Self::splat(value)
                }
            }
            impl From<&$type> for $name {
                #[inline]
                fn from(value: &$type) -> Self {
                    Self::splat(*value)
                }
            }

            impl From<[$type; 2]> for $name {
                #[inline]
                fn from(value: [$type; 2]) -> Self {
                    Self {
                        x: value[0],
                        y: value[1],
                    }
                }
            }
            impl From<&[$type; 2]> for $name {
                #[inline]
                fn from(value: &[$type; 2]) -> Self {
                    Self {
                        x: value[0],
                        y: value[1],
                    }
                }
            }

            impl From<($type, $type)> for $name {
                #[inline]
                fn from(value: ($type, $type)) -> Self {
                    Self {
                        x: value.0,
                        y: value.1,
                    }
                }
            }
            impl From<&($type, $type)> for $name {
                #[inline]
                fn from(value: &($type, $type)) -> Self {
                    Self {
                        x: value.0,
                        y: value.1,
                    }
                }
            }

            impl From<$name> for [$type; 2] {
                #[inline]
                fn from(value: $name) -> Self {
                    [value.x, value.y]
                }
            }
            impl From<&$name> for [$type; 2] {
                #[inline]
                fn from(value: &$name) -> Self {
                    [value.x, value.y]
                }
            }

        )+
    };
}
vec2s!((Vec2 => f32), (DVec2 => f64));
impl_float!((Vec2 =>f32), (DVec2=> f64));

vec2s!((I8Vec2 => i8), (U8Vec2 => u8),
    (I16Vec2 => i16), (U16Vec2 => u16),
    (IVec2 => i32), (UVec2 => u32),
    (I64Vec2 => i64), (U64Vec2 => u64),
    (I128Vec2 => i128), (U128Vec2 => u128),
    (IsizeVec2 => isize), (UsizeVec2 => usize));
impl_signed!(
    (I8Vec2 => i8),
    (I16Vec2 => i16),
    (IVec2 => i32),
    (I64Vec2 => i64),
    (I128Vec2 => i128),
    (IsizeVec2 => isize)
);

#[cfg(feature = "f16")]
vec2s!((F16Vec2 => f16));
#[cfg(feature = "f16")]
impl_float!((F16Vec2 => f16));

#[cfg(feature = "f128")]
vec2s!((F128Vec2 => f128));
#[cfg(feature = "f128")]
impl_float!((F128Vec2 => f128));
