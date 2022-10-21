use crate::registry::Registry;
use crate::runtime;
use crate::types::*;
use axum::extract::{Extension, Json, Path};
use log::debug;
use std::sync::Arc;

pub async fn recv(
    Path(module_id): Path<String>,
    Json((label, json)): Json<(String, serde_json::Value)>,
    Extension(registry): Extension<Arc<Registry>>,
) -> ExecutionResult {
    debug!(
        "processing request for {}: ({}, {:?})",
        module_id, label, json
    );

    runtime::exec(&registry, &module_id, label.as_str(), &json)
}
