//! Binance WebSocket Client
//! Connects to Binance WebSocket streams for real-time market data

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceTicker {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "E")]
    pub event_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinanceMessage {
    Ticker(BinanceTicker),
    Unknown,
}

pub struct BinanceWsClient {
    subscribed_symbols: Arc<RwLock<Vec<String>>>,
    tx: broadcast::Sender<String>,
}

impl BinanceWsClient {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            subscribed_symbols: Arc::new(RwLock::new(Vec::new())),
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub async fn add_symbol(&self, symbol: &str) {
        let mut symbols = self.subscribed_symbols.write().await;
        if !symbols.contains(&symbol.to_string()) {
            symbols.push(symbol.to_string());
        }
    }

    pub async fn start(&self) {
        let symbols = self.subscribed_symbols.read().await.clone();
        if symbols.is_empty() {
            tracing::info!("No symbols to subscribe to");
            return;
        }

        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@ticker", s.to_lowercase()))
            .collect();
        
        let url = format!(
            "wss://stream.binance.com:9443/stream?streams={}",
            streams.join("/")
        );

        tracing::info!("Connecting to Binance WebSocket: {}", url);

        tokio::spawn(async move {
            loop {
                match connect_async(&url).await {
                    Ok((ws_stream, _)) => {
                        tracing::info!("Connected to Binance WebSocket");
                        
                        let (_, mut read) = ws_stream.split();
                        
                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    // Parse and broadcast
                                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                                        if let Some(stream_data) = data.get("data") {
                                            if let Ok(ticker) = serde_json::from_value::<BinanceTicker>(stream_data.clone()) {
                                                let broadcast_msg = serde_json::json!({
                                                    "type": "price",
                                                    "symbol": ticker.symbol,
                                                    "price": ticker.price.parse::<f64>().unwrap_or(0.0),
                                                    "quantity": ticker.quantity.parse::<f64>().unwrap_or(0.0),
                                                    "timestamp": ticker.event_time,
                                                }).to_string();
                                                
                                                let _ = tx.send(broadcast_msg);
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    tracing::warn!("Binance WebSocket closed");
                                    break;
                                }
                                Err(e) => {
                                    tracing::error!("WebSocket error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to connect: {}", e);
                    }
                }
                
                tracing::info!("Reconnecting in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }
}

impl Default for BinanceWsClient {
    fn default() -> Self {
        Self::new()
    }
}
