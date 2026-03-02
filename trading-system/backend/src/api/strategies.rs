//! Strategy API Routes

use crate::models::strategy::{Strategy, StrategyType, validate_parameters};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Strategy state
pub struct StrategyState {
    pub strategies: RwLock<HashMap<String, Strategy>>,
}

impl StrategyState {
    pub fn new() -> Self {
        Self {
            strategies: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for StrategyState {
    fn default() -> Self {
        Self::new()
    }
}

/// API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

/// Create strategy request
#[derive(Debug, Deserialize)]
pub struct CreateStrategyRequest {
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: String,
    pub parameters: serde_json::Value,
}

/// Update strategy request
#[derive(Debug, Deserialize)]
pub struct UpdateStrategyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub strategy_type: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

/// List strategies query
#[derive(Debug, Deserialize)]
pub struct ListStrategiesQuery {
    pub enabled: Option<bool>,
    pub strategy_type: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Create a new strategy
async fn create_strategy(
    State(state): State<Arc<StrategyState>>,
    Json(req): Json<CreateStrategyRequest>,
) -> Result<Json<ApiResponse<Strategy>>, StatusCode> {
    // Validate name
    if req.name.is_empty() {
        return Ok(Json(ApiResponse::error("Name is required")));
    }
    if req.name.len() > 100 {
        return Ok(Json(ApiResponse::error("Name must be less than 100 characters")));
    }
    
    // Parse strategy type
    let strategy_type = StrategyType::from_str(&req.strategy_type)
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    
    // Validate parameters
    if let Err(e) = validate_parameters(&strategy_type, &req.parameters) {
        return Ok(Json(ApiResponse::error(&e)));
    }
    
    // Create strategy (using default user_id for now)
    let strategy = Strategy::new(
        "default_user".to_string(),
        req.name,
        req.description.unwrap_or_default(),
        strategy_type,
        req.parameters,
    );
    
    let id = strategy.id.clone();
    
    // Store strategy
    state.strategies.write().await.insert(id.clone(), strategy.clone());
    
    Ok(Json(ApiResponse::success(strategy)))
}

/// List strategies
async fn list_strategies(
    State(state): State<Arc<StrategyState>>,
    axum::extract::Query(query): axum::extract::Query<ListStrategiesQuery>,
) -> Json<ApiResponse<Vec<Strategy>>> {
    let all_strategies = state.strategies.read().await;
    let mut strategies: Vec<Strategy> = all_strategies.values().cloned().collect();
    
    // Filter by enabled
    if let Some(enabled) = query.enabled {
        strategies.retain(|s| s.enabled == enabled);
    }
    
    // Filter by type
    if let Some(ref type_filter) = query.strategy_type {
        strategies.retain(|s| s.strategy_type.to_string() == *type_filter);
    }
    
    // Sort by created_at descending
    strategies.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    // Apply pagination
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(50);
    strategies = strategies.into_iter().skip(offset).take(limit).collect();
    
    Json(ApiResponse::success(strategies))
}

/// Get a single strategy
async fn get_strategy(
    State(state): State<Arc<StrategyState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Strategy>>, StatusCode> {
    let strategies = state.strategies.read().await;
    
    if let Some(strategy) = strategies.get(&id) {
        Ok(Json(ApiResponse::success(strategy.clone())))
    } else {
        Ok(Json(ApiResponse::error("Strategy not found")))
    }
}

/// Update a strategy
async fn update_strategy(
    State(state): State<Arc<StrategyState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateStrategyRequest>,
) -> Result<Json<ApiResponse<Strategy>>, StatusCode> {
    let mut strategies = state.strategies.write().await;
    
    if let Some(strategy) = strategies.get_mut(&id) {
        // Update fields if provided
        if let Some(name) = req.name {
            if name.is_empty() {
                return Ok(Json(ApiResponse::error("Name cannot be empty")));
            }
            strategy.name = name;
        }
        
        if let Some(description) = req.description {
            strategy.description = description;
        }
        
        if let Some(type_str) = req.strategy_type {
            let strategy_type = StrategyType::from_str(&type_str)
                .ok_or_else(|| StatusCode::BAD_REQUEST)?;
            strategy.strategy_type = strategy_type;
        }
        
        if let Some(parameters) = req.parameters {
            // Validate new parameters
            if let Err(e) = validate_parameters(&strategy.strategy_type, &parameters) {
                return Ok(Json(ApiResponse::error(&e)));
            }
            strategy.parameters = parameters;
        }
        
        if let Some(enabled) = req.enabled {
            strategy.enabled = enabled;
        }
        
        strategy.updated_at = Utc::now();
        
        Ok(Json(ApiResponse::success(strategy.clone())))
    } else {
        Ok(Json(ApiResponse::error("Strategy not found")))
    }
}

/// Delete a strategy
async fn delete_strategy(
    State(state): State<Arc<StrategyState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let mut strategies = state.strategies.write().await;
    
    if strategies.remove(&id).is_some() {
        Ok(Json(ApiResponse::success(())))
    } else {
        Ok(Json(ApiResponse::error("Strategy not found")))
    }
}

pub fn routes(state: Arc<StrategyState>) -> Router {
    Router::new()
        .route("/api/strategies", get(list_strategies))
        .route("/api/strategies", post(create_strategy))
        .route("/api/strategies/:id", get(get_strategy))
        .route("/api/strategies/:id", put(update_strategy))
        .route("/api/strategies/:id", delete(delete_strategy))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strategy_type_parsing() {
        assert!(matches!(
            StrategyType::from_str("momentum"),
            Some(StrategyType::Momentum)
        ));
        assert!(matches!(
            StrategyType::from_str("mean_reversion"),
            Some(StrategyType::MeanReversion)
        ));
        assert_eq!(StrategyType::from_str("invalid"), None);
    }
    
    #[test]
    fn test_validate_momentum_parameters() {
        let params = serde_json::json!({"period": 20});
        assert!(validate_parameters(&StrategyType::Momentum, &params).is_ok());
        
        let params = serde_json::json!({"period": 0});
        assert!(validate_parameters(&StrategyType::Momentum, &params).is_err());
    }
}
