//! Trading System Backend
//! 
//! A Rust-based trading system for stocks and cryptocurrency.
//! 
//! ## Architecture
//! 
//! ```text
//! backend/
//! ├── src/
//! │   ├── api/        # HTTP handlers and routes
//! │   ├── models/     # Data structures
//! │   ├── services/   # Business logic
//! │   ├── utils/      # Utilities
//! │   └── main.rs     # Entry point
//! ```

pub mod api;
pub mod models;
pub mod services;
pub mod utils;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Health check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "trading-system"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Trading System Backend...");

    let bind = "127.0.0.1:8080";
    tracing::info!("Server binding to {}", bind);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health))
            .route("/health", web::get().to(health))
            // API routes will be mounted here
            // .service(api::v1::configure())
    })
    .bind(bind)?
    .run()
    .await
}
