#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_float_math))]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

mod angle;
mod math;
pub mod vec;

pub use angle::*;
pub use vec::*;
