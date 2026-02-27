//! Trading Strategies

use crate::models::{Candle, OrderSide, Signal, Symbol};
use async_trait::async_trait;
use serde::Deserialize;

pub mod sma_crossover {
    use super::*;

    /// Simple Moving Average Crossover Strategy
    pub struct SmaCrossoverStrategy {
        short_period: usize,
        long_period: usize,
    }

    impl SmaCrossoverStrategy {
        pub fn new(short_period: usize, long_period: usize) -> Self {
            Self {
                short_period,
                long_period,
            }
        }

        fn calculate_sma(candles: &[Candle], period: usize) -> Option<f64> {
            if candles.len() < period {
                return None;
            }
            let sum: f64 = candles
                .iter()
                .rev()
                .take(period)
                .map(|c| c.close)
                .sum();
            Some(sum / period as f64)
        }
    }

    #[async_trait]
    impl super::Strategy for SmaCrossoverStrategy {
        fn name(&self) -> &str {
            "sma_crossover"
        }

        async fn analyze(&self, candles: &[Candle]) -> Option<Signal> {
            if candles.len() < self.long_period + 1 {
                return None;
            }

            let short_sma = Self::calculate_sma(candles, self.short_period)?;
            let long_sma = Self::calculate_sma(candles, self.long_period)?;

            // Previous SMA values
            let prev_short = Self::calculate_sma(&candles[1..], self.short_period)?;
            let prev_long = Self::calculate_sma(&candles[1..], self.long_period)?;

            // Golden cross (bullish)
            if prev_short <= prev_long && short_sma > long_sma {
                return Some(Signal::new(
                    candles[0].symbol.clone(),
                    OrderSide::Buy,
                    0.8,
                    &format!(
                        "Golden cross: SMA{}={:.2} > SMA{}={:.2}",
                        self.short_period, short_sma, self.long_period, long_sma
                    ),
                ));
            }

            // Death cross (bearish)
            if prev_short >= prev_long && short_sma < long_sma {
                return Some(Signal::new(
                    candles[0].symbol.clone(),
                    OrderSide::Sell,
                    0.8,
                    &format!(
                        "Death cross: SMA{}={:.2} < SMA{}={:.2}",
                        self.short_period, short_sma, self.long_period, long_sma
                    ),
                ));
            }

            None
        }

        fn set_params(&mut self, params: serde_json::Value) {
            if let (Some(short), Some(long)) = (
                params.get("short_period").and_then(|v| v.as_u64()),
                params.get("long_period").and_then(|v| v.as_u64()),
            ) {
                self.short_period = short as usize;
                self.long_period = long as usize;
            }
        }
    }
}

pub mod rsi_strategy {
    use super::*;

    /// RSI Strategy
    pub struct RsiStrategy {
        period: usize,
        oversold: f64,
        overbought: f64,
    }

    impl RsiStrategy {
        pub fn new(period: usize, oversold: f64, overbought: f64) -> Self {
            Self {
                period,
                oversold,
                overbought,
            }
        }

        fn calculate_rsi(candles: &[Candle], period: usize) -> Option<f64> {
            if candles.len() < period + 1 {
                return None;
            }

            let mut gains = 0.0;
            let mut losses = 0.0;

            for i in 0..period {
                let change = candles[i].close - candles[i + 1].close;
                if change > 0 {
                    gains += change;
                } else {
                    losses -= change;
                }
            }

            let avg_gain = gains / period as f64;
            let avg_loss = losses / period as f64;

            if avg_loss == 0.0 {
                return Some(100.0);
            }

            let rs = avg_gain / avg_loss;
            Some(100.0 - (100.0 / (1.0 + rs)))
        }
    }

    #[async_trait]
    impl super::Strategy for RsiStrategy {
        fn name(&self) -> &str {
            "rsi"
        }

        async fn analyze(&self, candles: &[Candle]) -> Option<Signal> {
            let rsi = Self::calculate_rsi(candles, self.period)?;

            // Oversold - buy signal
            if rsi < self.oversold {
                return Some(Signal::new(
                    candles[0].symbol.clone(),
                    OrderSide::Buy,
                    (self.oversold - rsi) / self.oversold,
                    &format!("RSI oversold: {:.2}", rsi),
                ));
            }

            // Overbought - sell signal
            if rsi > self.overbought {
                return Some(Signal::new(
                    candles[0].symbol.clone(),
                    OrderSide::Sell,
                    (rsi - self.overbought) / (100.0 - self.overbought),
                    &format!("RSI overbought: {:.2}", rsi),
                ));
            }

            None
        }

        fn set_params(&mut self, params: serde_json::Value) {
            if let Some(period) = params.get("period").and_then(|v| v.as_u64()) {
                self.period = period as usize;
            }
            if let Some(oversold) = params.get("oversold").and_then(|v| v.as_f64()) {
                self.oversold = oversold;
            }
            if let Some(overbought) = params.get("overbought").and_then(|v| v.as_f64()) {
                self.overbought = overbought;
            }
        }
    }
}
