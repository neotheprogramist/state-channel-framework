use axum::extract::State;
use podman::runner::Runner;
use serde_json::Value;

use crate::server::AppState;

use super::ProveError;

pub async fn root(
    State(state): State<AppState>,
    program_input: String,
) -> Result<String, ProveError> {
    let runner = podman::runner::PodmanRunner::new(&state.prover_image_name);
    let v = program_input.to_string();
    let result: String = runner.run(&v).await?;
    let proof: Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}
