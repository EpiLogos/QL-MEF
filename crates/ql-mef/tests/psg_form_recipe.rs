//! PS-G (QL-MEF #299) first vertical specimen, end to end: the landed PS-E
//! producer fixture resolves to a real manifestation through the native
//! registry; the form recipe names the entity-to-form determination for its
//! formation occurrence and compiles it to the exact M3Operations the Ta-Onta
//! stage routes; the compiled command applies to the real M3State owner and
//! the form moves; and the refusals are by name where the source determines
//! no form. The body-movement proof on the installed worker is the existing
//! `tests/ta_onta_stage.rs` E2E path, which routes these same operations.
use ql_core::Codon64;
use ql_mef::MFace;
use ql_mef::continuous::coupled::CoupledInput;
use ql_mef::continuous::stage::{
    STAGE_PROCEDURE, StageChange, StageOwnership, StageProcedure, StageTrigger, evaluate,
};
use ql_mef::coordinate_expression::{
    DeterminedRelation, ExpressiveRole, MANIFESTATION_FIXTURES_CONTRACT, SubjectKind,
    SubjectManifestation, manifestation_fixture_file, resolve_subject_manifestation,
    verify_manifestation_fixtures, verify_producer_fixture,
};
use ql_mef::form_recipe::{
    DeclaredMobility, DeclaredPolarity, DeclaredSite, FORM_LAW_SOURCE_REFS, FORM_RECIPE_CONTRACT,
    FormDetermination, FormRecipe, resolve_form_recipe,
};
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::m3_state::M3State;

const MOON: &str = "#2-5-4";
const SUBJECT: &str = "ql:k2/default-subject";
/// A real Nara-family locus whose records carry c_2_/c_3_/c_4_ keys but no
/// c_1_ formation keys: the source qualifies no form there.
const UNREPRESENTED_FORMATION: &str = "#4.5.0";

const EVENT: &str = include_str!("../../../fixtures/kernel/scene-default-event-v2.json");

/// The specimen's declared entity-to-form determination: the formation
/// occurrence stands on the ATC fold motif — X yin/moving (A), the hinge Y
/// yang/moving (T), Z yin/resting (C). Declared by the caller, like a
/// determined relation; the recipe verifies and compiles, never selects.
fn specimen_determination() -> FormDetermination {
    FormDetermination::FoldMotif {
        sites: [
            DeclaredSite {
                polarity: DeclaredPolarity::Yin,
                mobility: DeclaredMobility::Moving,
            },
            DeclaredSite {
                polarity: DeclaredPolarity::Yang,
                mobility: DeclaredMobility::Moving,
            },
            DeclaredSite {
                polarity: DeclaredPolarity::Yin,
                mobility: DeclaredMobility::Resting,
            },
        ],
    }
}

/// The landed PS-E producer fixture, resolved against the real registry: the
/// Moon manifested as formation, force and sequence at `#2-5-4`.
fn moon_manifestation() -> SubjectManifestation {
    let file = manifestation_fixture_file().unwrap();
    assert_eq!(file.schema, MANIFESTATION_FIXTURES_CONTRACT);
    let registry = native_current_m_registry();
    let fixture = file
        .fixtures
        .iter()
        .find(|f| f.name() == "moon-as-formation-force-and-sequence")
        .unwrap();
    verify_producer_fixture(registry, fixture).unwrap();
    match fixture {
        ql_mef::coordinate_expression::ProducerFixture::Manifestation { request, .. } => {
            request.resolve(registry).unwrap()
        }
        _ => panic!("the named fixture is a manifestation body"),
    }
}

#[test]
fn the_producer_fixture_ground_stands_before_any_recipe() {
    // The whole landed packet still verifies against the real owners: the
    // recipe builds on it, never forks it.
    verify_manifestation_fixtures(native_current_m_registry()).unwrap();
    let manifestation = moon_manifestation();
    assert_eq!(manifestation.subject_ref, SUBJECT);
    assert_eq!(manifestation.locus.coordinate_ref, MOON);
    let formation = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Formation)
        .unwrap();
    assert!(formation.represented);
    assert!(formation.property_keys.contains(&"c_1_name".to_owned()));
}

