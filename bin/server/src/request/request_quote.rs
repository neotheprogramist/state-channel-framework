use crate::server::ServerError;
use crate::request::models::Nonce;
use axum::Json;
use axum::response::IntoResponse;
use super::models::{Quote, RequestQuotation, RequestQuotationResponse};
use keccak_hash::keccak;
use std::env;
use super::price::get_btc_usdt_price;

pub async fn request_quote(
    Json(payload): Json<RequestQuotation>,
) -> Result<impl IntoResponse, ServerError> {
    let nonce = Nonce::new(32);
    let btc_price = match get_btc_usdt_price().await {
        Ok(price) => price,
        Err(err) => {
            eprintln!("Error getting BTC price: {:?}", err);
            return Err(ServerError::BTCRequestFailure("Failed to get BTC price.".to_string()));

        }
    };
    let quote = Quote {
        address: payload.address,
        quantity: payload.quantity,
        nonce:nonce.clone(),
        price:btc_price,
    };

    println!("{}", quote);

    let hash = hash_quote(&quote);

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not found!");

    //TODO: SIGN WITH ACCOUNT
    //  let signature = account.signMessage( message: hash );
    let signature = "Signature";

    Ok(Json(RequestQuotationResponse {
        nonce: nonce,
        server_signature: signature.to_string(),
    }))
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
