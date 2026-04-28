use crate::application::error::ApplicationError;
use crate::infrastructure::criterion_csv_repository;
use crate::infrastructure::criterion_discovery::collect_result_files;
use std::path::PathBuf;

pub fn execute(
    criterion_path: PathBuf,
    output_path: PathBuf,
    clear_output: bool,
) -> Result<(), ApplicationError> {
    let architecture = std::env::consts::ARCH.to_string();
    let files = collect_result_files(&criterion_path);
    let records = criterion_csv_repository::read_all_records(&files, &architecture)?;
    criterion_csv_repository::save_records(&records, &output_path, clear_output)?;
    Ok(())
}
