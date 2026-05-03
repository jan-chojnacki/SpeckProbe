macro_rules! avx2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::speck::backend::x86_64::avx2::avx2_xor!($word, $y, $x);
        $y = $crate::speck::backend::x86_64::avx2::avx2_ror!($word, $y, $beta);

        $x = $crate::speck::backend::x86_64::avx2::avx2_xor!($word, $x, $k);
        $x = $crate::speck::backend::x86_64::avx2::avx2_sub!($word, $x, $y);
        $x = $crate::speck::backend::x86_64::avx2::avx2_rol!($word, $x, $alpha);
    };
}

pub(crate) use avx2_decrypt_round_inline;