#[test]
fn the_fixture_manifestation_compiles_to_a_source_qualified_form_recipe() {
    let manifestation = moon_manifestation();
    let recipe = resolve_form_recipe(&manifestation, specimen_determination()).unwrap();
    assert_eq!(recipe.schema, FORM_RECIPE_CONTRACT);
    assert_eq!(recipe.subject_ref, SUBJECT);
    assert_eq!(recipe.occurrence_ref, {
        let formation = manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Formation)
            .unwrap();
        formation.occurrence_ref.clone()
    });
    assert_eq!(recipe.locus_coordinate_ref, MOON);
    assert!(!recipe.locus_binding_content_revision.is_empty());
    // The qualification's source basis is carried: payload pins, file paths
    // and source revisions of the records that qualify the formation
    // occurrence — re-verifiable, not asserted.
    assert!(!recipe.source_basis.is_empty());
    assert!(recipe.source_basis.iter().all(|record| {
        !record.payload_sha256.is_empty()
            && !record.source_revision.is_empty()
            && !record.file_path.is_empty()
            && record
                .property_keys
                .iter()
                .all(|key| key.starts_with("c_1_"))
    }));
    assert!(
        recipe
            .source_basis
            .iter()
            .any(|record| record.property_keys.contains(&"c_1_name".to_owned()))
    );
    // The standing and the law's own source pins travel with the recipe.
    assert!(recipe.standing.contains("declared-form-determination"));
    assert!(
        FORM_LAW_SOURCE_REFS
            .iter()
            .any(|(repo, reference)| *repo == "EpiLogos/QL-MEF"
                && reference.contains("rūpa grammar"))
    );
    // The recipe travels: serialization round trip is exact.
    let json = serde_json::to_value(&recipe).unwrap();
    let back: FormRecipe = serde_json::from_value(json).unwrap();
    assert_eq!(
        serde_json::to_value(&back).unwrap(),
        serde_json::to_value(&recipe).unwrap()
    );
}

#[test]
fn the_recipe_compiles_the_exact_operations_the_stage_routes() {
    let manifestation = moon_manifestation();
    let recipe = resolve_form_recipe(&manifestation, specimen_determination()).unwrap();
    // The cast law's canonical telemetry: yin valley +22.5°, yang mountain
    // −22.5°, moving +22.5°/tick, resting 0 (deg10), in body order X, Y, Z.
    let ops = &recipe.operations;
    assert_eq!(ops.len(), 1);
    match ops[0] {
        ql_mef::m3_state::M3Operation::CastCreases {
            angles_deg10,
            velocities_deg10,
        } => {
            assert_eq!(angles_deg10, [225, -225, 225]);
            assert_eq!(velocities_deg10, [225, 225, 0]);
        }
        _ => panic!("a fold-motif determination compiles to CastCreases"),
    }
    // The form they resolve to, exactly: ATC, hinge T, the pair quanta.
    assert_eq!(recipe.form.address, 0b00_01_10);
    assert_eq!(recipe.form.nucleotide_bits, [0, 1, 2]);
    assert_eq!(recipe.form.hinge_bits, 1);
    assert_eq!(recipe.form.pair_angle_xy_deg10, 225);
    assert_eq!(recipe.form.pair_angle_yz_deg10, 6 * 225);
    assert_eq!(
        recipe.form.state_count,
        Codon64::new(0b00_01_10).rotational_state_count()
    );
}

