use super::fs::{ensure_parent_dir, file_has_data, open_file};
use crate::probe::record::BenchmarkRecord;
use crate::store::StoreError;
use std::path::Path;

/// Appends `records` to the CSV at `path`, writing a header row only when the file is empty.
pub fn save_benchmark_records(records: &[BenchmarkRecord], path: &Path) -> Result<(), StoreError> {
    ensure_parent_dir(path)?;
    let write_headers = !file_has_data(path)?;
    let file = open_file(path, false)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(file);
    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
