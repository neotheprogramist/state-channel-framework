use crate::prove::ProveError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Encodes a JWT token with the provided subject and expiration time.
///
/// # Parameters
///
/// - `sub`: The subject (usually a user identifier) to be included in the JWT claims.
/// - `exp`: The expiration time (in seconds since the UNIX epoch) for the JWT token.
///
/// # Returns
///
/// Returns a JWT token as a string if successful, or a `ProveError::InternalServerError` if encoding fails.
pub fn encode_jwt(sub: &str, exp: usize) -> Result<String, ProveError> {
    let secret = env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    let claims = Claims {
        sub: sub.to_owned(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ProveError::InternalServerError(format!("JWT generation failed: {}", e)))
}

/// Decodes a JWT token and verifies its signature.
///
/// # Parameters
///
/// - `jwt`: The JWT token to be decoded and verified.
///
/// # Returns
///
/// Returns the decoded JWT token data if verification is successful, or a `ProveError::InternalServerError` if decoding fails.
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, ProveError> {
    let secret = env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| ProveError::InternalServerError(format!("JWT generation failed: {}", e)))
}
