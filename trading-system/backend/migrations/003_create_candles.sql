-- Create candles (OHLCV) table for market data
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

CREATE INDEX idx_candles_symbol_time ON candles(symbol, timestamp DESC);
CREATE INDEX idx_candles_interval ON candles(interval);
