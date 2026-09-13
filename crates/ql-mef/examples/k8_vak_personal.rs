//! Cross-owner K8 acceptance: consume an owner-executed Factory Vāk receipt,
//! voice the existing dual-bus material owner through QL's accepted C′/musical
//! relation, and receive that same acknowledged world event as one Nara.
//!
//! Factory remains the execution/attempt owner. QL owns the CF→mode and M2
//! musical/material relations. #134 owns PersonalFieldInstance and its explicit
//! seven receiver mappings. This harness starts no second world simulation.

use ql_mef::continuous::coupled::{CoupledInput, REQUEST_V2, REQUEST_V3};
use ql_mef::continuous::personal::PersonalCoupledSession;
use ql_mef::continuous::FieldInput;
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyConstitution, EventBasisRefs, LifecycleState,
    PersonalConstitution, PersonalEventInput, PersonalLayer, ReceiverConstitution,
    ReceiverEventInput, SourceRevision, WorldContribution,
};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

fn read(path: &str) -> Result<Value, String> {
    if std::fs::metadata(path).map_err(|error| error.to_string())?.len() > 32 * 1024 * 1024 {
        return Err("input exceeds 32 MiB".into());
    }
    serde_json::from_slice(&std::fs::read(path).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn source(name: &str, factory_source_sha: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:k8-vak-personal:{name}"),
        revision: factory_source_sha.into(),
        standing_ref: "controlled-cross-owner-k8-acceptance".into(),
    }
}

fn layer(
    name: &str,
    quaternion: BioQuaternion,
    observed_at_unix_ms: u64,
    factory_source_sha: &str,
) -> PersonalLayer {
    PersonalLayer {
        source: source(name, factory_source_sha),
        observed_at_unix_ms,
        quaternion,
    }
}

fn constitution(
    subject: &str,
    observed_at_unix_ms: u64,
    factory_source_sha: &str,
) -> PersonalConstitution {
    PersonalConstitution {
        subject_id: subject.into(),
        constitution_ref: format!("constitution:{subject}:k8-vak-personal-v1"),
        source_revisions: vec![source("controlled-personal-constitution", factory_source_sha)],
        consent: ConsentState::Granted,
        lifecycle: LifecycleState::Active,
        identity: layer(
            "identity",
            BioQuaternion {
                w: 1.0,
                x: 0.03,
                y: 0.01,
                z: 0.0,
            },
            observed_at_unix_ms,
            factory_source_sha,
        ),
        transit: layer(
            "transit",
            BioQuaternion {
                w: 1.0,
                x: 0.0,
                y: 0.05,
                z: 0.02,
            },
            observed_at_unix_ms,
            factory_source_sha,
        ),
        activity: layer(
            "activity",
            BioQuaternion {
                w: 1.0,
                x: 0.02,
                y: 0.0,
                z: 0.04,
            },
            observed_at_unix_ms,
            factory_source_sha,
        ),
        ephemeral: Some(layer(
            "ephemeral",
            BioQuaternion::IDENTITY,
            observed_at_unix_ms,
            factory_source_sha,
        )),
        earth_body: EarthBodyConstitution {
            source: source("earth-body", factory_source_sha),
            frame_ref: "earth-fixed:k8-cross-owner".into(),
            orientation: BioQuaternion::IDENTITY,
        },
        receivers: (0u8..7)
            .map(|ordinal| ReceiverConstitution {
                ordinal,
                label: format!("source-centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}"), factory_source_sha),
                world_weights: [1.0 + f64::from(ordinal) / 10.0, 0.5, 0.25],
                orientation: BioQuaternion {
                    w: 1.0,
                    x: f64::from(ordinal) / 20.0,
                    y: 0.01,
                    z: 0.0,
                },
                resonance_gain: 0.8 + f64::from(ordinal) / 20.0,
                reradiation_gain: 0.4,
            })
            .collect(),
    }
}

