use podman::runner::Runner;
use serde_json::Value;
use super::ProveError;
use crate::server::AppState;
use axum::{extract::State
,http::HeaderMap};
use crate::auth::jwt::decode_jwt;



pub async fn root(
  State(_state): State<AppState>,
  headers: HeaderMap,
  payload: String,
) -> Result<String, ProveError> {
    //Attempt to decode and validate the JWT
    let jwt_token: Option<String> = match headers.get("Authorization") {
      Some(header_value) => {
          // Convert header value to string
          let header_str = header_value.to_str().unwrap_or("");
          
          if header_str.starts_with("Bearer ") {
              let token = header_str[7..].to_string();
              Some(token)
          } else {
              None
          }
      },
      None => None,
  };
    let token = jwt_token.unwrap();
    println!("{}",token);
    decode_jwt(token)?; 
    println!("{}",payload);
    let runner = podman::runner::PodmanRunner::new("state-diff-commitment:latest");
    let input_json = serde_json::to_string(&payload)?;
    let result: String = runner.run(&input_json).await?;
    let proof: Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}