use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
use std::arch::x86_64::{_mm_set1_epi16, _mm_set1_epi32, _mm_set1_epi64x};

mod common;
use crate::common::criterion_config;
use common::define_cipher_bench;

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_32_64_bench,
    prefix = "32_64",
    key = [
        _mm_set1_epi16(0),
        _mm_set1_epi16(0),
        _mm_set1_epi16(0),
        _mm_set1_epi16(0),
    ],
    pt = [_mm_set1_epi16(0), _mm_set1_epi16(0)],
    encrypt = speck::avx_encrypt_block_32_64,
    decrypt = speck::avx_decrypt_block_32_64
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_48_72_bench,
    prefix = "48_72",
    key = [_mm_set1_epi32(0), _mm_set1_epi32(0), _mm_set1_epi32(0),],
    pt = [_mm_set1_epi32(0), _mm_set1_epi32(0)],
    encrypt = speck::avx_encrypt_block_48_72,
    decrypt = speck::avx_decrypt_block_48_72
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_48_96_bench,
    prefix = "48_96",
    key = [
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
    ],
    pt = [_mm_set1_epi32(0), _mm_set1_epi32(0)],
    encrypt = speck::avx_encrypt_block_48_96,
    decrypt = speck::avx_decrypt_block_48_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_64_96_bench,
    prefix = "64_96",
    key = [_mm_set1_epi32(0), _mm_set1_epi32(0), _mm_set1_epi32(0),],
    pt = [_mm_set1_epi32(0), _mm_set1_epi32(0)],
    encrypt = speck::avx_encrypt_block_64_96,
    decrypt = speck::avx_decrypt_block_64_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_64_128_bench,
    prefix = "64_128",
    key = [
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
        _mm_set1_epi32(0),
    ],
    pt = [_mm_set1_epi32(0), _mm_set1_epi32(0)],
    encrypt = speck::avx_encrypt_block_64_128,
    decrypt = speck::avx_decrypt_block_64_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_96_96_bench,
    prefix = "96_96",
    key = [_mm_set1_epi64x(0), _mm_set1_epi64x(0)],
    pt = [_mm_set1_epi64x(0), _mm_set1_epi64x(0)],
    encrypt = speck::avx_encrypt_block_96_96,
    decrypt = speck::avx_decrypt_block_96_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_96_144_bench,
    prefix = "96_144",
    key = [_mm_set1_epi64x(0), _mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    pt = [_mm_set1_epi64x(0), _mm_set1_epi64x(0)],
    encrypt = speck::avx_encrypt_block_96_144,
    decrypt = speck::avx_decrypt_block_96_144
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_128_128_bench,
    prefix = "128_128",
    key = [_mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    pt = [_mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    encrypt = speck::avx_encrypt_block_128_128,
    decrypt = speck::avx_decrypt_block_128_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_128_192_bench,
    prefix = "128_192",
    key = [_mm_set1_epi64x(0), _mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    pt = [_mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    encrypt = speck::avx_encrypt_block_128_192,
    decrypt = speck::avx_decrypt_block_128_192
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    avx_128_256_bench,
    prefix = "128_256",
    key = [
        _mm_set1_epi64x(0),
        _mm_set1_epi64x(0),
        _mm_set1_epi64x(0),
        _mm_set1_epi64x(0),
    ],
    pt = [_mm_set1_epi64x(0), _mm_set1_epi64x(0),],
    encrypt = speck::avx_encrypt_block_128_256,
    decrypt = speck::avx_decrypt_block_128_256
);

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("avx");
    g.throughput(Throughput::Elements(4));

    unsafe {
        avx_32_64_bench(&mut g);
        avx_48_72_bench(&mut g);
        avx_48_96_bench(&mut g);
        avx_64_96_bench(&mut g);
        avx_64_128_bench(&mut g);
        avx_96_96_bench(&mut g);
        avx_96_144_bench(&mut g);
        avx_128_128_bench(&mut g);
        avx_128_192_bench(&mut g);
        avx_128_256_bench(&mut g);
    }

    g.finish();
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "avx")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
