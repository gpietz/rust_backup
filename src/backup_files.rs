use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::str::FromStr;
use system_extensions::metadata::attribute::{Attributes, has_attribute};

use crate::prelude::*;

pub struct BackupFiles {
    pub backup_files: Vec<PathBuf>,
    pub git_ignores: Vec<PathBuf>
} 

impl BackupFiles {
    pub fn create() -> Self {
        Self {
            backup_files : Vec::new(),
            git_ignores : Vec::new()
        }
    }

    pub fn collect_backup_files(&mut self, cli_args: &CliArgs) {
        let project_path = PathBuf::from(cli_args.project_path.to_string());
        self.add_git_ignores(&project_path);
        self.collect_files(&project_path);
    }

    fn collect_files(&mut self, path: &PathBuf) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if self.is_entry_hidden(&entry_path) 
                || self.is_git_ignore(&entry_path) 
                || self.is_rustbackup(&entry_path) {
                continue;
            }
            if entry_path.is_dir() {
                self.add_git_ignores(&entry_path);
                self.collect_files(&entry_path);
            } else {
                self.add_file(&entry_path);
            }
        }
    }
    
    fn add_file(&mut self, path: &PathBuf) {
        if self.ignore_file(&path) {
            return;
        }
        self.backup_files.push(path.clone());
        //println!("Added backup file: {}", path.clone().into_os_string().into_string().unwrap());
    }    

    fn ignore_file(&self, path: &PathBuf) -> bool {
        for ignored_path in self.git_ignores.iter() {
            if path.starts_with(ignored_path) {
                return true;
            }
        }       
        return false; 
    }
    
    fn add_git_ignores(&mut self, path: &PathBuf) {
        let mut file_path = path.clone();
        file_path = file_path.join(".gitignore");

        //println!("Reading .gitignore file: {}", file_path.clone().to_str().unwrap());
        if let Ok(lines) = BackupFiles::read_lines(file_path.as_path()) {
            for line in lines {
                if let Ok(entry) = line {
                    let mut entry = entry.trim().to_string();
                    if !entry.starts_with("#") && entry.len() > 0 {
                        //println!("DEBUG: {} ({})", entry, entry.len());
                        entry = BackupFiles::remove_first(&entry).unwrap().to_string();
                        let mut ignore_path = path.clone();
                        ignore_path.push(PathBuf::from_str(&entry).unwrap());
                        self.git_ignores.push(ignore_path.clone());
                        //println!("Adding gitignore: {}", ignore_path.into_os_string().into_string().unwrap());
                    }
                }
            }   
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn remove_first(s: &str) -> Option<&str> {
        s.chars().next().map(|c| &s[c.len_utf8()..])
    }

    fn is_entry_hidden(&self, path: &PathBuf) -> bool {
        has_attribute(path.as_path(), Attributes::HIDDEN)
    }

    fn is_git_ignore(&self, path: &PathBuf) -> bool {
        path.ends_with(".gitignore")
    }

    fn is_rustbackup(&self, path: &PathBuf) -> bool {
        path.ends_with(".rustbackup")
    }
}
