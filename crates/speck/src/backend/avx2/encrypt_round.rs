use crate::backend::avx2::operations::*;
use paste::paste;
use std::arch::x86_64::__m256i;

macro_rules! define_encrypt_round_avx2 {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub(crate) fn [<avx2_encrypt_round_ $word>](x: &mut __m256i, y: &mut __m256i, k: __m256i) {
                *x = [<avx2_ror_ $alpha _u $word>](*x);
                *x = [<avx2_add_u $word>](*x, *y);
                *x = [<avx2_xor_u $word>](*x, k);

                *y = [<avx2_rol_ $beta _u $word>](*y);
                *y = [<avx2_xor_u $word>](*y, *x);
            }
        }
    };
}

define_encrypt_round_avx2!(16, 7, 2);
define_encrypt_round_avx2!(24, 8, 3);
define_encrypt_round_avx2!(32, 8, 3);
define_encrypt_round_avx2!(48, 8, 3);
define_encrypt_round_avx2!(64, 8, 3);
