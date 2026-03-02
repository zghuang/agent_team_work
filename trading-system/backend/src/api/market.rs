//! Market Data API Routes

use axum::{
    extract::Query,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::models::{Candle, Symbol, Ticker};

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

#[derive(Deserialize)]
pub struct TickerRequest {
    pub base: String,
    pub quote: String,
    pub exchange: String,
}

#[derive(Deserialize)]
pub struct CandlesRequest {
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub interval: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct TickerQuery {
    pub symbol: Option<String>,
}

// Placeholder for market service - would be injected via state
async fn get_ticker_handler(
    axum::extract::Json(req): axum::extract::Json<TickerRequest>,
) -> axum::Json<ApiResponse<Ticker>> {
    // In production, this would use the MarketDataService
    let symbol = Symbol::new(&req.base, &req.quote, &req.exchange);
    
    // Return mock data for now
    let ticker = Ticker {
        symbol: symbol.to_string(),
        price: 50000.0,
        volume: 1000.0,
        timestamp: chrono::Utc::now(),
    };

    axum::Json(ApiResponse::success(ticker))
}

async fn get_ticker_by_symbol(
    Query(query): Query<TickerQuery>,
) -> axum::Json<ApiResponse<Vec<Ticker>>> {
    // Return common tickers
    let symbols = query.symbol.map(|s| vec![s]).unwrap_or_else(|| {
        vec!["BTC/USDT".to_string(), "ETH/USDT".to_string(), "SOL/USDT".to_string()]
    });
    
    let tickers: Vec<Ticker> = symbols
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split('/').collect();
            let (base, quote) = if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (s.to_string(), "USDT".to_string())
            };
            
            // Mock prices based on symbol
            let price = match base.to_uppercase().as_str() {
                "BTC" => 50000.0,
                "ETH" => 3000.0,
                "SOL" => 100.0,
                "BNB" => 350.0,
                "XRP" => 0.6,
                _ => 100.0,
            };
            
            Ticker {
                symbol: s.clone(),
                price,
                volume: 1000.0 + (price * 10.0),
                timestamp: chrono::Utc::now(),
            }
        })
        .collect();

    axum::Json(ApiResponse::success(tickers))
}

async fn get_candles_handler(
    axum::extract::Json(req): axum::extract::Json<CandlesRequest>,
) -> axum::Json<ApiResponse<Vec<Candle>>> {
    let interval = req.interval.unwrap_or_else(|| "1h".to_string());
    let limit = req.limit.unwrap_or(100);

    // Mock candles for demo
    let candles: Vec<Candle> = (0..limit)
        .map(|i| Candle {
            symbol: format!("{}/{}", req.base, req.quote),
            open: 50000.0 + i as f64 * 10.0,
            high: 50200.0 + i as f64 * 10.0,
            low: 49800.0 + i as f64 * 10.0,
            close: 50100.0 + i as f64 * 10.0,
            volume: 1000.0 + i as f64 * 100.0,
            timestamp: chrono::Utc::now() - chrono::Duration::hours(i as i64),
        })
        .collect();

    axum::Json(ApiResponse::success(candles))
}

pub fn routes() -> Router {
    Router::new()
        .route("/api/market/ticker", post(get_ticker_handler))
        .route("/api/market/ticker", get(get_ticker_by_symbol))
        .route("/api/market/candles", post(get_candles_handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<String>::error("error message");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("error message".to_string()));
    }

    #[test]
    fn test_symbol_parsing() {
        let symbol = "BTC/USDT".to_string();
        let parts: Vec<&str> = symbol.split('/').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "BTC");
        assert_eq!(parts[1], "USDT");
    }

    #[test]
    fn test_ticker_price_calculation() {
        let base_price = 50000.0;
        let volume = 1000.0;
        let estimated_volume_usd = base_price * volume;
        
        assert_eq!(estimated_volume_usd, 50_000_000.0);
    }
}
