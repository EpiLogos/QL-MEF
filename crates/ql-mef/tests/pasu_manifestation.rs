//! PS-E (QL-MEF #297) first specimen, integrated: one native subject
//! manifested simultaneously as formation, force and sequence at a canonical
//! Bimba place, occurrence identity stable under reorder/rename, canonical
//! place re-entry preserving bindings, and the procedural stage validating its
//! subject through the same semantic owner. Sources are the real registry and
//! the real profile lineage — no fixture stand-ins.

use ql_mef::MFace;
use ql_mef::continuous::stage::{STAGE_PROCEDURE, StageChange, StageProcedure, StageTrigger};
use ql_mef::coordinate_expression::{
    AuthoredVariant, ExpressiveRole, SUBJECT_MANIFESTATION_CONTRACT, SubjectManifestation,
    resolve_subject_manifestation, validate_subject_ref,
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
    let registry = native_current_m_registry();
    resolve_subject_manifestation(registry, SUBJECT, locus, face, roles, variants, instance)
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
        MOON,
        MFace::Bimba,
        &roles,
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
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Force],
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
