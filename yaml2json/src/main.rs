use std::io::{self, Read};
use serde_json::Value;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let yaml: Value = serde_yaml::from_str(&input).expect("Failed to parse YAML");
    serde_json::to_writer(io::stdout(), &yaml).expect("Failed to write JSON");
    Ok(())
}
