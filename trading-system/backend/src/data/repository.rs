//! Repository Pattern for Data Access

use crate::models::{Candle, Order, Position, Symbol};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{Row, FromRow};

/// Trade Repository
pub struct TradeRepository {
    pool: PgPool,
}

impl TradeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_order(&self, order: &Order) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO orders (id, symbol_base, symbol_quote, symbol_exchange, side, order_type, price, quantity, filled_quantity, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                filled_quantity = EXCLUDED.filled_quantity,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(&order.id)
        .bind(&order.symbol.base)
        .bind(&order.symbol.quote)
        .bind(&order.symbol.exchange)
        .bind(format!("{:?}", order.side).to_lowercase())
        .bind(format!("{:?}", order.order_type).to_lowercase())
        .bind(order.price)
        .bind(order.quantity)
        .bind(order.filled_quantity)
        .bind(format!("{:?}", order.status).to_lowercase())
        .bind(order.created_at)
        .bind(order.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_order(&self, id: &str) -> sqlx::Result<Option<Order>> {
        let row: Option<PgRow> = sqlx::query(
            "SELECT * FROM orders WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Order {
            id: r.get("id"),
            symbol: Symbol {
                base: r.get("symbol_base"),
                quote: r.get("symbol_quote"),
                exchange: r.get("symbol_exchange"),
            },
            side: /* parse from string */ crate::models::OrderSide::Buy,
            order_type: /* parse from string */ crate::models::OrderType::Market,
            price: r.get("price"),
            quantity: r.get("quantity"),
            filled_quantity: r.get("filled_quantity"),
            status: /* parse from string */ crate::models::OrderStatus::Open,
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }
}

/// Market Data Repository
pub struct MarketRepository {
    pool: PgPool,
}

impl MarketRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_candles(&self, candles: &[Candle]) -> sqlx::Result<()> {
        for candle in candles {
            sqlx::query(
                r#"
                INSERT INTO candles (symbol, open, high, low, close, volume, timestamp)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT (symbol, timestamp) DO NOTHING
                "#
            )
            .bind(&candle.symbol)
            .bind(candle.open)
            .bind(candle.high)
            .bind(candle.low)
            .bind(candle.close)
            .bind(candle.volume)
            .bind(candle.timestamp)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn get_candles(&self, symbol: &str, limit: i64) -> sqlx::Result<Vec<Candle>> {
        let rows = sqlx::query(
            "SELECT * FROM candles WHERE symbol = ? ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(symbol)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Candle {
                symbol: r.get("symbol"),
                open: r.get("open"),
                high: r.get("high"),
                low: r.get("low"),
                close: r.get("close"),
                volume: r.get("volume"),
                timestamp: r.get("timestamp"),
            })
            .collect())
    }
}

/// Position Repository
pub struct PositionRepository {
    pool: PgPool,
}

impl PositionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_position(&self, position: &Position) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO positions (symbol_base, symbol_quote, symbol_exchange, quantity, avg_price, current_price, unrealized_pnl, realized_pnl, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (symbol_base, symbol_quote, symbol_exchange) DO UPDATE SET
                quantity = EXCLUDED.quantity,
                avg_price = EXCLUDED.avg_price,
                current_price = EXCLUDED.current_price,
                unrealized_pnl = EXCLUDED.unrealized_pnl,
                realized_pnl = EXCLUDED.realized_pnl,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(&position.symbol.base)
        .bind(&position.symbol.quote)
        .bind(&position.symbol.exchange)
        .bind(position.quantity)
        .bind(position.avg_price)
        .bind(position.current_price)
        .bind(position.unrealized_pnl)
        .bind(position.realized_pnl)
        .bind(position.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_positions(&self) -> sqlx::Result<Vec<Position>> {
        let rows = sqlx::query("SELECT * FROM positions WHERE quantity > 0")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|r| Position {
                symbol: Symbol {
                    base: r.get("symbol_base"),
                    quote: r.get("symbol_quote"),
                    exchange: r.get("symbol_exchange"),
                },
                quantity: r.get("quantity"),
                avg_price: r.get("avg_price"),
                current_price: r.get("current_price"),
                unrealized_pnl: r.get("unrealized_pnl"),
                realized_pnl: r.get("realized_pnl"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }
}
