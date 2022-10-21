use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub trait Store {
    fn retrieve(&self, module_id: &str) -> Option<Vec<u8>>;
}

pub enum ExecutionResult {
    Success(String),
    ModuleResolutionError,
    RuntimeExecutionError,
}

impl IntoResponse for ExecutionResult {
    fn into_response(self) -> Response {
        match self {
            Self::Success(json) => (StatusCode::OK, json).into_response(),
            Self::ModuleResolutionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Module resolution error").into_response()
            }
            Self::RuntimeExecutionError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Runtime execution error").into_response()
            }
        }
    }
}
