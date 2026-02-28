//! WebSocket Routes for Real-time Data

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;

/// WebSocket state for managing connections
pub struct WsState {
    pub subscribers: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<String>>>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler for market data
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

/// Handle the WebSocket connection
async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // Spawn task to forward messages to client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::info!("Received WebSocket message: {}", text);
                
                // Echo back for now - in production, process trading commands
                let _ = tx.send(format!(r#"{{"type":"response","data":"{}"}}"#, text)).await;
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    send_task.abort();
}

/// Broadcast price update to all subscribers
pub async fn broadcast_price(state: &WsState, symbol: &str, price: f64) {
    let message = serde_json::json!({
        "type": "price",
        "symbol": symbol,
        "price": price,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }).to_string();

    let subscribers = state.subscribers.read().await;
    for tx in subscribers.iter() {
        let _ = tx.send(message.clone()).await;
    }
}

pub fn routes() -> Router {
    Router::new().route("/ws", get(ws_handler))
}
