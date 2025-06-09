use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Relation {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub cardinality: Cardinality,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub enum Cardinality {
    #[serde(rename = "one-to-one")]
    OneToOne,
    #[serde(rename = "many-to-one")]
    ManyToOne,
    #[serde(rename = "one-to-many")]
    OneToMany,
    #[serde(rename = "many-to-many")]
    ManyToMany,
}

#[derive(Debug, Deserialize)]
pub struct EntityModel {
    pub entity: String,
    pub purpose: String,
    pub attributes: Option<Vec<Attribute>>,
    pub relations: Option<Vec<Relation>>,
}

fn read_entity_model<P: AsRef<Path>>(path: P) -> Result<EntityModel, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let model: EntityModel = serde_yaml::from_str(&content)?;
    Ok(model)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "entity-model";
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let model = read_entity_model(&path)?;
            println!("{:}", model.entity);
            for r in model.relations.unwrap().iter() {
                println!("  {:20} {:20} {:?}", r.name, r.type_name,  r.cardinality);
            }
            println!();
        }
    }
    Ok(())
}