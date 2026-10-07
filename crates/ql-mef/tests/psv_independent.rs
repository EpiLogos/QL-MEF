//! PS-V (QL-MEF #300) — independent discriminating verification of the
//! Ta-Onta procedural stage (QL-MEF #296).
//!
//! Law of this file: the stage was NOT implemented here. Every expectation is
//! derived from the owner's contracts —
//! `docs/integrations/epi-logos/TA-ONTA-PROCEDURAL-EXPRESSION-CONTRACTS.md`
//! (labels P1–P6) and the dispatch acceptance cases (A02, A04–A08, A10) —
//! not from the implementation. The landed code was read only to construct
//! valid inputs (type spellings, host openings, envelope fields). A behaviour
//! that contradicts a quoted clause is a finding against the stage, not
//! something this file works around.
//!
//! Tiers:
//!
//! - in-process cases run everywhere: the pure stage surface over the default
//!   event fixture (`fixtures/kernel/scene-default-event-v2.json`), with the
//!   PS-E manifestation fixtures qualifying the subject;
//! - host cases run against the real installed worker through the exact field
//!   host the application drives:
//!   `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --locked
//!   -p ql-mef --test psv_independent -- --ignored`.
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use ql_mef::MFace;
use ql_mef::continuous::LiftInput;
use ql_mef::continuous::coupled::CoupledInput;
use ql_mef::continuous::host::{FieldHost, HostOperation, HostRequest};
use ql_mef::continuous::scene_field::{BINDING_REQUEST, BindingRequest, SceneGeometry};
use ql_mef::continuous::stage::{
    STAGE_PROCEDURE, StageBindings, StageChange, StageOwnership, StagePassage, StageProcedure,
    StageTrigger, evaluate,
};
use ql_mef::coordinate_expression::{ExpressiveRole, SubjectKind, resolve_subject_manifestation};
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::m3_state::M3Operation;
use serde_json::{Value, json};

// ---------------------------------------------------------------------------
// shared input construction (in-process tier)
// ---------------------------------------------------------------------------

/// The default coupled event: the stage's own standing basis. The fixture's
/// subject and event identity are read from the fixture, never hardcoded
/// here, so the tests assert agreements rather than copied constants.
fn event() -> CoupledInput {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/scene-default-event-v2.json"
    ))
    .unwrap()
}

fn procedure(
    procedure_ref: &str,
    revision: u64,
    subject_ref: &str,
    changes: Vec<StageChange>,
) -> StageProcedure {
    let selector: Vec<String> = changes
        .iter()
        .filter_map(|change| change.slot().ok().flatten().map(str::to_owned))
        .collect();
    StageProcedure {
        schema: STAGE_PROCEDURE.into(),
        procedure_ref: procedure_ref.into(),
        revision,
        subject_ref: subject_ref.into(),
        trigger: StageTrigger::Invocation,
        selector,
        changes,
        passage: None,
    }
}

/// My own canonical view of the owned constituents: slot → the contribution's
/// full record. This is the digest whose before/after equality carries the
/// no-partial-mutation and only-own-basis claims.
fn ownership_digest(ownership: &StageOwnership) -> Value {
    let mut slots = BTreeMap::new();
    for contribution in ownership.contributions() {
        slots.insert(
            contribution.slot.to_owned(),
            json!({
                "key": contribution.key,
                "procedure_ref": contribution.procedure_ref,
                "revision": contribution.revision,
                "warrant": contribution.warrant,
            }),
        );
    }
    json!(slots)
}

// ---------------------------------------------------------------------------
// A02 — address and subject identity across reorder, re-entry and rename
// ---------------------------------------------------------------------------

#[test]
fn psv_a02_same_subject_traces_across_reorder_reentry_and_rename() {
    // A02: "trace one subject's several occurrences; reorder/rename/move them
    // and reopen the same bindings". P1 §0.1: "A reference remains stable
    // under list reordering ... Explicit cloning creates a new occurrence
    // with the same subject binding and a new origin relation."
    let live = event();
    let subject = live.m3.subject_ref.clone();

    // Input qualification from the PS-E fixtures: this subject resolves
    // through the semantic owner as formation, force and sequence at the Moon
    // locus — one subject, three role occurrences (P1 §0.2).
    let registry = native_current_m_registry();
    let manifestation = resolve_subject_manifestation(
        registry,
        &subject,
        SubjectKind::Native,
        "#2-5-4",
        MFace::Bimba,
        &[
            ExpressiveRole::Formation,
            ExpressiveRole::Force,
            ExpressiveRole::Sequence,
        ],
        &[],
        &[],
        &[],
        None,
    )
    .unwrap();
    assert_eq!(manifestation.subject_ref, subject);
    assert_eq!(
        manifestation.occurrences.len(),
        3,
        "one subject, three simultaneous role occurrences"
    );

    let changes = vec![
        StageChange::Damping { per_second: 0.5 },
        StageChange::Form {
            operations: vec![M3Operation::SetPose { pose: 2 }],
        },
        StageChange::Clock {
            slot: "clock.lensing".into(),
            phase: LiftInput {
                turns: "0".into(),
                half_degrees: 90,
            },
        },
    ];
    let mut straight = procedure("ta-onta:stage:psv-a02", 1, &subject, changes);
    straight.selector = vec![
        "material.damping".into(),
        "form".into(),
        "clock.lensing".into(),
    ];

    // REORDER: the selector is an addressable target set (P1 §0.3), not an
    // ordered program; reordering it cannot change what the procedure
    // resolves, commands or claims.
    let mut reordered = straight.clone();
    reordered.selector = vec![
        "clock.lensing".into(),
        "form".into(),
        "material.damping".into(),
    ];
    let mut one = StageOwnership::default();
    let mut two = StageOwnership::default();
    let straight_plan = evaluate(&straight, &live, &mut one, 1, 11, 22).unwrap();
    let reordered_plan = evaluate(&reordered, &live, &mut two, 1, 11, 22).unwrap();
    assert_eq!(
        serde_json::to_value(&straight_plan).unwrap(),
        serde_json::to_value(&reordered_plan).unwrap(),
        "selector reordering changed the resolved target set"
    );
    assert_eq!(ownership_digest(&one), ownership_digest(&two));

    // RE-ENTRY: removing the contribution leaves the native subject intact
    // (P1 §0.1); the same identity re-enters and reclaims exactly its slots.
    let retired = one.retire("ta-onta:stage:psv-a02");
    assert_eq!(retired.len(), 3, "every claimed slot is released on retire");
    assert_eq!(ownership_digest(&one), json!({}));
    let reentered = evaluate(&straight, &live, &mut one, 1, 11, 22).unwrap();
    assert_eq!(
        serde_json::to_value(&reentered).unwrap(),
        serde_json::to_value(&straight_plan).unwrap(),
        "re-entry after release must reconstruct the same claims"
    );

    // RENAME (explicit clone): a new procedure_ref with the SAME subject
    // binding is a new origin relation. It cannot silently take over the
    // standing contribution while the original holds it (P3 §2.4: two
    // procedures writing the same property need a declared decision), and
    // after the original releases it claims under its own identity.
    let mut clone = straight.clone();
    clone.procedure_ref = "ta-onta:stage:psv-a02-clone".into();
    let error = evaluate(&clone, &live, &mut one, 1, 11, 22).unwrap_err();
    assert!(
        error.contains("owned by ta-onta:stage:psv-a02@"),
        "the clone must be refused while the original holds the slot: {error}"
    );
    one.retire("ta-onta:stage:psv-a02");
    let cloned_plan = evaluate(&clone, &live, &mut one, 1, 11, 22).unwrap();
    assert!(
        !cloned_plan.contributions.is_empty(),
        "the clone claims after the original releases"
    );
    for contribution in &cloned_plan.contributions {
        assert!(
            contribution.key.starts_with("ta-onta:stage:psv-a02-clone@"),
            "the clone must claim under its own identity, not the original's: {}",
            contribution.key
        );
    }
}

