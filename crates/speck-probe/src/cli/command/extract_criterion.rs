use crate::probe::ProbeError;
use crate::probe::ops::extract_criterion::execute as run_extract;
use std::path::PathBuf;

/// Collects Criterion CSV results and merges them into a single output CSV.
pub fn execute(
    criterion_path: PathBuf,
    output_path: PathBuf,
    clear_output: bool,
) -> Result<(), ProbeError> {
    run_extract(&criterion_path, &output_path, clear_output)
}
