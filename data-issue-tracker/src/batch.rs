// async fn api_odata_batch(
//     State(state): State<Arc<AppState>>,
//     req: Request<axum::body::Body>,
// ) -> impl IntoResponse {
//     let json = match json_from_body(req).await {
//         Ok(j) => j,
//         Err(e) => return odata_json_response(StatusCode::BAD_REQUEST, None, json!({"error": e})),
//     };
//     let mut responses = Vec::new();
//     if let Some(requests) = json.as_array() {
//         for r in requests {
//             let method = r.get("method").and_then(|m| m.as_str()).unwrap_or("GET");
//             let url = r.get("url").and_then(|u| u.as_str()).unwrap_or("");
//             let body = r.get("body").cloned().unwrap_or(json!({}));
//             let path = url.trim_start_matches("/api/");
//             let response = match method {
//                 "GET" => {
//                     // Simulate GET
//                     let fake_req = Request::builder().body(axum::body::Body::empty()).unwrap();
//                     let resp = api_odata_entity(path.to_string(), state.clone(), fake_req).await;
//                     let status = resp.status().as_u16();
//                     let body = resp.body().to_owned();
//                     let body_bytes = hyper::body::to_bytes(body).await.unwrap_or_default();
//                     let json_body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({"raw": String::from_utf8_lossy(&body_bytes)}));
//                     json!({"status": status, "body": json_body})
//                 }
//                 "POST" => {
//                     // Simulate POST
//                     let fake_req = Request::builder().body(axum::body::Body::from(body.to_string())).unwrap();
//                     let path_vec = path.split('/').map(any_to_string).collect();
//                     let resp = api_odata_entity_post(path_vec, state.clone(), fake_req).await;
//                     let status = resp.status().as_u16();
//                     let body = resp.body().to_owned();
//                     let body_bytes = hyper::body::to_bytes(body).await.unwrap_or_default();
//                     let json_body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({"raw": String::from_utf8_lossy(&body_bytes)}));
//                     json!({"status": status, "body": json_body})
//                 }
//                 _ => json!({"status": 405, "body": {"error": "Method not allowed"}}),
//             };
//             responses.push(response);
//         }
//     } else {
//         return odata_json_response(StatusCode::BAD_REQUEST, None, json!({"error": "Batch body must be an array"}));
//     }
//     odata_json_response(StatusCode::OK, None, json!({"responses": responses}))
// }