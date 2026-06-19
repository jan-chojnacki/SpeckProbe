use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, Criterion, SamplingMode, Throughput};
use speck_probe::search::executor::CipherMode::{Cbc, Ecb};
use speck_probe::search::executor::{
    BackendHint, CipherConfig, CipherFunction, CipherMode, Runtime, RuntimeConfig, SearchSpace,
};
use speck_probe::speck::SpeckVersion;
use speck_probe::speck::SpeckVersion::{
    Speck32_64, Speck48_72, Speck48_96, Speck64_96, Speck64_128, Speck96_96, Speck96_144,
    Speck128_128, Speck128_192, Speck128_256,
};
use std::hint::black_box;
use std::time::Duration;

#[allow(dead_code)]
pub fn criterion_speck_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(5))
        .sample_size(100)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

#[allow(dead_code)]
pub fn criterion_engine_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(5))
        .sample_size(30)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

#[allow(dead_code)]
pub fn criterion_system_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(5))
        .sample_size(30)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

#[allow(dead_code)]
pub fn criterion_compare_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(120))
        .sample_size(30)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

#[macro_export]
macro_rules! define_speck_bench {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        prefix = $prefix:literal,
        key = $key:expr,
        pt = $pt:expr,
        encrypt = $encrypt:path,
        encrypt_inflight = $encrypt_inflight:path,
        decrypt = $decrypt:path
    ) => {
        $(#[$meta])*
        fn $fn_name(g: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>) {
            let key = $key;
            let pt = $pt;
            let ct = $encrypt(pt, key);

            g.bench_function(criterion::BenchmarkId::new("encrypt", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $encrypt(black_box(pt), black_box(key));
                    black_box(out);
                })
            });

            g.bench_function(criterion::BenchmarkId::new("encrypt_inflight", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $encrypt_inflight(black_box(pt), black_box(key));
                    black_box(out);
                })
            });

            g.bench_function(criterion::BenchmarkId::new("decrypt", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $decrypt(black_box(ct), black_box(key));
                    black_box(out);
                })
            });
        }
    };
}

#[macro_export]
macro_rules! calculate_end {
    ($n:expr) => {{
        match $n {
            0 => 0,
            1..=7 => (1u64 << ($n * 8)) - 1,
            _ => u64::MAX,
        }
    }};
}

#[macro_export]
macro_rules! define_engine_bench {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        bytes = $bytes:expr,
        word = $word:ty,
        prefix = $prefix:literal,
        function = $function:path,
        function_name = $function_name:literal
    ) => {
        $(#[$meta])*
        fn $fn_name(g: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>) {
            g.sampling_mode(SamplingMode::Flat);

            seq_macro::seq!(I in 1..=3{
                let end = $crate::calculate_end!(I);
                let zero: $word = unsafe { std::mem::zeroed() };
                let task = Task::<$word, { $bytes }, { $bytes - I }> {
                    prefix: [0; $bytes - I],
                    start: 0,
                    end,
                    data: [zero; 2],
                    expected: [zero; 2],
                };

                let mut out: Vec<Key<{ $bytes }, { $bytes - I }>> = Vec::new();

                g.throughput(Throughput::Elements(end.saturating_add(1)));

                g.bench_function(criterion::BenchmarkId::new($function_name, format!("{}/{}", $prefix, I)), |b| {
                    b.iter(|| {
                        $function(black_box(task), black_box(&mut out));
                        black_box(&out);
                    })
                });
            });
        }
    };
}

#[allow(dead_code)]
pub(crate) struct BenchmarkTarget {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub backend_hint: BackendHint,
    pub suffix_bytes: usize,
}

