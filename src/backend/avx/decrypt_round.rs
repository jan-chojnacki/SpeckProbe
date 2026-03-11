use crate::backend::avx::operations::*;
use paste::paste;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m128i;

macro_rules! define_decrypt_round_avx {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub(crate) fn [<avx_decrypt_round_ $word>](x: &mut __m128i, y: &mut __m128i, k: __m128i) {
                *y = [<avx_xor_u $word>](*y, *x);
                *y = [<avx_ror_ $beta _u $word>](*y);

                *x = [<avx_xor_u $word>](*x, k);
                *x = [<avx_sub_u $word>](*x, *y);
                *x = [<avx_rol_ $alpha _u $word>](*x);
            }
        }
    };
}

define_decrypt_round_avx!(16, 7, 2);
define_decrypt_round_avx!(24, 8, 3);
define_decrypt_round_avx!(32, 8, 3);
define_decrypt_round_avx!(48, 8, 3);
define_decrypt_round_avx!(64, 8, 3);
