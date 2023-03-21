use std::collections::HashMap;

// Should be the same as the one specified
// at the Cargo.toml file.
pub const VERSION: &str = "1.0.0";


#[derive(PartialEq)]
pub enum Options {
    HELP,
    VERSION
}
fn options_map() -> HashMap<String, Options> {
    let mut map: HashMap<String, Options> = HashMap::new();

    map.insert(String::from("--help"), Options::HELP);
    map.insert(String::from("--version"), Options::VERSION);

    map
}