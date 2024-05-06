use crate::request::models::Contract;
use crate::request::models::{GenerateSettlementProofRequest, SettlementProofResponse};
use crate::request::price::get_btc_usdt_price;
use crate::server::{AppState, ServerError};
use axum::extract::{Json, Query, State};
use serde_json::json;
use std::collections::HashSet;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn request_settlement_proof(
    State(state): State<AppState>,
    Query(params): Query<GenerateSettlementProofRequest>,
) -> Result<Json<SettlementProofResponse>, ServerError> {
    println!("Request settlement proof called");
    if params.address.trim().is_empty() {
        println!("Missing public key");
        return Err(ServerError::DatabaseError("Missing address".to_string()));
    }
    println!("Address: {}",params.address);

    let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;
    let price = get_btc_usdt_price().await?;
    let mut diff: i64 = 0;
    let mut to_delete_contract_ids = HashSet::new();
    println!("current price: {}",price);
    for contract in contracts {
        println!("contrat price: {}",contract.price);
        diff += (price - contract.price) * contract.quantity;
        to_delete_contract_ids.insert(contract.id.id);
    }

    let to_delete_contract_ids: Vec<_> = to_delete_contract_ids.into_iter().collect();
    let delete_contracts_result = delete_contracts_by_ids(&state.db, &to_delete_contract_ids).await?;
    println!("deleteContractsByIdsResult: {:?}", delete_contracts_result);
    println!("GOT DIFF");
    let settlement_proof_response = SettlementProofResponse {
        address: params.address,
        balance: 0.0,
        diff,
    };
    Ok(Json(settlement_proof_response))
}

pub async fn delete_contracts_by_ids(
    db: &Surreal<Client>,
    to_delete_contract_ids: &[surrealdb::sql::Id], // Changed to slice reference
) -> Result<(), ServerError> {
    for id in to_delete_contract_ids {
        let query = format!("DELETE contract WHERE id = {} RETURN BEFORE", id);
        
        let result = db
            .query(&query)
            .await
            .map_err(|e| ServerError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

pub async fn get_all_contracts_for_address(
    db: &Surreal<Client>,
    address: &String,
) -> Result<Vec<Contract>, ServerError> {

    let query = "
    SELECT
    id,
    address,
    quantity,
    nonce,
    price,
    server_signature,
    client_signature
    FROM contract
    WHERE address = type::string($address)
     ";

    let params: serde_json::Value = json!({
        "address": address,
    });
    println!("{}",address);
    let mut result = db
        .query(query)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;
    println!("get_all_contracts_for_address2");

    let contracts: Vec<Contract> = result.take(0)?;
    println!("get_all_contracts_for_address3");

    Ok(contracts)
}
