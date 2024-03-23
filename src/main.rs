use sysinfo::System;
use std::sync::{Arc, Mutex};

use axum::{routing, Router, Json, extract::State, response::IntoResponse};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(root_get))
        .route("/api/cpus", routing::get(cpus_get))
        .with_state(AppState {sys: Arc::new(Mutex::new(System::new()))});

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {

    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}

async fn root_get() -> &'static str {
    "Hello"
}