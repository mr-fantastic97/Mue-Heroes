mod handlers;
mod engine;
#[path = "state/lib.rs"]
mod state;
#[path = "episode.rs"]
mod episode;

use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderMap, HeaderValue, Method},
    routing::{get, post},
    Json, Router,
};
use std::{net::SocketAddr, sync::{Arc, RwLock}, time::Duration};
use tower_http::{
    cors::{Any, AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
};

use handlers::events::get_events;
use handlers::health::{get_health, override_health, HealthState, Metrics, StatusKind};
use handlers::leaderboard::get_leaderboard;
use handlers::submission::{handle_submission, load_submissions_from_jsonl, SharedState, Submission};
use handlers::submit_mine::submit_mine;
use handlers::submit_witness::submit_witness;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // --- CORS
    let origins_env =
        std::env::var("CORS_ORIGINS").unwrap_or_else(|_| "http://localhost:5173".into());
    let origin_values: Vec<HeaderValue> = origins_env
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<HeaderValue>().ok())
        .collect();

    let cors = if origin_values.is_empty() {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([
                header::CONTENT_TYPE,
                header::HeaderName::from_static("x-mue-key"),
                header::HeaderName::from_static("x-admin-token"),
            ])
    } else {
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origin_values))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([
                header::CONTENT_TYPE,
                header::HeaderName::from_static("x-mue-key"),
                header::HeaderName::from_static("x-admin-token"),
            ])
    };

    // --- shared states
    let initial: Vec<Submission> = load_submissions_from_jsonl("logs/submissions.jsonl");
    let submissions_state: SharedState = Arc::new(RwLock::new(initial));

    let health_state = HealthState {
        metrics: Arc::new(RwLock::new(Metrics {
            db_up: true,
            node_reachable: true,
            unreachable_streak: 0,
            indexer_lag_sec: 12,
            queue_depth: 0,
            rpc_error_rate: 0.0,
        })),
        last_status: Arc::new(RwLock::new(StatusKind::Ready)),
        last_change_at: Arc::new(RwLock::new(std::time::Instant::now())),
        is_prod: std::env::var("NODE_ENV").ok().as_deref() == Some("production"),
        admin_token: std::env::var("ADMIN_TOKEN").ok(),
    };

    // --- routers by state type
    let api_router = Router::new()
        .route("/submit", post(handle_submission)) // legacy/compat
        .route("/submit/mine", post(submit_mine))
        .route("/submit/witness", post(submit_witness))
        .route("/events", get(get_events))
        .route("/leaderboard", get(get_leaderboard))
        .with_state(submissions_state.clone());

    let health_router = Router::new()
        .route("/health", get(get_health))
        .route("/health/:which", post(override_health))
        .with_state(health_state.clone());

    // /reset needs *both* states; use a tiny router with tuple state
    let reset_router = Router::new()
        .route("/reset", post(reset_dev_only))
        .with_state((submissions_state.clone(), health_state.clone()));

    // --- compose
    let app = Router::new()
        .merge(api_router)
        .merge(health_router)
        .merge(reset_router)
        .layer(DefaultBodyLimit::max(32 * 1024)) // 32KB
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 Müe Heroes backend on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// --- dev-only reset (requires x-admin-token) ---
async fn reset_dev_only(
    State((submissions, h)): State<(SharedState, HealthState)>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    if h.is_prod {
        return Err((axum::http::StatusCode::FORBIDDEN, "disabled in prod".into()));
    }
    let need = h.admin_token.clone().unwrap_or_default();
    let got = headers.get("x-admin-token").and_then(|v| v.to_str().ok()).unwrap_or("");
    if need.is_empty() || need != got {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "unauthorized".into()));
    }
    submissions.write().unwrap().clear();
    Ok(Json(serde_json::json!({ "ok": true })))
}
