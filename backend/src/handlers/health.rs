use axum::{extract::State, http::HeaderMap, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize)]
pub struct Metrics {
    pub db_up: bool,
    pub node_reachable: bool,
    pub unreachable_streak: u32,
    pub indexer_lag_sec: u32,
    pub queue_depth: u32,
    pub rpc_error_rate: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusKind { Ready, Degraded, Down }

#[derive(Clone)]
pub struct HealthState {
    pub metrics: Arc<RwLock<Metrics>>,
    pub last_status: Arc<RwLock<StatusKind>>,
    pub last_change_at: Arc<RwLock<Instant>>,
    pub is_prod: bool,
    pub admin_token: Option<String>,
}

#[derive(Serialize)]
pub struct HealthResp {
    pub ok: bool,
    pub status: StatusKind,
    pub message: String,
    pub metrics: Metrics,
}

/* thresholds & hysteresis */
const LAG_READY: u32 = 30;
const LAG_DEG: u32 = 120;
const Q_DEG: u32 = 100;
const Q_DOWN: u32 = 500;
const RPC_DEG: f32 = 0.10;
const RPC_DOWN: f32 = 0.80;
const UNR_STREAK_DOWN: u32 = 3;
const COOLDOWN: Duration = Duration::from_millis(30_000);
const RECOVER_FROM_DOWN: (u32, u32, f32) = (100, 300, 0.50);
const RECOVER_FROM_DEG: (u32, u32, f32) = (25, 80, 0.05);

fn decide(m: &Metrics, last: StatusKind, last_at: Instant) -> (StatusKind, String, bool) {
    let in_cooldown = last_at.elapsed() < COOLDOWN;

    let hard_down = !m.db_up
        || !m.node_reachable
        || m.unreachable_streak >= UNR_STREAK_DOWN
        || m.indexer_lag_sec >= LAG_DEG
        || m.queue_depth >= Q_DOWN
        || m.rpc_error_rate >= RPC_DOWN;

    let soft_deg = (m.indexer_lag_sec >= LAG_READY && m.indexer_lag_sec < LAG_DEG)
        || (m.queue_depth >= Q_DEG && m.queue_depth < Q_DOWN)
        || (m.rpc_error_rate >= RPC_DEG && m.rpc_error_rate < RPC_DOWN)
        || (m.unreachable_streak > 0 && m.node_reachable);

    let status = if hard_down { StatusKind::Down }
                 else if soft_deg { StatusKind::Degraded }
                 else { StatusKind::Ready };

    let msg = match status {
        StatusKind::Ready => "OK".to_string(),
        StatusKind::Degraded => "Degraded: thresholds exceeded".into(),
        StatusKind::Down => "Down or critical thresholds exceeded".into(),
    };

    if in_cooldown {
        if last == StatusKind::Down && status != StatusKind::Down {
            let good = m.indexer_lag_sec < RECOVER_FROM_DOWN.0
                && m.queue_depth < RECOVER_FROM_DOWN.1
                && m.rpc_error_rate < RECOVER_FROM_DOWN.2
                && m.db_up && m.node_reachable && m.unreachable_streak == 0;
            if !good { return (StatusKind::Down, "Cooling down".into(), true); }
        } else if last == StatusKind::Degraded && status == StatusKind::Ready {
            let stable = m.indexer_lag_sec < RECOVER_FROM_DEG.0
                && m.queue_depth < RECOVER_FROM_DEG.1
                && m.rpc_error_rate < RECOVER_FROM_DEG.2
                && m.db_up && m.node_reachable && m.unreachable_streak == 0;
            if !stable { return (StatusKind::Degraded, "Stabilizing".into(), true); }
        }
    }

    (status, msg, false)
}

pub async fn get_health(State(st): State<HealthState>) -> impl IntoResponse {
    let m = st.metrics.read().unwrap().clone();
    let last = *st.last_status.read().unwrap();
    let last_at = *st.last_change_at.read().unwrap();

    let (mut status, msg, stick) = decide(&m, last, last_at);
    if stick { status = last; }

    if status != last {
        *st.last_status.write().unwrap() = status;
        *st.last_change_at.write().unwrap() = Instant::now();
    }

    let ok = matches!(status, StatusKind::Ready);
    Json(HealthResp { ok, status, message: msg, metrics: m })
}

/// Dev-only overrides: POST /health/{ready|degraded|down}
pub async fn override_health(
    State(st): State<HealthState>,
    axum::extract::Path(which): axum::extract::Path<String>,
    headers: HeaderMap,
) -> Response {
    if st.is_prod {
        return (axum::http::StatusCode::FORBIDDEN,
                Json(serde_json::json!({"ok":false,"error":"disabled in prod"})))
            .into_response();
    }
    let need = st.admin_token.clone().unwrap_or_default();
    let got = headers.get("x-admin-token").and_then(|h| h.to_str().ok()).unwrap_or("");
    if need.is_empty() || need != got {
        return (axum::http::StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"ok":false,"error":"unauthorized"})))
            .into_response();
    }

    {
        let mut m = st.metrics.write().unwrap();
        match which.as_str() {
            "ready" => { m.db_up = true; m.node_reachable = true; m.indexer_lag_sec = 5; m.queue_depth = 0; m.rpc_error_rate = 0.0; m.unreachable_streak = 0; }
            "degraded" => { m.db_up = true; m.node_reachable = true; m.indexer_lag_sec = 45; m.queue_depth = 150; m.rpc_error_rate = 0.2; m.unreachable_streak = 0; }
            "down" => { m.db_up = false; m.node_reachable = false; m.indexer_lag_sec = 200; m.queue_depth = 800; m.rpc_error_rate = 0.9; m.unreachable_streak = 5; }
            _ => {
                return (axum::http::StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"ok":false,"error":"unknown state"})))
                    .into_response();
            }
        }
    }

    // Reuse normal health output
    get_health(State(st)).await.into_response()
}
