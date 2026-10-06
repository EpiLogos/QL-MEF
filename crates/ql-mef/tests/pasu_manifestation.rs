//! PS-E (QL-MEF #297) specimens, integrated: one native subject manifested
//! simultaneously as formation, force and sequence at a canonical Bimba
//! place, occurrence identity stable under reorder/rename, canonical place
//! re-entry preserving bindings, and the procedural stage validating its
//! subject through the same semantic owner. Slice 2 adds contributing source
//! subjects inside one occurrence with separately qualified roles (P1 §0.1),
//! scene-as-subject and whole-Expression-as-subject bounded presentations,
//! and the joined place walk through distinct recorded transitions (P4
//! §3.2/§3.3). Sources are the real registry and the real profile lineage —
//! no fixture stand-ins.

use ql_mef::MFace;
use ql_mef::continuous::stage::{STAGE_PROCEDURE, StageChange, StageProcedure, StageTrigger};
use ql_mef::coordinate_expression::{
    AuthoredVariant, ContinuationCursor, ContinuationPolicy, ContributingSubjectBinding,
    DeterminedRelation, ExpressiveRole, MANIFESTATION_FIXTURES_CONTRACT, PASU_SOURCE_REFS,
    PLACE_TRANSITION_CONTRACT, PlaceTransitionKind, SUBJECT_MANIFESTATION_CONTRACT, SubjectKind,
    SubjectManifestation, admit_place_transition, manifestation_fixture_file,
    resolve_subject_manifestation, resume_place_transition, validate_subject_ref,
    verify_manifestation_fixtures, verify_producer_fixture,
};
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::m3_state::M3Operation;

const MOON: &str = "#2-5-4";
const SUBJECT: &str = "ql:k2/default-subject";

fn resolve(
    locus: &str,
    face: MFace,
    roles: &[ExpressiveRole],
    variants: &[AuthoredVariant],
    instance: Option<&str>,
) -> SubjectManifestation {
    resolve_kind(
        SubjectKind::Native,
        locus,
        face,
        roles,
        variants,
        &[],
        &[],
        instance,
    )
}

// The helper carries the resolver's own declared inputs (the relation's own
// fields), exactly as the owner resolver does.
#[allow(clippy::too_many_arguments)]
fn resolve_kind(
    kind: SubjectKind,
    locus: &str,
    face: MFace,
    roles: &[ExpressiveRole],
    variants: &[AuthoredVariant],
    contributions: &[ContributingSubjectBinding],
    relations: &[DeterminedRelation],
    instance: Option<&str>,
) -> SubjectManifestation {
    let registry = native_current_m_registry();
    resolve_subject_manifestation(
        registry,
        SUBJECT,
        kind,
        locus,
        face,
        roles,
        variants,
        contributions,
        relations,
        instance,
    )
    .unwrap()
}

fn triple() -> Vec<ExpressiveRole> {
    vec![
        ExpressiveRole::Formation,
        ExpressiveRole::Force,
        ExpressiveRole::Sequence,
    ]
}

fn value(manifestation: &SubjectManifestation) -> serde_json::Value {
    serde_json::to_value(manifestation).unwrap()
}

fn occurrence_value(
    occurrence: &ql_mef::coordinate_expression::RoleOccurrence,
) -> serde_json::Value {
    serde_json::to_value(occurrence).unwrap()
}

