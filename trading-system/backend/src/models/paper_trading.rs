//! Paper Trading Model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Virtual wallet for paper trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualWallet {
    pub user_id: String,
    pub balance: f64,          // Available balance
    pub initial_balance: f64,  // Starting balance
    pub total_pnl: f64,       // Total profit/loss
    pub realized_pnl: f64,    // Realized P&L from closed positions
    pub unrealized_pnl: f64,  // Unrealized P&L from open positions
    pub updated_at: DateTime<Utc>,
}

impl VirtualWallet {
    pub fn new(user_id: String, initial_balance: f64) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            balance: initial_balance,
            initial_balance,
            total_pnl: 0.0,
            realized_pnl: 0.0,
            unrealized_pnl: 0.0,
            updated_at: now,
        }
    }
    
    /// Reset wallet to initial balance
    pub fn reset(&mut self) {
        self.balance = self.initial_balance;
        self.total_pnl = 0.0;
        self.realized_pnl = 0.0;
        self.unrealized_pnl = 0.0;
        self.updated_at = Utc::now();
    }
}

/// Paper trade order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperOrder {
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: PaperOrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub filled_price: Option<f64>,
    pub filled_quantity: f64,
    pub status: PaperOrderStatus,
    pub pnl: Option<f64>,       // P&L for this order (when closed)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PaperOrder {
    pub fn new_market(user_id: String, symbol: String, side: OrderSide, quantity: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            symbol,
            side,
            order_type: PaperOrderType::Market,
            quantity,
            price: None,
            filled_price: None,
            filled_quantity: 0.0,
            status: PaperOrderStatus::Pending,
            pnl: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn new_limit(user_id: String, symbol: String, side: OrderSide, price: f64, quantity: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            symbol,
            side,
            order_type: PaperOrderType::Limit,
            quantity,
            price: Some(price),
            filled_price: None,
            filled_quantity: 0.0,
            status: PaperOrderStatus::Pending,
            pnl: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Calculate P&L for a closed position
    pub fn calculate_pnl(&self) -> f64 {
        if let (Some(filled_price), Some(quantity)) = (self.filled_price, self.filled_quantity) {
            let cost = filled_price * quantity;
            // For buy: profit when sold higher
            // For sell: profit when bought lower
            match self.side {
                OrderSide::Buy => {
                    // Need exit price to calculate, return 0 for now
                    0.0
                }
                OrderSide::Sell => {
                    // Need entry price (filled_price is the sell price)
                    // Would need entry price to calculate actual P&L
                    0.0
                }
            }
        } else {
            0.0
        }
    }
}

/// Order side
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Paper order type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaperOrderType {
    Market,
    Limit,
}

/// Paper order status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaperOrderStatus {
    Pending,
    Open,
    Filled,
    Cancelled,
    Rejected,
}

/// Paper trading position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPosition {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub side: OrderSide,
    pub unrealized_pnl: f64,
    pub updated_at: DateTime<Utc>,
}

/// Paper trading portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperPortfolio {
    pub wallet: VirtualWallet,
    pub positions: Vec<PaperPosition>,
    pub total_value: f64,
    pub total_pnl: f64,
}
