use crate::runtime::Runtime;
use engine::domain::key::Key;
use paste::paste;
use std::arch::x86_64::{__m128i, __m256i, __m512i};

macro_rules! define_runtime {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        $bytes:literal,
        $prefix:literal,
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
            start: [u8; $prefix],
            end: [u8; $prefix],
            data: &[[$validator_word; 2]],
            expected: &[[$validator_word; 2]],
            num_threads: usize,
            cap: usize,
        ) -> (Vec<Key<$bytes, $prefix>>, Option<Key<$bytes, $prefix>>) {
            let mut runtime = Runtime::<_, _, $engine_word, $validator_word, $bytes, $prefix>::new(
                start,
                end,
                data,
                expected,
                num_threads,
                cap,
                |task, out| engine::[<$($simd)? search_encrypt_inflight_ $version>](task, out),
                engine::[<$mode _validate_encrypt_ $version>],
                |block| ($converter)(block),
            );

            runtime.run()
        }
    }};
}

define_runtime!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_runtime,
    8,
    5,
    __m128i,
    u16,
    engine::backend::x86_64::sse2::converter::sse2_u16x2_block_to_vec,
    32_64,
    ecb,
    sse2_
);

define_runtime!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_runtime,
    8,
    5,
    __m256i,
    u16,
    engine::backend::x86_64::avx2::converter::avx2_u16x2_block_to_vec,
    32_64,
    ecb,
    avx2_
);

define_runtime!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    #[target_feature(enable = "avx512bw")]
    avx512_runtime,
    8,
    5,
    __m512i,
    u16,
    engine::backend::x86_64::avx512::converter::avx512_u16x2_block_to_vec,
    32_64,
    ecb,
    avx512_
);

define_runtime!(scalar_runtime, 8, 5, u16, u16, |x| x, 32_64, ecb);
