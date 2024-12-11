#![no_std]
#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

macro_rules! as_trait {
    ($target_type:ty, $trait_name:ident, $method_name:ident, [$($type:ty),*]) => {
        #[doc = concat!("A trait for converting to [`", stringify!($target_type), "`].")]
        pub trait $trait_name {
            fn $method_name(self) -> $target_type;
        }

        $(
            impl $trait_name for $type {
                #[inline(always)]
                fn $method_name(self) -> $target_type {
                    self as $target_type
                }
            }
        )*
    };
}

as_trait!(
    i8,
    AsI8,
    as_i8,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    i16,
    AsI16,
    as_i16,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    i32,
    AsI32,
    as_i32,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    i64,
    AsI64,
    as_i64,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    i128,
    AsI128,
    as_i128,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    isize,
    AsIsize,
    as_isize,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    u8,
    AsU8,
    as_u8,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    u16,
    AsU16,
    as_u16,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    u32,
    AsU32,
    as_u32,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    u64,
    AsU64,
    as_u64,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    u128,
    AsU128,
    as_u128,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    usize,
    AsUsize,
    as_usize,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    f32,
    AsF32,
    as_f32,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
as_trait!(
    f64,
    AsF64,
    as_f64,
    [i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64]
);
