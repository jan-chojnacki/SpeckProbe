macro_rules! neon_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::speck::backend::aarch64::neon::neon_xor!($word, $y, $x);
        $y = $crate::speck::backend::aarch64::neon::neon_ror!($word, $y, $beta);

        $x = $crate::speck::backend::aarch64::neon::neon_xor!($word, $x, $k);
        $x = $crate::speck::backend::aarch64::neon::neon_sub!($word, $x, $y);
        $x = $crate::speck::backend::aarch64::neon::neon_rol!($word, $x, $alpha);
    };
}

pub(crate) use neon_decrypt_round_inline;