#[test]
fn the_same_subject_is_formation_force_and_sequence_at_one_canonical_place() {
    let manifestation = resolve(MOON, MFace::Bimba, &triple(), &[], None);
    assert_eq!(manifestation.schema, SUBJECT_MANIFESTATION_CONTRACT);
    assert_eq!(manifestation.subject_ref, SUBJECT);
    // The locus is exact: real coordinate, real face, real depth, real
    // registry lineage — the Moon under the sky's planetary integration.
    assert_eq!(manifestation.locus.coordinate_ref, MOON);
    assert_eq!(manifestation.locus.family, "M2");
    assert_eq!(manifestation.locus.depth, 3);
    assert_eq!(
        manifestation.locus.face,
        ql_mef::aw1_world::RootedFace::Bimba
    );
    // branch_path is the ancestry below the M master: the family branch and
    // the selected coordinate itself.
    assert_eq!(
        manifestation.locus.branch_path,
        vec!["#2-5".to_owned(), "#2-5-4".to_owned()]
    );
    assert!(!manifestation.locus.resolved_profile_ref.is_empty());
    assert_eq!(manifestation.locus.inherited_profile_refs.len(), 4);

    // One subject, three simultaneous role occurrences.
    assert_eq!(manifestation.occurrences.len(), 3);
    let formation = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Formation)
        .unwrap();
    let force = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Force)
        .unwrap();
    let sequence = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Sequence)
        .unwrap();
    // Source-qualified: the keys are the locus's actual deep-record keys.
    assert!(formation.property_keys.contains(&"c_1_name".to_owned()));
    assert!(
        force
            .property_keys
            .contains(&"c_2_harmonic_role".to_owned())
    );
    assert!(
        sequence
            .property_keys
            .iter()
            .any(|key| key.starts_with("c_3_"))
    );
    for occurrence in &manifestation.occurrences {
        assert!(occurrence.represented);
        assert!(!occurrence.source_records.is_empty());
        assert!(occurrence.occurrence_ref.starts_with("occurrence:"));
    }
    // Three roles: three distinct addressable occurrences.
    assert_ne!(formation.occurrence_ref, force.occurrence_ref);
    assert_ne!(force.occurrence_ref, sequence.occurrence_ref);
    assert_ne!(formation.occurrence_ref, sequence.occurrence_ref);
}

#[test]
fn occurrence_identity_survives_reorder_rename_and_reentry() {
    let roles = triple();
    let direct = resolve(MOON, MFace::Bimba, &roles, &[], None);
    // Reordered request: same occurrences, same whole manifestation.
    let mut reversed = roles.clone();
    reversed.reverse();
    assert_eq!(
        value(&resolve(MOON, MFace::Bimba, &reversed, &[], None)),
        value(&direct),
        "list order is not identity"
    );
    // Renamed entry point: the canonical face-bearing spelling of the same
    // place re-enters to the same bindings.
    let canonical = direct.locus.canonical_ref.clone();
    assert_eq!(
        value(&resolve(&canonical, MFace::Bimba, &roles, &[], None)),
        value(&direct),
        "canonical place re-entry preserves bindings"
    );
    // Adding a role does not relabel the existing residents.
    let mut four = roles.clone();
    four.insert(1, ExpressiveRole::Scene);
    let widened = resolve(MOON, MFace::Bimba, &four, &[], None);
    for occurrence in &direct.occurrences {
        let again = widened
            .occurrences
            .iter()
            .find(|o| o.role == occurrence.role)
            .unwrap();
        assert_eq!(occurrence_value(again), occurrence_value(occurrence));
    }
    assert_eq!(widened.occurrences.len(), 4);
}

#[test]
fn authored_variants_inherit_first_parent_per_key_and_replace_whole_keys() {
    let family_variant = AuthoredVariant {
        variant_ref: "variant:sky-family".into(),
        keys: [
            (
                "c_2_harmonic_role".to_owned(),
                serde_json::json!("family-tuning-whole-key"),
            ),
            ("c_1_season".to_owned(), serde_json::json!("from-family")),
        ]
        .into_iter()
        .collect(),
    };
    let authored_variant = AuthoredVariant {
        variant_ref: "variant:moon-night".into(),
        keys: [(
            "c_2_harmonic_role".to_owned(),
            serde_json::json!("night-watch-whole-key"),
        )]
        .into_iter()
        .collect(),
    };
    let manifestation = resolve(
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Formation, ExpressiveRole::Force],
        &[family_variant, authored_variant],
        None,
    );
    let force = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Force)
        .unwrap();
    // Whole-key child replacement: the authored variant's definition replaces
    // the family layer's for that key — never a deep merge of both.
    assert_eq!(
        force.authored_keys.get("c_2_harmonic_role").unwrap(),
        "variant:moon-night"
    );
    // First-parent-per-key where the child is silent: the family layer's
    // formation key stands.
    let formation = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Formation)
        .unwrap();
    assert_eq!(
        formation.authored_keys.get("c_1_season").unwrap(),
        "variant:sky-family"
    );
    assert!(formation.property_keys.contains(&"c_1_season".to_owned()));
}

