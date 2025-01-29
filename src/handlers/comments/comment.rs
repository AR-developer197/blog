use axum::{extract::Path, Json};

use crate::{db::DatabaseConnection, error::HttpError, jwt::Token};

use super::Body;

pub async fn create_comment(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Path(id): Path<i32>,
    Json(body): Json<Body>
) -> Result<String, HttpError> {
    let claims = Token{token: body.token}.validate_token("access_secret")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    sqlx::query("INSERT INTO comments (post_id, user_id, body) VALUES($1, $2, $3)")
        .bind(id)
        .bind(claims.sub)
        .bind(body.comment.body)
        .execute(&mut *conn).await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok("Comment created".to_owned())
}

pub async fn get_comments() -> &'static str {
    "sdess"
}

pub async fn delete_comments() -> &'static str {
    "sdess"
}