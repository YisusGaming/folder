use std::{fs, io, path};

use crate::cli;

#[derive(PartialEq)]
pub enum Mode<'a> {
    NEW,
    DELETE,
    UNKNOWN(&'a str),
}
impl<'a> Mode<'a> {
    /// Resolves the [`Mode`] Folder will run, which will be `Mode::NEW` if
    /// `arg` is "new", `Mode::DELETE` if `arg` is "del", or `Mode::UNKNOWN` if
    /// `arg` is anything else.
    pub fn resolve_mode(arg: &str) -> Mode {
        match arg {
            "new" => Mode::NEW,
            "del" => Mode::DELETE,
            _ => Mode::UNKNOWN(arg),
        }
    }
}

/// Specifies the mode Folder will run and the dirname that will be used.
pub struct FolderConfig<'a> {
    pub mode: Mode<'a>,
    pub dir_name: &'a str,
}

/// Executes Folder using the specified [`FolderConfig`].
pub fn run(config: &FolderConfig) -> io::Result<()> {
    match config.mode {
        Mode::NEW => {
            fs::create_dir_all(config.dir_name)?;
            println!("folder: created directory '{}'.", config.dir_name);

            Ok(())
        }
        Mode::DELETE => {
            if !cli::question(&format!(
                "Are you sure you want to delete '{}' and all of its contents?",
                config.dir_name
            ))? {
                return Ok(());
            }

            delete_directory_contents(path::Path::new(config.dir_name))?;
            fs::remove_dir(config.dir_name)?;
            println!("removed directory '{}'.", config.dir_name);

            Ok(())
        }
        Mode::UNKNOWN(mode) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("folder: unknown mode \"{mode}\" passed in."),
            ));
        }
    }
}

/// Recursively deletes the contents of a directory, while listing progress to stdout. This
/// function will preserve the root directory, leaving it as an empty folder.
pub fn delete_directory_contents(path: &path::Path) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?.path();

        if entry.is_file() {
            fs::remove_file(&entry)?;
            println!("removed file '{}'.", entry.display());
        } else if entry.is_dir() {
            delete_directory_contents(entry.as_path())?;
            fs::remove_dir(&entry)?;
            println!("removed directory '{}'.", entry.display());
        }
    }

    Ok(())
}
