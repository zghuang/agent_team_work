//! Paper Trading API Routes

use crate::models::paper_trading::{
    OrderSide, PaperOrder, PaperOrderStatus, PaperOrderType, PaperPortfolio, PaperPosition,
    VirtualWallet,
};
use crate::models::websocket::WsState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, delete},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Paper trading state
pub struct PaperTradingState {
    pub wallets: RwLock<HashMap<String, VirtualWallet>>,
    pub orders: RwLock<HashMap<String, PaperOrder>>,
    pub positions: RwLock<HashMap<String, PaperPosition>>,
    pub default_initial_balance: f64,
}

impl PaperTradingState {
    pub fn new() -> Self {
        Self {
            wallets: RwLock::new(HashMap::new()),
            orders: RwLock::new(HashMap::new()),
            positions: RwLock::new(HashMap::new()),
            default_initial_balance: 10000.0, // Default $10,000
        }
    }
    
    /// Get or create wallet for user
    pub async fn get_or_create_wallet(&self, user_id: &str) -> VirtualWallet {
        let wallets = self.wallets.read().await;
        if let Some(wallet) = wallets.get(user_id) {
            wallet.clone()
        } else {
            drop(wallets);
            let wallet = VirtualWallet::new(user_id.to_string(), self.default_initial_balance);
            self.wallets.write().await.insert(user_id.to_string(), wallet.clone());
            wallet
        }
    }
    
    /// Execute a paper order (simulated)
    pub async fn execute_order(&self, order: &mut PaperOrder, current_price: f64) -> Result<(), String> {
        let filled_price = match order.order_type {
            PaperOrderType::Market => current_price,
            PaperOrderType::Limit => {
                if let Some(limit_price) = order.price {
                    match order.side {
                        OrderSide::Buy if current_price > limit_price => return Err("Price above limit".to_string()),
                        OrderSide::Sell if current_price < limit_price => return Err("Price below limit".to_string()),
                        _ => limit_price,
                    }
                } else {
                    return Err("Limit order requires price".to_string());
                }
            }
        };
        
        // Check if user has sufficient balance
        let required_amount = filled_price * order.quantity;
        let mut wallets = self.wallets.write().await;
        let wallet = wallets.get_mut(&order.user_id).ok_or("Wallet not found")?;
        
        match order.side {
            OrderSide::Buy => {
                if wallet.balance < required_amount {
                    return Err("Insufficient balance".to_string());
                }
                wallet.balance -= required_amount;
            }
            OrderSide::Sell => {
                // Check if user has the position to sell
                let mut positions = self.positions.write().await;
                let pos_key = format!("{}:{}", order.user_id, order.symbol);
                if let Some(pos) = positions.get_mut(&pos_key) {
                    if pos.quantity < order.quantity {
                        return Err("Insufficient position to sell".to_string());
                    }
                    // Calculate realized P&L for closing position
                    let pnl = (filled_price - pos.avg_price) * order.quantity;
                    wallet.realized_pnl += pnl;
                    pos.quantity -= order.quantity;
                    if pos.quantity == 0.0 {
                        positions.remove(&pos_key);
                    }
                } else {
                    return Err("No position to sell".to_string());
                }
            }
        }
        
        // Update order
        order.filled_price = Some(filled_price);
        order.filled_quantity = order.quantity;
        order.status = PaperOrderStatus::Filled;
        order.updated_at = Utc::now();
        
        // Update or create position for buys
        if order.side == OrderSide::Buy {
            let mut positions = self.positions.write().await;
            let pos_key = format!("{}:{}", order.user_id, order.symbol);
            if let Some(pos) = positions.get_mut(&pos_key) {
                // Average down/up
                let total_cost = pos.avg_price * pos.quantity + filled_price * order.quantity;
                pos.quantity += order.quantity;
                pos.avg_price = total_cost / pos.quantity;
                pos.updated_at = Utc::now();
            } else {
                positions.insert(
                    pos_key,
                    PaperPosition {
                        symbol: order.symbol.clone(),
                        quantity: order.quantity,
                        avg_price: filled_price,
                        side: OrderSide::Buy,
                        unrealized_pnl: 0.0,
                        updated_at: Utc::now(),
                    },
                );
            }
        }
        
        wallet.updated_at = Utc::now();
        
        Ok(())
    }
}

impl Default for PaperTradingState {
    fn default() -> Self {
        Self::new()
    }
}

/// API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

/// Initialize wallet request
#[derive(Debug, Deserialize)]
pub struct InitWalletRequest {
    pub initial_balance: Option<f64>,
}

/// Place order request
#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub quantity: f64,
    pub price: Option<f64>,
}

/// Initialize virtual wallet
async fn init_wallet(
    State(state): State<Arc<PaperTradingState>>,
    Json(req): Json<InitWalletRequest>,
) -> Result<Json<ApiResponse<VirtualWallet>>, StatusCode> {
    let user_id = "default_user"; // Would come from auth in production
    
    let initial_balance = req.initial_balance.unwrap_or(state.default_initial_balance);
    let wallet = VirtualWallet::new(user_id.to_string(), initial_balance);
    
    state.wallets.write().await.insert(user_id.to_string(), wallet.clone());
    
    Ok(Json(ApiResponse::success(wallet)))
}

