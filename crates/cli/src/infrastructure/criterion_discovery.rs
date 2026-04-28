use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn is_raw_result_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    let in_new_dir = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        == Some("new");
    let is_raw_csv = path.extension().and_then(|e| e.to_str()) == Some("csv")
        && path.file_stem().and_then(|s| s.to_str()) == Some("raw");
    in_new_dir && is_raw_csv
}

pub(crate) fn collect_result_files(criterion_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(criterion_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_raw_result_file)
        .map(|e| e.path().to_path_buf())
        .collect()
}
