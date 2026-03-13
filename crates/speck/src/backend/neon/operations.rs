use paste::paste;

#[cfg(target_arch = "aarch64")]
use crate::backend::neon::neon_word_ty;
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

macro_rules! define_neon_ror {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_ror_ $n _u24>](v: neon_word_ty!(24)) -> neon_word_ty!(24) {
                let hi = vshrq_n_u32::<$n>(v);
                let lo = vshlq_n_u32::<{ 24 - $n }>(v);
                vandq_u32(vorrq_u32(hi, lo), vdupq_n_u32(0x00FF_FFFF))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_ror_ $n _u48>](v: neon_word_ty!(48)) -> neon_word_ty!(48) {
                let hi = vshrq_n_u64::<$n>(v);
                let lo = vshlq_n_u64::<{ 48 - $n }>(v);
                vandq_u64(vorrq_u64(hi, lo), vdupq_n_u64(0x0000_FFFF_FFFF_FFFF))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_ror_ $n _u $word>](v: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshrq_n_u $word>]::<$n>(v);
                let lo = [<vshlq_n_u $word>]::<{ $word - $n }>(v);
                [<vorrq_u $word>](hi, lo)
            }
        }
    };
}

define_neon_ror!(16, 8);
define_neon_ror!(16, 7);
define_neon_ror!(16, 3);
define_neon_ror!(16, 2);
define_neon_ror!(24, 8);
define_neon_ror!(24, 7);
define_neon_ror!(24, 3);
define_neon_ror!(24, 2);
define_neon_ror!(32, 8);
define_neon_ror!(32, 7);
define_neon_ror!(32, 3);
define_neon_ror!(32, 2);
define_neon_ror!(48, 8);
define_neon_ror!(48, 7);
define_neon_ror!(48, 3);
define_neon_ror!(48, 2);
define_neon_ror!(64, 8);
define_neon_ror!(64, 7);
define_neon_ror!(64, 3);
define_neon_ror!(64, 2);

macro_rules! define_neon_rol {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_rol_ $n _u24>](v: neon_word_ty!(24)) -> neon_word_ty!(24) {
                let hi = vshlq_n_u32::<$n>(v);
                let lo = vshrq_n_u32::<{ 24 - $n }>(v);
                vandq_u32(vorrq_u32(hi, lo), vdupq_n_u32(0x00FF_FFFF))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_rol_ $n _u48>](v: neon_word_ty!(48)) -> neon_word_ty!(48) {
                let hi = vshlq_n_u64::<$n>(v);
                let lo = vshrq_n_u64::<{ 48 - $n }>(v);
                vandq_u64(vorrq_u64(hi, lo), vdupq_n_u64(0x0000_FFFF_FFFF_FFFF))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_rol_ $n _u $word>](v: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshlq_n_u $word>]::<$n>(v);
                let lo = [<vshrq_n_u $word>]::<{ $word - $n }>(v);
                [<vorrq_u $word>](hi, lo)
            }
        }
    };
}

define_neon_rol!(16, 8);
define_neon_rol!(16, 7);
define_neon_rol!(16, 3);
define_neon_rol!(16, 2);
define_neon_rol!(24, 8);
define_neon_rol!(24, 7);
define_neon_rol!(24, 3);
define_neon_rol!(24, 2);
define_neon_rol!(32, 8);
define_neon_rol!(32, 7);
define_neon_rol!(32, 3);
define_neon_rol!(32, 2);
define_neon_rol!(48, 8);
define_neon_rol!(48, 7);
define_neon_rol!(48, 3);
define_neon_rol!(48, 2);
define_neon_rol!(64, 8);
define_neon_rol!(64, 7);
define_neon_rol!(64, 3);
define_neon_rol!(64, 2);

macro_rules! define_neon_add {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_add_u24(a: neon_word_ty!(24), b: neon_word_ty!(24)) -> neon_word_ty!(24) {
                let s = vaddq_u32(a, b);
                vandq_u32(s, vdupq_n_u32(0x00FF_FFFF))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_add_u48(a: neon_word_ty!(48), b: neon_word_ty!(48)) -> neon_word_ty!(48) {
                let s = vaddq_u64(a, b);
                vandq_u64(s, vdupq_n_u64(0x0000_FFFF_FFFF_FFFF))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_add_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<vaddq_u $word>](a, b)
            }
        }
    };
}

define_neon_add!(16);
define_neon_add!(24);
define_neon_add!(32);
define_neon_add!(48);
define_neon_add!(64);

macro_rules! define_neon_sub {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_sub_u24(a: neon_word_ty!(24), b: neon_word_ty!(24)) -> neon_word_ty!(24) {
                let s = vsubq_u32(a, b);
                vandq_u32(s, vdupq_n_u32(0x00FF_FFFF))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_sub_u48(a: neon_word_ty!(48), b: neon_word_ty!(48)) -> neon_word_ty!(48) {
                let s = vsubq_u64(a, b);
                vandq_u64(s, vdupq_n_u64(0x0000_FFFF_FFFF_FFFF))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_sub_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<vsubq_u $word>](a, b)
            }
        }
    };
}

define_neon_sub!(16);
define_neon_sub!(24);
define_neon_sub!(32);
define_neon_sub!(48);
define_neon_sub!(64);

macro_rules! define_neon_xor {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_xor_u24(a: neon_word_ty!(24), b: neon_word_ty!(24)) -> neon_word_ty!(24) {
                vandq_u32(veorq_u32(a, b), vdupq_n_u32(0x00FF_FFFF))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn neon_xor_u48(a: neon_word_ty!(48), b: neon_word_ty!(48)) -> neon_word_ty!(48) {
                vandq_u64(veorq_u64(a, b), vdupq_n_u64(0x0000_FFFF_FFFF_FFFF))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_xor_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<veorq_u $word>](a, b)
            }
        }
    };
}

define_neon_xor!(16);
define_neon_xor!(24);
define_neon_xor!(32);
define_neon_xor!(48);
define_neon_xor!(64);
