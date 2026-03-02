//! WebSocket Routes for Real-time Market Data
//!
//! Features:
//! - Subscribe/unsubscribe to trading pairs
//! - Real-time price push
//! - Heartbeat/keepalive mechanism

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::Response,
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;

/// WebSocket message types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WsMessage {
    // Client -> Server
    Subscribe { symbols: Vec<String> },
    Unsubscribe { symbols: Vec<String> },
    Ping,
    
    // Server -> Client
    Price {
        symbol: String,
        price: f64,
        volume: f64,
        timestamp: String,
    },
    Pong,
    Subscribed { symbols: Vec<String> },
    Unsubscribed { symbols: Vec<String> },
    Error { message: String },
    Heartbeat { timestamp: String },
}

/// WebSocket connection state
struct WsConnection {
    subscriptions: HashSet<String>,
    sender: mpsc::Sender<String>,
}

/// WebSocket state for managing connections
pub struct WsState {
    pub connections: RwLock<Vec<Arc<RwLock<WsConnection>>>>,
    /// Simulated prices for demo (in production, connect to real exchange)
    pub prices: RwLock<std::collections::HashMap<String, f64>>,
}

impl WsState {
    pub fn new() -> Self {
        let mut prices = std::collections::HashMap::new();
        // Initialize with some demo prices
        prices.insert("BTC/USDT".to_string(), 45000.0);
        prices.insert("ETH/USDT".to_string(), 2500.0);
        prices.insert("SOL/USDT".to_string(), 100.0);
        prices.insert("BNB/USDT".to_string(), 300.0);
        
        Self {
            connections: RwLock::new(Vec::new()),
            prices: RwLock::new(prices),
        }
    }
    
    /// Get current price for a symbol
    pub async fn get_price(&self, symbol: &str) -> Option<f64> {
        self.prices.read().await.get(symbol).copied()
    }
    
    /// Update price for a symbol (called by market data feed)
    pub async fn update_price(&self, symbol: &str, price: f64) {
        self.prices.write().await.insert(symbol.to_string(), price);
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler for market data
pub async fn ws_handler(
    State(state): State<Arc<WsState>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

/// Handle the WebSocket connection
async fn handle_socket(state: Arc<WsState>, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Create connection state
    let connection = Arc::new(RwLock::new(WsConnection {
        subscriptions: HashSet::new(),
        sender: tx.clone(),
    }));
    
    // Add to connections list
    state.connections.write().await.push(connection.clone());
    
    // Spawn task to forward messages to client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // Spawn heartbeat task
    let heartbeat_tx = tx.clone();
    let heartbeat_task = tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(30));
        loop {
            ticker.tick().await;
            let heartbeat = serde_json::to_string(&WsMessage::Heartbeat {
                timestamp: chrono::Utc::now().to_rfc3339(),
            }).unwrap_or_default();
            if heartbeat_tx.send(heartbeat).await.is_err() {
                break;
            }
        }
    });
    
    // Handle incoming messages from client
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_message(&state, &connection, &tx, &text).await {
                    let error_msg = serde_json::to_string(&WsMessage::Error {
                        message: e.to_string(),
                    }).unwrap_or_default();
                    let _ = tx.send(error_msg).await;
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = sender.send(Message::Pong(data)).await;
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }
    
    // Cleanup
    send_task.abort();
    heartbeat_task.abort();
    
    // Remove from connections
    let mut connections = state.connections.write().await;
    connections.retain(|c| !Arc::ptr_eq(c, &connection));
}

/// Handle incoming WebSocket message
async fn handle_message(
    state: &Arc<WsState>,
    connection: &Arc<RwLock<WsConnection>>,
    sender: &mpsc::Sender<String>,
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let msg: WsMessage = serde_json::from_str(text)?;
    
    match msg {
        WsMessage::Subscribe { symbols } => {
            {
                let mut conn = connection.write().await;
                for symbol in &symbols {
                    conn.subscriptions.insert(symbol.clone());
                }
            }
            let response = WsMessage::Subscribed { symbols: symbols.clone() };
            sender.send(serde_json::to_string(&response)?).await?;
            
            // Send current prices for subscribed symbols
            for symbol in &symbols {
                if let Some(price) = state.get_price(symbol).await {
                    let price_msg = WsMessage::Price {
                        symbol: symbol.clone(),
                        price,
                        volume: 0.0,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };
                    sender.send(serde_json::to_string(&price_msg)?).await?;
                }
            }
        }
        
        WsMessage::Unsubscribe { symbols } => {
            let mut conn = connection.write().await;
            for symbol in symbols {
                conn.subscriptions.remove(&symbol);
            }
            let response = WsMessage::Unsubscribed { symbols };
            sender.send(serde_json::to_string(&response)?).await?;
        }
        
        WsMessage::Ping => {
            let response = WsMessage::Pong;
            sender.send(serde_json::to_string(&response)?).await?;
        }
        
        _ => {}
    }
    
    Ok(())
}

/// Broadcast price update to all subscribers
pub async fn broadcast_price(state: &WsState, symbol: &str, price: f64, volume: f64) {
    let message = WsMessage::Price {
        symbol: symbol.to_string(),
        price,
        volume,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    let msg_string = serde_json::to_string(&message).unwrap_or_default();
    let connections = state.connections.read().await;
    
    for conn in connections.iter() {
        let conn = conn.read().await;
        if conn.subscriptions.contains(symbol) {
            let _ = conn.sender.send(msg_string.clone()).await;
        }
    }
}

use std::sync::Arc;

/// Get routes
pub fn routes(state: Arc<WsState>) -> Router {
    Router::new()
        .route("/ws/market", get(ws_handler))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ws_message_serialization() {
        let msg = WsMessage::Subscribe {
            symbols: vec!["BTC/USDT".to_string()],
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"Subscribe\""));
        
        let parsed: WsMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, WsMessage::Subscribe { .. }));
    }
}
