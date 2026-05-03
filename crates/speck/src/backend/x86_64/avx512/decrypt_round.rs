macro_rules! avx512_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::backend::x86_64::avx512::avx512_xor!($word, $y, $x);
        $y = $crate::backend::x86_64::avx512::avx512_ror!($word, $y, $beta);

        $x = $crate::backend::x86_64::avx512::avx512_xor!($word, $x, $k);
        $x = $crate::backend::x86_64::avx512::avx512_sub!($word, $x, $y);
        $x = $crate::backend::x86_64::avx512::avx512_rol!($word, $x, $alpha);
    };
}

pub(crate) use avx512_decrypt_round_inline;
