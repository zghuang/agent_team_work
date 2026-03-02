//! Authentication API Routes

use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bcrypt::{hash, verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Application state for auth
pub struct AuthState {
    pub users: RwLock<HashMap<String, User>>,
    pub jwt_secret: String,
}

impl AuthState {
    pub fn new(secret: String) -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            jwt_secret: secret,
        }
    }
}

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub username: String,
    pub exp: i64,
    pub iat: i64,
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

/// Register a new user
async fn register(
    State(state): State<Arc<AuthState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    // Validate input
    if req.username.len() < 3 {
        return Ok(Json(ApiResponse::error("Username must be at least 3 characters")));
    }
    if req.password.len() < 6 {
        return Ok(Json(ApiResponse::error("Password must be at least 6 characters")));
    }
    if !req.email.contains('@') {
        return Ok(Json(ApiResponse::error("Invalid email format")));
    }

    let users = state.users.read().await;
    
    // Check if username exists
    if users.values().any(|u| u.username == req.username) {
        return Ok(Json(ApiResponse::error("Username already exists")));
    }
    
    // Check if email exists
    if users.values().any(|u| u.email == req.email) {
        return Ok(Json(ApiResponse::error("Email already exists")));
    }
    drop(users);

    // Hash password
    let password_hash = hash(&req.password, 10).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create user
    let user = User::new(req.username, req.email, password_hash);
    let user_response: UserResponse = user.clone().into();
    
    // Generate JWT token
    let token = generate_token(&state, &user.id, &user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store user
    let mut users = state.users.write().await;
    users.insert(user.id.clone(), user);

    Ok(Json(ApiResponse::success(AuthResponse { token, user: user_response })))
}

/// Login user
async fn login(
    State(state): State<Arc<AuthState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    let users = state.users.read().await;
    
    // Find user by username
    let user = users.values().find(|u| u.username == req.username);
    
    if let Some(user) = user {
        // Verify password
        if verify(&req.password, &user.password_hash).unwrap_or(false) {
            let token = generate_token(&state, &user.id, &user.username)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let user_response: UserResponse = user.clone().into();
            return Ok(Json(ApiResponse::success(AuthResponse { token, user: user_response })));
        }
    }
    
    Ok(Json(ApiResponse::error("Invalid username or password")))
}

/// Get current user info
async fn me(
    State(state): State<Arc<AuthState>>,
    Json(claims): Json<Claims>,
) -> Json<ApiResponse<UserResponse>> {
    let users = state.users.read().await;
    
    if let Some(user) = users.get(&claims.sub) {
        let user_response: UserResponse = user.clone().into();
        Json(ApiResponse::success(user_response))
    } else {
        Json(ApiResponse::error("User not found"))
    }
}

/// Generate JWT token
fn generate_token(state: &AuthState, user_id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(24);
    
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
}

/// Decode and validate JWT token
pub fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub fn routes(state: Arc<AuthState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", get(me))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_and_decode_token() {
        let state = AuthState::new("test_secret".to_string());
        let token = generate_token(&state, "user123", "testuser").unwrap();
        let claims = decode_token(&token, "test_secret").unwrap();
        
        assert_eq!(claims.claims.sub, "user123");
        assert_eq!(claims.claims.username, "testuser");
    }
}
