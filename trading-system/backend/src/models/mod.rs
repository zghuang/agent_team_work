//! Data Models for Trading System

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Market data ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}

/// OHLCV candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}

/// Trading pair (e.g., BTC/USDT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub base: String,      // e.g., BTC
    pub quote: String,      // e.g., USDT
    pub exchange: String,   // e.g., binance
}

impl Symbol {
    pub fn new(base: &str, quote: &str, exchange: &str) -> Self {
        Self {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            exchange: exchange.to_lowercase(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

/// Order side
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Order status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Open,
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,
}

/// Trading order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new_market(symbol: Symbol, side: OrderSide, quantity: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            side,
            order_type: OrderType::Market,
            price: None,
            quantity,
            filled_quantity: 0.0,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_limit(symbol: Symbol, side: OrderSide, price: f64, quantity: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            side,
            order_type: OrderType::Limit,
            price: Some(price),
            quantity,
            filled_quantity: 0.0,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: Symbol,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub updated_at: DateTime<Utc>,
}

/// Portfolio - collection of positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub positions: Vec<Position>,
    pub total_value: f64,
    pub total_pnl: f64,
    pub updated_at: DateTime<Utc>,
}

/// Trade signal from strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub id: String,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub strength: f64,  // 0.0 to 1.0
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl Signal {
    pub fn new(symbol: Symbol, side: OrderSide, strength: f64, reason: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            side,
            strength: strength.clamp(0.0, 1.0),
            reason: reason.to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub parameters: serde_json::Value,
}

/// Exchange enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Exchange {
    Binance,
    Coinbase,
    Kraken,
    Alpaca,  // Stocks
}

impl Exchange {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "binance" => Some(Exchange::Binance),
            "coinbase" => Some(Exchange::Coinbase),
            "kraken" => Some(Exchange::Kraken),
            "alpaca" => Some(Exchange::Alpaca),
            _ => None,
        }
    }
}
