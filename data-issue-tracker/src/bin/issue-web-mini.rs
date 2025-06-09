use axum::{
    extract::Path,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router, Json,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_yaml;
use std::fs::create_dir_all;
use std::{collections::HashMap, fs, sync::Arc};
use tracing::{info, instrument};
use uuid::Uuid;

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

type EntityMap = Arc<HashMap<String, EntityModel>>;
type RelationOptions = HashMap<String, Vec<SelectionEntry>>;

#[derive(Debug)]
struct AppState {
    hb: Arc<Handlebars<'static>>,
    entities: EntityMap,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SelectionEntry {
    pub id: String,
    pub label: String,
    pub status: String,
}

impl AppState {
    fn handlebars(&self) -> Arc<Handlebars<'static>> {
        self.hb.clone()
    }
    fn entities(&self) -> EntityMap {
        self.entities.clone()
    }

    pub fn get_entity_model(&self, entity: &str) -> Option<&EntityModel> {
        self.entities.get(entity)
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

    pub fn build_relation_options(&self, entity: &str) -> RelationOptions {
        let mut result = HashMap::new();
        if let Some(model) = self.get_entity_model(entity) {
            for x in model.relations.as_ref().unwrap() {
                let values = self.load_entity_values(&x.type_name);
                result.insert(x.name.clone(), values);
            }
        }
        result
    }

    pub fn record_from_form(
        &self,
        entity: &str,
        form: Vec<(String, String)>,
    ) -> HashMap<String, serde_json::Value> {
        let mut rec = HashMap::new();
        if let Some(model) = self.get_entity_model(entity) {
            if let Some(attrs) = &model.attributes {
                for attr in attrs {
                    if let Some(value) = extract_value(&form, attr.name.as_str()) {
                        if !value.is_empty() {
                            rec.insert(attr.name.clone(), serde_json::Value::String(value));
                        }
                    }
                }
            }
            if let Some(relations) = &model.relations {
                for rel in relations {
                    rec.insert(
                        rel.name.clone(),
                        serde_json::to_value(extract_values(&form, rel.name.as_str())).unwrap(),
                    );
                }
            }
        }
        rec
    }

    pub fn get_record(&self, entity: &str, id: &str) -> serde_json::Value {
        if let Ok(content) = fs::read_to_string(&format!("data/{}/{}-{}.yaml", entity, entity, id))
        {
            if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                serde_json::to_value(record).unwrap_or_default()
            } else {
                serde_json::Value::default()
            }
        } else {
            serde_json::Value::default()
        }
    }
}

fn extract_value(form: &Vec<(String, String)>, attr: &str) -> Option<String> {
    form.iter()
        .filter_map(|(k, v)| if k == attr { Some(v.clone()) } else { None })
        .nth(0)
}

fn extract_values(form: &Vec<(String, String)>, attr: &str) -> Vec<String> {
    form.iter()
        .filter_map(|(k, v)| if k == attr { Some(v.clone()) } else { None })
        .collect()
}

#[instrument(skip(state))]
async fn list_entities(state: Arc<AppState>) -> impl IntoResponse {
    info!("Listing entities");
    let entities = state.entities();
    let entity_names: Vec<&String> = entities.keys().collect();
    let body = state.handlebars().render("list", &entity_names).unwrap();
    Html(body)
}

fn build_relation_options(
    model: &EntityModel,
    entities: &HashMap<String, EntityModel>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut relation_options = serde_json::Map::new();
    if let Some(relations) = &model.relations {
        for rel in relations {
            let rel_dir = format!("data/{}", rel.type_name);
            let type_detail = entities.get(&rel.type_name).unwrap();
            let mut options = Vec::new();
            if let Ok(entries) = fs::read_dir(&rel_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content)
                            {
                                let id = record.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                let label = record
                                    .get(&type_detail.title_attribute)
                                    .and_then(|v| v.as_str())
                                    .unwrap_or(id);
                                options.push(json!({"id": id, "label": label}));
                            }
                        }
                    }
                }
            }
            relation_options.insert(rel.name.clone(), json!(options));
        }
    }
    relation_options
}

