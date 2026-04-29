use crate::probe::config::BenchmarkConfig;
use crate::probe::config::enums::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use crate::probe::error::ProbeError;
use crate::runtime::Runtime;
use crate::runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
use std::hint::black_box;
use std::time::{Duration, Instant};

/// A single benchmark target derived from one combination of benchmark config fields.
pub struct BenchmarkTarget {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub backend_hint: BackendHint,
    pub suffix_bytes: usize,
}

/// Expands a `BenchmarkConfig` into every combination of its fields as individual targets.
pub fn targets_from_config(config: &BenchmarkConfig) -> Vec<BenchmarkTarget> {
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

/// Builds the runtime configuration structs for a single benchmark target and key-space bit width.
pub fn into_runtime_configs(
    target: &BenchmarkTarget,
    bits: usize,
) -> (CipherConfig, RuntimeConfig, SearchSpace) {
    debug_assert!(bits > 8);
    debug_assert!(bits < 64);

    let speck_version: speck::SpeckVersion = target.speck_version.into();

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

/// Runs one timed benchmark pass for `target` at the given key-space `bits` width.
pub fn run_pass(target: &BenchmarkTarget, bits: usize) -> Result<Duration, ProbeError> {
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
