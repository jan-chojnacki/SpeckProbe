#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_engine_config;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use criterion::{SamplingMode, Throughput};
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use speck_probe::search::domain::key::Key;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use speck_probe::search::domain::task::Task;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::hint::black_box;

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_32_64_bench,
    bytes = 8,
    word = uint16x8_t,
    prefix = "32_64",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_32_64,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_72_bench,
    bytes = 9,
    word = uint32x4_t,
    prefix = "48_72",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_48_72,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_96_bench,
    bytes = 12,
    word = uint32x4_t,
    prefix = "48_96",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_48_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_96_bench,
    bytes = 12,
    word = uint32x4_t,
    prefix = "64_96",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_64_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_128_bench,
    bytes = 16,
    word = uint32x4_t,
    prefix = "64_128",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_64_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_96_bench,
    bytes = 12,
    word = uint64x2_t,
    prefix = "96_96",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_96_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_144_bench,
    bytes = 18,
    word = uint64x2_t,
    prefix = "96_144",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_96_144,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_128_bench,
    bytes = 16,
    word = uint64x2_t,
    prefix = "128_128",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_192_bench,
    bytes = 24,
    word = uint64x2_t,
    prefix = "128_192",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_192,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_256_bench,
    bytes = 32,
    word = uint64x2_t,
    prefix = "128_256",
    function = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_256,
    function_name = "encrypt_inflight"
);

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("engine/neon");

    unsafe {
        neon_32_64_bench(&mut g);
        neon_48_72_bench(&mut g);
        neon_48_96_bench(&mut g);
        neon_64_96_bench(&mut g);
        neon_64_128_bench(&mut g);
        neon_96_96_bench(&mut g);
        neon_96_144_bench(&mut g);
        neon_128_128_bench(&mut g);
        neon_128_192_bench(&mut g);
        neon_128_256_bench(&mut g);
    }

    g.finish();
}

#[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_engine_config();
    targets = benchmark
}

criterion_main!(benches);
