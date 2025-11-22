use crate::num::prelude::*;

impl<T: num_traits::Float> Float for T {
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }

    #[inline]
    fn round(self) -> Self {
        self.round()
    }

    #[inline]
    fn trunc(self) -> Self {
        self.trunc()
    }

    #[inline]
    fn fract(self) -> Self {
        self.fract()
    }
}

impl<T: num_traits::identities::ConstZero> Zero for T {
    const ZERO: Self = T::ZERO;
}
impl<T: num_traits::identities::ConstOne> One for T {
    const ONE: Self = T::ONE;
}
