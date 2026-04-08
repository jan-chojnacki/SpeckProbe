use crate::record::{RawRecord, Record};
use std::error::Error;
use std::path::{Path, PathBuf};

fn read_records_from_file(file_path: &Path) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for (row_index, result) in reader.deserialize::<RawRecord>().enumerate() {
        let raw_record = result?;
        records.push((row_index, raw_record).into());
    }

    Ok(records)
}

pub(crate) fn read_all_records(paths: &[PathBuf]) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut all_records = Vec::new();

    for path in paths {
        all_records.extend(read_records_from_file(path)?);
    }

    Ok(all_records)
}

pub(crate) fn save_to_csv(records: &[Record], path: &Path) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
