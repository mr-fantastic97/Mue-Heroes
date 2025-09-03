// backend/src/handlers/submit_mine.rs
use axum::{extract::{State, Json}, http::HeaderMap};
use chrono::Utc;
use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, sync::{Arc, RwLock}};
use hex;

use crate::handlers::submission::{Submission, SharedState};
use crate::state::{SESSIONS, pki::PubKey};
use crate::engine::kdapp::MueHeroSession;
use crate::state::types::SuperblockEvent;
use crate::episode::PayloadMetadata;
use crate::episode::Episode;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MineReq {
    pub wallet: String,          // hex (switch to kaspa:... later)!!!!!
    pub mu_level: u8,
    pub block_height: u64,
}

pub async fn submit_mine(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(req): Json<MineReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // auth (reuse MUE_SECRET /submit)
    let expected = std::env::var("MUE_SECRET").unwrap_or_default();
    let got = headers.get("X-MUE-KEY").and_then(|h| h.to_str().ok()).unwrap_or("");
    if expected.is_empty() || got != expected {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "bad secret".into()));
    }

    if req.mu_level < 1 || req.mu_level > 64 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "invalid mu_level".into()));
    }

    // run through kdapp session 
    let pubkey_bytes = hex::decode(&req.wallet).map_err(|_| {
        (axum::http::StatusCode::BAD_REQUEST, "invalid wallet hex".to_string())
    })?;
    if pubkey_bytes.len() < 32 {
        return Err((axum::http::StatusCode::BAD_REQUEST, "wallet too short".to_string()));
    }
    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&pubkey_bytes[..32]);
    let pubkey = PubKey::new(pk_arr);

    let event = SuperblockEvent {
        mu_level: req.mu_level,
        is_witness: false,
        merkle_root: None,
        proof: None,
        witness_index: None,
        block_height: req.block_height,
    };

    {
        let mut sessions = SESSIONS.write().unwrap();
        let session = sessions.entry(pubkey.clone()).or_insert_with(|| {
            MueHeroSession::initialize(vec![pubkey.clone()], &PayloadMetadata { accepting_time: 0 })
        });
        let _ = session.execute(&event, Some(pubkey.clone()), &PayloadMetadata { accepting_time: 0 });
    }

    // append to memory + JSONL
    let payload = Submission {
        wallet: req.wallet,
        score: 0,                       // (optional) fill with engine score if needed
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

    Ok(Json(serde_json::json!({ "ok": true })))
}
