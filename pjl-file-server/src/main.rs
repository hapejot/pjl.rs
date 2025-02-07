use std::net::SocketAddr;

use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser)]
struct CmdArgs {
    #[clap(short, default_value = "0.0.0.0:3000")]
    port: String,
    path: String,
}

#[tokio::main]
async fn main() {
    let args = CmdArgs::parse();

    let try_from_default_env = tracing_subscriber::EnvFilter::try_from_default_env();
    tracing_subscriber::registry()
        .with(
            try_from_default_env
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let mut r = Router::new();
    r = r.nest_service("/files", ServeDir::new(args.path));
    serve(r, args.port).await;
}

async fn serve(app: Router, port: String) {
    // let addr = SocketAddr::from(port.as_str());
    let listener = tokio::net::TcpListener::bind(port.parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
