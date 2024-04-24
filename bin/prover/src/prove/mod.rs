use axum::{http::StatusCode, response::IntoResponse, routing::post,routing::get, Router};
use podman::process::ProcessError;
use thiserror::Error;

mod state_diff_commitment;
mod models;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
lazy_static! {
    static ref NONCES: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Error, Debug)]
pub enum ProveError {
    #[error("failed to prove state-diff-commitment")]
    StateDiffCommitment(#[from] ProcessError),

    #[error("failed to parse result")]
    Parse(#[from] serde_json::Error),
}

impl IntoResponse for ProveError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {self}"),
        )
            .into_response()
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/state-diff-commitment", post(state_diff_commitment::root))
        .route("/state-diff-commitment", get(state_diff_commitment::generate_nonce))
    }
