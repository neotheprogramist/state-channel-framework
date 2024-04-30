use super::models::Quote;
use super::models::RequestAcceptContract;
use crate::server::{AppState, ServerError};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{self, Deserialize, Deserializer, Serialize};
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
        " quote:{} \n server_signature:{} \n quclient_signatureote:{} \n",
        quote, server_signature, client_signature
    );
    //TODO: create contract
    let result = create_contract(state.db, &quote, &server_signature, &client_signature).await?;
    println!("{}", result.to_string());
    Ok(StatusCode::NO_CONTENT)
}
#[derive(Debug, Serialize, Deserialize)]
struct Id {
    tb: String,
    id: String,
}
fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        id: String,
    }

    let helper = Wrapper::deserialize(deserializer)?;
    Ok(helper.id)
}

#[derive(Debug, Serialize, Deserialize)]
struct Contract {
    address: String,
    quantity: u64,
    nonce: String,
    price: f64,
    server_signature: String,
    client_signature: String,
}
impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " \n address :{} \n quantity :{} \n nonce :{} \n price :{} \n server_signature :{} \n client_signature :{} \n",
    self.address,self.quantity,self.nonce,self.price,self.server_signature,self.client_signature)
    }
}

async fn create_contract(
    db: Surreal<Client>,
    quote: &Quote,
    server_signature: &str,
    client_signature: &str,
) -> Result<Contract, ServerError> {
    let query = r#"CREATE ONLY contract SET
        address = type::string($address),
        quantity = type::number($quantity),
        nonce = type::string($nonce),
        price = type::number($price),
        server_signature = type::string($server_signature),
        client_signature = type::string($client_signature)"#;

    let params = json!({
        "address": quote.address.to_string(),
        "quantity": quote.quantity,
        "nonce": quote.nonce.to_string(),
        "price": quote.price,
        "server_signature": server_signature.to_string(),
        "client_signature": client_signature.to_string(),
    });

    println!("Before creating contract");
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
