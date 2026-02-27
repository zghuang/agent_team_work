//! API handlers for the trading system

use crate::models::*;
use actix_web::{web, HttpResponse, Result};

/// Get market data for a symbol
pub async fn get_market_data(
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    // Placeholder - will connect to real data source
    let data = MarketData::new(
        symbol.clone(),
        format!("{} Asset", symbol),
        AssetType::Crypto,
        0.0,
    );
    
    Ok(HttpResponse::Ok().json(ApiResponse::ok(data)))
}

/// List all available strategies
pub async fn list_strategies() -> Result<HttpResponse> {
    let strategies = vec![
        Strategy::new(
            "Moving Average Crossover".to_string(),
            "Buy when short MA crosses above long MA".to_string(),
            AssetType::Crypto,
        ),
        Strategy::new(
            "RSI Overbought/Oversold".to_string(),
            "Buy when RSI < 30, sell when RSI > 70".to_string(),
            AssetType::Stock,
        ),
    ];
    
    Ok(HttpResponse::Ok().json(ApiResponse::ok(strategies)))
}

/// Create a new order
pub async fn create_order(
    web::Json(payload): web::Json<CreateOrderRequest>,
) -> Result<HttpResponse> {
    let order = Order::new_market(
        payload.symbol,
        payload.asset_type,
        payload.side,
        payload.quantity,
    );
    
    Ok(HttpResponse::Ok().json(ApiResponse::ok(order)))
}

/// Request structure for creating an order
#[derive(serde::Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub asset_type: AssetType,
    pub side: OrderSide,
    pub quantity: f64,
}
