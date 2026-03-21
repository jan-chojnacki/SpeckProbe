use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use engine::api::version::SpeckVersion;
use engine::domain::key_iterator::KeyIterator;
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

fn iterator_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("iterator");

    let version = SpeckVersion::Speck128_256;
    let prefix = [0xAA; 24];

    for i in 1..9 {
        let iters = 10u64.pow(i);
        g.throughput(Throughput::Elements(iters));

        g.bench_function(format!("{}", iters), |b| {
            b.iter_batched_ref(
                || {
                    let it = KeyIterator::new(0, iters, &prefix, version).unwrap();
                    let key = it.new_key();
                    (it, key)
                },
                |(it, key)| {
                    while it.next_into(key).is_some() {
                        black_box(&key);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }

    g.finish();
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
fn simd_iterator_bench<const T: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group("simd_iterator");

    let version = SpeckVersion::Speck128_256;
    let prefix = [0xAA; 24];

    for i in 1..9 {
        let iters = 10u64.pow(i);
        g.throughput(Throughput::Elements(iters));

        g.bench_function(format!("{}_lanes/{}", T, iters), |b| {
            b.iter_batched_ref(
                || {
                    let it = KeyIterator::new(0, iters, &prefix, version).unwrap();
                    let key = it.new_sse2_key::<T>();
                    (it, key)
                },
                |(it, key)| {
                    while it.simd_next_into(key).is_some() {
                        black_box(&key);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }

    g.finish();
}

fn benchmark(c: &mut Criterion) {
    iterator_bench(c);

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    unsafe {
        simd_iterator_bench::<2>(c);
        simd_iterator_bench::<4>(c);
        simd_iterator_bench::<8>(c);
        simd_iterator_bench::<16>(c);
        simd_iterator_bench::<32>(c);
    }
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
