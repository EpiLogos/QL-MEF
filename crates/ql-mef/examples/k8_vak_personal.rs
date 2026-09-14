//! Cross-owner K8 acceptance: execute Factory's retained Vāk owner, project that
//! exact snapshot through QL's accepted performance semantics, voice the existing
//! dual-bus material owner, and receive the same acknowledged world as one Nara.
//!
//! Factory remains the execution/attempt owner. QL owns C′/musical meaning and
//! M2 material projection. #134 owns PersonalFieldInstance and its explicit seven
//! receiver mappings. This harness starts no second world simulation.

use ql_core::{CallerProvenance, QlFace};
use ql_mef::continuous::FieldInput;
use ql_mef::continuous::coupled::{CoupledInput, REQUEST_V2, REQUEST_V3};
use ql_mef::continuous::personal::PersonalCoupledSession;
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyConstitution, EventBasisRefs, LifecycleState,
    PersonalConstitution, PersonalEventInput, PersonalLayer, ReceiverConstitution,
    ReceiverEventInput, SourceRevision, WorldContribution,
};
use ql_mef::vak_composition::{ActiveFrame, Basis, PositionBasis};
use ql_mef::vak_performance::{
    FactoryVakPerformanceSnapshot, PERFORMANCE_EVENT_CONTRACT, PERFORMANCE_PROJECTION_REQUEST,
    PerformanceObservation, PerformanceObservationMode, PerformanceProjectionRequest,
};
use ql_mef::vak_profile::{
    CPrimeProfile, CompiledProfile, ContentPosition, ContentType, ContextSequence,
    InquiryDirection, PROFILE_CONTRACT, PROFILE_SOURCE, Participation, ThreadForm,
};
use ql_mef::{ContextFrameId, LensId, MusicalBasis};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;

