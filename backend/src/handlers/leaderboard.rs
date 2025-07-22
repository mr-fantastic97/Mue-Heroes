use axum::{extract::State, Json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::handlers::submission::Submission;
use crate::engine::game::rank_from_level;

/// Struct representing a single leaderboard entry
#[derive(serde::Serialize)]
pub struct LeaderboardEntry {
    pub wallet: String,
    pub score: u32,
    pub rank: usize,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
    pub tier: String, // ier label with emoji
}

pub type SharedState = Arc<Mutex<Vec<Submission>>>;

/// GET /leaderboard
pub async fn get_leaderboard(State(state): State<SharedState>) -> Json<Vec<LeaderboardEntry>> {
    let data = state.lock().unwrap();

    //Group submissions by wallet
    let mut aggregated: HashMap<String, Vec<Submission>> = HashMap::new();
    for submission in data.iter() {
        aggregated
            .entry(submission.wallet.clone())
            .or_default()
            .push(submission.clone());
    }

    // Reduce per wallet into LeaderboardEntry
    let mut entries: Vec<LeaderboardEntry> = aggregated
        .into_iter()
        .map(|(wallet, subs)| {
            let score: u32 = subs.iter().map(|s| s.score).sum();
            let highest_mu = subs.iter().map(|s| s.mu_level).max().unwrap_or(0);

            // Use latest date_mined
            let latest_submission = subs
                .iter()
                .max_by(|a, b| a.date_mined.cmp(&b.date_mined))
                .unwrap();

            // Determine if mined μ-level exists
            let is_mined = subs
                .iter()
                .any(|s| s.mu_level == highest_mu && s.event_type == "mined");

            let tier = rank_from_level(highest_mu, is_mined).to_string();

            LeaderboardEntry {
                wallet,
                score,
                mu_level: highest_mu,
                block_height: latest_submission.block_height,
                date_mined: latest_submission.date_mined.clone(),
                rank: 0,
                tier,
            }
        })
        .collect();

    // Sort by score DESC, then latest date DESC
entries.sort_by(|a, b| {
    b.score
        .cmp(&a.score)
        .then_with(|| b.date_mined.cmp(&a.date_mined))
});


// Assign rank + medals
for (i, entry) in entries.iter_mut().enumerate() {
    entry.rank = i + 1;

    let medal = match i {
        0 => "🥇 ",
        1 => "🥈 ",
        2 => "🥉 ",
        _ => "",
    };

    entry.tier = format!("{}{}", medal, entry.tier);
}

Json(entries)

} 