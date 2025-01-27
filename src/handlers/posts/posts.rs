use axum::Json;
use chrono::Utc;
use serde::Serialize;
use sqlx::postgres::PgQueryResult;

use crate::{db::DatabaseConnection, error::HttpError, jwt::Token};

use super::Body;

pub async fn get_post() -> &'static str {
    "post"
}

pub async fn get_posts(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Json(body): Json<Token>
) -> Result<String, HttpError> {
    body.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    let rows = sqlx::query("SELECT * FROM posts LIMIT 10")
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    println!("{:#?}", rows);

    Ok("ssss".to_owned())
}

pub async fn create_post(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Json(body): Json<Body>
) -> Result<String, HttpError> {
    let claims = Token{token: body.token}.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    let current_date = Utc::now().timestamp();

    sqlx::query("INSERT INTO posts (user_id, title, body, publication_date) VALUES($1, $2, $3, $4)")
        .bind(claims.sub)
        .bind(body.post.title)
        .bind(body.post.body)
        .bind(current_date)
        .execute(&mut *conn).await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok("post created".to_owned())
}

pub async fn modify_post() -> &'static str {
    "modify_post"
}

pub async fn delete_post() -> &'static str {
    "delete_post"
}