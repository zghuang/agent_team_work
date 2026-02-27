use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: f64,
    pub change_24h: f64,
    pub volume_24h: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct ExchangeService {
    prices: Arc<RwLock<HashMap<String, Ticker>>>,
    client: reqwest::Client,
}

impl ExchangeService {
    pub fn new() -> Self {
        Self {
            prices: Arc::new(RwLock::new(HashMap::new())),
            client: reqwest::Client::new(),
        }
    }

    /// Get current price from Binance REST API
    pub async fn get_price(&self, symbol: &str) -> Result<Ticker, String> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/24hr?symbol={}",
            symbol.to_uppercase()
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        Ok(Ticker {
            symbol: data["symbol"].as_str().unwrap_or(symbol).to_string(),
            price: data["lastPrice"].as_f64().unwrap_or(0.0),
            change_24h: data["priceChangePercent"].as_f64().unwrap_or(0.0),
            volume_24h: data["volume"].as_f64().unwrap_or(0.0),
            high_24h: data["highPrice"].as_f64().unwrap_or(0.0),
            low_24h: data["lowPrice"].as_f64().unwrap_or(0.0),
            timestamp: data["closeTime"].as_i64().unwrap_or(0),
        })
    }

    /// Get multiple prices
    pub async fn get_prices(&self, symbols: &[&str]) -> Vec<Ticker> {
        let mut prices = Vec::new();
        for symbol in symbols {
            if let Ok(ticker) = self.get_price(symbol).await {
                prices.push(ticker);
            }
        }
        prices
    }

    /// Start WebSocket connection for real-time prices
    pub async fn start_websocket(&self, symbols: Vec<String>) {
        let symbols_str = symbols.iter()
            .map(|s| format!("{}@ticker", s.to_lowercase()))
            .collect::<Vec<_>>()
            .join("/");
        
        let url = format!("wss://stream.binance.com:9443/stream?streams={}", symbols_str);
        let prices = self.prices.clone();

        tokio::spawn(async move {
            if let Ok((ws_stream, _)) = connect_async(&url).await {
                let (_, mut read) = ws_stream.split();
                
                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(payload) = data.get("data") {
                                let ticker = Ticker {
                                    symbol: payload["s"].as_str().unwrap_or("").to_string(),
                                    price: payload["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    change_24h: payload["P"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    volume_24h: payload["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    high_24h: payload["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    low_24h: payload["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    timestamp: payload["E"].as_i64().unwrap_or(0),
                                };
                                
                                let mut prices_write = prices.write().await;
                                prices_write.insert(ticker.symbol.clone(), ticker);
                            }
                        }
                    }
                }
            }
        });
    }

    /// Get cached price
    pub async fn get_cached_price(&self, symbol: &str) -> Option<Ticker> {
        let prices = self.prices.read().await;
        prices.get(&symbol.to_uppercase()).cloned()
    }
}

impl Default for ExchangeService {
    fn default() -> Self {
        Self::new()
    }
}
