//! Order History API Routes

use axum::{
    routing::{get, post},
    extract::Query,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResponse {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct OrderFilter {
    pub symbol: Option<String>,
    pub status: Option<String>,
    pub side: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub quantity: f64,
}

// In-memory storage for orders (MVP)
use std::sync::Mutex;
use once_cell::sync::Lazy;

static ORDERS: Lazy<Mutex<Vec<OrderResponse>>> = Lazy::new(|| {
    let now = Utc::now();
    Mutex::new(vec![
        OrderResponse {
            id: Uuid::new_v4().to_string(),
            symbol: "BTC/USDT".to_string(),
            side: "buy".to_string(),
            order_type: "limit".to_string(),
            price: Some(45000.0),
            quantity: 0.5,
            filled_quantity: 0.5,
            status: "filled".to_string(),
            created_at: now,
            updated_at: now,
        },
        OrderResponse {
            id: Uuid::new_v4().to_string(),
            symbol: "ETH/USDT".to_string(),
            side: "buy".to_string(),
            order_type: "market".to_string(),
            price: None,
            quantity: 5.0,
            filled_quantity: 5.0,
            status: "filled".to_string(),
            created_at: now,
            updated_at: now,
        },
        OrderResponse {
            id: Uuid::new_v4().to_string(),
            symbol: "SOL/USDT".to_string(),
            side: "sell".to_string(),
            order_type: "limit".to_string(),
            price: Some(120.0),
            quantity: 100.0,
            filled_quantity: 0.0,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        },
    ])
});

async fn list_orders(Query(filter): Query<OrderFilter>) -> axum::Json<Vec<OrderResponse>> {
    let orders = ORDERS.lock().unwrap();
    
    let filtered: Vec<OrderResponse> = orders
        .iter()
        .filter(|o| {
            let symbol_match = filter.symbol.as_ref()
                .map(|s| o.symbol.to_uppercase() == s.to_uppercase())
                .unwrap_or(true);
            let status_match = filter.status.as_ref()
                .map(|s| o.status == s.to_lowercase())
                .unwrap_or(true);
            let side_match = filter.side.as_ref()
                .map(|s| o.side == s.to_lowercase())
                .unwrap_or(true);
            
            symbol_match && status_match && side_match
        })
        .cloned()
        .collect();

    axum::Json(filtered)
}

async fn get_order(axum::extract::Path(id): axum::extract::Path<String>) -> Result<axum::Json<OrderResponse>, axum::http::StatusCode> {
    let orders = ORDERS.lock().unwrap();
    
    if let Some(order) = orders.iter().find(|o| o.id == id) {
        Ok(axum::Json(order.clone()))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn create_order(
    axum::Json(payload): axum::Json<CreateOrderRequest>,
) -> (axum::http::StatusCode, axum::Json<OrderResponse>) {
    let now = Utc::now();
    let order = OrderResponse {
        id: Uuid::new_v4().to_string(),
        symbol: payload.symbol,
        side: payload.side,
        order_type: payload.order_type,
        price: payload.price,
        quantity: payload.quantity,
        filled_quantity: 0.0,
        status: "open".to_string(),
        created_at: now,
        updated_at: now,
    };
    
    let mut orders = ORDERS.lock().unwrap();
    orders.push(order.clone());
    
    (axum::http::StatusCode::CREATED, axum::Json(order))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/orders", get(list_orders))
        .route("/api/orders", post(create_order))
        .route("/api/orders/:id", get(get_order))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_filter_by_symbol() {
        let orders = vec![
            OrderResponse {
                id: "1".to_string(),
                symbol: "BTC/USDT".to_string(),
                side: "buy".to_string(),
                order_type: "market".to_string(),
                price: None,
                quantity: 1.0,
                filled_quantity: 1.0,
                status: "filled".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            OrderResponse {
                id: "2".to_string(),
                symbol: "ETH/USDT".to_string(),
                side: "buy".to_string(),
                order_type: "market".to_string(),
                price: None,
                quantity: 1.0,
                filled_quantity: 1.0,
                status: "filled".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        let symbol_filter = "BTC/USDT".to_string();
        let filtered: Vec<_> = orders
            .iter()
            .filter(|o| o.symbol.to_uppercase() == symbol_filter.to_uppercase())
            .collect();
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].symbol, "BTC/USDT");
    }

    #[test]
    fn test_order_filter_by_status() {
        let orders = vec![
            OrderResponse {
                id: "1".to_string(),
                symbol: "BTC/USDT".to_string(),
                side: "buy".to_string(),
                order_type: "market".to_string(),
                price: None,
                quantity: 1.0,
                filled_quantity: 1.0,
                status: "filled".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            OrderResponse {
                id: "2".to_string(),
                symbol: "ETH/USDT".to_string(),
                side: "sell".to_string(),
                order_type: "limit".to_string(),
                price: Some(100.0),
                quantity: 1.0,
                filled_quantity: 0.0,
                status: "open".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        let status_filter = "open".to_string();
        let filtered: Vec<_> = orders
            .iter()
            .filter(|o| o.status == status_filter)
            .collect();
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].status, "open");
    }

    #[test]
    fn test_order_pnl_calculation() {
        // Test: Calculate realized P&L from filled orders
        let buy_order = OrderResponse {
            id: "1".to_string(),
            symbol: "BTC/USDT".to_string(),
            side: "buy".to_string(),
            order_type: "limit".to_string(),
            price: Some(40000.0),
            quantity: 1.0,
            filled_quantity: 1.0,
            status: "filled".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let sell_order = OrderResponse {
            id: "2".to_string(),
            symbol: "BTC/USDT".to_string(),
            side: "sell".to_string(),
            order_type: "limit".to_string(),
            price: Some(50000.0),
            quantity: 1.0,
            filled_quantity: 1.0,
            status: "filled".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let buy_price = buy_order.price.unwrap();
        let sell_price = sell_order.price.unwrap();
        let pnl = (sell_price - buy_price) * sell_order.quantity;
        
        assert_eq!(pnl, 10000.0);
    }

    #[test]
    fn test_empty_order_list() {
        let orders: Vec<OrderResponse> = vec![];
        
        let filtered: Vec<_> = orders
            .iter()
            .filter(|_| true)
            .collect();
        
        assert_eq!(filtered.len(), 0);
    }
}
