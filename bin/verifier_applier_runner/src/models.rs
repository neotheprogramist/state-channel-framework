use crate::RunnerError;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use std::{fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub fn get_agreements_data(
) -> Result<(Vec<FieldElementAgreement>, FieldElement, FieldElement), RunnerError> {
    let mut file = File::open("target/generator_output/in.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: InputData = serde_json::from_str(&contents)?;
    let agreements = get_formated_agreements(data.agreements)?;

    Ok((
        agreements,
        FieldElement::from_dec_str(&data.client_public_key).unwrap(),
        FieldElement::from_dec_str(&data.server_public_key).unwrap(),
    ))
}
