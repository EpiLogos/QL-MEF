//! `ql epi-agent` — source-qualified domain surface for the Prime-QL body.

use crate::CliError;
use ql_mef::epi_agent::{
    self, EPI_AGENT_INVOCATION_VERSION, LogosReturnRequest, NaraElementalRequest,
    NaraJourneyApplyRequest, NaraJourneyOpenRequest, NaraJourneyReadRequest,
    NaraPersonalReceiveRequest, RepresentationBindingRequest, TdaRequest,
};
use serde_json::{Value, json};
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

fn input(value: &Value) -> R<&Value> {
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
        0 => &["anuttara.read", "ananda.m1-2"][..],
        1 => &["tda.vietoris-rips"][..],
        2 => &["bimba.neighborhood"][..],
        3 => &[
            "representation.bind",
            "mahamaya.tarot-score.resolve",
            "mahamaya.phase-bridge.read",
        ][..],
        4 => &[
            "nara.activity.validate",
            "nara.elemental-map",
            "nara.personal-receive",
            "nara.journey.open",
            "nara.journey.apply",
            "nara.journey.read",
            "nara.lived-context.compose",
        ][..],
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
        "ananda.m1-2" => epi_agent::ananda_m1_2(input.clone()).map_err(error)?,
        "tda.vietoris-rips" => {
            let request: TdaRequest = serde_json::from_value(input.clone()).map_err(error)?;
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
        "mahamaya.tarot-score.resolve" => {
            epi_agent::mahamaya_tarot_score(input.clone()).map_err(error)?
        }
        "mahamaya.phase-bridge.read" => {
            epi_agent::mahamaya_phase_bridge(input.clone()).map_err(error)?
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
        "nara.personal-receive" => {
            let request: NaraPersonalReceiveRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_personal_receive(request).map_err(error)?
        }
        "nara.journey.open" => {
            let request: NaraJourneyOpenRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_journey_open(request).map_err(error)?
        }
        "nara.journey.apply" => {
            let request: NaraJourneyApplyRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_journey_apply(request).map_err(error)?
        }
        "nara.journey.read" => {
            let request: NaraJourneyReadRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_journey_read(request).map_err(error)?
        }
        "nara.lived-context.compose" => {
            let request: ql_mef::nara::lived_context::LivedContextRequest =
                serde_json::from_value(input.clone()).map_err(error)?;
            epi_agent::nara_lived_context(request).map_err(error)?
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

    /// The dated-sky fixture with its admission binding refreshed to the
    /// current native registry — the same refresh the qualified transit
    /// admission requires of every producer of an occasion sky.
    fn kairos_sky_input() -> Value {
        let mut sky: Value = serde_json::from_str(include_str!(
            "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
        ))
        .unwrap();
        sky["source_binding"]["registry_revision"] =
            json!(ql_mef::m2::catalogue().registry_revision());
        sky
    }

    fn write_request(value: &Value) -> (std::path::PathBuf, std::path::PathBuf) {
        let dir = tempfile_dir();
        std::fs::write(&dir.1, value.to_string()).unwrap();
        dir
    }

    #[test]
    fn mahamaya_operations_are_refused_outside_faculty_three() {
        for operation in ["mahamaya.tarot-score.resolve", "mahamaya.phase-bridge.read"] {
            let (root, path) = write_request(&json!({
                "schema":EPI_AGENT_INVOCATION_VERSION,
                "position":"#2",
                "operation":operation,
                "input":{}
            }));
            let error = invoke(path.to_str().unwrap()).unwrap_err().to_string();
            assert!(
                error.contains("not admitted for faculty #2"),
                "{operation}: {error}"
            );
            let _ = std::fs::remove_file(path);
            let _ = root;
        }
    }

    #[test]
    fn mahamaya_tarot_score_resolves_deterministically_through_the_cli() {
        let request = json!({
            "schema":EPI_AGENT_INVOCATION_VERSION,
            "position":"#3",
            "operation":"mahamaya.tarot-score.resolve",
            "input":{
                "subject_ref":"ql:k2/default-subject",
                "locus_ref":"#2-5-4",
                "occasion_sky":kairos_sky_input(),
                "clock_steps":359,
                "primary_anchor":{"kind":"kairos","body_index":0}
            }
        });
        let (first_root, first_path) = write_request(&request);
        let first = invoke(first_path.to_str().unwrap()).unwrap();
        let second = invoke(first_path.to_str().unwrap()).unwrap();
        assert_eq!(first["position"], "#3");
        let result_a = &first["result"];
        assert_eq!(result_a["schema"], "ql.tarot-score/v1");
        assert_eq!(result_a["operation"], "mahamaya.tarot-score.resolve");
        assert_eq!(result_a["primary_token_role"], "kairos/Sun");
        assert!(!result_a["tokens"].as_array().unwrap().is_empty());
        // Determinism: two invocations of one request file, byte-equal results.
        assert_eq!(
            serde_json::to_string(&first["result"]).unwrap(),
            serde_json::to_string(&second["result"]).unwrap()
        );
        let _ = std::fs::remove_file(first_path);
        let _ = first_root;
        // A basis with no admitted anchor refuses by name through the CLI.
        let empty = json!({
            "schema":EPI_AGENT_INVOCATION_VERSION,
            "position":"#3",
            "operation":"mahamaya.tarot-score.resolve",
            "input":{
                "subject_ref":"ql:k2/default-subject",
                "locus_ref":"#2-5-4",
                "clock_steps":359,
                "primary_anchor":{"kind":"kairos","body_index":0}
            }
        });
        let (empty_root, empty_path) = write_request(&empty);
        let error = invoke(empty_path.to_str().unwrap())
            .unwrap_err()
            .to_string();
        assert!(error.contains("no anchor basis"), "{error}");
        let _ = std::fs::remove_file(empty_path);
        let _ = empty_root;
    }

    #[test]
    fn mahamaya_phase_bridge_reads_the_three_declared_clock_positions() {
        for (steps, layer, covers) in [(0u64, 0u8, 0u64), (360, 1, 0), (720, 0, 1)] {
            let (root, path) = write_request(&json!({
                "schema":EPI_AGENT_INVOCATION_VERSION,
                "position":"#3",
                "operation":"mahamaya.phase-bridge.read",
                "input":{"steps":steps}
            }));
            let result = invoke(path.to_str().unwrap()).unwrap()["result"].clone();
            assert_eq!(result["schema"], "ql.pole.phase-bridge/v1");
            assert_eq!(result["clock"]["steps"], json!(steps), "steps {steps}");
            assert_eq!(result["clock"]["layer"], json!(layer), "steps {steps}");
            assert_eq!(
                result["clock"]["completed_double_covers"],
                json!(covers),
                "steps {steps}"
            );
            let lut = result["ring_lut"].as_array().unwrap();
            assert_eq!(lut.len(), 12);
            assert_eq!(lut[0]["tick"], json!(0));
            assert_eq!(lut[0]["clock_steps"], json!(0));
            assert_eq!(lut[11]["clock_steps"], json!(360));
            assert!(result["registers"]["conversion"].is_string());
            assert_eq!(
                result["source"]["module"],
                "crates/ql-core/src/pole/phase.rs"
            );
            let _ = std::fs::remove_file(path);
            let _ = root;
        }
        // Missing steps refuse before any register is read.
        let (root, path) = write_request(&json!({
            "schema":EPI_AGENT_INVOCATION_VERSION,
            "position":"#3",
            "operation":"mahamaya.phase-bridge.read",
            "input":{}
        }));
        let error = invoke(path.to_str().unwrap()).unwrap_err().to_string();
        assert!(error.contains("requires steps"), "{error}");
        let _ = std::fs::remove_file(path);
        let _ = root;
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
