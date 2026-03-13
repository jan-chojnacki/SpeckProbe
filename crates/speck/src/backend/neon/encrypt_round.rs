#[cfg(target_arch = "aarch64")]
use crate::backend::neon::neon_word_ty;
use crate::backend::neon::operations::*;
use paste::paste;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

macro_rules! define_encrypt_round_neon {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub(crate) fn [<neon_encrypt_round_ $word>](x: &mut neon_word_ty!($word), y: &mut neon_word_ty!($word), k: neon_word_ty!($word)) {
                *x = [<neon_ror_ $alpha _u $word>](*x);
                *x = [<neon_add_u $word>](*x, *y);
                *x = [<neon_xor_u $word>](*x, k);

                *y = [<neon_rol_ $beta _u $word>](*y);
                *y = [<neon_xor_u $word>](*y, *x);
            }
        }
    };
}

define_encrypt_round_neon!(16, 7, 2);
define_encrypt_round_neon!(24, 8, 3);
define_encrypt_round_neon!(32, 8, 3);
define_encrypt_round_neon!(48, 8, 3);
define_encrypt_round_neon!(64, 8, 3);
