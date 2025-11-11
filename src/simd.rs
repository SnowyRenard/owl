use crate::num::prelude::*;
use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

impl<T, const N: usize> Zero for Simd<T, N>
where
    T: SimdElement + Zero,
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Simd<T, N> = Simd::splat(T::ZERO);
}
impl<T, const N: usize> One for Simd<T, N>
where
    T: SimdElement + One,
    LaneCount<N>: SupportedLaneCount,
{
    const ONE: Simd<T, N> = Simd::splat(T::ONE);
}
impl<T, const N: usize> NegOne for Simd<T, N>
where
    T: SimdElement + NegOne,
    LaneCount<N>: SupportedLaneCount,
{
    const NEG_ONE: Simd<T, N> = Simd::splat(T::NEG_ONE);
}
