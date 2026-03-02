//! Strategy Model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub strategy_type: StrategyType,
    pub parameters: serde_json::Value,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Strategy {
    pub fn new(user_id: String, name: String, description: String, strategy_type: StrategyType, parameters: serde_json::Value) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            name,
            description,
            strategy_type,
            parameters,
            enabled: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Strategy type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StrategyType {
    Momentum,
    MeanReversion,
    Grid,
    Arbitrage,
    Custom,
}

impl StrategyType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "momentum" => Some(StrategyType::Momentum),
            "mean_reversion" | "meanreversion" => Some(StrategyType::MeanReversion),
            "grid" => Some(StrategyType::Grid),
            "arbitrage" => Some(StrategyType::Arbitrage),
            "custom" => Some(StrategyType::Custom),
            _ => None,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            StrategyType::Momentum => "momentum".to_string(),
            StrategyType::MeanReversion => "mean_reversion".to_string(),
            StrategyType::Grid => "grid".to_string(),
            StrategyType::Arbitrage => "arbitrage".to_string(),
            StrategyType::Custom => "custom".to_string(),
        }
    }
}

/// Strategy parameters validation
pub fn validate_parameters(strategy_type: &StrategyType, params: &serde_json::Value) -> Result<(), String> {
    match strategy_type {
        StrategyType::Momentum => {
            // Validate momentum parameters
            if let Some(obj) = params.as_object() {
                if let Some(period) = obj.get("period") {
                    if let Some(p) = period.as_i64().or_else(|| period.as_f64().map(|f| f as i64)) {
                        if p <= 0 || p > 500 {
                            return Err("period must be between 1 and 500".to_string());
                        }
                    }
                }
            }
        }
        StrategyType::MeanReversion => {
            if let Some(obj) = params.as_object() {
                if let Some(rsi_period) = obj.get("rsi_period") {
                    if let Some(p) = rsi_period.as_i64().or_else(|| rsi_period.as_f64().map(|f| f as i64)) {
                        if p <= 0 || p > 100 {
                            return Err("rsi_period must be between 1 and 100".to_string());
                        }
                    }
                }
            }
        }
        StrategyType::Grid => {
            if let Some(obj) = params.as_object() {
                if let Some(grid_levels) = obj.get("grid_levels") {
                    if let Some(levels) = grid_levels.as_i64().or_else(|| grid_levels.as_f64().map(|f| f as i64)) {
                        if levels < 2 || levels > 100 {
                            return Err("grid_levels must be between 2 and 100".to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
