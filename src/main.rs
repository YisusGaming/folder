//! **Folder** is just a simple wrapper around `mkdir` and `rm` that allows you create and delete folders.
//!
//! Creating a folder looks like this:
//!
//! ```sh
//! folder new dirname
//! ```
//!
//! that'll run `mkdir -v dirname` for you.
//!
//! Deleting folders looks like this:
//!
//! ```sh
//! folder del dirname
//! ```
//!
//! that'll run `rm -v -r -I dirname` for you.
//!
//! **Keep in mind!** This is not meant to be complete CLI to manage folders, it's just simply a convenience.

use folder::{FolderConfig, Mode};
use std::env;
use std::process;

mod cli;
mod folder;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Evaluate cli options first because some of them
    // could trigger a premature exit.
    cli::parse_options(&args);
    let config = parse_config(&args);
    folder::run(&config);
}

/// Generates the configuration that will be used to run Folder.
pub fn parse_config(args: &[String]) -> FolderConfig {
    if args.len() < 2 {
        eprintln!("folder: More arguments needed");
        process::exit(1);
    }

    let mode = folder::Mode::resolve_mode(&args[0]);
    match mode {
        Mode::UNKNOWN => {
            eprintln!("folder: Unknown mode {}", &args[0]);
            process::exit(1);
        }
        _ => (),
    };

    let dir_name = String::from(&args[1]);

    FolderConfig { mode, dir_name }
}
