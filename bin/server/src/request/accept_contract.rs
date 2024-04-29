use super::models::RequestAcceptContract;
use crate::server::ServerError;
use axum::Json;
use axum::response::IntoResponse;

pub async fn accept_contract(
    Json(payload): Json<RequestAcceptContract>,
) -> Result<impl IntoResponse, ServerError> {
    let quote: super::models::Quote = payload.quote;
    let server_signature = payload.server_signature;
    let client_signature = payload.client_signature;

    //TODO: create contract
    let result = create_contract().await;

    Ok(Json("OK"))
}



pub async fn create_contract() {}
