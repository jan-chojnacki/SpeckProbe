use crate::error::ProbeError;
use crate::store;
use std::path::{Path, PathBuf};

/// Collects Criterion CSV results and merges them into a single output CSV.
pub fn handle_extract(
    criterion_path: PathBuf,
    output_path: PathBuf,
    clear_output: bool,
) -> Result<(), ProbeError> {
    execute(&criterion_path, &output_path, clear_output)
}

fn execute(
    criterion_path: &Path,
    output_path: &Path,
    clear_output: bool,
) -> Result<(), ProbeError> {
    let architecture = std::env::consts::ARCH.to_string();
    let files = store::collect_criterion_files(criterion_path);
    let records = store::read_criterion_records(&files, &architecture)?;
    store::save_criterion_records(&records, output_path, clear_output)?;
    Ok(())
}
