use crate::prove::ProveError;
use axum::{
    async_trait, extract::FromRequestParts, http::header::AUTHORIZATION, http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub fn encode_jwt(sub: &str, exp: usize) -> Result<String, ProveError> {
    let claims = Claims {
        sub: sub.to_owned(),
        exp,
    };
    encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|e| ProveError::InternalServerError(format!("JWT generation failed: {}", e)))
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ProveError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the 'Authorization' header
        let header_value = parts.headers.get(AUTHORIZATION).ok_or(ProveError::Auth(
            crate::prove::AuthError::MissingAuthorizationHeader,
        ))?;

        // Convert the header value to a string and validate it starts with "Bearer "
        let token_str = header_value
            .to_str()
            .map_err(|_| ProveError::Auth(crate::prove::AuthError::InvalidToken))?;
        if !token_str.starts_with("Bearer ") {
            return Err(ProveError::Auth(crate::prove::AuthError::Unauthorized));
        }

        let token = &token_str["Bearer ".len()..];

        let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| ProveError::Auth(crate::prove::AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sub: {}", self.sub)
    }
}
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
