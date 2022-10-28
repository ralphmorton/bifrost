use crate::registry::Environment;
use crate::registry::Registry;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::{debug, error};
use std::string::ToString;
use wasmtime::*;
use wasmtime_wasi::tokio::WasiCtxBuilder;

pub async fn exec(
    registry: &Registry,
    module_id: &str,
    label: &str,
    json: &serde_json::Value,
) -> ExecutionResult {
    debug!("executing request for module {}", module_id);

    match registry.resolve(module_id) {
        None => ExecutionResult::ModuleResolutionError,
        Some(env_ref) => match exec_env(&*env_ref, label, json).await {
            None => ExecutionResult::RuntimeExecutionError,
            Some(res) => ExecutionResult::Success(res),
        },
    }
}

async fn exec_env(
    env: &Environment,
    label: &str,
    json: &serde_json::Value,
) -> Option<String> {
    let engine = &env.engine;
    let module = &env.module;
    let variables = &env.variables;

    let mongo = bifrost_mongodb_wasmtime::Mongo::new();

    let mut linker = Linker::new(engine);

    or_error(
        wasmtime_wasi::tokio::add_to_linker(&mut linker, |s| s),
        "could not add WASI runtime to linker",
    )?;

    or_error(
        mongo.add_to_linker(&mut linker),
        "could not add MongoDB runtime to linker"
    )?;

    let json = or_error(serde_json::to_string(json), "serializing runtime payload")?;

    let stdout = wasi_common::pipe::WritePipe::new_in_memory();

    {
        let wasi = or_error(
            WasiCtxBuilder::new()
                .stdout(Box::new(stdout.clone()))
                .arg(label)
                .and_then(|b| b.arg(&json))
                .and_then(|b| b.envs(variables))
                .map(|b| b.build()),
            "failed to build WASI context",
        )?;

        let mut store = Store::new(engine, wasi);

        or_error(
            linker.module_async(&mut store, "", module).await,
            "unable to link module",
        )?;

        let entrypoint = or_error(
            linker
                .get_default(&mut store, "")
                .and_then(|f| f.typed::<(), (), _>(&store)),
            "unable to resolve WASM entrypoint",
        )?;

        or_error(
            entrypoint.call_async(&mut store, ()).await,
            "unable to execute WASM entrypoint",
        )?;
    }

    let stdout_contents: Vec<u8> = or_error(
        stdout
            .try_into_inner()
            .map(|i| i.into_inner())
            .map_err(|_| "pipe still referenced elsewhere"),
        "unable to retrieve stdout output",
    )?;

    or_error(
        std::str::from_utf8(&stdout_contents).map(String::from),
        "unable to read stdout contents",
    )
}

#[inline]
fn or_error<T, E>(res: Result<T, E>, prefix: &'static str) -> Option<T>
where
    E: ToString,
{
    match res {
        Ok(v) => Some(v),
        Err(e) => {
            error!("{}: {}", prefix, e.to_string());
            None
        }
    }
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
