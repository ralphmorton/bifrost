
use crate::registry::Registry;
use crate::types::*;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub fn exec(registry: &Registry, module_name: &str, label: &str, json: &serde_json::Value) -> ExecutionResult {
    match registry.resolve(module_name) {
        None => ExecutionResult::ModuleResolutionError,
        Some(mod_ref) => match exec_module(&mod_ref.0, &mod_ref.1, label, json) {
            None => ExecutionResult::RuntimeExecutionError,
            Some(res) => ExecutionResult::Success(res) 
        }
    }
}

fn exec_module(engine: &Engine, module: &Module, label: &str, json: &serde_json::Value) -> Option<String> {
    let mut linker = Linker::new(engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).ok()?;

    let json = serde_json::to_string(json).ok()?;
    let stdout = wasi_common::pipe::WritePipe::new_in_memory();

    {
        let wasi =
            WasiCtxBuilder::new()
            .stdout(Box::new(stdout.clone()))
            .arg(label).ok()?
            .arg(&json).ok()?
            .build();

        let mut store = Store::new(engine, wasi);

        linker.module(&mut store, "", module).ok()?;
        linker
            .get_default(&mut store, "").ok()?
            .typed::<(), (), _>(&store).ok()?
            .call(&mut store, ()).ok()?;
    }

    let stdout_contents: Vec<u8> =
        stdout
        .try_into_inner()
        .ok()?
        .into_inner();

    std::str::from_utf8(&stdout_contents)
        .ok()
        .map(|e| e.to_string())
}
