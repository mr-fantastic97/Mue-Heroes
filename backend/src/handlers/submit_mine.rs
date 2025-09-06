//src/handlers/submit_mine.rs

use axum::{extract::{State, Json}, http::HeaderMap};
use chrono::Utc;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write};
use hex;

use crate::handlers::submission::{Submission, SharedState};
use crate::state::{SESSIONS, pki::PubKey};
use crate::engine::kdapp::MueHeroSession;
use crate::state::types::SuperblockEvent;
use crate::episode::{Episode, PayloadMetadata};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MineReq {
    pub wallet: String,          // hex (switch to kaspa:... later)
    pub mu_level: u8,
    pub block_height: u64,
}

fn decode_pubkey_from_hex(s: &str) -> Result<PubKey, (axum::http::StatusCode, String)> {
    let h = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(h).map_err(|_| (axum::http::StatusCode::BAD_REQUEST, "invalid wallet hex".to_string()))?;
    if bytes.len() < 32 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "wallet too short".to_string()));
    }
    let mut pk = [0u8; 32];
    pk.copy_from_slice(&bytes[..32]);
    Ok(PubKey::new(pk))
}

pub async fn submit_mine(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<MineReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // auth
    let expected = std::env::var("MUE_SECRET").unwrap_or_default();
    let got = headers.get("X-MUE-KEY").and_then(|h| h.to_str().ok()).unwrap_or("");
    if expected.is_empty() || got != expected {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "bad secret".into()));
    }

    if !(1..=64).contains(&req.mu_level) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid mu_level".into()));
    }

    // canonical identity for sessions (PubKey)
    let pubkey = decode_pubkey_from_hex(&req.wallet)?;

    // self-contained event (includes wallet string for UI/logs)
    let event = SuperblockEvent {
        wallet: req.wallet.clone(),
        mu_level: req.mu_level,
        is_witness: false,
        merkle_root: None,
        proof: None,
        witness_index: None,
        block_height: req.block_height,
    };

    // engine/session update → get awarded points (delta)
    let meta = PayloadMetadata { accepting_time: Utc::now().timestamp() as u64 };
    let delta = {
        let mut sessions = SESSIONS.write().unwrap();
        let session = sessions.entry(pubkey.clone()).or_insert_with(|| {
            MueHeroSession::initialize(vec![pubkey.clone()], &meta)
        });
        session.execute(&event, Some(pubkey.clone()), &meta).unwrap_or(0)
    };

    // append to memory + JSONL (log the delta so Events can show per-row points)
    let payload = Submission {
        wallet: req.wallet.clone(),
        score: delta, // <-- if Submission.score is i32, change to `delta as i32`
        mu_level: req.mu_level,
        block_height: req.block_height,
        date_mined: Utc::now().to_rfc3339(),
        event_type: "mined".into(),
    };
    {
        let mut vec = state.write().unwrap();
        vec.push(payload.clone());
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("logs/submissions.jsonl") {
        let _ = writeln!(f, "{}", serde_json::to_string(&payload).unwrap());
    }

    Ok(Json(serde_json::json!({ "ok": true, "points_awarded": delta })))
}
