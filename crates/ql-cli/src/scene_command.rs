//! `ql scene compose`: one integrated M1–M2–M3 event from a dated sky.

use crate::CliError;
use crate::nara_command::{SnapshotPurpose, sky_snapshot};
use ql_mef::scene::{SceneTuning, compose};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::time::Duration;

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
    let usage = || {
        CliError(
            "usage: ql scene <compose|binding|world> <request.json|-> [--json]\n       \
             ql scene render <request.json|-> --worker <path> --seconds N --out <path> [--json]"
                .into(),
        )
    };
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
            let admission = purpose.admission(&request.sky)?;
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
        // The played scene rendered offline through the actual native pipeline
        // (issue #281, G6 offline clause): binding → native host → WAV.
        "render" => {
            let mut worker = None;
            let mut out = None;
            let mut seconds = 1.0_f64;
            let mut i = 2;
            while i < args.len() {
                let flag = args[i].as_str();
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| CliError(format!("scene render flag `{flag}` needs a value")))?;
                match flag {
                    "--worker" => worker = Some(value),
                    "--out" => out = Some(value),
                    "--seconds" => {
                        seconds = value
                            .parse::<f64>()
                            .map_err(|_| CliError(format!("invalid --seconds `{value}`")))?;
                        if !seconds.is_finite() || seconds <= 0.0 {
                            return Err(CliError("--seconds must be finite and positive".into()));
                        }
                    }
                    other => return Err(CliError(format!("unknown scene render flag `{other}`"))),
                }
                i += 2;
            }
            let worker = worker
                .ok_or_else(|| CliError("scene render requires --worker <path>".into()))?
                .as_str();
            let out = out
                .ok_or_else(|| CliError("scene render requires --out <path>".into()))?
                .as_str();
            let request: ql_mef::continuous::scene_field::BindingRequest =
                serde_json::from_slice(&read(&args[1])?).map_err(|e| CliError(e.to_string()))?;
            let binding = ql_mef::continuous::scene_field::binding(request).map_err(CliError)?;
            let config: ql_mef::continuous::scene_field::SceneConfig =
                serde_json::from_value(binding["host"].clone())
                    .map_err(|e| CliError(e.to_string()))?;
            let sample_rate = config.field.sample_rate;
            let mut instrument = ql_mef::continuous::scene_field::SceneInstrument::open(
                std::path::Path::new(worker),
                config,
                Duration::from_secs(20),
            )
            .map_err(CliError)?;
            let frames = (seconds * f64::from(sample_rate)).ceil() as u64;
            // The canonical container stores its sizes in 32 bits.
            if frames == 0 || frames > (u32::MAX - 36) as u64 / 2 {
                return Err(CliError(
                    "render length exceeds the 32-bit WAV container".into(),
                ));
            }
            let frames = frames as usize;
            const BLOCK: usize = 8192;
            let block = u32::try_from(BLOCK).unwrap();
            let blocks = frames.div_ceil(BLOCK);
            let mut pcm = Vec::with_capacity(frames);
            for _ in 0..blocks {
                let field = instrument
                    .session_mut()
                    .advance_field(block, false)
                    .map_err(CliError)?;
                let audio = field["audio"]
                    .as_array()
                    .ok_or_else(|| CliError("the worker field carried no audio array".into()))?;
                for sample in audio {
                    pcm.push(sample.as_f64().ok_or_else(|| {
                        CliError("the worker audio carried a non-numeric sample".into())
                    })?);
                }
            }
            pcm.truncate(frames);
            let wav = ql_mef::wav16(&pcm, sample_rate);
            std::fs::write(out, &wav).map_err(|e| CliError(e.to_string()))?;
            let receipt = serde_json::json!({
                "schema": "ql.scene-render/v1",
                "frames": frames,
                "sample_rate": sample_rate,
                "seconds": seconds,
                "out": out,
                "pcm_sha256": format!("{:x}", Sha256::digest(&wav[44..])),
                "worker": worker,
                "blocks": blocks,
            });
            return serde_json::to_string(&receipt).map_err(|e| CliError(e.to_string()));
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
    scene["sky_admission"] = request.snapshot_purpose.admission(&sky)?;
    serde_json::to_string_pretty(&scene).map_err(|e| CliError(e.to_string()))
}
