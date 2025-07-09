//! OData V4-compatible handlers for the data-issue-tracker backend
// This module mirrors the odata.rs API but returns OData V4-compliant responses.
use crate::{any_to_string, parse_entity_ref, AppState};
use axum::body::{self, Body};
use axum::extract;
use axum::http::HeaderMap;
use axum::http::{Request, Response, StatusCode};
use axum::response::IntoResponse;
use http::Request as HttpRequest;
use http_body_util::BodyExt;
use hyper::header;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;
use tracing::*;

// Enum for OData Result types
// Represents the possible results of an OData V4 request.
pub enum ODataV4Result {
    Empty,
    Single(serde_json::Value),
    Collection(Vec<serde_json::Value>),
    Error(String),
}

enum ParseState {
    Boundary,
    PartHeaders,
    RequestMethod,
    RequestHeaders,
    BinaryBody,
    LinesBody,
    Error,
}

pub async fn batch(
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let (parts, body) = req.into_parts();
    let mut body_stream = http_body_util::BodyStream::new(body);

    let content_type = parts
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if content_type.starts_with("multipart/mixed") {
        let mut responses: Vec<Response<Body>> = vec![];
        // Parse multipart/mixed batch
        let boundary = format!("--{}", content_type.split("boundary=").nth(1).unwrap_or(""));
        info!("boundary: {}", boundary);
        let mut parse_state = ParseState::Boundary;
        while let Some(x) = body_stream.frame().await {
            match &x {
                Ok(f) => {
                    if let Some(data) = f.data_ref() {
                        let mut context = &data[..];
                        let boundary = boundary.as_bytes();
                        let mut req_builder = HttpRequest::builder();
                        let mut body_start = None;
                        let mut body_size: usize = 0;
                        // states:
                        // 1. boundary + \r\n matches
                        // 2. part header lines -> \r\n\r\b -> request headers
                        // 3. request header lines -> \r\n\r\n -> body
                        // 4. body raw data using content-length
                        // 5. body lines
                        while context.len() > 0 {
                            // Process the current context based on the parse state
                            match parse_state {
                                ParseState::Boundary => {
                                    if context.starts_with(boundary) {
                                        context = &context[boundary.len()..];
                                        if context[0] == 13 && context[1] == 10 {
                                            // Check for \r\n after boundary
                                            context = &context[2..];
                                            parse_state = ParseState::PartHeaders;
                                            info!("boundary found");
                                        } else {
                                            context = &context[1..1];
                                        }
                                    }
                                }
                                ParseState::PartHeaders => {
                                    if let Some(idx) = context.iter().position(|x| *x == 13) {
                                        let header_line = str::from_utf8(&context[..idx]).unwrap();
                                        assert!(context[idx + 1] == 10); // Check for \r\n
                                        context = &context[idx + 2..];
                                        if header_line.is_empty() {
                                            parse_state = ParseState::RequestMethod;
                                        }
                                    }
                                }
                                ParseState::RequestMethod => {
                                    if let Some(idx) = context.iter().position(|x| *x == 13) {
                                        {
                                            let parts = str::from_utf8(&context[..idx])
                                                .unwrap()
                                                .split(' ')
                                                .map(|s| s.to_string())
                                                .collect::<Vec<_>>();
                                            info!("Request method line: {:?}", parts);
                                            let end = path.find("$batch").unwrap_or(path.len());
                                            let url = format!("{}{}", &path[..end], parts[1]);
                                            req_builder =
                                                req_builder.method(parts[0].as_str()).uri(url);
                                            context = &context[idx + 2..];
                                            parse_state = ParseState::RequestHeaders;
                                        }
                                    }
                                }
                                ParseState::RequestHeaders => {
                                    if let Some(idx) = context.iter().position(|x| *x == 13) {
                                        {
                                            let header_line =
                                                str::from_utf8(&context[..idx]).unwrap();
                                            info!("request header line: {}", header_line);
                                            //     req_builder = req_builder.header(k, v);
                                            assert!(context[idx + 1] == 10); // Check for \r\n
                                            context = &context[idx + 2..];
                                            if header_line.is_empty() {
                                                // End of part headers, switch to request headers
                                                parse_state = ParseState::LinesBody;
                                                info!("End of request headers");
                                            }
                                        }
                                    }
                                }
                                ParseState::LinesBody => {
                                    if let Some(idx) = context.iter().position(|x| *x == 10) {
                                        if body_start.is_none() {
                                            body_start = Some(context);
                                            body_size = 0;
                                        }
                                        if context[..idx].starts_with(boundary) {
                                            parse_state = ParseState::Boundary;
                                            let req = req_builder
                                                .body(axum::body::Body::from(
                                                    body::Bytes::copy_from_slice(
                                                        &body_start.unwrap()[..body_size],
                                                    ),
                                                ))
                                                .unwrap();
                                            // Call router
                                            let r = (*state.router()).clone();
                                            let response = r.oneshot(req).await;
                                            info!("Response: {:?}", response);
                                            responses.push(response.unwrap());
                                            req_builder = HttpRequest::builder(); // Reset for next part
                                            continue;
                                        }
                                        body_size += idx + 1;
                                        context = &context[idx + 1..];
                                    }
                                }
                                _ => todo!(),
                            }
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
        info!("done");

        let mut response_body = String::new();
        let resp_boundary = "batchresponse";
        for resp in responses {
            let resp = response_to_http_string(resp).await;
            let lines = vec![
                format!("--{}", resp_boundary),
                "Content-Type: application/http".to_string(),
                format!("Content-Length: {}", resp.len()),
                "Content-Transfer-Encoding: binary".to_string(),
                "".to_string(), // Empty line to separate headers from body
                resp,
            ];
            for s in lines {
                response_body.push_str(&s);
                response_body.push_str("\r\n");
            }
        }
        response_body.push_str(&format!("--{}--\r\n", resp_boundary));
        return Response::builder()
            .status(StatusCode::OK)
            .header(
                "Content-Type",
                format!("multipart/mixed; boundary={}", resp_boundary),
            )
            .header("OData-Version", "4.0")
            .body(axum::body::Body::from(response_body))
            .unwrap();
    }
    // Fallback: JSON batch
    // match body_bytes {
    //     Ok(data) => {
    //         let result = handle_batch_json(&data.to_bytes());
    //         let status = if result.get("error").is_some() {
    //             StatusCode::BAD_REQUEST
    //         } else {
    //             StatusCode::OK
    //         };
    //         odata_v4_json_response(status, None, result).into_response()
    //     }
    //     Err(e) => odata_v4_json_response(
    //         StatusCode::BAD_REQUEST,
    //         None,
    //         json!({"error": format!("Body read error: {}", e)}),
    //     )
    //     .into_response(),
    // }
    int_json_response(
        StatusCode::BAD_REQUEST,
        None,
        json!({"error": format!("Body read error")}),
    )
    .into_response()
}

// Helper to parse OData V4 entity path: Entity(key)
/// Parses a path like Entity(key)/SubEntity(key2)/... into a Vec of (entity, key) pairs.
pub fn parse_odata_entity_path(path: &str) -> Vec<(String, Option<serde_json::Value>)> {
    let mut result = Vec::new();
    for segment in path.split('/') {
        match (segment.find("("), segment.find(")")) {
            (Some(idx), Some(end)) => {
                if idx >= end || end != segment.len() - 1 {
                    return vec![]; //
                }
                let entity = &segment[..idx];
                let after_entity = &segment[idx + 1..end];

                if after_entity.len() == 0 || !segment.ends_with(")") {
                    return vec![];
                }
                result.push((
                    entity.to_string(),
                    match serde_yaml::from_str(after_entity) {
                        Ok(value) => Some(value),
                        Err(e) => {
                            error!(after_entity = after_entity, "{e}");
                            None
                        } // If parsing fails, return None
                    },
                ));
            }
            (None, None) => {
                result.push((segment.to_string(), None));
            }
            _ => {
                return vec![]; // Invalid segment, missing parentheses
            }
        }
    }
    result
}

// OData V4 JSON response
fn int_json_response<T: serde::Serialize>(
    status: StatusCode,
    etag: Option<String>,
    value: T,
) -> impl IntoResponse {
    let json_str = serde_json::to_string(&value).unwrap();
    let mut b = Response::builder()
        .status(status)
        .header(
            header::CONTENT_TYPE,
            "application/json;odata.metadata=minimal",
        )
        .header("OData-Version", "4.0");
    if let Some(etag) = etag {
        b = b.header("ETag", etag);
    }
    b.body(json_str).unwrap()
}

// #[instrument(skip(state, req))]
pub async fn entity(
    extract::Path(path): extract::Path<String>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    let mut result = ODataV4Result::Empty;
    let path_parts = path.split("/").map(any_to_string).collect::<Vec<_>>();

    info!(path = ?path, "API: OData V4 entity request");
    let headers = req.headers();
    let if_none_match = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok());
    for part in &path_parts {
        info!(part = part, "Path part");
        if part == "$count" {
            if let ODataV4Result::Collection(col) = &result {
                let count = col.len();
                info!("Count of records: {}", count);
                return int_json_response(StatusCode::OK, None, json!(count)).into_response();
            } else {
                return int_json_response(
                    StatusCode::BAD_REQUEST,
                    None,
                    json!({"error": "count can only be used on collections"}),
                )
                .into_response();
            }
        } else {
            let pair = parse_entity_ref(part);
            if let Ok((entity, Some(id))) = pair {
                result = api_get_record_v4(&entity, &id, state.clone()).await;
            } else {
                match result {
                    ODataV4Result::Empty => {
                        let _model = state.get_entity_model(part);
                        let x = state.load_entity_refs(part);
                        let lst = state.get_all_records(x);
                        result = ODataV4Result::Collection(lst);
                    }
                    ODataV4Result::Single(ref record) => {
                        let obj = record.as_object().unwrap();
                        let r = obj
                            .get(part)
                            .and_then(|v| v.as_array().and_then(|w| Some(w.clone())))
                            .unwrap_or_default();
                        result = ODataV4Result::Collection(r);
                    }
                    ODataV4Result::Collection(_) => {}
                    ODataV4Result::Error(_) => {
                        break;
                    }
                }
            }
        }
    }
    match result {
        ODataV4Result::Empty => {
            int_json_response(StatusCode::NOT_FOUND, None, json!({})).into_response()
        }
        ODataV4Result::Single(ref record) => {
            let etag = record.get("@odata.etag").and_then(|e| e.as_str());
            if let (Some(client_etag), Some(server_etag)) = (if_none_match, etag) {
                if client_etag == server_etag {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .body(String::new())
                        .unwrap()
                        .into_response();
                }
            }
            int_json_response(StatusCode::OK, etag.map(|x| x.to_string()), record).into_response()
        }
        ODataV4Result::Collection(records) => {
            let mut total_etag = None;
            for record in records.iter() {
                let etag = record.get("@odata.etag").and_then(|e| e.as_str());
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
            let response_json = json!({"value": records});
            int_json_response(
                StatusCode::OK,
                total_etag.map(|s| s.to_string()),
                response_json,
            )
            .into_response()
        }
        ODataV4Result::Error(e) => {
            int_json_response(StatusCode::INTERNAL_SERVER_ERROR, None, json!({"error": e}))
                .into_response()
        }
    }
}

pub async fn entity_patch(
    extract::Path(path): extract::Path<Vec<String>>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    match int_odatav4_entity_patch(path, state, req).await {
        Ok(id) => int_json_response(StatusCode::OK, None, json!({"id": id})).into_response(),
        Err(e) => int_json_response(StatusCode::UNPROCESSABLE_ENTITY, None, json!({"error": e}))
            .into_response(),
    }
}

pub async fn entity_post(
    extract::Path(path): extract::Path<Vec<String>>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    match int_odatav4_entity_post(path, state, req).await {
        Ok(id) => int_json_response(StatusCode::OK, None, json!({"id": id})).into_response(),
        Err(e) => int_json_response(StatusCode::UNPROCESSABLE_ENTITY, None, json!({"error": e}))
            .into_response(),
    }
}

pub async fn metadata(
    extract::State(state): extract::State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    info!("metadata handler v4");
    let accept = headers
        .get("accept")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let wants_xml = !accept.contains("application/json") || accept.contains("text/json");
    if wants_xml {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str(
            r#"<edmx:Edmx Version="4.0" xmlns:edmx="http://docs.oasis-open.org/odata/ns/edmx">"#,
        );
        xml.push_str(r#"<edmx:DataServices><Schema Namespace="Service" xmlns="http://docs.oasis-open.org/odata/ns/edm">"#);
        for (entity_name, model) in state.entities().iter() {
            xml.push_str(&format!(r#"<EntityType Name="{}">"#, entity_name));
            xml.push_str(r#"<Key><PropertyRef Name="id"/></Key>"#);
            for attr in model.attributes() {
                xml.push_str(&format!(
                    r#"<Property Name="{}" Type="Edm.String" Nullable="{}"/>"#,
                    attr.name, attr.nullable
                ));
            }

            for rel in model.relations() {
                // <NavigationProperty Name="Orders" Type="Collection(NorthwindModel.Order)" Partner="Customer" />
                // <NavigationProperty Name="CustomerDemographics" Type="Collection(NorthwindModel.CustomerDemographic)" Partner="Customers" />
                match rel.cardinality.as_str() {
                    "one-to-many" => {
                        xml.push_str(&format!(
                                    r#"<NavigationProperty Name="{}" Type="Collcetion(Service.{})" Partner="{}"/>"#,
                                    rel.name, rel.type_name, rel.target
                                ));
                    }
                    "many-to-one" => {
                        xml.push_str(&format!(
                            r#"<NavigationProperty Name="{}" Type="{}" Partner="{}"/>"#,
                            rel.name, rel.type_name, rel.target
                        ));
                    }
                    "many-to-many" => {
                        xml.push_str(&format!(
                                    r#"<NavigationProperty Name="{}" Type="Collcetion(Service.{})" Partner="{}"/>"#,
                                    rel.name, rel.type_name, rel.target
                                ));
                    }
                    _ => todo!("Unsupported relation type: {}", rel.cardinality),
                }
            }
            xml.push_str(r#"</EntityType>"#);
        }
        xml.push_str(r#"<EntityContainer Name="Container">"#);
        for (entity_name, _model) in state.entities().iter() {
            xml.push_str(&format!(
                r#"<EntitySet Name="{}" EntityType="Service.{}"/>"#,
                entity_name, entity_name
            ));
        }
        xml.push_str(r#"</EntityContainer>"#);
        xml.push_str(r#"</Schema></edmx:DataServices></edmx:Edmx>"#);
        return Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(xml)
            .unwrap();
    }
    let mut json = serde_json::Map::new();
    json.insert("@odata.context".to_string(), json!("$metadata"));
    let entity_sets: Vec<_> = state
        .entities()
        .iter()
        .map(|(name, model)| {
            json!({
                "name": name,
                "entityType": model.service_name,
                "title": model.title_attribute,
                "attributes": model.attributes,
                "relations": model.relations
            })
        })
        .collect();
    json.insert("EntitySets".to_string(), json!(entity_sets));
    let json_str = serde_json::to_string(&json).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json;odata.metadata=minimal")
        .body(json_str)
        .unwrap()
}

pub async fn response_to_http_string<T>(resp: Response<T>) -> String
where
    T: axum::body::HttpBody + Unpin,
    T::Data: std::ops::Deref<Target = [u8]>,
    T::Error: std::fmt::Debug,
{
    let (parts, body) = resp.into_parts();
    let mut result = String::new();
    use std::fmt::Write;
    // Statuszeile
    write!(
        &mut result,
        "HTTP/1.1 {} {}\r\n",
        parts.status.as_u16(),
        parts.status.canonical_reason().unwrap_or("")
    )
    .unwrap();
    // Header
    for (k, v) in parts.headers.iter() {
        write!(&mut result, "{}: {}\r\n", k, v.to_str().unwrap_or("")).unwrap();
    }
    result.push_str("\r\n");
    // Body
    let body_bytes = body.collect().await.unwrap().to_bytes();
    result.push_str(&String::from_utf8_lossy(&body_bytes));
    result
}

// === Private/Helper methods (alphabetically) ===

async fn api_get_record_v4(
    entity: &str,
    id: &serde_json::Value,
    state: Arc<AppState>,
) -> ODataV4Result {
    info!(entity = %entity, id = %id, "API: Get record V4");
    match state.get_record_ext("/apiv4", &entity, &id) {
        Ok(r) => match r {
            crate::EntityResult::Single {
                entity,
                id,
                label,
                etag,
                value,
            } => {
                info!(?value, "Found single record");
                let mut record = value.clone();
                record["@odata.context"] = json!(format!("/apiv4/$metadata#Service.{}", entity));
                record["@odata.id"] = json!(format!("/apiv4/Service.{}/{}", entity, id));
                record["@odata.etag"] = json!(etag);
                record["@odata.type"] = json!(format!("Service.{entity}"));
                if !label.is_empty() {
                    record["title"] = json!(label);
                }
                record["id"] = json!(id);
                return ODataV4Result::Single(record);
            }
            crate::EntityResult::Collection {
                entity,
                etag,
                count,
                next_token,
                value,
            } => todo!(),
        },
        Err(e) => todo!(),
    }
}

fn handle_batch_json(data: &[u8]) -> serde_json::Value {
    use serde_json::Value;
    let mut responses = Vec::new();
    let json_val: Result<Value, _> = serde_json::from_slice(data);
    if let Ok(Value::Array(requests)) = json_val {
        for req in requests {
            let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("GET");
            let url = req.get("url").and_then(|u| u.as_str()).unwrap_or("");
            let body = req.get("body").cloned().unwrap_or(Value::Null);
            responses.push(json!({
                "method": method,
                "url": url,
                "body": body
            }));
        }
        json!({"responses": responses})
    } else {
        json!({"error": "Batch body must be a JSON array of requests"})
    }
}

async fn int_odatav4_entity_patch(
    path: Vec<String>,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> Result<String, String> {
    info!(path=%path.len(), "patching");
    let pairs = parse_odata_entity_path(path[0].as_str());
    if let Some((entity, Some(id))) = pairs.get(0) {
        let json_val = json_from_body(req).await?;
        let mut record = state.get_record("/apiv4", entity, id);
        info!(?json_val, "record from {id}");
        for (key, val) in json_val.as_object().unwrap().iter() {
            record[key] = val.clone();
        }
        record.as_object_mut().unwrap().remove("@odata.etag");
        state.save_record(entity, record)
    } else {
        Err(format!("Invalid OData path: {:?}", path))
    }
}

async fn int_odatav4_entity_post(
    path: Vec<String>,
    state: Arc<AppState>,
    req: Request<axum::body::Body>,
) -> Result<String, String> {
    let pairs = parse_odata_entity_path(path[0].as_str());
    if let Some((_entity, Some(_id))) = pairs.get(0) {
        let json_val = json_from_body(req).await?;
        let mut record = state.get_record("/apiv4", _entity, _id);
        for (key, val) in json_val.as_object().unwrap().iter() {
            record[key] = val.clone();
        }
        record.as_object_mut().unwrap().remove("@odata.etag");
        state.save_record(_entity, record)
    } else {
        let json_val = json_from_body(req).await?;
        let _entity = &path[0];
        state.save_record(_entity, json_val)
    }
}

async fn json_from_body(req: Request<axum::body::Body>) -> Result<serde_json::Value, String> {
    let (_parts, mut body) = req.into_parts();
    let body_bytes = BodyExt::collect(&mut body).await;
    match body_bytes {
        Ok(data) => {
            let b = data.to_bytes();
            match serde_json::from_slice(&b) {
                Ok(val) => Ok(val),
                Err(e) => Err(format!(
                    "Invalid JSON: {} <{}>",
                    e,
                    str::from_utf8(&b).unwrap()
                )),
            }
        }
        Err(e) => Err(format!("Body read error: {}", e)),
    }
}
