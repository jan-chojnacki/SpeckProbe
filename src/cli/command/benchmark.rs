use crate::benchmark::{BenchmarkConfig, BenchmarkRecord};
use crate::cli::display::{display_banner, display_benchmark_info};
use crate::cli::progress::ui::build_benchmark_progress_bar;
use crate::probe::ProbeError;
use crate::search::executor::CipherMode::Cbc;
use crate::search::executor::Runtime;
use crate::search::executor::{
    BackendHint, CipherConfig, CipherFunction, CipherMode, RuntimeConfig, SearchSpace,
};
use crate::speck::SpeckVersion;
use crate::store::{load_config, save_benchmark_records};
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Loads a benchmark config, runs all passes, and writes results to a CSV file.
pub fn execute(config_path: PathBuf, output_path: PathBuf) -> Result<(), ProbeError> {
    let config = load_config::<BenchmarkConfig>(&config_path)?;
    let targets = targets_from_config(&config);

    let total_passes: usize = targets
        .iter()
        .map(|t| config.bits.saturating_sub(t.suffix_bytes * 8))
        .sum();

    display_banner();
    display_benchmark_info(&config, &output_path, total_passes);

    let pb = build_benchmark_progress_bar(total_passes as u64);
    let architecture = std::env::consts::ARCH.to_string();
    let mut records: Vec<BenchmarkRecord> = Vec::new();

    for t in targets {
        for bits in (t.suffix_bytes * 8 + 1)..=config.bits {
            let duration = run_pass(&t, bits)?;
            records.push(BenchmarkRecord {
                bits_measured: bits,
                benchmark: "system",
                backend: t.backend_hint,
                architecture: architecture.clone(),
                function: t.cipher_function,
                version: t.speck_version,
                suffix: t.suffix_bytes,
                throughput_num: 1u64 << bits,
                unit: "ns",
                duration_ns: duration.as_nanos(),
            });
            pb.inc(1);
        }
    }

    pb.finish();
    save_benchmark_records(&records, &output_path)?;
    Ok(())
}

/// A single benchmark target derived from one combination of benchmark config fields.
struct BenchmarkTarget {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub backend_hint: BackendHint,
    pub suffix_bytes: usize,
}

/// Expands a `BenchmarkConfig` into every combination of its fields as individual targets.
fn targets_from_config(config: &BenchmarkConfig) -> Vec<BenchmarkTarget> {
    let mut targets = Vec::new();
    for &version in &config.speck_versions {
        for &function in &config.cipher_functions {
            for &mode in &config.cipher_modes {
                for &backend in &config.backend_hints {
                    for &suffix_bytes in &config.suffix_bytes_values {
                        targets.push(BenchmarkTarget {
                            cipher_mode: mode,
                            speck_version: version,
                            cipher_function: function,
                            backend_hint: backend,
                            suffix_bytes,
                        });
                    }
                }
            }
        }
    }
    targets
}

/// Builds the executor configuration structs for a single benchmark target and key-space bit width.
fn into_runtime_configs(
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

/// Runs one timed benchmark pass for `target` at the given key-space `bits` width.
fn run_pass(target: &BenchmarkTarget, bits: usize) -> Result<Duration, ProbeError> {
    let (cipher_config, runtime_config, search_space) = into_runtime_configs(target, bits);
    let mut runtime = Runtime::new(
        black_box(cipher_config),
        black_box(runtime_config),
        black_box(search_space),
    );
    let t = Instant::now();
    let results = runtime.run()?;
    black_box(results);
    Ok(t.elapsed())
}
