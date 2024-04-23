use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use podman::process::ProcessError;
use thiserror::Error;

mod state_diff_commitment;

#[derive(Error, Debug)]
pub enum ProveError {
    #[error("failed to prove state-diff-commitment")]
    StateDiffCommitment(#[from] ProcessError),
}

impl IntoResponse for ProveError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.to_string()),
        )
            .into_response()
    }
}

pub fn router() -> Router {
    Router::new().route("/state-diff-commitment", get(state_diff_commitment::root))
}
