use std::process::{self, Command};
use std::str::from_utf8;

#[derive(PartialEq)]
pub enum Mode {
    NEW,
    DELETE,
    UNKNOWN,
}
impl Mode {
    /// Resolves the [`Mode`] Folder will run, which will be `Mode::NEW` if
    /// `arg` is "new", `Mode::DELETE` if `arg` is "del", or `Mode::UNKNOWN` if
    /// `arg` is anything else.
    pub fn resolve_mode(arg: &str) -> Mode {
        if arg == "new" {
            return Mode::NEW;
        } else if arg == "del" {
            return Mode::DELETE;
        }

        Mode::UNKNOWN
    }
}

/// Specifies the mode Folder will run and the dirname that will be used.
pub struct FolderConfig {
    pub mode: Mode,
    pub dir_name: String,
}

/// Takes a buffer and returns an String
/// replacing "mkdir: " and "rm: " for "folder: ".
pub fn format_output(buf: &[u8]) -> String {
    let str = from_utf8(&buf).unwrap_or("");
    let str = str.replace("mkdir: ", "folder: ");
    let str = str.replace("rm: ", "folder: ");

    String::from(str.trim())
}

/// Executes Folder using the specified [`FolderConfig`].
pub fn run(config: &FolderConfig) {
    if config.mode == Mode::NEW {
        let command = Command::new("sh")
            .arg("-c")
            .arg(format!("mkdir -v {}", config.dir_name))
            .output()
            .expect("Failed to spawn mkdir process");

        let stdout = format_output(&command.stdout);
        let stderr = format_output(&command.stderr);

        if stdout != "" {
            println!("{}", stdout);
        }
        if stderr != "" {
            eprintln!("{}", stderr);
            process::exit(1);
        }
    } else if config.mode == Mode::DELETE {
        let command = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -v -r -I {}", config.dir_name))
            .spawn()
            .expect("Failed to spawn rm process");

        let output = command.wait_with_output().unwrap();

        process::exit(output.status.code().unwrap_or(0));
    }
}
