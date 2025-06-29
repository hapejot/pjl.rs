//! OData V4-compatible handlers for the data-issue-tracker backend
// This module mirrors the odata.rs API but returns OData V4-compliant responses.
use crate::{any_to_string, AppState};
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

// Enum for OData V4 results (mirrors odata.rs)
pub enum ODataV4Result {
    Empty,
    Single(serde_json::Value),
    Collection(Vec<serde_json::Value>),
    Error(String),
}

// Helper to parse OData V4 entity path: Entity(key)
pub fn parse_odata_entity_path(path: &str) -> Option<(String, String)> {
    if let Some(idx) = path.find('(') {
        let entity = &path[..idx];
        let rest = &path[idx..];
        if rest.starts_with('(') && rest.ends_with(')') && rest.len() > 2 {
            let id = &rest[1..rest.len() - 1];
            return Some((entity.to_string(), id.to_string()));
        }
    }
    None
}

// OData V4 JSON response (uses @odata.* annotations)
pub fn odata_v4_json_response<T: serde::Serialize>(
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

pub async fn entity_post(
    extract::Path(path): extract::Path<Vec<String>>,
    extract::State(state): extract::State<Arc<AppState>>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    match int_odatav4_entity_post(path, state, req).await {
        Ok(id) => odata_v4_json_response(StatusCode::OK, None, json!({"id": id})).into_response(),
        Err(e) => {
            odata_v4_json_response(StatusCode::UNPROCESSABLE_ENTITY, None, json!({"error": e}))
                .into_response()
        }
    }
}

async fn int_odatav4_entity_post(
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
        record.as_object_mut().unwrap().remove("@odata.etag");
        state.save_record(&_entity, record)
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
        Ok(data) => match serde_json::from_slice(&data.to_bytes()) {
            Ok(val) => Ok(val),
            Err(e) => Err(format!("Invalid JSON: {}", e)),
        },
        Err(e) => Err(format!("Body read error: {}", e)),
    }
}

