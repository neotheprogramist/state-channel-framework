use crate::request::models::AppState;
use axum::{routing::get, routing::post, Router};
mod accept_contract;
pub mod account;
pub mod models;
mod price;
mod request_quote;
mod request_settlement_proof;
mod tests;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/requestQuote", post(request_quote::request_quote))
        .route(
            "/requestQuoteWithPrice",
            post(request_quote::request_quote_with_price),
        )
        .route("/acceptContract", post(accept_contract::accept_contract))
        .route(
            "/requestSettlementProof",
            get(request_settlement_proof::request_settlement_proof),
        )
        .route(
            "/requestSettlementProofWithPrice",
            get(request_settlement_proof::request_settlement_proof_with_set_price),
        )
        .route(
            "/requestSettlementProofWithPriceAndData",
            get(request_settlement_proof::request_settlement_proof_with_set_price_and_data),
        )
        .with_state(app_state.clone())
}
