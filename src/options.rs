extern crate structopt;

use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Default)]
#[structopt(name="RUSTBACKUP", no_version, global_settings = &[AppSettings::DisableVersion])]
pub struct CliArgs {
    pub root_path: String,

    /// Makes full backup of all files by ignoring any information from .gitignore files.
    #[structopt(short = "f", long = "full")]
    pub full_backup: bool,

    /// Target path or filename of the generated backup file.
    #[structopt(short = "t", long = "target", parse(from_os_str))]
    pub target_path: Option<PathBuf>,

    /// Less output during the backup process.
    #[structopt(short = "q", long = "quiet")]
    pub quiet_mode: bool,

    /// Lists only all files that will be included in the backup file.
    #[structopt(short = "l", long = "list")]
    pub list_mode: bool,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl CliArgs {
    pub fn create() -> CliArgs {
        CliArgs::from_args()
    }

    pub fn validate(&self) -> bool {
        if !TomlReader::has_toml(&self.root_path) {
            return false;
        }
        true
    }

    pub fn load_target_path(&mut self) -> bool {
        if !self.root_path.is_empty() {
            let path = Path::new(&self.root_path);
            let path_rustbackup = path.join(".rustbackup");
            if path_rustbackup.exists() {
                let rustbackup_content = fs::read_to_string(path_rustbackup).unwrap();
                for keyword in ["target", "target_dir"] {
                    let target_path = TomlReader::get_toml_entry(&rustbackup_content, keyword)
                        .unwrap_or_default();
                    if !target_path.is_empty() {
                        self.target_path = Some(PathBuf::new().join(path));
                        return true;
                    }
                }
            }
        }
        false
    }
}
