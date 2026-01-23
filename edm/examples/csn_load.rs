//! Example: Load a CSN JSON file and parse it into the internal Rust structure

use edm::csn::CsnModel;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <csn.json>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let model: CsnModel = serde_json::from_str(&contents).expect("Failed to parse CSN JSON");
    for (key, x) in model.definitions.iter() {
        match x {
            edm::csn::CsnDefinition::Entity(_) => println!("Entity: {}", key),
            edm::csn::CsnDefinition::Type(csn_type) => println!("Type: {} {}", key, csn_type.base_type.as_deref().unwrap_or("")),
            edm::csn::CsnDefinition::Service(_) => println!("Service: {}", key),
            edm::csn::CsnDefinition::Aspect(_) => println!("Aspect: {}", key),
            edm::csn::CsnDefinition::Context(_) => println!("Context: {}", key),
            edm::csn::CsnDefinition::Action(_) => println!("Action: {}", key),
            edm::csn::CsnDefinition::Function(_) => println!("Function: {}", key),
            edm::csn::CsnDefinition::Other => println!("Other: {}", key),
        }
        
    }
}
