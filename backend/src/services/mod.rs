//! Business logic services

use crate::models::*;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Market data service - fetches data from external sources
pub struct MarketService {
    client: Client,
}

impl MarketService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch current price for a symbol
    /// Note: This is a placeholder - will integrate real APIs
    pub async fn get_price(&self, symbol: &str) -> Result<f64, String> {
        // Placeholder: In production, connect to:
        // - Crypto: Binance, Coinbase APIs
        // - Stocks: Alpha Vantage, Yahoo Finance, Polygon.io
        tracing::info!("Fetching price for {}", symbol);
        Ok(0.0)
    }
}

/// Trading service - manages orders and positions
pub struct TradingService {
    orders: Arc<RwLock<Vec<Order>>>,
}

impl TradingService {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Submit a new order
    pub async fn submit_order(&self, order: Order) -> Result<Order, String> {
        let mut orders = self.orders.write().await;
        orders.push(order.clone());
        tracing::info!("Order submitted: {}", order.id);
        Ok(order)
    }

    /// Get order by ID
    pub async fn get_order(&self, id: &str) -> Option<Order> {
        let orders = self.orders.read().await;
        orders.iter().find(|o| o.id == id).cloned()
    }

    /// List all orders
    pub async fn list_orders(&self) -> Vec<Order> {
        let orders = self.orders.read().await;
        orders.clone()
    }
}

/// Strategy service - manages trading strategies
pub struct StrategyService {
    strategies: Arc<RwLock<Vec<Strategy>>>,
}

impl StrategyService {
    pub fn new() -> Self {
        let strategies = vec![
            Strategy::new(
                "Moving Average Crossover".to_string(),
                "Buy when short MA crosses above long MA".to_string(),
                AssetType::Crypto,
            ),
            Strategy::new(
                "RSI Strategy".to_string(),
                "RSI-based entry points".to_string(),
                AssetType::Stock,
            ),
        ];
        
        Self {
            strategies: Arc::new(RwLock::new(strategies)),
        }
    }

    /// Get all strategies
    pub async fn list(&self) -> Vec<Strategy> {
        let strategies = self.strategies.read().await;
        strategies.clone()
    }

    /// Get enabled strategies
    pub async fn list_enabled(&self) -> Vec<Strategy> {
        let strategies = self.strategies.read().await;
        strategies.iter().filter(|s| s.enabled).cloned().collect()
    }
}
