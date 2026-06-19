use crate::bench_common::criterion_system_config;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
use crate::bench_common::run_system_backend;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
use speck_probe::search::executor::BackendHint::Avx512;

#[path = "bench_common.rs"]
mod bench_common;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
fn benchmark(c: &mut Criterion) {
    run_system_backend(c, "system/avx512", Avx512);
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "avx512bw")))]
fn benchmark(_: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = criterion_system_config();
    targets = benchmark
}

criterion_main!(benches);
