use crate::backend::macors::define_runtime_variants_default;
use engine::x86_64::avx2::converter::{
    avx2_u16x2_block_to_vec, avx2_u32x2_block_to_vec, avx2_u64x2_block_to_vec,
};
use std::arch::x86_64::__m256i;

define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_32_64,
    32_64,
    __m256i,
    u16,
    converter = avx2_u16x2_block_to_vec,
    bytes = 8,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_48_72,
    48_72,
    __m256i,
    u32,
    converter = avx2_u32x2_block_to_vec,
    bytes = 9,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_48_96,
    48_96,
    __m256i,
    u32,
    converter = avx2_u32x2_block_to_vec,
    bytes = 12,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_64_96,
    64_96,
    __m256i,
    u32,
    converter = avx2_u32x2_block_to_vec,
    bytes = 12,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_64_128,
    64_128,
    __m256i,
    u32,
    converter = avx2_u32x2_block_to_vec,
    bytes = 16,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_96_96,
    96_96,
    __m256i,
    u64,
    converter = avx2_u64x2_block_to_vec,
    bytes = 12,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_96_144,
    96_144,
    __m256i,
    u64,
    converter = avx2_u64x2_block_to_vec,
    bytes = 18,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_128_128,
    128_128,
    __m256i,
    u64,
    converter = avx2_u64x2_block_to_vec,
    bytes = 16,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_128_192,
    128_192,
    __m256i,
    u64,
    converter = avx2_u64x2_block_to_vec,
    bytes = 24,
    simd = avx2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        #[target_feature(enable = "avx2")]
    ],
    avx2_128_256,
    128_256,
    __m256i,
    u64,
    converter = avx2_u64x2_block_to_vec,
    bytes = 32,
    simd = avx2_,
);
