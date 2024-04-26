use super::ProveError;
use crate::auth::jwt::decode_jwt;
use axum::http::HeaderMap;
use podman::runner::Runner;
use serde_json::Value;

/// Handles incoming requests, validates JWT authorization, and executes a program.
///
/// # Parameters
///
/// - `headers`: HTTP headers containing the Authorization header with the JWT token.
/// - `program_input`: Input string for the program execution.
///
/// # Returns
///
/// Returns a JSON string containing the result of the program execution if successful,
/// or a `ProveError` if there's an authentication error or an error during execution.
pub async fn root(headers: HeaderMap, program_input: String) -> Result<String, ProveError> {
    //Attempt to decode and validate the JWT
    if let Some(header_value) = headers.get("Authorization") {
        let header_str = header_value.to_str().unwrap_or("");
        if header_str.starts_with("Bearer ") {
            let token = header_str[7..].to_string();
            decode_jwt(token)?;
        } else {
            return Err(ProveError::Unauthorized("Invalid or missing Bearer token".to_string()));
        }
    } else {
        return Err(ProveError::Unauthorized("Missing Authorization header".to_string()));
    }

    let runner = podman::runner::PodmanRunner::new("state-diff-commitment:latest");
    let v = program_input.to_string();
    let result: String = runner.run(&v).await?;
    let proof: Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}
