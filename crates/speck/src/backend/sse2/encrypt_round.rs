use crate::backend::sse2::operations::*;
use paste::paste;
use std::arch::x86_64::__m128i;

macro_rules! define_encrypt_round_sse2 {
    ($word:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            #[target_feature(enable = "sse2")]
            pub(crate) fn [<sse2_encrypt_round_ $word>](x: &mut __m128i, y: &mut __m128i, k: __m128i) {
                *x = [<sse2_ror_ $alpha _u $word>](*x);
                *x = [<sse2_add_u $word>](*x, *y);
                *x = [<sse2_xor_u $word>](*x, k);

                *y = [<sse2_rol_ $beta _u $word>](*y);
                *y = [<sse2_xor_u $word>](*y, *x);
            }
        }
    };
}

#[macro_export]
macro_rules! sse2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = sse2_ror!($word, $x, $alpha);
        $x = sse2_add!($word, $x, $y);
        $x = sse2_xor!($word, $x, $k);

        $y = sse2_rol!($word, $y, $beta);
        $y = sse2_xor!($word, $y, $x);
    };
}

define_encrypt_round_sse2!(16, 7, 2);
define_encrypt_round_sse2!(24, 8, 3);
define_encrypt_round_sse2!(32, 8, 3);
define_encrypt_round_sse2!(48, 8, 3);
define_encrypt_round_sse2!(64, 8, 3);
