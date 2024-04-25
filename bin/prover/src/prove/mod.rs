use axum::{http::StatusCode, response::IntoResponse, routing::post,routing::get, Router};
use podman::process::ProcessError;
use thiserror::Error;
use crate::server::AppState;
mod state_diff_commitment;
pub mod models;



#[derive(Error, Debug)]
pub enum ProveError {
    #[error("failed to prove state-diff-commitment")]
    StateDiffCommitment(#[from] ProcessError),

    #[error("failed to parse result")]
    Parse(#[from] serde_json::Error),

    #[error("failed to acquire lock")]
    LockError(String),

    #[error("unauthorized access")]
    Unauthorized(String),

    #[error("resource not found")]
    NotFound(String),

    #[error("internal server error")]
    InternalServerError(String),

    #[error("validation error: {0}")]
    Validation(String),
    #[error("Missing or invalid public key")]
    MissingPublicKey,
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

pub fn router(app_state: &AppState) -> Router{
    Router::new()
        .route("/state-diff-commitment", post(state_diff_commitment::root))
        .with_state(app_state.clone())
    }

    
pub fn auth(app_state: &AppState) -> Router{
    Router::new()
        .route("/auth", get(crate::auth::generate_nonce))
        .route("/auth", post(crate::auth::validate_signature))
        .with_state(app_state.clone())
    }
