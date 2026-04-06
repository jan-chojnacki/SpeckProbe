use crate::runtime::Runtime;
use paste::paste;
#[cfg(target_arch = "x86_64")]
mod x86_64 {
    pub use engine::x86_64::avx2::converter::{
        avx2_u16x2_block_to_vec, avx2_u32x2_block_to_vec, avx2_u64x2_block_to_vec,
    };
    pub use engine::x86_64::avx512::converter::{
        avx512_u16x2_block_to_vec, avx512_u32x2_block_to_vec, avx512_u64x2_block_to_vec,
    };
    pub use engine::x86_64::sse2::converter::{
        sse2_u16x2_block_to_vec, sse2_u32x2_block_to_vec, sse2_u64x2_block_to_vec,
    };
    pub use std::arch::x86_64::{__m128i, __m256i, __m512i};
}
#[cfg(target_arch = "x86_64")]
use x86_64::*;
#[cfg(target_arch = "aarch64")]
mod aarch64 {
    pub use engine::aarch64::neon::converter::{
        neon_u16x2_block_to_vec, neon_u32x2_block_to_vec, neon_u64x2_block_to_vec,
    };
    pub use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};
}
use crate::dispatch::CipherMode;
#[cfg(target_arch = "aarch64")]
use aarch64::*;
use speck::SpeckVersion;

pub struct RuntimeRequest {
    pub cipher_config: CipherConfig,
    pub runtime_config: RuntimeConfig,
    pub search_space: SearchSpace,
}

pub struct RuntimeConfig {
    pub suffix_bytes_size: usize,
    pub num_threads: usize,
    pub cap: usize,
}

pub struct CipherConfig {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
}

pub struct SearchSpace {
    pub start: Vec<u8>,
    pub end: Vec<u8>,
    pub data: Vec<[u64; 2]>,
    pub expected: Vec<[u64; 2]>,
}

macro_rules! define_runtime {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        $bytes:literal,
        $suffix:literal,
        $engine_word:ty,
        $validator_word:ty,
        $converter:expr,
        $version:tt,
        $mode:tt
        $(, $simd:tt)?
        $(,)?
    ) => {paste! {
        $(#[$meta])*
        pub fn $fn_name(
            runtime_request: RuntimeRequest,
        ) -> (Vec<Vec<u8>>, Option<Vec<u8>>) {
            let start: [u8; { $bytes - $suffix }] = runtime_request.search_space.start
                .try_into()
                .expect("start length mismatch");
            let end: [u8; { $bytes - $suffix }] = runtime_request.search_space.end
                .try_into()
                .expect("end length mismatch");

            let data: Vec<[$validator_word; 2]> = runtime_request.search_space.data
                .iter()
                .map(|[a, b]| [*a as $validator_word, *b as $validator_word])
                .collect();
            let expected: Vec<[$validator_word; 2]> = runtime_request.search_space.expected
                .iter()
                .map(|[a, b]| [*a as $validator_word, *b as $validator_word])
                .collect();

            let mut runtime = Runtime::<_, _, $engine_word, $validator_word, $bytes, { $bytes - $suffix }>::new(
                start,
                end,
                &data,
                &expected,
                runtime_request.runtime_config.num_threads,
                runtime_request.runtime_config.cap,
                |task, out| engine::[<$($simd)? search_encrypt_inflight_ $version>](task, out),
                engine::[<$mode _validate_encrypt_ $version>],
                |block| ($converter)(block),
            );

            let (keys, found) = runtime.run();
            (
                keys.into_iter().map(|k| k.to_vec()).collect(),
                found.map(|k| k.to_vec()),
            )
        }
    }};
}

macro_rules! define_runtime_with_attrs {
    ([$($attrs:tt)*], $($rest:tt)*) => {
        define_runtime!($($attrs)* $($rest)*);
    };
}

