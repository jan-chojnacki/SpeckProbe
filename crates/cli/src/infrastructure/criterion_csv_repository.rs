use crate::domain::criterion_record::{CriterionRecord, RawRecord};
use crate::infrastructure::error::ConfigRepositoryError;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

fn read_records_from_file(
    path: &Path,
    architecture: &str,
) -> Result<Vec<CriterionRecord>, ConfigRepositoryError> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for (row_index, result) in reader.deserialize::<RawRecord>().enumerate() {
        records.push(CriterionRecord::from_raw(row_index, result?, architecture));
    }
    Ok(records)
}

pub(crate) fn read_all_records(
    paths: &[PathBuf],
    architecture: &str,
) -> Result<Vec<CriterionRecord>, ConfigRepositoryError> {
    let mut all = Vec::new();
    for path in paths {
        all.extend(read_records_from_file(path, architecture)?);
    }
    Ok(all)
}

fn ensure_parent_dir(path: &Path) -> Result<(), ConfigRepositoryError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn file_has_data(path: &Path) -> Result<bool, ConfigRepositoryError> {
    if !path.exists() {
        return Ok(false);
    }
    Ok(fs::metadata(path)?.len() > 0)
}

fn open_output_file(path: &Path, clear: bool) -> Result<File, ConfigRepositoryError> {
    let mut opts = OpenOptions::new();
    opts.create(true);
    if clear {
        opts.write(true).truncate(true);
    } else {
        opts.append(true);
    }
    Ok(opts.open(path)?)
}

pub(crate) fn save_records(
    records: &[CriterionRecord],
    path: &Path,
    clear_output: bool,
) -> Result<(), ConfigRepositoryError> {
    ensure_parent_dir(path)?;
    let write_headers = clear_output || !file_has_data(path)?;
    let file = open_output_file(path, clear_output)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(file);
    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
