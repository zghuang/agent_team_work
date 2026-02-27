-- Create positions table
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

CREATE INDEX idx_positions_symbol ON positions(symbol_base, symbol_quote, symbol_exchange);