/// Get wallet info
async fn get_wallet(
    State(state): State<Arc<PaperTradingState>>,
) -> Result<Json<ApiResponse<VirtualWallet>>, StatusCode> {
    let user_id = "default_user";
    let wallet = state.get_or_create_wallet(user_id).await;
    
    Ok(Json(ApiResponse::success(wallet)))
}

/// Reset virtual funds
async fn reset_wallet(
    State(state): State<Arc<PaperTradingState>>,
) -> Result<Json<ApiResponse<VirtualWallet>>, StatusCode> {
    let user_id = "default_user";
    
    let mut wallets = state.wallets.write().await;
    if let Some(wallet) = wallets.get_mut(user_id) {
        wallet.reset();
        Ok(Json(ApiResponse::success(wallet.clone())))
    } else {
        // Create new wallet if doesn't exist
        let wallet = VirtualWallet::new(user_id.to_string(), state.default_initial_balance);
        wallets.insert(user_id.to_string(), wallet.clone());
        Ok(Json(ApiResponse::success(wallet)))
    }
}

/// Place paper trade order
async fn place_order(
    State(state): State<Arc<PaperTradingState>>,
    Json(req): Json<PlaceOrderRequest>,
) -> Result<Json<ApiResponse<PaperOrder>>, StatusCode> {
    let user_id = "default_user";
    
    // Validate input
    if req.symbol.is_empty() {
        return Ok(Json(ApiResponse::error("Symbol is required")));
    }
    if req.quantity <= 0.0 {
        return Ok(Json(ApiResponse::error("Quantity must be positive")));
    }
    
    // Parse side
    let side = match req.side.to_lowercase().as_str() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => return Ok(Json(ApiResponse::error("Side must be 'buy' or 'sell'"))),
    };
    
    // Parse order type
    let order_type = match req.order_type.to_lowercase().as_str() {
        "market" => PaperOrderType::Market,
        "limit" => {
            if req.price.is_none() {
                return Ok(Json(ApiResponse::error("Limit orders require a price")));
            }
            PaperOrderType::Limit
        }
        _ => return Ok(Json(ApiResponse::error("Order type must be 'market' or 'limit'"))),
    };
    
    // Create order
    let mut order = if let Some(price) = req.price {
        PaperOrder::new_limit(user_id.to_string(), req.symbol.clone(), side, price, req.quantity)
    } else {
        PaperOrder::new_market(user_id.to_string(), req.symbol.clone(), side, req.quantity)
    };
    
    // Get current market price (simulated)
    let current_price = 45000.0; // Would fetch from market data in production
    
    // Execute order
    match state.execute_order(&mut order, current_price).await {
        Ok(()) => {
            state.orders.write().await.insert(order.id.clone(), order.clone());
            Ok(Json(ApiResponse::success(order)))
        }
        Err(e) => Ok(ApiResponse::error(&e).into()),
    }
}

/// Get order history
async fn get_orders(
    State(state): State<Arc<PaperTradingState>>,
) -> Result<Json<ApiResponse<Vec<PaperOrder>>>, StatusCode> {
    let user_id = "default_user";
    let orders = state.orders.read().await;
    let user_orders: Vec<PaperOrder> = orders
        .values()
        .filter(|o| o.user_id == user_id)
        .cloned()
        .collect();
    
    Ok(Json(ApiResponse::success(user_orders)))
}

/// Get portfolio
async fn get_portfolio(
    State(state): State<Arc<PaperTradingState>>,
) -> Result<Json<ApiResponse<PaperPortfolio>>, StatusCode> {
    let user_id = "default_user";
    
    let wallet = state.get_or_create_wallet(user_id).await;
    let positions: Vec<PaperPosition> = state.positions.read().await
        .values()
        .filter(|p| p.symbol.split(':').next() == Some(user_id))
        .cloned()
        .collect();
    
    // Calculate total value and P&L
    let positions_value: f64 = positions.iter()
        .map(|p| p.avg_price * p.quantity)
        .sum();
    let total_value = wallet.balance + positions_value;
    let total_pnl = wallet.total_pnl;
    
    let portfolio = PaperPortfolio {
        wallet,
        positions,
        total_value,
        total_pnl,
    };
    
    Ok(Json(ApiResponse::success(portfolio)))
}

pub fn routes(state: Arc<PaperTradingState>) -> Router {
    Router::new()
        .route("/api/paper/init", post(init_wallet))
        .route("/api/paper/wallet", get(get_wallet))
        .route("/api/paper/reset", post(reset_wallet))
        .route("/api/paper/orders", post(place_order))
        .route("/api/paper/orders", get(get_orders))
        .route("/api/paper/portfolio", get(get_portfolio))
        .with_state(state)
}
