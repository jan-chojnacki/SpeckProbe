use crate::bench_common::{create_targets, criterion_compare_config, run_system_benchmarks};
use criterion::{Criterion, SamplingMode, Throughput, criterion_group, criterion_main};
#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
use speck_probe::search::executor::BackendHint::Avx512;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use speck_probe::search::executor::BackendHint::Neon;
use speck_probe::search::executor::CipherMode;
use speck_probe::search::executor::CipherMode::Ecb;
use speck_probe::speck::SpeckVersion;
use speck_probe::speck::SpeckVersion::{Speck32_64, Speck48_72, Speck64_96, Speck128_128};

#[path = "bench_common.rs"]
mod bench_common;

const COMPARE_BITS: usize = 32;
const COMPARE_SPECK_VERSIONS: [SpeckVersion; 4] =
    [Speck32_64, Speck48_72, Speck64_96, Speck128_128];
const COMPARE_CIPHER_MODES: [CipherMode; 1] = [Ecb];
const COMPARE_SUFFIX_BYTES: [usize; 1] = [2];

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn benchmark_neon(c: &mut Criterion) {
    let mut g = c.benchmark_group("compare/neon");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << COMPARE_BITS));

    let targets = create_targets(
        Neon,
        &COMPARE_CIPHER_MODES,
        &COMPARE_SUFFIX_BYTES,
        &COMPARE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, COMPARE_BITS);
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
fn benchmark_avx512(c: &mut Criterion) {
    let mut g = c.benchmark_group("compare/avx512");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << COMPARE_BITS));

    let targets = create_targets(
        Avx512,
        &COMPARE_CIPHER_MODES,
        &COMPARE_SUFFIX_BYTES,
        &COMPARE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, COMPARE_BITS);
}

fn benchmark(c: &mut Criterion) {
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    benchmark_avx512(c);

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    benchmark_neon(c);
}

criterion_group! {
    name = benches;
    config = criterion_compare_config();
    targets = benchmark
}

criterion_main!(benches);
