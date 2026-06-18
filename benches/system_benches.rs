use crate::bench_common::{create_targets, criterion_system_config, run_system_benchmarks};
use criterion::{Criterion, SamplingMode, Throughput, criterion_group, criterion_main};
use speck_probe::search::executor::BackendHint::{Avx2, Avx512, Scalar, Sse2};
use speck_probe::search::executor::CipherMode;
use speck_probe::search::executor::CipherMode::{Cbc, Ecb};
use speck_probe::speck::SpeckVersion;
use speck_probe::speck::SpeckVersion::{
    Speck32_64, Speck48_72, Speck48_96, Speck64_96, Speck64_128, Speck96_96, Speck96_144,
    Speck128_128, Speck128_192, Speck128_256,
};

#[path = "bench_common.rs"]
mod bench_common;

const ENGINE_BITS: usize = 26;
const ENGINE_SPECK_VERSIONS: [SpeckVersion; 10] = [
    Speck32_64,
    Speck48_72,
    Speck48_96,
    Speck64_96,
    Speck64_128,
    Speck96_96,
    Speck96_144,
    Speck128_128,
    Speck128_192,
    Speck128_256,
];
const ENGINE_CIPHER_MODES: [CipherMode; 2] = [Ecb, Cbc];
const ENGINE_SUFFIX_BYTES: [usize; 2] = [1, 2];

fn benchmark_scalar(c: &mut Criterion) {
    let mut g = c.benchmark_group("system/scalar");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << ENGINE_BITS));

    let targets = create_targets(
        Scalar,
        &ENGINE_CIPHER_MODES,
        &ENGINE_SUFFIX_BYTES,
        &ENGINE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, ENGINE_BITS);
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn benchmark_sse2(c: &mut Criterion) {
    let mut g = c.benchmark_group("system/neon");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << ENGINE_BITS));

    let targets = create_targets(
        Neon,
        &ENGINE_CIPHER_MODES,
        &ENGINE_SUFFIX_BYTES,
        &ENGINE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, ENGINE_BITS);
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
fn benchmark_sse2(c: &mut Criterion) {
    let mut g = c.benchmark_group("system/sse2");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << ENGINE_BITS));

    let targets = create_targets(
        Sse2,
        &ENGINE_CIPHER_MODES,
        &ENGINE_SUFFIX_BYTES,
        &ENGINE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, ENGINE_BITS);
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn benchmark_avx2(c: &mut Criterion) {
    let mut g = c.benchmark_group("system/avx2");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << ENGINE_BITS));

    let targets = create_targets(
        Avx2,
        &ENGINE_CIPHER_MODES,
        &ENGINE_SUFFIX_BYTES,
        &ENGINE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, ENGINE_BITS);
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
fn benchmark_avx512(c: &mut Criterion) {
    let mut g = c.benchmark_group("system/avx512");

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << ENGINE_BITS));

    let targets = create_targets(
        Avx512,
        &ENGINE_CIPHER_MODES,
        &ENGINE_SUFFIX_BYTES,
        &ENGINE_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, ENGINE_BITS);
}

fn benchmark(c: &mut Criterion) {
    benchmark_scalar(c);

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    benchmark_sse2(c);
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    benchmark_avx2(c);
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    benchmark_avx512(c);

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    benchmark_neon(c);
}

criterion_group! {
    name = benches;
    config = criterion_system_config();
    targets = benchmark
}

criterion_main!(benches);