// ---------------------------------------------------------------------------
// A07 — regeneration revises only its own contributions
// ---------------------------------------------------------------------------

#[test]
fn psv_a07_regeneration_revises_only_its_own_contributions() {
    // A07: "Re-run and revise a procedure, preserve stable contributions and
    // authored edits". P3 §2.2: "Procedural updates replace their own
    // generated basis." P3 §2.4: "Removing one procedural contribution leaves
    // unrelated local work and native subjects intact."
    let live = event();
    let subject = live.m3.subject_ref.clone();
    let alpha = procedure(
        "ta-onta:stage:psv-alpha",
        1,
        &subject,
        vec![
            StageChange::Damping { per_second: 0.5 },
            StageChange::Form {
                operations: vec![M3Operation::SetPose { pose: 2 }],
            },
        ],
    );
    let beta = procedure(
        "ta-onta:stage:psv-beta",
        3,
        &subject,
        vec![
            StageChange::Clock {
                slot: "clock.inscription".into(),
                phase: LiftInput {
                    turns: "1".into(),
                    half_degrees: 0,
                },
            },
            StageChange::Clock {
                slot: "clock.lensing".into(),
                phase: LiftInput {
                    turns: "0".into(),
                    half_degrees: 36,
                },
            },
        ],
    );
    let mut ownership = StageOwnership::default();
    evaluate(&alpha, &live, &mut ownership, 1, 0, 0).unwrap();
    evaluate(&beta, &live, &mut ownership, 1, 0, 0).unwrap();
    let before = ownership_digest(&ownership);
    assert_eq!(
        before.as_object().unwrap().len(),
        4,
        "two procedures, four owned slots"
    );

    // Regenerate alpha at a new revision with a changed value: ONLY alpha's
    // entries may move. The rule version is retained as provenance and update
    // basis (P3 §2.4), so the revised keys carry the new revision.
    let mut revised = StageProcedure {
        revision: 2,
        ..alpha.clone()
    };
    let StageChange::Damping { per_second } = &mut revised.changes[0] else {
        panic!("the first alpha change is the damping change");
    };
    *per_second = 0.125;
    let plan = evaluate(&revised, &live, &mut ownership, 1, 0, 0).unwrap();
    assert_eq!(
        plan.damping,
        Some(0.125),
        "the basis was replaced, not kept"
    );
    let after = ownership_digest(&ownership);
    assert_eq!(
        after.as_object().unwrap().len(),
        4,
        "regeneration may not add or drop constituents"
    );
    for slot in ["clock.inscription", "clock.lensing"] {
        assert_eq!(
            after[slot], before[slot],
            "unrelated constituent {slot} moved under alpha's regeneration"
        );
    }
    assert_ne!(after["material.damping"], before["material.damping"]);
    assert_ne!(after["form"], before["form"]);
    assert!(
        after["material.damping"]["key"]
            .as_str()
            .unwrap()
            .contains("@2/"),
        "the revised basis carries its rule version as provenance"
    );

    // Retiring alpha releases only alpha's key family; beta survives
    // byte-identical and the native subject is untouched.
    let retired = ownership.retire("ta-onta:stage:psv-alpha");
    assert_eq!(retired.len(), 2);
    assert!(
        retired
            .iter()
            .all(|key| key.starts_with("ta-onta:stage:psv-alpha@")),
        "retire must release only its own family: {retired:?}"
    );
    let remaining = ownership_digest(&ownership);
    assert_eq!(remaining.as_object().unwrap().len(), 2);
    assert_eq!(remaining["clock.inscription"], before["clock.inscription"]);
    assert_eq!(remaining["clock.lensing"], before["clock.lensing"]);
}

// ---------------------------------------------------------------------------
// A05 — refused batches leave no partial state (in-process half)
// ---------------------------------------------------------------------------

#[test]
fn psv_a05_refused_evaluation_leaves_no_partial_ownership() {
    // A05: "Reject a stale/invalid batch without partial document mutation."
    // P2 §1.4: "validate the complete batch before mutation."
    let live = event();
    let subject = live.m3.subject_ref.clone();
    let mut ownership = StageOwnership::default();

    // One procedure claiming the same slot twice is an invalid batch; the
    // first change's claim must not survive the refusal.
    let invalid = procedure(
        "ta-onta:stage:psv-partial",
        1,
        &subject,
        vec![
            StageChange::Damping { per_second: 0.5 },
            StageChange::Damping { per_second: 0.9 },
        ],
    );
    let error = evaluate(&invalid, &live, &mut ownership, 1, 0, 0).unwrap_err();
    assert!(error.contains("two changes address"), "{error}");
    assert_eq!(
        ownership_digest(&ownership),
        json!({}),
        "a refused batch mutated owned state"
    );

    // A cross-procedure conflict likewise leaves the refuser with nothing,
    // and arrival order must not arbitrate the conflict (P3 §2.4).
    let holder = procedure(
        "ta-onta:stage:psv-holder",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    evaluate(&holder, &live, &mut ownership, 1, 0, 0).unwrap();
    let arrival = procedure(
        "ta-onta:stage:psv-arrival",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.9 }],
    );
    let error = evaluate(&arrival, &live, &mut ownership, 1, 0, 0).unwrap_err();
    assert!(error.contains("owned by"), "{error}");
    let digest = ownership_digest(&ownership);
    assert_eq!(digest.as_object().unwrap().len(), 1);
    assert!(
        digest["material.damping"]["key"]
            .as_str()
            .unwrap()
            .starts_with("ta-onta:stage:psv-holder@"),
        "arrival order arbitrated the conflict"
    );
}

