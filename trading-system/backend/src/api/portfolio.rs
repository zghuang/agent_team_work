//! Portfolio API Routes

use axum::{
    routing::get,
    Router,
};
use serde::Serialize;
use crate::models::{Position, Portfolio, Symbol};

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub total_value: f64,
    pub total_pnl: f64,
    pub positions: Vec<PositionResponse>,
}

#[derive(Serialize)]
pub struct PositionResponse {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
    pub pnl: f64,
}

async fn get_portfolio() -> axum::Json<PortfolioResponse> {
    // Mock data - in production, fetch from database
    let positions = vec![
        PositionResponse {
            symbol: "BTC/USDT".to_string(),
            quantity: 0.5,
            avg_price: 45000.0,
            current_price: 50123.50,
            pnl: 2561.75,
        },
        PositionResponse {
            symbol: "ETH/USDT".to_string(),
            quantity: 5.0,
            avg_price: 2500.0,
            current_price: 2756.80,
            pnl: 1284.0,
        },
    ];
    
    let total_value: f64 = positions.iter().map(|p| p.current_price * p.quantity).sum();
    let total_pnl: f64 = positions.iter().map(|p| p.pnl).sum();

    axum::Json(PortfolioResponse {
        total_value,
        total_pnl,
        positions,
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/portfolio", get(get_portfolio))
}
