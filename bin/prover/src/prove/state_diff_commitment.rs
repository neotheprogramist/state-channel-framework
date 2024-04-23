use axum::Json;

use super::ProveError;

pub async fn root() -> Result<Json<String>, ProveError> {
    Ok(Json::default())
}
