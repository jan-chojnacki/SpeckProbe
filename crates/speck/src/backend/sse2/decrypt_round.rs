use crate::backend::sse2::operations::*;
use paste::paste;
use std::arch::x86_64::__m128i;

macro_rules! define_decrypt_round_sse2 {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            #[target_feature(enable = "sse2")]
            pub(crate) fn [<sse2_decrypt_round_ $word>](x: &mut __m128i, y: &mut __m128i, k: __m128i) {
                *y = [<sse2_xor_u $word>](*y, *x);
                *y = [<sse2_ror_ $beta _u $word>](*y);

                *x = [<sse2_xor_u $word>](*x, k);
                *x = [<sse2_sub_u $word>](*x, *y);
                *x = [<sse2_rol_ $alpha _u $word>](*x);
            }
        }
    };
}

#[macro_export]
macro_rules! sse2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = sse2_xor!($word, $y, $x);
        $y = sse2_ror!($word, $y, $beta);

        $x = sse2_xor!($word, $x, $k);
        $x = sse2_sub!($word, $x, $y);
        $x = sse2_rol!($word, $x, $alpha);
    };
}

define_decrypt_round_sse2!(16, 7, 2);
define_decrypt_round_sse2!(24, 8, 3);
define_decrypt_round_sse2!(32, 8, 3);
define_decrypt_round_sse2!(48, 8, 3);
define_decrypt_round_sse2!(64, 8, 3);
