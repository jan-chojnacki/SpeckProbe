#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_engine_config;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use criterion::{Throughput, SamplingMode};
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use speck_probe::search::domain::key::Key;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use speck_probe::search::domain::task::Task;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use std::hint::black_box;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use std::arch::x86_64::__m128i;

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_32_64_bench,
    bytes = 8,
    word = __m128i,
    prefix = "32_64",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_32_64,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_48_72_bench,
    bytes = 9,
    word = __m128i,
    prefix = "48_72",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_48_72,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_48_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "48_96",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_48_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_64_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "64_96",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_64_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_64_128_bench,
    bytes = 16,
    word = __m128i,
    prefix = "64_128",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_64_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_96_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "96_96",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_96_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_96_144_bench,
    bytes = 18,
    word = __m128i,
    prefix = "96_144",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_96_144,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_128_bench,
    bytes = 16,
    word = __m128i,
    prefix = "128_128",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_192_bench,
    bytes = 24,
    word = __m128i,
    prefix = "128_192",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_192,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_256_bench,
    bytes = 32,
    word = __m128i,
    prefix = "128_256",
    function = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_256,
    function_name = "encrypt_inflight"
);

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("engine/sse2");

    unsafe {
        sse2_32_64_bench(&mut g);
        sse2_48_72_bench(&mut g);
        sse2_48_96_bench(&mut g);
        sse2_64_96_bench(&mut g);
        sse2_64_128_bench(&mut g);
        sse2_96_96_bench(&mut g);
        sse2_96_144_bench(&mut g);
        sse2_128_128_bench(&mut g);
        sse2_128_192_bench(&mut g);
        sse2_128_256_bench(&mut g);
    }

    g.finish();
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_engine_config();
    targets = benchmark
}

criterion_main!(benches);
