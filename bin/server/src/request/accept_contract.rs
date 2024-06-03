use super::models::{Contract, RequestAcceptContract};
use crate::request::models::AppState;
use crate::server::ServerError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use starknet::core::types::FieldElement;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use utils::models::Quote;
use uuid::Uuid;

pub async fn accept_contract(
    State(state): State<AppState>,
    Json(payload): Json<RequestAcceptContract>,
) -> Result<impl IntoResponse, ServerError> {
    create_contract(
        state.db,
        &payload.quote,
        payload.server_signature_r,
        payload.server_signature_s,
        payload.client_signature_r,
        payload.client_signature_s,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
#[derive(Debug, Serialize)]
struct CreateContractQueryParams<'a> {
    id: &'a str,
    address: &'a str,
    quantity: &'a str,
    nonce: &'a str,
    price: &'a str,
    server_signature_r: &'a str,
    server_signature_s: &'a str,
    client_signature_r: &'a str,
    client_signature_s: &'a str,
}

impl CreateContractQueryParams<'_> {
    pub const QUERY: &'static str = r#"CREATE contract SET
    id = type::string($id),
    address = type::string($address),
    quantity = type::string($quantity),
    nonce = type::string($nonce),
    price = type::string($price),
    server_signature_r = type::string($server_signature_r),
    server_signature_s = type::string($server_signature_s),
    client_signature_r = type::string($client_signature_r),
    client_signature_s = type::string($client_signature_s)"#;
}

async fn create_contract(
    db: Surreal<Db>,
    quote: &Quote,
    server_signature_r: FieldElement,
    server_signature_s: FieldElement,
    client_signature_r: FieldElement,
    client_signature_s: FieldElement,
) -> Result<Contract, ServerError> {
    let params = CreateContractQueryParams {
        id: &Uuid::new_v4().to_string(),
        address: &quote.address.to_string(),
        quantity: &quote.quantity.to_string(),
        nonce: &quote.nonce.to_string(),
        price: &quote.price.to_string(),
        server_signature_r: &server_signature_r.to_string(),
        server_signature_s: &server_signature_s.to_string(),
        client_signature_r: &client_signature_r.to_string(),
        client_signature_s: &client_signature_s.to_string(),
    };

    let mut result = db
        .query(CreateContractQueryParams::QUERY)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

    match result.take(0) {
        Ok(Some(contract)) => {
            tracing::info!("Contract created successfully.");
            Ok(contract)
        }
        Ok(None) => {
            tracing::info!("No contract was created.");
            Err(ServerError::DatabaseError(
                "No contract was created.".to_string(),
            ))
        }
        Err(e) => {
            tracing::info!("Error retrieving contract: {:?}", e);
            Err(ServerError::DatabaseError(e.to_string()))
        }
    }
}
