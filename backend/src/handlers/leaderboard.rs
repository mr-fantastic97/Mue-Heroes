// backend/src/handlers/leaderboard.rs
use axum::{extract::State, Json};
use std::collections::HashMap;  
use crate::handlers::submission::{Submission, SharedState};
use crate::engine::game::Game;

#[derive(serde::Serialize)]
pub struct LeaderboardEntry {
    pub wallet_tag: String,   // truncated for UI
    pub score: u32,
    pub rank: usize,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
    pub tier: String,
}


fn tag_wallet(addr: &str) -> String {
    if addr.len() <= 14 { return addr.to_string(); }
    let start = &addr[..10.min(addr.len())];
    let end = &addr[addr.len().saturating_sub(4)..];
    format!("{start}...{end}")
}

pub async fn get_leaderboard(State(state): State<SharedState>) -> Json<Vec<LeaderboardEntry>> {
    let data = state.read().unwrap().clone();

    // Group by full wallet
    let mut by_wallet: HashMap<String, Vec<Submission>> = HashMap::new();
    for s in data {
        by_wallet.entry(s.wallet.clone()).or_default().push(s);
    }

    // Reduce per wallet
    let mut entries: Vec<LeaderboardEntry> = by_wallet.into_iter().map(|(wallet, subs)| {
        let score: u32 = subs.iter().map(|s| s.score).sum();
        let highest_mu = subs.iter().map(|s| s.mu_level).max().unwrap_or(0);

        let latest = subs.iter()
            .max_by(|a,b| a.date_mined.cmp(&b.date_mined))
            .unwrap();

        let is_mined_highest = subs.iter()
            .any(|s| s.mu_level == highest_mu && s.event_type == "mined");

        let base_tier = Game::rank_from_level(highest_mu, is_mined_highest).to_string();

        LeaderboardEntry {
            wallet_tag: tag_wallet(&wallet),
            score,
            mu_level: highest_mu,
            block_height: latest.block_height,
            date_mined: latest.date_mined.clone(),
            rank: 0,
            tier: base_tier,
        }
    }).collect();

    // Sort and medalize
    entries.sort_by(|a,b| b.score.cmp(&a.score).then_with(|| b.date_mined.cmp(&a.date_mined)));
    for (i, e) in entries.iter_mut().enumerate() {
        e.rank = i + 1;
        e.tier = match i {
            0 => format!("🥇 {}", e.tier),
            1 => format!("🥈 {}", e.tier),
            2 => format!("🥉 {}", e.tier),
            _ => e.tier.clone(),
        };
    }

    Json(entries)
}