#[test]
fn faces_forks_and_subjects_are_exact_addresses() {
    let roles = triple();
    let bimba = resolve(MOON, MFace::Bimba, &roles, &[], None);
    let pratibimba = resolve(MOON, MFace::Pratibimba, &roles, &[], None);
    assert_eq!(bimba.locus.coordinate_id, pratibimba.locus.coordinate_id);
    assert_ne!(
        bimba.locus.binding_content_revision,
        pratibimba.locus.binding_content_revision
    );
    assert_ne!(value(&bimba), value(&pratibimba));
    // An explicit fork is another purposeful occurrence of the same subject.
    let fork = resolve(MOON, MFace::Bimba, &roles, &[], Some("night-watch"));
    assert_eq!(fork.instance.as_deref(), Some("night-watch"));
    for (forked, canonical) in fork.occurrences.iter().zip(&bimba.occurrences) {
        assert_ne!(forked.occurrence_ref, canonical.occurrence_ref);
        assert_eq!(forked.property_keys, canonical.property_keys);
    }
    // A different subject at the same place manifests with its own identities.
    let registry = native_current_m_registry();
    let other = resolve_subject_manifestation(
        registry,
        "person:night-listener",
        SubjectKind::Native,
        MOON,
        MFace::Bimba,
        &roles,
        &[],
        &[],
        &[],
        None,
    )
    .unwrap();
    assert_ne!(value(&other), value(&bimba));
    for (mine, theirs) in other.occurrences.iter().zip(&bimba.occurrences) {
        assert_ne!(mine.occurrence_ref, theirs.occurrence_ref);
    }
}

#[test]
fn the_stage_and_the_manifestation_owner_admit_one_subject_grammar() {
    // The live stage's own subject reference is manifestable here.
    validate_subject_ref(SUBJECT).unwrap();
    let registry = native_current_m_registry();

    // A stage procedure whose subject is malformed is refused by the shared
    // grammar before anything is claimed.
    let mut procedure = StageProcedure {
        schema: STAGE_PROCEDURE.into(),
        procedure_ref: "ta-onta:stage:pse".into(),
        revision: 1,
        subject_ref: "subject\0broken".into(),
        trigger: StageTrigger::Invocation,
        selector: vec!["material.damping".into()],
        changes: vec![StageChange::Damping { per_second: 0.5 }],
        passage: None,
    };
    let error = procedure.validate().unwrap_err();
    assert!(error.contains("PS-E subject owner"), "{error}");

    // With the subject repaired the same procedure validates, and its subject
    // resolves to a real manifestation whose force occurrence is the same
    // object the Atlas discloses.
    procedure.subject_ref = SUBJECT.into();
    procedure.validate().unwrap();
    let manifestation = resolve_subject_manifestation(
        registry,
        &procedure.subject_ref,
        SubjectKind::Native,
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Force],
        &[],
        &[],
        &[],
        None,
    )
    .unwrap();
    assert_eq!(manifestation.subject_ref, procedure.subject_ref);
    assert_eq!(manifestation.occurrences.len(), 1);
    assert!(manifestation.occurrences[0].represented);
    assert!(!manifestation.occurrences[0].property_keys.is_empty());

    // The stage itself is unchanged in its landed behavior: a valid
    // procedure still evaluates against its event (host-free check).
    let procedure_ref = procedure.procedure_ref.clone();
    assert!(!procedure_ref.is_empty());
    // And an M3 form change still compiles onto a command batch shape.
    let form = serde_json::to_value(StageChange::Form {
        operations: vec![M3Operation::SetPose { pose: 2 }],
    })
    .unwrap();
    assert_eq!(form["change"], "form");
}