// ---------------------------------------------------------------------------
// A10 — bindings fire canonically and stop at their budget (in-process half)
// ---------------------------------------------------------------------------

#[test]
fn psv_a10_bindings_fire_in_canonical_order_and_stop_at_their_budget() {
    // A10. P3 §2.4: "Arrival order is not a semantic arbitration rule."
    // P3 §2.5: "Configure maximum rule evaluations ... stop or coalesce the
    // affected procedure with an inspectable outcome while the rest of the
    // stage continues."
    let bind = |bindings: &mut StageBindings, name: &str, budget: u32| {
        let mut bound = procedure(
            &format!("ta-onta:stage:psv-{name}"),
            1,
            "ql:k2/default-subject",
            vec![StageChange::Strike {
                mode_ref: "scene:planet/#2-5-4".into(),
                amplitude: [0.3, 0.0],
            }],
        );
        bound.selector = Vec::new();
        bound.trigger = StageTrigger::Determinant {
            operation: "m1-advance".into(),
        };
        bindings.bind(bound, Some(budget)).unwrap();
    };
    let admissions_of = |bindings: &StageBindings| bindings.admissions("m1-advance");

    // Canonical order: two different arrival orders must admit identically,
    // and that order is the sorted reference order, never arrival order.
    let mut arrival = StageBindings::default();
    bind(&mut arrival, "c", 2);
    bind(&mut arrival, "a", 2);
    bind(&mut arrival, "b", 2);
    let mut sorted = StageBindings::default();
    bind(&mut sorted, "a", 2);
    bind(&mut sorted, "b", 2);
    bind(&mut sorted, "c", 2);
    assert_eq!(
        admissions_of(&arrival),
        admissions_of(&sorted),
        "admissions depend on arrival order"
    );
    assert_eq!(
        admissions_of(&arrival),
        vec![
            "ta-onta:stage:psv-a".to_owned(),
            "ta-onta:stage:psv-b".to_owned(),
            "ta-onta:stage:psv-c".to_owned(),
        ],
        "admissions must be reference-canonical, not arrival-ordered"
    );

    // Budgets are per binding: each firing consumes exactly one evaluation;
    // exhaustion is disclosed and stops only the affected procedure.
    let mut bindings = arrival;
    bindings.record_fired("ta-onta:stage:psv-a");
    assert_eq!(
        bindings.get("ta-onta:stage:psv-a").unwrap().standing,
        "active"
    );
    bindings.record_fired("ta-onta:stage:psv-a");
    assert_eq!(
        bindings.get("ta-onta:stage:psv-a").unwrap().standing,
        "exhausted"
    );
    assert!(
        !admissions_of(&bindings).contains(&"ta-onta:stage:psv-a".to_owned()),
        "an exhausted binding must stop firing"
    );
    assert_eq!(admissions_of(&bindings).len(), 2, "the rest continues");
    bindings.record_fired("ta-onta:stage:psv-b");
    assert_eq!(
        bindings.get("ta-onta:stage:psv-b").unwrap().evaluations,
        1,
        "one firing consumes exactly one evaluation"
    );

    // Unbinding stops the firing and returns the final standing.
    let removed = bindings.unbind("ta-onta:stage:psv-b").unwrap();
    assert_eq!(removed.standing, "active");
    assert!(bindings.get("ta-onta:stage:psv-b").is_none());
    assert_eq!(
        admissions_of(&bindings),
        vec!["ta-onta:stage:psv-c".to_owned()]
    );
}

// ---------------------------------------------------------------------------
// negative mutations — one violated clause per variant, refused by name
// ---------------------------------------------------------------------------

