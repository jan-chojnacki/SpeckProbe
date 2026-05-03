macro_rules! avx512_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::backend::x86_64::avx512::avx512_ror!($word, $x, $alpha);
        $x = $crate::backend::x86_64::avx512::avx512_add!($word, $x, $y);
        $x = $crate::backend::x86_64::avx512::avx512_xor!($word, $x, $k);

        $y = $crate::backend::x86_64::avx512::avx512_rol!($word, $y, $beta);
        $y = $crate::backend::x86_64::avx512::avx512_xor!($word, $y, $x);
    };
}

pub(crate) use avx512_encrypt_round_inline;
