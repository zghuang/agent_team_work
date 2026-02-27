use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::Json,
};
use sqlx::Pool;
use sqlx::Postgres;

use crate::models::{Message, Strategy, Trade, User};

pub type DbPool = Pool<Postgres>;

// User handlers
pub async fn list_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

// Strategy handlers
pub async fn list_strategies(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Strategy>>, StatusCode> {
    let strategies = sqlx::query_as::<_, Strategy>("SELECT * FROM strategies LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(strategies))
}

pub async fn get_strategy(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Strategy>, StatusCode> {
    let strategy = sqlx::query_as::<_, Strategy>("SELECT * FROM strategies WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(strategy))
}

// Message handlers
pub async fn list_messages(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let messages = sqlx::query_as::<_, Message>("SELECT * FROM messages ORDER BY created_at DESC LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(messages))
}

pub async fn create_message(
    State(pool): State<DbPool>,
    Json(payload): Json<Create>,
) -> ResultMessageRequest<Json<Message>, StatusCode> {
    let message = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (title, content, message_type, user_id) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.message_type)
    .bind(payload.user_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(message))
}

// Trade handlers
pub async fn list_trades(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Trade>>, StatusCode> {
    let trades = sqlx::query_as::<_, Trade>("SELECT * FROM trades ORDER BY created_at DESC LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(trades))
}

#[derive(serde::Deserialize)]
pub struct CreateMessageRequest {
    pub title: String,
    pub content: String,
    pub message_type: String,
    pub user_id: i64,
}