fn reception(refs: &EventBasisRefs, observed_at_unix_ms: u64) -> PersonalEventInput {
    PersonalEventInput {
        event_ref: refs.event_ref.clone(),
        profile_generation: refs.profile_generation,
        observed_at_unix_ms,
        receivers: (0u8..7)
            .map(|ordinal| ReceiverEventInput {
                ordinal,
                m1: WorldContribution {
                    basis_ref: refs.m1_revision.clone(),
                    source_ref: format!("cross-owner:m1:{ordinal}"),
                    value: 0.7 + f64::from(ordinal) / 10.0,
                },
                m2: WorldContribution {
                    basis_ref: refs.m2_source_ref.clone(),
                    source_ref: format!("cross-owner:m2:{ordinal}"),
                    value: 1.2 + f64::from(ordinal) / 20.0,
                },
                m3: WorldContribution {
                    basis_ref: refs.m3_source_ref.clone(),
                    source_ref: format!("cross-owner:m3:{ordinal}"),
                    value: 0.4 + f64::from(ordinal) / 30.0,
                },
            })
            .collect(),
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        return Err(
            "usage: k8_vak_personal WORKER DUAL_BUS_INPUT_JSON FACTORY_RECEIPT_JSON FACTORY_SOURCE_SHA OUT_DIR RECEIPT_SHA256"
                .into(),
        );
    }
    let installed = read(&args[2])?;
    let factory = read(&args[3])?;
    let factory_source_sha = &args[4];
    let out = Path::new(&args[5]);
    let factory_receipt_sha256 = &args[6];
    if factory_source_sha.len() != 40 || !factory_source_sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Factory source SHA must be an exact Git commit".into());
    }
    if factory_receipt_sha256.len() != 64
        || !factory_receipt_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("Factory receipt digest must be SHA-256 hex".into());
    }
    if factory["contract"] != "factory.vak-orchestration/v1" {
        return Err("not a Factory Vāk performance receipt".into());
    }
    let subject = factory["subjectRef"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or("Factory receipt lacks subjectRef")?
        .to_owned();
    let mut basis: CoupledInput = serde_json::from_value(installed["basis"].clone())
        .map_err(|error| error.to_string())?;
    let mut field: FieldInput = serde_json::from_value(installed["field"].clone())
        .map_err(|error| error.to_string())?;
    if basis.schema != REQUEST_V2 {
        return Err("cross-owner acceptance requires the accepted dual-bus K8 input".into());
    }
    basis.schema = REQUEST_V3.into();
    basis.m3.subject_ref.clone_from(&subject);
    field.subject_ref.clone_from(&subject);
    basis.source_receipts.push(factory.clone());

    let composed = basis.compose()?;
    let performance = &composed.derivation["factory_vak_performance"];
    if performance["subject_ref"] != subject
        || performance["performance_ref"] != factory["performanceRef"]
        || performance["run_ref"] != factory["runRef"]
    {
        return Err("Factory provenance changed during QL composition".into());
    }
    if composed.m2_input.resonator.as_ref().map_or(0, |value| value.modes.len()) != 16 {
        return Err("dual musical inputs did not retain one sixteen-mode material owner".into());
    }
    let performed_mode = composed
        .m2_input
        .vimarsha
        .as_ref()
        .ok_or("performed occasion lacks Vimarśā")?
        .musical_mode;
    let observed_at_unix_ms = composed.m2_input.at_unix_ms;
    let personal = constitution(&subject, observed_at_unix_ms, factory_source_sha);
    std::fs::create_dir_all(out).map_err(|error| error.to_string())?;

    let mut owner = PersonalCoupledSession::open(
        Path::new(&args[1]),
        basis,
        field,
        personal,
        Duration::from_secs(20),
    )?;
    let initial_field = owner.last_field().clone();
    let refs = EventBasisRefs::from_basis(owner.current_basis())?;
    let personal_input = reception(&refs, observed_at_unix_ms);
    let personal_state = owner.receive_personal(personal_input.clone())?;
    if owner.last_field() != &initial_field {
        return Err("personal reception advanced the native material owner".into());
    }
    if !owner.personal_is_current()? || personal_state.receivers.len() != 7 {
        return Err("Nara did not receive all seven centres for the performed world event".into());
    }
    let replay = owner.receive_personal(personal_input)?;
    if replay != personal_state || owner.last_field() != &initial_field {
        return Err("exact personal replay changed the acknowledged world".into());
    }
    let advanced = owner.advance_field(2048, false)?;
    if !owner.personal_is_current()? {
        return Err("native sample continuation changed the world-basis occasion".into());
    }
    if advanced["samples_elapsed"] == initial_field["samples_elapsed"] {
        return Err("installed native owner did not advance".into());
    }

    let acceptance = json!({
        "schema":"ql.k8-vak-personal-acceptance/v1",
        "factory_source_sha":factory_source_sha,
        "factory_receipt_sha256":factory_receipt_sha256,
        "factory_contract":factory["contract"],
        "factory_performance_ref":factory["performanceRef"],
        "factory_run_ref":factory["runRef"],
        "factory_run_revision":factory["runRevision"],
        "factory_actor_ref":factory["actorRef"],
        "factory_subject_ref":subject,
        "factory_frame":factory["frame"],
        "factory_thread":factory["thread"],
        "factory_sequence":factory["sequence"],
        "factory_direction":factory["direction"],
        "factory_musical_role":factory["musicalRole"],
        "factory_attempts":factory["attempts"].as_array().map_or(0, Vec::len),
        "ql_performed_musical_mode":performed_mode,
        "same_dual_bus_material_owner":owner.current_basis().m2_input.resonator.as_ref().map_or(0, |value| value.modes.len()) == 16,
        "native_field_generation":owner.last_field()["generation"],
        "native_samples_elapsed":owner.last_field()["samples_elapsed"],
        "nara_subject_ref":personal_state.subject_id,
        "nara_reception_generation":personal_state.reception_generation,
        "nara_receivers":personal_state.receivers.len(),
        "nara_current_after_native_advance":owner.personal_is_current()?,
        "personal_reception_did_not_advance_native_field":true,
        "exact_personal_replay":replay == personal_state,
        "one_runtime_owner":true,
        "standing":"Factory owner-executed Vāk performance -> QL-owned CF/music/material composition -> #134 Nara reception on one installed C++ field owner; centre inputs are controlled source-qualified acceptance values, not inferred physiology or owner-machine lived evidence"
    });
    std::fs::write(
        out.join("acceptance.json"),
        format!("{}\n", serde_json::to_string_pretty(&acceptance).unwrap()),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("factory-performance.json"),
        format!("{}\n", serde_json::to_string_pretty(&factory).unwrap()),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("inspection.json"),
        format!("{}\n", serde_json::to_string_pretty(&owner.inspect()?).unwrap()),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
