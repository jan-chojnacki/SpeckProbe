macro_rules! neon_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = neon_xor!($word, $y, $x);
        $y = neon_ror!($word, $y, $beta);

        $x = neon_xor!($word, $x, $k);
        $x = neon_sub!($word, $x, $y);
        $x = neon_rol!($word, $x, $alpha);
    };
}

pub(crate) use neon_decrypt_round_inline;
