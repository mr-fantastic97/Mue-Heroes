use axum::{Json, extract::State};
use std::sync::{Arc, Mutex};
use crate::handlers::submission::Submission;


/// Struct representing a single leaderboard entry
#[derive(serde::Serialize)]
pub struct LeaderboardEntry {
    pub wallet: String,
    pub score: u32,
    pub rank: usize,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
}

pub type SharedState = Arc<Mutex<Vec<Submission>>>;

/// GET /leaderboard
pub async fn get_leaderboard(State(state): State<SharedState>) -> Json<Vec<LeaderboardEntry>> {
    let data = state.lock().unwrap();

    let mut entries: Vec<LeaderboardEntry> = data
        .iter()
        .map(|s| LeaderboardEntry {
            wallet: s.wallet.clone(),
            score: s.score,
            mu_level: s.mu_level,
            block_height: s.block_height,
            date_mined: s.date_mined.clone(),
            rank: 0,
        })
        .collect();

    entries.sort_by(|a, b| b.score.cmp(&a.score));
    for (i, entry) in entries.iter_mut().enumerate() {
        entry.rank = i + 1;
    }

    Json(entries)
}
