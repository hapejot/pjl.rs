use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
struct MarkdownRequest {
    markdown: String,
}

#[derive(Serialize)]
struct ConvertResponse {
    result: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn convert_markdown(
    Json(payload): Json<MarkdownRequest>,
) -> Result<Json<ConvertResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Rufe pandoc auf, um Markdown zu konvertieren
    let output = Command::new("pandoc")
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("json")
        .arg("--no-highlight")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    let mut child = match output {
        Ok(child) => child,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to spawn pandoc: {}", e),
                }),
            ));
        }
    };

    // Schreibe Markdown in stdin
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        if let Err(e) = stdin.write_all(payload.markdown.as_bytes()) {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to write to pandoc stdin: {}", e),
                }),
            ));
        }
    }

    // Warte auf Pandoc und hole die Ausgabe
    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to wait for pandoc: {}", e),
                }),
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Pandoc error: {}", stderr),
            }),
        ));
    }

    let result = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to parse pandoc output: {}", e),
                }),
            ));
        }
    };

    Ok(Json(ConvertResponse { result }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/convert", post(convert_markdown));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server listening on http://0.0.0.0:3000");
    println!("POST to /convert with JSON: {{\"markdown\": \"# Hello World\"}}");
    
    axum::serve(listener, app).await.unwrap();
}
