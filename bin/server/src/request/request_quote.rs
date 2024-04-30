use super::account::MockAccount;
use super::models::{Quote, RequestQuotation, RequestQuotationResponse};
use super::price::get_btc_usdt_price;
use crate::request::account::scalar_to_hex;
use crate::request::models::Nonce;
use crate::server::{AppState, ServerError};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
pub async fn request_quote(
    State(_state): State<AppState>,
    Json(payload): Json<RequestQuotation>,
) -> Result<impl IntoResponse, ServerError> {
    let nonce = Nonce::new(32);
    let btc_price = match get_btc_usdt_price().await {
        Ok(price) => price,
        Err(err) => {
            eprintln!("Error getting BTC price: {:?}", err);
            return Err(ServerError::BTCRequestFailure(
                "Failed to get BTC price.".to_string(),
            ));
        }
    };
    let quote = Quote {
        address: payload.address,
        quantity: payload.quantity,
        nonce: nonce.clone(),
        price: btc_price,
    };

    println!("{}", quote);

    let quote_json = serde_json::to_string(&quote).unwrap();
    let quote_bytes = quote_json.as_bytes();

    // Use MockAccount for signing the quote
    let mut rng = OsRng; // Create an instance of a cryptographically secure RNG
    let mut mock_account = MockAccount::new(&mut rng); // Initialize MockAccount with RNG

    let server_signature = mock_account.sign_message(quote_bytes, &mut rng);
    let server_signature = match server_signature {
        Ok(signature) => {
            println!("Signature R part: {:?}", scalar_to_hex(&signature.r));
            println!("Signature S part: {:?}", scalar_to_hex(&signature.s));
            // Create a JSON-serializable format or a simple string representation of the signature
            let signature_json = format!(
                "{{\"r\": \"{}\", \"s\": \"{}\"}}",
                scalar_to_hex(&signature.r),
                scalar_to_hex(&signature.s)
            );
            println!("Serialized Signature: {}", signature_json);
            signature_json
        }
        Err(e) => {
            //todo: fix the error
            println!("Failed to sign message: {}", e);
            return Err(ServerError::DatabaseError("ERROR".to_string())); // Convert the error &str to a String if necessary
        }
    };
    // Convert signature to a string for serialization
    Ok(Json(RequestQuotationResponse {
        quote,
        server_signature: server_signature,
    }))
}
