use axum::{routing::{get, post, put}, Router};
use bcrypt::{hash, verify};
use users::{login, logout, new_access, profile, register};

use crate::error::{ErrorMessage, HttpError};

mod users;

pub fn create_user_routes() -> Router<sqlx::Pool<sqlx::Postgres>>{
    let user_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", put(new_access))
        .route("/profile/{id}", get(profile));

    user_router
}

pub fn hash_password(password: String) -> Result<String, HttpError> {

    if password.trim().is_empty() {
        return Err(HttpError::unique_violation(ErrorMessage::EmptyPassword.to_string()));
    }
    
    let password = hash(password, 8).map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(password)
}

pub fn compare(password: String, hash: String) -> Result<bool, HttpError> {

    if password.trim().is_empty() {
        return Err(HttpError::unique_violation(ErrorMessage::EmptyPassword.to_string()));
    }

    let verify_password =
        verify(password, &hash).map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(verify_password)
}