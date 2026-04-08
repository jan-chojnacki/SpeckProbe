use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_raw_result_file(entry: &DirEntry) -> bool {
    let path = entry.path();

    let is_in_new_dir = path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        == Some("new");

    let has_csv_extension = path.extension().and_then(|ext| ext.to_str()) == Some("csv");
    let has_raw_stem = path.file_stem().and_then(|stem| stem.to_str()) == Some("raw");

    is_in_new_dir && has_csv_extension && has_raw_stem
}

pub(crate) fn collect_result_files(criterion_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(criterion_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_raw_result_file)
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
