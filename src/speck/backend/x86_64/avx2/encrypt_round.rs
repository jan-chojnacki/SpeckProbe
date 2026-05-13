macro_rules! avx2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = avx2_ror!($word, $x, $alpha);
        $x = avx2_add!($word, $x, $y);
        $x = avx2_xor!($word, $x, $k);

        $y = avx2_rol!($word, $y, $beta);
        $y = avx2_xor!($word, $y, $x);
    };
}

pub(crate) use avx2_encrypt_round_inline;
