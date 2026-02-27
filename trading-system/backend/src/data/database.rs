//! Database Connection Module

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub mod migrations {
    use sqlx::postgres::PgPool;
    
    pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id VARCHAR(255) PRIMARY KEY,
                symbol_base VARCHAR(50) NOT NULL,
                symbol_quote VARCHAR(50) NOT NULL,
                symbol_exchange VARCHAR(50) NOT NULL,
                side VARCHAR(10) NOT NULL,
                order_type VARCHAR(20) NOT NULL,
                price DECIMAL(20, 8),
                quantity DECIMAL(20, 8) NOT NULL,
                filled_quantity DECIMAL(20, 8) DEFAULT 0,
                status VARCHAR(20) NOT NULL DEFAULT 'pending',
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            );
            "#
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS positions (
                id SERIAL PRIMARY KEY,
                symbol_base VARCHAR(50) NOT NULL,
                symbol_quote VARCHAR(50) NOT NULL,
                symbol_exchange VARCHAR(50) NOT NULL,
                quantity DECIMAL(20, 8) NOT NULL DEFAULT 0,
                avg_price DECIMAL(20, 8) NOT NULL DEFAULT 0,
                current_price DECIMAL(20, 8) NOT NULL DEFAULT 0,
                unrealized_pnl DECIMAL(20, 8) NOT NULL DEFAULT 0,
                realized_pnl DECIMAL(20, 8) NOT NULL DEFAULT 0,
                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                UNIQUE(symbol_base, symbol_quote, symbol_exchange)
            );
            "#
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS candles (
                id SERIAL PRIMARY KEY,
                symbol VARCHAR(50) NOT NULL,
                interval VARCHAR(10) NOT NULL,
                open DECIMAL(20, 8) NOT NULL,
                high DECIMAL(20, 8) NOT NULL,
                low DECIMAL(20, 8) NOT NULL,
                close DECIMAL(20, 8) NOT NULL,
                volume DECIMAL(20, 8) NOT NULL,
                timestamp TIMESTAMP NOT NULL,
                UNIQUE(symbol, interval, timestamp)
            );
            "#
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
}
