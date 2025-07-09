use axum::body::Body;
use axum::http::Request;
use data_issue_tracker::{odatav4::*, AppState};
use std::sync::Arc;
#[tokio::test]
async fn test_batch_exists() {
    // Only checks that the function can be called and returns a response
    let state = Arc::new(AppState::default());
    let req = Request::builder().body(Body::empty()).unwrap();
    let _ = batch(axum::extract::State(state), req).await;
}

/// gute Idee, jedoch kein Unit-Test, da der Kontext fehlt. Die Entität User müsste vorhanden sein.
// #[tokio::test]
// async fn test_entity_exists() {
//     let state = Arc::new(AppState::default());
//     let _headers = axum::http::HeaderMap::new();
//     let req = Request::builder().body(Body::empty()).unwrap();
//     let path = "User".to_string();
//     let _ = entity(axum::extract::Path(path), axum::extract::State(state), req).await;
// }

#[tokio::test]
async fn test_entity_patch_exists() {
    let state = Arc::new(AppState::default());
    let req = Request::builder().body(Body::empty()).unwrap();
    let path = vec!["User(1)".to_string()];
    let _ = entity_patch(axum::extract::Path(path), axum::extract::State(state), req).await;
}

#[tokio::test]
async fn test_entity_post_exists() {
    let state = Arc::new(AppState::default());
    let req = Request::builder().body(Body::empty()).unwrap();
    let path = vec!["User(1)".to_string()];
    let _ = entity_post(axum::extract::Path(path), axum::extract::State(state), req).await;
}

#[tokio::test]
async fn test_metadata_exists() {
    let state = Arc::new(AppState::default());
    let headers = axum::http::HeaderMap::new();
    let _ = metadata(axum::extract::State(state), headers).await;
}


#[tokio::test]
async fn test_odata_v4_json_response() {
    // let resp = json_response(StatusCode::OK, None, serde_json::json!({"foo": 1}));
    // let body = data_issue_tracker::str_from_response(resp).await;
    // assert_eq!(body, "{\"foo\":1}");
}


#[test]
fn test_parse_odata_entity_path() {
    let res = parse_odata_entity_path("A(1)/B(2)");
    assert_eq!(
        res,
        vec![
            ("A".to_string(), Some(serde_json::Value::Number(1.into()))),
            ("B".to_string(), Some(serde_json::Value::Number(2.into()))),
        ]
    );
}

#[tokio::test]
async fn test_response_to_http_string() {
    // let resp =
    //     odata_v4_json_response(StatusCode::OK, None, serde_json::json!({"foo": 1})).into_response();
    // let s = response_to_http_string(resp).await;
    // assert!(s.contains("HTTP/1.1 200 OK"));
    // assert!(s.contains("foo"));
}
