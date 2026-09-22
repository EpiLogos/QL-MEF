//! `ql epi-agent` — source-qualified domain surface for the Prime-QL body.

use crate::CliError;
use ql_mef::epi_agent::{
    self, LogosReturnRequest, NaraElementalRequest, RepresentationBindingRequest, TdaRequest,
    EPI_AGENT_INVOCATION_VERSION,
};
use serde_json::{json, Value};
use std::io::Read;

type R<T> = Result<T, CliError>;
fn error(value: impl std::fmt::Display) -> CliError {
    CliError(value.to_string())
}

pub fn command(args: &[String], json: bool) -> R<String> {
    let document = match args {
        [op] if op == "constitution" => epi_agent::constitution(),
        [op, position] if op == "faculty" => {
            let position = position
                .trim_start_matches('#')
                .parse::<u8>()
                .map_err(|_| error("faculty position must be #0..#5"))?;
            epi_agent::faculty(position).map_err(error)?
        }
        [op, path] if op == "invoke" => invoke(path)?,
        _ => {
            return Err(error(
                "usage: ql epi-agent <constitution|faculty #0..#5|invoke request.json> [--json]",
            ));
        }
    };
    if json {
        serde_json::to_string_pretty(&document).map_err(error)
    } else {
        serde_json::to_string(&document).map_err(error)
    }
}

fn read_request(path: &str) -> R<Value> {
    let file = std::fs::File::open(path).map_err(error)?;
    let mut data = Vec::new();
    file.take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)
        .map_err(error)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err(error("epi-agent request exceeds 16 MiB"));
    }
    serde_json::from_slice(&data).map_err(|e| error(format!("invalid epi-agent request: {e}")))
}

fn position(value: &Value) -> R<u8> {
    value["position"]
        .as_str()
        .and_then(|value| value.trim_start_matches('#').parse::<u8>().ok())
        .filter(|value| *value <= 5)
        .ok_or_else(|| error("epi-agent invocation position must be #0..#5"))
}

fn input<'a>(value: &'a Value) -> R<&'a Value> {
    value
        .get("input")
        .filter(|input| input.is_object())
        .ok_or_else(|| error("epi-agent invocation requires an input object"))
}

fn invoke(path: &str) -> R<Value> {
    let request = read_request(path)?;
    if request["schema"] != EPI_AGENT_INVOCATION_VERSION {
        return Err(error("wrong epi-agent invocation schema"));
    }
    let position = position(&request)?;
    let operation = request["operation"]
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| error("epi-agent invocation requires operation"))?;
    let input = input(&request)?;

    let expected = match position {
        0 => &["anuttara.read"][..],
        1 => &["tda.vietoris-rips"][..],
        2 => &["bimba.neighborhood"][..],
        3 => &["representation.bind"][..],
        4 => &["nara.activity.validate", "nara.elemental-map"][..],
        5 => &["logos.return"][..],
        _ => unreachable!(),
    };
    if !expected.contains(&operation) {
        return Err(error(format!(
            "operation {operation} is not admitted for faculty #{position}"
        )));
    }

    let result = match operation {
        "anuttara.read" => {
            let reference = input["reference"]
                .as_str()
                .ok_or_else(|| error("anuttara.read requires reference"))?;
            let limit = input["max_relations"].as_u64().unwrap_or(128) as usize;
            epi_agent::anuttara_read(reference, limit).map_err(error)?
        }
        "tda.vietoris-rips" => {
            let request: TdaRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::persistent_homology(request).map_err(error)?
        }
        "bimba.neighborhood" => {
            let reference = input["reference"]
                .as_str()
                .ok_or_else(|| error("bimba.neighborhood requires reference"))?;
            let limit = input["max_relations"].as_u64().unwrap_or(256) as usize;
            epi_agent::bimba_neighborhood(reference, limit).map_err(error)?
        }
        "representation.bind" => {
            let request: RepresentationBindingRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::bind_representation(request).map_err(error)?
        }
        "nara.activity.validate" => {
            let log = input
                .get("activity")
                .cloned()
                .ok_or_else(|| error("nara.activity.validate requires activity"))?;
            epi_agent::validate_nara_activity(log).map_err(error)?
        }
        "nara.elemental-map" => {
            let request: NaraElementalRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_elemental_map(request).map_err(error)?
        }
        "logos.return" => {
            let request: LogosReturnRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::logos_return(request).map_err(error)?
        }
        _ => unreachable!(),
    };

    Ok(json!({
        "schema":"ql.epi-logos-agent-invocation-result/v1",
        "position":format!("#{position}"),
        "operation":operation,
        "result":result,
        "canonical_mutation":false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_position_operation_is_refused_before_dispatch() {
        let dir = tempfile_dir();
        let request = json!({
            "schema":EPI_AGENT_INVOCATION_VERSION,
            "position":"#0",
            "operation":"logos.return",
            "input":{"inquiry_ref":"i"}
        });
        std::fs::write(&dir.1, request.to_string()).unwrap();
        let error = invoke(dir.1.to_str().unwrap()).unwrap_err().to_string();
        assert!(error.contains("not admitted for faculty #0"), "{error}");
        let _ = std::fs::remove_file(dir.1);
    }

    fn tempfile_dir() -> (std::path::PathBuf, std::path::PathBuf) {
        let root = std::env::temp_dir();
        let path = root.join(format!(
            "ql-epi-agent-{}-{}.json",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        (root, path)
    }
}
