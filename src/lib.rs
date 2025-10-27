mod angle;
pub mod vec;

pub use angle::*;
pub use vec::*;

macro_rules! impl_const {
    ($trait:ident $const:ident for $($type:ty: $val:expr)+) => {
        $(
            impl $trait for $type {
                const $const: Self = $val;
            }
        )+
    };
    ($trait:ident $const:ident for $($type:ty)+) => {
        $(
            impl $trait for $type {
                const $const: Self = <$type>::$const;
            }
        )+
    };
}

pub mod consts {
    pub trait Zero {
        const ZERO: Self;
    }
    pub trait One {
        const ONE: Self;
    }
    pub trait NegOne {
        const NEG_ONE: Self;
    }

    impl_const!(Zero ZERO for usize:0 u8:0 u16:0 u32:0 u128:0 isize:0 i8:0 i16:0 i32:0 i64:0 i128:0 f32:0.0 f64:0.0);
    impl_const!(One ONE for usize:1 u8:1 u16:1 u32:1 u128:1 isize:1 i8:1 i16:1 i32:1 i64:1 i128:1 f32:1.0 f64:1.0);
    impl_const!(NegOne NEG_ONE for isize:-1 i8:-1 i16:-1 i32:-1 i64:-1 i128:-1 f32:-1.0 f64:-1.0);
}
