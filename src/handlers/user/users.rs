use axum::extract::Path;
use axum::response::Response;
use axum::Extension;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::cookie::Cookie;
use axum::http::{header, HeaderMap};

use chrono::Duration;
use sqlx::Row;

use crate::jwt::Claims;
use crate::{
    db::DatabaseConnection,
    error::{ErrorMessage, HttpError},
    jwt::Token,
};

use super::{compare, hash_password, User};

pub async fn register(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(user): Json<User>,
) -> Result<&'static str, HttpError> {

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
) -> Result<Response, HttpError> {
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
    
    let access = Token::new_token(row.get("user_id"), "access_secret", 1)?;
    let refresh = Token::new_token(row.get("user_id"), "refresh_secret", 3)?;

    let expiration_duration = Duration::minutes(3);

    let cookie_duration = time::Duration::seconds(expiration_duration.num_seconds() as i64).abs();
    let cookie = Cookie::build(("refresh_token", refresh.token))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    let mut headers = HeaderMap::new();

    headers.append(
        header::SET_COOKIE,
        cookie.to_string().parse().unwrap(), 
    );

    let response = Json(access);

    let mut response = response.into_response();
        
    response
        .headers_mut()
        .extend(headers);

    Ok(response)
}

pub async fn logout(Json(body): Json<Token>) -> Result<Json<String>, HttpError> {
    body.validate_token("access_secret")
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    Token::create_secret("access_secret");
    Token::create_secret("refresh_secret");

    Ok(Json("User Has Been Logged Off".to_owned()))
}

pub async fn profile(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>, 
    Json(body): Json<Token>
) -> Result<Json<String>, HttpError> {
    body.validate_token("access_secret")
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    let row = sqlx::query("SELECT * FROM users WHERE user_id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| HttpError::unique_violation(e.to_string()))?;

    let body_username: String = row.get("username");

    Ok(Json(body_username.to_owned()))
}

pub async fn new_access(Extension(claims): Extension<Claims>) -> Result<Json<Token>, HttpError> {
    let token = Token::new_token(claims.sub, "access_secret", 1)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(token))
}
