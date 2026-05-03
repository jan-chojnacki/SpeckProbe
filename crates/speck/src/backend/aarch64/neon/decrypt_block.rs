use super::neon_decrypt_round_inline;
use super::neon_expand_key_inline;
use super::neon_set;
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

macro_rules! impl_decrypt_block_neon {
    ($fn_name:ident, $word:tt, $vec_type:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = "Decrypts one Speck block using NEON."]
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
        #[cfg(all(target_arch = "aarch64"))]
        #[target_feature(enable = "neon")]
        pub fn $fn_name(ct: [$vec_type; 2], key: [$vec_type; $key_words]) -> [$vec_type; 2] {
            let mut round_keys: [$vec_type; $rounds + 1] = [neon_set!($word, 0); $rounds + 1];
            neon_expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq_macro::seq!(I in 0..=$rounds {
                neon_decrypt_round_inline!(x, y, round_keys[$rounds - I], $word, $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_decrypt_block_neon!(neon_decrypt_block_32_64, 16, uint16x8_t, 4, 7, 2, 21);
impl_decrypt_block_neon!(neon_decrypt_block_48_72, 24, uint32x4_t, 3, 8, 3, 21);
impl_decrypt_block_neon!(neon_decrypt_block_48_96, 24, uint32x4_t, 4, 8, 3, 22);
impl_decrypt_block_neon!(neon_decrypt_block_64_96, 32, uint32x4_t, 3, 8, 3, 25);
impl_decrypt_block_neon!(neon_decrypt_block_64_128, 32, uint32x4_t, 4, 8, 3, 26);
impl_decrypt_block_neon!(neon_decrypt_block_96_96, 48, uint64x2_t, 2, 8, 3, 27);
impl_decrypt_block_neon!(neon_decrypt_block_96_144, 48, uint64x2_t, 3, 8, 3, 28);
impl_decrypt_block_neon!(neon_decrypt_block_128_128, 64, uint64x2_t, 2, 8, 3, 31);
impl_decrypt_block_neon!(neon_decrypt_block_128_192, 64, uint64x2_t, 3, 8, 3, 32);
impl_decrypt_block_neon!(neon_decrypt_block_128_256, 64, uint64x2_t, 4, 8, 3, 33);
