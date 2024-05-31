use super::models::GenerateSettlementProofRequestWithPrice;
use crate::request::models::AppState;
use crate::request::models::Contract;
use crate::request::models::SettlementProofResponseWithData;
use crate::request::models::{GenerateSettlementProofRequest, SettlementProofResponse};
use crate::server::ServerError;
use axum::extract::{Json, Query, State};
use serde_json::json;
use starknet::core::types::FieldElement;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub async fn request_settlement_proof(
    State(state): State<AppState>,
    Query(params): Query<GenerateSettlementProofRequest>,
) -> Result<Json<SettlementProofResponse>, ServerError> {
    tracing::info!("Address: {}", params.address);

    let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;
    for contract in &contracts {
        tracing::info!(
            "Contract: Quantity: {}, Price: {}",
            contract.quantity,
            contract.price
        );
    }
    let settlement_proof_response = SettlementProofResponse {
        address: params.address,
        balance: 0u64.into(),
        diff: 0u64.into(),
    };
    Ok(Json(settlement_proof_response))
}

// HELPER FUNCTION : Returns settlement and all the contracts data for cairo 0 program
pub async fn request_settlement_proof_with_set_price_and_data(
    State(state): State<AppState>,
    Query(params): Query<GenerateSettlementProofRequestWithPrice>,
) -> Result<Json<SettlementProofResponseWithData>, ServerError> {
    let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;

    delete_all_contracts_for_addresss(&params.address, &state.db).await?;
    let (a, b) = aggregate(&contracts, 0u64.into(), 0u64.into());
    let diff: FieldElement = a * params.price + b;
    let settlement_proof_response = SettlementProofResponseWithData {
        contracts,
        address: params.address,
        balance: 0u64.into(),
        diff,
    };
    Ok(Json(settlement_proof_response))
}

fn aggregate(
    agreements: &[Contract],
    a: FieldElement,
    b: FieldElement,
) -> (FieldElement, FieldElement) {
    if agreements.is_empty() {
        return (a, b);
    }

    let first = &agreements[0];
    let rest = &agreements[1..];

    aggregate(rest, a + first.quantity, b - first.quantity * first.price)
}
pub async fn request_settlement_proof_with_set_price(
    State(state): State<AppState>,
    Query(params): Query<GenerateSettlementProofRequestWithPrice>,
) -> Result<Json<SettlementProofResponse>, ServerError> {
    let contracts = get_all_contracts_for_address(&state.db, &params.address).await?;

    delete_all_contracts_for_addresss(&params.address, &state.db).await?;
    let (a, b) = aggregate(&contracts, 0u64.into(), 0u64.into());
    let diff: FieldElement = a * params.price + b;

    let settlement_proof_response = SettlementProofResponse {
        address: params.address,
        balance: 0u64.into(),
        diff,
    };
    Ok(Json(settlement_proof_response))
}

pub async fn delete_all_contracts_for_addresss(
    address: &FieldElement,
    db: &Surreal<Db>,
) -> Result<(), ServerError> {
    let query = r#"DELETE FROM contract WHERE address = $address"#;

    let params = json!({
        "address": address,
    });

    db.query(query)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub async fn get_all_contracts_for_address(
    db: &Surreal<Db>,
    address: &FieldElement,
) -> Result<Vec<Contract>, ServerError> {
    let query = "
    SELECT
    id,
    address,
    quantity,
    nonce,
    price,
    server_signature_r,
    server_signature_s,
    client_signature_r,
    client_signature_s
    FROM contract
    WHERE address = type::string($address)
     ";

    let params: serde_json::Value = json!({
        "address": address,
    });
    let mut result = db
        .query(query)
        .bind(params)
        .await
        .map_err(|e| ServerError::DatabaseError(e.to_string()))?;

    let contracts: Vec<Contract> = result.take(0)?;

    Ok(contracts)
}
