use std::process;

// Should be the same as the one specified
// at the Cargo.toml file.
pub const VERSION: &str = "1.0.0";

#[derive(PartialEq)]
pub enum Options {
    HELP,
    VERSION,
    UNKNOW,
}

/// Takes in all the cli arguments as a parameter and
/// resolves which arguments are valid cli options.
///
/// Automatically executes the options found.
pub fn parse_options(args: &[String]) {
    let mut options: Vec<Options> = Vec::new();

    if args.len() == 0 {
        return;
    };

    for (i, arg) in args.iter().enumerate() {
        // We skip the first argument since
        // it's traditionally the path of the executable.
        if i == 0 {
            continue;
        }
        // We also ignore any argument not formatted as an option.
        if !arg.starts_with("--") || !arg.starts_with("-") {
            continue;
        }

        let option = match arg.as_str() {
            "--help" | "-h" => Options::HELP,
            "--version" | "-v" => Options::VERSION,
            _ => Options::UNKNOW,
        };

        options.push(option);
    }

    run_options(&options);
}

pub fn run_options(ops: &[Options]) {
    for op in ops {
        if op == &Options::HELP {
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
        } else if op == &Options::VERSION {
            println!("Folder v{VERSION}");
            println!("Made by Yisus. 2024.\n@ItsJustYisus & @yisuscoding on Youtube.");
            println!("Happy Hacking!");

            // Options like this would trigger a premature exit.
            process::exit(0);
        } else if op == &Options::UNKNOW {
            eprintln!("folder: An unknow option was passed.");
            process::exit(1);
        }
    }
}
