//backend > src > handlers > submissions.rs

use axum::{extract::{State, Json}, http::HeaderMap};
use serde::{Deserialize, Serialize};
use std::{
    fs::{OpenOptions, create_dir_all, File},
    io::{Write, BufRead, BufReader},
    sync::{Arc, RwLock},
};
use chrono::Utc;
use hex;

use crate::state::{pki::PubKey, SESSIONS};
use crate::engine::kdapp::MueHeroSession;
use crate::state::types::SuperblockEvent;
use crate::episode::PayloadMetadata;
use crate::episode::Episode;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub wallet: String,
    pub score: u32,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,   // ISO-8601 UTC preferred
    pub event_type: String,   // "mined" | "witness"
}

pub type SharedState = Arc<RwLock<Vec<Submission>>>;

pub async fn handle_submission(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(mut payload): Json<Submission>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // --- auth ---
    let expected = std::env::var("MUE_SECRET").unwrap_or_default();
    let got = headers.get("X-MUE-KEY").and_then(|h| h.to_str().ok()).unwrap_or("");
    if expected.is_empty() || got != expected {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "bad secret".into()));
    }

    // --- sanity tweaks for MVP ---
    if payload.mu_level < 15 {
        return Ok(Json(serde_json::json!({"status":"ignored"})));
    }
    if payload.date_mined.trim().is_empty() {
        payload.date_mined = Utc::now().to_rfc3339();
    }

    // --- feed into kdapp session ---
    let pubkey_bytes = hex::decode(&payload.wallet).map_err(|_| {
        (axum::http::StatusCode::BAD_REQUEST, "invalid wallet hex".to_string())
    })?;
    if pubkey_bytes.len() < 32 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "wallet too short".to_string()));
    }
    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&pubkey_bytes[..32]);
    let pubkey = PubKey::new(pk_arr);

    let event = SuperblockEvent {
        wallet: payload.wallet.clone(),
        mu_level: payload.mu_level,
        is_witness: payload.event_type == "witness",
        merkle_root: None,
        proof: None,
        witness_index: None,
        block_height: payload.block_height,
    };

    {
        let mut sessions = SESSIONS.write().unwrap();
        let session = sessions.entry(pubkey.clone()).or_insert_with(|| {
            MueHeroSession::initialize(vec![pubkey.clone()], &PayloadMetadata { accepting_time: 0 })
        });
        let _ = session.execute(&event, Some(pubkey.clone()), &PayloadMetadata { accepting_time: 0 });
    }

    // --- append to memory + JSONL ---
    {
        let mut vec = state.write().unwrap();
        vec.push(payload.clone());
    }
    create_dir_all("logs").ok();
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("logs/submissions.jsonl") {
        let _ = writeln!(f, "{}", serde_json::to_string(&payload).unwrap());
    }

    Ok(Json(serde_json::json!({"status":"ok"})))
}

// -------- JSONL loader for preload on startup --------
pub fn load_submissions_from_jsonl(path: &str) -> Vec<Submission> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines().flatten() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Ok(s) = serde_json::from_str::<Submission>(line) {
            out.push(s);
        }
    }
    out
}
