//! Risk Manager
//! Manages portfolio risk and position sizing

use crate::models::{Order, OrderSide, Position, Signal, Symbol};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub max_position_size: f64,      // Max % of portfolio per position
    pub max_loss_per_trade: f64,    // Max % loss per trade
    pub max_daily_loss: f64,        // Max % daily loss
    pub max_open_positions: usize,   // Max number of open positions
    pub stop_loss_pct: f64,          // Stop loss percentage
    pub take_profit_pct: f64,       // Take profit percentage
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_position_size: 0.1,     // 10% max per position
            max_loss_per_trade: 0.02,  // 2% max loss per trade
            max_daily_loss: 0.05,      // 5% max daily loss
            max_open_positions: 5,
            stop_loss_pct: 0.02,        // 2% stop loss
            take_profit_pct: 0.04,     // 4% take profit
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RiskCheckResult {
    Approved,
    Rejected(String),
    Reduced(f64),  // Reduced quantity
}

/// Risk Manager
pub struct RiskManager {
    config: RiskConfig,
    daily_pnl: f64,
    daily_trades: usize,
}

impl RiskManager {
    pub fn new(config: RiskConfig) -> Self {
        Self {
            config,
            daily_pnl: 0.0,
            daily_trades: 0,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(RiskConfig::default())
    }

    /// Check if a signal meets risk criteria
    pub fn check_signal(&self, signal: &Signal, portfolio_value: f64) -> RiskCheckResult {
        // Check signal strength
        if signal.strength < 0.5 {
            return RiskCheckResult::Rejected("Signal strength too low".to_string());
        }

        RiskCheckResult::Approved
    }

    /// Calculate position size based on risk parameters
    pub fn calculate_position_size(
        &self,
        signal: &Signal,
        portfolio_value: f64,
        entry_price: f64,
    ) -> f64 {
        // Risk-based position sizing
        let max_amount = portfolio_value * self.config.max_position_size;
        let risk_amount = portfolio_value * self.config.max_loss_per_trade;
        
        // Calculate quantity based on stop loss
        let stop_loss_distance = entry_price * self.config.stop_loss_pct;
        let max_quantity = risk_amount / stop_loss_distance;
        
        // Use the smaller of the two
        let quantity = max_amount.min(max_quantity) / entry_price;
        
        quantity
    }

    /// Check if order passes risk checks
    pub fn check_order(&self, order: &Order, positions: &[Position]) -> RiskCheckResult {
        // Check max positions
        if positions.len() >= self.config.max_open_positions {
            return RiskCheckResult::Rejected("Max open positions reached".to_string());
        }

        // Check position size
        // Simplified - would calculate based on current portfolio value
        
        RiskCheckResult::Approved
    }

    /// Check stop loss / take profit
    pub fn check_exit_conditions(&self, position: &Position) -> Option<OrderSide> {
        let pnl_pct = (position.current_price - position.avg_price) / position.avg_price;

        if pnl_pct <= -self.config.stop_loss_pct {
            return Some(OrderSide::Sell);
        }

        if pnl_pct >= self.config.take_profit_pct {
            return Some(OrderSide::Sell);
        }

        None
    }

    /// Update daily P&L
    pub fn update_daily_pnl(&mut self, pnl: f64) {
        self.daily_pnl += pnl;
        self.daily_trades += 1;
    }

    /// Reset daily stats (call at start of trading day)
    pub fn reset_daily(&mut self) {
        self.daily_pnl = 0.0;
        self.daily_trades = 0;
    }

    /// Check if daily loss limit reached
    pub fn daily_loss_limit_reached(&self, portfolio_value: f64) -> bool {
        let daily_loss_pct = self.daily_pnl / portfolio_value;
        daily_loss_pct <= -self.config.max_daily_loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_sizing() {
        let risk_manager = RiskManager::with_default_config();
        let signal = Signal::new(
            Symbol::new("BTC", "USDT", "binance"),
            OrderSide::Buy,
            0.8,
            "Test signal",
        );
        
        let position_size = risk_manager.calculate_position_size(&signal, 10000.0, 50000.0);
        
        // Should be positive
        assert!(position_size > 0.0);
    }
}
