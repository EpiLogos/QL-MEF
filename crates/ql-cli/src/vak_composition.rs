//! CLI transport for the single canonical native composition adapter.
use crate::CliError;
use ql_mef::vak_composition::VakComposition;
use serde_json::Value;
use std::io::Read;

type R<T> = std::result::Result<T, CliError>;
fn error(e: impl std::fmt::Display) -> CliError {
    CliError(e.to_string())
}

pub fn execute_request(request: &Value) -> R<Value> {
    ql_mef::vak_composition_wire::execute_request(request).map_err(error)
}
pub fn execute_request_with_composition<T>(
    request: &Value,
    project: impl FnOnce(&VakComposition) -> R<T>,
) -> R<(Value, T)> {
    ql_mef::vak_composition_wire::execute_request_with_composition(request, |graph| {
        project(graph).map_err(|failure| failure.to_string())
    })
    .map_err(error)
}

pub fn command(args: &[String], _json: bool) -> R<String> {
    if args.len() != 1 {
        return Err(error("usage: ql vak compose <request.json> [--json]"));
    }
    let file = std::fs::File::open(&args[0]).map_err(error)?;
    let mut data = Vec::new();
    file.take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .map_err(error)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(error("composition request exceeds 16 MiB"));
    }
    let request = serde_json::from_slice(&data).map_err(error)?;
    serde_json::to_string_pretty(&execute_request(&request)?).map_err(error)
}
