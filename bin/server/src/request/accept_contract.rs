use super::models::{Contract, Quote, RequestAcceptContract};
use crate::request::models::AppState;
use crate::server::ServerError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use uuid::Uuid;

pub async fn accept_contract(
    State(state): State<AppState>,
    Json(payload): Json<RequestAcceptContract>,
) -> Result<impl IntoResponse, ServerError> {
    let quote: super::models::Quote = payload.quote;
    let server_signature_r = payload.server_signature_r;
    let server_signature_s = payload.server_signature_s;
    let client_signature_r = payload.client_signature_r;
    let client_signature_s = payload.client_signature_s;

    create_contract(
        state.db,
        &quote,
        &server_signature_r,
        &server_signature_s,
        &client_signature_r,
        &client_signature_s,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize)]
struct CreateContractQueryParams<'a> {
    id: &'a str,
    address: &'a str,
    quantity: i64,
    nonce: &'a str,
    price: i64,
    server_signature_r: &'a str,
    server_signature_s: &'a str,
    client_signature_r: &'a str,
    client_signature_s: &'a str,
}

impl CreateContractQueryParams<'_> {
    pub const QUERY: &'static str = r#"CREATE contract SET
    id = type::string($id),
    address = type::string($address),
    quantity = type::number($quantity),
    nonce = type::string($nonce),
    price = type::number($price),
    server_signature_r = type::string($server_signature_r),
    server_signature_s = type::string($server_signature_s),
    client_signature_r = type::string($client_signature_r),
    client_signature_s = type::string($client_signature_s)"#;
}

async fn create_contract(
    db: Surreal<Db>,
    quote: &Quote,
    server_signature_r: &str,
    server_signature_s: &str,
    client_signature_r: &str,
    client_signature_s: &str,
) -> Result<Contract, ServerError> {
    let params = CreateContractQueryParams {
        id: &Uuid::new_v4().to_string(),
        address: &quote.address,
        quantity: quote.quantity,
        nonce: &quote.nonce,
        price: quote.price,
        server_signature_r: server_signature_r,
        server_signature_s: server_signature_s,
        client_signature_r: client_signature_r,
        client_signature_s: client_signature_s,
    };

    let mut result = db
        .query(CreateContractQueryParams::QUERY)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

    match result.take(0) {
        Ok(Some(contract)) => {
            println!("Contract created successfully.");
            Ok(contract)
        }
        Ok(None) => {
            println!("No contract was created.");
            Err(ServerError::DatabaseError(
                "No contract was created.".to_string(),
            ))
        }
        Err(e) => {
            println!("Error retrieving contract: {:?}", e);
            Err(ServerError::DatabaseError(e.to_string()))
        }
    }
}
