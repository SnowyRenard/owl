#![cfg_attr(feature = "simd", feature(portable_simd))]

#[cfg(feature = "num-traits")]
mod numtraits;
#[cfg(feature = "simd")]
mod simd;

mod num;

pub mod mat;
pub mod vec;

pub use mat::*;
pub use vec::*;
