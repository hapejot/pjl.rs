use axum::http::header::IF_NONE_MATCH;
use axum::http::HeaderMap;
use axum::response::Response as AxumResponse;
// Use axum's Json extractor for simplicity
use axum::extract::Json;
use axum::http::{header, Request, Response, StatusCode};
use axum::response::Json as AxumJson;
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use data_issue_tracker::{any_to_string, load_entity_models, AppState};
use data_issue_tracker::{EntityMap, EntityModel, RelationOptions, SelectionEntry};
use http_body_util::BodyExt;
use serde_json::json;
use serde_json::Value; // for collect().await
use serde_yaml;
use sha2::{Digest, Sha256};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{collections::HashMap, fs, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, instrument, trace};
use uuid::Uuid; // for write! macro

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

// --- REST API HANDLERS ---

#[instrument(skip(state, req))]
async fn api_odata_entity(
    path: Vec<String>,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    info!(path = ?path, "API: OData entity request");
    let last = path.last().unwrap();
    info!(last = %last, "Last segment of path");
    if last == "$count" {
        // Handle $count for entity set
        let entity = path[0].as_str();
        let data_dir = format!("data/{}", entity);
        let count = fs::read_dir(&data_dir)
            .map(|entries| entries.count())
            .unwrap_or(0);
        return odata_json_response(StatusCode::OK, json!(count)).into_response();
    }
    if let Some((entity, id)) = parse_odata_entity_path(&path[0].as_str()) {
        api_get_record(Path((entity, id)), state)
            .await
            .into_response()
    } else {
        api_list_records(path[0].as_str(), state, req).await
    }
}

// Helper to parse OData V2 entity path: Entity('id')
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

fn odata_json_response<T: serde::Serialize>(status: StatusCode, value: T) -> impl IntoResponse {
    use base64::{engine::general_purpose, Engine as _};
    use sha2::{Digest, Sha256};
    let json_str = serde_json::to_string(&value).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json_str.as_bytes());
    let hash = hasher.finalize();
    let etag = format!("\"{}\"", general_purpose::STANDARD.encode(hash));
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .header("DataServiceVersion", "2.0")
        .header("ETag", etag)
        .body(json_str)
        .unwrap()
}

async fn int_list_records(
    entity: &str,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> Result<(StatusCode, String, String), String> {
    info!(entity = %entity, "API: Listing records");
    let _model = state.get_entity_model(entity)?;
    let mut records = Vec::new();
    let entries = fs::read_dir(&_model.data_directory).map_err(|x| x.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = fs::read_to_string(&path).map_err(any_to_string)?;
            let record =
                serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(any_to_string)?;
            let mut json_record = serde_json::to_value(record).unwrap_or_default();
            let id = json_record.get("id").and_then(|v| v.as_str()).unwrap_or("");
            // Compute ETag for this record
            let record_str = serde_json::to_string(&json_record).unwrap_or_default();
            let mut hasher = Sha256::new();
            hasher.update(record_str.as_bytes());
            let hash = hasher.finalize();
            let etag = format!("\"{}\"", general_purpose::STANDARD.encode(hash));
            let uri = format!("/api/{}('{}')", entity, id);
            if let Some(obj) = json_record.as_object_mut() {
                obj.insert(
                    "__metadata".to_string(),
                    json!({
                        "type": _model.service_name.clone(),
                        "uri": uri,
                        "etag": etag
                    }),
                );
            }
            records.push(json_record);
        }
    }
    let response_json = json!({"d": {"results":records}});
    // Compute ETag for the whole response
    let json_str = serde_json::to_string(&response_json).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json_str.as_bytes());
    let hash = hasher.finalize();
    let etag = format!("\"{}\"", general_purpose::STANDARD.encode(hash));
    // Check If-None-Match
    let headers: &HeaderMap = req.headers();
    if let Some(if_none_match) = headers.get(IF_NONE_MATCH) {
        if if_none_match.to_str().ok() == Some(&etag) {
            return Ok((StatusCode::NOT_MODIFIED, etag, String::new()));
        }
    }
    Ok((StatusCode::OK, etag, json_str))
}

#[instrument(skip(state, req))]
async fn api_list_records(
    entity: &str,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> Response<axum::body::Body> {
match int_list_records(entity, state, req).await {
        Ok((status, etag, json_str)) => {
            if status == StatusCode::NOT_MODIFIED {
                Response::builder()
                    .status(status)
                    .header("ETag", etag)
                    .body(axum::body::Body::empty())
                    .unwrap()
            } else {
                Response::builder()
                    .status(status)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("DataServiceVersion", "2.0")
                    .header("ETag", etag)
                    .body(axum::body::Body::from(json_str))
                    .unwrap()
            }
        }
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(json!({"error": e}).to_string()))
            .unwrap(),
    }

}

