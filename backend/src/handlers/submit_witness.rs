// backend/src/handlers/submit_witness.rs
use axum::{extract::{State, Json}, http::HeaderMap};
use chrono::Utc;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write};
use hex;

use crate::handlers::submission::{Submission, SharedState};
use crate::state::{SESSIONS, pki::PubKey};
use crate::engine::kdapp::MueHeroSession;
use crate::state::types::SuperblockEvent;
use crate::episode::PayloadMetadata;
use crate::episode::Episode;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofJson {
    pub siblings: Vec<String>,   // each "0x" + 64 hex chars
    pub path: String,            // bitstring e.g. "0101"
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WitnessReq {
    pub wallet: String,
    pub mu_level: u8,
    pub proof: ProofJson,
}

fn is_hex64(s: &str) -> bool {
    s.len() == 66 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_bitstring(s: &str) -> bool { s.chars().all(|c| c == '0' || c == '1') }

pub async fn submit_witness(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<WitnessReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let expected = std::env::var("MUE_SECRET").unwrap_or_default();
    let got = headers.get("X-MUE-KEY").and_then(|h| h.to_str().ok()).unwrap_or("");
    if expected.is_empty() || got != expected {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "bad secret".into()));
    }

    if req.mu_level < 1 || req.mu_level > 64 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid mu_level".into()));
    }
    if req.proof.siblings.is_empty() || req.proof.siblings.len() > 64 || !req.proof.siblings.iter().all(|h| is_hex64(h)) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid proof.siblings".into()));
    }
    if !is_bitstring(&req.proof.path) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid proof.path".into()));
    }

    // kdapp session call (proof not yet fed into engine; placeholder None like my current flow)
    let pubkey_bytes = hex::decode(&req.wallet).map_err(|_| {
        (axum::http::StatusCode::BAD_REQUEST, "invalid wallet hex".to_string())
    })?;
    if pubkey_bytes.len() < 32 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "wallet too short".to_string()));
    }
    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&pubkey_bytes[..32]);
    let pubkey = crate::state::pki::PubKey::new(pk_arr);

    let event = SuperblockEvent {
        mu_level: req.mu_level,
        is_witness: true,
        merkle_root: None,
        proof: None,                 // TODO: map req.proof -> engine type when ready
        witness_index: None,
        block_height: 0,
    };
    {
        let mut sessions = crate::state::SESSIONS.write().unwrap();
        let session = sessions.entry(pubkey.clone()).or_insert_with(|| {
            MueHeroSession::initialize(vec![pubkey.clone()], &PayloadMetadata { accepting_time: 0 })
        });
        let _ = session.execute(&event, Some(pubkey.clone()), &PayloadMetadata { accepting_time: 0 });
    }

    // append to log
    let payload = Submission {
        wallet: req.wallet,
        score: 0,
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

    Ok(Json(serde_json::json!({ "ok": true })))
}
