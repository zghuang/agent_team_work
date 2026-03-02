//! Trading System Backend - Main Entry Point

mod api;
mod core;
mod data;
mod models;

use api::auth::AuthState;
use api::websocket::WsState;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    ws_state: WsState,
    auth_state: Arc<AuthState>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "trading_backend=info,tower_http=info".into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Trading System Backend...");

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build auth state with JWT secret (in production, load from config/env)
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_development_secret_change_in_production".to_string());
    let auth_state = Arc::new(AuthState::new(jwt_secret));

    // Build application state
    let state = AppState {
        ws_state: WsState::new(),
        auth_state,
    };

    // Build router
    let app = Router::new()
        .merge(api::health::routes())
        .merge(api::market::routes())
        .merge(api::websocket::routes())
        .merge(api::auth::routes(state.auth_state.clone()))
        .merge(api::portfolio::routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
