
use crate::registry::Registry;
use crate::runtime;
use crate::types::*;
use rocket::serde::json::Json;
use rocket::State;

#[rocket::post("/<module_name>", format = "json", data = "<op>")]
pub async fn recv(module_name: &str, op: Json<(String, serde_json::Value)>, registry: &State<Registry>) -> ExecutionResult {
    let (label, json) = op.into_inner();
    runtime::exec(registry, module_name, label.as_str(), &json)
}
