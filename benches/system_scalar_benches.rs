use crate::bench_common::{criterion_system_config, run_system_backend};
use criterion::{Criterion, criterion_group, criterion_main};
use speck_probe::search::executor::BackendHint::Scalar;

#[path = "bench_common.rs"]
mod bench_common;

fn benchmark(c: &mut Criterion) {
    run_system_backend(c, "system/scalar", Scalar);
}

criterion_group! {
    name = benches;
    config = criterion_system_config();
    targets = benchmark
}

criterion_main!(benches);
