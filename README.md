# RustBackup

## Project Source Code Storage and Description

In addition to hosting my projects on GitHub, I prefer to maintain local copies of the source code on my disk. This practice, though perhaps considered old-fashioned, offers practical benefits, particularly during the prototyping phase. I prefer not to upload unfinished states of my projects to the web, and having a local backup ensures that I can work seamlessly offline.

Recently, I embarked on a small Rust project that I had been contemplating for some time. As someone relatively new to the Rust environment, this endeavor presented an excellent opportunity for learning and growth.

## Project Description

This project is a command-line interface (CLI) tool designed to streamline the process of packing entire projects into zip files while considering project structure and utilizing .gitignore files effectively. The primary objective is to include only the files necessary for creating executable files in the resulting archive. This tool proves invaluable for simplifying project packaging and distribution tasks.

## Command-line Parameters

#### Usage
```console
rustbackup [FLAGS] [OPTIONS] <project-path>
```

#### Flags

```
-f, --full: Makes a full backup of all files by ignoring any information from .gitignore files.
-h, --help: Prints help information.
-l, --list: Lists only all files that will be included in the backup file.
-q, --quiet: Reduces output during the backup process.
-b, --verbose: Increases verbosity level for more detailed output.
```

#### Options

```
-t, --target <target-path>: Specifies the target path or filename of the generated backup file.
```

#### Arguments

```
<project-path>: Path to the project directory to be backed up.
```

### Disclaimer

**Note: The usage of this tool is entirely at your own risk. The author accepts no liability for any damages or issues arising from its use.**
