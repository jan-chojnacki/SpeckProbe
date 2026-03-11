#[cfg(target_arch = "aarch64")]
use crate::neon_word_ty;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
macro_rules! define_neon_ror {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_ror_alpha_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshrq_n_u $word>]::<{ [<ALPHA_ $word>] as i32 }>(x);
                let lo = [<vshlq_n_u $word>]::<{ $word - [<ALPHA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }

            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_ror_beta_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshrq_n_u $word>]::<{ [<BETA_ $word>] as i32 }>(x);
                let lo = [<vshlq_n_u $word>]::<{ $word - [<BETA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
define_neon_ror!(16);

#[cfg(target_arch = "aarch64")]
define_neon_ror!(32);

#[cfg(target_arch = "aarch64")]
define_neon_ror!(64);

#[cfg(target_arch = "aarch64")]
macro_rules! define_neon_rol {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_rol_alpha_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshlq_n_u $word>]::<{ [<ALPHA_ $word>] as i32 }>(x);
                let lo = [<vshrq_n_u $word>]::<{ $word - [<ALPHA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }

            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_rol_beta_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshlq_n_u $word>]::<{ [<BETA_ $word>] as i32 }>(x);
                let lo = [<vshrq_n_u $word>]::<{ $word - [<BETA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
define_neon_rol!(16);

#[cfg(target_arch = "aarch64")]
define_neon_rol!(32);

#[cfg(target_arch = "aarch64")]
define_neon_rol!(64);

#[cfg(target_arch = "aarch64")]
macro_rules! define_neon_add {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_add_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<vaddq_u $word>](a, b)
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
define_neon_add!(16);

#[cfg(target_arch = "aarch64")]
define_neon_add!(32);

#[cfg(target_arch = "aarch64")]
define_neon_add!(64);

#[cfg(target_arch = "aarch64")]
macro_rules! define_neon_sub {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_sub_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<vsubq_u $word>](a, b)
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
define_neon_sub!(16);

#[cfg(target_arch = "aarch64")]
define_neon_sub!(32);

#[cfg(target_arch = "aarch64")]
define_neon_sub!(64);

#[cfg(target_arch = "aarch64")]
macro_rules! define_neon_xor {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_xor_u $word>](a: neon_word_ty!($word), b: neon_word_ty!($word)) -> neon_word_ty!($word) {
                [<veorq_u $word>](a, b)
            }
        }
    };
}

#[cfg(target_arch = "aarch64")]
define_neon_xor!(16);

#[cfg(target_arch = "aarch64")]
define_neon_xor!(32);

#[cfg(target_arch = "aarch64")]
define_neon_xor!(64);