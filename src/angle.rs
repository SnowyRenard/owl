#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f32);

impl Angle {
    #[inline(always)]
    pub const fn from_radians(rad: f32) -> Self {
        Self(rad)
    }
    #[inline(always)]
    pub fn from_degrees(deg: f32) -> Self {
        Self(deg.to_radians())
    }

    #[inline(always)]
    pub const fn radians(&self) -> f32 {
        self.0
    }
    #[inline(always)]
    pub fn degrees(&self) -> f32 {
        self.0.to_degrees()
    }
}
