
use crate::registry::Registry;
use crate::runtime;
use crate::types::*;
use axum::extract::{Extension, Json, Path};
use std::sync::Arc;

pub async fn recv(
    Path(module_id): Path<String>,
    Json((label, json)): Json<(String, serde_json::Value)>,
    Extension(registry): Extension<Arc<Registry>>
) -> ExecutionResult {
    runtime::exec(&registry, &module_id, label.as_str(), &json)
}
