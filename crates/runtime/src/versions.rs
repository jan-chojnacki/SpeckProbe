use crate::runtime::Runtime;
use engine::domain::key::Key;
use paste::paste;
use std::arch::x86_64::__m256i;

macro_rules! define_runtime {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        $bytes:literal,
        $prefix:literal,
        $engine_word:ty,
        $validator_word:ty,
        $converter:expr,
        $version:tt
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
                engine::[<validate_encrypt_ $version>],
                |block| ($converter)(block),
            );

            runtime.run()
        }
    }};
}

define_runtime!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    runtime2,
    8,
    7,
    __m256i,
    u16,
    engine::backend::x86_64::avx2::converter::avx2_u16x2_block_to_vec,
    32_64,
    avx2_
);

define_runtime!(runtime1, 8, 7, u16, u16, |x| x, 32_64,);
