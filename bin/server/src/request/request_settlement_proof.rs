use crate::request::models::Contract;
use crate::request::models::{GenerateSettlementProofRequest, SettlementProofResponse};
use crate::request::price::get_btc_usdt_price;
use crate::server::{AppState, ServerError};
use axum::extract::{Json, Query, State};
use serde_json::json;
use std::collections::HashSet;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use std::collections::HashMap;
use super::models::GenerateSettlementProofRequestWithPrice;

pub async fn request_settlement_proof(
    State(state): State<AppState>,
    Query(params): Query<GenerateSettlementProofRequest>,
) -> Result<Json<SettlementProofResponse>, ServerError> {
    if params.address.trim().is_empty() {
        return Err(ServerError::DatabaseError("Missing address".to_string()));
    }
    println!("Address: {}",params.address);

    let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;
    println!("DISPLAY AGGREMENTS ");
    for contract in &contracts {
        println!("Contract: Quantity: {}, Price: {}", contract.quantity, contract.price);
    }
    let (a,b) = aggregate(&contracts,0, 0);
    let settlement_proof_response = SettlementProofResponse{address:params.address,balance:0.0,diff:0};
    Ok(Json(settlement_proof_response))
}
fn aggregate(agreements: &[Contract], a: i64, b: i64) -> (i64, i64) {
    if agreements.is_empty() {
        return (a, b);
    }
    
    let first = &agreements[0];
    let rest = &agreements[1..];

    aggregate(
        rest,
        a + first.quantity,
        b - first.quantity * first.price
    )
}
// pub async fn request_settlement_proof(
//     State(state): State<AppState>,
//     Query(params): Query<GenerateSettlementProofRequest>,
// ) -> Result<Json<SettlementProofResponse>, ServerError> {
//     dbg!("Request settlement proof called");
//     if params.address.trim().is_empty() {
//         println!("Missing public key");
//         return Err(ServerError::DatabaseError("Missing address".to_string()));
//     }
//     println!("Address: {}",params.address);

//     let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;
//     let price = get_btc_usdt_price().await?;
//     let mut diff: i64 = 0;
//     let mut to_delete_contract_ids = HashSet::new();
//     dbg!("current price: {}",price);
//     for contract in contracts {
//         dbg!("contrat price: {}",contract.price);
//         diff += (price - contract.price) * contract.quantity;
//         to_delete_contract_ids.insert(contract.id.id);
//     }

//     // let to_delete_contract_ids: Vec<_> = to_delete_contract_ids.into_iter().collect();
//     // let delete_contracts_result = delete_contracts_by_ids(&state.db, &to_delete_contract_ids).await?;
//     // dbg!("deleteContractsByIdsResult: {:?}", delete_contracts_result);
//     // dbg!("GOT DIFF");
//     let settlement_proof_response = SettlementProofResponse {
//         address: params.address,
//         balance: 0.0,
//         diff,
//     };
//     Ok(Json(settlement_proof_response))
// }

// pub async fn request_settlement_proof_with_set_price(
//     State(state): State<AppState>,
//     Query(params): Query<GenerateSettlementProofRequestWithPrice>,
// ) -> Result<Json<SettlementProofResponse>, ServerError> {
//     println!("Request settlement proof called");
//     if params.address.trim().is_empty() {
//         println!("Missing public key");
//         return Err(ServerError::DatabaseError("Missing address".to_string()));
//     }
//     println!("Address: {}", params.address);

//     // Get all contracts for the specified address
//     let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;
    
//     let price: i64 = params.price;
//     let mut diff: i64 = 0;
//    // let mut to_delete_contract_ids = HashSet::new();

//     dbg!("Current price: {}", price);

//     for contract in &contracts {
//         println!("Contract ID: {}", contract.id.id);
//         dbg!("Contract price: {}", contract.price);

//         // Calculate the difference based on the current price and contract price
//         diff += (price - ( price-1)) * contract.quantity;

//     }

//     dbg!("Difference: {}", diff);

//     // Convert the set of contract IDs to a vector
//     let to_delete_contract_ids: Vec<String> = contracts.iter().map(|contract| contract.id.id.to_string()).collect();
//     let delete_contracts_result = delete_contracts_by_ids(&state.db, &to_delete_contract_ids).await?;
//     // Delete contracts by IDs
//     //verify_deletion_before_and_after(&state.db, &params.address, &to_delete_contract_ids).await?;

//     //let delete_contracts_result = delete_contracts_by_ids(&state.db, &to_delete_contract_ids).await?;
//     //println!("deleteContractsByIdsResult: {:?}", delete_contracts_result);

//     println!("Settlement proof calculation complete");

//     // Create the settlement proof response
//     let settlement_proof_response = SettlementProofResponse {
//         address: params.address.clone(),
//         balance: 0.0,
//         diff,
//     };

//     Ok(Json(settlement_proof_response))
// }

// pub async fn delete_contracts_by_ids(
//     db: &Surreal<Client>,
//     to_delete_contract_ids: &[String], // Change the type to Vec<String>
// ) -> Result<surrealdb::Response, ServerError> {
//     // Construct the parameter map
//     let mut params = HashMap::new();
//     params.insert("ids", to_delete_contract_ids);

//     // Construct the query string
//     let query = "DELETE ONLY contract WHERE id IN ($ids) RETURN NONE";

//     // Execute the query
//     let result = db
//         .query(query)
//         .bind(params)
//         .await
//         .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

//     Ok(result)
// }
// pub async fn verify_deletion_before_and_after(
//     db: &Surreal<Client>,
//     address:&String,
//     to_delete_contract_ids: &[surrealdb::sql::Id]
// ) -> Result<(), ServerError> {
//     // Query the contracts before deletion
//     let contracts_before = get_all_contracts_for_address(&db, address).await?;

//     // Delete the contracts
//     //let result = delete_contracts_by_ids(&db, &to_delete_contract_ids).await?;
//     println!("Response from");

//     // Print the response
//     // println!("Response from delete_contracts_by_ids: {:?}", result);

//     let contracts_after = get_all_contracts_for_address(&db, address).await?;

//     // Print the contracts before deletion
//     println!("Contracts before deletion: {:?}", contracts_before);

//     // Print the contracts after deletion
//     println!("Contracts after deletion: {:?}", contracts_after);

//     // Verify if the contracts are deleted by comparing the number of contracts before and after deletion
//     if contracts_before.len() == contracts_after.len() {
//         println!("Contracts were not deleted properly.");
//     } else {
//         println!("Contracts were deleted properly.");
//     }

//     Ok(())
// }


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

    let contracts: Vec<Contract> = result.take(0)?;

    Ok(contracts)
}
