use crate::record::{RawRecord, Record};
use csv::StringRecord;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

fn read_records_from_file(
    file_path: &Path,
    architecture: &str,
) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for (row_index, result) in reader.deserialize::<RawRecord>().enumerate() {
        let raw_record = result?;
        records.push(Record::from_raw(row_index, raw_record, architecture));
    }

    Ok(records)
}

pub(crate) fn read_all_records(
    paths: &[PathBuf],
    architecture: &str,
) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut all_records = Vec::new();

    for path in paths {
        all_records.extend(read_records_from_file(path, architecture)?);
    }

    Ok(all_records)
}

fn ensure_parent_dir(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn file_has_data(path: &Path) -> Result<bool, Box<dyn Error>> {
    if !path.exists() {
        return Ok(false);
    }

    Ok(fs::metadata(path)?.len() > 0)
}

fn open_output_file(path: &Path, clear_output: bool) -> Result<File, Box<dyn Error>> {
    let mut options = OpenOptions::new();
    options.create(true);

    if clear_output {
        options.write(true).truncate(true);
    } else {
        options.append(true);
    }

    Ok(options.open(path)?)
}

pub(crate) fn save_to_csv(
    records: &[Record],
    path: &Path,
    clear_output: bool,
) -> Result<(), Box<dyn Error>> {
    ensure_parent_dir(path)?;
    let output_has_data = file_has_data(path)?;
    let write_headers = clear_output || !output_has_data;
    let output_file = open_output_file(path, clear_output)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(output_file);

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

fn read_headers(path: &Path) -> Result<Option<StringRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();

    if headers.is_empty() {
        return Ok(None);
    }

    Ok(Some(headers))
}

fn read_records(path: &Path) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();

    for record in reader.records() {
        records.push(record?);
    }

    Ok(records)
}

pub(crate) fn merge_csv_files(
    first_input: &Path,
    second_input: &Path,
) -> Result<(), Box<dyn Error>> {
    ensure_parent_dir(first_input)?;
    let first_records = read_records(first_input)?;
    let second_records = read_records(second_input)?;
    let output_file = open_output_file(first_input, true)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(output_file);

    let headers = read_headers(first_input)?.or(read_headers(second_input)?);
    if let Some(headers) = headers {
        writer.write_record(&headers)?;
    }

    for record in first_records {
        writer.write_record(&record)?;
    }
    for record in second_records {
        writer.write_record(&record)?;
    }
    writer.flush()?;
    Ok(())
}
