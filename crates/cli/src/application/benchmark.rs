use crate::application::error::ApplicationError;
use crate::domain::config::benchmark::BenchmarkConfig;
use crate::domain::config::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use crate::infrastructure::benchmark_config_repository::load_benchmark_config;
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

pub fn execute(config_path: PathBuf) -> Result<(), ApplicationError> {
    let config = load_benchmark_config(&config_path)?;
    let targets = targets_from_config(&config);

    for t in targets {
        let mut results: Vec<(usize, Duration)> =
            Vec::with_capacity(((t.suffix_bytes * 8 + 1)..=config.bits).count());

        dbg!(((t.suffix_bytes * 8 + 1)..=config.bits).count());

        for b in (t.suffix_bytes * 8 + 1)..=config.bits {
            let result = benchmark_pass(&t, b)?;
            results.push((b, result));
        }

        dbg!(t.backend_hint);
        dbg!(results.last().unwrap());
    }

    Ok(())
}
