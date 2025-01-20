use std::{collections::HashSet, env};

use chrono::{Duration, Utc};
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
    pub sub: String,
    pub exp: usize,
}

impl Token {
    pub fn create_secret(env_var_name: &str) {
        let secret = Uuid::new_v4();
        env::set_var(env_var_name, secret.to_string());
    }
    
    pub fn validate_token(&self, env_secret_name: &str) -> Result<Claims, HttpError> {
        let secret = env::var(env_secret_name).unwrap();
        let key = secret.as_ref();
        let key = &DecodingKey::from_secret(key);

        let validation = &mut Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(&self.token, key, validation)
            .map_err(|e| HttpError::unauthorized(e.to_string()))?;

        Ok(token_data.claims)
    }
    
    pub fn new_token(sub: String, env_secret_name: &str, exp: i64) -> Result<Token, HttpError> {
        let now = Utc::now();
        let exp = (now + Duration::minutes(exp)).timestamp() as usize;
        let claims = Claims { sub, exp };

        Token::create_secret(env_secret_name);       
     
        let token = encode(
            &Header::default(), 
            &claims,  
            &EncodingKey::from_secret(env::var(env_secret_name).unwrap().as_ref())
        )
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        Ok(Token { token })
    }
}