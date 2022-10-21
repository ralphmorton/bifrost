use crate::registry::Registry;
use crate::types::ExecutionResult;
use log::{debug, error};
use std::string::ToString;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub fn exec(
    registry: &Registry,
    module_id: &str,
    label: &str,
    json: &serde_json::Value,
) -> ExecutionResult {
    debug!("executing request for module {}", module_id);

    match registry.resolve(module_id) {
        None => ExecutionResult::ModuleResolutionError,
        Some(mod_ref) => match exec_module(&mod_ref.0, &mod_ref.1, label, json) {
            None => ExecutionResult::RuntimeExecutionError,
            Some(res) => ExecutionResult::Success(res),
        },
    }
}

fn exec_module(
    engine: &Engine,
    module: &Module,
    label: &str,
    json: &serde_json::Value,
) -> Option<String> {
    let mut linker = Linker::new(engine);

    or_error(
        wasmtime_wasi::add_to_linker(&mut linker, |s| s),
        "could not add WASI runtime to linker",
    )?;

    let json = or_error(serde_json::to_string(json), "serializing runtime payload")?;

    let stdout = wasi_common::pipe::WritePipe::new_in_memory();

    {
        let wasi = or_error(
            WasiCtxBuilder::new()
                .stdout(Box::new(stdout.clone()))
                .arg(label)
                .and_then(|b| b.arg(&json))
                .map(|b| b.build()),
            "failed to build WASI context",
        )?;

        let mut store = Store::new(engine, wasi);

        or_error(
            linker.module(&mut store, "", module),
            "unable to link module",
        )?;

        let entrypoint = or_error(
            linker
                .get_default(&mut store, "")
                .and_then(|f| f.typed::<(), (), _>(&store)),
            "unable to resolve WASM entrypoint",
        )?;

        or_error(
            entrypoint.call(&mut store, ()),
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
