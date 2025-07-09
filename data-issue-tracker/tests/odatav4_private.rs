// use data_issue_tracker::odatav4::*;
// use axum::http::{Request, StatusCode};
// use axum::body::Body;
// use std::sync::Arc;

// #[tokio::test]
// async fn test_api_get_record_v4_exists() {
//     let state = Arc::new(crate::AppState::default());
//     let _ = api_get_record_v4("User", "1", state).await;
// }

// #[test]
// fn test_handle_batch_json_exists() {
//     let data = serde_json::to_vec(&vec![serde_json::json!({"method": "GET", "url": "/foo"})]).unwrap();
//     let val = handle_batch_json(&data);
//     assert!(val.get("responses").is_some());
// }

// #[tokio::test]
// async fn test_int_odatav4_entity_patch_exists() {
//     let state = Arc::new(crate::AppState::default());
//     let req = Request::builder().body(Body::empty()).unwrap();
//     let path = vec!["User(1)".to_string()];
//     let _ = int_odatav4_entity_patch(path, state, req).await;
// }

// #[tokio::test]
// async fn test_int_odatav4_entity_post_exists() {
//     let state = Arc::new(crate::AppState::default());
//     let req = Request::builder().body(Body::empty()).unwrap();
//     let path = vec!["User(1)".to_string()];
//     let _ = int_odatav4_entity_post(path, state, req).await;
// }

// #[tokio::test]
// async fn test_json_from_body_exists() {
//     let req = Request::builder().body(Body::from("{\"foo\":1}".to_string())).unwrap();
//     let val = json_from_body(req).await;
//     assert!(val.is_ok());
// }
