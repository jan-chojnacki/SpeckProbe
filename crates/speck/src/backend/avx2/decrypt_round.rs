#[macro_export]
macro_rules! avx2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::avx2_xor!($word, $y, $x);
        $y = $crate::avx2_ror!($word, $y, $beta);

        $x = $crate::avx2_xor!($word, $x, $k);
        $x = $crate::avx2_sub!($word, $x, $y);
        $x = $crate::avx2_rol!($word, $x, $alpha);
    };
}
