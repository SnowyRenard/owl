#![cfg_attr(feature = "simd", feature(portable_simd))]

mod angle;
mod num;
pub mod vec;

#[cfg(feature = "num-traits")]
mod numtraits;
#[cfg(feature = "simd")]
mod simd;

pub use vec::*;