#[test]
fn psv_negative_mutations_are_refused_by_name() {
    // P2 §1.3: "Preparation validates all targets, capabilities, units,
    // ranges, source versions, cardinality limits and authority
    // requirements." Each variant below violates ONE clause.
    let live = event();
    let subject = live.m3.subject_ref.clone();
    let mut ownership = StageOwnership::default();
    let mut refused = |procedure: &StageProcedure| {
        evaluate(procedure, &live, &mut ownership, 1, 0, 0).unwrap_err()
    };

    // (i) A change addressing a slot outside the procedure's selector
    // (P1 §0.3: the selector names the addressed set).
    let mut unselected = procedure(
        "ta-onta:stage:psv-m1",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    unselected.selector = vec!["form".into()];
    assert!(
        refused(&unselected).contains("outside the procedure's selector"),
        "an unselected slot must be refused by name"
    );

    // (ii) A selector naming a voice slot: voice retuning is not an admitted
    // stage change, and must be refused rather than silently ignored.
    let mut voice = procedure(
        "ta-onta:stage:psv-m2",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    voice.selector = vec!["voice:#2-5-4".into()];
    assert!(
        refused(&voice).contains("voice retuning is not an admitted stage change"),
        "a voice selector must be refused by name"
    );

    // (iii) A selector naming no admitted slot at all.
    let mut unknown = procedure(
        "ta-onta:stage:psv-m3",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    unknown.selector = vec!["colour.wheel".into()];
    assert!(
        refused(&unknown).contains("unknown stage slot"),
        "an unknown slot must be refused by name"
    );

    // (iv) A passage outside the declared bound (P3 §2.5: bounded feedback;
    // the passage count is bounded, here 2..=8).
    let above = StageChange::Form {
        operations: vec![M3Operation::SetPose { pose: 1 }],
    };
    let mut wide = procedure("ta-onta:stage:psv-m4", 1, &subject, vec![above.clone()]);
    wide.passage = Some(StagePassage { scenes: 9 });
    assert!(
        refused(&wide).contains("2..=8 scenes"),
        "a passage above the bound must be refused by name"
    );
    let mut narrow = procedure("ta-onta:stage:psv-m4b", 1, &subject, vec![above]);
    narrow.passage = Some(StagePassage { scenes: 1 });
    assert!(
        refused(&narrow).contains("2..=8 scenes"),
        "a one-scene passage is not a passage and must be refused by name"
    );

    // (v) A strike above the declared material policy: amplitude is modal
    // metres, finite, within |a| <= 1 (P3 §2.1: declared type, units,
    // domain).
    let loud = procedure(
        "ta-onta:stage:psv-m5",
        1,
        &subject,
        vec![StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [1.5, 0.0],
        }],
    );
    assert!(
        refused(&loud).contains("modal metres"),
        "an over-policy strike must be refused by name"
    );
    let infinite = procedure(
        "ta-onta:stage:psv-m5b",
        1,
        &subject,
        vec![StageChange::Strike {
            mode_ref: "scene:planet/#2-5-4".into(),
            amplitude: [f64::NAN, 0.0],
        }],
    );
    assert!(
        refused(&infinite).contains("modal metres"),
        "a non-finite strike must be refused by name"
    );
    let anonymous = procedure(
        "ta-onta:stage:psv-m5c",
        1,
        &subject,
        vec![StageChange::Strike {
            mode_ref: String::new(),
            amplitude: [0.3, 0.0],
        }],
    );
    assert!(
        refused(&anonymous).contains("names a current scene voice"),
        "a strike without a mode reference must be refused by name"
    );

    // (vi) Two changes addressing one slot in one procedure (P3 §2.4: each
    // output records its exact owned constituents; one procedure cannot
    // double-claim a property).
    let duplicate = procedure(
        "ta-onta:stage:psv-m6",
        1,
        &subject,
        vec![
            StageChange::Damping { per_second: 0.5 },
            StageChange::Damping { per_second: 0.9 },
        ],
    );
    assert!(
        refused(&duplicate).contains("two changes address"),
        "a double claim must be refused by name"
    );

    // (vii) A noncanonical clock phase (P1 §0.3: deep coordinates keep
    // canonical forms through validated parsers).
    let wide_phase = procedure(
        "ta-onta:stage:psv-m7",
        1,
        &subject,
        vec![StageChange::Clock {
            slot: "clock.inscription".into(),
            phase: LiftInput {
                turns: "0".into(),
                half_degrees: 720,
            },
        }],
    );
    assert!(
        refused(&wide_phase).contains("half_degrees"),
        "a phase at or beyond one turn must be refused by name"
    );
    let unparsed = procedure(
        "ta-onta:stage:psv-m7b",
        1,
        &subject,
        vec![StageChange::Clock {
            slot: "clock.lensing".into(),
            phase: LiftInput {
                turns: "01".into(),
                half_degrees: 0,
            },
        }],
    );
    assert!(
        refused(&unparsed).contains("canonical"),
        "a noncanonical turns form must be refused by name"
    );

    // (viii) A foreign subject (P1 §0.1: subject resolution against the live
    // event).
    let foreign = procedure(
        "ta-onta:stage:psv-m8",
        1,
        "person:psv-stranger",
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    assert!(
        refused(&foreign).contains("but the live event belongs to"),
        "a foreign subject must be refused by name"
    );

    // (ix) Cardinality and identity limits (P2 §1.3).
    let mut wrong_schema = procedure(
        "ta-onta:stage:psv-m9",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    wrong_schema.schema = "ql.stage-procedure/v0".into();
    assert!(
        refused(&wrong_schema).contains("unsupported stage procedure contract"),
        "an unsupported contract version must be refused by name"
    );
    let unrevised = procedure(
        "ta-onta:stage:psv-m9b",
        0,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    assert!(
        refused(&unrevised).contains("revision must be at least 1"),
        "revision 0 must be refused by name"
    );
    let empty = procedure("ta-onta:stage:psv-m9c", 1, &subject, vec![]);
    assert!(
        refused(&empty).contains("1..16 changes"),
        "a changeless procedure must be refused by name"
    );
    let no_operands = procedure(
        "ta-onta:stage:psv-m9d",
        1,
        &subject,
        vec![StageChange::Form { operations: vec![] }],
    );
    assert!(
        refused(&no_operands).contains("1..64 operations"),
        "an empty form change must be refused by name"
    );

    // (x) Damping outside the declared unit policy (P3 §2.1 declared domain).
    let negative = procedure(
        "ta-onta:stage:psv-m10",
        1,
        &subject,
        vec![StageChange::Damping { per_second: -1.0 }],
    );
    assert!(
        refused(&negative).contains("finite and in 0.."),
        "a negative damping must be refused by name"
    );

    // (xi) An anonymous procedure reference (P3 §2.3: stable procedure
    // identity).
    let anonymous = procedure(
        "",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    assert!(
        refused(&anonymous).contains("invalid stage procedure reference"),
        "an anonymous procedure must be refused by name"
    );

    // No refused preparation may have left anything behind.
    assert_eq!(
        ownership_digest(&ownership),
        json!({}),
        "refused preparations accumulated owned state"
    );
}

// ---------------------------------------------------------------------------
// host tier — the real installed worker through the application's field host
// ---------------------------------------------------------------------------

fn worker() -> PathBuf {
    PathBuf::from(
        std::env::var("QL_FIELD_WORKER")
            .expect("QL_FIELD_WORKER must name the installed ql-field-worker"),
    )
}

/// The application's plain opening: the composed default scene binding, its
/// host configuration, one supervised native worker behind it.
fn open_scene(instance: &str) -> FieldHost {
    let binding = ql_mef::continuous::scene_field::binding(BindingRequest {
        schema: BINDING_REQUEST.into(),
        instance_ref: instance.into(),
        texture: [32, 16],
        units_per_metre: 1.0,
        event: None,
        sky: None,
        field: None,
        geometry: Some(SceneGeometry {
            longitude_samples: 32,
            latitude_samples: 16,
            metres_per_unit: 1.0,
            attachment: 1,
        }),
        material: None,
        reception: None,
    })
    .unwrap();
    let config = serde_json::from_value(binding["host"].clone()).unwrap();
    FieldHost::open_scene(&worker(), config, Duration::from_secs(20)).unwrap()
}

/// Drives the host the way a consumer must: each request is addressed from
/// the LAST response's disclosed cursor — instance, event, subject,
/// generation, samples and the sequence successor. A refusal therefore
/// resyncs through the response itself, which is exactly the contract's
/// resynchronisation route (P2 §1.1).
struct Driver {
    host: FieldHost,
    current: Value,
}

impl Driver {
    fn open(instance: &str) -> Self {
        let host = open_scene(instance);
        let current = host.ready();
        assert_eq!(current["status"], "ready");
        assert_eq!(current["available"], json!(true));
        Self { host, current }
    }

    fn packet(&self, command: Value) -> HostRequest {
        HostRequest {
            schema: "ql.field-host-request/v1".into(),
            instance_ref: self.current["instance_ref"].as_str().unwrap().into(),
            event_ref: self.current["field"]["event_ref"].as_str().unwrap().into(),
            subject_ref: self.current["field"]["subject_ref"]
                .as_str()
                .unwrap()
                .into(),
            request_id: (self.current["last_request_id"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                + 1)
            .to_string(),
            expected_generation: self.current["field"]["generation"].as_str().unwrap().into(),
            expected_samples_elapsed: self.current["field"]["samples_elapsed"]
                .as_str()
                .unwrap()
                .into(),
            command: serde_json::from_value(command).unwrap(),
        }
    }

    fn send(&mut self, command: Value) -> Value {
        let request = self.packet(command);
        self.current = self.host.execute(request);
        self.current.clone()
    }

    /// Executes a hand-built request without touching the driver's cursor —
    /// the caller takes the returned response as the new current itself.
    fn execute_raw(&mut self, request: HostRequest) -> Value {
        let response = self.host.execute(request);
        self.current = response.clone();
        response
    }

    fn subject(&self) -> String {
        self.current["field"]["subject_ref"]
            .as_str()
            .unwrap()
            .to_owned()
    }

    fn generation(&self) -> u64 {
        self.current["field"]["generation"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap()
    }

    /// The state digest used for no-partial-mutation claims: the disclosed
    /// stage and field are structurally compared against a prior read.
    fn read_digest(&mut self) -> Value {
        let state = self.send(json!({"operation": "stage-state"}));
        assert_eq!(state["status"], "ok", "{}", state["error"]);
        json!({"stage": state["stage"], "field": state["field"]})
    }
}

// ---------------------------------------------------------------------------
// A04 (host) — staged read/change agreement against an independent ledger
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_a04_stage_state_agrees_with_independent_recomputation() {
    let pose = 2u8;
    let damping = 0.5f64;
    let damping_change = || StageChange::Damping {
        per_second: damping,
    };
    let clock_change = || StageChange::Clock {
        slot: "clock.inscription".into(),
        phase: LiftInput {
            turns: "1".into(),
            half_degrees: 360,
        },
    };
    let form_change = || StageChange::Form {
        operations: vec![M3Operation::SetPose { pose }],
    };

    // The independent ledger, derived from the contract and the commanded
    // values alone:
    //
    // - every applied effective change moves the observed native generation,
    //   and a combined apply on a fresh basis moves exactly the sum of its
    //   classes' measured movements (P2 §1.4: the applied result "identifies
    //   the native generations actually observed"; P2 §1.1: "Return a
    //   consistent revision/generation basis");
    // - after each apply, the staged read discloses the commanded value as
    //   the effective one (P2 §1.1 effective observation; P1 §0.3 exact deep
    //   coordinates);
    // - the receipt's claimed keys are exactly the ownership the readback
    //   names, and the receipt names the basis it was applied on.
    //
    // The per-class movements are MEASURED on a first host, never copied
    // from the implementation; the combined apply is then RECOMPUTED on a
    // second fresh host standing on the same initial basis.

    // Host one: reads, per-class measurement, per-class read/change
    // agreement. One procedure identity re-placing its own basis (P3 §2.4).
    let mut warm = Driver::open("psv:test-a04-warm");

    // A read is an observation, not a determinant (P2 §1.1): reading twice
    // must disclose the same basis and move nothing.
    let first = warm.send(json!({"operation": "stage-state"}));
    assert_eq!(first["status"], "ok", "{}", first["error"]);
    let second = warm.send(json!({"operation": "stage-state"}));
    assert_eq!(
        first["stage"], second["stage"],
        "a read changed the disclosed stage state"
    );
    assert_eq!(first["field"]["generation"], second["field"]["generation"]);
    assert_eq!(
        first["field"]["samples_elapsed"],
        second["field"]["samples_elapsed"]
    );

    let subject = warm.subject();
    let apply = |driver: &mut Driver, revision: u64, changes: Vec<StageChange>| {
        let applied = driver.send(
            serde_json::to_value(HostOperation::StageEvaluate {
                procedure: Box::new(StageProcedure {
                    revision,
                    ..procedure("ta-onta:stage:psv-a04", 1, &subject, changes)
                }),
            })
            .unwrap(),
        );
        assert_eq!(applied["status"], "ok", "{}", applied["error"]);
        assert_eq!(applied["stage"]["applied"], json!(true));
        applied
    };

    // Class one: damping. The staged read must disclose the commanded value.
    let basis = warm.generation();
    apply(&mut warm, 1, vec![damping_change()]);
    let damping_steps = warm.generation() - basis;
    let state = warm.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(damping),
        "the effective damping must equal the commanded damping"
    );
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["procedure_ref"],
        json!("ta-onta:stage:psv-a04")
    );

    // Class two: the display clock. Exact deep coordinates (P1 §0.3).
    let before = warm.generation();
    apply(&mut warm, 2, vec![clock_change()]);
    let clock_steps = warm.generation() - before;
    let state = warm.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["field"]["clock"]["inscription"]["turns"],
        json!("1"),
        "the display phase turns must equal the commanded exact coordinate"
    );
    assert_eq!(
        state["field"]["clock"]["inscription"]["half_degrees"],
        json!(360),
        "the display phase half_degrees must equal the commanded exact coordinate"
    );
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(damping),
        "the earlier class's effective value must stand"
    );

    // Class three: the form. The posed body is the effective one.
    let before = warm.generation();
    apply(&mut warm, 3, vec![form_change()]);
    let form_steps = warm.generation() - before;
    let state = warm.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["form"]["pose"],
        json!(pose),
        "the effective pose must equal the commanded pose"
    );

    assert!(
        damping_steps >= 1 && clock_steps >= 1 && form_steps >= 1,
        "every applied effective change must move the observed generation \
         (measured damping={damping_steps}, clock={clock_steps}, form={form_steps})"
    );
    assert_eq!(
        warm.current["field"]["samples_elapsed"], first["field"]["samples_elapsed"],
        "no advance happened; the samples cursor must stand still"
    );

    // Host two: the recomputation target. A fresh host stands on the same
    // initial basis; the combined procedure commands all three classes, each
    // differing from that standing basis, so its observed movement must
    // equal the sum of the measured per-class movements — no hidden extra
    // determinations, none lost.
    let mut fresh = Driver::open("psv:test-a04-fresh");
    assert_eq!(fresh.subject(), subject);
    let fresh_basis = fresh.generation();
    let applied = apply(
        &mut fresh,
        1,
        vec![form_change(), damping_change(), clock_change()],
    );
    assert_eq!(
        fresh.generation() - fresh_basis,
        damping_steps + clock_steps + form_steps,
        "the combined apply moved more or less than its own applied classes"
    );
    let stage = &applied["stage"];

    // The receipt names the basis it was applied on.
    assert_eq!(
        stage["event_ref"], applied["field"]["event_ref"],
        "the receipt must name the basis the changes were applied on"
    );
    // The claimed keys derive from procedure identity, rule version and slot
    // (P3 §2.4), and they are exactly what the readback discloses as owners.
    let receipt_ref = stage["procedure_ref"].as_str().unwrap();
    let receipt_revision = stage["revision"].as_u64().unwrap();
    let claimed: BTreeMap<String, String> = stage["contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|contribution| {
            (
                contribution["slot"].as_str().unwrap().to_owned(),
                contribution["key"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert_eq!(claimed.len(), 3, "three changes, three claimed slots");
    for slot in ["form", "material.damping", "clock.inscription"] {
        let key = &claimed[slot];
        assert!(
            *key == format!("{receipt_ref}@{receipt_revision}/{slot}"),
            "the claimed key must derive from procedure identity, version and slot: {key}"
        );
    }

    // The staged read must agree with the ledger, field for field.
    let state = fresh.send(json!({"operation": "stage-state"}));
    let disclosed = &state["stage"];
    assert_eq!(disclosed["schema"], "ql.stage-state/v1");
    assert_eq!(
        disclosed["generation"], applied["field"]["generation"],
        "the disclosure and the applied readback must carry one basis"
    );
    assert_eq!(
        disclosed["form"]["pose"],
        json!(pose),
        "effective pose must equal the commanded pose"
    );
    assert_eq!(
        disclosed["material"]["damping_per_second"],
        json!(damping),
        "effective damping must equal the commanded damping"
    );
    assert_eq!(
        state["field"]["clock"]["inscription"]["turns"],
        json!("1"),
        "the display phase turns must equal the commanded exact coordinate"
    );
    assert_eq!(
        state["field"]["clock"]["inscription"]["half_degrees"],
        json!(360),
        "the display phase half_degrees must equal the commanded exact coordinate"
    );
    for slot in ["form", "material.damping", "clock.inscription"] {
        assert_eq!(
            disclosed["slots"][slot]["owner"],
            json!(claimed[slot]),
            "slot {slot} must be owned by exactly the claimed key"
        );
    }
    assert_eq!(
        disclosed["slots"]["clock.lensing"]["owner"],
        Value::Null,
        "an unaddressed slot must stay unowned"
    );
    assert!(
        disclosed["bindings"].as_array().unwrap().is_empty(),
        "nothing was bound in this scenario"
    );
}

// ---------------------------------------------------------------------------
// A05 (host) — the admission envelope: refusal by cause, no partial mutation
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_a05_envelope_refuses_by_cause_and_never_partially_mutates() {
    let mut driver = Driver::open("psv:test-a05");
    let subject = driver.subject();
    let before = driver.read_digest();

    let damping_procedure = || {
        Box::new(procedure(
            "ta-onta:stage:psv-a05",
            1,
            &subject,
            vec![StageChange::Damping { per_second: 0.25 }],
        ))
    };

    // (1) A stale native cursor: the batch is refused by cause. The
    // acknowledgement must be honest — nothing applied (A05: "Reject a
    // stale/invalid batch without partial document mutation"; P2 §1.5: a
    // retry "first inspects the operation and current native generations").
    let mut stale = driver.packet(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: damping_procedure(),
        })
        .unwrap(),
    );
    stale.expected_generation = "999999".into();
    let refused = driver.execute_raw(stale);
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("stale native cursor"),
        "{}",
        refused["error"]
    );
    assert_eq!(refused["available"], json!(true), "the stage stays usable");
    assert_eq!(refused["field"], before["field"]);
    let after = driver.read_digest();
    assert_eq!(after, before, "the refused batch mutated state");

    // (2) A repeated request id carrying a different payload is a conflict
    // (P2 §1.5: "Repeating an ID with a different payload is a conflict").
    let spent: u64 = driver.current["last_request_id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let mut replay = driver.packet(json!({"operation": "stage-state"}));
    replay.request_id = spent.to_string();
    let replayed = driver.execute_raw(replay);
    assert_eq!(replayed["status"], "refused");
    assert!(
        replayed["error"]
            .as_str()
            .unwrap()
            .contains("stale, repeated or skipped host request sequence"),
        "{}",
        replayed["error"]
    );
    let after = driver.read_digest();
    assert_eq!(after, before, "the replayed request mutated state");

    // (3) A foreign subject on the envelope: a reference is not a grant of
    // authority (P1 §0.1 host law; P2 §1.3 authority validation).
    let mut foreign = driver.packet(json!({"operation": "stage-state"}));
    foreign.subject_ref = "person:psv-stranger".into();
    let refused = driver.execute_raw(foreign);
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("foreign schema/instance/event/subject"),
        "{}",
        refused["error"]
    );
    let after = driver.read_digest();
    assert_eq!(after, before, "the foreign envelope mutated state");

    // The stage stays usable on its current admitted state (P2 §1.5), and the
    // refused envelopes accumulated nothing: a fresh valid procedure applies
    // exactly its own single change.
    let applied = driver.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: damping_procedure(),
        })
        .unwrap(),
    );
    assert_eq!(applied["status"], "ok", "{}", applied["error"]);
    assert_eq!(
        driver.generation(),
        before["field"]["generation"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap()
            + 1,
        "only the one admitted apply may have advanced the basis"
    );
    let state = driver.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(0.25),
        "the effective damping is the one admitted change, nothing else"
    );
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["owner"],
        json!("ta-onta:stage:psv-a05@1/material.damping")
    );
    for slot in ["form", "clock.inscription", "clock.lensing"] {
        assert_eq!(
            state["stage"]["slots"][slot]["owner"],
            Value::Null,
            "no refused envelope may have left an owner behind on {slot}"
        );
    }
}