#[allow(dead_code)]
pub(crate) fn into_runtime_configs(
    target: &BenchmarkTarget,
    bits: usize,
) -> (CipherConfig, RuntimeConfig, SearchSpace) {
    debug_assert!(bits > 8);
    debug_assert!(bits < 64);

    let speck_version: SpeckVersion = target.speck_version;

    let start_key = 0u64;
    let mut start: Vec<u8> = start_key.to_le_bytes().to_vec();
    let end_key = (1u64 << (bits - target.suffix_bytes * 8)) - 1;
    let mut end: Vec<u8> = end_key.to_le_bytes().to_vec();

    start.truncate(8 - target.suffix_bytes);
    end.truncate(8 - target.suffix_bytes);

    for _ in 8..speck_version.key_size_bytes() {
        start.push(0);
        end.push(0);
    }

    let cipher_config = CipherConfig {
        cipher_mode: target.cipher_mode,
        speck_version,
        cipher_function: target.cipher_function,
    };
    let runtime_config = RuntimeConfig {
        suffix_bytes_size: target.suffix_bytes,
        num_threads: num_cpus::get(),
        backend_hint: target.backend_hint,
    };
    let search_space = SearchSpace {
        start,
        end,
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
        iv: {
            if target.cipher_mode == Cbc {
                Some([1, 2])
            } else {
                None
            }
        },
    };

    (cipher_config, runtime_config, search_space)
}

#[allow(dead_code)]
pub(crate) fn create_targets(
    backend_hint: BackendHint,
    cipher_modes: &[CipherMode],
    suffix_bytes: &[usize],
    speck_versions: &[SpeckVersion],
) -> Vec<BenchmarkTarget> {
    let mut targets = Vec::new();

    for version in speck_versions {
        for mode in cipher_modes {
            for suffix in suffix_bytes {
                targets.push(BenchmarkTarget {
                    cipher_mode: *mode,
                    speck_version: *version,
                    cipher_function: CipherFunction::EncryptInflight,
                    backend_hint,
                    suffix_bytes: *suffix,
                });
            }
        }
    }

    targets
}

#[allow(dead_code)]
pub(crate) const SYSTEM_BITS: usize = 26;
#[allow(dead_code)]
pub(crate) const SYSTEM_SPECK_VERSIONS: [SpeckVersion; 10] = [
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
#[allow(dead_code)]
pub(crate) const SYSTEM_CIPHER_MODES: [CipherMode; 2] = [Ecb, Cbc];
#[allow(dead_code)]
pub(crate) const SYSTEM_SUFFIX_BYTES: [usize; 2] = [1, 2];

#[allow(dead_code)]
pub(crate) fn run_system_backend(c: &mut Criterion, group: &str, backend_hint: BackendHint) {
    let mut g = c.benchmark_group(group);

    g.sampling_mode(SamplingMode::Flat);
    g.throughput(Throughput::Elements(1 << SYSTEM_BITS));

    let targets = create_targets(
        backend_hint,
        &SYSTEM_CIPHER_MODES,
        &SYSTEM_SUFFIX_BYTES,
        &SYSTEM_SPECK_VERSIONS,
    );

    run_system_benchmarks(targets, g, SYSTEM_BITS);
}

#[allow(dead_code)]
pub(crate) fn run_system_benchmarks(
    benchmark_targets: Vec<BenchmarkTarget>,
    mut benchmark_group: BenchmarkGroup<WallTime>,
    bits: usize,
) {
    for t in benchmark_targets {
        let (cipher_config, runtime_config, search_space) = into_runtime_configs(&t, bits);
        let mode = t.cipher_mode.to_string().to_lowercase();
        let version = t.speck_version.to_string();
        let version = version
            .strip_prefix("Speck")
            .unwrap_or(&version)
            .to_string();
        let suffix = t.suffix_bytes.to_string();

        benchmark_group.bench_function(
            criterion::BenchmarkId::new(mode, format!("{}/{}", version, suffix)),
            |b| {
                b.iter_batched(
                    || Runtime::new(cipher_config, runtime_config, search_space.clone()),
                    |mut runtime| {
                        black_box(runtime.run().ok());
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
}
