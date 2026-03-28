use benchmarks::criterion_config;
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use engine::domain::task_producer::TaskProducer;
use rayon::prelude::*;
use std::hint::black_box;

fn benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("task_producer");

    let start = [0, 0, 0, 0, 0];
    let end = [255, 255, 0, 0, 0];
    let data = [0, 0];
    let expected = [0, 0];

    g.throughput(Throughput::Elements(255 * 255));

    g.bench_function("normal", |b| {
        b.iter_batched(
            || TaskProducer::<u16, 8, 5>::new(start, end, data, expected),
            |producer| {
                for i in producer {
                    black_box(i);
                }
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("rayon", |b| {
        b.iter_batched(
            || TaskProducer::<u16, 8, 5>::new(start, end, data, expected),
            |producer| {
                producer.par_bridge().for_each(|i| {
                    black_box(i);
                })
            },
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = benchmark
}

criterion_main!(benches);
