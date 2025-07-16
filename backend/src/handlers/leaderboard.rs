use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::sync::MutexGuard;
use crate::lib::SESSIONS;
use crate::pki::PubKey;

#[derive(Serialize)]
pub struct LeaderboardEntry {
    wallet: String,
    score: u32,
}

pub async fn get_leaderboard() -> Json<Vec<LeaderboardEntry>> {
    let sessions = SESSIONS.lock().unwrap();
    let mut entries: Vec<LeaderboardEntry> = sessions
        .iter()
        .map(|(wallet, session)| LeaderboardEntry {
            wallet: wallet.to_string(),  // Uses your custom wallet display
            score: session.get_score(),
        })
        .collect();

    // Sort by descending score
    entries.sort_by(|a, b| b.score.cmp(&a.score));

    Json(entries)
}

pub fn leaderboard_routes() -> Router {
    Router::new().route("/leaderboard", get(get_leaderboard))
}
