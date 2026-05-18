#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_slow_config;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use speck_probe::search::domain::key::Key;
use speck_probe::search::domain::task::Task;
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
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_32_64,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_32_64,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_32_64
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_48_72_bench,
    bytes = 9,
    word = __m128i,
    prefix = "48_72",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_48_72,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_48_72,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_48_72
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_48_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "48_96",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_48_96,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_48_96,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_48_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_64_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "64_96",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_64_96,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_64_96,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_64_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_64_128_bench,
    bytes = 16,
    word = __m128i,
    prefix = "64_128",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_64_128,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_64_128,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_64_128
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_96_96_bench,
    bytes = 12,
    word = __m128i,
    prefix = "96_96",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_96_96,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_96_96,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_96_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_96_144_bench,
    bytes = 18,
    word = __m128i,
    prefix = "96_144",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_96_144,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_96_144,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_96_144
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_128_bench,
    bytes = 16,
    word = __m128i,
    prefix = "128_128",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_128_128,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_128,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_128_128
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_192_bench,
    bytes = 24,
    word = __m128i,
    prefix = "128_192",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_128_192,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_192,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_128_192
);

define_engine_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    sse2_128_256_bench,
    bytes = 32,
    word = __m128i,
    prefix = "128_256",
    encrypt = speck_probe::search::executor::backend::sse2_search_encrypt_128_256,
    encrypt_inflight = speck_probe::search::executor::backend::sse2_search_encrypt_inflight_128_256,
    decrypt = speck_probe::search::executor::backend::sse2_search_decrypt_128_256
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
    config = criterion_slow_config();
    targets = benchmark
}

criterion_main!(benches);
