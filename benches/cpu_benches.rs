use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use speck::{
    decrypt_block, decrypt_block_avx, decrypt_block_avx2, encrypt_block_avx, encrypt_block_avx2,
};
use speck::{decrypt_block_avx512, encrypt_block, encrypt_block_avx512};
use std::arch::x86_64::{_mm256_set1_epi32, _mm512_set1_epi32, _mm_set1_epi32};
use std::hint::black_box;
use std::time::Duration;

fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .sample_size(100)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
}

fn scalar_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar");
    group.throughput(Throughput::Elements(1));

    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let pt = [0x3b726574, 0x7475432d];
    let ct = [0x8c6fa548, 0x454e028b];

    group.bench_function("encrypt", |b| {
        b.iter(|| {
            let out = encrypt_block(black_box(pt), black_box(key));
            black_box(out);
        })
    });

    group.bench_function("decrypt", |b| {
        b.iter(|| {
            let out = decrypt_block(black_box(ct), black_box(key));
            black_box(out);
        })
    });

    group.finish();
}

fn avx_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("avx");
    group.throughput(Throughput::Elements(4));

    let key = unsafe {
        [
            _mm_set1_epi32(0x1b1a1918),
            _mm_set1_epi32(0x13121110),
            _mm_set1_epi32(0x0b0a0908),
            _mm_set1_epi32(0x03020100),
        ]
    };

    let pt = unsafe { [_mm_set1_epi32(0x3b726574), _mm_set1_epi32(0x7475432d)] };
    let ct = unsafe {
        [
            _mm_set1_epi32(0x8c6fa548u32 as i32),
            _mm_set1_epi32(0x454e028b),
        ]
    };

    group.bench_function("encrypt", |b| {
        b.iter(|| {
            let out = encrypt_block_avx(black_box(pt), black_box(key));
            black_box(out);
        })
    });

    group.bench_function("decrypt", |b| {
        b.iter(|| {
            let out = decrypt_block_avx(black_box(ct), black_box(key));
            black_box(out);
        })
    });

    group.finish();
}

fn avx2_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("avx2");
    group.throughput(Throughput::Elements(8));

    let key = unsafe {
        [
            _mm256_set1_epi32(0x1b1a1918),
            _mm256_set1_epi32(0x13121110),
            _mm256_set1_epi32(0x0b0a0908),
            _mm256_set1_epi32(0x03020100),
        ]
    };

    let pt = unsafe { [_mm256_set1_epi32(0x3b726574), _mm256_set1_epi32(0x7475432d)] };
    let ct = unsafe {
        [
            _mm256_set1_epi32(0x8c6fa548u32 as i32),
            _mm256_set1_epi32(0x454e028b),
        ]
    };

    group.bench_function("encrypt", |b| {
        b.iter(|| {
            let out = encrypt_block_avx2(black_box(pt), black_box(key));
            black_box(out);
        })
    });

    group.bench_function("decrypt", |b| {
        b.iter(|| {
            let out = decrypt_block_avx2(black_box(ct), black_box(key));
            black_box(out);
        })
    });

    group.finish();
}

fn avx512_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("avx512");
    group.throughput(Throughput::Elements(16));

    let key = unsafe {
        [
            _mm512_set1_epi32(0x1b1a1918),
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ]
    };

    let pt = unsafe { [_mm512_set1_epi32(0x3b726574), _mm512_set1_epi32(0x7475432d)] };
    let ct = unsafe {
        [
            _mm512_set1_epi32(0x8c6fa548u32 as i32),
            _mm512_set1_epi32(0x454e028b),
        ]
    };

    group.bench_function("encrypt", |b| {
        b.iter(|| {
            let out = encrypt_block_avx512(black_box(pt), black_box(key));
            black_box(out);
        })
    });

    group.bench_function("decrypt", |b| {
        b.iter(|| {
            let out = decrypt_block_avx512(black_box(ct), black_box(key));
            black_box(out);
        })
    });

    group.finish();
}

fn benchmark(c: &mut Criterion) {
    scalar_bench(c);
    avx_bench(c);
    avx2_bench(c);
    avx512_bench(c);
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
