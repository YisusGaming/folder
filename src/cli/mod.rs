use std::{collections::HashMap, process};

// Should be the same as the one specified
// at the Cargo.toml file.
pub const VERSION: &str = "1.0.0";


#[derive(PartialEq)]
pub enum Options {
    HELP,
    VERSION,
    UNKNOW
}
fn options_map() -> HashMap<String, Options> {
    let mut map: HashMap<String, Options> = HashMap::new();

    map.insert(String::from("--help"), Options::HELP);
    map.insert(String::from("--version"), Options::VERSION);

    map
}

/// Takes in all the cli arguments as a parameter and
/// resolves which arguments are valid cli options.
/// 
/// Automatically executes the options found.
pub fn parse_options(args: &Vec<String>) {
    let map = options_map();
    let mut options: Vec<&Options> = Vec::new();

    if args.len() == 0 { return };

    for (i, arg) in args.iter().enumerate() {
        // We skip the first argument since
        // it's traditionally the path of the executable.
        if i == 0 { continue; }
        // We also ignore any argument not formatted as an option.
        if !arg.starts_with("--") { continue; }

        let option = map.get(arg).unwrap_or(&Options::UNKNOW);
        options.push(option);
    }

    run_options(options);

}

pub fn run_options(ops: Vec<&Options>) {
    for op in ops {
        if op == &Options::HELP {
            println!("Help message");

            // Options like this would trigger a premature exit.
            process::exit(0);
        } else if op == &Options::VERSION {
            println!("Folder v{VERSION}");
            println!("Made by Yisus. 2023.\n@ItsJustYisus & @yisuscoding on Youtube.");
            println!("Happy Hacking!");

            // Options like this would trigger a premature exit.
            process::exit(0);
        } else if op == &Options::UNKNOW {
            eprintln!("folder: An unknow option was passed.");
            process::exit(1);
        }
    }
}