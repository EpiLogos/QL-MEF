//! `ql context-frame read <request.json|-> [--json]`: the External Context
//! Frame reading (`read_external_context_frame`) over a sixfold mapping and a
//! structural probe that the caller supplies. The reading is descriptive
//! evidence only. It never gives the target runtime standing, activation,
//! authority, ownership or a lifecycle state, and the command keeps no store.
use crate::CliError;
use ql_mef::{
    CONTEXT_FRAME_TARGET_READING_VERSION, ContextFrameId, ContextFrameReadingOrigin,
    ContextFrameReadingStatus, ContextFrameStructuralProbe, ExternalContextFrameReading,
    ExternalSixfoldMapping, MefGrain, MefUnitFace, read_external_context_frame,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::io::Read;

const MAX_INPUT: u64 = 1024 * 1024;
const USAGE: &str = "usage: ql context-frame read <request.json|-> [--json]";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    target_ref: String,
    mapping_source_ref: String,
    external_position_refs: [String; 6],
    probe: Probe,
    provider_ref: String,
    origin: Origin,
    #[serde(default)]
    evidence_refs: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Probe {
    local_position: u8,
    #[serde(default)]
    unit_face: Option<Face>,
    #[serde(default)]
    grain: Option<Grain>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Face {
    Name,
    Power,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Grain {
    InnerFour,
    OuterTwo,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum Origin {
    Derived,
    Proposed,
    Recognised,
}

pub(crate) fn command(args: &[String], json: bool) -> Result<String, CliError> {
    let [path] = args else {
        return Err(CliError(USAGE.into()));
    };
    let reader: Box<dyn Read> = if path == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(path).map_err(|e| CliError(e.to_string()))?)
    };
    let mut input = String::new();
    reader
        .take(MAX_INPUT + 1)
        .read_to_string(&mut input)
        .map_err(|e| CliError(e.to_string()))?;
    if input.len() as u64 > MAX_INPUT {
        return Err(CliError(
            "Context-Frame reading request exceeds 1 MiB".into(),
        ));
    }
    let document = read(&input)?;
    if json {
        serde_json::to_string_pretty(&document).map_err(CliError::from)
    } else {
        Ok(render_plain(&document))
    }
}

/// The reading of one request, as the JSON document the CLI prints.
pub(crate) fn read(input: &str) -> Result<Value, CliError> {
    let request: Request = serde_json::from_str(input)
        .map_err(|e| CliError(format!("invalid Context-Frame reading request: {e}")))?;
    let mapping = ExternalSixfoldMapping::new(
        request.target_ref,
        request.mapping_source_ref,
        request.external_position_refs,
    )
    .map_err(|e| CliError(e.to_string()))?;
    let probe = ContextFrameStructuralProbe::new(
        request.probe.local_position,
        request.probe.unit_face.map(|face| match face {
            Face::Name => MefUnitFace::Name,
            Face::Power => MefUnitFace::Power,
        }),
        request.probe.grain.map(|grain| match grain {
            Grain::InnerFour => MefGrain::InnerFour,
            Grain::OuterTwo => MefGrain::OuterTwo,
        }),
    )
    .map_err(|e| CliError(e.to_string()))?;
    let origin = match request.origin {
        Origin::Derived => ContextFrameReadingOrigin::Derived,
        Origin::Proposed => ContextFrameReadingOrigin::Proposed,
        Origin::Recognised => ContextFrameReadingOrigin::Recognised,
    };
    let reading = read_external_context_frame(
        &mapping,
        probe,
        request.provider_ref,
        origin,
        request.evidence_refs,
    )
    .map_err(|e| CliError(e.to_string()))?;
    Ok(document(&mapping, &reading))
}

fn frame(frame: ContextFrameId) -> Value {
    json!({"code": frame.code(), "expression": frame.expression(), "name": frame.name()})
}

fn document(mapping: &ExternalSixfoldMapping, reading: &ExternalContextFrameReading) -> Value {
    let probe = reading.probe();
    let status = match reading.status() {
        ContextFrameReadingStatus::Exact(found) => json!({"kind": "exact", "frame": frame(*found)}),
        ContextFrameReadingStatus::Partial {
            frame: found,
            missing_face,
            missing_grain,
        } => json!({
            "kind": "partial",
            "frame": frame(*found),
            "missing_face": missing_face,
            "missing_grain": missing_grain,
        }),
        ContextFrameReadingStatus::Ambiguous(frames) => json!({
            "kind": "ambiguous",
            "frames": frames.iter().copied().map(frame).collect::<Vec<_>>(),
        }),
        ContextFrameReadingStatus::NoReading => json!({"kind": "no-reading"}),
    };
    json!({
        "contract": CONTEXT_FRAME_TARGET_READING_VERSION,
        "target_ref": reading.target_ref(),
        "mapping_source_ref": reading.mapping_source_ref(),
        "mapping_digest": reading.mapping_digest(),
        "external_position_refs": mapping.external_position_refs(),
        "provider_ref": reading.provider_ref(),
        "operator_version": reading.operator_version(),
        "origin": match reading.origin() {
            ContextFrameReadingOrigin::Derived => "derived",
            ContextFrameReadingOrigin::Proposed => "proposed",
            ContextFrameReadingOrigin::Recognised => "recognised",
        },
        "probe": {
            "local_position": probe.local_position().value(),
            "unit_face": probe.unit_face().map(|face| match face {
                MefUnitFace::Name => "name",
                MefUnitFace::Power => "power",
            }),
            "grain": probe.grain().map(|grain| match grain {
                MefGrain::InnerFour => "inner-four",
                MefGrain::OuterTwo => "outer-two",
            }),
        },
        "status": status,
        "evidence_refs": reading.evidence_refs(),
        "runtime_authority": reading.is_runtime_authority(),
    })
}

fn render_plain(document: &Value) -> String {
    let status = &document["status"];
    let frames = |value: &Value| {
        value["code"]
            .as_str()
            .map(|code| format!("{code} {}", value["expression"].as_str().unwrap_or("")))
            .unwrap_or_default()
    };
    let verdict = match status["kind"].as_str() {
        Some("exact") => format!("exact: {}", frames(&status["frame"])),
        Some("partial") => format!(
            "partial: {} (face {}, grain {})",
            frames(&status["frame"]),
            if status["missing_face"] == true {
                "missing"
            } else {
                "given"
            },
            if status["missing_grain"] == true {
                "missing"
            } else {
                "given"
            }
        ),
        Some("ambiguous") => format!(
            "ambiguous: {}",
            status["frames"]
                .as_array()
                .into_iter()
                .flatten()
                .map(frames)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => "no reading".into(),
    };
    format!(
        "{} at local position {}: {verdict}\nmapping {} ({})\nprovider {} · {} · descriptive evidence only",
        document["target_ref"].as_str().unwrap_or("?"),
        document["probe"]["local_position"],
        document["mapping_source_ref"].as_str().unwrap_or("?"),
        document["mapping_digest"].as_str().unwrap_or("?"),
        document["provider_ref"].as_str().unwrap_or("?"),
        document["operator_version"].as_str().unwrap_or("?"),
    )
}