// ---------------------------------------------------------------------------
// A07 (host) — regeneration touches only its own slot; retire retains the
// authored value
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_a07_regeneration_revises_only_its_own_slot_and_retire_retains_authored() {
    let mut driver = Driver::open("psv:test-a07");
    let subject = driver.subject();

    let alpha = |revision: u64, per_second: f64| {
        Box::new(procedure(
            "ta-onta:stage:psv-alpha",
            revision,
            &subject,
            vec![StageChange::Damping { per_second }],
        ))
    };
    let beta = procedure(
        "ta-onta:stage:psv-beta",
        1,
        &subject,
        vec![StageChange::Clock {
            slot: "clock.lensing".into(),
            phase: LiftInput {
                turns: "0".into(),
                half_degrees: 90,
            },
        }],
    );

    let send_evaluate = |driver: &mut Driver, procedure: Box<StageProcedure>| {
        let applied =
            driver.send(serde_json::to_value(HostOperation::StageEvaluate { procedure }).unwrap());
        assert_eq!(applied["status"], "ok", "{}", applied["error"]);
        applied
    };

    send_evaluate(&mut driver, alpha(1, 0.5));
    send_evaluate(&mut driver, Box::new(beta.clone()));
    let state = driver.send(json!({"operation": "stage-state"}));
    let beta_owned = state["stage"]["slots"]["clock.lensing"].clone();
    assert_eq!(beta_owned["procedure_ref"], json!("ta-onta:stage:psv-beta"));
    assert_eq!(state["stage"]["material"]["damping_per_second"], json!(0.5));

    // Regenerate alpha at a new revision with a changed value: only alpha's
    // slot may move; beta's disclosure must be byte-identical (P3 §2.2:
    // updates replace their OWN generated basis).
    send_evaluate(&mut driver, alpha(2, 0.9));
    let state = driver.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["clock.lensing"], beta_owned,
        "beta's contribution moved under alpha's regeneration"
    );
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["revision"],
        json!(2),
        "the regenerated basis carries its new rule version"
    );
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(0.9),
        "the effective damping is the regenerated value"
    );

    // Retire alpha: its own key family is released, beta survives, and the
    // last commanded value stays in force as the now-authored state — an
    // explicitly chosen retained value (P3 §2.2), disclosed by the receipt.
    let retired = driver.send(
        serde_json::to_value(HostOperation::StageRetire {
            procedure_ref: "ta-onta:stage:psv-alpha".into(),
        })
        .unwrap(),
    );
    assert_eq!(retired["status"], "ok", "{}", retired["error"]);
    assert_eq!(
        retired["stage"]["retired"],
        json!(["ta-onta:stage:psv-alpha@2/material.damping"]),
        "retire must release only its own key family"
    );
    let state = driver.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["owner"],
        Value::Null,
        "the retired slot must stand unowned"
    );
    assert_eq!(
        state["stage"]["slots"]["clock.lensing"], beta_owned,
        "retiring alpha must leave beta's contribution intact"
    );
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(0.9),
        "the retained value must stay in force after ownership is released"
    );
    assert_eq!(
        state["stage"]["subject_ref"],
        json!(subject),
        "removing a contribution leaves the native subject intact (P1 §0.1)"
    );
}

