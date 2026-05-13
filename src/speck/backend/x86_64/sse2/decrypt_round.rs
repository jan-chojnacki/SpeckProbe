macro_rules! sse2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = sse2_xor!($word, $y, $x);
        $y = sse2_ror!($word, $y, $beta);

        $x = sse2_xor!($word, $x, $k);
        $x = sse2_sub!($word, $x, $y);
        $x = sse2_rol!($word, $x, $alpha);
    };
}

pub(crate) use sse2_decrypt_round_inline;
