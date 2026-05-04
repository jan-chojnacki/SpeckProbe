macro_rules! sse2_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $x = $crate::speck::backend::x86_64::sse2::sse2_ror!($word, $x, $alpha);
        $x = $crate::speck::backend::x86_64::sse2::sse2_add!($word, $x, $y);
        $x = $crate::speck::backend::x86_64::sse2::sse2_xor!($word, $x, $k);

        $y = $crate::speck::backend::x86_64::sse2::sse2_rol!($word, $y, $beta);
        $y = $crate::speck::backend::x86_64::sse2::sse2_xor!($word, $y, $x);
    };
}

pub(crate) use sse2_encrypt_round_inline;
