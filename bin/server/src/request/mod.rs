use axum::{ routing::post, Router};

mod accept_contract;
mod models;
mod request_quote;
mod price;


pub fn router() -> Router {
    Router::new()
        .route("/requestQuote", post(request_quote::request_quote))
        .route("/acceptContract", post(accept_contract::accept_contract))
}
