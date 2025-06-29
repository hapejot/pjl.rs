//! OData-related handlers and helpers for the data-issue-tracker backend
use crate::{any_to_string, AppState};
use hyper::header;
use axum::extract;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use serde_json::json;
use std::fs;
use std::sync::Arc;
use tracing::*;


enum ODataResult {
    Empty, // initia state
    Single(serde_json::Value),
    Collection(Vec<serde_json::Value>),
    Error(String),
}



// Handler for POST requests to OData entity URIs
pub async fn entity_post(
    extract::Path(path): extract::Path<Vec<String>>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    match int_odata_entity_post(path, state, req).await {
        Ok(id) => odata_json_response(StatusCode::OK, None, json!({"id": id})).into_response(),
        Err(e) => odata_json_response(StatusCode::UNPROCESSABLE_ENTITY, None, json!({"error": e}))
            .into_response(),
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

fn odata_json_response<T: serde::Serialize>(
    status: StatusCode,
    etag: Option<String>,
    value: T,
) -> impl IntoResponse {
    let json_str = serde_json::to_string(&value).unwrap();
    let mut b = Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .header("DataServiceVersion", "2.0");
    if let Some(etag) = etag {
        b = b.header("ETag", etag);
    }
    b.body(json_str).unwrap()
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

// #[instrument(skip(state))]
pub async fn metadata(extract::State(state): extract::State<Arc<AppState>>) -> impl IntoResponse {
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
            for attr in model.attributes() {
                // All attributes as string for simplicity
                xml.push_str(&format!(
                    r#"<Property Name="{}" Type="Edm.String" Nullable="true"/>"#,
                    attr.name
                ));
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

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(xml)
        .unwrap()
}

// #[instrument(skip(state))]
pub async fn batch(extract::State(state): extract::State<Arc<AppState>>) -> impl IntoResponse {
    let _ = state;
    info!("batch handler");
    Response::builder().body(json!({}).to_string()).unwrap()
}

// #[instrument(skip(state, req))]
pub async fn entity(
    extract::Path(path): extract::Path<String>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    let mut result = ODataResult::Empty;
    let path_parts = path.split("/").map(any_to_string).collect::<Vec<_>>();

    info!(path = ?path, "API: OData entity request");
    let headers = req.headers();
    let if_none_match = headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok());
    for part in &path_parts {
        info!(part = part, "Path part");
        if part == "$count" {
            // Handle $count for entity set
            if let ODataResult::Collection(col) = &result {
                let count = col.len();
                info!("Count of records: {}", count);
                return odata_json_response(StatusCode::OK, None, json!(count)).into_response();
            } else {
                panic!("$count can only be used on collections");
            }
        } else if let Some((entity, id)) = parse_odata_entity_path(part) {
            result = api_get_record(&entity, &id, state.clone()).await;
        } else {
            match result {
                ODataResult::Empty => {
                    let _model = state.get_entity_model(part);
                    // Initial state, list records for the entity
                    let x = state.load_entity_refs(part);
                    let lst = state.get_all_records(x);
                    result = ODataResult::Collection(lst);
                }

                ODataResult::Single(record) => {
                    let obj = record.as_object().unwrap();
                    let r = obj
                        .get(part)
                        .and_then(|v| v.as_array().and_then(|w| Some(w.clone())))
                        .unwrap_or_default();

                    result = ODataResult::Collection(r);
                }

                ODataResult::Collection(_) => {
                    // this is an error
                }

                ODataResult::Error(_) => {
                    break;
                }
            }
            let _x = api_list_records(part, state.clone()).await;
        }
    }

    match result {
        ODataResult::Empty => {
            odata_json_response(StatusCode::NOT_FOUND, None, json!({})).into_response()
        }
        ODataResult::Single(ref record) => {
            // Check ETag for If-None-Match
            let etag = record
                .get("__metadata")
                .and_then(|m| m.get("etag"))
                .and_then(|e| e.as_str());
            if let (Some(client_etag), Some(server_etag)) = (if_none_match, etag) {
                if client_etag == server_etag {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .body(String::new())
                        .unwrap()
                        .into_response();
                }
            }
            odata_json_response(
                StatusCode::OK,
                etag.map(|x| x.to_string()),
                json!({"d": record}),
            )
            .into_response()
        }
        ODataResult::Collection(records) => {
            let mut total_etag = None;
            for record in records.iter() {
                let etag = record
                    .get("__metadata")
                    .and_then(|m| m.get("etag"))
                    .and_then(|e| e.as_str());
                if let (Some(etag1), Some(etag2)) = (total_etag, etag) {
                    if etag1 < etag2 {
                        total_etag = Some(etag2);
                    }
                } else {
                    total_etag = etag;
                }
            }
            info!(?total_etag, "Computed total ETag for collection");
            if let (Some(client_etag), Some(server_etag)) = (if_none_match, total_etag) {
                if client_etag == server_etag {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .body(String::new())
                        .unwrap()
                        .into_response();
                }
            }
            let response_json = json!({"d": {"results": records}});
            odata_json_response(StatusCode::OK, total_etag.map(any_to_string), response_json)
                .into_response()
        }
        ODataResult::Error(e) => {
            odata_json_response(StatusCode::INTERNAL_SERVER_ERROR, None, json!({"error": e}))
                .into_response()
        }
    }
}

// #[instrument(skip(state))]
async fn api_list_records(
    entity: &str,
    state: Arc<AppState>,
    // req: Request<axum::body::Body>,
) -> ODataResult {
    match int_list_records(entity, state).await {
        Ok((_, _, _)) => ODataResult::Collection(vec![]),
        Err(e) => ODataResult::Error(e.to_string()),
    }
}

async fn int_list_records(
    entity: &str,
    state: Arc<AppState>, // req: Request<axum::body::Body>,
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
            let _record_str = serde_json::to_string(&json_record).unwrap_or_default();
            let uri = format!("/api/{}('{}')", entity, id);
            if let Some(obj) = json_record.as_object_mut() {
                obj.insert(
                    "__metadata".to_string(),
                    json!({
                        "type": _model.service_name.clone(),
                        "uri": uri,
                        "etag": "...."
                    }),
                );
            }
            records.push(json_record);
        }
    }
    let response_json = json!({"d": {"results":records}});
    // Compute ETag for the whole response
    let json_str = serde_json::to_string(&response_json).unwrap();
    Ok((StatusCode::OK, "etag".into(), json_str))
}

// #[instrument(skip(state))]
async fn api_get_record(entity: &str, id: &str, state: Arc<AppState>) -> ODataResult {
    info!(entity = %entity, id = %id, "API: Get record");
    let record = state.get_record(&entity, &id);
    if record.is_null() {
        ODataResult::Empty
    } else {
        ODataResult::Single(record)
    }
}
