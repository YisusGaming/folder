use std::process;

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
    pub dir_name: String,
}

/// Executes Folder using the specified [`FolderConfig`].
pub fn run(config: &FolderConfig) -> process::ExitCode {
    match config.mode {
        Mode::NEW => todo!(),
        Mode::DELETE => todo!(),
        Mode::UNKNOWN(mode) => {
            eprintln!("folder: unknown mode \"{mode}\" passed in.");
            return process::ExitCode::from(1);
        }
    }
    todo!()
}
