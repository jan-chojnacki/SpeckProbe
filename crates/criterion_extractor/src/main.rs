use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct RawRecord {
    group: String,
    function: String,
    value: String,
    throughput_num: u64,
    throughput_type: String,
    sample_measured_value: f64,
    unit: String,
    iteration_count: u64,
}

#[derive(Debug, Serialize)]
struct Record {
    benchmark: String,
    backend: String,
    function: String,
    version: String,
    suffix: String,
    throughput_num: u64,
    throughput_type: String,
    sample_measured_value: f64,
    unit: String,
    iteration_count: u64,
}

impl TryFrom<RawRecord> for Record {
    type Error = String;

    fn try_from(value: RawRecord) -> Result<Self, Self::Error> {
        let (benchmark, backend) = match value.group.split_once("/") {
            None => (value.group, "".to_string()),
            Some((a, b)) => (a.to_string(), b.to_string()),
        };

        let (version, variant) = match value.value.split_once("/") {
            None => (value.value, "".to_string()),
            Some((a, b)) => (a.to_string(), b.to_string()),
        };

        Ok(Self {
            benchmark,
            backend,
            function: value.function,
            version,
            suffix: variant,
            throughput_num: value.throughput_num,
            throughput_type: value.throughput_type,
            sample_measured_value: value.sample_measured_value,
            unit: value.unit,
            iteration_count: value.iteration_count,
        })
    }
}

fn save_to_csv(records: &[Record], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let criterion_path = Path::new("./target/criterion/");

    println!("{}", criterion_path.exists());

    let results: Vec<std::path::PathBuf> = WalkDir::new(criterion_path)
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| {
            let path = e.path();
            let new_ok = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .map(|s| s == "new")
                .unwrap_or(false);
            let ext_ok = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "csv")
                .unwrap_or(false);
            let name_ok = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s == "raw")
                .unwrap_or(false);
            ext_ok && name_ok && new_ok
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    let mut all_results: Vec<Record> = Vec::new();

    for f in results {
        let mut rdr = csv::Reader::from_path(f).unwrap();

        for result in rdr.deserialize() {
            let record: RawRecord = result.unwrap();
            all_results.push(record.try_into().unwrap());
        }
    }

    save_to_csv(&all_results, "output.csv")?;
    Ok(())
}