#[instrument(skip(state))]
async fn new_entity(Path(entity): Path<String>, state: Arc<AppState>) -> impl IntoResponse {
    info!(entity = %entity, "Editing entity");
    let entities = state.entities();
    if let Some(model) = entities.get(&entity) {
        let relation_options = state.build_relation_options(&entity);
        let mut ctx = serde_json::to_value(model).unwrap();
        ctx["relation_options"] = serde_json::to_value(relation_options).unwrap();
        let body = state.handlebars().render("edit", &ctx).unwrap();
        Html(body)
    } else {
        Html("Entity not found".to_string())
    }
}

#[instrument(skip(state))]
async fn save_entity(
    Path(entity): Path<String>,
    Form(form): Form<Vec<(String, String)>>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    info!(form=?form, "Saving entity record");
    let mut record = (*state).record_from_form(&entity, form);
    let id = if let Some(serde_json::Value::String(id)) = record.get("id").cloned() {
        if (&id).len() == 0 {
            let id = Uuid::new_v4().to_string();
            record.insert("id".to_string(), serde_json::to_value(id.clone()).unwrap());
            id
        } else {
            id
        }
    } else {
        let id = Uuid::new_v4().to_string();
        record.insert("id".to_string(), serde_json::to_value(id.clone()).unwrap());
        id
    };
    let yaml = serde_yaml::to_string(&record).unwrap();
    let data_dir = format!("data/{}", entity);
    let _ = create_dir_all(&data_dir);
    let file_path = format!("{}/{}-{}.yaml", data_dir, entity, id);
    std::fs::write(&file_path, yaml).unwrap();
    Redirect::to(&format!("/edit/{}", entity))
}

#[instrument(skip(state))]
async fn list_records(Path(entity): Path<String>, state: Arc<AppState>) -> impl IntoResponse {
    info!(entity = %entity, "Listing records");
    let model = if let Some(m) = state.get_entity_model(&entity) {
        m
    } else {
        return Html("Entity not found".to_string());
    };
    let data_dir = format!("data/{}", entity);
    let mut records = Vec::new();
    if let Ok(entries) = fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        records.push(record);
                    }
                }
            }
        }
    }
    info!("Found {} records for entity {}", records.len(), entity);
    let mut ctx = serde_json::Map::new();
    ctx.insert(
        "entity".to_string(),
        serde_json::Value::String(entity.clone()),
    );
    ctx.insert(
        "attributes".to_string(),
        serde_json::to_value(&model.attributes).unwrap_or_default(),
    );
    let records = serde_json::to_value(records).unwrap_or_default();
    info!("records: {:?}", records);
    ctx.insert("records".to_string(), records);
    let body = state.handlebars().render("records", &ctx).unwrap();
    Html(body)
}

// --- REST API HANDLERS ---

#[instrument(skip(state))]
async fn api_list_records(Path(entity): Path<String>, state: Arc<AppState>) -> impl IntoResponse {
    info!(entity = %entity, "API: Listing records");
    let model = if let Some(m) = state.get_entity_model(&entity) {
        m
    } else {
        return (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Entity not found"})));
    };
    let data_dir = format!("data/{}", entity);
    let mut records = Vec::new();
    if let Ok(entries) = fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(record) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        // Convert YAML value to JSON value for API response
                        let json_record = serde_json::to_value(record).unwrap_or_default();
                        records.push(json_record);
                    }
                }
            }
        }
    }
    (axum::http::StatusCode::OK, Json(json!(records)))
}

