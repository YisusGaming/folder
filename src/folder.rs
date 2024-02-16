use std::{fs, io};

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
        if arg == "new" {
            return Mode::NEW;
        } else if arg == "del" {
            return Mode::DELETE;
        }

        Mode::UNKNOWN(arg)
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
            println!("folder: created directory {}.", config.dir_name);

            Ok(())
        }
        Mode::DELETE => {
            if !cli::question(&format!(
                "Are you sure you want to delete {} and all of its contents?",
                config.dir_name
            ))? {
                return Ok(());
            }

            fs::remove_dir_all(config.dir_name)?;
            println!("folder: deleted directory {}.", config.dir_name);

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
