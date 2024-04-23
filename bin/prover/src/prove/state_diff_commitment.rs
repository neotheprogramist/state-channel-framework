use podman::runner::Runner;
use serde_json::Value;

use super::ProveError;

pub async fn root(program_input: String) -> Result<String, ProveError> {
    let runner = podman::runner::PodmanRunner::new("localhost/state-diff-commitment:latest");
    let v = program_input.to_string();
    let result: String = runner.run(&v).await?;
    let proof: Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}