// #[instrument(skip(state, headers))]
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
                        r#"<Property Name="{}" Type="Edm.String" Nullable="true"/>"#,
                        attr.name
                    ));
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
// #[debug_handler]
// #[instrument(skip(state, headers, req))]
pub async fn batch(
    extract::State(state): extract::State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    use http_body_util::BodyExt;
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let path = req.uri().path().to_string();
    let (_parts, mut body) = req.into_parts();
    let body_bytes = BodyExt::collect(&mut body).await;
    if content_type.starts_with("multipart/mixed") {
        // Parse multipart/mixed batch
        let boundary = format!("--{}", content_type.split("boundary=").nth(1).unwrap_or(""));
        info!("boundary: {}", boundary);
        if let Ok(data) = body_bytes {
            let _responses: Vec<String> = Vec::new();
            let bytes = data.to_bytes();
            let bytes = String::from_utf8_lossy(bytes.iter().as_slice());
            let mut x = bytes.split("\r\n").map(|x| x.to_string());

            while let Some(line) = x.next() {
                if line == boundary {
                    // parse mime header until empty line
                    while let Some(line) = x.next() {
                        if line.is_empty() {
                            break; // End of headers
                        }
                        // Here you can parse the header line if needed
                        info!("Header line: {}", line);
                    }

                    if let Some(line) = x.next() {
                        let parts = line.split(" ").collect::<Vec<_>>();
                        info!("{:?}", parts);
                        let end = path.find("$batch").unwrap_or(path.len());
                        let url = format!("{}{}", &path[..end], parts[1]);
                        info!("URL: {}", url);
                        let req_builder = HttpRequest::builder().method(parts[0]).uri(url);
                        // for (k, v) in headers.iter() {
                        //     req_builder = req_builder.header(k, v);
                        // }
                        let req = req_builder
                            .body(axum::body::Body::from(format!("")))
                            .unwrap();
                        // Call router
                        let r = (*state.router()).clone();
                        let response = r.oneshot(req).await;
                        info!("Response: {:?}", response);
                    }
                }
            }
        }

        // for part in body_str.split(&format!("--{}", boundary)) {
        //     let part = part.trim().to_string();
        //     if part.is_empty() || part == "--" {
        //         continue;
        //     }
        //     // Find the HTTP segment (e.g., GET /apiv4/Entity HTTP/1.1 ...)
        //     let http_start = part.find("GET ").or_else(|| part.find("POST "));
        //     if let Some(start) = http_start {
        //         let http_segment = part[start..].to_string();
        //         // Parse request line
        //         let mut lines = http_segment
        //             .lines()
        //             .map(|x| x.to_string())
        //             .collect::<Vec<_>>()
        //             .iter();
        //         let request_line = lines.next().unwrap_or(&String::new());
        //         let mut method = "GET";
        //         let mut url = "";
        //         if let Some((m, rest)) = request_line.split_once(' ') {
        //             method = m;
        //             url = rest.split_whitespace().next().unwrap_or("");
        //         }
        //         // Collect headers
        //         let mut headers = HttpHeaderMap::new();
        //         for line in &mut lines {
        //             let line = line.trim();
        //             if line.is_empty() {
        //                 break;
        //             }
        //             if let Some((k, v)) = line.split_once(":") {
        //                 if let Ok(header_value) = v.trim().parse() {
        //                     headers.insert(k.trim(), header_value);
        //                 }
        //             }
        //         }
        //         // Collect body
        //         let body: String = lines.as_slice().join("\r\n");
        //         // Build Axum request
        //         let mut req_builder = HttpRequest::builder().method(method).uri(url);
        //         for (k, v) in headers.iter() {
        //             req_builder = req_builder.header(k, v);
        //         }
        //         let req = req_builder
        //             .body(axum::body::Body::from(body.clone()))
        //             .unwrap();
        //         // Call router
        //         let response = router.clone().oneshot(req).await.unwrap();
        //         // Serialize response as HTTP/1.1
        //         let (parts, body) = response.into_parts();
        //         let status = parts.status.as_str();
        //         let mut resp_headers = String::new();
        //         for (k, v) in parts.headers.iter() {
        //             resp_headers.push_str(&format!("{}: {}\r\n", k, v.to_str().unwrap_or("")));
        //         }
        //         let body_bytes = body.collect().await.unwrap().to_bytes();
        //         let resp_body = String::from_utf8_lossy(&body_bytes);
        //         let http_resp = format!("HTTP/1.1 {}\r\n{}\r\n{}", status, resp_headers, resp_body);
        //         responses.push(http_resp);
        //     }
        // }
        // Compose a multipart/mixed response
        let mut response_body = String::new();
        let resp_boundary = "batchresponse";
        // for resp in responses {
        //     response_body.push_str(&format!("--{}\r\nContent-Type: application/http\r\nContent-Transfer-Encoding: binary\r\n\r\n{}\r\n", resp_boundary, resp));
        // }
        response_body.push_str(&format!("--{}--\r\n", resp_boundary));
        return Response::builder()
            .status(StatusCode::OK)
            .header(
                "Content-Type",
                format!("multipart/mixed; boundary={}", resp_boundary),
            )
            .body(axum::body::Body::from(response_body))
            .unwrap();
    }
    // Fallback: JSON batch
    match body_bytes {
        Ok(data) => {
            let result = handle_batch_json(&data.to_bytes());
            let status = if result.get("error").is_some() {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::OK
            };
            odata_v4_json_response(status, None, result).into_response()
        }
        Err(e) => odata_v4_json_response(
            StatusCode::BAD_REQUEST,
            None,
            json!({"error": format!("Body read error: {}", e)}),
        )
        .into_response(),
    }
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
                return odata_v4_json_response(
                    StatusCode::OK,
                    None,
                    json!({"@odata.count": count}),
                )
                .into_response();
            } else {
                panic!("$count can only be used on collections");
            }
        } else if let Some((entity, id)) = parse_odata_entity_path(part) {
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
    match result {
        ODataV4Result::Empty => {
            odata_v4_json_response(StatusCode::NOT_FOUND, None, json!({})).into_response()
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
            odata_v4_json_response(StatusCode::OK, etag.map(|x| x.to_string()), record)
                .into_response()
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
            odata_v4_json_response(
                StatusCode::OK,
                total_etag.map(|s| s.to_string()),
                response_json,
            )
            .into_response()
        }
        ODataV4Result::Error(e) => {
            odata_v4_json_response(StatusCode::INTERNAL_SERVER_ERROR, None, json!({"error": e}))
                .into_response()
        }
    }
}

async fn api_get_record_v4(entity: &str, id: &str, state: Arc<AppState>) -> ODataV4Result {
    info!(entity = %entity, id = %id, "API: Get record V4");
    let record = state.get_record(&entity, &id);
    if record.is_null() {
        ODataV4Result::Empty
    } else {
        ODataV4Result::Single(record)
    }
}
