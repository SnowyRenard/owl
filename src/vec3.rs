use core::ops::*;

use crate::math;

pub type Rgb = Vec3;
pub type Uvw = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const ZERO: Self = Self::splat(0.);
    pub const ONE: Self = Self::splat(1.);
    pub const NEG_ONE: Self = Self::splat(-1.);

    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);

    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    pub const X: Self = Self::new(1., 0., 0.);
    pub const Y: Self = Self::new(0., 1., 0.);
    pub const Z: Self = Self::new(0., 0., 1.);
    pub const NEG_X: Self = Self::new(-1., 0., 0.);
    pub const NEG_Y: Self = Self::new(0., -1., 0.);
    pub const NEG_Z: Self = Self::new(0., 0., -1.);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline]
    pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y).min(self.z)
    }
    #[inline]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }

    #[inline]
    pub fn element_sum(self) -> f32 {
        self.x + self.y + self.z
    }
    #[inline]
    pub fn element_product(self) -> f32 {
        self.x * self.y * self.z
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    #[inline]
    pub fn length(self) -> f32 {
        math::sqrt(self.length_squared())
    }
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: math::round(self.x),
            y: math::round(self.y),
            z: math::round(self.z),
        }
    }
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: math::floor(self.x),
            y: math::floor(self.y),
            z: math::floor(self.z),
        }
    }
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: math::ceil(self.x),
            y: math::ceil(self.y),
            z: math::ceil(self.z),
        }
    }
    #[inline]
    pub fn trunc(self) -> Self {
        Self {
            x: math::trunc(self.x),
            y: math::trunc(self.y),
            z: math::trunc(self.z),
        }
    }
    #[inline]
    pub fn fract(self) -> Self {
        Self {
            x: math::fract(self.x),
            y: math::fract(self.y),
            z: math::fract(self.z),
        }
    }

    #[inline]
    pub fn reflect(&self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
    #[inline]
    pub fn refract(&self, normal: Self, eta: f32) -> Self {
        let n_dot_i = normal.dot(*self);
        let k = 1. - eta * eta * (1. - n_dot_i * n_dot_i);
        if k >= 0. {
            eta * self - (eta * n_dot_i + math::sqrt(k)) * normal
        } else {
            Self::ZERO
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl From<f32> for Vec3 {
    #[inline]
    fn from(value: f32) -> Self {
        Self::splat(value)
    }
}
impl From<&f32> for Vec3 {
    #[inline]
    fn from(value: &f32) -> Self {
        Self::splat(*value)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl From<&[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: &[f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl From<&(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(value: &(f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(value: Vec3) -> Self {
        [value.x, value.y, value.z]
    }
}
impl From<&Vec3> for [f32; 3] {
    #[inline]
    fn from(value: &Vec3) -> Self {
        [value.x, value.y, value.z]
    }
}

// Vec3 x Vec3
// -----------
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}
impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}
impl Div<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}
impl Div<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}
impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}
impl AddAssign<&Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}
impl SubAssign<&Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}
impl MulAssign<&Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}
impl DivAssign<&Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec3) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

// Vec3 x f32
// -----------
impl Add<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}
impl Add<&f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}
impl Add<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}
impl Add<&f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}
impl Sub<&f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}
impl Sub<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}
impl Sub<&f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}
impl Mul<&f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}
impl Mul<&f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}
impl Div<&f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}
impl Div<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}
impl Div<&f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        Self::Output {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl AddAssign<f32> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}
impl AddAssign<&f32> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl SubAssign<f32> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}
impl SubAssign<&f32> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}
impl MulAssign<&f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}
impl DivAssign<&f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

// f32 x Vec3
// ----------
impl Add<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}
impl Add<Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}
impl Add<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}
impl Add<&Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}
impl Sub<Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}
impl Sub<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}
impl Sub<&Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}
impl Mul<Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}
impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}
impl Mul<&Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}
impl Div<Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}
impl Div<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}
impl Div<&Vec3> for &f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}

impl AddAssign<Vec3> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.add_assign(rhs.x);
        self.add_assign(rhs.y);
        self.add_assign(rhs.z);
    }
}
impl AddAssign<&Vec3> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec3) {
        self.add_assign(rhs.x);
        self.add_assign(rhs.y);
        self.add_assign(rhs.z);
    }
}

impl SubAssign<Vec3> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.sub_assign(rhs.x);
        self.sub_assign(rhs.y);
        self.sub_assign(rhs.z);
    }
}
impl SubAssign<&Vec3> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.sub_assign(rhs.x);
        self.sub_assign(rhs.y);
        self.sub_assign(rhs.z);
    }
}

impl MulAssign<Vec3> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        self.mul_assign(rhs.x);
        self.mul_assign(rhs.y);
        self.mul_assign(rhs.z);
    }
}
impl MulAssign<&Vec3> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.mul_assign(rhs.x);
        self.mul_assign(rhs.y);
        self.mul_assign(rhs.z);
    }
}

impl DivAssign<Vec3> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        self.div_assign(rhs.x);
        self.div_assign(rhs.y);
        self.div_assign(rhs.z);
    }
}
impl DivAssign<&Vec3> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec3) {
        self.div_assign(rhs.x);
        self.div_assign(rhs.y);
        self.div_assign(rhs.z);
    }
}
