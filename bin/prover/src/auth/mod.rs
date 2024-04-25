pub const MESSAGE_EXPIRATION_TIME: usize = 60; // in seconds
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in seconds
use crate::prove::ProveError;
use crate::server::AppState;
use ed25519_dalek::{Signature, PublicKey, Verifier}; // Ensure these are properly imported

use crate::prove::models::{Nonce,GenerateNonceResponse,Message,ValidateSignatureRequest,SessionResponse,GenerateNonceRequest,SessionId};
use axum::{
  http::{self, HeaderValue,HeaderMap},
  extract::{State,Json,Query},
  response::IntoResponse
};



pub const COOKIE_NAME: &str = "session_id";

pub async fn generate_nonce(
    State(state): State<AppState>,
    Query(params): Query<GenerateNonceRequest>,

) -> Result<Json<GenerateNonceResponse>, ProveError>{
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
      message: Message::from(nonce),
      expiration: MESSAGE_EXPIRATION_TIME,
  }))
}


pub async fn validate_signature(
  State(state): State<AppState>,
  Json(payload): Json<ValidateSignatureRequest>
)-> Result<impl IntoResponse, ProveError>{ 
  let nonces = state.nonces.lock().map_err(|_| ProveError::InternalServerError("Failed to lock state".to_string()))?;
  // Printing whether the nonce was found or not, along with the public key

  let user_nonce = nonces.get(&payload.public_key)
    .ok_or_else(|| ProveError::NotFound(format!("Nonce not found for the provided public key: {}", &payload.public_key)))?;

  let signature_valid = verify_signature(&payload.signature, &user_nonce, &payload.public_key);

  if !signature_valid {
    return Err(ProveError::Unauthorized("Invalid signature".to_string()));
  }
  let session_id=SessionId::new(32);
  let mut headers = HeaderMap::new();
  headers.insert(
    http::header::SET_COOKIE,
    HeaderValue::from_str(&format!("{}={}", COOKIE_NAME, session_id))
        .map_err(|e| ProveError::InternalServerError(e.to_string()))?
  );

  Ok((
    headers,
    Json(SessionResponse {
        session_id,
        expiration: SESSION_EXPIRATION_TIME,
    }),
  ))
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