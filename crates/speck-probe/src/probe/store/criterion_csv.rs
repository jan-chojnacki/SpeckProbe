use super::fs::{ensure_parent_dir, file_has_data, open_file};
use crate::probe::record::criterion::{CriterionRecord, RawRecord};
use crate::probe::store::StoreError;
use std::path::{Path, PathBuf};

fn read_from_file(path: &Path, architecture: &str) -> Result<Vec<CriterionRecord>, StoreError> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for (row_index, result) in reader.deserialize::<RawRecord>().enumerate() {
        records.push(CriterionRecord::from_raw(row_index, result?, architecture));
    }
    Ok(records)
}

/// Reads and merges Criterion CSV files from `paths`, tagging each record with `architecture`.
pub fn read_all(paths: &[PathBuf], architecture: &str) -> Result<Vec<CriterionRecord>, StoreError> {
    let mut all = Vec::new();
    for path in paths {
        all.extend(read_from_file(path, architecture)?);
    }
    Ok(all)
}

/// Writes `records` to the CSV at `path`. When `clear` is true the file is truncated first.
pub fn save(records: &[CriterionRecord], path: &Path, clear: bool) -> Result<(), StoreError> {
    ensure_parent_dir(path)?;
    let write_headers = clear || !file_has_data(path)?;
    let file = open_file(path, clear)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(file);
    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
