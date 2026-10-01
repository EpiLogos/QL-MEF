//! `ql scene compose`: one integrated M1–M2–M3 event from a dated sky.

use crate::CliError;
use crate::nara_command::{SnapshotPurpose, sky_snapshot};
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
    #[serde(default)]
    snapshot_purpose: SnapshotPurpose,
}

fn read(path: &str) -> Result<Vec<u8>, CliError> {
    let mut bytes = Vec::new();
    if path == "-" {
        std::io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|e| CliError(e.to_string()))?;
    } else {
        bytes = std::fs::read(path).map_err(|e| CliError(e.to_string()))?;
    }
    Ok(bytes)
}

pub fn command(args: &[String]) -> Result<String, CliError> {
    let usage =
        || CliError("usage: ql scene <compose|binding|world> <request.json|-> [--json]".into());
    if args.len() < 2 {
        return Err(usage());
    }
    match args[0].as_str() {
        "compose" => {}
        "world" => {
            // Purpose belongs to this provider boundary, not the numerical
            // WorldRequest. All remaining fields retain its strict admission.
            let mut input: Value =
                serde_json::from_slice(&read(&args[1])?).map_err(|e| CliError(e.to_string()))?;
            let purpose: SnapshotPurpose = input
                .as_object_mut()
                .ok_or_else(|| CliError("world request must be an object".into()))?
                .remove("snapshot_purpose")
                .map(serde_json::from_value)
                .transpose()
                .map_err(|e| CliError(e.to_string()))?
                .unwrap_or_default();
            let mut request: ql_mef::scene::WorldRequest =
                serde_json::from_value(input).map_err(|e| CliError(e.to_string()))?;
            request.sky = sky_snapshot(&request.sky, true, purpose)?;
            let admission = purpose.admission(&request.sky);
            let mut world = ql_mef::scene::world(request).map_err(CliError)?;
            world["sky_admission"] = admission;
            return serde_json::to_string(&world).map_err(|e| CliError(e.to_string()));
        }
        // The continuous-field host binding the O:I Live instrument opens.
        "binding" => {
            let request: ql_mef::continuous::scene_field::BindingRequest =
                serde_json::from_slice(&read(&args[1])?).map_err(|e| CliError(e.to_string()))?;
            let binding = ql_mef::continuous::scene_field::binding(request).map_err(CliError)?;
            return serde_json::to_string(&binding).map_err(|e| CliError(e.to_string()));
        }
        _ => return Err(usage()),
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
        (Some(sky), None) => sky_snapshot(sky, false, request.snapshot_purpose)?,
        (None, Some(sky)) => sky_snapshot(sky, true, request.snapshot_purpose)?,
        _ => {
            return Err(CliError(
                "a scene requires exactly one sky_request or sky_snapshot".into(),
            ));
        }
    };
    let mut scene = compose(
        &sky,
        request.tick12,
        request.cycle,
        request.tuning.unwrap_or_default(),
    )
    .map_err(CliError)?;
    scene["sky_admission"] = request.snapshot_purpose.admission(&sky);
    serde_json::to_string_pretty(&scene).map_err(|e| CliError(e.to_string()))
}
