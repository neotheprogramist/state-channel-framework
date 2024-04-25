use std::env;
use crate::prove::ProveError;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode,encode,EncodingKey,Header,Validation,DecodingKey,TokenData};

#[derive(Serialize,Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn encode_jwt(sub: &str, exp: usize) -> Result<String,ProveError> {
    let secret = env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    let claims = Claims {
        sub: sub.to_owned(),
        exp,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
    .map_err(|e| ProveError::InternalServerError(format!("JWT generation failed: {}", e)))
}

pub fn decode_jwt(jwt:String) -> Result<TokenData<Claims>, ProveError> {
    let secret = env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    decode(&jwt, &DecodingKey::from_secret(secret.as_ref()),&Validation::default())
    .map_err(|e| ProveError::InternalServerError(format!("JWT generation failed: {}", e)))
}