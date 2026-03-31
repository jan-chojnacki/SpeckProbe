#[macro_export]
macro_rules! sse2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::sse2_ror!($word, $x, $alpha);
        $x = $crate::sse2_add!($word, $x, $y);
        $x = $crate::sse2_xor!($word, $x, $k);

        $y = $crate::sse2_rol!($word, $y, $beta);
        $y = $crate::sse2_xor!($word, $y, $x);
    };
}