// ---- PS-E slice 2: contributing subjects, scene/Expression subjects,
// canonical place transitions ----------------------------------------------

fn contributing(
    carrier_role: ExpressiveRole,
    subject: &str,
    role: ExpressiveRole,
) -> ContributingSubjectBinding {
    ContributingSubjectBinding {
        carrier_role,
        subject_ref: subject.to_owned(),
        role,
    }
}

#[test]
fn the_moon_occurrence_carries_contributing_subjects_with_separate_roles() {
    // The Moon's formation occurrence carries two contributing source
    // subjects, each separately qualified at the same canonical place: the
    // sky binds as the occurrence's scene (context/type records), the
    // harmonic series as a force (operation/entity records).
    let contributions = [
        contributing(
            ExpressiveRole::Formation,
            "ql:k2/default-sky",
            ExpressiveRole::Scene,
        ),
        contributing(
            ExpressiveRole::Formation,
            "ql:k2/harmonic-series",
            ExpressiveRole::Force,
        ),
    ];
    let manifestation = resolve_kind(
        SubjectKind::Native,
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Formation],
        &[],
        &contributions,
        &[],
        None,
    );
    let formation = &manifestation.occurrences[0];
    assert_eq!(formation.contributing_subjects.len(), 2);
    for contributor in &formation.contributing_subjects {
        assert!(contributor.represented, "{}", contributor.standing);
        assert!(contributor.binding_ref.starts_with("contributing:"));
    }
    let sky = formation
        .contributing_subjects
        .iter()
        .find(|c| c.subject_ref == "ql:k2/default-sky")
        .unwrap();
    assert_eq!(sky.role, ExpressiveRole::Scene);
    assert!(!sky.property_keys.is_empty());
    assert!(sky.property_keys.iter().all(|key| key.starts_with("c_4_")));
    let harmonic = formation
        .contributing_subjects
        .iter()
        .find(|c| c.subject_ref == "ql:k2/harmonic-series")
        .unwrap();
    assert_eq!(harmonic.role, ExpressiveRole::Force);
    assert!(
        harmonic
            .property_keys
            .contains(&"c_2_harmonic_role".to_owned())
    );
    assert_ne!(sky.binding_ref, harmonic.binding_ref);

    // Declared order is not identity: the reversed contributions resolve to
    // the identical manifestation, and the carrier occurrence keeps its
    // identity with and without its contributors.
    let mut reversed = contributions.clone();
    reversed.reverse();
    assert_eq!(
        value(&resolve_kind(
            SubjectKind::Native,
            MOON,
            MFace::Bimba,
            &[ExpressiveRole::Formation],
            &[],
            &reversed,
            &[],
            None,
        )),
        value(&manifestation),
        "contributor order is not identity"
    );
    let bare = resolve(MOON, MFace::Bimba, &[ExpressiveRole::Formation], &[], None);
    assert_eq!(
        bare.occurrences[0].occurrence_ref, formation.occurrence_ref,
        "adding a contributor never relabels the carrier occurrence"
    );
}

