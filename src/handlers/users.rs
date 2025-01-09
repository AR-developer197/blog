use std::env;

use axum::Json;
use bcrypt::{hash, verify};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use jsonwebtoken::{self, decode, encode, EncodingKey, Header, DecodingKey, Validation, Algorithm};
use uuid::Uuid;

use crate::db::{internal_error, DatabaseConnection};

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Token {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
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

pub async fn login(DatabaseConnection(mut conn): DatabaseConnection, Json(user): Json<User>) -> Result<Json<(Token, Token)>, (StatusCode, String)> {
    let row = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(user.username)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let password: String = row.get("password"); 
    let id: i32 = row.get("user_id");

    match verify(user.password, &password) {
        Ok(true) => {
            create_secret();
            let tokens = new_tokens(id);
            

            Ok(Json(tokens))
        }
        _ => return Err((StatusCode::BAD_REQUEST, "wrong password".to_owned())),
    }
}

pub async fn logout(Json(body): Json<Token>) -> Result<String, (StatusCode, String)> {
    match validate_token(body.token) {
        Ok(_) => {
            create_secret();

            Ok("logout".to_owned())
        },
        Err(err) => Err(err),
    }
}

pub async fn profile(Json(body): Json<Token>) -> Result<String, (StatusCode, String)> {
    match validate_token(body.token) {
        Ok(_) => Ok("profile".to_owned()),
        Err(err) => Err(err),
    }
}

pub async fn new_access(Json(body): Json<Token>) -> Json<Token> {
    let key = &DecodingKey::from_secret(env::var("refresh_secret").unwrap().as_bytes());
    let validation = &Validation::new(Algorithm::HS256);

    match decode::<Claims>(&body.token, key, validation) {
        Ok(claims) => {
            let current_time = chrono::Utc::now();
            let access_exp = (current_time + chrono::Duration::minutes(15)).timestamp() as u64;
            let access_secret = Uuid::new_v4();
            env::set_var("access_secret", access_secret.to_string());

            let access_token = match encode(&Header::default(), &claims.claims,  &EncodingKey::from_secret(std::env::var("access_secret").unwrap().as_bytes())) {
                Ok(token) => Json(Token{ token }),
                Err(_) => panic!(),
            };

            access_token
        },
        Err(_) => panic!(),
    }
}

fn create_secret() {
    let access_secret = Uuid::new_v4();
    env::set_var("access_secret", access_secret.to_string());

    let refresh_secret = Uuid::new_v4();
    env::set_var("refresh_secret", refresh_secret.to_string());
}

fn validate_token(token: String) -> Result<String, (StatusCode, String)> {
    let key = &DecodingKey::from_secret(env::var("access_secret").unwrap().as_bytes());
    let validation = &Validation::new(Algorithm::HS256);

    match decode::<Claims>(&token, key, validation) {
        Ok(_) => Ok("profile".to_owned()),
        Err(_) => Err((StatusCode::FORBIDDEN, "invalid token".to_owned())),
    }
}

fn new_tokens(id: i32) -> (Token, Token) {
    let current_time = chrono::Utc::now();
    let refresh_exp = (current_time + chrono::Duration::minutes(3)).timestamp() as u64;

    let access_exp = (current_time + chrono::Duration::minutes(1)).timestamp() as u64;

    let refresh_claims = Claims { id, exp: refresh_exp };

    let access_claims = Claims { id, exp: access_exp };

    let access_token = match encode(&Header::default(), &access_claims,  &EncodingKey::from_secret(std::env::var("access_secret").unwrap().as_bytes())) {
        Ok(token) => token,
        Err(_) => panic!(),
    }; 

    let refresh_token = match encode(&Header::default(), &refresh_claims,  &EncodingKey::from_secret(std::env::var("refresh_secret").unwrap().as_bytes())) {
        Ok(token) => token,
        Err(_) => panic!(),
    }; 

    (Token { token: access_token}, Token {token: refresh_token})
}