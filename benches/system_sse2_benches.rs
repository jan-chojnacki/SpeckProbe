use crate::bench_common::criterion_system_config;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use crate::bench_common::run_system_backend;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use speck_probe::search::executor::BackendHint::Sse2;

#[path = "bench_common.rs"]
mod bench_common;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
fn benchmark(c: &mut Criterion) {
    run_system_backend(c, "system/sse2", Sse2);
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_system_config();
    targets = benchmark
}

criterion_main!(benches);
