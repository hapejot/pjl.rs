use std::{
    convert::Infallible,
    future::{Future, IntoFuture},
    net::SocketAddr,
    pin::Pin,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    extract::{Extension, Path},
    http::Request,
    response::Response,
    routing::{future::RouteFuture, get},
    Router,
};
use tower::Service;
use tracing::{info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod controllers;
mod error;
mod models;

#[derive(Clone)]
struct State {
    last_call: usize,
}

#[allow(dead_code)]
async fn get_handler(Path(objtype): Path<String>) -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!([format!(
        "hello, I am here {:?}.",
        objtype
    )]))
}

#[derive(Clone, Debug)]
struct SampleData {
    value: usize,
}

async fn handler_out() -> (Extension<SampleData>, String) {
    (
        Extension(SampleData { value: 42 }),
        String::from("output handler was here"),
    )
}

async fn handler_in(e: Extension<Arc<Mutex<State>>>) -> String {
    if let Ok(mut x) = e.try_lock() {
        x.last_call += 1;
        format!("state last call: {}\n", x.last_call)
    }
    else {
        format!("race condition\n")
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "simple_rest_server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let extension = Extension(Arc::new(Mutex::new(State { last_call: 0 })));
    let svc = Router::new()
        .route("/Entity", get(handler_in))
        .layer(extension);

    let app = Router::new().nest("/", svc);

    let port = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    tracing::debug!("listening on {}", port);
    axum::Server::bind(&port)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
