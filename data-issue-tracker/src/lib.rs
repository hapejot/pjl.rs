//! # data-issue-tracker Library
//!
//! This library provides the core data models, entity logic, and helper functions for the OData/REST backend service.
//! It contains structures for entities, attributes, relations, and functions for loading, saving, and managing entity data from YAML files.
//!
//! Main components:
//! - EntityModel, Attribute, Relation: Data models for entities
//! - AppState: Central application structure with entity and router management
//! - Helper functions for OData and REST APIs

use axum::Router;
use chrono::{DateTime, Utc};
use serde::{ser, Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::PathBuf};
use tracing::{debug, error, info};

pub mod batch;
pub mod odata;
pub mod odatav4;

/// Describes an attribute of an entity.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub description: String,
    #[serde(default)]
    pub nullable: bool,
}

/// Describes a relation between entities, including name, type, target, cardinality, and description.
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

/// Describes the data model of an entity.
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
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    pub relations: Vec<Relation>,
}

pub enum EntityResult {
    Single {
        entity: String,
        id: String,
        label: String,
        etag: String,
        value: serde_json::Value,
    },
    Collection {
        entity: String,
        etag: String,
        count: usize,
        next_token: Option<String>,
        value: serde_json::Value,
    },
}

/// Map from entity name to their model definition.
pub type EntityMap = Arc<HashMap<String, EntityModel>>;

/// Deprecated: Use a more specific type or structure for relation options if possible.
#[deprecated(note = "OData provides better ways to do this.")]
pub type RelationOptions = HashMap<String, Vec<SelectionEntry>>;

/// An entry for a selection (e.g., for relations), consisting of id, label, and status.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectionEntry {
    pub id: String,
    pub label: String,
    pub status: String,
}

/// Reference to an entity (e.g., for relations), including entity type, id, label, and ETag.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityRef {
    pub entity: String,
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub etag: String,
}

