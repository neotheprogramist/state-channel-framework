use super::models::{RequestQuotationResponse, RequestQuotationWithPrice};
use crate::request::models::Nonce;
use crate::request::AppState;
use crate::server::ServerError;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use utils::models::Quote;

pub async fn request_quote_with_price(
    State(state): State<AppState>,
    Json(payload): Json<RequestQuotationWithPrice>,
) -> Result<impl IntoResponse, ServerError> {
    let nonce = Nonce::new()?;

    let btc_price = payload.price;
    let quote = Quote {
        address: payload.address,
        quantity: payload.quantity,
        nonce: *nonce.as_field_element(),
        price: btc_price,
    };

    let server_mock = state.server_mock;

    let quote_clone = quote.clone();
    let (server_signature_r, server_signature_s) = server_mock.sign_quote(quote_clone);

    Ok(Json(RequestQuotationResponse {
        quote,
        server_signature_r,
        server_signature_s,
    }))
}
