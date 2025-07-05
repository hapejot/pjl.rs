use axum::{
    routing::{get, patch, post},
    Router,
};
use data_issue_tracker::{odata, odatav4, AppState};
use tower_http::services::ServeDir;
use tracing::info;



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!(
        "current working directory: {}",
        std::env::current_dir().unwrap().display()
    );

    let state = AppState::new();
    let app = Router::new()
        .route("/api/$metadata", get(odata::metadata))
        .route("/api/$batch", post(odata::batch))
        .route("/api/{*path}", get(odata::entity))
        .route("/api/{*path}", post(odata::entity_post))
        .route("/api/{*path}", patch(odata::entity_post))
        .route("/apiv4/$metadata", get(odatav4::metadata))
        .route("/apiv4/$batch", post(odatav4::batch))
        .route("/apiv4/{*path}", get(odatav4::entity))
        .route("/apiv4/{*path}", post(odatav4::entity_post))
        .route("/apiv4/{*path}", patch(odatav4::entity_patch))
        // Serve static files
        .with_state(state.clone())
        .nest_service("/static", ServeDir::new("webapp"));
    state.set_router(app.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.clone()).await.unwrap();
    // state.set_router(app);
}
