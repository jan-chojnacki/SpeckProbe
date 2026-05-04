use crate::store::StoreError;
use std::fs::{self, File, OpenOptions};
use std::path::Path;

/// Creates all parent directories of `path` if they do not already exist.
pub(crate) fn ensure_parent_dir(path: &Path) -> Result<(), StoreError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Returns `true` if `path` exists and contains at least one byte.
pub(crate) fn file_has_data(path: &Path) -> Result<bool, StoreError> {
    if !path.exists() {
        return Ok(false);
    }
    Ok(fs::metadata(path)?.len() > 0)
}

/// Opens `path` for writing. Truncates when `clear` is true, otherwise appends.
pub(crate) fn open_file(path: &Path, clear: bool) -> Result<File, StoreError> {
    let mut opts = OpenOptions::new();
    opts.create(true);
    if clear {
        opts.write(true).truncate(true);
    } else {
        opts.append(true);
    }
    Ok(opts.open(path)?)
}
