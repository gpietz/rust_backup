use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TomlReader {
    pub name: String,
    pub version: String,
}

impl TomlReader {
    pub fn create(cli_args: &CliArgs) -> Self {
        let toml_path = TomlReader::get_toml_path(&cli_args.root_path.to_string());
        let toml_text = fs::read_to_string(toml_path).unwrap();
        let toml_name = TomlReader::get_toml_entry(&toml_text, "name");
        let toml_version = TomlReader::get_toml_entry(&toml_text, "version");
        let name = toml_name.unwrap_or("".to_string());
        let version = toml_version.unwrap_or("".to_string());
        Self { name, version }
    }

    pub fn has_toml(project_path: &str) -> bool {
        let toml_path = TomlReader::get_toml_path(project_path);
        if !toml_path.exists() {
            eprintln!("Can't find cargo.toml in project-path: {}", project_path);
            return false;
        }
        true
    }

    fn get_toml_path(project_path: &str) -> PathBuf {
        let path = Path::new(&project_path);
        path.join("cargo.toml")
    }

    pub fn get_toml_entry(toml_content: &str, keyword: &str) -> Option<String> {
        for line in toml_content.split('\n') {
            if line.starts_with(keyword) {
                let start = line.find('"').unwrap() + 1;
                let end = line.rfind('"').unwrap();
                let value = &line[start..end];
                return Some(value.to_string());
            }
        }
        None
    }
}