fn read(path: &str) -> Result<Value, String> {
    if std::fs::metadata(path)
        .map_err(|error| error.to_string())?
        .len()
        > 32 * 1024 * 1024
    {
        return Err("input exceeds 32 MiB".into());
    }
    serde_json::from_slice(&std::fs::read(path).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn source(name: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:k8-vak-personal:{name}"),
        revision: "controlled-personal-source-v1".into(),
        standing_ref: "controlled-cross-owner-k8-acceptance".into(),
    }
}

fn layer(name: &str, quaternion: BioQuaternion, observed_at_unix_ms: u64) -> PersonalLayer {
    PersonalLayer {
        source: source(name),
        observed_at_unix_ms,
        quaternion,
    }
}

fn constitution(subject: &str, observed_at_unix_ms: u64) -> PersonalConstitution {
    PersonalConstitution {
        subject_id: subject.into(),
        constitution_ref: format!("constitution:{subject}:k8-vak-personal-v1"),
        source_revisions: vec![source("controlled-personal-constitution")],
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
        ),
        ephemeral: Some(layer(
            "ephemeral",
            BioQuaternion::IDENTITY,
            observed_at_unix_ms,
        )),
        earth_body: EarthBodyConstitution {
            source: source("earth-body"),
            frame_ref: "earth-fixed:k8-cross-owner".into(),
            orientation: BioQuaternion::IDENTITY,
        },
        receivers: (0u8..7)
            .map(|ordinal| ReceiverConstitution {
                ordinal,
                label: format!("source-centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}")),
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

fn ql_profile(factory: &FactoryVakPerformanceSnapshot) -> Result<CompiledProfile, String> {
    if factory.frame != "CF5"
        || factory.thread != "CFP0"
        || factory.sequence != "CS2"
        || factory.direction != "forward"
        || factory.musical_role != "single-voice"
    {
        return Err("Factory K8 receipt changed its controlled QL profile".into());
    }
    let profile = CPrimeProfile {
        participation: Participation::AuthorisedUndertaking,
        content: ContentType::Operations,
        position: ContentPosition::Operation,
        thread: ThreadForm::Single,
        sequence: ContextSequence::ThroughOperation,
        direction: InquiryDirection::Forward,
    };
    let provenance = CallerProvenance::new(
        "k8-cross-owner-acceptance",
        PROFILE_SOURCE,
        "CONTROLLED_ACCEPTANCE",
    )
    .map_err(|error| error.to_string())?;
    Ok(CompiledProfile {
        contract: PROFILE_CONTRACT,
        whole_use: factory.whole_ref.clone(),
        subject_ref: factory.subject_ref.clone(),
        frame: ActiveFrame {
            id: ContextFrameId::Cf5,
            lens: LensId::L0,
            basis: MusicalBasis::Chromatic,
            face: QlFace::Direct,
            positions: PositionBasis::Local,
        },
        pairs: profile.sequence.pairs(),
        walk: profile.sequence.walk(profile.direction),
        profile,
        basis: vec![Basis {
            provenance,
            revision: factory.ql_binding_revision.clone(),
            evidence: vec![factory.ql_binding_ref.clone()],
        }],
    })
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
    let factory_json = read(&args[3])?;
    let factory_source_sha = &args[4];
    let out = Path::new(&args[5]);
    let factory_receipt_sha256 = &args[6];
    if factory_source_sha.len() != 40
        || !factory_source_sha
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("Factory source SHA must be an exact Git commit".into());
    }
    if factory_receipt_sha256.len() != 64
        || !factory_receipt_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("Factory receipt digest must be SHA-256 hex".into());
    }
    let factory: FactoryVakPerformanceSnapshot = serde_json::from_value(factory_json.clone())
        .map_err(|error| {
            format!("Factory owner receipt does not match its published wire: {error}")
        })?;
    let profile = ql_profile(&factory)?;
    let performance_event = profile
        .project_factory_performance(PerformanceProjectionRequest {
            schema: PERFORMANCE_PROJECTION_REQUEST.into(),
            ql_binding_ref: factory.ql_binding_ref.clone(),
            ql_binding_revision: factory.ql_binding_revision.clone(),
            observation: PerformanceObservation {
                observation_ref: format!("observation:k8-live:{factory_source_sha}"),
                mode: PerformanceObservationMode::Live,
                replay_of: None,
            },
            factory_receipt_refs: BTreeSet::from([
                format!("git:EpiLogos/Factory@{factory_source_sha}"),
                format!("sha256:{factory_receipt_sha256}"),
            ]),
            snapshot: factory,
        })
        .map_err(|error| error.to_string())?;
    if performance_event.contract != PERFORMANCE_EVENT_CONTRACT {
        return Err("QL Vāk producer returned another contract".into());
    }
    let subject = performance_event.factory.subject_ref.clone();
    let event_json = serde_json::to_value(&performance_event).map_err(|error| error.to_string())?;

    let mut basis: CoupledInput =
        serde_json::from_value(installed["basis"].clone()).map_err(|error| error.to_string())?;
    let mut field: FieldInput =
        serde_json::from_value(installed["field"].clone()).map_err(|error| error.to_string())?;
    if basis.schema != REQUEST_V2 {
        return Err("cross-owner acceptance requires the accepted dual-bus K8 input".into());
    }
    basis.schema = REQUEST_V3.into();
    basis.m3.subject_ref.clone_from(&subject);
    field.subject_ref.clone_from(&subject);
    basis.source_receipts.push(event_json.clone());

    let composed = basis.compose()?;
    let projected = &composed.derivation["vak_performance_event"];
    if projected["performance_ref"] != performance_event.performance_ref
        || projected["factory"]["run_ref"] != performance_event.factory.run_ref
        || projected["factory"]["subject_ref"] != subject
    {
        return Err("performance provenance changed during K8 composition".into());
    }
    if composed
        .m2_input
        .resonator
        .as_ref()
        .map_or(0, |value| value.modes.len())
        != 16
    {
        return Err("dual musical inputs did not retain one sixteen-mode material owner".into());
    }
    let performed_mode = composed
        .m2_input
        .vimarsha
        .as_ref()
        .ok_or("performed occasion lacks Vimarśā")?
        .musical_mode;
    if performed_mode != performance_event.semantics.musical_mode_index {
        return Err("K8 material voice differs from the QL performance semantics".into());
    }
    let observed_at_unix_ms = composed.m2_input.at_unix_ms;
    let personal = constitution(&subject, observed_at_unix_ms);
    std::fs::create_dir_all(out).map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("input.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({"basis": &basis, "field": &field})).unwrap()
        ),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("ql-performance-event.json"),
        format!("{}\n", serde_json::to_string_pretty(&event_json).unwrap()),
    )
    .map_err(|error| error.to_string())?;

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
        "schema":"ql.k8-vak-personal-acceptance/v2",
        "factory_source_sha":factory_source_sha,
        "factory_receipt_sha256":factory_receipt_sha256,
        "factory_contract":performance_event.factory_contract,
        "factory_performance_ref":performance_event.factory.performance_ref,
        "factory_run_ref":performance_event.factory.run_ref,
        "factory_run_revision":performance_event.factory.run_revision,
        "factory_actor_ref":performance_event.factory.actor_ref,
        "factory_subject_ref":subject,
        "ql_performance_contract":performance_event.contract,
        "ql_binding_ref":performance_event.ql_binding_ref,
        "ql_binding_revision":performance_event.ql_binding_revision,
        "ql_semantics":performance_event.semantics,
        "factory_attempts":performance_event.factory.attempts.len(),
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
        "standing":"Factory owner-executed Vāk performance -> QL-owned performance event -> QL music/material composition -> #134 Nara reception on one installed C++ field owner; centre inputs are controlled source-qualified acceptance values, not inferred physiology or owner-machine lived evidence"
    });
    std::fs::write(
        out.join("acceptance.json"),
        format!("{}\n", serde_json::to_string_pretty(&acceptance).unwrap()),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("factory-performance.json"),
        format!("{}\n", serde_json::to_string_pretty(&factory_json).unwrap()),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("inspection.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&owner.inspect()?).unwrap()
        ),
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
