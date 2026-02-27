use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use chrono::Utc;


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

// Message-related handlers - in-memory storage for MVP
use std::sync::Mutex;
use once_cell::sync::Lazy;

static MESSAGES: Lazy<Mutex<Vec<Message>>> = Lazy::new(|| {
    Mutex::new(vec![
        Message {
            id: 1,
            content: "BTC price update: $50000".to_string(),
            message_type: "crypto_price".to_string(),
            source: "binance".to_string(),
            created_at: Utc::now(),
        },
        Message {
            id: 2,
            content: "ETH price update: $3000".to_string(),
            message_type: "crypto_price".to_string(),
            source: "binance".to_string(),
            created_at: Utc::now(),
        },
    ])
});

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub message_type: String,
    pub source: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub async fn list_messages() -> Json<Vec<Message>> {
    let messages = MESSAGES.lock().unwrap();
    Json(messages.clone())
}

// Fetch crypto prices from public API and create messages
pub async fn fetch_crypto_prices() -> (StatusCode, Json<Vec<Message>>) {
    let client = reqwest::Client::new();
    
    // Fetch BTC price from CoinGecko API (free, no API key needed)
    let btc_response = client
        .get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
        .send()
        .await;
    
    let mut new_messages: Vec<Message> = Vec::new();
    let mut next_id = MESSAGES.lock().unwrap().len() as i64 + 1;
    
    if let Ok(response) = btc_response {
        if let Ok(data) = response.json::<serde_json::Value>().await {
            if let Some(btc_price) = data.get("bitcoin").and_then(|v| v.get("usd")) {
                let price = btc_price.as_f64().unwrap_or(0.0);
                let content = format!("BTC price update: ${}", price);
                new_messages.push(Message {
                    id: next_id,
                    content,
                    message_type: "crypto_price".to_string(),
                    source: "coingecko".to_string(),
                    created_at: Utc::now(),
                });
                next_id += 1;
            }
        }
    }
    
    // Fetch ETH price
    let eth_response = client
        .get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
        .send()
        .await;
    
    if let Ok(response) = eth_response {
        if let Ok(data) = response.json::<serde_json::Value>().await {
            if let Some(eth_price) = data.get("ethereum").and_then(|v| v.get("usd")) {
                let price = eth_price.as_f64().unwrap_or(0.0);
                let content = format!("ETH price update: ${}", price);
                new_messages.push(Message {
                    id: next_id,
                    content,
                    message_type: "crypto_price".to_string(),
                    source: "coingecko".to_string(),
                    created_at: Utc::now(),
                });
            }
        }
    }
    
    // Store the new messages
    if !new_messages.is_empty() {
        let mut messages = MESSAGES.lock().unwrap();
        messages.extend(new_messages.clone());
    }
    
    (StatusCode::OK, Json(new_messages))
}
