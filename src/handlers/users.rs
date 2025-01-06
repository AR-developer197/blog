use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{self, encode, Header};
use uuid::Uuid;

use crate::db::{internal_error, DatabaseConnection};

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

pub async fn register(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Json(user): Json<User>
) -> Result<&'static str, (StatusCode, String)> {
    let password = hash(user.password, 8).unwrap();
    sqlx::query("INSERT INTO users (username, password) VALUES($1, $2)")
        .bind(user.username)
        .bind(password)
        .execute(&mut *conn)
        .await
        .map_err(internal_error)?;

    Ok("the user has been registered")
}

pub async fn login() -> &'static str {
    "login"
}

pub async fn profile() -> &'static str {
    "profile"
}

fn new_token() {
    let current_time = chrono::Utc::now();
    let exp_date = current_time + chrono::Duration::hours(1);

    let claims = Claims {
        exp: exp_date.timestamp() as u64
    };

}