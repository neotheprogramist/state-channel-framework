use axum::{ routing::post, Router};

use crate::server::AppState;

mod accept_contract;
mod models;
mod request_quote;
mod price;
mod tests;
mod account;

pub fn router(app_state:&AppState) -> Router {
    Router::new()
        .route("/requestQuote", post(request_quote::request_quote))
        .route("/acceptContract", post(accept_contract::accept_contract))
        .with_state(app_state.clone())
}
