//! Installed full-event acceptance consumer. Geometry and subjects are explicitly
//! controlled here; the dated sky comes from the actual provider receipt.
use ql_mef::continuous::coupled::{
    CoupledFieldSession, CoupledInput, FrequencyBinding, HarmonicSource, REQUEST,
};
use ql_mef::continuous::{FieldInput, LiftInput};
use ql_mef::m1_engine::{EngineConfig, HarmonicSelection, M1Engine};
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::{M3Command, M3Request};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

fn read(path: &str) -> Result<Value, String> {
    if std::fs::metadata(path).map_err(|e| e.to_string())?.len() > 32 * 1024 * 1024 {
        return Err("input exceeds 32 MiB".into());
    }
    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn newer_seed(input: &mut CoupledInput) {
    input.m2.stamp.identity.profile_generation += 1;
    let identity = input.m2.stamp.identity.clone();
    if let Some(v) = input.m2.vimarsha.as_mut() {
        v.stamp.identity = identity.clone();
    }
    if let Some(v) = input.m2.m1_excitation.as_mut() {
        v.stamp.identity = identity.clone();
    }
    if let Some(v) = input.m2.resonator.as_mut() {
        v.stamp.identity = identity.clone();
    }
    if let Some(v) = input.m2.condition.as_mut().and_then(|v| v.palette.as_mut()) {
        v.stamp.identity = identity;
    }
}
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: k8_coupled WORKER SKY_M2_EVENT INITIAL_FIELD_JSON OUT_DIR".into());
    }
    let event = read(&args[2])?;
    let initial = read(&args[3])?;
    let field: FieldInput =
        serde_json::from_value(initial["field"].clone()).map_err(|e| e.to_string())?;
    let sky_m2: M2Request =
        serde_json::from_value(event["m2_input"].clone()).map_err(|e| e.to_string())?;
    let config: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(config["config"].clone()).unwrap();
    let mut m2: M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    m1.event_ref.clone_from(&sky_m2.stamp.identity.event_ref);
    m2.stamp = sky_m2.stamp.clone();
    m2.at_unix_ms = sky_m2.at_unix_ms;
    m2.world_observations = sky_m2.world_observations.clone();
    m2.resonator = sky_m2.resonator.clone();
    m2.m1_excitation = None; // no fixture zero-vector is represented as a live M1 observation
    m2.vimarsha.as_mut().unwrap().stamp.identity = m2.stamp.identity.clone();
    if let Some(palette) = m2.condition.as_mut().and_then(|c| c.palette.as_mut()) {
        palette.stamp.identity = m2.stamp.identity.clone();
    }
    let m3 = M3Request {
        schema: ql_mef::m3_state::REQUEST_SCHEMA.into(),
        registry_revision: m2.registry_revision.clone(),
        stamp: ql_mef::m2_engine::InputStamp {
            identity: m2.stamp.identity.clone(),
            source_ref: "controlled:chosen-M3-form".into(),
            contract_ref: ql_mef::m3_state::REQUEST_SCHEMA.into(),
        },
        subject_ref: field.subject_ref.clone(),
        occurrence_unix_ms: m2.at_unix_ms,
        receipt_unix_ms: m2.at_unix_ms,
        clock_steps: 359,
        address: 7,
        pose: 6,
        aperture: 2,
        matrix_axis: 0,
        rna: false,
        bases: vec![],
        m2_basis: Some(m2.stamp.clone()),
    };
    let bindings = m2
        .resonator
        .as_ref()
        .unwrap()
        .modes
        .iter()
        .enumerate()
        .map(|(i, mode)| FrequencyBinding {
            mode_ref: mode.mode_ref.clone(),
            octet_index: i as u8,
        })
        .collect();
    let input = CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: bindings,
        source_receipts: vec![event["sky"].clone()],
    };
    let original = serde_json::to_value(&input).unwrap();
    let out = Path::new(&args[4]);
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let save = |name: &str, value: &Value| -> Result<(), String> {
        std::fs::write(
            out.join(name),
            format!("{}\n", serde_json::to_string_pretty(value).unwrap()),
        )
        .map_err(|e| e.to_string())
    };
    save("input.json", &json!({"basis":input, "field":field}))?;
    let mut owner = CoupledFieldSession::open(
        Path::new(&args[1]),
        input.clone(),
        field.clone(),
        Duration::from_secs(20),
    )?;
    let mut states = vec![owner.snapshot()];
    assert_eq!(owner.snapshot(), owner.snapshot()); // no second integration on view focus
    assert_eq!(
        owner.current_basis().m2["world"].as_array().unwrap().len(),
        10
    );
    assert_eq!(
        owner.current_basis().m2["aspects"]
            .as_array()
            .unwrap()
            .len(),
        45
    );
    assert_eq!(
        owner.current_basis().m2["modal"]["coefficients"]
            .as_array()
            .unwrap()
            .len(),
        72
    );
    assert!(owner.current_basis().m2["condition"].is_object());
    for (mode, frequency) in owner
        .current_basis()
        .m2_input
        .resonator
        .as_ref()
        .unwrap()
        .modes
        .iter()
        .zip(
            owner.current_basis().m2["vimarsha"]["reading"]["audio_octet_hz"]
                .as_array()
                .unwrap(),
        )
    {
        assert_eq!(mode.frequency_hz, frequency.as_f64().unwrap());
    }
    states.push(owner.advance(2048, false)?);
    let active = owner.snapshot();
    let read = owner.read()?;
    assert_eq!(read["field"]["targets"], active["field"]["targets"]);
    assert_eq!(read["field"]["clock"], active["field"]["clock"]);
    let axis = LiftInput {
        turns: "-2".into(),
        half_degrees: 41,
    };
    states.push(owner.set_axis(1, axis.clone())?);
    let before = owner.snapshot();
    let mut changed = input.clone();
    newer_seed(&mut changed);
    let mut m1_owner = M1Engine::new(changed.m1.clone())?;
    m1_owner.configure_harmonics(
        changed.m1.revision.parse::<u64>().unwrap(),
        HarmonicSelection {
            family: 1,
            row12: 7,
            col12: changed.m1.col12,
            flowering_substage: changed.m1.flowering_substage,
            lens12: 1,
            context_frame: 2,
            basis: changed.m1.basis,
        },
    )?;
    changed.m1 = m1_owner.config().clone();
    changed.harmonic_source = HarmonicSource::SelectedSourceRow;
    let command: M3Command = serde_json::from_value(json!({
        "schema":"ql.m3-command/v1", "event_ref":changed.m3.stamp.identity.event_ref,
        "subject_ref":changed.m3.subject_ref, "expected_generation":changed.m3.stamp.identity.profile_generation,
        "actor_ref":"controlled:authorised-test-host", "cause_ref":"controlled:form-and-transcription",
        "occurrence_unix_ms":changed.m3.occurrence_unix_ms, "receipt_unix_ms":changed.m3.receipt_unix_ms,
        "operations":[{"operation":"apply-matrix","family":0},{"operation":"set-pose","pose":0},
                      {"operation":"transcribe","rna":true},{"operation":"advance-clock","steps":1}]
    })).unwrap();
    changed.m3_commands.push(command);
    states.push(owner.replace(changed.clone())?);
    let updated = owner.snapshot();
    assert_eq!(
        updated["field"]["amplitudes_metres"],
        before["field"]["amplitudes_metres"]
    );
    assert_eq!(
        updated["field"]["samples_elapsed"],
        before["field"]["samples_elapsed"]
    );
    assert_eq!(updated["field"]["clock"], before["field"]["clock"]);
    assert_ne!(
        updated["basis"]["m2"]["vimarsha"],
        before["basis"]["m2"]["vimarsha"]
    );
    assert!(
        updated["basis"]["m3"]["transcription"]["rna"]
            .as_bool()
            .unwrap()
    );
    assert_eq!(updated["basis"]["m3"]["clock"]["steps"], 360);
    assert_eq!(owner.current_basis().m2_input.world_observations.len(), 10);
    assert_eq!(
        serde_json::to_value(&owner.original_basis().input).unwrap(),
        original
    );
    assert_eq!(
        serde_json::to_value(owner.original_field()).unwrap(),
        serde_json::to_value(&field).unwrap()
    );
    assert!(owner.replace(changed.clone()).is_err()); // stale full-basis replacement
    assert_eq!(owner.snapshot(), updated);
    let mut invalid = changed.clone();
    invalid.m3.pose = 8;
    newer_seed(&mut invalid);
    assert!(owner.replace(invalid).is_err());
    assert_eq!(owner.snapshot(), updated);
    let mut invalid = changed.clone();
    invalid.m3.subject_ref = "other-subject".into();
    newer_seed(&mut invalid);
    assert!(owner.replace(invalid).is_err());
    assert_eq!(owner.snapshot(), updated);
    states.push(owner.advance(512, true)?);
    assert!(
        owner.snapshot()["field"]["audio"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v.as_f64() == Some(0.0))
    );
    for _ in 0..26 {
        states.push(owner.advance(256, false)?);
    }
    let mut replay =
        CoupledFieldSession::open(Path::new(&args[1]), input, field, Duration::from_secs(20))?;
    let mut repeated = vec![replay.snapshot(), replay.advance(2048, false)?];
    replay.read()?;
    repeated.push(replay.set_axis(1, axis)?);
    repeated.push(replay.replace(changed)?);
    repeated.push(replay.advance(512, true)?);
    for _ in 0..26 {
        repeated.push(replay.advance(256, false)?);
    }
    assert_eq!(states, repeated);
    let frames: String = states.iter().map(|v| format!("{}\n", v["field"])).collect();
    std::fs::write(out.join("frames.jsonl"), frames).map_err(|e| e.to_string())?;
    save("original-event.json", &states[0])?;
    save("changed-event.json", &states[3])?;
    save("last-event.json", states.last().unwrap())?;
    save(
        "acceptance.json",
        &json!({"schema":"ql.k8-coupled-acceptance/v1", "frames":states.len(),
        "full_M1_M2_M3":true, "native_harmonic_and_pose_to_material":true, "originals_retained":true,
        "atomic_full_basis_adoption":true,"stale_and_invalid_refused":true,"one_continuous_owner":true,
        "exact_replay":true,"current_sky_snapshot":event["sky"]["snapshot_ref"],
        "limits":["controlled source-linked geometry, not a measured eigensystem","no Nara reception proof","no local desktop or acoustic-device claim"]}),
    )?;
    println!(
        "PASS: complete native M1/M2/M3 -> persistent C++ material field; atomic replacement, unchanged original sky, exact replay"
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
