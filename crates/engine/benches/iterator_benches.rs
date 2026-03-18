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

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("iterator");

    let version = SpeckVersion::Speck128_256;
    let prefix = [0xAA; 24];

    for i in 1..9 {
        let iters = 10u64.pow(i);
        g.throughput(Throughput::Elements(iters));

        g.bench_function(format!("{}", iters), |b| {
            b.iter_batched_ref(
                || {
                    let it = KeyIterator::new(0, iters, &prefix, &version).unwrap();
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

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
