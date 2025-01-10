
use std::env;

use axum::Json;
use bcrypt::{hash, verify};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::Row;
use jsonwebtoken::{self, decode, encode, EncodingKey, Header, DecodingKey, Validation, Algorithm};
use uuid::Uuid;

use crate::{db::{internal_error, DatabaseConnection}, error::HttpError, jwt::{Claims, Token}};

#[derive(Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

pub async fn register(
    DatabaseConnection(mut conn): DatabaseConnection, 
    Json(user): Json<User>
) -> Result<&'static str, HttpError> {
    let password = hash(user.password, 8).unwrap();
    sqlx::query("INSERT INTO users (username, password) VALUES($1, $2)")
        .bind(user.username)
        .bind(password)
        .execute(&mut *conn)
        .await
        .map_err(|e| HttpError::server_error(e))?;

    Ok("the user has been registered")
}

pub async fn login(DatabaseConnection(mut conn): DatabaseConnection, Json(user): Json<User>) -> Result<Json<(Token, Token)>, (StatusCode, String)> {
    let row = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(user.username)
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)?;

    let password: String = row.get("password"); 

    match verify(user.password, &password) {
        Ok(true) => {
            Token::create_secret("access_secret");
            Token::create_secret("refresh_secret");
            let access = Token::new_token(row.get("username") ,"access_secret", 1);
            let refresh = Token::new_token(row.get("username"), "refresh_secret", 3);
            

            Ok(Json((access, refresh)))
        }
        _ => return Err((StatusCode::BAD_REQUEST, "wrong password".to_owned())),
    }
}

pub async fn logout(Json(body): Json<Token>) -> Result<String, (StatusCode, String)> {
    match Token::validate_token(body.token, "access_secrets") {
        Ok(_) => {
            Token::create_secret("access_secret");
            Token::create_secret("refresh_secret");

            Ok("logout".to_owned())
        },
        Err(err) => Err(err),
    }
}

pub async fn profile(Json(body): Json<Token>) -> Result<String, (StatusCode, String)> {
    match Token::validate_token(body.token, "access_secrets") {
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

            let access_token = match encode(&Header::default(), &Claims{aud: claims.claims.aud, exp: access_exp},  &EncodingKey::from_secret(std::env::var("access_secret").unwrap().as_bytes())) {
                Ok(token) => Json(Token{ token }),
                Err(_) => panic!(),
            };

            access_token
        },
        Err(_) => panic!(),
    }
}