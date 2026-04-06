use crate::backend::macors::define_runtime_variants_default;
use engine::x86_64::avx512::converter::{
    avx512_u16x2_block_to_vec, avx512_u32x2_block_to_vec, avx512_u64x2_block_to_vec,
};
use std::arch::x86_64::__m512i;

define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
        #[target_feature(enable = "avx512bw")]
    ],
    avx512_32_64,
    32_64,
    __m512i,
    u16,
    converter = avx512_u16x2_block_to_vec,
    bytes = 8,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_48_72,
    48_72,
    __m512i,
    u32,
    converter = avx512_u32x2_block_to_vec,
    bytes = 9,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_48_96,
    48_96,
    __m512i,
    u32,
    converter = avx512_u32x2_block_to_vec,
    bytes = 12,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_64_96,
    64_96,
    __m512i,
    u32,
    converter = avx512_u32x2_block_to_vec,
    bytes = 12,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_64_128,
    64_128,
    __m512i,
    u32,
    converter = avx512_u32x2_block_to_vec,
    bytes = 16,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_96_96,
    96_96,
    __m512i,
    u64,
    converter = avx512_u64x2_block_to_vec,
    bytes = 12,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_96_144,
    96_144,
    __m512i,
    u64,
    converter = avx512_u64x2_block_to_vec,
    bytes = 18,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_128_128,
    128_128,
    __m512i,
    u64,
    converter = avx512_u64x2_block_to_vec,
    bytes = 16,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_128_192,
    128_192,
    __m512i,
    u64,
    converter = avx512_u64x2_block_to_vec,
    bytes = 24,
    simd = avx512_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        #[target_feature(enable = "avx512f")]
    ],
    avx512_128_256,
    128_256,
    __m512i,
    u64,
    converter = avx512_u64x2_block_to_vec,
    bytes = 32,
    simd = avx512_,
);
