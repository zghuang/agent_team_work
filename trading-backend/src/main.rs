use axum::{
    routing::{get, post},
    Router,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod api;
mod db;
mod models;
mod exchange;
mod auth;

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
        // WebSocket endpoint for real-time market data
        .route("/ws/market", get(ws_market_handler))
        // Auth routes
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
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

/// WebSocket handler for real-time market data
async fn ws_market_handler(
    ws: WebSocketUpgrade,
    axum::State(state): axum::State<exchange::ExchangeService>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: exchange::ExchangeService) {
    use futures_util::{SinkExt, StreamExt};
    
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial prices
    let symbols = ["btcusdt", "ethusdt", "bnbusdt", "solusdt", "xrpusdt"];
    if let Ok(prices) = state.get_prices(&symbols).await {
        if let Ok(json) = serde_json::to_string(&prices) {
            let _ = sender.send(axum::extract::ws::Message::Text(json.into())).await;
        }
    }

    // Subscribe to price updates and send to client
    let state_clone = state.clone();
    let mut sender_clone = sender;
    
    tokio::spawn(async move {
        // Send price updates every second
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            
            let symbols = ["BTCUSDT", "ETHUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT"];
            let mut updates = Vec::new();
            
            for symbol in symbols {
                if let Some(ticker) = state_clone.get_cached_price(symbol).await {
                    updates.push(ticker);
                }
            }
            
            if !updates.is_empty() {
                if let Ok(json) = serde_json::to_string(&updates) {
                    let _ = sender_clone.send(axum::extract::ws::Message::Text(json.into())).await;
                }
            }
        }
    });

    // Handle incoming messages (e.g., subscribe to specific symbols)
    while let Some(msg) = receiver.next().await {
        if let Ok(axum::extract::ws::Message::Text(text)) = msg {
            // Handle subscription requests if needed
            tracing::debug!("Received WebSocket message: {}", text);
        }
    }
}
