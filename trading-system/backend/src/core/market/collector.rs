//! Market Data Collector
//! Fetches market data from various exchanges

use crate::models::{Candle, Exchange, Symbol, Ticker};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync RwLock;

#[derive(Error, Debug)]
pub enum MarketError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unsupported exchange: {0}")]
    UnsupportedExchange(String),
}

#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn fetch_ticker(&self, symbol: &Symbol) -> Result<Ticker, MarketError>;
    async fn fetch_candles(&self, symbol: &Symbol, interval: &str, limit: u32) -> Result<Vec<Candle>, MarketError>;
}

/// Market Data Service - aggregates data from multiple providers
pub struct MarketDataService {
    client: Client,
    providers: Arc<RwLock<Vec<Box<dyn MarketDataProvider>>>>,
}

impl MarketDataService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            providers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_provider(&self, provider: Box<dyn MarketDataProvider>) {
        let mut providers = self.providers.write().await;
        providers.push(provider);
    }

    pub async fn get_ticker(&self, symbol: &Symbol) -> Result<Ticker, MarketError> {
        // Use first available provider
        let providers = self.providers.read().await;
        if let Some(provider) = providers.first() {
            provider.fetch_ticker(symbol).await
        } else {
            Err(MarketError::UnsupportedExchange(symbol.exchange.clone()))
        }
    }

    pub async fn get_candles(&self, symbol: &Symbol, interval: &str, limit: u32) -> Result<Vec<Candle>, MarketError> {
        let providers = self.providers.read().await;
        if let Some(provider) = providers.first() {
            provider.fetch_candles(symbol, interval, limit).await
        } else {
            Err(MarketError::UnsupportedExchange(symbol.exchange.clone()))
        }
    }
}

impl Default for MarketDataService {
    fn default() -> Self {
        Self::new()
    }
}

// Binance provider implementation
pub struct BinanceProvider {
    client: Client,
}

impl BinanceProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl MarketDataProvider for BinanceProvider {
    async fn fetch_ticker(&self, symbol: &Symbol) -> Result<Ticker, MarketError> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/24hr?symbol={}{}",
            symbol.base, symbol.quote
        );

        #[derive(Deserialize)]
        struct BinanceTicker {
            symbol: String,
            last_price: String,
            volume: String,
        }

        let response: BinanceTicker = self.client.get(&url).send().await?.json().await?;

        Ok(Ticker {
            symbol: response.symbol,
            price: response.last_price.parse().unwrap_or(0.0),
            volume: response.volume.parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now(),
        })
    }

    async fn fetch_candles(&self, symbol: &Symbol, interval: &str, limit: u32) -> Result<Vec<Candle>, MarketError> {
        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}{}&interval={}&limit={}",
            symbol.base, symbol.quote, interval, limit
        );

        #[derive(Deserialize)]
        struct BinanceCandle {
            #[serde(rename = "0")]
            open_time: i64,
            #[serde(rename = "1")]
            open: String,
            #[serde(rename = "2")]
            high: String,
            #[serde(rename = "3")]
            low: String,
            #[serde(rename = "4")]
            close: String,
            #[serde(rename = "5")]
            volume: String,
        }

        let response: Vec<BinanceCandle> = self.client.get(&url).send().await?.json().await?;

        let candles: Vec<Candle> = response
            .into_iter()
            .map(|c| Candle {
                symbol: format!("{}{}", symbol.base, symbol.quote),
                open: c.open.parse().unwrap_or(0.0),
                high: c.high.parse().unwrap_or(0.0),
                low: c.low.parse().unwrap_or(0.0),
                close: c.close.parse().unwrap_or(0.0),
                volume: c.volume.parse().unwrap_or(0.0),
                timestamp: chrono::DateTime::from_timestamp_millis(c.open_time)
                    .unwrap_or_else(|| chrono::Utc::now()),
            })
            .collect();

        Ok(candles)
    }
}

impl Default for BinanceProvider {
    fn default() -> Self {
        Self::new()
    }
}
