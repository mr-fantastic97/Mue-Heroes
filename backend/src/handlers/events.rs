//src/handlers/events.rs

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use blake2::{Blake2b512, Digest};
use serde::{Deserialize, Serialize};

use crate::handlers::submission::{SharedState, Submission};

#[derive(Deserialize)]
pub struct EventsQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub wallet: Option<String>,
    pub since: Option<String>, // ISO
    #[serde(default = "default_order")]
    pub order: String,         // "desc" | "asc"
}
fn default_limit() -> usize { 100 }
fn default_order() -> String { "desc".into() }

#[derive(Serialize, Clone)]
pub struct EnrichedEvent {
    pub wallet: String,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
    pub event_type: String,
    pub score_delta: u32,
    pub command: String,
}

pub async fn get_events(
    State(state): State<SharedState>,
    Query(q): Query<EventsQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let mut list = state.read().unwrap().clone();

    if let Some(w) = &q.wallet {
        list.retain(|s| &s.wallet == w);
    }
    if let Some(since) = &q.since {
        list.retain(|s| s.date_mined > *since);
    }

    if q.order.to_lowercase() == "asc" {
        list.sort_by(|a,b| a.date_mined.cmp(&b.date_mined));
    } else {
        list.sort_by(|a,b| b.date_mined.cmp(&a.date_mined));
    }

    if list.len() > q.limit {
        list.truncate(q.limit);
    }

    let enriched: Vec<EnrichedEvent> = list.into_iter().map(|s| {
        let (command, score_delta) = if s.event_type == "witness" {
            ("WitnessPoints".to_string(), (s.mu_level as u32) / 2)
        } else {
            ("AddPoints".to_string(), s.mu_level as u32)
        };
        EnrichedEvent { wallet: s.wallet, mu_level: s.mu_level, block_height: s.block_height,
                        date_mined: s.date_mined, event_type: s.event_type, score_delta, command }
    }).collect();

    let next_since = enriched.first().map(|s| s.date_mined.clone());
    let body = serde_json::json!({ "events": enriched, "next_since": next_since });

    // Weak ETag over body
    let mut hasher = Blake2b512::new();
    hasher.update(serde_json::to_vec(&body).unwrap());
    let etag = format!("W/\"{:x}\"", hasher.finalize());

    if let Some(im) = headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) {
        if im == etag {
            return (StatusCode::NOT_MODIFIED, [(header::ETAG, etag)]).into_response();
        }
    }

    (StatusCode::OK, [(header::ETAG, etag)], Json(body)).into_response()
}
