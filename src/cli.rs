use std::{
    io::{self, Write},
    process,
};

// Should be the same as the one specified
// at the Cargo.toml file.
pub const VERSION: &str = "2.0.0";

#[derive(Debug, PartialEq)]
/// CLI options.
pub enum Options {
    HELP,
    VERSION,

    /// This is not a CLI option, it's just used to
    /// handle the case where an unknown argument was passed.
    UNKNOWN,
}

/// Takes in all the cli arguments as a parameter and
/// resolves which arguments are valid cli options.
///
/// Automatically executes the options found.
pub fn parse_options(args: &[String]) {
    let mut options: Vec<Options> = Vec::new();

    for arg in args {
        // Ignore arg if it's not formatted like an option.
        if !arg.starts_with("--") && !arg.starts_with("-") {
            continue;
        }

        match arg.as_str() {
            "--version" | "-v" => options.push(Options::VERSION),
            "--help" | "-h" => options.push(Options::HELP),

            _ => options.push(Options::UNKNOWN),
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
            Options::UNKNOWN => {
                eprintln!("folder: An UNKNOWN option was passed.");
                process::exit(1);
            }
        }
    }
}

/// Presents a question to the user with a yes or no answer. It returns `Ok(bool)` where bool is
/// `true` if the answer was (y)es, or `false` if the answer was (n)o.
///
/// This function won't return until the user provides a valid yes or no answer.
///
/// This function will fail if any of the IO operations fail.
pub fn question(q: &'static str) -> io::Result<bool> {
    let mut buf = String::new();

    print!("{q} [(y)es, (n)n] ");
    io::stdout().flush()?;

    loop {
        io::stdin().read_line(&mut buf)?;

        match buf.to_lowercase().as_str() {
            "yes" | "y" => return Ok(true),
            "no" | "n" => return Ok(false),
            _ => {}
        }

        print!("{q} [(y)es, (n)n] ");
        io::stdout().flush()?;
    }
}
