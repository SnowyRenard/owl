use crate::math::Math;

use std::ops::*;

pub type Rg = Vec2;
pub type Uv = Vec2;
pub type Point2 = Vec2;

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

        // Vec2 x type
        // ----------
        impl $op<$vec2> for $type {
            #[inline]
            fn $fn(&mut self, rhs: $vec2) {
                self.$fn(rhs.x);
                self.$fn(rhs.y);
            }
        }
        impl $op<&$vec2> for $type {
            #[inline]
            fn $fn(&mut self, rhs: &$vec2) {
                self.$fn(rhs.x);
                self.$fn(rhs.y);
            }
        }
    };
}

macro_rules! vec2s {
    ($(($name:ident) => $type:ident), +) => {
        $(
            impl_op!(Add, add, $name, $type);
            impl_op!(Sub, sub, $name, $type);
            impl_op!(Mul, mul, $name, $type);
            impl_op!(Div, div, $name, $type);

            impl_op_assign!(AddAssign, add_assign, $name, $type);
            impl_op_assign!(SubAssign, sub_assign, $name, $type);
            impl_op_assign!(MulAssign, mul_assign, $name, $type);
            impl_op_assign!(DivAssign, div_assign, $name, $type);

            #[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct $name {
                pub x: $type,
                pub y: $type,
            }


            impl $name {
                pub const ZERO: Self = Self::splat(0.);
                pub const ONE: Self = Self::splat(1.);
                pub const NEG_ONE: Self = Self::splat(-1.);

                pub const X: Self = Self::new(1., 0.);
                pub const Y: Self = Self::new(0., 1.);
                pub const Z: Self = Self::new(0., 0.);

                pub const NEG_X: Self = Self::new(-1., 0.);
                pub const NEG_Y: Self = Self::new(0., -1.);
                pub const NEG_Z: Self = Self::new(0., 0.);

                pub const MIN: Self = Self::splat(<$type>::MIN);
                pub const MAX: Self = Self::splat(<$type>::MAX);

                pub const INFINITY: Self = Self::splat(<$type>::INFINITY);
                pub const NEG_INFINITY: Self = Self::splat(<$type>::NEG_INFINITY);

                pub const NAN: Self = Self::splat(<$type>::NAN);

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
                pub fn dot(&self, other: Self) -> $type {
                    (self.x * other.x) + (self.y * other.y)
                }
                // #[inline]
                // #[must_use]
                // pub fn cross(&self, other: Self) -> Self {
                //     Self {
                //         x: self.y * other.z - other.y * self.z,
                //         y: self.z * other.x - other.z * self.x,
                //         z: self.x * other.y - other.x * self.y,
                //     }
                // }

                #[inline]
                #[must_use]
                pub fn min(&self, other: Self) -> Self {
                    Self {
                        x: if self.x < other.x { self.x } else { other.x },
                        y: if self.y < other.y { self.y } else { other.y },
                    }
                }
                #[inline]
                #[must_use]
                pub fn max(&self, other: Self) -> Self {
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

                #[inline]
                #[must_use]
                pub fn abs(self) -> Self {
                    Self {
                        x: self.x.abs(),
                        y: self.y.abs(),
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
                pub fn reflect(&self, normal: Self) -> Self {
                    self - 2.0 * self.dot(normal) * normal
                }
                #[inline]
                #[must_use]
                pub fn refract(&self, normal: Self, eta: $type) -> Self {
                    let n_dot_i = normal.dot(*self);
                    let k = 1. - eta * eta * (1. - n_dot_i * n_dot_i);
                    if k >= 0. {
                        eta * self - (eta * n_dot_i + Math::sqrt(k)) * normal
                    } else {
                        Self::ZERO
                    }
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

            impl Neg for $name {
                type Output = $name;

                #[inline]
                fn neg(self) -> Self::Output {
                    Self::Output {
                        x: -self.x,
                        y: -self.y,
                    }
                }
            }
            impl Neg for &$name{
                type Output = $name;

                #[inline]
                fn neg(self) -> Self::Output {
                    Self::Output {
                        x: -self.x,
                        y: -self.y,
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
vec2s!((Vec2) => f32, (DVec2) => f64);