// ---------------------------------------------------------------------------
// A10 (host) — a bound passage never re-enters; exhaustion stops the firing
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_a10_a_passage_never_reenters_and_exhaustion_stops_firing() {
    // A10's runaway failure, in this stage's shape: a bound procedure whose
    // own generated passage re-triggers the binding. The passage's scenes are
    // applied through the same instrument paths and must never re-enter the
    // firing (P3 §2.5: bounded feedback; only request-level determinants
    // admit bindings). The budget is the discriminator.
    let mut driver = Driver::open("psv:test-a10");
    let mut passage = procedure(
        "ta-onta:stage:psv-passage",
        1,
        &driver.subject(),
        vec![StageChange::Form {
            operations: vec![M3Operation::SetPose { pose: 5 }],
        }],
    );
    passage.passage = Some(StagePassage { scenes: 3 });
    passage.trigger = StageTrigger::Determinant {
        operation: "m1-advance".into(),
    };
    let bound = driver.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(passage),
            max_evaluations: Some(2),
        })
        .unwrap(),
    );
    assert_eq!(bound["status"], "ok", "{}", bound["error"]);
    assert_eq!(
        bound["stage"]["binding"]["determinant"],
        json!("m1-advance")
    );
    assert_eq!(bound["stage"]["binding"]["standing"], json!("active"));

    // Firing one: the invoked fold plus its two generated scenes — all real
    // determinants on the flow — yet exactly ONE evaluation consumed. Under
    // re-entry, the passage's internal advances would consume the budget
    // within this single request.
    let before = driver.generation();
    let first = driver.send(json!({"operation": "m1-advance", "ticks": 1}));
    assert_eq!(first["status"], "ok", "{}", first["error"]);
    let firings = first["stage_bound_firings"].as_array().unwrap();
    assert_eq!(firings.len(), 1, "the binding must fire exactly once");
    assert_eq!(firings[0]["applied"], json!(true));
    assert_eq!(
        firings[0]["receipt"]["generated_scenes"],
        json!(2),
        "the receipt must name its two generated scenes"
    );
    assert!(
        driver.generation() > before,
        "the invoked fold and its passage were real determinants"
    );
    let state = driver.send(json!({"operation": "stage-state"}));
    let binding = &state["stage"]["bindings"][0];
    assert_eq!(
        binding["evaluations"],
        json!(1),
        "the passage's internal determinants must never re-enter the firing"
    );
    assert_eq!(binding["standing"], json!("active"));

    // Firing two: the last admitted evaluation.
    let second = driver.send(json!({"operation": "m1-advance", "ticks": 1}));
    assert_eq!(second["status"], "ok", "{}", second["error"]);
    assert_eq!(second["stage_bound_firings"].as_array().unwrap().len(), 1);
    let state = driver.send(json!({"operation": "stage-state"}));
    let binding = &state["stage"]["bindings"][0];
    assert_eq!(binding["evaluations"], json!(2));
    assert_eq!(binding["max_evaluations"], json!(2));
    assert_eq!(
        binding["standing"],
        json!("exhausted"),
        "exhaustion must be inspectable"
    );

    // The budget is spent: the binding stops, while the stage itself
    // continues — the flow still advances (P3 §2.5: "stop or coalesce the
    // affected procedure ... while the rest of the stage continues").
    let before = driver.generation();
    let third = driver.send(json!({"operation": "m1-advance", "ticks": 1}));
    assert_eq!(third["status"], "ok", "{}", third["error"]);
    assert!(
        third["stage_bound_firings"].is_null(),
        "an exhausted binding must not fire"
    );
    assert!(
        driver.generation() > before,
        "the exhausted binding must not stop the stage"
    );
}

