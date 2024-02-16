//! **Folder** is a CLI tool that allows you manage folders in your filesystem. It's cross-platform.
//!
//! Currently, only creating and deleting folders is supported. More operations soon!
//!
//! Creating a folder looks like this:
//!
//! ```sh
//! folder new dirname
//! ```
//!
//! that'll create the path `dirname/`.
//!
//! Deleting folders looks like this:
//!
//! ```sh
//! folder del dirname
//! ```
//!
//! that'll delete the path `dirname/`.

use folder::FolderConfig;
use std::env;
use std::process;

mod cli;
mod folder;

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    // Evaluate cli options first because some of them
    // could trigger a premature exit.
    cli::parse_options(&args);
    let config = parse_config(&args);

    match folder::run(&config) {
        Ok(_) => process::ExitCode::from(0),
        Err(err) => {
            eprintln!("{err}");
            process::ExitCode::from(1)
        }
    }
}

/// Generates the configuration that will be used to run Folder.
pub fn parse_config(args: &[String]) -> FolderConfig {
    if args.len() < 2 {
        eprintln!("folder: More arguments needed");
        process::exit(1);
    }

    let mode = folder::Mode::resolve_mode(&args[0]);

    let dir_name = &args[1];

    FolderConfig { mode, dir_name }
}
