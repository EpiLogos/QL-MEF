//! `ql scene compose`: one integrated M1–M2–M3 event from a dated sky.

use crate::CliError;
use crate::nara_command::sky_snapshot;
use ql_mef::scene::{SceneTuning, compose};
use serde_json::Value;
use std::io::Read;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneRequest {
    schema: String,
    sky_request: Option<Value>,
    sky_snapshot: Option<Value>,
    tick12: u8,
    cycle: u64,
    #[serde(default)]
    tuning: Option<SceneTuning>,
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    let usage = || CliError("usage: ql scene compose <request.json|-> [--json]".into());
    if args.first().map(String::as_str) != Some("compose") || args.len() < 2 {
        return Err(usage());
    }
    let mut bytes = Vec::new();
    if args[1] == "-" {
        std::io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|e| CliError(e.to_string()))?;
    } else {
        bytes = std::fs::read(&args[1]).map_err(|e| CliError(e.to_string()))?;
    }
    let request: SceneRequest =
        serde_json::from_slice(&bytes).map_err(|e| CliError(e.to_string()))?;
    if request.schema != "ql.scene-request/v1" {
        return Err(CliError("unsupported scene request".into()));
    }
    let sky = match (&request.sky_request, &request.sky_snapshot) {
        (Some(sky), None) => sky_snapshot(sky, false)?,
        (None, Some(sky)) => sky_snapshot(sky, true)?,
        _ => {
            return Err(CliError(
                "a scene requires exactly one sky_request or sky_snapshot".into(),
            ));
        }
    };
    let scene = compose(
        &sky,
        request.tick12,
        request.cycle,
        request.tuning.unwrap_or_default(),
    )
    .map_err(CliError)?;
    serde_json::to_string_pretty(&scene).map_err(|e| CliError(e.to_string()))
}
