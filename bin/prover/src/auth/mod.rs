use crate::prove::ProveError;
use crate::server::AppState;
use ed25519_dalek::{Signature, PublicKey}; // Ensure these are properly imported
use std::env;
use jsonwebtoken::{encode,EncodingKey,Header};
use crate::prove::models::{Nonce,GenerateNonceResponse,ValidateSignatureRequest,JWTResponse,GenerateNonceRequest};
use axum::{
  http::{self, HeaderValue,HeaderMap},
  extract::{State,Json,Query},
  response::IntoResponse
};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub const COOKIE_NAME: &str = "session_id";

pub async fn generate_nonce(
    State(state): State<AppState>,
    Query(params): Query<GenerateNonceRequest>,

) -> Result<Json<GenerateNonceResponse>, ProveError>{
  let env_var_name = "MESSAGE_EXPIRATION_TIME"; // Environment variable name
  let message_expiration_str = env::var(env_var_name)
  .expect("MESSAGE_EXPIRATION_TIME environment variable not found!");

  let message_expiration_time: usize = message_expiration_str
    .parse::<usize>().unwrap();


  if params.public_key.trim().is_empty() {
    return Err(ProveError::MissingPublicKey);
  }
  let nonce: Nonce = Nonce::new(32);
  let mut nonces: std::sync::MutexGuard<'_, std::collections::HashMap<String, String>> = state.nonces.lock().unwrap();
  let formatted_key = params.public_key.trim().to_lowercase();
  nonces.insert(formatted_key.clone(), nonce.clone().to_string());
  // Printing whether the nonce was found or not, along with the public key
  match nonces.get(&params.public_key) {
    Some(nonce) => println!("Nonce for public key {}: {}", &params.public_key, nonce),
    None => println!("No nonce found for public key: {}", &params.public_key),
  }    

  
  Ok(Json(GenerateNonceResponse {
      message:nonce.to_string(),
      expiration: message_expiration_time,
  }))
}


pub async fn validate_signature(
  State(state): State<AppState>,
  Json(payload): Json<ValidateSignatureRequest>
)-> Result<impl IntoResponse, ProveError>{ 
  let env_var_name = "SESSION_EXPIRATION_TIME"; // Environment variable name
  let message_expiration_str = env::var(env_var_name)
  .expect("SESSION_EXPIRATION_TIME environment variable not found!");

  let session_expiration_time: usize = message_expiration_str
    .parse::<usize>().unwrap();


  let nonces = state.nonces.lock().map_err(|_| ProveError::InternalServerError("Failed to lock state".to_string()))?;

  let user_nonce = nonces.get(&payload.public_key)
    .ok_or_else(|| ProveError::NotFound(format!("Nonce not found for the provided public key: {}", &payload.public_key)))?;

  let signature_valid = verify_signature(&payload.signature, &user_nonce, &payload.public_key);

  if !signature_valid {
    return Err(ProveError::Unauthorized("Invalid signature".to_string()));
  }

  let expiration = chrono::Utc::now() + chrono::Duration::seconds(session_expiration_time as i64);
  let token = generate_jwt(&payload.public_key, expiration.timestamp() as usize)
      .map_err(|_| ProveError::InternalServerError("JWT generation failed".to_string()))?;


  let cookie_value = format!("{}={}; HttpOnly; Secure; Path=/; Max-Age={}", COOKIE_NAME, token, session_expiration_time);
  let mut headers = HeaderMap::new();
  headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie_value)
      .map_err(|_| ProveError::InternalServerError("Failed to set cookie header".to_string()))?);

  Ok((
      headers,
      Json(JWTResponse {
          session_id: token,  // Use the JWT as the session identifier
          expiration: session_expiration_time,
      })
  ))

}

fn generate_jwt(sub: &str, exp: usize) -> Result<String, jsonwebtoken::errors::Error> {
  let secret = env::var("ENV_VAR_JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
  println!("{} {} {}",secret,sub,exp);
  let claims = Claims {
      sub: sub.to_owned(),
      exp,
  };
  encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
/// Verify signature using ed25519_dalek
/// Verifies a signature given a nonce and a public key.
///
/// - `signature`: The signature object.
/// - `nonce`: The message that was signed, as a string.
/// - `public_key_hex`: The hexadecimal string of the public key.
///
/// Returns `true` if the signature is valid; `false` otherwise.
fn verify_signature(signature: &Signature, nonce: &str, public_key_hex: &str) -> bool {
  // Decode the hex public key
  let public_key_bytes = match hex::decode(public_key_hex) {
      Ok(bytes) => bytes,
      Err(_) => return false, // return false if the public key hex is invalid
  };

  let public_key = match PublicKey::from_bytes(&public_key_bytes) {
      Ok(pk) => pk,
      Err(_) => return false, // return false if bytes are not a valid public key
  };

  // Verify the signature
  public_key.verify_strict(nonce.as_bytes(), &signature).is_ok()
}