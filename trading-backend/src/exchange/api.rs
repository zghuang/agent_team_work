use crate::exchange::ExchangeService;
use axum::{
    extract::Query,
    response::Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PriceQuery {
    symbol: Option<String>,
}

pub async fn get_prices(
    Query(query): Query<PriceQuery>,
    State(exchange): State<ExchangeService>,
) -> Json<Vec<crate::exchange::Ticker>> {
    match query.symbol {
        Some(symbol) => {
            if let Some(ticker) = exchange.get_cached_price(&symbol).await {
                Json(vec![ticker])
            } else {
                let prices = exchange.get_prices(&[&symbol]).await;
                Json(prices)
            }
        }
        None => {
            // Return common crypto prices
            let symbols = ["BTCUSDT", "ETHUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT"];
            let prices = exchange.get_prices(&symbols).await;
            Json(prices)
        }
    }
}

pub async fn get_price(
    Query(query): Query<PriceQuery>,
    State(exchange): State<ExchangeService>,
) -> Json<Option<crate::exchange::Ticker>> {
    if let Some(symbol) = query.symbol {
        if let Some(ticker) = exchange.get_cached_price(&symbol).await {
            Json(Some(ticker))
        } else {
            match exchange.get_price(&symbol).await {
                Ok(ticker) => Json(Some(ticker)),
                Err(_) => Json(None),
            }
        }
    } else {
        Json(None)
    }
}
