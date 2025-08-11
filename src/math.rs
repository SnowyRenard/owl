/// A trait that provides an API for the math functions that don't have an implementation in std.
pub(crate) trait Math {
    fn sqrt(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
}

/// A macro that provides both implementations for the std and experimental core implementations of [`Math`].
macro_rules! impl_math {
    ($($type: ident), +) => {
        $(
            impl Math for $type {
                fn sqrt(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::sqrt(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::sqrt(self)
                }

                fn round(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::round(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::round(self)
                }

                fn trunc(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::trunc(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::trunc(self)
                }

                fn ceil(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::ceil(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::ceil(self)
                }

                fn floor(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::floor(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::floor(self)
                }

                fn fract(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::fract(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::fract(self)
                }
            }
        )+
    };
}

impl_math!(f32, f64);
#[cfg(feature = "f16")]
impl_math!(f16);
#[cfg(feature = "f128")]
impl_math!(f128);
