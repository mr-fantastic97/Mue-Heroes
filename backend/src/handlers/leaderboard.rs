// backend/src/handlers/leaderboard.rs

use axum::{extract::State, Json};
use crate::handlers::submission::{SharedState, Submission};
use crate::state::SESSIONS;

#[derive(serde::Serialize)]
pub struct LeaderboardEntry {
    pub wallet_tag: String,
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
    let sessions = SESSIONS.read().unwrap();
    let submissions = state.read().unwrap();

    // Group submissions by wallet so we can pull metadata
    let mut entries: Vec<LeaderboardEntry> = sessions.iter().map(|(wallet, session)| {
        // Find this wallet’s submissions
        let mut wallet_subs: Vec<&Submission> = submissions
            .iter()
            .filter(|s| s.wallet == hex::encode(wallet.as_bytes()))
            .collect();

        // Pull latest submission for metadata
        wallet_subs.sort_by(|a, b| b.date_mined.cmp(&a.date_mined));
        let latest = wallet_subs.first();

        LeaderboardEntry {
            wallet_tag: tag_wallet(&wallet.to_string()),
            score: session.get_score(),
            mu_level: latest.map(|s| s.mu_level).unwrap_or(0),
            block_height: latest.map(|s| s.block_height).unwrap_or(0),
            date_mined: latest.map(|s| s.date_mined.clone()).unwrap_or_default(),
            rank: 0,
            tier: session.get_rank(),
        }
    }).collect();

    // Sort and assign ranks + medals
    entries.sort_by(|a,b| b.score.cmp(&a.score).then(b.date_mined.cmp(&a.date_mined)));
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
