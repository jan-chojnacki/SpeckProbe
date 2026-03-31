#[macro_export]
macro_rules! sse2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::sse2_xor!($word, $y, $x);
        $y = $crate::sse2_ror!($word, $y, $beta);

        $x = $crate::sse2_xor!($word, $x, $k);
        $x = $crate::sse2_sub!($word, $x, $y);
        $x = $crate::sse2_rol!($word, $x, $alpha);
    };
}
