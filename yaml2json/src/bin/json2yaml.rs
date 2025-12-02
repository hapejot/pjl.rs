use std::io::{self, Read};
use serde_json::Value;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let v: Value = serde_json::from_str(&input).expect("Failed to parse JSON");
    serde_yaml::to_writer(io::stdout(), &v).expect("Failed to write YAML");
    Ok(())
}
