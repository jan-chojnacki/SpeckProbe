use crate::backend::avx2::encrypt_round::*;
use crate::backend::avx2::expand_key::*;
use paste::paste;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m256i;

macro_rules! impl_encrypt_block_avx2 {
    ($block:literal, $key:literal, $word:literal, $key_words:literal) => {
        paste! {
            #[doc = concat!(
                "Encrypts one Speck block (",
                stringify!($block),
                "/",
                stringify!($key),
                ") using AVX2."
            )]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = concat!(
                "Caller must ensure CPU support for `avx2` before calling this function."
            )]
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_encrypt_block_ $block _ $key>](ct: [__m256i; 2], key: [__m256i; $key_words]) -> [__m256i; 2] {
                let round_keys = [<avx2_expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for k in round_keys {
                    [<avx2_encrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

impl_encrypt_block_avx2!(32, 64, 16, 4);
impl_encrypt_block_avx2!(48, 72, 24, 3);
impl_encrypt_block_avx2!(48, 96, 24, 4);
impl_encrypt_block_avx2!(64, 96, 32, 3);
impl_encrypt_block_avx2!(64, 128, 32, 4);
impl_encrypt_block_avx2!(96, 96, 48, 2);
impl_encrypt_block_avx2!(96, 144, 48, 3);
impl_encrypt_block_avx2!(128, 128, 64, 2);
impl_encrypt_block_avx2!(128, 192, 64, 3);
impl_encrypt_block_avx2!(128, 256, 64, 4);
