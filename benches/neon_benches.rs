use std::arch::aarch64::{vdupq_n_u16, vdupq_n_u32, vdupq_n_u64};
use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]


mod common;
use crate::common::criterion_config;
use common::define_cipher_bench;

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_32_64_bench,
    prefix = "32_64",
    key = [
        vdupq_n_u16(0),
        vdupq_n_u16(0),
        vdupq_n_u16(0),
        vdupq_n_u16(0),
    ],
    pt = [vdupq_n_u16(0), vdupq_n_u16(0)],
    encrypt = speck::neon_encrypt_block_32_64,
    decrypt = speck::neon_decrypt_block_32_64
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_72_bench,
    prefix = "48_72",
    key = [vdupq_n_u32(0), vdupq_n_u32(0), vdupq_n_u32(0),],
    pt = [vdupq_n_u32(0), vdupq_n_u32(0)],
    encrypt = speck::neon_encrypt_block_48_72,
    decrypt = speck::neon_decrypt_block_48_72
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_48_96_bench,
    prefix = "48_96",
    key = [
        vdupq_n_u32(0),
        vdupq_n_u32(0),
        vdupq_n_u32(0),
        vdupq_n_u32(0),
    ],
    pt = [vdupq_n_u32(0), vdupq_n_u32(0)],
    encrypt = speck::neon_encrypt_block_48_96,
    decrypt = speck::neon_decrypt_block_48_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_96_bench,
    prefix = "64_96",
    key = [vdupq_n_u32(0), vdupq_n_u32(0), vdupq_n_u32(0),],
    pt = [vdupq_n_u32(0), vdupq_n_u32(0)],
    encrypt = speck::neon_encrypt_block_64_96,
    decrypt = speck::neon_decrypt_block_64_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_64_128_bench,
    prefix = "64_128",
    key = [
        vdupq_n_u32(0),
        vdupq_n_u32(0),
        vdupq_n_u32(0),
        vdupq_n_u32(0),
    ],
    pt = [vdupq_n_u32(0), vdupq_n_u32(0)],
    encrypt = speck::neon_encrypt_block_64_128,
    decrypt = speck::neon_decrypt_block_64_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_96_bench,
    prefix = "96_96",
    key = [vdupq_n_u64(0), vdupq_n_u64(0)],
    pt = [vdupq_n_u64(0), vdupq_n_u64(0)],
    encrypt = speck::neon_encrypt_block_96_96,
    decrypt = speck::neon_decrypt_block_96_96
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_96_144_bench,
    prefix = "96_144",
    key = [vdupq_n_u64(0), vdupq_n_u64(0), vdupq_n_u64(0),],
    pt = [vdupq_n_u64(0), vdupq_n_u64(0)],
    encrypt = speck::neon_encrypt_block_96_144,
    decrypt = speck::neon_decrypt_block_96_144
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_128_bench,
    prefix = "128_128",
    key = [vdupq_n_u64(0), vdupq_n_u64(0),],
    pt = [vdupq_n_u64(0), vdupq_n_u64(0),],
    encrypt = speck::neon_encrypt_block_128_128,
    decrypt = speck::neon_decrypt_block_128_128
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_192_bench,
    prefix = "128_192",
    key = [vdupq_n_u64(0), vdupq_n_u64(0), vdupq_n_u64(0),],
    pt = [vdupq_n_u64(0), vdupq_n_u64(0),],
    encrypt = speck::neon_encrypt_block_128_192,
    decrypt = speck::neon_decrypt_block_128_192
);

define_cipher_bench!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    neon_128_256_bench,
    prefix = "128_256",
    key = [
        vdupq_n_u64(0),
        vdupq_n_u64(0),
        vdupq_n_u64(0),
        vdupq_n_u64(0),
    ],
    pt = [vdupq_n_u64(0), vdupq_n_u64(0),],
    encrypt = speck::neon_encrypt_block_128_256,
    decrypt = speck::neon_decrypt_block_128_256
);

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("neon");
    unsafe {
        g.throughput(Throughput::Elements(8));
        neon_32_64_bench(&mut g);

        g.throughput(Throughput::Elements(4));
        neon_48_72_bench(&mut g);
        neon_48_96_bench(&mut g);
        neon_64_96_bench(&mut g);
        neon_64_128_bench(&mut g);

        g.throughput(Throughput::Elements(2));
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
