//! Strategy API Routes

use axum::{
    routing::{get, post, delete},
    Router,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StrategyInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub params: serde_json::Value,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

// In-memory store for demo
static STRATEGIES: std::sync::LazyLock<Vec<StrategyInfo>> = std::sync::LazyLock::new(|| {
    vec![
        StrategyInfo {
            id: "1".to_string(),
            name: "SMA Crossover".to_string(),
            status: "active".to_string(),
            params: serde_json::json!({"short_period": 20, "long_period": 50}),
        },
        StrategyInfo {
            id: "2".to_string(),
            name: "RSI Strategy".to_string(),
            status: "paused".to_string(),
            params: serde_json::json!({"period": 14, "oversold": 30, "overbought": 70}),
        },
    ]
});

async fn list_strategies() -> axum::Json<Vec<StrategyInfo>> {
    axum::Json(STRATEGIES.clone())
}

async fn get_strategy(
    Path(id): Path<String>,
) -> axum::Json<ApiResponse<StrategyInfo>> {
    if let Some(s) = STRATEGIES.iter().find(|s| s.id == id) {
        axum::Json(ApiResponse::success(s.clone()))
    } else {
        axum::Json(ApiResponse::error("Strategy not found"))
    }
}

#[derive(Deserialize)]
pub struct UpdateStrategyRequest {
    pub status: Option<String>,
    pub params: Option<serde_json::Value>,
}

async fn update_strategy(
    Path(id): Path<String>,
    _req: axum::extract::Json<UpdateStrategyRequest>,
) -> axum::Json<ApiResponse<StrategyInfo>> {
    // In production, update in database
    axum::Json(ApiResponse::error("Not implemented"))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/strategies", get(list_strategies))
        .route("/api/strategies/:id", get(get_strategy))
        .route("/api/strategies/:id", post(update_strategy))
}
