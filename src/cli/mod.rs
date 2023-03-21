use std::collections::HashMap;

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

pub fn parse_options(args: Vec<String>) {
    let map = options_map();
    let mut options: Vec<&Options> = Vec::new();

    for arg in args {
        let option = map.get(&arg).unwrap_or(&Options::UNKNOW);
        options.push(option);
    }

    run_options(options);

}

pub fn run_options(ops: Vec<&Options>) {

}