use crate::server::{ServerError,AppState};
use crate::request::models::Nonce;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use super::models::{Quote, RequestQuotation, RequestQuotationResponse};
use super::price::get_btc_usdt_price;
use ed25519_dalek::Signature;
use super::account::MockAccount;

pub async fn request_quote(
    State(_state): State<AppState>,
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

    let quote_json = serde_json::to_string(&quote).unwrap();
    let quote_bytes = quote_json.as_bytes();

    // Use MockAccount for signing the quote
    let mut mock_account = MockAccount::new();
    let server_signature: Signature = mock_account.sign_message(&quote_bytes);

    // Convert signature to a string for serialization
    Ok(Json(RequestQuotationResponse {
        quote,
        server_signature: server_signature.to_string(),
    }))
}