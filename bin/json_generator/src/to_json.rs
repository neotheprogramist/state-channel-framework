
use std::fs::File;
use std::io::Write;
use crate::models::{OutputData,InputData,Agreement};
use server::request::models::SettlementProofResponseWithData;
use server::request::account::MockAccount;

pub async fn save_out(path:String,settlement_price:i64, diff:i64) -> Result<(), std::io::Error> {
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

pub async fn prepare_and_save_data(path:String,
    settlement_proof: SettlementProofResponseWithData, // Assuming this struct is defined elsewhere
    client_mock_account: MockAccount,      // Assuming Account has a `verifying_key` field
    server_mock_account: MockAccount,
) -> Result<(), std::io::Error> {
    let agreements: Vec<Agreement> = settlement_proof.contracts.iter().map(|contract| {
        Agreement {
            quantity: contract.quantity.to_string(),
            nonce: contract.nonce.to_string(),
            price: contract.price.to_string(),
            serverSignatureR: contract.server_signature_r.to_string(),
            serverSignatureS: contract.server_signature_s.to_string(),
            clientSignatureR: contract.client_signature_r.to_string(),
            clientSignatureS: contract.client_signature_s.to_string(),
        }
    }).collect();

    let output = InputData {
        clientPublicKey: client_mock_account.verifying_key.to_string(),
        serverPublicKey: server_mock_account.verifying_key.to_string(),
        agreements,
    };

    save_input(path,output).await
}


pub async fn save_input(path:String,output: InputData) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(&output)?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}