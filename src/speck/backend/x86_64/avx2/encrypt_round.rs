macro_rules! avx2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::speck::backend::x86_64::avx2::avx2_ror!($word, $x, $alpha);
        $x = $crate::speck::backend::x86_64::avx2::avx2_add!($word, $x, $y);
        $x = $crate::speck::backend::x86_64::avx2::avx2_xor!($word, $x, $k);

        $y = $crate::speck::backend::x86_64::avx2::avx2_rol!($word, $y, $beta);
        $y = $crate::speck::backend::x86_64::avx2::avx2_xor!($word, $y, $x);
    };
}

pub(crate) use avx2_encrypt_round_inline;
