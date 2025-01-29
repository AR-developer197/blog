use axum::{extract::Path, Json};
use chrono::Utc;

use crate::{db::DatabaseConnection, error::HttpError, handlers::posts::Post, jwt::Token};

use super::Body;

pub async fn get_post(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Path(id): Path<i32>,
    Json(body): Json<Token>
) -> Result<Json<Post>, HttpError> {
    body.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    let row: Post = sqlx::query_as( "SELECT * FROM posts WHERE post_id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(row))
}

pub async fn get_posts(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Json(body): Json<Token>
) -> Result<Json<Vec<Post>>, HttpError> {
    body.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    let rows: Vec<Post> = sqlx::query_as( "SELECT * FROM posts LIMIT 10")
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(rows))
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

pub async fn delete_post(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Path(id): Path<i32>,
    Json(body): Json<Token>
) -> Result<Json<String>, HttpError> {
    let claims = Token{token: body.token}.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    let row: Post = sqlx::query_as( "SELECT * FROM posts WHERE post_id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    if row.user_id.unwrap() != claims.sub {
        return Err(HttpError::forbidden("yu cant".to_owned()));
    }

    sqlx::query( "DELETE FROM posts WHERE post_id = $1")
        .bind(id)
        .execute(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json("Post Deleted".to_owned()))
}