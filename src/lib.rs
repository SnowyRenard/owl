#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_float_math))]

mod math;
mod vec2;
mod vec3;

pub use vec2::*;
pub use vec3::*;