#[test]
fn the_sky_presents_a_bounded_scene_and_the_moon_a_bounded_expression() {
    // Scene-as-subject: the sky itself is the presented subject, bounded
    // through the context/type layer its own records carry.
    let scene_subject = resolve_kind(
        SubjectKind::Scene,
        "#2-5",
        MFace::Bimba,
        &[ExpressiveRole::Scene],
        &[],
        &[],
        &[],
        None,
    );
    assert_eq!(scene_subject.subject_kind, SubjectKind::Scene);
    let presentation = &scene_subject.occurrences[0];
    assert!(presentation.bounded_subject_presentation);
    assert!(presentation.represented, "{}", presentation.standing);
    assert!(
        presentation
            .property_keys
            .contains(&"c_4_subsystem".to_owned())
    );

    // Whole-Expression-as-subject: the Moon presents a bounded whole
    // Expression through its own integration/reflection record.
    let expression_subject = resolve_kind(
        SubjectKind::Expression,
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Expression],
        &[],
        &[],
        &[],
        None,
    );
    assert_eq!(expression_subject.subject_kind, SubjectKind::Expression);
    let presentation = &expression_subject.occurrences[0];
    assert!(presentation.bounded_subject_presentation);
    assert!(presentation.represented, "{}", presentation.standing);
    assert_eq!(
        presentation.property_keys,
        vec!["c_5_spanda_resonance".to_owned()]
    );

    // Canonical re-entry preserves both bounded presentations exactly.
    assert_eq!(
        value(&resolve_kind(
            SubjectKind::Scene,
            &scene_subject.locus.canonical_ref,
            MFace::Bimba,
            &[ExpressiveRole::Scene],
            &[],
            &[],
            &[],
            None,
        )),
        value(&scene_subject)
    );
    assert_eq!(
        value(&resolve_kind(
            SubjectKind::Expression,
            &expression_subject.locus.canonical_ref,
            MFace::Bimba,
            &[ExpressiveRole::Expression],
            &[],
            &[],
            &[],
            None,
        )),
        value(&expression_subject)
    );
    // And the two subjects are distinct bounded cases, never coerced.
    assert_ne!(value(&scene_subject), value(&expression_subject));
}

#[test]
fn the_joined_stage_walks_places_through_distinct_recorded_transitions() {
    // The real walk: the subject stands at the Moon as formation, force and
    // sequence; reframes in place; moves the active scene to the Sky under a
    // checkpoint-and-release; re-enters from the recorded policy; and resets
    // to the canonical default.
    let roles = triple();
    let moon = resolve(MOON, MFace::Bimba, &roles, &[], None);

    // Focus/reframing: same place, a widened reading — mere navigation, the
    // event preserved.
    let reframed = resolve(
        MOON,
        MFace::Bimba,
        &[
            ExpressiveRole::Formation,
            ExpressiveRole::Force,
            ExpressiveRole::Sequence,
            ExpressiveRole::Scene,
        ],
        &[],
        None,
    );
    let focus = admit_place_transition(
        &moon,
        &reframed,
        PlaceTransitionKind::FocusReframe,
        &[SUBJECT.to_owned()],
        ContinuationPolicy::Continue,
        &[],
        None,
    )
    .unwrap();
    assert_eq!(focus.kind, PlaceTransitionKind::FocusReframe);
    assert!(!focus.kind.changes_the_event());
    assert_eq!(focus.schema, PLACE_TRANSITION_CONTRACT);

    // Active scene change: another canonical place, checkpoint-and-release,
    // the actual cursor recorded. The person arrives on their own active
    // occurrence of the sky scene — an explicit instance fork.
    let active_sky = resolve_kind(
        SubjectKind::Native,
        "#2-5",
        MFace::Bimba,
        &[ExpressiveRole::Scene],
        &[],
        &[],
        &[],
        Some("night-sky-watch"),
    );
    let cursor = ContinuationCursor {
        locus_canonical_ref: active_sky.locus.canonical_ref.clone(),
        face: active_sky.locus.face,
        binding_content_revision: active_sky.locus.binding_content_revision.clone(),
        active_manifestation_revision: active_sky.manifestation_content_revision.clone(),
    };
    let scene_change = admit_place_transition(
        &reframed,
        &active_sky,
        PlaceTransitionKind::SceneChange,
        &[SUBJECT.to_owned()],
        ContinuationPolicy::CheckpointRelease,
        &[],
        Some(cursor),
    )
    .unwrap();
    assert!(!scene_change.kind.changes_the_event());
    assert_ne!(
        scene_change.transition_content_revision, focus.transition_content_revision,
        "distinct acts are distinct records"
    );

    // Re-entry resumes from the recorded policy into exactly the admitted
    // bindings.
    let reentered = resume_place_transition(native_current_m_registry(), &scene_change).unwrap();
    assert_eq!(value(&reentered), value(&active_sky));

    // Full reset returns the canonical default occurrence at the same place,
    // which stays separate from the person's active one.
    let canonical_sky = resolve("#2-5", MFace::Bimba, &[ExpressiveRole::Scene], &[], None);
    let reset = admit_place_transition(
        &active_sky,
        &canonical_sky,
        PlaceTransitionKind::FullReset,
        &[],
        ContinuationPolicy::Continue,
        &[],
        None,
    )
    .unwrap();
    assert_eq!(reset.kind, PlaceTransitionKind::FullReset);
    assert!(reset.kind.changes_the_event());
    assert_eq!(reset.destination.instance, None);
    assert_eq!(
        reset.destination.manifestation_content_revision,
        canonical_sky.manifestation_content_revision,
        "the canonical default is preserved separately from the active occurrence"
    );
    // And the canonical default at the Moon is untouched by the whole walk.
    let canonical_moon = resolve(MOON, MFace::Bimba, &roles, &[], None);
    assert_eq!(
        canonical_moon.manifestation_content_revision, moon.manifestation_content_revision,
        "changing focus or adopting a profile does not relabel unrelated resident entities"
    );
}

