use crate::RunnerError;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use std::{fs::File, io::Read};
#[derive(Debug, Clone)]
pub struct DeployResult {
    pub deployed_address: FieldElement,
    pub transaction_hash: FieldElement,
}

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
    pub server_public_key:FieldElement,
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

pub fn get_agreements_data() -> Result<(Vec<Agreement>, FieldElement, FieldElement), RunnerError> {
    let mut file = File::open("target/generator_output/in.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: InputData = serde_json::from_str(&contents)?;

    Ok((
        data.agreements,
        data.client_public_key,
        data.server_public_key,
    ))
}
