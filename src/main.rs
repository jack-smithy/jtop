use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing, Json, Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use sysinfo::System;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(root_get))
        .route("/index.js", routing::get(indexjs_get))
        .route("/api/cpus", routing::get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[derive(Debug, Serialize)]
struct DataResponse {
    labels: Vec<i32>,
    values: Vec<f32>,
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    let data = DataResponse {
        values: v.clone(),
        labels: v
            .clone()
            .iter()
            .enumerate()
            .map(|(i, _)| (i + 1) as i32)
            .collect(),
    };

    Json(data)
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(markup)
}

async fn indexjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset-utf-8")
        .body(markup)
        .unwrap()
}
