use std::process::{self, Command};
use std::str::from_utf8;

#[derive(PartialEq)]
pub enum Mode {
    NEW,
    DELETE,
    UNKNOW
}

pub struct FolderConfig {
    pub mode: Mode,
    pub dir_name: String
}

/// Takes a buffer and returns an String
/// replacing "mkdir: " for "folder: ".
pub fn format_output(buf: &Vec<u8>) -> String {
    let str = from_utf8(&buf).unwrap_or("");
    let str = str.replace("mkdir: ", "folder: ");
    
    String::from(str.trim())
}

pub fn resolve_mode(arg: &String) -> Mode {
    if arg == "new" {
        return Mode::NEW;
    } else if arg == "del" {
        return Mode::DELETE
    }

    Mode::UNKNOW
}

pub fn run(config: &FolderConfig) {
    if config.mode == Mode::NEW {
        let command = Command::new("sh")
            .arg("-c")
            .arg(format!("mkdir -v {}", config.dir_name))
            .output()
            .expect("Failed to create folder");
    
        let stdout = format_output(&command.stdout);
        let stderr = format_output(&command.stderr);

        if stdout != "" {
            println!("{}", stdout);
        }
        if stderr != "" {
            eprintln!("{}", stderr);
            process::exit(1);
        }
    }
}