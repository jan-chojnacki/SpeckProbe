macro_rules! sse2_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $word:tt, $alpha:expr, $beta:expr) => {
        $y = $crate::backend::x86_64::sse2::sse2_xor!($word, $y, $x);
        $y = $crate::backend::x86_64::sse2::sse2_ror!($word, $y, $beta);

        $x = $crate::backend::x86_64::sse2::sse2_xor!($word, $x, $k);
        $x = $crate::backend::x86_64::sse2::sse2_sub!($word, $x, $y);
        $x = $crate::backend::x86_64::sse2::sse2_rol!($word, $x, $alpha);
    };
}

pub(crate) use sse2_decrypt_round_inline;