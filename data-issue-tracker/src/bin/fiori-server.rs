use axum::{
    routing::{get, patch, post},
    Router,
};
use data_issue_tracker::{odata, odatav4, AppState};
use tower_http::services::{ServeDir, ServeFile};
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
        // .route("/odata/v4/processor/$metadata", get(odatav4::metadata))
        .route("/odata/v4/processor/$batch", post(odatav4::batch))
        .route("/odata/v4/processor/{*path}", get(odatav4::entity))
        .route("/odata/v4/processor/{*path}", post(odatav4::entity_post))
        .route("/odata/v4/processor/{*path}", patch(odatav4::entity_patch))
        .with_state(state.clone())
        // Serve static files
        .fallback_service(ServeDir::new("fiori"));

    state.set_router(app.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.clone()).await.unwrap();
    // state.set_router(app);
}
