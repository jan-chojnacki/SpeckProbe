#[macro_export]
macro_rules! avx512_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::avx512_ror!($word, $x, $alpha);
        $x = $crate::avx512_add!($word, $x, $y);
        $x = $crate::avx512_xor!($word, $x, $k);

        $y = $crate::avx512_rol!($word, $y, $beta);
        $y = $crate::avx512_xor!($word, $y, $x);
    };
}
