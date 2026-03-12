use crate::backend::avx512::operations::*;
use paste::paste;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m512i;

macro_rules! define_decrypt_round_avx512 {
    ($word:literal, $alpha:literal, $beta:literal, $feature:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = $feature))]
            #[target_feature(enable = $feature)]
            pub(crate) fn [<avx512_decrypt_round_ $word>](x: &mut __m512i, y: &mut __m512i, k: __m512i) {
                *y = [<avx512_xor_u $word>](*y, *x);
                *y = [<avx512_ror_ $beta _u $word>](*y);

                *x = [<avx512_xor_u $word>](*x, k);
                *x = [<avx512_sub_u $word>](*x, *y);
                *x = [<avx512_rol_ $alpha _u $word>](*x);
            }
        }
    };
}

define_decrypt_round_avx512!(16, 7, 2, "avx512bw");
define_decrypt_round_avx512!(24, 8, 3, "avx512f");
define_decrypt_round_avx512!(32, 8, 3, "avx512f");
define_decrypt_round_avx512!(48, 8, 3, "avx512f");
define_decrypt_round_avx512!(64, 8, 3, "avx512f");
