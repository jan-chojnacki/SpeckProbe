#[macro_export]
macro_rules! neon_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::neon_xor!($word, $y, $x);
        $y = $crate::neon_ror!($word, $y, $beta);

        $x = $crate::neon_xor!($word, $x, $k);
        $x = $crate::neon_sub!($word, $x, $y);
        $x = $crate::neon_rol!($word, $x, $alpha);
    };
}
