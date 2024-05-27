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
    pub quantity: String,
    pub nonce: String,
    pub price: String,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InvokeResponse2 {
    pub transaction_hash: FieldElement,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldElementAgreement {
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    pub client_public_key: String,
    pub server_public_key: String,
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

pub fn to_field_elements_agreement(
    agreement: Agreement,
) -> Result<FieldElementAgreement, RunnerError> {
    let quantity: i32 = agreement.quantity.parse().unwrap();

    let quantity_hex = format!("{:x}", quantity);
    let quantity = FieldElement::from_hex_be(&quantity_hex)?;
    let nonce = FieldElement::from_hex_be(&agreement.nonce)?;
    let price = FieldElement::from_dec_str(&agreement.price)?;
    let server_signature_r = FieldElement::from_hex_be(&agreement.server_signature_r)?;
    let server_signature_s = FieldElement::from_hex_be(&agreement.server_signature_s)?;
    let client_signature_r = FieldElement::from_hex_be(&agreement.client_signature_r)?;
    let client_signature_s = FieldElement::from_hex_be(&agreement.client_signature_s)?;
    let final_agreement = FieldElementAgreement {
        quantity,
        nonce,
        price,
        server_signature_r,
        server_signature_s,
        client_signature_r,
        client_signature_s,
    };
    Ok(final_agreement)
}

pub fn get_formated_agreements(
    agreements: Vec<Agreement>,
) -> Result<Vec<FieldElementAgreement>, RunnerError> {
    let mut field_agreements: Vec<FieldElementAgreement> = Vec::new();
    for agreement in agreements {
        let field_agreement: FieldElementAgreement = to_field_elements_agreement(agreement)?;
        field_agreements.push(field_agreement);
    }
    Ok(field_agreements)
}

pub fn get_agreements_data() -> Result<(Vec<FieldElementAgreement>, String, String), RunnerError> {
    let mut file = File::open("resources/json_generator_out/in.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: InputData = serde_json::from_str(&contents)?;
    let agreements = get_formated_agreements(data.agreements)?;

    Ok((agreements, data.client_public_key, data.server_public_key))
}
