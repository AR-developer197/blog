use std::env;

use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{self, decode, encode, EncodingKey, Header, DecodingKey, Validation, Algorithm};
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize)]
pub struct Token {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: u64,
}

impl Token {
    pub fn create_secret(env_var_name: &str) {
        let secret = Uuid::new_v4();
        env::set_var(env_var_name, secret.to_string());

    }
    
    pub fn validate_token(token: String, env_secret_name: &str) -> Result<String, (StatusCode, String)> {
        let key = &DecodingKey::from_secret(env::var(env_secret_name).unwrap().as_bytes());
        let validation = &Validation::new(Algorithm::HS256);
    
        match decode::<Claims>(&token, key, validation) {
            Ok(_) => Ok("token validated".to_owned()),
            Err(_) => Err((StatusCode::FORBIDDEN, "invalid token".to_owned())),
        }
    }
    
    pub fn new_token(aud: String, env_secret_name: &str, exp: i64) -> Token {
        let current_time = chrono::Utc::now();
        let exp = (current_time + chrono::Duration::minutes(exp)).timestamp() as u64;
        
        let claims = Claims { aud, exp };
     
    
        let token = match encode(&Header::default(), &claims,  &EncodingKey::from_secret(std::env::var(env_secret_name).unwrap().as_bytes())) {
            Ok(token) => token,
            Err(_) => panic!(),
        }; 
    
        Token { token }
    }
}