use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use engine::SearchEngineBackend;
use engine::api::request::Operation::{Decrypt, Encrypt};
use engine::api::request::{Operation, SearchRangeRequest};
use engine::api::version::SpeckVersion;
use engine::api::version::SpeckVersion::{
    Speck32_64, Speck48_72, Speck48_96, Speck64_96, Speck64_128, Speck96_96, Speck96_144,
    Speck128_128, Speck128_192, Speck128_256,
};
use engine::backend::avx2::engine::SearchEngineAVX2;
use engine::backend::scalar::engine::SearchEngineScalar;
use engine::backend::sse2::engine::SearchEngineSSE2;
use engine::domain::block::Block;
use std::hint::black_box;
use std::sync::LazyLock;
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

fn generate_request<const B: usize, const P: usize>(
    n: u64,
    speck_version: SpeckVersion,
    operation: Operation,
) -> SearchRangeRequest {
    SearchRangeRequest {
        speck_version,
        start_key: 0,
        key_count: n,
        prefix: vec![0u8; P],
        data_bytes: Block::new(&[0u8; B], &speck_version).unwrap(),
        expected_bytes: Block::new(&[0u8; B], &speck_version).unwrap(),
        operation,
    }
}

const ITERATIONS: u64 = 100_000;
static REQUESTS: LazyLock<[SearchRangeRequest; 20]> = LazyLock::new(|| {
    [
        generate_request::<4, 0>(ITERATIONS, Speck32_64, Decrypt),
        generate_request::<4, 0>(ITERATIONS, Speck32_64, Encrypt),
        generate_request::<6, 1>(ITERATIONS, Speck48_72, Decrypt),
        generate_request::<6, 1>(ITERATIONS, Speck48_72, Encrypt),
        generate_request::<6, 4>(ITERATIONS, Speck48_96, Decrypt),
        generate_request::<6, 4>(ITERATIONS, Speck48_96, Encrypt),
        generate_request::<8, 4>(ITERATIONS, Speck64_96, Decrypt),
        generate_request::<8, 4>(ITERATIONS, Speck64_96, Encrypt),
        generate_request::<8, 8>(ITERATIONS, Speck64_128, Decrypt),
        generate_request::<8, 8>(ITERATIONS, Speck64_128, Encrypt),
        generate_request::<12, 4>(ITERATIONS, Speck96_96, Decrypt),
        generate_request::<12, 4>(ITERATIONS, Speck96_96, Encrypt),
        generate_request::<12, 10>(ITERATIONS, Speck96_144, Decrypt),
        generate_request::<12, 10>(ITERATIONS, Speck96_144, Encrypt),
        generate_request::<16, 8>(ITERATIONS, Speck128_128, Decrypt),
        generate_request::<16, 8>(ITERATIONS, Speck128_128, Encrypt),
        generate_request::<16, 16>(ITERATIONS, Speck128_192, Decrypt),
        generate_request::<16, 16>(ITERATIONS, Speck128_192, Encrypt),
        generate_request::<16, 24>(ITERATIONS, Speck128_256, Decrypt),
        generate_request::<16, 24>(ITERATIONS, Speck128_256, Encrypt),
    ]
});

fn scalar_engine_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("scalar_engine");
    g.throughput(Throughput::Elements(ITERATIONS));

    for r in REQUESTS.iter() {
        g.bench_function(format!("{}/{}", r.speck_version, r.operation), |b| {
            b.iter(|| {
                let out = SearchEngineScalar::handle_request(black_box(r.clone())).unwrap();
                black_box(out);
            })
        });
    }
}

fn sse2_engine_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("sse2_engine");
    g.throughput(Throughput::Elements(ITERATIONS));

    for r in REQUESTS.iter() {
        g.bench_function(format!("{}/{}", r.speck_version, r.operation), |b| {
            b.iter(|| {
                let out = SearchEngineSSE2::handle_request(black_box(r.clone())).unwrap();
                black_box(out);
            })
        });
    }
}

fn avx2_engine_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("avx2_engine");
    g.throughput(Throughput::Elements(ITERATIONS));

    for r in REQUESTS.iter() {
        g.bench_function(format!("{}/{}", r.speck_version, r.operation), |b| {
            b.iter(|| {
                let out = SearchEngineAVX2::handle_request(black_box(r.clone())).unwrap();
                black_box(out);
            })
        });
    }
}

fn benchmark(c: &mut Criterion) {
    scalar_engine_bench(c);
    sse2_engine_bench(c);
    avx2_engine_bench(c);
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
