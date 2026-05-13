macro_rules! avx2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = avx2_xor!($word, $y, $x);
        $y = avx2_ror!($word, $y, $beta);

        $x = avx2_xor!($word, $x, $k);
        $x = avx2_sub!($word, $x, $y);
        $x = avx2_rol!($word, $x, $alpha);
    };
}

pub(crate) use avx2_decrypt_round_inline;
