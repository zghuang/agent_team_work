use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod api;
mod db;
mod models;
mod exchange;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize exchange service
    let exchange_service = exchange::ExchangeService::new();
    
    // Start WebSocket for common symbols
    let exchange_for_ws = exchange_service.clone();
    tokio::spawn(async move {
        exchange_for_ws.start_websocket(vec![
            "btcusdt".to_string(),
            "ethusdt".to_string(),
            "bnbusdt".to_string(),
            "solusdt".to_string(),
            "xrpusdt".to_string(),
        ]).await;
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(api::health))
        .route("/api/users", get(db::handlers::list_users))
        .route("/api/strategies", get(db::handlers::list_strategies))
        .route("/api/strategies/:id", get(db::handlers::get_strategy))
        .route("/api/messages", get(db::handlers::list_messages))
        .route("/api/messages", post(db::handlers::create_message))
        .route("/api/trades", get(db::handlers::list_trades))
        .route("/api/orders", get(api::list_orders))
        .route("/api/orders", post(api::create_order))
        .route("/api/v1/messages", get(api::list_messages))
        .route("/api/v1/messages/fetch", get(api::fetch_crypto_prices))
        // Exchange API routes
        .route("/api/v1/prices", get(exchange::api::get_prices))
        .route("/api/v1/price", get(exchange::api::get_price))
        .with_state(exchange_service)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Trading Backend API v1.0"
}
