use super::models::{Contract, Quote, RequestAcceptContract};
use crate::server::{AppState, ServerError};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn accept_contract(
    State(state): State<AppState>,
    Json(payload): Json<RequestAcceptContract>,
) -> Result<impl IntoResponse, ServerError> {
    println!("accept_contract");

    let quote: super::models::Quote = payload.quote;
    let server_signature = payload.server_signature;
    let client_signature = payload.client_signature;
    println!(
        "accept_contract\n
         quote:{} \n server_signature:{} \n quclient_signatureote:{} \n",
        quote, server_signature, client_signature
    );
    //TODO: create contract
    let result = create_contract(state.db, &quote, &server_signature, &client_signature).await?;
    println!("{}", result.to_string());
    Ok(StatusCode::NO_CONTENT)
}

async fn create_contract(
    db: Surreal<Client>,
    quote: &Quote,
    server_signature: &str,
    client_signature: &str,
) -> Result<Contract, ServerError> {
    let query = r#"CREATE ONLY contract SET
        id = type::string($id),
        address = type::string($address),
        quantity = type::number($quantity),
        nonce = type::string($nonce),
        price = type::number($price),
        server_signature = type::string($server_signature),
        client_signature = type::string($client_signature)"#;
    let id = uuid::Uuid::new_v4();

    let params = json!({
        "id": id.to_string(),
        "address": quote.address.to_string(),
        "quantity": quote.quantity,
        "nonce": quote.nonce.to_string(),
        "price": quote.price,
        "server_signature": server_signature.to_string(),
        "client_signature": client_signature.to_string(),
    });

    println!("Before creating contract");
    println!(
        "create_contract\n server_signature:{} \n quclient_signatureote:{} \n",
        server_signature, client_signature
    );
    let mut result = db
        .query(query)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

    println!("After creating contract");
    // Check the result of taking the contract data
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
