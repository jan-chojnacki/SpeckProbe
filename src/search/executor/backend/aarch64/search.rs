use super::neon::comparator::{
    neon_block_compare_u16, neon_block_compare_u32, neon_block_compare_u64,
};
use crate::search::executor::backend::macros::define_search;
use crate::speck::SpeckVersion;
use paste::paste;
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = uint16x8_t,
    comparator = neon_block_compare_u16,
    key_conversion = neon_u16x4_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 32_64,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u24x3_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 48_72,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u24x4_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 48_96,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u32x3_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 64_96,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u32x4_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 64_128,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u48x2_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 96_96,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u48x3_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 96_144,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x2_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 128_128,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x3_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 128_192,
    simd = neon
);
define_search!(
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x4_key,
    new_key = neon_new_key,
    next_key = simd_next_into,
    name = 128_256,
    simd = neon
);
