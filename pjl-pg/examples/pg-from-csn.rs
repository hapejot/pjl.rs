use clap::Parser;
use edm::csdl::Property;
use edm::csn::{CsnDefinition, CsnModel};
use edm::Schema;
use pjl_pg::Database;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
struct Args {
    /// Namespace for entities
    #[clap(short, long)]
    namespace: String,

    /// Path to the CSN JSON file
    #[clap(short, long)]
    csn_file: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let filename = args.csn_file;
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let model: CsnModel = serde_yaml::from_str(&contents).expect("Failed to parse CSN JSON");
    let mut s = Schema::new();
    let key_n = args.namespace.len();
    for (key, x) in model.definitions.iter() {
        if key.starts_with(&args.namespace) {
            match x {
                CsnDefinition::Entity(csn_entity) => {
                    let entity_name = &key[key_n..];
                    println!("\n\nEntity: {}", entity_name);
                    s.new_entity(entity_name);
                    for (prop_name, prop) in csn_entity.elements.as_ref().unwrap().iter() {
                        if let Some(datatype) = &prop.datatype {
                            match datatype.as_str() {
                                "cds.Association" | "cds.Composition" => {
                                    println!(
                                        "  {} - {:?} {}",
                                        prop_name,
                                        prop.cardinality,
                                        prop.target.as_ref().unwrap()
                                    );
                                }
                                _ => {
                                    s.set_property(
                                        entity_name,
                                        Property::new(prop_name).with_type(datatype.to_string()),
                                    );
                                    println!("  {} - {}", prop_name, datatype);
                                }
                            }
                        } else {
                            println!("  {}", prop_name);
                        }
                    }
                }
                CsnDefinition::Service(csn_service) => println!("Service: {}", key),
                CsnDefinition::Aspect(csn_aspect) => println!("Aspect: {}", key),
                CsnDefinition::Context(csn_context) => println!("Context: {}", key),
                CsnDefinition::Action(csn_action) => println!("Action: {}", key),
                CsnDefinition::Function(csn_action) => println!("Function: {}", key),
                CsnDefinition::Other => println!("Other: {}", key),
                CsnDefinition::Type(csn_type) => todo!(),
            }
        }
    }

    if let Ok(mut db) =
        Database::new("host=localhost user=peter password=Kennwort01 dbname=blog").await
    {
        // println!("{s:#?}");

        db.activate(s).await.unwrap();
    }
}
