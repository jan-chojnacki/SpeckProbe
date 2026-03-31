#[macro_export]
macro_rules! avx512_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::avx512_xor!($word, $y, $x);
        $y = $crate::avx512_ror!($word, $y, $beta);

        $x = $crate::avx512_xor!($word, $x, $k);
        $x = $crate::avx512_sub!($word, $x, $y);
        $x = $crate::avx512_rol!($word, $x, $alpha);
    };
}
