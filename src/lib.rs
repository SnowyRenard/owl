#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_float_math))]

mod angle;
mod math;
mod vec;

pub use angle::*;
pub use vec::*;
