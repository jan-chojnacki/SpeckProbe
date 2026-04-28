use crate::application::error::ApplicationError;
use crate::domain::benchmark_record::BenchmarkRecord;
use crate::domain::config::benchmark::BenchmarkConfig;
use crate::domain::config::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use crate::infrastructure::benchmark_csv_repository::save_records;
use crate::infrastructure::config_repository::load_config;
use crate::presentation::display::{display_banner, display_benchmark_info};
use crate::presentation::progress::ui::build_benchmark_progress_bar;
use runtime::Runtime;
use runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct BenchmarkTarget {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub backend_hint: BackendHint,
    pub suffix_bytes: usize,
}

pub type BenchmarkTargets = Vec<BenchmarkTarget>;

pub type BenchmarkConfig_ = (CipherConfig, RuntimeConfig, SearchSpace);

pub type BenchmarkResults = (BenchmarkTarget, usize, Duration);

pub fn targets_from_config(config: &BenchmarkConfig) -> BenchmarkTargets {
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

fn create_runtime_config(target: &BenchmarkTarget, bits: usize) -> BenchmarkConfig_ {
    debug_assert!(bits > 8);
    debug_assert!(bits < 64);

    let speck_version: speck::SpeckVersion = target.speck_version.into();

    let start = 0u64;
    let mut start: Vec<u8> = start.to_le_bytes().to_vec();

    let end = (1u64 << (bits - target.suffix_bytes * 8)) - 1;
    let mut end: Vec<u8> = end.to_le_bytes().to_vec();

    start.truncate(8 - target.suffix_bytes);
    end.truncate(8 - target.suffix_bytes);

    for _ in 8..speck_version.key_size_bytes() {
        start.push(0);
        end.push(0);
    }

    let cipher_config = CipherConfig {
        cipher_mode: target.cipher_mode.into(),
        speck_version,
        cipher_function: target.cipher_function.into(),
    };

    let runtime_config = RuntimeConfig {
        suffix_bytes_size: target.suffix_bytes,
        num_threads: num_cpus::get(),
        backend_hint: target.backend_hint.into(),
    };

    let search_space = SearchSpace {
        start,
        end,
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
    };

    (cipher_config, runtime_config, search_space)
}

fn benchmark_pass(target: &BenchmarkTarget, bits: usize) -> Result<Duration, ApplicationError> {
    let config = create_runtime_config(target, bits);
    let mut runtime = Runtime::new(
        black_box(config.0),
        black_box(config.1),
        black_box(config.2),
    );
    let start = Instant::now();
    let results = runtime.run()?;
    black_box(results);
    Ok(start.elapsed())
}

pub fn execute(config_path: PathBuf, output_path: PathBuf) -> Result<(), ApplicationError> {
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
    let mut all_records: Vec<BenchmarkRecord> = Vec::new();

    for t in targets {
        for bits in (t.suffix_bytes * 8 + 1)..=config.bits {
            let duration = benchmark_pass(&t, bits)?;
            let throughput_num = 1u64 << bits;
            all_records.push(BenchmarkRecord {
                bits_measured: bits,
                benchmark: "system",
                backend: t.backend_hint,
                architecture: architecture.clone(),
                function: t.cipher_function,
                version: t.speck_version,
                suffix: t.suffix_bytes,
                throughput_num,
                unit: "ns",
                duration_ns: duration.as_nanos(),
            });

            pb.inc(1);
        }
    }

    pb.finish();
    save_records(&all_records, &output_path)?;
    Ok(())
}
