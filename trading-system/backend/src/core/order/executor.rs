//! Order Executor
//! Handles order execution with exchanges

use crate::models::{Order, OrderSide, OrderStatus, OrderType, Symbol};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Order rejected: {0}")]
    Rejected(String),
}

#[async_trait]
pub trait ExchangeAdapter: Send + Sync {
    async fn place_order(&self, order: &Order) -> Result<Order, OrderError>;
    async fn cancel_order(&self, order_id: &str) -> Result<(), OrderError>;
    async fn get_order_status(&self, order_id: &str) -> Result<Order, OrderError>;
}

/// Order Executor - manages order lifecycle
pub struct OrderExecutor {
    client: Client,
    adapter: Arc<RwLock<Box<dyn ExchangeAdapter>>>,
}

impl OrderExecutor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            adapter: Arc::new(RwLock::new(Box::new(BinanceAdapter::new()))),
        }
    }

    pub async fn set_adapter(&self, adapter: Box<dyn ExchangeAdapter>) {
        let mut a = self.adapter.write().await;
        *a = adapter;
    }

    pub async fn execute(&self, mut order: Order) -> Result<Order, OrderError> {
        let adapter = self.adapter.read().await;
        
        // Update status to open
        order.status = OrderStatus::Open;
        order.updated_at = chrono::Utc::now();

        // Place order with exchange
        let result = adapter.place_order(&order).await?;
        
        Ok(result)
    }

    pub async fn cancel(&self, order_id: &str) -> Result<(), OrderError> {
        let adapter = self.adapter.read().await;
        adapter.cancel_order(order_id).await
    }
}

impl Default for OrderExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// Binance adapter implementation
pub struct BinanceAdapter {
    client: Client,
    api_key: Option<String>,
}

impl BinanceAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: None,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key: Some(api_key),
        }
    }
}

#[async_trait]
impl ExchangeAdapter for BinanceAdapter {
    async fn place_order(&self, order: &Order) -> Result<Order, OrderError> {
        let side = match order.side {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        };

        let order_type = match order.order_type {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            _ => "LIMIT", // Simplified for now
        };

        let url = "https://api.binance.com/api/v3/order";
        
        // For market orders, we simulate success
        // In production, this would make actual API calls
        let mut filled_order = order.clone();
        
        if order.order_type == OrderType::Market {
            filled_order.status = OrderStatus::Filled;
            filled_order.filled_quantity = order.quantity;
        }

        filled_order.updated_at = chrono::Utc::now();
        
        tracing::info!(
            "Placed order: {} {} {} {} @ {:?}",
            side,
            order_type,
            order.quantity,
            order.symbol.to_string(),
            order.price
        );

        Ok(filled_order)
    }

    async fn cancel_order(&self, order_id: &str) -> Result<(), OrderError> {
        tracing::info!("Cancelled order: {}", order_id);
        Ok(())
    }

    async fn get_order_status(&self, order_id: &str) -> Result<Order, OrderError> {
        // Placeholder - would fetch from exchange
        Err(OrderError::Rejected("Not implemented".to_string()))
    }
}

impl Default for BinanceAdapter {
    fn default() -> Self {
        Self::new()
    }
}
