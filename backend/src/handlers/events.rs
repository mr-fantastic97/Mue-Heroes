// backend/src/handlers/events.rs
use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use crate::handlers::submission::SharedState;

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


pub async fn get_events(
    State(state): State<SharedState>,
    Query(q): Query<EventsQuery>,
) -> Json<serde_json::Value> {
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

    let next_since = list.first().map(|s| s.date_mined.clone());
    Json(serde_json::json!({ "events": list, "next_since": next_since }))
}
