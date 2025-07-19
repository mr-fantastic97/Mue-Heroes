use axum::{Json, extract::State};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

pub type SharedState = Arc<Mutex<Vec<Submission>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Submission {
    pub wallet: String,
    pub score: u32,
    pub mu_level: u8,
    pub block_height: u64,
    pub date_mined: String,
    pub event_type: String,
}

pub async fn handle_submission(
    State(state): State<SharedState>,
    Json(payload): Json<Submission>,
) {
    let mut data = state.lock().unwrap();
    data.push(payload);
}
