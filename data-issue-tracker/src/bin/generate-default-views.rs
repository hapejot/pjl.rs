// generate-default-views.rs
// Generates default view and form definitions for each entity in entity-model, if not already present.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Attribute {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Relation {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    cardinality: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EntityModel {
    entity: String,
    purpose: String,
    title_attribute: String,
    attributes: Option<Vec<Attribute>>,
    relations: Option<Vec<Relation>>,
}

fn main() {
    let entity_dir = "entity-model";
    let forms_dir = "forms";
    let views_dir = "view";
    fs::create_dir_all(forms_dir).unwrap();
    fs::create_dir_all(views_dir).unwrap();
    for entry in fs::read_dir(entity_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap();
        let model: EntityModel = serde_yaml::from_str(&content).unwrap();
        let entity = &model.entity;
        let form_path = format!("{}/{}.yaml", forms_dir, entity);
        let view_path = format!("{}/{}.yaml", views_dir, entity);
        // Generate default form if not exists
        if !Path::new(&form_path).exists() {
            let fields: Vec<_> = model.attributes.as_ref().map(|attrs| {
                attrs.iter().map(|a| {
                    let mut m = HashMap::new();
                    m.insert("name", a.name.clone());
                    m.insert("label", a.name.clone());
                    m.insert("type", a.type_name.clone());
                    m
                }).collect()
            }).unwrap_or_default();
            let relations: Vec<_> = model.relations.as_ref().map(|rels| {
                rels.iter().map(|r| {
                    let mut m = HashMap::new();
                    m.insert("name", r.name.clone());
                    m.insert("label", r.name.clone());
                    m.insert("type", r.type_name.clone());
                    m.insert("cardinality", r.cardinality.clone());
                    m
                }).collect()
            }).unwrap_or_default();
            let form_def = serde_yaml::to_string(&serde_yaml::Value::Mapping([
                (serde_yaml::Value::String("title".to_string()), serde_yaml::Value::String(format!("Edit {}", entity))),
                (serde_yaml::Value::String("fields".to_string()), serde_yaml::to_value(fields).unwrap()),
                (serde_yaml::Value::String("relations".to_string()), serde_yaml::to_value(relations).unwrap()),
            ].into())).unwrap();
            fs::write(&form_path, form_def).unwrap();
            println!("Created default form: {}", form_path);
        }
        // Generate default view if not exists
        if !Path::new(&view_path).exists() {
            let columns: Vec<_> = model.attributes.as_ref().map(|attrs| {
                attrs.iter().map(|a| {
                    let mut m = HashMap::new();
                    m.insert("name", a.name.clone());
                    m.insert("label", a.name.clone());
                    m
                }).collect()
            }).unwrap_or_default();
            let view_def = serde_yaml::to_string(&serde_yaml::Value::Mapping([
                (serde_yaml::Value::String("title".to_string()), serde_yaml::Value::String(format!("List of {}", entity))),
                (serde_yaml::Value::String("columns".to_string()), serde_yaml::to_value(columns).unwrap()),
            ].into())).unwrap();
            fs::write(&view_path, view_def).unwrap();
            println!("Created default view: {}", view_path);
        }
    }
}