#[instrument(skip(state))]
async fn api_get_record(Path((entity, id)): Path<(String, String)>, state: Arc<AppState>) -> impl IntoResponse {
    info!(entity = %entity, id = %id, "API: Get record");
    let record = state.get_record(&entity, &id);
    if record.is_null() {
        (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Record not found"})))
    } else {
        (axum::http::StatusCode::OK, Json(record))
    }
}

#[instrument]
fn load_entities() -> HashMap<String, EntityModel> {
    info!("Loading entity models");
    let mut map = HashMap::new();
    let dir = "entity-model";
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

#[instrument(skip(state))]
async fn change_entity(
    Path((entity, id)): Path<(String, String)>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    info!(entity = %entity, "Editing entity record");
    if let Some(model) = state.get_entity_model(&entity) {
        let mut relation_options = state.build_relation_options(&entity);
        let record = state.get_record(&entity, &id);

        update_relation_options(&mut relation_options, &record, model);

        let mut ctx = serde_json::to_value(model).unwrap();
        let r_opts = serde_json::to_value(relation_options).unwrap();
        info!(relation_options = ?r_opts, "Relation options for entity");
        ctx["relation_options"] = r_opts;
        ctx["record"] = record;
        let body = state.handlebars().render("edit", &ctx).unwrap();
        Html(body)
    } else {
        Html("Entity not found".to_string())
    }
}

fn update_relation_options(
    relation_options: &mut RelationOptions,
    record: &serde_json::Value,
    model: &EntityModel,
) {
    for r in model.relations.as_ref().unwrap() {
        match r.cardinality.as_str() {
            "many-to-many" | "one-to-many" => {
                let values = relation_options.get_mut(&r.name).unwrap();
                if let Some(x) = record.get(&r.name).and_then(|v| v.as_array()) {
                    for item in x {
                        if let Some(id) = item.as_str() {
                            if let Some(entry) = values.iter_mut().find(|e| &e.id == id) {
                                entry.status = " selected".to_string();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    info!(relation_options = ?relation_options, "Updated relation options");
}

#[instrument(skip(state, payload))]
async fn api_upsert_record(
    Path(entity): Path<String>,
    Json(payload): Json<serde_json::Value>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    info!(entity = %entity, payload = ?payload, "API: Upsert record");
    // Extract or generate ID
    let id = payload.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    // Write to YAML file
    let yaml = match serde_yaml::to_string(&payload) {
        Ok(y) => y,
        Err(e) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": format!("YAML serialization error: {}", e)}))),
    };
    let data_dir = format!("data/{}", entity);
    if let Err(e) = create_dir_all(&data_dir) {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to create data dir: {}", e)})));
    }
    let file_path = format!("{}/{}-{}.yaml", data_dir, entity, id);
    if let Err(e) = std::fs::write(&file_path, yaml) {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to write file: {}", e)})));
    }
    (axum::http::StatusCode::OK, Json(json!({"id": id})))
}

#[instrument(skip(state))]
async fn api_list_entities(state: Arc<AppState>) -> impl IntoResponse {
    let entities: Vec<String> = state.entities.keys().cloned().collect();
    (axum::http::StatusCode::OK, Json(entities))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut hb = Handlebars::new();
    hb.register_template_file("list", std::path::Path::new("templates/list.hbs"))
        .expect("Failed to load list.hbs");
    hb.register_template_file("edit", std::path::Path::new("templates/edit.hbs"))
        .expect("Failed to load edit.hbs");
    hb.register_template_file("records", std::path::Path::new("templates/records.hbs"))
        .expect("Failed to load records.hbs");
    let entities = Arc::new(load_entities());
    let hb = Arc::new(hb);
    let state = Arc::new(AppState { hb, entities });
    let app = Router::new()
        .route(
            "/",
            get({
                let state = state.clone();
                move || list_entities(state)
            }),
        )
        .route(
            "/edit/{entity}",
            get({
                let state = state.clone();
                move |path| new_entity(path, state)
            }),
        )
        .route(
            "/edit/{entity}",
            post({
                let state = state.clone();
                move |path, form| save_entity(path, form, state)
            }),
        )
        .route(
            "/records/{entity}",
            get({
                let state = state.clone();
                move |path| list_records(path, state)
            }),
        )
        .route(
            "/edit/{entity}/{id}",
            get({
                let state = state.clone();
                move |path| change_entity(path, state)
            }),
        )
        .route(
            "/api/{entity}",
            get({
                let state = state.clone();
                move |path| api_list_records(path, state)
            }),
        )
        .route(
            "/api/{entity}/{id}",
            get({
                let state = state.clone();
                move |path| api_get_record(path, state)
            }),
        )
        .route(
            "/api/{entity}",
            post({
                let state = state.clone();
                move |path, payload| api_upsert_record(path, payload, state)
            }),
        )
        .route(
            "/api/entities",
            get({
                let state = state.clone();
                move || api_list_entities(state)
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
