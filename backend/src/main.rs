mod handlers;


use axum::{Router, routing::post};
use handlers::{submission::handle_submission, leaderboard::get_leaderboard};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use handlers::submission::Submission;


type SharedState = Arc<Mutex<Vec<Submission>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/submit", post(handle_submission))
        .route("/leaderboard", axum::routing::get(get_leaderboard))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Müe Heroes backend running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}