/// Loads all entity models from YAML files in the 'entity-model' directory.
/// Returns a HashMap mapping entity names to their models.
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
                        match serde_yaml::from_str::<EntityModel>(&content) {
                            Ok(mut model) => {
                                if model.data_directory.is_empty() {
                                    model.data_directory = format!("data/{}", model.entity);
                                }
                                if model.service_name.is_empty() {
                                    model.service_name = format!("Service.{}", model.entity);
                                }
                                info!(
                                    "Entity: {:20} Attributes: {:3}, Relations: {:3}",
                                    &model.entity,
                                    model.attributes().len(),
                                    model.relations().len()
                                );
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
    /// Logs all relations of the entity for debugging purposes.
    pub fn scan_relations(&self) {
        for r in self.relations() {
            info!("Relation: {} ({} - {})", r.name, r.type_name, r.cardinality);
            match r.cardinality.as_str() {
                "many-to-many" | "one-to-many" => {}
                _ => {}
            }
        }
    }

    /// Returns all relations with cardinality 'many-to-many' or 'one-to-many'.
    pub fn multi_relations(&self) -> Vec<Relation> {
        self.relations
            .iter()
            .filter(|r| r.cardinality == "many-to-many" || r.cardinality == "one-to-many")
            .cloned()
            .collect()
    }

    /// Returns all relations with cardinality 'one-to-one' or 'many-to-one'.
    pub fn single_relations(&self) -> Vec<Relation> {
        self.relations
            .iter()
            .filter(|r| r.cardinality == "one-to-one" || r.cardinality == "many-to-one")
            .cloned()
            .collect()
    }

    /// Returns the OData service name for the entity.
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Returns a reference to the list of attributes for the entity.
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    /// Returns a reference to the list of relations for the entity.
    pub fn relations(&self) -> &Vec<Relation> {
        &self.relations
    }

    pub fn title_attribute(&self) -> &str {
        &self.title_attribute
    }
}

/// Represents the application state, including entity models and a router.
/// It caches the list of entities it loaded once during startup.
/// It also links to the router for handling HTTP requests.
/// This structure is designed to be shared across the application, allowing access to entity models and routing
/// logic in a thread-safe manner.
/// It provides methods to load entity references, values, and relations, as well as to get records and save them.
/// The `AppState` is designed to be used in a web server context, where it can be shared across multiple requests and threads.
#[derive(Debug, Default)]
pub struct AppState {
    entities: EntityMap,
    router: Mutex<Option<Arc<Router>>>,
}

impl AppState {
    /// Returns a clone of the entity map.
    pub fn entities(&self) -> EntityMap {
        self.entities.clone()
    }

    /// Returns the entity model for the given entity name, or an error if not found.
    pub fn get_entity_model(&self, entity: &str) -> Result<&EntityModel, String> {
        match self.entities.get(entity) {
            Some(model) => Ok(model),
            None => Err(format!("Entity model for '{}' not found", entity)),
        }
    }

    /// Loads all entity references for the given entity.
    /// Labels are empty, and ETags are generated from the file's last modification time.
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

    /// Loads all selection values for the given entity, using the title attribute as label.
    #[deprecated(note = "Use `load_entity_refs` instead.")]
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

    /// Builds a map of relation options for the given entity, mapping relation names to possible values.
    #[deprecated(note = "Use `load_entity_refs` instead.")]
    pub fn build_relation_options(&self, entity: &str) -> Result<RelationOptions, String> {
        let mut result = HashMap::new();
        let model = self.get_entity_model(entity)?;
        for x in model.relations() {
            let values = self.load_entity_values(&x.type_name);
            result.insert(x.name.clone(), values);
        }
        Ok(result)
    }

    /// Loads all records for the given list of entity references.
    pub fn get_all_records(&self, ids: Vec<EntityRef>) -> Vec<serde_json::Value> {
        let mut records = Vec::new();
        for id in ids {
            let local_id = serde_json::Value::String(id.id.clone());
            records.push(self.get_record("/lib", &id.entity, &local_id));
        }
        records
    }

    /// Loads a single record for the given entity and ID, returning it as JSON.
    /// The record is loaded from a YAML file in the data directory for the entity.
    /// If the record does not exist, it returns an empty JSON object.
    /// # Arguments
    /// * `prefix` - The URL prefix for the entity (e.g., "/lib").
    /// * `entity` - The name of the entity to load.
    /// * `id` - The ID of the record to load, as a JSON value.
    ///
    /// # Returns
    /// A JSON value representing the record, or an empty JSON object if the record does not exist.
    /// The returned JSON object includes metadata such as the last modified time and ETag.
    /// # Example
    /// ```
    /// use data_issue_tracker::AppState;
    /// let state = AppState::new();
    /// let record = state.get_record("/lib", "User", &serde_json::json!("123"));
    /// println!("{:?}", record);
    /// ```
    /// The "last_modified" field will contain the last modification time of the record file.
    /// The URI is constructed using the prefix and the entity ID.
    /// The ETag is generated from the file's last modification time.
    /// and stored as @odata.etag.
    /// If the record file cannot be read, it returns an empty JSON object.
    /// If the record file is not found, it returns an empty JSON object.
    /// If the record file is malformed, it returns an empty JSON object.
    /// If the ID is not a string, it returns an empty JSON object.
    /// If the entity model is not found, it returns an error.
    pub fn get_record(
        &self,
        prefix: &str,
        entity: &str,
        id: &serde_json::Value,
    ) -> serde_json::Value {
        match self.get_record_ext(prefix, entity, id) {
            Ok(EntityResult::Single { value, .. }) => value,
            Ok(_) => {
                error!("Unexpected result type");
                serde_json::Value::default()
            }
            Err(e) => {
                error!("Failed to get record: {}", e);
                serde_json::Value::default()
            }
        }
    }

    fn get_record_ext(
        &self,
        prefix: &str,
        entity: &str,
        id: &serde_json::Value,
    ) -> Result<EntityResult, String> {
        let model = self.get_entity_model(entity)?;
        let id = id.as_str().ok_or_else(|| format!("ID must be a string"))?;
        let path = format!("data/{}/{}-{}.yaml", entity, entity, id);
        let content = fs::read_to_string(&path).map_err(any_to_string)?;
        let meta = fs::metadata(&path).map_err(any_to_string)?;
        let mut json_src = serde_json::to_value(
            serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(any_to_string)?,
        )
        .map_err(any_to_string)?;

        let obj = json_src
            .as_object_mut()
            .ok_or_else(|| "could not convert to mutable object")?;
        let uri = format!(
            "{prefix}/{entity}('{}')",
            obj["id"]
                .as_str()
                .ok_or_else(|| format!("ID must be a string"))?
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
        // obj.insert(
        //     "__metadata".to_string(),
        //     json!({
        //         "type": model.service_name(),
        //         "uri": uri,
        //         "etag": etag_from_path(&PathBuf::from(&path)),
        //     }),
        // );
        let label = obj
            .get(model.title_attribute())
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(EntityResult::Single {
            entity: entity.to_string(),
            id: id.to_string(),
            label,
            etag: etag_from_path(&PathBuf::from(&path)),
            value: serde_json::Value::Object(obj.clone()),
        })
    }

    /// Saves a record for the given entity. Generates an ID if missing, updates references, and writes YAML file.
    /// Returns the record ID or an error message.
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

    /// Creates a new AppState with loaded entity models and no router set.
    /// This function initializes the application state, loading all entity models from the 'entity-model' directory.
    /// It returns an `Arc` (atomic reference counted) pointer to the new `AppState` instance.
    /// The `Arc` allows for shared ownership of the application state across multiple threads or components,
    /// making it suitable for use in a web server context where the state needs to be accessed concurrently.
    pub fn new() -> Arc<Self> {
        let entities = load_entity_models();
        Arc::new(AppState {
            entities: Arc::new(entities),
            router: Mutex::new(None),
        })
    }

    /// Returns the current router.
    pub fn router(&self) -> Arc<Router> {
        self.router.lock().unwrap().as_ref().unwrap().clone()
    }

    /// Sets the router for the application state.
    /// should only be called once, typically during application startup.
    /// It replaces the existing router if one is already set.
    /// This allows the application to dynamically change its routing logic if needed.
    /// This is useful for testing or when the router needs to be updated after initialization.
    ///
    /// # Arguments
    /// * `router` - The new router to set for the application state.
    ///
    /// # Example
    /// ```
    /// use data_issue_tracker::AppState;
    /// use axum::routing::Router;
    /// use axum::routing::get;
    /// let state = AppState::new();
    /// let router = Router::new().route("/", get(|| async { "Hello, World!" }));
    /// state.set_router(router);
    /// ```
    pub fn set_router(&self, router: Router) {
        let mut m = self.router.lock().unwrap();
        *m = Some(Arc::new(router));
    }
}

/// Returns the last modification time of a file as a string, for use as an ETag.
/// If the file does not exist or cannot be read, it returns an empty string.
/// The ETag is formatted as an RFC 3339 timestamp.
/// This is useful for caching and conditional requests in HTTP APIs.
/// # Arguments
/// * `path` - The path to the file for which to generate the ETag.
///
/// # Returns
/// A string representing the last modification time of the file, formatted as an RFC 3339 timestamp.
/// If the file does not exist or cannot be read, it returns an empty string.
fn etag_from_path(path: &PathBuf) -> String {
    let meta = path.metadata().unwrap();
    let etag = meta
        .modified()
        .ok()
        .and_then(|mtime| DateTime::<Utc>::from(mtime).to_rfc3339().into())
        .unwrap_or_default();
    etag
}

/// Converts any value implementing ToString into a String.
/// This is a simple utility function that can be used to convert various types into their string representation.
/// # Arguments
/// * `value` - The value to convert to a String.
///
/// # Returns
/// A String representation of the value.   
pub fn any_to_string<T: ToString>(value: T) -> String {
    value.to_string()
}

/// Collects the body of an axum response and returns it as a String.
/// it failes miserably if the body is not UTF-8 encoded.
/// Use with caution, as it will panic if the body cannot be converted to a String.
pub async fn str_from_response(resp: impl axum::response::IntoResponse) -> String {
    let mut response = resp.into_response();
    let body = response.body_mut();
    let body_bytes = http_body_util::BodyExt::collect(body).await;
    let b = body_bytes.unwrap().to_bytes();
    String::from_utf8(b.to_vec()).unwrap()
}

/// parses a string of the form "EntityName(key)" into a pair of the entity name and an optional id.
/// The ID is represented as Value, since it can be a string or a number.
/// If the segment does not contain parentheses, it returns the segment as the entity name and None
/// as the ID.
/// If the segment is malformed, it returns an error message.
/// Example: "User('abc-123')" -> ("User", Some(String("abc-123")))
/// Example: "User(123)" -> ("User", Some(Number(123)))
/// Example: "User" -> ("User", None)
/// If the segment is malformed, it returns an error message.
/// Example: "User(abc-123)" -> Err("Failed to parse key: invalid number: invalid digit found in string")
///
pub fn parse_entity_ref(segment: &str) -> Result<(String, Option<serde_json::Value>), String> {
    match (segment.find("("), segment.find(")")) {
        (Some(idx), Some(end)) => {
            if idx + 2 > end || end != segment.len() - 1 {
                return Err(format!("invalid segment")); // Invalid segment
            }
            let entity = &segment[..idx];
            let after_entity = &segment[idx + 1..end];
            match serde_yaml::from_str(after_entity) {
                Ok(value) => Ok((entity.to_string(), Some(value))),
                Err(e) => Err(format!("Failed to parse key: {}", e)),
            }
        }
        (None, None) => Ok((segment.to_string(), None)),
        _ => Err(format!("Invalid segment {}", segment)), // Invalid segment, missing parentheses
    }
}
