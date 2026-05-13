macro_rules! neon_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = neon_ror!($word, $x, $alpha);
        $x = neon_add!($word, $x, $y);
        $x = neon_xor!($word, $x, $k);

        $y = neon_rol!($word, $y, $beta);
        $y = neon_xor!($word, $y, $x);
    };
}

pub(crate) use neon_encrypt_round_inline;
