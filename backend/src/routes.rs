use async_stream::stream;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Json, Sse,
    },
};
use futures_util::StreamExt;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}

pub struct AppState {
    pub ollama: Ollama,
    pub model: String,
}

#[derive(Deserialize)]
pub struct AskRequest {
    pub question: String,
}

pub async fn ask(state: State<Arc<AppState>>, payload: Json<AskRequest>) -> impl IntoResponse {
    let question = payload.question.clone();

    let request = GenerationRequest::new(state.model.clone(), question);
    let mut stream = state.ollama.generate_stream(request).await.unwrap();

    let sse_stream = stream! {

        while let Some(Ok(chunks)) = stream.next().await {
            for chunk in chunks {
                yield Ok::<_, axum::Error>(Event::default().data(format!(r#"{{"token":"{}","done":false}}"#, chunk.response)));
            }
        }

        yield Ok::<_, axum::Error>(Event::default().data(r#"{"token":"","done":true,"source":"ollama"}"#));
    };

    Sse::new(sse_stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}
