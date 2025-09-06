// backend/src/handlers/submit_witness.rs
use axum::{extract::{State, Json}, http::HeaderMap};
use chrono::Utc;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write};
use hex;

use crate::handlers::submission::{Submission, SharedState};
use crate::engine::kdapp::MueHeroSession;
use crate::state::{SESSIONS, pki::PubKey};
use crate::state::types::SuperblockEvent;
use crate::episode::{Episode, PayloadMetadata};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofJson {
    pub siblings: Vec<String>,   // each "0x" + 64 hex chars
    pub path: String,            // bitstring e.g. "0101"
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WitnessReq {
    pub wallet: String,          // hex, same identity as mining
    pub mu_level: u8,
    pub proof: ProofJson,
    // optional later: pub block_height: Option<u64>,
}

fn is_hex64(s: &str) -> bool {
    s.len() == 66 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}
fn is_bitstring(s: &str) -> bool { s.chars().all(|c| c == '0' || c == '1') }

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

pub async fn submit_witness(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<WitnessReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // auth
    let expected = std::env::var("MUE_SECRET").unwrap_or_default();
    let got = headers.get("X-MUE-KEY").and_then(|h| h.to_str().ok()).unwrap_or("");
    if expected.is_empty() || got != expected {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "bad secret".into()));
    }

    // basic validation (keep as is)
    if !(1..=64).contains(&req.mu_level) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid mu_level".into()));
    }
    if req.proof.siblings.is_empty() || req.proof.siblings.len() > 64 || !req.proof.siblings.iter().all(|h| is_hex64(h)) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid proof.siblings".into()));
    }
    if !is_bitstring(&req.proof.path) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid proof.path".into()));
    }

    // canonical identity for sessions (PubKey)
    let pubkey = decode_pubkey_from_hex(&req.wallet)?;

    // self-contained event (includes wallet string for UI/logs)
    let event = SuperblockEvent {
        wallet: req.wallet.clone(), 
        mu_level: req.mu_level,
        is_witness: true,
        merkle_root: None,          // TODO: wire verification later
        proof: None,
        witness_index: None,
        block_height: 0,            // TODO: real height later
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

    // append to memory + JSONL (log wallet + delta so Events can show per-row points)
    let payload = Submission {
        wallet: req.wallet.clone(),
        score: delta, // <-- if Submission.score is i32, change to `delta as i32`
        mu_level: req.mu_level,
        block_height: 0,
        date_mined: Utc::now().to_rfc3339(),
        event_type: "witness".into(),
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
