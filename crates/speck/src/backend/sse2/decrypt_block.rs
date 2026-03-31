use std::arch::x86_64::__m128i;

macro_rules! impl_decrypt_block_sse2 {
    ($fn_name:ident, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = "Decrypts one Speck block using AVX."]
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
        pub fn $fn_name(ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
            let mut round_keys: [__m128i; $rounds + 1] = [core::arch::x86_64::_mm_setzero_si128(); $rounds + 1];
            $crate::sse2_expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq_macro::seq!(I in 0..=$rounds {
                $crate::sse2_decrypt_round_inline!(x, y, round_keys[$rounds - I], $word, $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_decrypt_block_sse2!(sse2_decrypt_block_32_64, 16, 4, 7, 2, 21);
impl_decrypt_block_sse2!(sse2_decrypt_block_48_72, 24, 3, 8, 3, 21);
impl_decrypt_block_sse2!(sse2_decrypt_block_48_96, 24, 4, 8, 3, 22);
impl_decrypt_block_sse2!(sse2_decrypt_block_64_96, 32, 3, 8, 3, 25);
impl_decrypt_block_sse2!(sse2_decrypt_block_64_128, 32, 4, 8, 3, 26);
impl_decrypt_block_sse2!(sse2_decrypt_block_96_96, 48, 2, 8, 3, 27);
impl_decrypt_block_sse2!(sse2_decrypt_block_96_144, 48, 3, 8, 3, 28);
impl_decrypt_block_sse2!(sse2_decrypt_block_128_128, 64, 2, 8, 3, 31);
impl_decrypt_block_sse2!(sse2_decrypt_block_128_192, 64, 3, 8, 3, 32);
impl_decrypt_block_sse2!(sse2_decrypt_block_128_256, 64, 4, 8, 3, 33);
