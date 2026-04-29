use crate::probe::error::ProbeError;
use crate::probe::store;
use std::path::Path;

/// Collects Criterion CSV files under `criterion_path` and writes them to `output_path`.
///
/// When `clear_output` is true the destination file is truncated before writing.
pub fn execute(
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
