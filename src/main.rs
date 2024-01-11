use folder::{FolderConfig, Mode};
use std::env;
use std::process;

pub mod cli;
pub mod folder;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Evaluate cli options first becuase some of them
    // could trigger a premature exit.
    cli::parse_options(&args);
    let config = parse_config(&args);
    folder::run(&config);
}

pub fn parse_config(args: &Vec<String>) -> FolderConfig {
    if args.len() < 3 {
        eprintln!("folder: More arguments needed");
        process::exit(1);
    }

    let mode = folder::resolve_mode(&args[1]);
    match mode {
        Mode::UNKNOW => {
            eprintln!("folder: Unknow mode {}", &args[1]);
            process::exit(1);
        }
        _ => (),
    };

    let dir_name = String::from(&args[2]);

    FolderConfig { mode, dir_name }
}

