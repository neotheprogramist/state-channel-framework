use crate::models::{Agreement, InputData, OutputData};
use server::request::account::MockAccount;
use server::request::models::SettlementProofResponseWithData;
use std::fs::File;
use std::io::Write;

pub async fn save_out(
    path: String,
    settlement_price: i64,
    diff: i64,
) -> Result<(), std::io::Error> {
    let data = OutputData {
        settlement_price,
        expected_diff: diff,
    };

    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub async fn prepare_and_save_data(
    path: String,
    settlement_proof: SettlementProofResponseWithData,
    client_mock_account: MockAccount,
    server_mock_account: MockAccount,
    settlement_price: i64,
) -> Result<(), std::io::Error> {
    let agreements: Vec<Agreement> = settlement_proof
        .contracts
        .iter()
        .map(|contract| Agreement {
            quantity: contract.quantity.to_string(),
            nonce: contract.nonce.to_string(),
            price: contract.price.to_string(),
            server_signature_r: contract.server_signature_r.to_string(),
            server_signature_s: contract.server_signature_s.to_string(),
            client_signature_r: contract.client_signature_r.to_string(),
            client_signature_s: contract.client_signature_s.to_string(),
        })
        .collect();

    let output = InputData {
        client_public_key: format!("0x{:x}", client_mock_account.public_key.scalar()),
        server_public_key: format!("0x{:x}", server_mock_account.public_key.scalar()),
        agreements,
        settlement_price: format!("{}", settlement_price),
    };

    save_input(path, output).await
}

pub async fn save_input(path: String, output: InputData) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(&output)?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
