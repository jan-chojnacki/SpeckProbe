use crate::domain::benchmark_record::BenchmarkRecord;
use crate::infrastructure::error::ConfigRepositoryError;
use std::fs::{self, File, OpenOptions};
use std::path::Path;

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

fn open_output_file(path: &Path) -> Result<File, ConfigRepositoryError> {
    Ok(OpenOptions::new().create(true).append(true).open(path)?)
}

pub(crate) fn save_records(
    records: &[BenchmarkRecord],
    path: &Path,
) -> Result<(), ConfigRepositoryError> {
    ensure_parent_dir(path)?;
    let write_headers = !file_has_data(path)?;
    let output_file = open_output_file(path)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(output_file);

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
