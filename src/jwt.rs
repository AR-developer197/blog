use std::env;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{self, decode, encode, EncodingKey, Header, DecodingKey, Validation, Algorithm};
use uuid::Uuid;

use crate::error::HttpError;

#[derive(Deserialize, Debug, Serialize)]
pub struct Token {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub aud: String,
    pub exp: u64,
}

impl Token {
    pub fn create_secret(env_var_name: &str) {
        let secret = Uuid::new_v4();
        env::set_var(env_var_name, secret.to_string());
    }
    
    pub fn validate_token(&self, env_secret_name: &str) -> Result<Claims, HttpError> {
        let key = &DecodingKey::from_secret(env::var(env_secret_name).unwrap().as_bytes());
        let validation = &Validation::new(Algorithm::HS256);
    
        let token_data = decode::<Claims>(&self.token, key, validation)
            .map_err(|e| HttpError::unauthorized(e.to_string()))?;

        Ok(token_data.claims)
    }
    
    pub fn new_token(aud: String, env_secret_name: &str, exp: i64) -> Result<Token, HttpError> {
        let current_time= time::Duration::minutes(3).whole_seconds() as u64;
        
        let claims = Claims { aud, exp: exp.try_into().unwrap() };
     
        let token = encode(
            &Header::default(), 
            &claims,  
            &EncodingKey::from_secret(std::env::var(env_secret_name).unwrap().as_bytes())
        )
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        Ok(Token { token })
    }
}