// ---------------------------------------------------------------------------
// determinism — two fresh hosts, one basis, byte-identical receipt and body
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_determinism_two_fresh_hosts_replay_identically() {
    // P3 §2.5: seeded generators with retained seed; canonicalised target
    // ordering. The stage's own contract is stricter: evaluation is
    // deterministic in the event basis — no clock, no randomness — so the
    // same procedure on the same basis must replay byte-identically on a
    // fresh host (A13's exact discrete identity).
    let mut one = Driver::open("psv:test-det-a");
    let mut two = Driver::open("psv:test-det-b");
    let seed = json!({"operation": "advance", "frames": 256, "muted": true});
    assert_eq!(one.send(seed.clone())["status"], "ok");
    assert_eq!(two.send(seed)["status"], "ok");

    let build = |driver: &Driver| {
        Box::new(procedure(
            "ta-onta:stage:psv-determinism",
            1,
            &driver.subject(),
            vec![StageChange::Form {
                operations: vec![
                    M3Operation::CastCreases {
                        angles_deg10: [120, -60, 30],
                        velocities_deg10: [0, 0, 0],
                    },
                    M3Operation::SetPose { pose: 3 },
                ],
            }],
        ))
    };
    let a = one.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: build(&one),
        })
        .unwrap(),
    );
    let b = two.send(
        serde_json::to_value(HostOperation::StageEvaluate {
            procedure: build(&two),
        })
        .unwrap(),
    );
    assert_eq!(a["status"], "ok", "{}", a["error"]);
    assert_eq!(b["status"], "ok", "{}", b["error"]);
    assert_eq!(
        a["stage"], b["stage"],
        "the stage receipts must be byte-identical on one basis"
    );
    assert_eq!(
        a["influence"]["shape_ref"], b["influence"]["shape_ref"],
        "the fold's shape identity must be identical"
    );
    assert_eq!(
        a["field"]["targets"], b["field"]["targets"],
        "the deformed body must be identical without hidden generator state"
    );
    assert_eq!(
        a["field"]["generation"], b["field"]["generation"],
        "the applied bases must carry the same generation"
    );
}

