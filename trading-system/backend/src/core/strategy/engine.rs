//! Strategy Engine
//! Executes trading strategies and generates signals

use crate::models::{Candle, Signal, Symbol};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[async_trait]
pub trait Strategy: Send + Sync {
    /// Get strategy name
    fn name(&self) -> &str;
    
    /// Analyze market data and generate signal
    async fn analyze(&self, candles: &[Candle]) -> Option<Signal>;
    
    /// Update strategy parameters
    fn set_params(&mut self, params: serde_json::Value);
}

/// Strategy engine - manages multiple strategies
pub struct StrategyEngine {
    strategies: Arc<RwLock<HashMap<String, Box<dyn Strategy>>>>,
}

impl StrategyEngine {
    pub fn new() -> Self {
        Self {
            strategies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, strategy: Box<dyn Strategy>) {
        let mut strategies = self.strategies.write().await;
        strategies.insert(strategy.name().to_string(), strategy);
    }

    pub async fn run(&self, symbol: &Symbol, candles: &[Candle]) -> Vec<Signal> {
        let strategies = self.strategies.read().await;
        let mut signals = Vec::new();

        for (_, strategy) in strategies.iter() {
            if let Some(signal) = strategy.analyze(candles).await {
                signals.push(signal);
            }
        }

        signals
    }

    pub async fn list_strategies(&self) -> Vec<String> {
        let strategies = self.strategies.read().await;
        strategies.keys().cloned().collect()
    }
}

impl Default for StrategyEngine {
    fn default() -> Self {
        Self::new()
    }
}
