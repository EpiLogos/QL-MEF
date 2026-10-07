//! Native event/decision/harmonic transport. No model runtime or QL tables.
use crate::CliError;
use ql_mef::agent_event::{DecisionResponse, ProjectionRequest, admit_decision, project_event};
use serde::Deserialize;
use serde_json::Value;
use std::io::Read;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmissionRequest {
    projection: ProjectionRequest,
    response: DecisionResponse,
}

fn read(path: &str) -> Result<Vec<u8>, CliError> {
    let source: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|e| CliError(e.to_string()))?)
    };
    let mut bytes = Vec::new();
    source
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| CliError(e.to_string()))?;
    if bytes.len() > 1024 * 1024 {
        return Err(CliError("agent-event request exceeds 1 MiB".into()));
    }
    Ok(bytes)
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    if args.len() != 2 {
        return Err(CliError(
            "usage: ql agent-event <project|frame|harmonic|validate> <request.json|-> [--json]"
                .into(),
        ));
    }
    let operation = args[0].as_str();
    if !matches!(operation, "project" | "frame" | "harmonic" | "validate") {
        return Err(CliError("unknown agent-event operation".into()));
    }
    let bytes = read(&args[1])?;
    let result: Value = if operation == "validate" {
        let request: AdmissionRequest =
            serde_json::from_slice(&bytes).map_err(|e| CliError(e.to_string()))?;
        serde_json::to_value(
            admit_decision(request.projection, request.response).map_err(CliError)?,
        )
        .map_err(|e| CliError(e.to_string()))?
    } else {
        let request: ProjectionRequest =
            serde_json::from_slice(&bytes).map_err(|e| CliError(e.to_string()))?;
        let projection = project_event(request).map_err(CliError)?;
        match operation {
            "frame" => projection.frame,
            "harmonic" => projection.harmonic,
            _ => serde_json::to_value(projection).map_err(|e| CliError(e.to_string()))?,
        }
    };
    serde_json::to_string(&result).map_err(|e| CliError(e.to_string()))
}
