use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use std::arch::x86_64::{_mm256_set1_epi16, _mm256_set1_epi32, _mm256_set1_epi64x};

use benchmarks::criterion_config;
use benchmarks::define_cipher_bench;

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_32_64_bench,
    prefix = "32_64",
    key = [
        _mm256_set1_epi16(0),
        _mm256_set1_epi16(0),
        _mm256_set1_epi16(0),
        _mm256_set1_epi16(0),
    ],
    pt = [_mm256_set1_epi16(0), _mm256_set1_epi16(0)],
    encrypt = speck::avx2_encrypt_block_32_64,
    encrypt_inflight = speck::avx2_encrypt_block_32_64,
    decrypt = speck::avx2_decrypt_block_32_64
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_48_72_bench,
    prefix = "48_72",
    key = [
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
    ],
    pt = [_mm256_set1_epi32(0), _mm256_set1_epi32(0)],
    encrypt = speck::avx2_encrypt_block_48_72,
    encrypt_inflight = speck::avx2_encrypt_block_48_72,
    decrypt = speck::avx2_decrypt_block_48_72
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_48_96_bench,
    prefix = "48_96",
    key = [
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
    ],
    pt = [_mm256_set1_epi32(0), _mm256_set1_epi32(0)],
    encrypt = speck::avx2_encrypt_block_48_96,
    encrypt_inflight = speck::avx2_encrypt_block_48_96,
    decrypt = speck::avx2_decrypt_block_48_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_64_96_bench,
    prefix = "64_96",
    key = [
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
    ],
    pt = [_mm256_set1_epi32(0), _mm256_set1_epi32(0)],
    encrypt = speck::avx2_encrypt_block_64_96,
    encrypt_inflight = speck::avx2_encrypt_block_64_96,
    decrypt = speck::avx2_decrypt_block_64_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_64_128_bench,
    prefix = "64_128",
    key = [
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
        _mm256_set1_epi32(0),
    ],
    pt = [_mm256_set1_epi32(0), _mm256_set1_epi32(0)],
    encrypt = speck::avx2_encrypt_block_64_128,
    encrypt_inflight = speck::avx2_encrypt_block_64_128,
    decrypt = speck::avx2_decrypt_block_64_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_96_96_bench,
    prefix = "96_96",
    key = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0)],
    pt = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0)],
    encrypt = speck::avx2_encrypt_block_96_96,
    encrypt_inflight = speck::avx2_encrypt_block_96_96,
    decrypt = speck::avx2_decrypt_block_96_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_96_144_bench,
    prefix = "96_144",
    key = [
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
    ],
    pt = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0)],
    encrypt = speck::avx2_encrypt_block_96_144,
    encrypt_inflight = speck::avx2_encrypt_block_96_144,
    decrypt = speck::avx2_decrypt_block_96_144
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_128_128_bench,
    prefix = "128_128",
    key = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0),],
    pt = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0),],
    encrypt = speck::avx2_encrypt_block_128_128,
    encrypt_inflight = speck::avx2_encrypt_block_128_128,
    decrypt = speck::avx2_decrypt_block_128_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_128_192_bench,
    prefix = "128_192",
    key = [
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
    ],
    pt = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0),],
    encrypt = speck::avx2_encrypt_block_128_192,
    encrypt_inflight = speck::avx2_encrypt_block_128_192,
    decrypt = speck::avx2_decrypt_block_128_192
);

define_cipher_bench!(
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    avx2_128_256_bench,
    prefix = "128_256",
    key = [
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
        _mm256_set1_epi64x(0),
    ],
    pt = [_mm256_set1_epi64x(0), _mm256_set1_epi64x(0),],
    encrypt = speck::avx2_encrypt_block_128_256,
    encrypt_inflight = speck::avx2_encrypt_block_128_256,
    decrypt = speck::avx2_decrypt_block_128_256
);

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("speck/avx2");

    unsafe {
        g.throughput(Throughput::Elements(16));
        avx2_32_64_bench(&mut g);

        g.throughput(Throughput::Elements(8));
        avx2_48_72_bench(&mut g);
        avx2_48_96_bench(&mut g);
        avx2_64_96_bench(&mut g);
        avx2_64_128_bench(&mut g);

        g.throughput(Throughput::Elements(4));
        avx2_96_96_bench(&mut g);
        avx2_96_144_bench(&mut g);
        avx2_128_128_bench(&mut g);
        avx2_128_192_bench(&mut g);
        avx2_128_256_bench(&mut g);
    }

    g.finish();
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
