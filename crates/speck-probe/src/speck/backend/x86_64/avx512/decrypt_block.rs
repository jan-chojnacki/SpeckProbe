use std::arch::x86_64::__m512i;
use super::avx512_expand_key_inline;
use super::avx512_decrypt_round_inline;

macro_rules! impl_decrypt_block_avx512 {
    ($fn_name:ident, $feature:literal, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = "Decrypts one Speck block using avx512."]
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `avx512` before calling this function."]
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = $feature)]
        pub fn $fn_name(ct: [__m512i; 2], key: [__m512i; $key_words]) -> [__m512i; 2] {
            let mut round_keys: [__m512i; $rounds + 1] = [core::arch::x86_64::_mm512_setzero_si512(); $rounds + 1];
            avx512_expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq_macro::seq!(I in 0..=$rounds {
                avx512_decrypt_round_inline!(x, y, round_keys[$rounds - I], $word, $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_decrypt_block_avx512!(avx512_decrypt_block_32_64, "avx512bw", 16, 4, 7, 2, 21);
impl_decrypt_block_avx512!(avx512_decrypt_block_48_72, "avx512f", 24, 3, 8, 3, 21);
impl_decrypt_block_avx512!(avx512_decrypt_block_48_96, "avx512f", 24, 4, 8, 3, 22);
impl_decrypt_block_avx512!(avx512_decrypt_block_64_96, "avx512f", 32, 3, 8, 3, 25);
impl_decrypt_block_avx512!(avx512_decrypt_block_64_128, "avx512f", 32, 4, 8, 3, 26);
impl_decrypt_block_avx512!(avx512_decrypt_block_96_96, "avx512f", 48, 2, 8, 3, 27);
impl_decrypt_block_avx512!(avx512_decrypt_block_96_144, "avx512f", 48, 3, 8, 3, 28);
impl_decrypt_block_avx512!(avx512_decrypt_block_128_128, "avx512f", 64, 2, 8, 3, 31);
impl_decrypt_block_avx512!(avx512_decrypt_block_128_192, "avx512f", 64, 3, 8, 3, 32);
impl_decrypt_block_avx512!(avx512_decrypt_block_128_256, "avx512f", 64, 4, 8, 3, 33);
