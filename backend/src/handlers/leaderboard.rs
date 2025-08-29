use axum::{extract::State, Json};
use crate::handlers::submission::SharedState;
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

pub async fn get_leaderboard(State(_): State<SharedState>) -> Json<Vec<LeaderboardEntry>> {
    let sessions = SESSIONS.read().unwrap();

    let mut entries: Vec<LeaderboardEntry> = sessions.iter().map(|(wallet, session)| {
        LeaderboardEntry {
            wallet_tag: tag_wallet(&wallet.to_string()),
            score: session.get_score(),
            mu_level: 0,            // TODO: track highest μ per session
            block_height: 0,        // TODO: track last block
            date_mined: "".into(),  // TODO: track last timestamp
            rank: 0,
            tier: session.get_rank(),
        }
    }).collect();

    entries.sort_by(|a,b| b.score.cmp(&a.score));
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