macro_rules! define_runtime_for_mode {
    (
     $base:ident, $version:tt, $engine_word:ty, $converter:expr,
     $prefix:literal, $mode:tt, [$($suffix:literal),+ $(,)?]) => {
        $( paste! {
            define_runtime_with_attrs!(
                [],
                [< $base _s $suffix _ $mode _runtime >],
                $prefix, $suffix, $engine_word, $engine_word, $converter, $version, $mode
            );
        } )+
    };

    (
     $attrs:tt,
     $base:ident, $version:tt, $engine_word:ty, $validator_word:ty, $converter:expr,
     $prefix:literal, $mode:tt, [$($suffix:literal),+ $(,)?], $simd:tt) => {
        $( paste! {
            define_runtime_with_attrs!(
                $attrs,
                [< $base _s $suffix _ $mode _runtime >],
                $prefix, $suffix, $engine_word, $validator_word, $converter, $version, $mode, $simd
            );
        } )+
    };
}

macro_rules! define_runtime_variants {
    (
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $converter:expr,
        bytes    = $bytes:literal,
        suffixes = $suffixes:tt,
        modes    = [$($mode:tt),+ $(,)?]
        $(,)?
    ) => {
        $( define_runtime_for_mode!($base, $version, $engine_word, $converter, $bytes, $mode,
                                    $suffixes); )+
    };

    (
        attrs = $attrs:tt,
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $validator_word:ty,
        $converter:expr,
        bytes    = $bytes:literal,
        suffixes = $suffixes:tt,
        modes    = [$($mode:tt),+ $(,)?],
        simd     = $simd:tt
        $(,)?
    ) => {
        $( define_runtime_for_mode!($attrs, $base, $version, $engine_word, $validator_word,
                                    $converter, $bytes, $mode, $suffixes, $simd); )+
    };
}

macro_rules! define_runtime_variants_default {
    (
        attrs = $attrs:tt,
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $validator_word:ty,
        converter = $converter:expr,
        bytes = $bytes:literal
        $(, simd = $simd:tt)?
        $(,)?
    ) => {
        define_runtime_variants! {
            attrs = $attrs,
            $base,
            $version,
            $engine_word,
            $validator_word,
            $converter,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            $(simd = $simd,)?
        }
    };

    (
        $base:ident,
        $version:tt,
        $engine_word:ty,
        bytes = $bytes:literal
        $(,)?
    ) => {
        define_runtime_variants! {
            $base,
            $version,
            $engine_word,
            |x| x,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
        }
    };
}

define_runtime_variants_default!(scalar_32_64, 32_64, u16, bytes = 8);
define_runtime_variants_default!(scalar_48_72, 48_72, u32, bytes = 9);
define_runtime_variants_default!(scalar_48_96, 48_96, u32, bytes = 12);
define_runtime_variants_default!(scalar_64_96, 64_96, u32, bytes = 12);
define_runtime_variants_default!(scalar_64_128, 64_128, u32, bytes = 16);
define_runtime_variants_default!(scalar_96_96, 96_96, u64, bytes = 12);
define_runtime_variants_default!(scalar_96_144, 96_144, u64, bytes = 18);
define_runtime_variants_default!(scalar_128_128, 128_128, u64, bytes = 16);
define_runtime_variants_default!(scalar_128_192, 128_192, u64, bytes = 24);
define_runtime_variants_default!(scalar_128_256, 128_256, u64, bytes = 32);

define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_32_64,
    32_64,
    __m128i,
    u16,
    converter = sse2_u16x2_block_to_vec,
    bytes = 8,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_48_72,
    48_72,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 9,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_48_96,
    48_96,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_64_96,
    64_96,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_64_128,
    64_128,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 16,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_96_96,
    96_96,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_96_144,
    96_144,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 18,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_128,
    128_128,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 16,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_192,
    128_192,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 24,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_256,
    128_256,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 32,
    simd = sse2_,
);

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

define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_32_64,
    32_64,
    uint16x8_t,
    u16,
    converter = neon_u16x2_block_to_vec,
    bytes = 8,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_48_72,
    48_72,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 9,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_48_96,
    48_96,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_64_96,
    64_96,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_64_128,
    64_128,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 16,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_96_96,
    96_96,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_96_144,
    96_144,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 18,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_128_128,
    128_128,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 16,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_128_192,
    128_192,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 24,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        #[target_feature(enable = "neon")]
    ],
    neon_128_256,
    128_256,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 32,
    simd = neon_,
);
