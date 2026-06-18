#[path = "bench_common.rs"]
mod bench_common;

use bench_common::criterion_speck_config;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use criterion::Throughput;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use std::hint::black_box;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use std::arch::x86_64::{_mm512_set1_epi16, _mm512_set1_epi32, _mm512_set1_epi64};

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    #[target_feature(enable = "avx512bw")]
    avx512_32_64_bench,
    prefix = "32_64",
    key = [
        _mm512_set1_epi16(0),
        _mm512_set1_epi16(0),
        _mm512_set1_epi16(0),
        _mm512_set1_epi16(0),
    ],
    pt = [_mm512_set1_epi16(0), _mm512_set1_epi16(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_32_64,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_32_64,
    decrypt = speck_probe::speck::avx512_decrypt_block_32_64
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_48_72_bench,
    prefix = "48_72",
    key = [
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
    ],
    pt = [_mm512_set1_epi32(0), _mm512_set1_epi32(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_48_72,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_48_72,
    decrypt = speck_probe::speck::avx512_decrypt_block_48_72
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_48_96_bench,
    prefix = "48_96",
    key = [
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
    ],
    pt = [_mm512_set1_epi32(0), _mm512_set1_epi32(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_48_96,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_48_96,
    decrypt = speck_probe::speck::avx512_decrypt_block_48_96
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_64_96_bench,
    prefix = "64_96",
    key = [
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
    ],
    pt = [_mm512_set1_epi32(0), _mm512_set1_epi32(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_64_96,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_64_96,
    decrypt = speck_probe::speck::avx512_decrypt_block_64_96
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_64_128_bench,
    prefix = "64_128",
    key = [
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
        _mm512_set1_epi32(0),
    ],
    pt = [_mm512_set1_epi32(0), _mm512_set1_epi32(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_64_128,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_64_128,
    decrypt = speck_probe::speck::avx512_decrypt_block_64_128
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_96_96_bench,
    prefix = "96_96",
    key = [_mm512_set1_epi64(0), _mm512_set1_epi64(0)],
    pt = [_mm512_set1_epi64(0), _mm512_set1_epi64(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_96_96,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_96_96,
    decrypt = speck_probe::speck::avx512_decrypt_block_96_96
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_96_144_bench,
    prefix = "96_144",
    key = [
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
    ],
    pt = [_mm512_set1_epi64(0), _mm512_set1_epi64(0)],
    encrypt = speck_probe::speck::avx512_encrypt_block_96_144,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_96_144,
    decrypt = speck_probe::speck::avx512_decrypt_block_96_144
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_128_128_bench,
    prefix = "128_128",
    key = [_mm512_set1_epi64(0), _mm512_set1_epi64(0),],
    pt = [_mm512_set1_epi64(0), _mm512_set1_epi64(0),],
    encrypt = speck_probe::speck::avx512_encrypt_block_128_128,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_128_128,
    decrypt = speck_probe::speck::avx512_decrypt_block_128_128
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_128_192_bench,
    prefix = "128_192",
    key = [
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
    ],
    pt = [_mm512_set1_epi64(0), _mm512_set1_epi64(0),],
    encrypt = speck_probe::speck::avx512_encrypt_block_128_192,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_128_192,
    decrypt = speck_probe::speck::avx512_decrypt_block_128_192
);

define_speck_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    #[target_feature(enable = "avx512f")]
    avx512_128_256_bench,
    prefix = "128_256",
    key = [
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
        _mm512_set1_epi64(0),
    ],
    pt = [_mm512_set1_epi64(0), _mm512_set1_epi64(0),],
    encrypt = speck_probe::speck::avx512_encrypt_block_128_256,
    encrypt_inflight = speck_probe::speck::avx512_encrypt_block_128_256,
    decrypt = speck_probe::speck::avx512_decrypt_block_128_256
);

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("speck/avx512");

    unsafe {
        g.throughput(Throughput::Elements(32));
        avx512_32_64_bench(&mut g);

        g.throughput(Throughput::Elements(16));
        avx512_48_72_bench(&mut g);
        avx512_48_96_bench(&mut g);
        avx512_64_96_bench(&mut g);
        avx512_64_128_bench(&mut g);

        g.throughput(Throughput::Elements(8));
        avx512_96_96_bench(&mut g);
        avx512_96_144_bench(&mut g);
        avx512_128_128_bench(&mut g);
        avx512_128_192_bench(&mut g);
        avx512_128_256_bench(&mut g);
    }

    g.finish();
}

#[cfg(not(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw"
)))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_speck_config();
    targets = benchmark
}

criterion_main!(benches);
