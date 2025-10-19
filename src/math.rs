/// A trait that provides an API for the math functions that don't have an implementation in std.
pub(crate) trait Math {
    fn sqrt(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
}

macro_rules! impl_fn {
    ($type: ident, ($($fn: ident),*)) => {
            $(
                fn $fn(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        <$type>::$fn(self)
                    }
                    #[cfg(not(feature = "std"))]
                    core::$type::math::$fn(self)
                }
            )*
        }
}

/// A macro that provides both implementations for the std and experimental core implementations of [`Math`].
macro_rules! impl_math {
    ($($type: ident), +) => {
        $(
            impl Math for $type {
                impl_fn!($type, (sqrt, round, trunc, ceil, floor, fract));
            }
        )+
    };
}

impl_math!(f32, f64);
#[cfg(feature = "f16")]
impl_math!(f16);
#[cfg(feature = "f128")]
impl_math!(f128);
