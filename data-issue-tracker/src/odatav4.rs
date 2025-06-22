//! OData V4-compatible handlers for the data-issue-tracker backend
// This module mirrors the odata.rs API but returns OData V4-compliant responses.
use axum::http::{header, Request, Response, StatusCode};
use axum::response::IntoResponse;
use axum::extract::{Path, State};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, instrument};
use crate::{any_to_string, AppState};
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};
use std::fs;

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
        .header(header::CONTENT_TYPE, "application/json;odata.metadata=minimal")
        .header("OData-Version", "4.0");
    if let Some(etag) = etag {
        b = b.header("ETag", etag);
    }
    b.body(json_str).unwrap()
}

// ... Implement api_odatav4_entity, api_odatav4_entity_post, api_odatav4_list_records, api_odatav4_get_record, api_odatav4_metadata, etc. mirroring odata.rs, but with OData V4 conventions ...
