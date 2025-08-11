// backend/src/main.rs
mod handlers;
mod engine;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use handlers::submission::{
    handle_submission,
    SharedState,
    Submission,
    load_submissions_from_jsonl,
};
use handlers::leaderboard::get_leaderboard;
use handlers::events::get_events;

#[tokio::main]
async fn main() {
    // Load env (for MUE_SECRET)
    dotenvy::dotenv().ok();

    // CORS: wide-open for now (ok for local dev)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Preload prior submissions from JSONL (if present)
    let initial: Vec<Submission> = load_submissions_from_jsonl("logs/submissions.jsonl");
    let state: SharedState = Arc::new(RwLock::new(initial));

    // Build routes and attach shared state
    let app = Router::new()
        .route("/submit", post(handle_submission))
        .route("/leaderboard", get(get_leaderboard))
        .route("/events", get(get_events))
        .with_state(state.clone())
        .layer(cors);

    // Bind public for dev
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 Müe Heroes backend on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
