use axum::{Json, Router, routing::get};
use serde::Serialize;
use crate::lib::SESSIONS;


#[derive(Serialize)]
pub struct LeaderboardEntry {
    wallet: String,
    score: u32,
    rank: usize,
    mu_level: u8,
    block_height: u64,
    date_mined: String,
}

pub async fn get_leaderboard() -> Json<Vec<LeaderboardEntry>> {
    let sessions = SESSIONS.lock().unwrap();
    let mut entries: Vec<LeaderboardEntry> = sessions
        .iter()
        .map(|(wallet, session)| LeaderboardEntry {
            wallet: wallet.to_string(),  // Uses your custom wallet display
            score: session.get_score(),
            mu_level: session.get_mu_level(),
            block_height: session.get_block_height(),
            date_mined: session.get_date_mined(),
            rank: 0,
        })
        .collect();

    // Sort by descending score
    entries.sort_by(|a, b| b.score.cmp(&a.score));
    for (i, entry) in entries.iter_mut().enumerate() {
        entry.rank = i + 1;

    }

    Json(entries)
}

pub fn leaderboard_routes() -> Router {
    Router::new().route("/leaderboard", get(get_leaderboard))
}
