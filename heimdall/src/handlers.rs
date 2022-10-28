use crate::registry::Registry;
use crate::runtime;
use axum::extract::{Extension, Json, Multipart, Path};
use axum::http::StatusCode;
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn register(
    Path(module_id): Path<String>,
    mut multipart: Multipart,
    Extension(registry): Extension<Arc<Registry>>,
) -> StatusCode {
    debug!("processing upload for module {}", module_id);

    let multipart_field = multipart.next_field().await.ok().flatten();

    let binary = match multipart_field {
        Some(f) => f.bytes().await.ok(),
        None => None,
    };

    match binary {
        None => {
            error!("unable to extract module for upload");
            StatusCode::BAD_REQUEST
        }
        Some(bytes) => {
            debug!("extracted module binary: {} bytes", bytes.len());
            let result = registry.add(module_id.as_str(), bytes.to_vec());

            if result {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub async fn attach_variables(
    Path(module_id): Path<String>,
    Json(variables): Json<Vec<(String, String)>>,
    Extension(registry): Extension<Arc<Registry>>,
) -> StatusCode {
    debug!("attaching env vars to module {}", module_id);

    let result = registry.attach_variables(module_id.as_str(), &variables);

    if result {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn attach_capabilities(
    Path(module_id): Path<String>,
    Json(capabilities): Json<HashMap<String, HashMap<String, String>>>,
    Extension(registry): Extension<Arc<Registry>>,
) -> StatusCode {
    debug!("attaching capabilities to module {}", module_id);

    let result = registry.attach_capabilities(module_id.as_str(), &capabilities);

    if result {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete(
    Path(module_id): Path<String>,
    Extension(registry): Extension<Arc<Registry>>,
) -> StatusCode {
    debug!("deleting module {}", module_id);

    let result = registry.delete(module_id.as_str());

    if result {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn recv(
    Path(module_id): Path<String>,
    Json((label, json)): Json<(String, serde_json::Value)>,
    Extension(registry): Extension<Arc<Registry>>,
) -> runtime::ExecutionResult {
    debug!(
        "processing request for {}: ({}, {:?})",
        module_id, label, json
    );

    runtime::exec(&registry, &module_id, label.as_str(), &json).await
}