#[instrument(skip(state))]
async fn api_get_record(
    Path((entity, id)): Path<(String, String)>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    info!(entity = %entity, id = %id, "API: Get record");
    if id == "$count" {
        let data_dir = format!("data/{}", entity);
        let count = fs::read_dir(&data_dir)
            .map(|entries| entries.count())
            .unwrap_or(0);
        return odata_json_response(StatusCode::OK, json!(count));
    }
    let mut record = state.get_record(&entity, &id);
    if record.is_null() {
        odata_json_response(StatusCode::NOT_FOUND, json!({"error": "Record not found"}))
    } else {
        // Compute ETag for this record
        let record_str = serde_json::to_string(&record).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(record_str.as_bytes());
        let hash = hasher.finalize();
        let etag = format!("\"{}\"", general_purpose::STANDARD.encode(hash));
        let uri = format!("/api/{}('{}')", entity, id);
        if let Some(obj) = record.as_object_mut() {
            obj.insert(
                "__metadata".to_string(),
                json!({
                    "type": format!("Service.{}", entity),
                    "uri": uri,
                    "etag": etag
                }),
            );
        }
        odata_json_response(StatusCode::OK, json!({"d": record}))
    }
}

#[instrument(skip(state))]
async fn api_get_entity_model(
    Path(entity): Path<String>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    match state.get_entity_model(&entity) {
        Ok(model) => AxumJson(model).into_response(),
        Err(e) => AxumJson(json!({"error": e})).into_response(),
    }
}

#[instrument(skip(state))]
async fn odata_metadata(state: Arc<AppState>) -> impl IntoResponse {
    let mut xml = String::new();
    // OData EDMX header
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(
        r#"<edmx:Edmx Version="1.0" xmlns:edmx="http://schemas.microsoft.com/ado/2007/06/edmx">"#,
    );
    xml.push_str(r#"<edmx:DataServices m:DataServiceVersion="2.0" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata">"#);
    xml.push_str(
        r#"<Schema Namespace="Service" xmlns="http://schemas.microsoft.com/ado/2008/09/edm">"#,
    );

    // EntityTypes
    for (entity_name, model) in state.entities().iter() {
        xml.push_str(&format!(r#"<EntityType Name="{}">"#, entity_name));
        // Assume "id" is the key
        xml.push_str(r#"<Key><PropertyRef Name="id"/></Key>"#);
        if let Some(attrs) = &model.attributes {
            for attr in attrs {
                // All attributes as string for simplicity
                xml.push_str(&format!(
                    r#"<Property Name="{}" Type="Edm.String" Nullable="true"/>"#,
                    attr.name
                ));
            }
        }
        xml.push_str(r#"</EntityType>"#);
    }

    // EntitySets
    xml.push_str(r#"<EntityContainer Name="Container"  m:IsDefaultEntityContainer="true">"#);
    for (entity_name, _) in state.entities().iter() {
        xml.push_str(&format!(
            r#"<EntitySet Name="{}" EntityType="Service.{}"/>"#,
            entity_name, entity_name
        ));
    }
    xml.push_str(r#"</EntityContainer>"#);

    xml.push_str(r#"</Schema></edmx:DataServices></edmx:Edmx>"#);

    // Compute ETag from the XML body
    let mut hasher = Sha256::new();
    hasher.update(xml.as_bytes());
    let hash = hasher.finalize();
    let etag = format!("\"{}\"", general_purpose::STANDARD.encode(hash));

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .header("ETag", etag)
        .body(xml)
        .unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!(
        "current working directory: {}",
        std::env::current_dir().unwrap().display()
    );

    let state = AppState::new();
    let app = Router::new()
        .route(
            "/api/$metadata",
            get({
                let state = state.clone();
                move || odata_metadata(state)
            }),
        )
        .route(
            "/api/{*path}",
            get({
                let state = state.clone();
                move |Path(path): Path<String>, req| {
                    api_odata_entity(
                        path.split("/").map(any_to_string).collect(),
                        state.clone(),
                        req,
                    )
                }
            }),
        )
        .route(
            "/api/{*path}",
            post({
                let state = state.clone();
                move |Path(path): Path<String>, req| {
                    api_odata_entity_post(
                        path.split("/").map(any_to_string).collect(),
                        state.clone(),
                        req,
                    )
                }
            }),
        )
        // Serve static files
        .nest_service("/static", ServeDir::new("webapp"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn json_from_body(req: Request<axum::body::Body>) -> Result<serde_json::Value, String> {
    let (_parts, mut body) = req.into_parts();
    let body_bytes = BodyExt::collect(&mut body).await;
    match body_bytes {
        Ok(data) => match serde_json::from_slice(&data.to_bytes()) {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("Invalid JSON: {}", e)),
        },
        Err(e) => Err(format!("Body read error: {}", e)),
    }
}

async fn int_odata_entity_post(
    path: Vec<String>,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> Result<String, String> {
    if let Some((_entity, _id)) = parse_odata_entity_path(path[0].as_str()) {
        let json_val = json_from_body(req).await?;
        let mut record = state.get_record(&_entity, &_id);
        for (key, val) in json_val.as_object().unwrap().iter() {
            record[key] = val.clone();
        }
        record.as_object_mut().unwrap().remove("__metadata");
        state.save_record(&_entity, record)
    } else {
        let json_val = json_from_body(req).await?;
        let _entity = &path[0];
        state.save_record(_entity, json_val)
        // Call api_save_record
    }
}

// Handler for POST requests to OData entity URIs
async fn api_odata_entity_post(
    path: Vec<String>,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    match int_odata_entity_post(path, state, req).await {
        Ok(id) => odata_json_response(StatusCode::OK, json!({"id": id})).into_response(),
        Err(e) => odata_json_response(StatusCode::UNPROCESSABLE_ENTITY, json!({"error": e}))
            .into_response(),
    }
}
