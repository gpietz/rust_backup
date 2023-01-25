mod backup_creator;
mod backup_files;
mod cli_args;
mod string_utils;
mod toml_reader;

mod prelude {
    pub use crate::backup_creator::*;
    pub use crate::backup_files::*;
    pub use crate::cli_args::*;
    pub use crate::string_utils::*;
    pub use crate::toml_reader::*;
}

use prelude::*;
use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    println!("**** RUST BACKUP v{VERSION} ****");

    let mut cli_args = CliArgs::create();
    if !cli_args.validate() {
        return Err("Missing something!".into());
    }
    cli_args.load_target_path();

    let mut backup_files = BackupFiles::create();
    backup_files.collect_backup_files(&cli_args);

    let toml_reader = TomlReader::create(&cli_args);
    BackupCreator::create_backup(&cli_args, &backup_files, &toml_reader);
    Ok(())
}
