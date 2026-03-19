use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

mod common;
use crate::common::criterion_config;
use common::define_cipher_bench;

define_cipher_bench!(
    scalar_32_64_bench,
    prefix = "32_64",
    key = [0, 0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_32_64,
    decrypt = speck::decrypt_block_32_64
);

define_cipher_bench!(
    scalar_48_72_bench,
    prefix = "48_72",
    key = [0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_48_72,
    decrypt = speck::decrypt_block_48_72
);

define_cipher_bench!(
    scalar_48_96_bench,
    prefix = "48_96",
    key = [0, 0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_48_96,
    decrypt = speck::decrypt_block_48_96
);

define_cipher_bench!(
    scalar_64_96_bench,
    prefix = "64_96",
    key = [0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_64_96,
    decrypt = speck::decrypt_block_64_96
);

define_cipher_bench!(
    scalar_64_128_bench,
    prefix = "64_128",
    key = [0, 0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_64_128,
    decrypt = speck::decrypt_block_64_128
);

define_cipher_bench!(
    scalar_96_96_bench,
    prefix = "96_96",
    key = [0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_96_96,
    decrypt = speck::decrypt_block_96_96
);

define_cipher_bench!(
    scalar_96_144_bench,
    prefix = "96_144",
    key = [0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_96_144,
    decrypt = speck::decrypt_block_96_144
);

define_cipher_bench!(
    scalar_128_128_bench,
    prefix = "128_128",
    key = [0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_128_128,
    decrypt = speck::decrypt_block_128_128
);

define_cipher_bench!(
    scalar_128_192_bench,
    prefix = "128_192",
    key = [0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_128_192,
    decrypt = speck::decrypt_block_128_192
);

define_cipher_bench!(
    scalar_128_256_bench,
    prefix = "128_256",
    key = [0, 0, 0, 0],
    pt = [0, 0],
    encrypt = speck::encrypt_block_128_256,
    decrypt = speck::decrypt_block_128_256
);

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("scalar");
    g.throughput(Throughput::Elements(1));

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

    g.finish()
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
