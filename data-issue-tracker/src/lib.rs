// This file defines the public API of the "data issue tracker" library.

use axum::Router;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::PathBuf};
use tracing::{debug, error, info};

pub mod batch;
pub mod odata;


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
    #[serde(default)]
    pub target: String,
    pub cardinality: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityModel {
    pub entity: String,
    #[serde(default)]
    pub purpose: String,
    pub title_attribute: String,
    #[serde(default)]
    pub data_directory: String,
    #[serde(default)]
    pub service_name: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityRef {
    pub entity: String,
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub etag: String,
}

#[tracing::instrument]
pub fn load_entity_models() -> HashMap<String, EntityModel> {
    info!("Loading entity models from YAML files in 'entity-model' directory");
    let dir = "entity-model";
    let mut map = HashMap::new();
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        info!("Processing file: {}", path.display());
                        match serde_yaml::from_str::<EntityModel>(&content) {
                            Ok(mut model) => {
                                if model.data_directory.is_empty() {
                                    model.data_directory = format!("data/{}", model.entity);
                                }
                                if model.service_name.is_empty() {
                                    model.service_name = format!("Service.{}", model.entity);
                                }
                                map.insert(model.entity.clone(), model);
                            }
                            Err(e) => {
                                error!(
                                    "Failed to parse YAML content from '{}': {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    } else {
                        error!("Failed to read file '{}'", path.display());
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to read directory '{}': {}", dir, e);
        }
    }

    map
}

impl EntityModel {
    pub fn scan_relations(&self) {
        for r in self.relations.as_ref().unwrap() {
            info!("Relation: {} ({} - {})", r.name, r.type_name, r.cardinality);
            match r.cardinality.as_str() {
                "many-to-many" | "one-to-many" => {}
                _ => {}
            }
        }
    }

    pub fn multi_relations(&self) -> Vec<Relation> {
        if let Some(relations) = &self.relations {
            relations
                .iter()
                .filter(|r| r.cardinality == "many-to-many" || r.cardinality == "one-to-many")
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }

    pub fn single_relations(&self) -> Vec<Relation> {
        if let Some(relations) = &self.relations {
            relations
                .iter()
                .filter(|r| r.cardinality == "one-to-one" || r.cardinality == "many-to-one")
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }

    pub fn relations(&self) -> Vec<Relation> {
        if let Some(relations) = &self.relations {
            relations.clone()
        } else {
            vec![]
        }
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }
}

#[derive(Debug)]
pub struct AppState {
    entities: EntityMap,
    router: Mutex<Option<Arc<Router>>>,
}

impl AppState {
    pub fn entities(&self) -> EntityMap {
        self.entities.clone()
    }

    pub fn get_entity_model(&self, entity: &str) -> Result<&EntityModel, String> {
        match self.entities.get(entity) {
            Some(model) => Ok(model),
            None => Err(format!("Entity model for '{}' not found", entity)),
        }
    }

    pub fn load_entity_refs(&self, entity: &str) -> Vec<EntityRef> {
        let data_dir = format!("data/{}", entity);
        let mut refs = Vec::new();
        if let Ok(entries) = fs::read_dir(&data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let id = path.file_stem().and_then(|s| s.to_str()).unwrap();
                    let start = id.find('-').unwrap_or(0);
                    let etag = etag_from_path(&path);
                    let er = EntityRef {
                        entity: entity.to_string(),
                        id: id[(start + 1)..].to_string(),
                        label: String::new(),
                        etag,
                    };
                    debug!("Loading entity ref: {:?}", er);
                    refs.push(er);
                }
            }
        }
        refs
    }

    pub fn load_entity_values(&self, entity: &str) -> Vec<SelectionEntry> {
        let entity_info = self.get_entity_model(entity).unwrap();
        let data_dir = format!("data/{}", entity);
        let mut values = Vec::new();
        if let Ok(entries) = fs::read_dir(&data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                            values.push(SelectionEntry {
                                id: record
                                    .get("id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                label: record
                                    .get(&entity_info.title_attribute)
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                status: String::new(),
                            });
                        }
                    }
                }
            }
        }
        values
    }

    pub fn build_relation_options(&self, entity: &str) -> Result<RelationOptions, String> {
        let mut result = HashMap::new();
        let model = self.get_entity_model(entity)?;
        for x in model.relations.as_ref().unwrap() {
            let values = self.load_entity_values(&x.type_name);
            result.insert(x.name.clone(), values);
        }
        Ok(result)
    }

    pub fn get_all_records(&self, ids: Vec<EntityRef>) -> Vec<serde_json::Value> {
        let mut records = Vec::new();
        for id in ids {
            records.push(self.get_record(&id.entity, &id.id));
        }
        records
    }

    pub fn get_record(&self, entity: &str, id: &str) -> serde_json::Value {
        let _model = self.get_entity_model(entity).unwrap();
        let path = format!("data/{}/{}-{}.yaml", entity, entity, id);
        if let Ok(content) = fs::read_to_string(&path) {
            let meta = fs::metadata(&path).unwrap();
            let mut json_src =
                if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                    serde_json::to_value(record).unwrap_or_default()
                } else {
                    serde_json::Value::default()
                };
            let obj = json_src.as_object_mut().unwrap();
            let uri = format!(
                "/api/{}('{}')",
                entity,
                obj["id"].as_str().unwrap_or_default()
            );
            info!("uri: {}", uri);
            obj.insert(
                "last_modified".to_string(),
                serde_json::Value::String(
                    meta.modified()
                        .map(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339())
                        .unwrap_or_else(|_| "".to_string()),
                ),
            );
            obj.insert(
                "__metadata".to_string(),
                json!({
                    "type": _model.service_name(),
                    "uri": uri,
                    "etag": etag_from_path(&PathBuf::from(&path)),
                }),
            );
            json_src
        } else {
            serde_json::Value::default()
        }
    }

    pub fn save_record(
        &self,
        entity: &str,
        mut record: serde_json::Value,
    ) -> Result<String, String> {
        let mut id = record
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| String::new());
        if id.is_empty() {
            id = uuid::Uuid::new_v4().to_string();
        }

        let r = record.as_object_mut().unwrap();

        let e = self.get_entity_model(entity)?;
        for a in e.single_relations() {
            if r.contains_key(&a.name) {
                let v = r[&a.name].as_str().unwrap_or_default();
                if !v.is_empty() && !v.starts_with(&a.type_name) {
                    info!("reference to {}({}) required.", a.type_name, v);
                    r[&a.name] = serde_json::Value::String(format!("{}('{}')", a.type_name, v));
                }
            }
        }
        r.insert("id".to_string(), serde_json::Value::String(id.clone()));
        let yaml = match serde_yaml::to_string(&record) {
            Ok(y) => y,
            Err(e) => Err(format!("could not convert to YAML: {}", e))?,
        };
        let data_dir = format!("data/{}", entity);
        if let Err(e) = fs::create_dir_all(&data_dir) {
            Err(format!("Failed to create data dir: {}", e))?;
        }
        let file_path = format!("{}/{}-{}.yaml", data_dir, entity, id);
        if let Err(e) = std::fs::write(&file_path, yaml) {
            Err(format!("Failed to write file: {}", e))?;
        };
        Ok(id)
    }

    pub fn new() -> Arc<Self> {
        let entities = load_entity_models();
        Arc::new(AppState {
            entities: Arc::new(entities),
            router: Mutex::new(None),
        })
    }

    pub fn router(&self) -> Arc<Router> {
        self.router.lock().unwrap().as_ref().unwrap().clone()
    }

    pub fn set_router(&self, router: Router) {
        let mut m = self.router.lock().unwrap();
        *m = Some(Arc::new(router));
    }
}

fn etag_from_path(path: &PathBuf) -> String {
    let meta = path.metadata().unwrap();
    let etag = meta
        .modified()
        .ok()
        .and_then(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339().into())
        .unwrap_or_default();
    etag
}

pub fn any_to_string<T: ToString>(value: T) -> String {
    value.to_string()
}

fn parse_odata_entity_path(path: &str) -> Option<(String, String)> {
    // Example: path = "User('abc-123')"
    if let Some(idx) = path.find('(') {
        let entity = &path[..idx];
        let rest = &path[idx..];
        if rest.starts_with("('") && rest.ends_with("')") && rest.len() > 4 {
            let id = &rest[2..rest.len() - 2];
            return Some((entity.to_string(), id.to_string()));
        }
    }
    None
}
