extern crate structopt;

use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
//#[structopt(name="", no_version, global_settings = &[AppSettings::DisableVersion])]
pub struct CliArgs {
    pub project_path: String,

    /// Makes full backup of all files by ignoring any information from .gitignore files.
    #[structopt(short = "f", long = "full")]
    pub full_backup: bool,

    /// Target path or filename of the generated backup file.
    #[structopt(short = "t", long = "target", parse(from_os_str))]
    pub target_path: Option<std::path::PathBuf>,

    /// Less output during the backup process.
    #[structopt(short = "q", long = "quiet")]
    pub quiet_mode: bool,

    /// Lists only all files that will be included in the backup file.
    #[structopt(short = "l", long = "list")]
    pub list_mode: bool,
}

impl CliArgs {
    pub fn create() -> CliArgs {
        let cli_args = CliArgs::from_args();
        Self {
            project_path: cli_args.project_path,
            full_backup: cli_args.full_backup,
            target_path: cli_args.target_path,
            quiet_mode: cli_args.quiet_mode,
            list_mode: cli_args.list_mode,
        }
    }

    pub fn validate(self: &Self) -> bool {
        if !TomlReader::has_toml(&self.project_path) {
            return false;
        }
        true
    }

    pub fn load_target_path(self: &mut Self) -> bool {
        if !self.project_path.is_empty() {
            let path = Path::new(&self.project_path);
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
