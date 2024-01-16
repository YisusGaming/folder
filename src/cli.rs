use std::process;

// Should be the same as the one specified
// at the Cargo.toml file.
pub const VERSION: &str = "1.0.0";

#[derive(PartialEq)]
/// CLI options.
pub enum Options {
    HELP,
    VERSION,

    /// This is not a CLI option, it's just used to
    /// handle the case where an unknow argument was passed.
    UNKNOW,
}

/// Takes in all the cli arguments as a parameter and
/// resolves which arguments are valid cli options.
///
/// Automatically executes the options found.
pub fn parse_options(args: &[String]) {
    let mut options: Vec<Options> = Vec::new();

    for arg in args {
        // Ignore arg if it's not formatted like an option.
        if !arg.starts_with("--") || !arg.starts_with("-") {
            continue;
        }

        match arg.as_str() {
            "--version" | "-v" => options.push(Options::VERSION),
            "--help" | "-h" => options.push(Options::HELP),

            _ => options.push(Options::UNKNOW),
        }
    }

    run_options(&options);
}

/// Runs a list of CLI [`Options`].
pub fn run_options(ops: &[Options]) {
    for op in ops {
        match op {
            Options::HELP => {
                println!("Usage:");
                println!("    folder <mode> <path>");
                println!("Modes:");
                println!("    new - Creates a new folder.");
                println!("    del - Deletes a folder and its contents.");
                println!("Options:");
                println!("    -v, --version | Shows the program version.");
                println!("    -h, --help| Shows help message.");

                // Options like this would trigger a premature exit.
                process::exit(0);
            }
            Options::VERSION => {
                println!("Folder v{VERSION}");
                println!("Made by Yisus. 2024.\n@ItsJustYisus & @yisuscoding on Youtube.");
                println!("Happy Hacking!");

                // Options like this would trigger a premature exit.
                process::exit(0);
            }
            Options::UNKNOW => {
                eprintln!("folder: An unknow option was passed.");
                process::exit(1);
            }
        }
    }
}
