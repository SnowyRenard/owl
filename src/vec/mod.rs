mod vec2;
mod vec3;
mod vec4;

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

/// A macro for swizzling vectors, the valid inputs are x,y,z,w.
#[macro_export]
macro_rules! swizzle {
    ($(($vec: expr, $a: ident, $b: ident, $c: ident, $d: ident) => $ret: ty)?) => {
        $(<$ret>::new($vec.$a, $vec.$b, $vec.$c, $vec.$d))?
    };
    ($(($vec: expr, $a: ident, $b: ident, $c: ident) => $ret: ty)?) => {
        $(<$ret>::new($vec.$a, $vec.$b, $vec.$c))?
    };
    ($(($vec: expr, $a: ident, $b: ident) => $ret: ty)?) => {
        $(<$ret>::new($vec.$a, $vec.$b))?
    };
}
