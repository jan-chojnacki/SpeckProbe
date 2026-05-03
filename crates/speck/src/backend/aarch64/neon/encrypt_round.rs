macro_rules! neon_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::backend::aarch64::neon::neon_ror!($word, $x, $alpha);
        $x = $crate::backend::aarch64::neon::neon_add!($word, $x, $y);
        $x = $crate::backend::aarch64::neon::neon_xor!($word, $x, $k);

        $y = $crate::backend::aarch64::neon::neon_rol!($word, $y, $beta);
        $y = $crate::backend::aarch64::neon::neon_xor!($word, $y, $x);
    };
}

pub(crate) use neon_encrypt_round_inline;
