//! Order Management API Routes

use axum::{
    routing::{get, post, delete},
    Router,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use crate::models::{Order, OrderSide, OrderType, OrderStatus, Symbol};

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
pub struct CreateOrderRequest {
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub quantity: f64,
}

async fn create_order(
    State(_state): State<()>,
    axum::extract::Json(req): axum::extract::Json<CreateOrderRequest>,
) -> axum::Json<ApiResponse<Order>> {
    let side = match req.side.to_lowercase().as_str() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => return axum::Json(ApiResponse::error("Invalid side")),
    };

    let order_type = match req.order_type.to_lowercase().as_str() {
        "market" => OrderType::Market,
        "limit" => OrderType::Limit,
        _ => return axum::Json(ApiResponse::error("Invalid order type")),
    };

    let symbol = Symbol::new(&req.base, &req.quote, &req.exchange);
    
    let order = if let Some(price) = req.price {
        Order::new_limit(symbol, side, price, req.quantity)
    } else {
        Order::new_market(symbol, side, req.quantity)
    };

    axum::Json(ApiResponse::success(order))
}

async fn get_order(
    Path(order_id): Path<String>,
) -> axum::Json<ApiResponse<Order>> {
    // In production, fetch from database
    axum::Json(ApiResponse::error("Not implemented"))
}

async fn cancel_order(
    Path(order_id): Path<String>,
) -> axum::Json<ApiResponse<()>> {
    // In production, cancel in database
    axum::Json(ApiResponse::success(()))
}

async fn list_orders() -> axum::Json<ApiResponse<Vec<Order>>> {
    // In production, fetch from database
    axum::Json(ApiResponse::success(vec![]))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/orders", post(create_order))
        .route("/api/orders", get(list_orders))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/:id", delete(cancel_order))
}
