use podman::runner::Runner;
use serde_json::Value;
use super::ProveError;
pub const MESSAGE_EXPIRATION_TIME: usize = 60; // in seconds
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in seconds
use crate::server::AppState;
use crate::prove::models::{Nonce,GenerateNonceResponse,Message,ValidateSignatureRequest,SessionResponse,GenerateNonceRequest,SessionId};
use axum::{
  http::{self, HeaderValue,HeaderMap},
  extract::{State,Json},
  response::IntoResponse
};
use ethers_core::types::Signature;

pub const COOKIE_NAME: &str = "session_id";

pub async fn generate_nonce(
    State(state): State<AppState>,
    payload: Json<GenerateNonceRequest>
) -> Result<Json<GenerateNonceResponse>, ProveError>{

  let nonce: Nonce = Nonce::new(32);
  let mut nonces: std::sync::MutexGuard<'_, std::collections::HashMap<String, String>> = state.nonces.lock().unwrap();
  nonces.insert(payload.public_key.clone(), nonce.clone().to_string());

  Ok(Json(GenerateNonceResponse {
      message: Message::from(nonce),
      expiration: MESSAGE_EXPIRATION_TIME,
  }))
}

pub async fn validate_signature(
  State(state): State<AppState>,
  Json(payload): Json<ValidateSignatureRequest>
)-> Result<impl IntoResponse, ProveError>{
  let mut nonces: std::sync::MutexGuard<'_, std::collections::HashMap<String, String>> = state.nonces.lock().unwrap();
  let user_nonce = nonces.get(&payload.public_key)
  .ok_or_else(|| ProveError::NotFound("Nonce not found for the provided public key".to_string()))?;


  // payload
  // .signature
  // .verify(Message::from(user_nonce).as_str(), payload.address.clone())?;
  //verify signature using curve25519_dalek
  let signature_valid = verify_signature(&payload.signature, &user_nonce, &payload.address);

  if !signature_valid {
    return Err(ProveError::Unauthorized("Invalid signature".to_string()));
  }
  let session_id =SessionId::new(32); 

  let mut headers = HeaderMap::new();
  headers.insert(
    http::header::SET_COOKIE,
    HeaderValue::from_str(&format!("{}={}", COOKIE_NAME, session_id))
        .map_err(|e| ProveError::InternalServerError(e.to_string()))?
  );

  Ok((
    headers,
    Json(SessionResponse {
        address: payload.address,
        session_id,
        expiration: SESSION_EXPIRATION_TIME,
    }),
  ))
}

// Dummy function for signature verification
fn verify_signature(signature: &Signature, nonce: &str, address: &str) -> bool {

  true
}

pub async fn root(
  State(_state): State<AppState>,
  program_input:String// Extracts the JSON body
) -> Result<String, ProveError> {

    let runner = podman::runner::PodmanRunner::new("state-diff-commitment:latest");
    // Convert the program input to a JSON string if needed
    let input_json = serde_json::to_string(&program_input)?;
    let result: String = runner.run(&input_json).await?;
    let proof: Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}