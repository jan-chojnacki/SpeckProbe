use crate::backend::avx::decrypt_round::*;
use crate::backend::avx::expand_key::*;
use paste::paste;
use std::arch::x86_64::__m128i;

macro_rules! impl_decrypt_block_avx {
    ($block:literal, $key:literal, $word:literal, $key_words:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_decrypt_block_ $block _ $key>](ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
                let round_keys = [<avx_expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for &k in round_keys.iter().rev() {
                    [<avx_decrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

impl_decrypt_block_avx!(32, 64, 16, 4);
impl_decrypt_block_avx!(48, 72, 24, 3);
impl_decrypt_block_avx!(48, 96, 24, 4);
impl_decrypt_block_avx!(64, 96, 32, 3);
impl_decrypt_block_avx!(64, 128, 32, 4);
impl_decrypt_block_avx!(96, 96, 48, 2);
impl_decrypt_block_avx!(96, 144, 48, 3);
impl_decrypt_block_avx!(128, 128, 64, 2);
impl_decrypt_block_avx!(128, 192, 64, 3);
impl_decrypt_block_avx!(128, 256, 64, 4);
