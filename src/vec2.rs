use core::ops::*;

use crate::math::Math;

pub type Rg = Vec2;
pub type Uv = Vec2;
pub type Point2 = Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const ZERO: Self = Self::splat(0.);
    pub const ONE: Self = Self::splat(1.);
    pub const NEG_ONE: Self = Self::splat(-1.);

    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);

    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    pub const X: Self = Self::new(1., 0.);
    pub const Y: Self = Self::new(0., 1.);
    pub const NEG_X: Self = Self::new(-1., 0.);
    pub const NEG_Y: Self = Self::new(0., -1.);

    #[inline(always)]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn splat(value: f32) -> Self {
        Self { x: value, y: value }
    }

    #[inline]
    #[must_use]
    pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
        }
    }

    #[inline]
    #[must_use]
    pub fn dot(&self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
        }
    }
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }

    #[inline]
    #[must_use]
    pub fn min_element(self) -> f32 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, self.y)
    }
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f32 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn element_sum(self) -> f32 {
        self.x + self.y
    }
    #[inline]
    #[must_use]
    pub fn element_product(self) -> f32 {
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
    pub fn length(self) -> f32 {
        Math::sqrt(self.length_squared())
    }
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
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
    pub fn refract(&self, normal: Self, eta: f32) -> Self {
        let n_dot_i = normal.dot(*self);
        let k = 1. - eta * eta * (1. - n_dot_i * n_dot_i);
        if k >= 0. {
            eta * self - (eta * n_dot_i + Math::sqrt(k)) * normal
        } else {
            Self::ZERO
        }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}
impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl From<f32> for Vec2 {
    #[inline]
    fn from(value: f32) -> Self {
        Self::splat(value)
    }
}
impl From<&f32> for Vec2 {
    #[inline]
    fn from(value: &f32) -> Self {
        Self::splat(*value)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
impl From<&[f32; 2]> for Vec2 {
    #[inline]
    fn from(value: &[f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<(f32, f32, f32)> for Vec2 {
    #[inline]
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<&(f32, f32, f32)> for Vec2 {
    #[inline]
    fn from(value: &(f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}
impl From<&Vec2> for [f32; 2] {
    #[inline]
    fn from(value: &Vec2) -> Self {
        [value.x, value.y]
    }
}

impl Default for Vec2 {
    #[inline]
    fn default() -> Self {
        Self::splat(0.)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Neg for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Vec2 x Vec2
// -----------
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}
impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}
impl Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}
impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}
impl Mul<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}
impl Mul<Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}
impl Mul<&Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}
impl Div<&Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}
impl Div<Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}
impl Div<&Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}
impl AddAssign<&Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec2) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl SubAssign<Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}
impl SubAssign<&Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}
impl MulAssign<&Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec2) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec2) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}
impl DivAssign<&Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec2) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

// Vec2 x f32
// -----------
impl Add<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}
impl Add<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}
impl Add<f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}
impl Add<&f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}
impl Sub<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}
impl Sub<f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}
impl Sub<&f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}
impl Mul<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}
impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}
impl Mul<&f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}
impl Div<&f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}
impl Div<f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}
impl Div<&f32> for &Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}
impl AddAssign<&f32> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}
impl SubAssign<&f32> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}
impl MulAssign<&f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}
impl DivAssign<&f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

// f32 x Vec2
// ----------
impl Add<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}
impl Add<Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}
impl Add<&Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}
impl Add<&Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}
impl Sub<Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}
impl Sub<&Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}
impl Sub<&Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}
impl Mul<Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}
impl Mul<&Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}
impl Mul<&Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}
impl Div<Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}
impl Div<&Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}
impl Div<&Vec2> for &f32 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: &Vec2) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl AddAssign<Vec2> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2) {
        self.add_assign(rhs.x);
        self.add_assign(rhs.y);
    }
}
impl AddAssign<&Vec2> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec2) {
        self.add_assign(rhs.x);
        self.add_assign(rhs.y);
    }
}

impl SubAssign<Vec2> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.sub_assign(rhs.x);
        self.sub_assign(rhs.y);
    }
}
impl SubAssign<&Vec2> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.sub_assign(rhs.x);
        self.sub_assign(rhs.y);
    }
}

impl MulAssign<Vec2> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec2) {
        self.mul_assign(rhs.x);
        self.mul_assign(rhs.y);
    }
}
impl MulAssign<&Vec2> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec2) {
        self.mul_assign(rhs.x);
        self.mul_assign(rhs.y);
    }
}

impl DivAssign<Vec2> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec2) {
        self.div_assign(rhs.x);
        self.div_assign(rhs.y);
    }
}
impl DivAssign<&Vec2> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec2) {
        self.div_assign(rhs.x);
        self.div_assign(rhs.y);
    }
}
