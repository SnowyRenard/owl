#[cfg(not(feature = "std"))]
pub(crate) use core_math::*;
#[cfg(feature = "std")]
pub(crate) use std_math::*;

#[cfg(feature = "std")]
pub mod std_math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        f32::abs(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        f32::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn round(f: f32) -> f32 {
        f32::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        f32::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f32) -> f32 {
        f32::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        f32::floor(f)
    }

    #[inline(always)]
    pub(crate) fn fract(f: f32) -> f32 {
        f32::fract(f)
    }
}

#[cfg(not(feature = "std"))]
pub mod core_math {
    use core::f32::math;

    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        f32::abs(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        math::sqrt(f)
    }

    #[inline(always)]
    pub(crate) fn round(f: f32) -> f32 {
        math::round(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        math::trunc(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f32) -> f32 {
        math::ceil(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        math::floor(f)
    }

    #[inline(always)]
    pub(crate) fn fract(f: f32) -> f32 {
        math::fract(f)
    }
}
