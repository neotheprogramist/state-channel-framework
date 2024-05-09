use serde::{Deserialize,Serialize};
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
pub struct Agreement {
    pub quantity: String,
    pub nonce: String,
    pub price: String,
    pub serverSignatureR: String,
    pub serverSignatureS: String,
    pub clientSignatureR: String,
    pub clientSignatureS: String,
}


#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub clientPublicKey: String,
    pub serverPublicKey: String,
    pub agreements: Vec<Agreement>,
}
#[derive(Serialize, Deserialize)]
pub struct OutputData {
    pub settlement_price: i64,
    pub expected_diff: i64,
}