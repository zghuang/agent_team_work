//! Market Data API Routes

use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::models::{Candle, Symbol, Ticker};

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

#[derive(Deserialize)]
pub struct TickerRequest {
    pub base: String,
    pub quote: String,
    pub exchange: String,
}

#[derive(Deserialize)]
pub struct CandlesRequest {
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub interval: Option<String>,
    pub limit: Option<u32>,
}

// Placeholder for market service - would be injected via state
async fn get_ticker_handler(
    axum::extract::Json(req): axum::extract::Json<TickerRequest>,
) -> axum::Json<ApiResponse<Ticker>> {
    // In production, this would use the MarketDataService
    let symbol = Symbol::new(&req.base, &req.quote, &req.exchange);
    
    // Return mock data for now
    let ticker = Ticker {
        symbol: symbol.to_string(),
        price: 50000.0,
        volume: 1000.0,
        timestamp: chrono::Utc::now(),
    };

    axum::Json(ApiResponse::success(ticker))
}

async fn get_candles_handler(
    axum::extract::Json(req): axum::extract::Json<CandlesRequest>,
) -> axum::Json<ApiResponse<Vec<Candle>>> {
    let interval = req.interval.unwrap_or_else(|| "1h".to_string());
    let limit = req.limit.unwrap_or(100);

    // Mock candles for demo
    let candles: Vec<Candle> = (0..limit)
        .map(|i| Candle {
            symbol: format!("{}/{}", req.base, req.quote),
            open: 50000.0 + i as f64 * 10.0,
            high: 50200.0 + i as f64 * 10.0,
            low: 49800.0 + i as f64 * 10.0,
            close: 50100.0 + i as f64 * 10.0,
            volume: 1000.0 + i as f64 * 100.0,
            timestamp: chrono::Utc::now() - chrono::Duration::hours(i as i64),
        })
        .collect();

    axum::Json(ApiResponse::success(candles))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/market/ticker", post(get_ticker_handler))
        .route("/api/market/candles", post(get_candles_handler))
}
