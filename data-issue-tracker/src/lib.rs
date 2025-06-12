// This file defines the public API of the "data issue tracker" library.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relation {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub cardinality: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityModel {
    pub entity: String,
    pub purpose: String,
    pub title_attribute: String,
    pub attributes: Option<Vec<Attribute>>,
    pub relations: Option<Vec<Relation>>,
}

pub type EntityMap = Arc<HashMap<String, EntityModel>>;
pub type RelationOptions = HashMap<String, Vec<SelectionEntry>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectionEntry {
    pub id: String,
    pub label: String,
    pub status: String,
}

#[tracing::instrument]
pub fn load_entity_models() -> HashMap<String, EntityModel> {
    let dir = "entity-model";
    let mut map = HashMap::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(model) = serde_yaml::from_str::<EntityModel>(&content) {
                        map.insert(model.entity.clone(), model);
                    }
                }
            }
        }
    }
    map
}

