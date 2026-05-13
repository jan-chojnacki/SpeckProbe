macro_rules! avx512_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = avx512_ror!($word, $x, $alpha);
        $x = avx512_add!($word, $x, $y);
        $x = avx512_xor!($word, $x, $k);

        $y = avx512_rol!($word, $y, $beta);
        $y = avx512_xor!($word, $y, $x);
    };
}

pub(crate) use avx512_encrypt_round_inline;
