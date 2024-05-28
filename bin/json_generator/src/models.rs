use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use server::request::models::Quote;
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgreeToQuotation {
    pub quote: Quote,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotationResponse {
    pub quote: Quote,
    pub server_signature_r: String,
    pub server_signature_s: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    pub client_public_key: String,
    pub server_public_key: String,
    pub agreements: Vec<Agreement>,
    pub settlement_price: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub settlement_price: i64,
    pub expected_diff: i64,
}
