use crate::backend::macros::define_search;
use crate::backend::scalar::comparator::scalar_block_compare;
use paste::paste;
use speck::SpeckVersion;

define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = u16,
    comparator = scalar_block_compare,
    key_conversion = as_u16x4_le,
    new_key = new_key,
    next_key = next_into,
    name = 32_64,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = u32,
    comparator = scalar_block_compare,
    key_conversion = as_u24x3_le,
    new_key = new_key,
    next_key = next_into,
    name = 48_72,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = u32,
    comparator = scalar_block_compare,
    key_conversion = as_u24x4_le,
    new_key = new_key,
    next_key = next_into,
    name = 48_96,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = u32,
    comparator = scalar_block_compare,
    key_conversion = as_u32x3_le,
    new_key = new_key,
    next_key = next_into,
    name = 64_96,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = u32,
    comparator = scalar_block_compare,
    key_conversion = as_u32x4_le,
    new_key = new_key,
    next_key = next_into,
    name = 64_128,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = u64,
    comparator = scalar_block_compare,
    key_conversion = as_u48x2_le,
    new_key = new_key,
    next_key = next_into,
    name = 96_96,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = u64,
    comparator = scalar_block_compare,
    key_conversion = as_u48x3_le,
    new_key = new_key,
    next_key = next_into,
    name = 96_144,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = u64,
    comparator = scalar_block_compare,
    key_conversion = as_u64x2_le,
    new_key = new_key,
    next_key = next_into,
    name = 128_128,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = u64,
    comparator = scalar_block_compare,
    key_conversion = as_u64x3_le,
    new_key = new_key,
    next_key = next_into,
    name = 128_192,
    simd = scalar
);
define_search!(
    #[inline(always)]
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = u64,
    comparator = scalar_block_compare,
    key_conversion = as_u64x4_le,
    new_key = new_key,
    next_key = next_into,
    name = 128_256,
    simd = scalar
);
