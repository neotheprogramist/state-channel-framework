use crate::RunnerError;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;

use std::{fs::File, io::Read};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agreement {
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InvokeResponse2 {
    pub transaction_hash: FieldElement,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    pub client_public_key: FieldElement,
    pub server_public_key: FieldElement,
    pub agreements: Vec<Agreement>,
}

pub struct AgreementConstructor {
    pub client_balance: FieldElement,
    pub server_balance: FieldElement,
    pub client_public_key: FieldElement,
    pub server_public_key: FieldElement,
    pub a: FieldElement,
    pub b: FieldElement,
}

pub fn get_agreements_data(
    path_to_input: &str,
) -> Result<(Vec<Agreement>, FieldElement, FieldElement), RunnerError> {
    let mut file = File::open(path_to_input).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            RunnerError::ReadFileError(
                "Agreements input data not found, please verify path, default target/generator_output".to_string()
            )
        } else {
            RunnerError::ReadFileError(e.to_string())
        }
    })?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| RunnerError::ReadFileError(e.to_string()))?;
    let data: InputData = serde_json::from_str(&contents)?;

    Ok((
        data.agreements,
        data.client_public_key,
        data.server_public_key,
    ))
}
