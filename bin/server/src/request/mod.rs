use crate::server::AppState;
use axum::{routing::post,routing::get, Router};
use thiserror::Error;

mod accept_contract;
mod models;
mod request_quote;

#[derive(Error, Debug)]
pub enum ServerError {

    #[error("unauthorized access")]
    Unauthorized(String),

    #[error("resource not found")]
    NotFound(String),
}

pub fn router() -> Router {
    Router::new()
        .route("/requestQuote", post(request_quote::request_quote))
        .route("/acceptContract", post(accept_contract::accept_contract))
}
