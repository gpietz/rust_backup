#[cfg(not(target_os = "windows"))]
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;
use std::{fmt, fs};

/// Represents the possible status of a file.
#[derive(Debug, PartialEq, Eq)]
pub enum DirEntryStatus {
    /// The path doesn't exist.
    NotFound,
    /// The path is hidden.
    HiddenFile,
    /// The path is a directory.
    HiddenDirectory,
    /// The path is a regular file.
    File,
    /// The path is a regular file.
    Directory,
}

impl Display for DirEntryStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirEntryStatus::NotFound => write!(f, "The path doesn't exist."),
            DirEntryStatus::HiddenFile => write!(f, "The path is a hidden file."),
            DirEntryStatus::HiddenDirectory => write!(f, "The path is a hidden directory."),
            DirEntryStatus::File => write!(f, "The path is a regular file."),
            DirEntryStatus::Directory => write!(f, "The path is a directory."),
        }
    }
}

/// Check the status of a dir entry based on its attributes and metadata.
///
/// This function takes a path as input and returns a `DirEntryStatus` enumeration
/// indicating whether the given path is not found, hidden, a directory, or a regular file.
///
/// # Arguments
///
/// * `path` - A path containing the name of the path to check.
///
/// # Returns
///
/// * `DirEntryStatus` - An enumeration representing the status of the path.
pub fn check_dir_entry_status<P: Into<PathBuf>>(path: P) -> DirEntryStatus {
    let path = path.into();
    let metadata = match fs::metadata(&path) {
        Ok(meta) => meta,
        Err(_) => return DirEntryStatus::NotFound,
    };

    let is_hidden = {
        #[cfg(target_os = "windows")]
        {
            metadata.file_attributes() & 0x2 > 0
        }
        #[cfg(not(target_os = "windows"))]
        {
            path.file_name()
                .and_then(OsStr::to_str)
                .map_or(false, |name| name.starts_with('.'))
        }
    };

    match (is_hidden, metadata.is_dir()) {
        (true, false) => DirEntryStatus::HiddenFile,
        (true, true) => DirEntryStatus::HiddenDirectory,
        (false, false) => DirEntryStatus::File,
        (false, true) => DirEntryStatus::Directory,
    }
}
