use axum::{
    routing::{get, post},
    Router,
};
use data_issue_tracker::{odata, AppState};
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
        // Serve static files
        .with_state(state)
        .nest_service("/static", ServeDir::new("webapp"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.clone()).await.unwrap();
    // state.set_router(app);
}
