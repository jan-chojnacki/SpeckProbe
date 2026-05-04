#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_config;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use criterion::Throughput;
use criterion::{Criterion, criterion_group, criterion_main};
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
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_32_64,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_32_64,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_32_64
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_72_bench,
    bytes = 9,
    word = uint32x4_t,
    prefix = "48_72",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_48_72,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_48_72,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_48_72
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_96_bench,
    bytes = 12,
    word = uint32x4_t,
    prefix = "48_96",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_48_96,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_48_96,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_48_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_96_bench,
    bytes = 12,
    word = uint32x4_t,
    prefix = "64_96",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_64_96,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_64_96,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_64_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_128_bench,
    bytes = 16,
    word = uint32x4_t,
    prefix = "64_128",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_64_128,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_64_128,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_64_128
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_96_bench,
    bytes = 12,
    word = uint64x2_t,
    prefix = "96_96",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_96_96,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_96_96,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_96_96
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_144_bench,
    bytes = 18,
    word = uint64x2_t,
    prefix = "96_144",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_96_144,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_96_144,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_96_144
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_128_bench,
    bytes = 16,
    word = uint64x2_t,
    prefix = "128_128",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_128_128,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_128,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_128_128
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_192_bench,
    bytes = 24,
    word = uint64x2_t,
    prefix = "128_192",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_128_192,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_192,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_128_192
);

define_engine_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_256_bench,
    bytes = 32,
    word = uint64x2_t,
    prefix = "128_256",
    encrypt = speck_probe::search::executor::backend::neon_search_encrypt_128_256,
    encrypt_inflight = speck_probe::search::executor::backend::neon_search_encrypt_inflight_128_256,
    decrypt = speck_probe::search::executor::backend::neon_search_decrypt_128_256
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
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
