use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: i64,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "trading-backend"
    }))
}

pub async fn list_orders() -> Json<Vec<Order>> {
    Json(vec![
        Order {
            id: 1,
            symbol: "BTC/USD".to_string(),
            quantity: 0.5,
            price: 50000.0,
            status: "filled".to_string(),
        },
        Order {
            id: 2,
            symbol: "ETH/USD".to_string(),
            quantity: 10.0,
            price: 3000.0,
            status: "pending".to_string(),
        },
    ])
}

pub async fn create_order(
    Json(payload): Json<CreateOrderRequest>,
) -> (StatusCode, Json<Order>) {
    let order = Order {
        id: 3,
        symbol: payload.symbol,
        quantity: payload.quantity,
        price: payload.price,
        status: "pending".to_string(),
    };
    (StatusCode::CREATED, Json(order))
}
