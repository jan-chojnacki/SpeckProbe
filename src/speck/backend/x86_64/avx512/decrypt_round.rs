macro_rules! avx512_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = avx512_xor!($word, $y, $x);
        $y = avx512_ror!($word, $y, $beta);

        $x = avx512_xor!($word, $x, $k);
        $x = avx512_sub!($word, $x, $y);
        $x = avx512_rol!($word, $x, $alpha);
    };
}

pub(crate) use avx512_decrypt_round_inline;
