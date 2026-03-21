use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use engine::SearchEngineBackend;
use engine::api::request::{Operation, SearchRangeRequest};
use engine::api::version::SpeckVersion;
use engine::backend::avx::engine::SearchEngineAvx;
use engine::backend::scalar::engine::SearchEngineScalar;
use engine::domain::block::Block;
use std::hint::black_box;
use std::time::Duration;

pub(crate) fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
        .sample_size(100)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
}

fn generate_request(n: u64, operation: Operation) -> SearchRangeRequest {
    SearchRangeRequest {
        speck_version: SpeckVersion::Speck32_64,
        start_key: 0,
        key_count: n,
        prefix: vec![],
        data_bytes: Block::new(&[0, 0, 0, 0], &SpeckVersion::Speck32_64).unwrap(),
        expected_bytes: Block::new(&[0, 0, 0, 0], &SpeckVersion::Speck32_64).unwrap(),
        operation,
    }
}

fn scalar_engine_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("scalar_engine");

    for i in 1..6 {
        let iters = 10u64.pow(i);
        g.throughput(Throughput::Elements(iters));

        g.bench_function(format!("encrypt_{}", iters), |b| {
            b.iter_batched(
                || {
                    let r = generate_request(iters, Operation::Encrypt);
                    r
                },
                |r| {
                    let out = SearchEngineScalar::search_range_encrypt(r).ok();
                    black_box(out);
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(format!("decrypt_{}", iters), |b| {
            b.iter_batched(
                || {
                    let r = generate_request(iters, Operation::Decrypt);
                    r
                },
                |r| {
                    let out = SearchEngineScalar::search_range_decrypt(r).ok();
                    black_box(out);
                },
                BatchSize::SmallInput,
            )
        });
    }
}

fn avx_engine_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("avx_engine");

    for i in 1..6 {
        let iters = 10u64.pow(i);
        g.throughput(Throughput::Elements(iters));

        g.bench_function(format!("encrypt_{}", iters), |b| {
            b.iter_batched(
                || {
                    let r = generate_request(iters, Operation::Encrypt);
                    r
                },
                |r| {
                    let out = SearchEngineAvx::search_range_encrypt(r).ok();
                    black_box(out);
                },
                BatchSize::SmallInput,
            )
        });

        g.bench_function(format!("decrypt_{}", iters), |b| {
            b.iter_batched(
                || {
                    let r = generate_request(iters, Operation::Decrypt);
                    r
                },
                |r| {
                    let out = SearchEngineAvx::search_range_decrypt(r).ok();
                    black_box(out);
                },
                BatchSize::SmallInput,
            )
        });
    }
}

fn benchmark(c: &mut Criterion) {
    scalar_engine_bench(c);
    avx_engine_bench(c);
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
