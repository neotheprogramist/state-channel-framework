use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use podman::process::ProcessError;
use thiserror::Error;

mod state_diff_commitment;

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
    Router::new().route("/state-diff-commitment", post(state_diff_commitment::root))
}
