use axum::{
    extract::{State, Json},
    Router,
    routing::{get, post},
    http::Method,
};
use tower_http::cors::{CorsLayer, Any};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub wallet: String,
    pub score: u32,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
    pub event_type: String, // "mined" or "witness"
}

type AppState = Arc<Mutex<Vec<Submission>>>;

#[tokio::main]
async fn main() {
    // ✅ Allow Vite frontend access
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ✅ Shared state: in-memory submission store
    let state: AppState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/submit", post(handle_submit))
        .route("/leaderboard", get(handle_leaderboard))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ✅ POST /submit → store submission in memory
async fn handle_submit(
    State(state): State<AppState>,
    Json(payload): Json<Submission>,
) -> &'static str {
    let mut data = state.lock().unwrap();
    data.push(payload);
    "Submission received"
}

// ✅ GET /leaderboard → return all stored submissions as JSON
async fn handle_leaderboard(
    State(state): State<AppState>,
) -> Json<Vec<Submission>> {
    let data = state.lock().unwrap();
    Json(data.clone())
}
