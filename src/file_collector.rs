use crate::dir_entry_status::{check_dir_entry_status, DirEntryStatus};
use ignore::WalkBuilder;
use std::path::PathBuf;

pub(crate) struct FileCollector {
    root_path: PathBuf,
}

impl FileCollector {
    // Creates a new instance of `FileCollector`.
    pub fn new<P: Into<PathBuf>>(root_path: P) -> Self {
        FileCollector {
            root_path: root_path.into(),
        }
    }

    // Collects and returns the filenames starting from the root path
    pub fn collect(&self, include_hidden: bool, ignore_gitignore: bool) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut hidden_dirs = Vec::new();
        for result in WalkBuilder::new(self.root_path.clone())
            .hidden(false)
            .ignore(!ignore_gitignore)
            .build()
        {
            match result {
                Ok(dir_entry) => {
                    let path = dir_entry.path();
                    let status = check_dir_entry_status(path);
                    match status {
                        DirEntryStatus::HiddenDirectory => {
                            if !include_hidden {
                                hidden_dirs.push(path.to_path_buf())
                            }
                        }
                        DirEntryStatus::Directory | DirEntryStatus::File => {
                            if include_hidden
                                || !hidden_dirs
                                    .iter()
                                    .any(|hidden_dir| path.starts_with(hidden_dir))
                            {
                                files.push(path.to_path_buf());
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    //TODO Write to log file!
                    eprintln!("Error while collecting files: {:?}", e);
                }
            }
        }
        files
    }
}
