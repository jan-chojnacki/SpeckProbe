use benchmarks::{criterion_config, define_engine_bench};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use engine::domain::key::Key;
use engine::domain::task::Task;
use std::hint::black_box;

define_engine_bench!(
    scalar_32_64_bench,
    bytes = 8,
    word = u16,
    prefix = "32_64",
    encrypt = engine::search_encrypt_32_64,
    encrypt_inflight = engine::search_encrypt_inflight_32_64,
    decrypt = engine::search_decrypt_32_64
);

define_engine_bench!(
    scalar_48_72_bench,
    bytes = 9,
    word = u32,
    prefix = "48_72",
    encrypt = engine::search_encrypt_48_72,
    encrypt_inflight = engine::search_encrypt_inflight_48_72,
    decrypt = engine::search_decrypt_48_72
);

define_engine_bench!(
    scalar_48_96_bench,
    bytes = 12,
    word = u32,
    prefix = "48_96",
    encrypt = engine::search_encrypt_48_96,
    encrypt_inflight = engine::search_encrypt_inflight_48_96,
    decrypt = engine::search_decrypt_48_96
);

define_engine_bench!(
    scalar_64_96_bench,
    bytes = 12,
    word = u32,
    prefix = "64_96",
    encrypt = engine::search_encrypt_64_96,
    encrypt_inflight = engine::search_encrypt_inflight_64_96,
    decrypt = engine::search_decrypt_64_96
);

define_engine_bench!(
    scalar_64_128_bench,
    bytes = 16,
    word = u32,
    prefix = "64_128",
    encrypt = engine::search_encrypt_64_128,
    encrypt_inflight = engine::search_encrypt_inflight_64_128,
    decrypt = engine::search_decrypt_64_128
);

define_engine_bench!(
    scalar_96_96_bench,
    bytes = 12,
    word = u64,
    prefix = "96_96",
    encrypt = engine::search_encrypt_96_96,
    encrypt_inflight = engine::search_encrypt_inflight_96_96,
    decrypt = engine::search_decrypt_96_96
);

define_engine_bench!(
    scalar_96_144_bench,
    bytes = 18,
    word = u64,
    prefix = "96_144",
    encrypt = engine::search_encrypt_96_144,
    encrypt_inflight = engine::search_encrypt_inflight_96_144,
    decrypt = engine::search_decrypt_96_144
);

define_engine_bench!(
    scalar_128_128_bench,
    bytes = 16,
    word = u64,
    prefix = "128_128",
    encrypt = engine::search_encrypt_128_128,
    encrypt_inflight = engine::search_encrypt_inflight_128_128,
    decrypt = engine::search_decrypt_128_128
);

define_engine_bench!(
    scalar_128_192_bench,
    bytes = 24,
    word = u64,
    prefix = "128_192",
    encrypt = engine::search_encrypt_128_192,
    encrypt_inflight = engine::search_encrypt_inflight_128_192,
    decrypt = engine::search_decrypt_128_192
);

define_engine_bench!(
    scalar_128_256_bench,
    bytes = 32,
    word = u64,
    prefix = "128_256",
    encrypt = engine::search_encrypt_128_256,
    encrypt_inflight = engine::search_encrypt_inflight_128_256,
    decrypt = engine::search_decrypt_128_256
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
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
