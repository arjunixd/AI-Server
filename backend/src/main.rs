mod routes;

use anyhow::Result;
use axum::{http::Method, routing::get, routing::post, serve, Router};
use ollama_rs::Ollama;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

const MODEL: &str = "ibm/granite4:3b-h";

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(routes::AppState {
        ollama: Ollama::default(),
        model: MODEL.to_string(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/ask", post(routes::ask))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    println!("Server running on http://localhost:8000");

    serve(listener, app).await?;
    Ok(())
}
