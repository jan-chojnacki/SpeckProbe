macro_rules! sse2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = sse2_ror!($word, $x, $alpha);
        $x = sse2_add!($word, $x, $y);
        $x = sse2_xor!($word, $x, $k);

        $y = sse2_rol!($word, $y, $beta);
        $y = sse2_xor!($word, $y, $x);
    };
}

pub(crate) use sse2_encrypt_round_inline;
