use std::process::Command;
use std::io::{self, Write};

pub enum Mode {
    NEW,
    DELETE,
    UNKNOW
}

pub struct FolderConfig {
    pub mode: Mode,
    pub dir_name: String
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
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("mkdir -v {}", config.dir_name))
        .output()
        .expect("Failed to created folder");
    
    io::stdout().write_all(&output.stdout).unwrap();
}