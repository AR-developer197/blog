use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::Row;

use crate::{
    db::DatabaseConnection,
    error::{ErrorMessage, HttpError},
    jwt::Token,
};

use validator::Validate;

use super::{compare, hash_password};

#[derive(Deserialize, Debug, Validate)]
pub struct User {
    #[validate(length(min = 6))]
    username: String,
    #[validate(length(min = 10, max = 60))]
    password: String,
}

pub async fn register(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(user): Json<User>,
) -> Result<&'static str, HttpError> {
    user.validate()
        .map_err(|e| HttpError::unique_violation(e.to_string()))?;

    let password = hash_password(user.password)
        .map_err(|e| HttpError::new(e.to_string(), e.into_response().status()))?;

    sqlx::query("INSERT INTO users (username, password) VALUES($1, $2)")
        .bind(user.username)
        .bind(password)
        .execute(&mut *conn)
        .await
        .map_err(|e| HttpError::unique_violation(e.to_string()))?;

    Ok("The User Has Been Registered")
}

pub async fn login(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(user): Json<User>,
) -> Result<Json<(Token, Token)>, HttpError> {
    let row = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(user.username)
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| HttpError::unique_violation(e.to_string()))?;

    let password: String = row.get("password");

    let verify_password = compare(user.password, password)
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    if !verify_password {
        return Err(HttpError::unauthorized(
            ErrorMessage::WrongPassword.to_string(),
        ));
    }

    Token::create_secret("access_secret");
    Token::create_secret("refresh_secret");
    let access = Token::new_token(row.get("username"), "access_secret", 1)?;
    let refresh = Token::new_token(row.get("username"), "refresh_secret", 3)?;

    Ok(Json((access, refresh)))
}

pub async fn logout(Json(body): Json<Token>) -> Result<Json<String>, HttpError> {
    body.validate_token("access_secrets")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    Token::create_secret("access_secret");
    Token::create_secret("refresh_secret");

    Ok(Json("User Has Been Logged Off".to_owned()))
}

pub async fn profile(Json(body): Json<Token>) -> Result<Json<String>, HttpError> {
    let claims = body
        .validate_token("access_secrets")
        .map_err(|e| HttpError::forbidden(e.to_string()))?;

    Ok(Json(claims.aud.to_owned()))
}

pub async fn new_access(Json(body): Json<Token>) -> Result<Json<Token>, HttpError> {
    let claims = body
        .validate_token("refresh_secret")
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    let token = Token::new_token(claims.aud, "access_secret", 1)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(token))
}