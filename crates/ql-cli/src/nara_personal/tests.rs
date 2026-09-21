use super::operations::{Cast, Mutation};
use super::*;
use ql_mef::nara::{
    BioQuaternion, ConsentState, EarthBodyState, EventBasisRefs, LifecycleState, ReceiverState,
    SourceRevision, WorldContribution,
};
use std::sync::atomic::{AtomicU64, Ordering};
static ID: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!(
            "ql-nara-{}-{}-{}",
            std::process::id(),
            now().unwrap(),
            ID.fetch_add(1, Ordering::SeqCst)
        )))
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn source(n: &str) -> SourceRevision {
    SourceRevision {
        source_ref: format!("source:{n}"),
        revision: "r1".into(),
        standing_ref: "controlled-test".into(),
    }
}
fn protected(n: &str) -> ProtectedRef {
    ProtectedRef {
        ref_id: format!("central:source:{n}"),
        revision: "r1".into(),
        owner_ref: "central".into(),
    }
}
fn consent() -> Consent {
    Consent {
        actor_ref: "person:test".into(),
        actor_kind: "human".into(),
        personal_data: true,
    }
}
fn seed() -> Seed {
    let p = PersonalFieldState {
        schema: "ql.nara-personal-field/v1".into(),
        subject_id: "subject:one".into(),
        constitution_ref: "constitution:one".into(),
        reception_generation: 1,
        event: EventBasisRefs {
            event_ref: "event:one".into(),
            subject_ref: "subject:one".into(),
            profile_generation: 1,
            registry_revision: "r1".into(),
            m1_revision: "m1:r1".into(),
            m2_source_ref: "m2:r1".into(),
            m2_contract_ref: "m2:contract".into(),
            m3_source_ref: "m3:r1".into(),
            m3_contract_ref: "m3:contract".into(),
        },
        observed_at_unix_ms: 1,
        consent: ConsentState::Granted,
        lifecycle: LifecycleState::Active,
        q_identity: BioQuaternion::IDENTITY,
        q_transit: BioQuaternion::IDENTITY,
        q_activity: BioQuaternion::IDENTITY,
        q_composed: BioQuaternion::IDENTITY,
        ephemeral_source: None,
        receivers: (0..7)
            .map(|n| ReceiverState {
                ordinal: n,
                label: format!("centre:{n}"),
                source: source(&format!("centre:{n}")),
                input_basis: ["m1", "m2", "m3"].map(|m| WorldContribution {
                    basis_ref: format!("{m}:r1"),
                    source_ref: format!("{m}:{n}"),
                    value: f64::from(n),
                }),
                bioquaternion: BioQuaternion::IDENTITY,
                receiver_orientation: BioQuaternion::IDENTITY,
                composed_orientation: BioQuaternion::IDENTITY,
                orientation_alignment: 1.0,
                drive: f64::from(n),
                resonance: f64::from(n) / 10.0,
                reradiation: f64::from(n) / 20.0,
            })
            .collect(),
        earth_body: EarthBodyState {
            source: source("earthbody"),
            frame_ref: "earth-fixed".into(),
            orientation: BioQuaternion::IDENTITY,
            relation_alignment: 1.0,
        },
        aggregate_resonance: 0.0,
        aggregate_reradiation: 0.0,
        source_revisions: vec![source("personal")],
        standing: "controlled-test".into(),
    };
    Seed {
        nara_ref: "nara:one".into(),
        personal: p,
        embodied: EmbodiedContinuationInput {
            elemental_efwa: ElementalEfwa {
                earth: 0.1,
                fire: 0.2,
                water: 0.3,
                air: 0.4,
            },
            elemental_source: source("elemental"),
            elemental_standing: EvidenceStanding::Reported,
            nadi_refs: vec![],
            sushumna_ref: None,
            temporal_astrology_refs: vec![],
            materia_refs: vec![],
            operation_refs: vec![],
            safety_intensity: None,
            contraindication_refs: vec![],
            response_refs: vec![],
            adjustment_refs: vec![],
        },
    }
}
fn create(root: &PathBuf) -> PersonalRecord {
    serde_json::from_value(
        execute(
            root,
            Request::Create {
                request_id: "create:1".into(),
                consent: consent(),
                seed: Box::new(seed()),
            },
        )
        .unwrap()["record"]
            .clone(),
    )
    .unwrap()
}
fn request(r: &PersonalRecord, id: &str, mutation: Mutation) -> Request {
    Request::Apply {
        request_id: id.into(),
        consent: consent(),
        target: r.target.clone(),
        expected_revision: r.revision,
        mutation: Box::new(mutation),
    }
}
fn apply(root: &PathBuf, r: &PersonalRecord, id: &str, m: Mutation) -> PersonalRecord {
    serde_json::from_value(execute(root, request(r, id, m)).unwrap()["record"].clone()).unwrap()
}
fn cast(system: OracleSystem) -> Mutation {
    Mutation::OracleCast {
        cast: Cast {
            system,
            tradition_ref: "tradition:test".into(),
            deck_or_method_ref: "method:test".into(),
            spread_ref: None,
            query_ref: protected("question"),
            draw_count: 6,
            source_revisions: vec![source("cast")],
            hygiene: OracleHygiene::Clear,
        },
    }
}
fn phase() -> TransformationPhase {
    TransformationPhase {
        phase_ref: "phase:1".into(),
        storey: 11,
        decan: 2,
        stroke: 23,
        operation_refs: vec!["operation:test".into()],
        protocol_ref: "protocol:test".into(),
        container_ref: "container:test".into(),
        source_revisions: vec![source("protocol")],
        opened_at_unix_ms: 1,
        closed_at_unix_ms: None,
        safety: TransformationSafety::Clear,
        feedback_refs: vec![],
    }
}
#[test]
fn explicit_consent_before_storage() {
    let t = Temp::new();
    let mut c = consent();
    c.personal_data = false;
    assert!(
        execute(
            &t.0,
            Request::Create {
                request_id: "r".into(),
                consent: c,
                seed: Box::new(seed())
            }
        )
        .is_err()
    );
    assert!(!t.0.exists());
    let mut c = consent();
    c.actor_kind = "agent".into();
    assert!(execute(&t.0, Request::List { consent: c }).is_err());
    assert!(!t.0.exists());
}
#[test]
fn absent_record_is_empty_not_fixture_or_side_effect() {
    let t = Temp::new();
    assert_eq!(
        execute(&t.0, Request::List { consent: consent() }).unwrap()["records"],
        json!([])
    );
    assert!(!t.0.exists());
    assert_eq!(
        execute(&t.0, Request::Capabilities).unwrap()["public_export"],
        false
    );
    assert!(!t.0.exists());
}
#[test]
fn seven_centres_all_six_branches_without_inferred_identity() {
    let t = Temp::new();
    let r = create(&t.0);
    assert_eq!(r.domain.embodied.centres.len(), 7);
    assert_eq!(r.domain.identity.slots.len(), 6);
    assert_eq!(r.domain.context.branches.len(), 6);
    assert_eq!(r.domain.integration.offices.len(), 6);
    for slot in r.domain.identity.slots {
        assert_eq!(slot.standing, EvidenceStanding::Unavailable);
        assert!(slot.source.is_none());
    }
    assert_eq!(r.domain.embodied.centres[4].amplitude, Some(0.4));
    assert!(r.domain.embodied.earth_body.amplitude.is_none());
    assert_eq!(
        execute(&t.0, Request::Capabilities).unwrap()["centres_are_cymatic_stations"],
        false
    );
}
#[test]
fn native_identity_survives_reopen() {
    let t = Temp::new();
    let r = create(&t.0);
    let again: PersonalRecord = serde_json::from_value(
        execute(
            &t.0,
            Request::Read {
                target: r.target.clone(),
                consent: consent(),
            },
        )
        .unwrap()["record"]
            .clone(),
    )
    .unwrap();
    assert_eq!(r.target.record_ref, again.target.record_ref);
    assert_eq!(again.revision, 1);
    assert_eq!(
        execute(
            &t.0,
            Request::Create {
                request_id: "create:1".into(),
                consent: consent(),
                seed: Box::new(seed())
            }
        )
        .unwrap()["duplicate"],
        true
    );
}
#[test]
fn stale_writes_and_wrong_nara_preserve_bytes() {
    let t = Temp::new();
    let r = create(&t.0);
    let changed = apply(
        &t.0,
        &r,
        "feedback",
        Mutation::CentreFeedback {
            ordinal: 3,
            feedback_ref: "feedback:3".into(),
        },
    );
    let path = t.0.join(format!(
        "{}.json",
        r.target.record_ref.rsplit(':').next().unwrap()
    ));
    let before = std::fs::read(&path).unwrap();
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "stale",
                Mutation::JournalLink {
                    source: protected("journal")
                }
            )
        )
        .unwrap_err()
        .contains("revision conflict")
    );
    let mut target = changed.target.clone();
    target.nara_ref = "nara:other".into();
    assert!(
        execute(
            &t.0,
            Request::Read {
                target,
                consent: consent()
            }
        )
        .is_err()
    );
    assert_eq!(std::fs::read(path).unwrap(), before);
}
#[test]
fn every_identity_slot_has_independent_replacement() {
    let t = Temp::new();
    let mut r = create(&t.0);
    for (i, mut slot) in r.domain.identity.slots.clone().into_iter().enumerate() {
        slot.source = Some(source(&format!("identity:{i}")));
        slot.protected_value_ref = Some(protected(&format!("identity:{i}")));
        slot.absence_reason = None;
        slot.standing = EvidenceStanding::Source;
        r = apply(
            &t.0,
            &r,
            &format!("identity:{i}"),
            Mutation::IdentityReplace {
                slot,
                identity_revision: format!("r{i}"),
            },
        );
    }
    assert!(r.domain.identity.slots.iter().all(|s| s.source.is_some()));
    assert_eq!(r.revision, 7);
}
#[test]
fn invalid_identity_never_partially_persists() {
    let t = Temp::new();
    let r = create(&t.0);
    let mut slot = r.domain.identity.slots[0].clone();
    slot.coordinate_ref = "M4.0.5".into();
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "bad",
                Mutation::IdentityReplace {
                    slot,
                    identity_revision: "r2".into()
                }
            )
        )
        .is_err()
    );
    assert_eq!(store::read(&t.0, &r.target.record_ref).unwrap().revision, 1);
}
#[test]
fn newer_reception_preserves_exact_inputs_refuses_other_occasion() {
    let t = Temp::new();
    let r = create(&t.0);
    let mut s = seed();
    s.personal.reception_generation = 2;
    s.personal.receivers[2].resonance = 0.9;
    let next = apply(
        &t.0,
        &r,
        "reception",
        Mutation::EmbodiedReceive {
            personal: Box::new(s.personal.clone()),
            input: s.embodied.clone(),
        },
    );
    assert_eq!(next.domain.embodied.centres[2].amplitude, Some(0.9));
    assert_eq!(
        next.domain.embodied.centres[2].world_inputs.m1,
        s.personal.receivers[2].input_basis[0]
    );
    s.personal.event.event_ref = "another-event".into();
    assert!(
        execute(
            &t.0,
            request(
                &next,
                "wrong",
                Mutation::EmbodiedReceive {
                    personal: Box::new(s.personal),
                    input: s.embodied
                }
            )
        )
        .is_err()
    );
}
#[test]
fn os_entropy_cast_persists_once_replay_never_recasts() {
    let t = Temp::new();
    let r = create(&t.0);
    let req = request(&r, "cast", cast(OracleSystem::IChingCoins));
    let v = execute(&t.0, req.clone()).unwrap();
    let next: PersonalRecord = serde_json::from_value(v["record"].clone()).unwrap();
    assert_eq!(next.domain.oracle.records[0].original.tokens.len(), 6);
    let mut forbidden =
        |_: &mut [u8]| -> Result<(), String> { panic!("replayed cast asked for fresh entropy") };
    let replay = execute_with(&t.0, req, &mut forbidden, &|| Ok(2)).unwrap();
    assert_eq!(replay["duplicate"], true);
    assert_eq!(
        replay["record"]["domain"]["oracle"]["records"],
        v["record"]["domain"]["oracle"]["records"]
    );
}
#[test]
fn all_native_oracles_keep_their_system() {
    let t = Temp::new();
    let mut r = create(&t.0);
    for (i, system) in [
        OracleSystem::TarotRws,
        OracleSystem::TarotThoth,
        OracleSystem::TarotMarseille,
        OracleSystem::TarotQl,
        OracleSystem::IChingCoins,
        OracleSystem::IChingYarrow,
    ]
    .into_iter()
    .enumerate()
    {
        r = apply(&t.0, &r, &format!("cast:{i}"), cast(system));
        assert_eq!(
            r.domain.oracle.records.last().unwrap().original.system,
            system
        );
    }
    assert_eq!(r.domain.oracle.records.len(), 6);
    assert!(serde_json::to_string(&r).unwrap().contains("getrandom/os"));
}
#[test]
fn entropy_failure_never_falls_back_to_zero() {
    let t = Temp::new();
    let r = create(&t.0);
    let mut unavailable = |_: &mut [u8]| Err("entropy-unavailable".into());
    assert!(
        execute_with(
            &t.0,
            request(&r, "cast", cast(OracleSystem::TarotQl)),
            &mut unavailable,
            &|| Ok(2)
        )
        .is_err()
    );
    assert!(
        store::read(&t.0, &r.target.record_ref)
            .unwrap()
            .domain
            .oracle
            .records
            .is_empty()
    );
}
#[test]
fn reinterpretation_never_changes_original_or_claims_source() {
    let t = Temp::new();
    let r = create(&t.0);
    let r = apply(&t.0, &r, "cast", cast(OracleSystem::IChingYarrow));
    let original = r.domain.oracle.records[0].original.clone();
    let i = OracleInterpretation {
        interpretation_ref: "interpretation:1".into(),
        packet_ref: original.packet_ref.clone(),
        interpretation_revision: "r1".into(),
        model_ref: Some("agent:epii".into()),
        output_ref: protected("interpretation"),
        source_refs: vec!["source:context".into()],
        evidence_refs: vec![],
        standing: EvidenceStanding::Derived,
        interpreted_at_unix_ms: now().unwrap(),
    };
    let r = apply(
        &t.0,
        &r,
        "interpret",
        Mutation::OracleInterpret {
            packet_ref: original.packet_ref.clone(),
            interpretation: i.clone(),
        },
    );
    assert_eq!(r.domain.oracle.records[0].original, original);
    let mut forged = i;
    forged.standing = EvidenceStanding::Observed;
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "forged",
                Mutation::OracleInterpret {
                    packet_ref: original.packet_ref,
                    interpretation: forged
                }
            )
        )
        .is_err()
    );
}
#[test]
fn practice_explicit_hold_resume_close_and_next_native_coordinate() {
    let t = Temp::new();
    let r = create(&t.0);
    let r = apply(
        &t.0,
        &r,
        "start",
        Mutation::PracticeStart { phase: phase() },
    );
    let r = apply(
        &t.0,
        &r,
        "hold",
        Mutation::PracticeHold {
            phase_ref: "phase:1".into(),
            feedback_ref: "feedback:user-stop".into(),
        },
    );
    assert_eq!(
        r.domain.transformation.phase_history.phases[0].safety,
        TransformationSafety::ConsentRequired
    );
    let r = apply(
        &t.0,
        &r,
        "resume",
        Mutation::PracticeResume {
            phase_ref: "phase:1".into(),
            safety_review_ref: "review:user-resume".into(),
        },
    );
    assert!(
        execute(
            &t.0,
            request(&r, "early", Mutation::PracticeStart { phase: phase() })
        )
        .is_err()
    );
    let r = apply(
        &t.0,
        &r,
        "close",
        Mutation::PracticeClose {
            phase_ref: "phase:1".into(),
            feedback_refs: vec![],
        },
    );
    let mut next = phase();
    next.phase_ref = "phase:2".into();
    next.storey = 0;
    next.decan = 0;
    next.stroke = 0;
    next.opened_at_unix_ms = now().unwrap();
    let r = apply(&t.0, &r, "next", Mutation::PracticeStart { phase: next });
    assert_eq!(r.domain.transformation.phase_history.phases.len(), 2);
}
#[test]
fn six_context_branches_accept_attributable_interpretation() {
    let t = Temp::new();
    let mut r = create(&t.0);
    for s in r.domain.context.branches.clone() {
        let b = s.branch;
        let reading = ContextReading {
            reading_ref: format!("reading:{}", b.coordinate()),
            coordinate_ref: b.coordinate().into(),
            source: source("context"),
            protected_content_ref: protected(b.coordinate()),
            model_ref: None,
            confidence: None,
            evidence_refs: vec![],
            standing: EvidenceStanding::Reported,
        };
        r = apply(
            &t.0,
            &r,
            &format!("context:{}", b.coordinate()),
            Mutation::ContextRecord { branch: b, reading },
        );
    }
    assert!(
        r.domain
            .context
            .branches
            .iter()
            .all(|b| b.readings.len() == 1 && b.absence_reason.is_none())
    );
}
#[test]
fn integration_and_journal_keep_owner_refs_only() {
    let t = Temp::new();
    let mut r = create(&t.0);
    for s in r.domain.integration.offices.clone() {
        let office = s.office;
        let returned = IntegrationReturn {
            return_ref: format!("return:{}", office.coordinate()),
            office,
            input_refs: vec![],
            source_refs: vec!["source:basis".into()],
            method_ref: None,
            output_ref: protected(office.coordinate()),
            evidence_refs: vec![],
            human_response_ref: None,
            retention_policy_ref: "policy:private".into(),
            standing: EvidenceStanding::Reported,
        };
        r = apply(
            &t.0,
            &r,
            &format!("return:{}", office.coordinate()),
            Mutation::IntegrationReturn { returned },
        );
    }
    r = apply(
        &t.0,
        &r,
        "journal",
        Mutation::JournalLink {
            source: protected("journal"),
        },
    );
    assert_eq!(r.journal_refs[0].owner_ref, "central");
    assert!(r.receipts.iter().all(|r| !r.source_mutated));
}
#[test]
fn request_id_cannot_change_effect() {
    let t = Temp::new();
    let r = create(&t.0);
    apply(
        &t.0,
        &r,
        "same",
        Mutation::CentreFeedback {
            ordinal: 0,
            feedback_ref: "one".into(),
        },
    );
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "same",
                Mutation::CentreFeedback {
                    ordinal: 1,
                    feedback_ref: "two".into()
                }
            )
        )
        .unwrap_err()
        .contains("different input")
    );
}
#[test]
fn concurrent_writer_refuses() {
    let t = Temp::new();
    let r = create(&t.0);
    let _lock = store::lock(&t.0, &r.target.record_ref).unwrap();
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "blocked",
                Mutation::JournalLink {
                    source: protected("journal")
                }
            )
        )
        .unwrap_err()
        .contains("busy")
    );
}
#[cfg(unix)]
#[test]
fn private_modes_refuse_public_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let t = Temp::new();
    let r = create(&t.0);
    assert_eq!(
        std::fs::metadata(&t.0).unwrap().permissions().mode() & 0o777,
        0o700
    );
    let path = t.0.join(format!(
        "{}.json",
        r.target.record_ref.rsplit(':').next().unwrap()
    ));
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(store::read(&t.0, &r.target.record_ref).is_err());
}
#[cfg(unix)]
#[test]
fn symlink_state_never_read_or_overwritten() {
    let t = Temp::new();
    let r = create(&t.0);
    let path = t.0.join(format!(
        "{}.json",
        r.target.record_ref.rsplit(':').next().unwrap()
    ));
    let other = t.0.join("preserve");
    std::fs::rename(&path, &other).unwrap();
    std::os::unix::fs::symlink(&other, &path).unwrap();
    assert!(store::read(&t.0, &r.target.record_ref).is_err());
    assert!(
        execute(
            &t.0,
            request(
                &r,
                "bad",
                Mutation::JournalLink {
                    source: protected("journal")
                }
            )
        )
        .is_err()
    );
    assert!(other.exists());
}
#[test]
fn traversal_and_unknown_fields_refuse_without_creation() {
    let t = Temp::new();
    let target = Target {
        record_ref: "../../evil".into(),
        nara_ref: "one".into(),
        subject_ref: "one".into(),
    };
    assert!(
        execute(
            &t.0,
            Request::Read {
                target,
                consent: consent()
            }
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<Request>(
            json!({"operation":"capabilities","raw_transcript":"private"})
        )
        .is_err()
    );
    assert!(!t.0.exists());
}