// ---------------------------------------------------------------------------
// A08 (host) — conflicts and binding limits refused by name
// ---------------------------------------------------------------------------

#[ignore = "requires the installed ql-field-worker"]
#[test]
fn psv_host_a08_conflicts_and_binds_are_refused_by_name() {
    let mut driver = Driver::open("psv:test-a08");
    let subject = driver.subject();

    // Two procedures writing one property need an explicit composition;
    // arrival order must not arbitrate (P3 §2.4) — the refusal names the
    // holder, and the holder's declared value stays in force.
    let first = Box::new(procedure(
        "ta-onta:stage:psv-first",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.75 }],
    ));
    let held = driver
        .send(serde_json::to_value(HostOperation::StageEvaluate { procedure: first }).unwrap());
    assert_eq!(held["status"], "ok", "{}", held["error"]);
    let second = Box::new(procedure(
        "ta-onta:stage:psv-second",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.2 }],
    ));
    let refused = driver
        .send(serde_json::to_value(HostOperation::StageEvaluate { procedure: second }).unwrap());
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("owned by ta-onta:stage:psv-first@1/material.damping"),
        "{}",
        refused["error"]
    );
    let state = driver.send(json!({"operation": "stage-state"}));
    assert_eq!(
        state["stage"]["slots"]["material.damping"]["owner"],
        json!("ta-onta:stage:psv-first@1/material.damping"),
        "the refusal must not have moved ownership"
    );
    assert_eq!(
        state["stage"]["material"]["damping_per_second"],
        json!(0.75),
        "arrival order must not arbitrate the conflicting value"
    );

    // Only determinant triggers bind (P3 §2.3: explicit invocation is an
    // evaluation, not a flow binding); an unknown determinant is refused by
    // name; an absent binding cannot be unbound.
    let mut invocation = procedure(
        "ta-onta:stage:psv-invocation",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    invocation.trigger = StageTrigger::Invocation;
    let bind = driver.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(invocation),
            max_evaluations: None,
        })
        .unwrap(),
    );
    assert_eq!(bind["status"], "refused");
    assert!(
        bind["error"].as_str().unwrap().contains("invocation"),
        "{}",
        bind["error"]
    );

    let mut unknown = procedure(
        "ta-onta:stage:psv-unknown",
        1,
        &subject,
        vec![StageChange::Damping { per_second: 0.5 }],
    );
    unknown.trigger = StageTrigger::Determinant {
        operation: "shutdown".into(),
    };
    let bind = driver.send(
        serde_json::to_value(HostOperation::StageBind {
            procedure: Box::new(unknown),
            max_evaluations: None,
        })
        .unwrap(),
    );
    assert_eq!(bind["status"], "refused");
    assert!(
        bind["error"]
            .as_str()
            .unwrap()
            .contains("unknown determinant"),
        "{}",
        bind["error"]
    );

    let unbind = driver.send(
        serde_json::to_value(HostOperation::StageUnbind {
            procedure_ref: "ta-onta:stage:psv-absent".into(),
        })
        .unwrap(),
    );
    assert_eq!(unbind["status"], "refused");
    assert!(
        unbind["error"]
            .as_str()
            .unwrap()
            .contains("no binding names"),
        "{}",
        unbind["error"]
    );
    let state = driver.send(json!({"operation": "stage-state"}));
    assert!(
        state["stage"]["bindings"].as_array().unwrap().is_empty(),
        "no refused bind may have left a binding behind"
    );
}
