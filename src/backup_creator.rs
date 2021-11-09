use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use chrono::{Datelike, Timelike, Local};
use zip::write::FileOptions;
use zip::{self, ZipWriter, CompressionMethod};

use crate::prelude::*;

pub struct BackupCreator {}

impl BackupCreator {
    pub fn create_backup(cli_args: &CliArgs, backup_files: &BackupFiles, toml_reader: &TomlReader) {
        // Create filename for backup file
        let mut backup_file_name = format!("{}", toml_reader.name);
        if !toml_reader.version.is_empty() {
            backup_file_name.push('-');
            backup_file_name.push_str(toml_reader.version.as_str());
        }
        backup_file_name.push('-');
        backup_file_name.push_str(BackupCreator::get_formatted_datetime().as_str());
        backup_file_name.push_str(".zip");

        // Get target directory depending on cli arguments
        let target_path = if cli_args.target_path.is_some() {
            cli_args.target_path.clone().unwrap().to_str().unwrap().to_string()
        } else {
            env::current_dir().unwrap().to_str().unwrap().to_string()
        };

        // Combine target directory with backup filename
        let backup_path = Path::new(target_path.as_str());
        let backup_path = backup_path.join(&backup_file_name);
        
        println!("Generating backup file: {}", backup_path.to_str().unwrap());

        let path = Path::new(backup_path.as_path());
        let backup_file = File::create(path).unwrap();

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Bzip2)
            .unix_permissions(0o755);

        let mut zip = ZipWriter::new(backup_file);
        let mut dir_names : Vec<String> = Vec::new();
        let mut buffer = Vec::new();
        for backup_file in backup_files.backup_files.iter() {
            println!("Storing file: {}", backup_file.to_str().unwrap());

            let mut file = File::open(backup_file).unwrap();
            file.read_to_end(&mut buffer).unwrap();

            let dir = backup_file.parent().unwrap().as_os_str().to_str().unwrap().to_string();
            if !dir.is_empty() && dir_names.contains(&dir) {
                dir_names.push(dir.clone());
                zip.add_directory(&dir, options).unwrap();
            }

            // Create container file name
            let mut file_name = backup_file.to_str().unwrap();
            file_name = file_name.slice(cli_args.project_path.as_str().len()..);

            // Write data to zip container
            #[allow(deprecated)]
            zip.start_file(file_name, options).unwrap();
            zip.write_all(&*buffer).unwrap();
            buffer.clear();
        }
        
        zip.finish().unwrap();
    }

    fn get_formatted_datetime() -> String {
        let now = Local::now();
        format!("{}{:02}{:02}-{:02}{:02}{:02}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())
    }
}
