#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_slow_config;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use speck_probe::search::domain::key::Key;
use speck_probe::search::domain::task::Task;
use std::hint::black_box;

define_engine_bench!(
    scalar_32_64_bench,
    bytes = 8,
    word = u16,
    prefix = "32_64",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_32_64,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_48_72_bench,
    bytes = 9,
    word = u32,
    prefix = "48_72",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_48_72,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_48_96_bench,
    bytes = 12,
    word = u32,
    prefix = "48_96",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_48_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_64_96_bench,
    bytes = 12,
    word = u32,
    prefix = "64_96",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_64_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_64_128_bench,
    bytes = 16,
    word = u32,
    prefix = "64_128",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_64_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_96_96_bench,
    bytes = 12,
    word = u64,
    prefix = "96_96",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_96_96,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_96_144_bench,
    bytes = 18,
    word = u64,
    prefix = "96_144",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_96_144,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_128_128_bench,
    bytes = 16,
    word = u64,
    prefix = "128_128",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_128_128,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_128_192_bench,
    bytes = 24,
    word = u64,
    prefix = "128_192",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_128_192,
    function_name = "encrypt_inflight"
);

define_engine_bench!(
    scalar_128_256_bench,
    bytes = 32,
    word = u64,
    prefix = "128_256",
    function = speck_probe::search::executor::backend::scalar_search_encrypt_inflight_128_256,
    function_name = "encrypt_inflight"
);

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("engine/scalar");

    scalar_32_64_bench(&mut g);
    scalar_48_72_bench(&mut g);
    scalar_48_96_bench(&mut g);
    scalar_64_96_bench(&mut g);
    scalar_64_128_bench(&mut g);
    scalar_96_96_bench(&mut g);
    scalar_96_144_bench(&mut g);
    scalar_128_128_bench(&mut g);
    scalar_128_192_bench(&mut g);
    scalar_128_256_bench(&mut g);

    g.finish();
}

criterion_group! {
    name = benches;
    config = criterion_slow_config();
    targets = benchmark
}

criterion_main!(benches);
