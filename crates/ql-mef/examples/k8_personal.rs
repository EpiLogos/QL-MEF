//! Installed K8 Nara/runtime acceptance. The existing coupled input and native
//! worker are reused; personal reception neither advances nor duplicates them.
use ql_mef::continuous::FieldInput;
use ql_mef::continuous::coupled::CoupledInput;
use ql_mef::continuous::personal::PersonalCoupledSession;
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyConstitution, EventBasisRefs, LifecycleState,
    PersonalConstitution, PersonalEventInput, PersonalLayer, ReceiverConstitution,
    ReceiverEventInput, SourceRevision, WorldContribution,
};
use serde_json::{Value, json};
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
        source_ref: format!("source:installed-k8:{name}"),
        revision: "controlled-installed-v1".into(),
        standing_ref: "controlled-installed-K8-Nara-acceptance".into(),
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
        constitution_ref: format!("constitution:{subject}:installed-k8-v1"),
        source_revisions: vec![source("personal-constitution")],
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
            frame_ref: "earth-fixed:installed-k8".into(),
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
                    source_ref: format!("installed:m1:{ordinal}"),
                    value: 0.7 + f64::from(ordinal) / 10.0,
                },
                m2: WorldContribution {
                    basis_ref: refs.m2_source_ref.clone(),
                    source_ref: format!("installed:m2:{ordinal}"),
                    value: 1.2 + f64::from(ordinal) / 20.0,
                },
                m3: WorldContribution {
                    basis_ref: refs.m3_source_ref.clone(),
                    source_ref: format!("installed:m3:{ordinal}"),
                    value: 0.4 + f64::from(ordinal) / 30.0,
                },
            })
            .collect(),
    }
}

fn advance_generation(input: &mut CoupledInput) {
    input.m2.stamp.identity.profile_generation += 1;
    let identity = input.m2.stamp.identity.clone();
    input.m3.stamp.identity = identity.clone();
    if let Some(m2_basis) = input.m3.m2_basis.as_mut() {
        m2_basis.identity = identity.clone();
    }
    if let Some(vimarsha) = input.m2.vimarsha.as_mut() {
        vimarsha.stamp.identity = identity.clone();
    }
    if let Some(excitation) = input.m2.m1_excitation.as_mut() {
        excitation.stamp.identity = identity.clone();
    }
    if let Some(resonator) = input.m2.resonator.as_mut() {
        resonator.stamp.identity = identity.clone();
    }
    if let Some(palette) = input
        .m2
        .condition
        .as_mut()
        .and_then(|condition| condition.palette.as_mut())
    {
        palette.stamp.identity = identity;
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: k8_personal WORKER COUPLED_INPUT_JSON OUT_DIR".into());
    }
    let installed = read(&args[2])?;
    let basis: CoupledInput =
        serde_json::from_value(installed["basis"].clone()).map_err(|error| error.to_string())?;
    let field: FieldInput =
        serde_json::from_value(installed["field"].clone()).map_err(|error| error.to_string())?;
    let observed_at_unix_ms = basis.m2.at_unix_ms;
    let constitution = constitution(&basis.m3.subject_ref, observed_at_unix_ms);
    let out = Path::new(&args[3]);
    std::fs::create_dir_all(out).map_err(|error| error.to_string())?;

    let mut owner = PersonalCoupledSession::open(
        Path::new(&args[1]),
        basis.clone(),
        field,
        constitution,
        Duration::from_secs(20),
    )?;
    assert!(!owner.personal_is_current()?);
    let initial_field = owner.last_field().clone();
    let refs = EventBasisRefs::from_basis(owner.current_basis())?;
    let first_input = reception(&refs, observed_at_unix_ms);
    let first = owner.receive_personal(first_input.clone())?;
    assert!(owner.personal_is_current()?);
    assert_eq!(
        owner.last_field(),
        &initial_field,
        "personal reception advanced native field"
    );
    assert_eq!(first.receivers.len(), 7);
    assert_eq!(first.subject_id, refs.subject_ref);

    let replay = owner.receive_personal(first_input)?;
    assert_eq!(replay, first);
    assert_eq!(
        owner.last_field(),
        &initial_field,
        "personal replay changed native field"
    );

    owner.advance_field(1024, false)?;
    assert!(
        owner.personal_is_current()?,
        "sample advance changed world-basis identity"
    );
    assert_ne!(
        owner.last_field()["samples_elapsed"],
        initial_field["samples_elapsed"]
    );

    let before_replace_field = owner.last_field().clone();
    let mut changed = basis;
    advance_generation(&mut changed);
    owner.replace_field(changed)?;
    assert!(
        !owner.personal_is_current()?,
        "old personal reading relabelled after world replacement"
    );
    assert_eq!(
        owner.last_field()["samples_elapsed"],
        before_replace_field["samples_elapsed"],
        "world replacement advanced native time"
    );
    let changed_refs = EventBasisRefs::from_basis(owner.current_basis())?;
    let second = owner.receive_personal(reception(&changed_refs, observed_at_unix_ms))?;
    assert!(owner.personal_is_current()?);
    assert_eq!(second.reception_generation, 2);
    assert_eq!(
        second.event.profile_generation,
        changed_refs.profile_generation
    );

    let inspection = owner.inspect()?;
    let currentness = owner.currentness()?;
    std::fs::write(
        out.join("acceptance.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "schema":"ql.k8-personal-runtime-acceptance/v1",
                "subject_ref":changed_refs.subject_ref,
                "event_ref":changed_refs.event_ref,
                "profile_generation":changed_refs.profile_generation,
                "seven_independent_receivers":second.receivers.len() == 7,
                "same_native_owner_preserved":true,
                "personal_reception_did_not_advance_field":true,
                "native_advance_did_not_rewrite_personal_basis":true,
                "world_replacement_made_old_personal_reading_stale":true,
                "explicit_rereception_restored_currentness":owner.personal_is_current()?,
                "exact_personal_replay":replay == first,
                "currentness":currentness,
                "standing":"installed C++ field worker plus subject-bound #134 PersonalFieldInstance; receiver values are controlled source-qualified inputs, not owner-machine lived/clinical evidence"
            }))
            .unwrap()
        ),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        out.join("inspection.json"),
        format!("{}\n", serde_json::to_string_pretty(&inspection).unwrap()),
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
