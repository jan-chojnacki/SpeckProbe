use crate::cli::presentation::display::{display_banner, display_benchmark_info};
use crate::cli::presentation::progress::ui::build_benchmark_progress_bar;
use crate::probe::ProbeError;
use crate::probe::config::BenchmarkConfig;
use crate::probe::ops::benchmark::{run_pass, targets_from_config};
use crate::probe::record::BenchmarkRecord;
use crate::store::{load_config, save_benchmark_records};
use std::path::PathBuf;

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
