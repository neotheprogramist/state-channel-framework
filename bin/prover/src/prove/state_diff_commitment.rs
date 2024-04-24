use podman::runner::Runner;
use serde_json::Value;
use super::ProveError;
use axum::extract::{Query, Json};
use super::models::{Nonce, PublicKeyQuery, ProgramInput,GenerateNonceResponse,Message};
use super::NONCES;
pub const MESSAGE_EXPIRATION_TIME: usize = 60; // in seconds
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in seconds


pub async fn generate_nonce() -> Json<&'static str> {
    Json("Hello, World!")
}

pub async fn root(
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