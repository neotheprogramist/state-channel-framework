use super::ServerError;
use crate::request::models::Nonce;
use axum::Json;

use super::models::{Quote, RequestQuotation, RequestQuotationResponse};
use keccak_hash::keccak;
use std::env;

pub async fn request_quote(
    Json(payload): Json<RequestQuotation>,
) -> Result<Json<RequestQuotationResponse>, ServerError> {
    let nonce = Nonce::new(32);
    let quote = Quote {
        address: payload.address,
        quantity: payload.quantity,
        nonce,
        price: get_btc_usdt_price(),
    };

    println!("{}", quote);

    let hash = hash_quote(&quote);

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not found!");

    //TODO: SIGN WITH ACCOUNT
    //  let signature = account.signMessage( message: hash );
    let signature = "";

    //TODO : use of moved value: `nonce` (cant us Copy)???
    Ok(Json(RequestQuotationResponse {
        nonce: nonce.clone(),
        server_signature: signature.to_string(),
    }))
}

pub fn get_btc_usdt_price() -> u64 {
    let price: u64 = 0;
    return price;
}

fn hash_quote(quote: &Quote) -> String {
    // Serialize the quote to JSON
    let serialized_quote = serde_json::to_string(quote).unwrap();

    // Compute the Keccak-256 hash of the UTF-8 encoded JSON string
    let hash = keccak(serialized_quote.as_bytes());

    // Convert the hash bytes to a hexadecimal string
    let hash_hex = hex::encode(hash);

    hash_hex
}
