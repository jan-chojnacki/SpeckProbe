use super::sse2_encrypt_round_inline;
use super::sse2_expand_key_inline;
use super::sse2_set;
use std::arch::x86_64::__m128i;

macro_rules! impl_encrypt_block_sse2 {
    ($fn_name:ident, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = "Encrypts one Speck block using SSE2."]
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
        pub fn $fn_name(ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
            let mut round_keys: [__m128i; $rounds + 1] = [core::arch::x86_64::_mm_setzero_si128(); $rounds + 1];
            sse2_expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq_macro::seq!(I in 0..=$rounds {
                sse2_encrypt_round_inline!(x, y, round_keys[I], $word, $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_encrypt_block_sse2!(sse2_encrypt_block_32_64, 16, 4, 7, 2, 21);
impl_encrypt_block_sse2!(sse2_encrypt_block_48_72, 24, 3, 8, 3, 21);
impl_encrypt_block_sse2!(sse2_encrypt_block_48_96, 24, 4, 8, 3, 22);
impl_encrypt_block_sse2!(sse2_encrypt_block_64_96, 32, 3, 8, 3, 25);
impl_encrypt_block_sse2!(sse2_encrypt_block_64_128, 32, 4, 8, 3, 26);
impl_encrypt_block_sse2!(sse2_encrypt_block_96_96, 48, 2, 8, 3, 27);
impl_encrypt_block_sse2!(sse2_encrypt_block_96_144, 48, 3, 8, 3, 28);
impl_encrypt_block_sse2!(sse2_encrypt_block_128_128, 64, 2, 8, 3, 31);
impl_encrypt_block_sse2!(sse2_encrypt_block_128_192, 64, 3, 8, 3, 32);
impl_encrypt_block_sse2!(sse2_encrypt_block_128_256, 64, 4, 8, 3, 33);

macro_rules! impl_encrypt_block_sse2_inflight {
    ($fn_name:ident, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = "Encrypts one Speck block using AVX."]
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
        pub fn $fn_name(ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
            let mut l = $crate::key_words_inline!(key, $key_words);
            let mut k = key[$key_words - 1];

            let mut x = ct[0];
            let mut y = ct[1];

            seq_macro::seq!(I in 0..$rounds {
                sse2_encrypt_round_inline!(x, y, k, $word, $alpha, $beta);
                sse2_encrypt_round_inline!(l[$crate::key_idx!($key_words, I)],
                    k, sse2_set!($word, I), $word, $alpha, $beta);
            });
            sse2_encrypt_round_inline!(x, y, k, $word, $alpha, $beta);

            [x, y]
        }
    };
}

impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_32_64, 16, 4, 7, 2, 21);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_48_72, 24, 3, 8, 3, 21);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_48_96, 24, 4, 8, 3, 22);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_64_96, 32, 3, 8, 3, 25);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_64_128, 32, 4, 8, 3, 26);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_96_96, 48, 2, 8, 3, 27);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_96_144, 48, 3, 8, 3, 28);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_128, 64, 2, 8, 3, 31);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_192, 64, 3, 8, 3, 32);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_256, 64, 4, 8, 3, 33);
