use crate::backend::neon::neon_word_ty;
use crate::backend::neon::operations::*;
use paste::paste;
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

macro_rules! define_decrypt_round_neon {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub(crate) fn [<neon_decrypt_round_ $word>](x: &mut neon_word_ty!($word), y: &mut neon_word_ty!($word), k: neon_word_ty!($word)) {
                *y = [<neon_xor_u $word>](*y, *x);
                *y = [<neon_ror_ $beta _u $word>](*y);

                *x = [<neon_xor_u $word>](*x, k);
                *x = [<neon_sub_u $word>](*x, *y);
                *x = [<neon_rol_ $alpha _u $word>](*x);
            }
        }
    };
}

define_decrypt_round_neon!(16, 7, 2);
define_decrypt_round_neon!(24, 8, 3);
define_decrypt_round_neon!(32, 8, 3);
define_decrypt_round_neon!(48, 8, 3);
define_decrypt_round_neon!(64, 8, 3);