#[test]
fn the_compiled_recipe_moves_the_form_on_the_real_m3_owner() {
    let event: CoupledInput = serde_json::from_str(EVENT).unwrap();
    let manifestation = moon_manifestation();
    let recipe = resolve_form_recipe(&manifestation, specimen_determination()).unwrap();

    // The stage procedure the recipe compiles into: one Form change on the
    // event's own command batch — the exact path tests/ta_onta_stage.rs
    // proves moves the live body.
    let procedure = StageProcedure {
        schema: STAGE_PROCEDURE.into(),
        procedure_ref: "ta-onta:stage:psg-form-recipe".into(),
        revision: 1,
        subject_ref: recipe.subject_ref.clone(),
        trigger: StageTrigger::Invocation,
        selector: vec!["form".into()],
        changes: vec![StageChange::Form {
            operations: recipe.operations.clone(),
        }],
        passage: None,
    };
    procedure.validate().unwrap();
    assert_eq!(procedure.subject_ref, event.m3.subject_ref);
    let mut ownership = StageOwnership::default();
    // The owner's applied generation basis (0) sits behind the event's own
    // M2/M3 generation (1), so the composer keeps the event's own M3
    // generation as the command's expected generation (its documented rule).
    let plan = evaluate(&procedure, &event, &mut ownership, 0, 1, 1).unwrap();
    assert_eq!(plan.contributions.len(), 1);
    assert_eq!(plan.contributions[0].slot, "form");
    assert_eq!(
        plan.contributions[0].warrant["determinant"],
        serde_json::json!("the event's own M3 form law")
    );
    let next = plan.event.as_ref().unwrap();
    assert_eq!(next.m3_commands.len(), 1);
    let command = &next.m3_commands[0];
    assert_eq!(command.event_ref, event.m1.event_ref);
    assert_eq!(command.subject_ref, event.m3.subject_ref);
    assert_eq!(
        command.expected_generation,
        event.m3.stamp.identity.profile_generation
    );
    let rendered = serde_json::to_value(command).unwrap();
    assert_eq!(
        rendered["operations"],
        serde_json::to_value(&recipe.operations).unwrap()
    );

    // And the command applies to the real M3State owner: the form moves from
    // the event's standing address (7) to the recipe's declared form (ATC),
    // at the cast law's exact site telemetry.
    let mut state = M3State::new(event.m3.clone()).unwrap();
    let before = state.snapshot();
    assert_eq!(before["form"]["address"], 7);
    let receipt = state.apply(command.clone()).unwrap();
    assert_eq!(receipt.status, "applied");
    assert_eq!(receipt.before_generation, 1);
    assert_eq!(receipt.after_generation, 2);
    let after = state.snapshot();
    assert_eq!(after["form"]["address"], recipe.form.address);
    assert_eq!(after["form"]["address"], 0b00_01_10);
    assert_eq!(
        after["form"]["angles_deg10"],
        serde_json::json!([225, -225, 225])
    );
    assert_eq!(
        after["form"]["velocities_deg10"],
        serde_json::json!([225, 225, 0])
    );
}

#[test]
fn refusals_name_where_the_source_determines_no_form() {
    let registry = native_current_m_registry();

    // No formation occurrence requested: the recipe names the roles it saw.
    let force_only = resolve_subject_manifestation(
        registry,
        SUBJECT,
        SubjectKind::Native,
        MOON,
        MFace::Bimba,
        &[ExpressiveRole::Force],
        &[],
        &[],
        &[DeterminedRelation {
            role: ExpressiveRole::Force,
            relation_ref: "bimba:relation:0bcfeaf7bf1582d0c70f9f10".into(),
        }],
        None,
    )
    .unwrap();
    let error = resolve_form_recipe(&force_only, specimen_determination()).unwrap_err();
    assert!(error.contains("formation occurrence"), "{error}");
    assert!(error.contains("force"), "{error}");

    // The locus qualifies no formation layer: #4.5.0's records carry c_2_,
    // c_3_ and c_4_ keys but no c_1_ keys — the source determines no form
    // there, and the recipe refuses rather than fabricating.
    let unrepresented = resolve_subject_manifestation(
        registry,
        SUBJECT,
        SubjectKind::Native,
        UNREPRESENTED_FORMATION,
        MFace::Bimba,
        &[ExpressiveRole::Formation],
        &[],
        &[],
        &[],
        None,
    )
    .unwrap();
    let formation = &unrepresented.occurrences[0];
    assert!(
        !formation.represented,
        "the fixture locus must stay source-bare"
    );
    assert!(formation.source_records.is_empty());
    let error = resolve_form_recipe(&unrepresented, specimen_determination()).unwrap_err();
    assert!(error.contains("not represented in source"), "{error}");
    assert!(error.contains("c_1_"), "{error}");
    assert!(error.contains("determines no form"), "{error}");

    // The address law bounds the declared field.
    let manifestation = moon_manifestation();
    let error = resolve_form_recipe(&manifestation, FormDetermination::Address { address: 64 })
        .unwrap_err();
    assert!(error.contains("six-bit field 0..64"), "{error}");
}
