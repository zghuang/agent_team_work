//! Health Check Routes

use axum::{routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/ready", get(ready_check))
}

async fn health_check() -> axum::Json<HealthResponse> {
    axum::Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

async fn ready_check() -> axum::Json<HealthResponse> {
    axum::Json(HealthResponse {
        status: "ready".to_string(),
        version: "0.1.0".to_string(),
    })
}