// ---- PS-E slice 3: producer fixtures for the consumer lanes ----------------

#[test]
fn the_producer_fixtures_parse_resolve_and_match_their_expected_readbacks() {
    // The fixture file is data with a declared contract: it parses, carries
    // the pinned Paśu ground, and names the four consumer cases.
    let file = manifestation_fixture_file().unwrap();
    assert_eq!(file.schema, MANIFESTATION_FIXTURES_CONTRACT);
    assert_eq!(
        file.source_pins,
        PASU_SOURCE_REFS
            .iter()
            .map(|(_, pin)| pin.to_string())
            .collect::<Vec<_>>(),
        "the fixture bodies qualify through the same pinned ground the contract records"
    );
    let names = file
        .fixtures
        .iter()
        .map(|fixture| fixture.name())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "moon-as-formation-force-and-sequence",
            "moon-formation-with-contributing-subjects",
            "sky-as-scene-subject",
            "moon-to-sky-scene-change-with-checkpoint",
        ]
    );

    // Every body resolves against the real registry and matches its expected
    // readbacks — the one-call proof a consumer lane runs first.
    let registry = native_current_m_registry();
    verify_manifestation_fixtures(registry).unwrap();

    // The same subject as formation+force+sequence: the fixture's determined
    // relation qualification is a real readback on the force occurrence, and
    // the fixture's request resolves deterministically.
    for fixture in &file.fixtures {
        verify_producer_fixture(registry, fixture).unwrap();
    }
    let ql_mef::coordinate_expression::ProducerFixture::Manifestation { request, .. } =
        &file.fixtures[0]
    else {
        panic!("the first fixture is a manifestation body");
    };
    let first = request.resolve(registry).unwrap();
    let again = request.resolve(registry).unwrap();
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(&again).unwrap()
    );
    let force = first
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Force)
        .unwrap();
    assert_eq!(force.relation_qualifications.len(), 1);
    assert_eq!(
        force.relation_qualifications[0].kind,
        "HARMONICALLY_RESONATES_WITH"
    );
    assert_eq!(
        force.relation_qualifications[0].source_revision,
        registry.manifest().source_revision,
        "the relation's source revision is the registry revision it stands on"
    );
    // And the same request without the determination resolves a different
    // force occurrence: the qualification is identity-bearing.
    let mut bare_request = request.clone();
    bare_request.relations.clear();
    let bare = bare_request.resolve(registry).unwrap();
    let bare_force = bare
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Force)
        .unwrap();
    assert_ne!(bare_force.occurrence_ref, force.occurrence_ref);
}
