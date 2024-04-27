use super::models::RequestAcceptContract;
use crate::server::ServerError;
use axum::Json;

pub async fn accept_contract(
    Json(payload): Json<RequestAcceptContract>,
) -> Result<String, ServerError> {
    let quote: super::models::Quote = payload.quote;
    let server_signature = payload.server_signature;
    let client_signature = payload.client_signature;

    let result = create_contract().await;

    Ok("OK")
}

pub async fn create_contract() {}
