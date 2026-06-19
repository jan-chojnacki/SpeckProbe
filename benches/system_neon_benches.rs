use crate::bench_common::criterion_system_config;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::bench_common::run_system_backend;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use speck_probe::search::executor::BackendHint::Neon;

#[path = "bench_common.rs"]
mod bench_common;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn benchmark(c: &mut Criterion) {
    run_system_backend(c, "system/neon", Neon);
}

#[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_system_config();
    targets = benchmark
}

criterion_main!(benches);
