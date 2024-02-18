mod backup_creator;
mod dir_entry_status;
mod file_collector;
mod options;
mod string_utils;
mod toml_reader;

mod prelude {
    pub use crate::backup_creator::*;
    pub use crate::options::*;
    pub use crate::string_utils::*;
    pub use crate::toml_reader::*;
}

use crate::file_collector::FileCollector;
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

    if cli_args.root_path.is_empty() {
        eprintln!("No root directory defined");
        return Err("No root directory defined!".into());
    }

    let full_backup = cli_args.full_backup;
    let project_path = cli_args.root_path.clone();
    let backup_files = FileCollector::new(project_path).collect(full_backup, full_backup);

    let toml_reader = TomlReader::create(&cli_args);
    BackupCreator::create_backup(&cli_args, &backup_files, &toml_reader);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file_collector::FileCollector;
    use crate::prelude::CliArgs;

    #[test]
    fn test_collect_files() {
        let cli_args = CliArgs {
            root_path: ".\\testdata".to_string(),
            list_mode: true,
            ..CliArgs::default()
        };

        let file_collector = FileCollector::new(cli_args.root_path);
        let files = file_collector.collect(false, false);

        println!("ok");
        for f in files {
            println!("{}", f.display());
        }
    }
}
