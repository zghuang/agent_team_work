//! Portfolio API Routes

use axum::{
    routing::get,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub total_value: f64,
    pub total_pnl: f64,
    pub total_pnl_percentage: f64,
    pub positions: Vec<PositionResponse>,
}

#[derive(Serialize)]
pub struct PositionResponse {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

async fn get_portfolio() -> axum::Json<PortfolioResponse> {
    // Mock data - in production, fetch from database
    let positions = vec![
        PositionResponse {
            symbol: "BTC/USDT".to_string(),
            quantity: 0.5,
            avg_price: 45000.0,
            current_price: 50123.50,
            pnl: 2561.75,
            pnl_percentage: 11.39,
        },
        PositionResponse {
            symbol: "ETH/USDT".to_string(),
            quantity: 5.0,
            avg_price: 2500.0,
            current_price: 2756.80,
            pnl: 1284.0,
            pnl_percentage: 10.27,
        },
    ];
    
    let total_value: f64 = positions.iter().map(|p| p.current_price * p.quantity).sum();
    let total_pnl: f64 = positions.iter().map(|p| p.pnl).sum();
    let total_cost: f64 = positions.iter().map(|p| p.avg_price * p.quantity).sum();
    let total_pnl_percentage = if total_cost > 0.0 {
        (total_pnl / total_cost) * 100.0
    } else {
        0.0
    };

    axum::Json(PortfolioResponse {
        total_value,
        total_pnl,
        total_pnl_percentage,
        positions,
    })
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/portfolio", get(get_portfolio))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_pnl_calculation() {
        let position = PositionResponse {
            symbol: "BTC/USDT".to_string(),
            quantity: 1.0,
            avg_price: 40000.0,
            current_price: 50000.0,
            pnl: 0.0,
            pnl_percentage: 0.0,
        };
        
        // Calculate P&L
        let pnl = (position.current_price - position.avg_price) * position.quantity;
        let pnl_percentage = ((position.current_price - position.avg_price) / position.avg_price) * 100.0;
        
        assert_eq!(pnl, 10000.0);
        assert!((pnl_percentage - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_total_portfolio_value() {
        let positions = vec![
            PositionResponse {
                symbol: "BTC/USDT".to_string(),
                quantity: 0.5,
                avg_price: 45000.0,
                current_price: 50000.0,
                pnl: 2500.0,
                pnl_percentage: 11.11,
            },
            PositionResponse {
                symbol: "ETH/USDT".to_string(),
                quantity: 10.0,
                avg_price: 2500.0,
                current_price: 3000.0,
                pnl: 5000.0,
                pnl_percentage: 20.0,
            },
        ];
        
        let total_value: f64 = positions.iter().map(|p| p.current_price * p.quantity).sum();
        let total_pnl: f64 = positions.iter().map(|p| p.pnl).sum();
        let total_cost: f64 = positions.iter().map(|p| p.avg_price * p.quantity).sum();
        let total_pnl_percentage = (total_pnl / total_cost) * 100.0;
        
        assert_eq!(total_value, 55000.0); // 0.5 * 50000 + 10 * 3000
        assert_eq!(total_pnl, 7500.0);
        assert!((total_pnl_percentage - 17.65).abs() < 0.1);
    }

    #[test]
    fn test_empty_portfolio() {
        let positions: Vec<PositionResponse> = vec![];
        
        let total_value: f64 = positions.iter().map(|p| p.current_price * p.quantity).sum();
        let total_pnl: f64 = positions.iter().map(|p| p.pnl).sum();
        
        assert_eq!(total_value, 0.0);
        assert_eq!(total_pnl, 0.0);
    }
}
