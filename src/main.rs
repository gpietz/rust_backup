mod cli_args;
mod backup_creator;
mod backup_files;
mod toml_reader;
mod string_utils;

mod prelude {
    pub use crate::cli_args::*;
    pub use crate::backup_creator::*;
    pub use crate::backup_files::*;
    pub use crate::toml_reader::*;
    pub use crate::string_utils::*;
}

use prelude::*;

fn main() {
    println!("**** RUST BACKUP v0.1 ****");

    let mut cli_args = CliArgs::create();
    if !cli_args.validate() {
        return;
    }
    cli_args.load_target_path();

    let mut backup_files = BackupFiles::create();
    backup_files.collect_backup_files(&cli_args);

    let toml_reader = TomlReader::create(&cli_args);
    BackupCreator::create_backup(&cli_args, &backup_files, &toml_reader);
}
