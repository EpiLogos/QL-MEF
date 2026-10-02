//! Real native registry/M3 producer tests. These do not simulate stage ACKs.
use ql_mef::{MFace, aw1_world, coordinate_expression, m_tree, m3_state};
#[path = "../src/procedural_manifestation.rs"]
pub mod procedural_manifestation;
use procedural_manifestation::*;
use serde_json::{Value, json};

fn subject(reference: &str) -> NativeSubject {
    let manifest = m_tree::native_current_m_registry().manifest();
    NativeSubject {
        subject_ref: reference.into(),
        native_owner: "ql-mef".into(),
        presentation_role: SubjectRole::Thing,
        sources: vec![NativeReading {
            reference: format!(
                "{}:{}",
                manifest.source_repository, manifest.source_dataset_tree
            ),
            revision: manifest.source_snapshot_sha256.clone(),
            availability: ReadingAvailability::Available,
        }],
        readings: vec![],
        actions: vec![],
    }
}

fn request(coordinate: &str, face: MFace) -> ManifestationRequest {
    let registry = m_tree::native_current_m_registry();
    let atlas =
        coordinate_expression::resolve_coordinate_expression(registry, coordinate, face).unwrap();
    let canonical = if face == MFace::Bimba {
        atlas.rooted_world.direct.canonical_ref.clone()
    } else {
        atlas.rooted_world.conjugate.canonical_ref.clone()
    };
    ManifestationRequest {
        coordinate_ref: canonical.clone(),
        face,
        expected_registry_revision: registry.manifest().registry_revision.clone(),
        expected_profile_revision: atlas.binding_content_revision,
        occasion_ref: "occasion:native-source".into(),
        principal: subject(&canonical),
        contributors: vec![],
        required_relations: vec![],
        intended_act: "inhabit source-qualified composition".into(),
        manifestations: vec![ManifestationInput {
            output_slot: "form".into(),
            occurrence: OccurrenceAddress {
                expression_ref: "expression:acceptance".into(),
                scene_ref: Some("expression:acceptance:scene:clock".into()),
                entity_ref: Some("expression:acceptance:entity:source".into()),
                component: ManifestationRole::Formation,
                constituent_ref: None,
            },
            recipe: SourceBasis {
                source_ref: "recipe:source-native-form".into(),
                revision: "1".into(),
            },
            material: json!({"source_operation":"native_m3_state","treatment":"glyph_mask"}),
            standing: ChoiceStanding::SourceDerived,
        }],
    }
}

#[test]
fn same_subject_manifests_as_formation_force_sequence_and_whole_without_new_subjects() {
    let registry = m_tree::native_current_m_registry();
    let mut request = request("#3", MFace::Bimba);
    for (slot, role) in [
        ("force", ManifestationRole::Force),
        ("sequence", ManifestationRole::Sequence),
        ("scene", ManifestationRole::Scene),
        ("whole", ManifestationRole::Expression),
    ] {
        let mut output = request.manifestations[0].clone();
        output.output_slot = slot.into();
        output.occurrence.component = role;
        if role == ManifestationRole::Scene {
            output.occurrence.entity_ref = None;
        }
        if role == ManifestationRole::Expression {
            output.occurrence.entity_ref = None;
            output.occurrence.scene_ref = None;
        }
        request.manifestations.push(output);
    }
    for (slot, role) in [
        ("field", ManifestationRole::Field),
        ("layer", ManifestationRole::Layer),
        ("link", ManifestationRole::SequenceLink),
        ("driver", ManifestationRole::Modulation),
    ] {
        let mut output = request.manifestations[0].clone();
        output.output_slot = slot.into();
        output.occurrence.component = role;
        if role == ManifestationRole::Field {
            output.occurrence.entity_ref = None;
        } else {
            output.occurrence.constituent_ref = Some(format!("native-{slot}"));
        }
        request.manifestations.push(output);
    }
    let plan = resolve_manifestation(registry, request).unwrap();
    let bindings = native_procedural_bindings(&plan).unwrap();
    assert_eq!(bindings.len(), 9);
    assert_eq!(plan.native_subject_changes.len(), 1);
    assert_eq!(plan.native_subject_changes[0]["change"], "subject_bind");
    for binding in &bindings {
        assert_eq!(
            binding["principal"]["subject_ref"],
            plan.principal.subject_ref
        );
        assert!(binding["address"].get("property").unwrap().is_null());
        assert_eq!(
            binding["locus"]["ref"],
            plan.atlas.rooted_world.direct.canonical_ref
        );
    }
    assert_eq!(bindings[3]["address"]["component"], "scene");
    assert_eq!(bindings[4]["address"]["component"], "expression");
    assert_eq!(bindings[5]["address"]["component"], "field");
    assert_eq!(bindings[6]["address"]["component"], "layer");
    assert_eq!(bindings[7]["address"]["component"], "sequence_link");
    assert_eq!(bindings[8]["address"]["component"], "driver");
}

#[test]
fn exact_prime_deep_source_only_places_keep_current_profile_lineage() {
    let registry = m_tree::native_current_m_registry();
    let node = registry
        .manifest()
        .nodes
        .iter()
        .filter(|n| n.root_position.is_some())
        .max_by_key(|n| n.depth)
        .unwrap();
    let direct = resolve_manifestation(registry, request(&node.source_ref, MFace::Bimba)).unwrap();
    let prime =
        resolve_manifestation(registry, request(&node.source_ref, MFace::Pratibimba)).unwrap();
    assert_eq!(direct.atlas.coordinate_id, prime.atlas.coordinate_id);
    assert_ne!(
        direct.atlas.binding_content_revision,
        prime.atlas.binding_content_revision
    );
    assert!(direct.atlas.rooted_world.ancestry.len() > 4);
    assert_eq!(
        native_procedural_bindings(&prime).unwrap()[0]["locus"]["ref"],
        prime.atlas.rooted_world.conjugate.canonical_ref
    );
    let inventory = manifestation_inventory(registry);
    assert_eq!(
        inventory.len(),
        registry
            .manifest()
            .nodes
            .iter()
            .filter(|n| n.root_position.is_some())
            .count()
    );
    assert!(
        inventory.values().any(Vec::is_empty),
        "source-only branches remain addressable"
    );
}

#[test]
fn wrong_principal_prime_profile_registry_or_unavailable_source_is_refused() {
    let registry = m_tree::native_current_m_registry();
    let base = request("#3", MFace::Bimba);
    let mut wrong = base.clone();
    wrong.principal.subject_ref = "ql:m-coordinate:bimba:M2".into();
    assert!(
        resolve_manifestation(registry, wrong)
            .unwrap_err()
            .contains("locus")
    );
    let mut wrong = base.clone();
    wrong.face = MFace::Pratibimba;
    assert!(
        resolve_manifestation(registry, wrong)
            .unwrap_err()
            .contains("face")
    );
    let mut wrong = base.clone();
    wrong.expected_profile_revision = "stale".into();
    assert!(
        resolve_manifestation(registry, wrong)
            .unwrap_err()
            .contains("profile")
    );
    let mut wrong = base.clone();
    wrong.expected_registry_revision = "stale".into();
    assert!(
        resolve_manifestation(registry, wrong)
            .unwrap_err()
            .contains("registry")
    );
    for mutation in 0..3 {
        let mut wrong = base.clone();
        match mutation {
            0 => wrong.principal.sources[0].revision = "stale".into(),
            1 => wrong.principal.sources[0].reference = "foreign:source".into(),
            _ => wrong.principal.native_owner = "foreign:owner".into(),
        }
        assert!(
            resolve_manifestation(registry, wrong)
                .unwrap_err()
                .contains("source")
        );
    }
    let mut wrong = base;
    wrong.principal.sources[0].availability = ReadingAvailability::Withheld;
    assert!(
        resolve_manifestation(registry, wrong)
            .unwrap_err()
            .contains("withheld")
    );
}

#[test]
fn native_relation_kind_endpoints_source_and_contributors_are_detected() {
    let registry = m_tree::native_current_m_registry();
    let actual = registry
        .manifest()
        .relations
        .iter()
        .find(|r| r.from_ref.is_some() && r.to_ref.is_some())
        .unwrap();
    let required = RequiredRelation {
        relation_id: actual.id,
        source_kind: actual.source_kind.clone(),
        from_ref: actual.from_ref.clone().unwrap(),
        to_ref: actual.to_ref.clone().unwrap(),
        registry_revision: registry.manifest().registry_revision.clone(),
    };
    validate_relation(registry, &required).unwrap();
    let mut wrong = required.clone();
    wrong.to_ref.push_str("-wrong");
    assert!(validate_relation(registry, &wrong).is_err());
    let mut input = request("#3", MFace::Bimba);
    input.required_relations.push(required.clone());
    input.contributors.push(Contributor {
        subject: subject(&required.to_ref),
        role: "native-related-subject".into(),
        relation_ref: Some(actual.relation_ref.clone()),
        standing: ChoiceStanding::SourceDerived,
    });
    let plan = resolve_manifestation(registry, input.clone()).unwrap();
    let binding = &native_procedural_bindings(&plan).unwrap()[0];
    assert_eq!(binding["contributors"][0]["subject_ref"], required.to_ref);
    assert!(binding["contributors"][0].get("subject").is_none());
    input.contributors[0].subject.subject_ref = "native:unrelated-subject".into();
    assert!(resolve_manifestation(registry, input).is_err());
}

fn place(coordinate: &str, scene: &str) -> PlaceOccurrence {
    let registry = m_tree::native_current_m_registry();
    let atlas =
        coordinate_expression::resolve_coordinate_expression(registry, coordinate, MFace::Bimba)
            .unwrap();
    PlaceOccurrence {
        occurrence_ref: format!("occurrence:{scene}"),
        expression_ref: "expression:acceptance".into(),
        scene_ref: format!("expression:acceptance:scene:{scene}"),
        canonical_locus: atlas.rooted_world.direct.canonical_ref,
        registry_revision: registry.manifest().registry_revision.clone(),
        profile_revision: atlas.binding_content_revision,
        occasion_ref: "occasion:same".into(),
        principal_subject_ref: "native:continuing-subject".into(),
        retained_entity_refs: vec!["expression:acceptance:entity:retained".into()],
        runtime_instance_ref: "native:runtime:same".into(),
        native_cursor: 187,
        background_policy: ContinuityPolicy::Continue,
    }
}

#[test]
fn atlas_change_and_reentry_preserve_runtime_occurence_subjects_and_native_cursor() {
    let registry = m_tree::native_current_m_registry();
    let a = place("#3", "canonical");
    let mut b = place("#4.4.4.4", "generated");
    b.native_cursor = 220;
    b.background_policy = ContinuityPolicy::Hold;
    let transition =
        prepare_atlas_transition(registry, &a, &b, MFace::Bimba, &b.canonical_locus).unwrap();
    assert_eq!(transition.runtime_instance_ref, a.runtime_instance_ref);
    assert_eq!(transition.retained_entity_refs, a.retained_entity_refs);
    assert_eq!(
        transition.native_changes,
        vec![json!({"change":"focus","scene_ref":b.scene_ref,"entity_ref":null})]
    );
    assert_eq!(transition.destination_cursor, 220);
    let return_path =
        prepare_atlas_transition(registry, &b, &a, MFace::Bimba, &a.canonical_locus).unwrap();
    assert_eq!(return_path.destination_occurrence_ref, a.occurrence_ref);
    assert_eq!(return_path.destination_cursor, 187);
    b.runtime_instance_ref = "native:duplicate-runtime".into();
    assert!(prepare_atlas_transition(registry, &a, &b, MFace::Bimba, &b.canonical_locus).is_err());
    b.runtime_instance_ref = a.runtime_instance_ref.clone();
    b.occasion_ref = "occasion:wrong".into();
    assert!(prepare_atlas_transition(registry, &a, &b, MFace::Bimba, &b.canonical_locus).is_err());
    b.occasion_ref = a.occasion_ref.clone();
    let mut wrong = b.clone();
    wrong.scene_ref = "expression:foreign:scene:wrong".into();
    assert!(
        prepare_atlas_transition(registry, &a, &wrong, MFace::Bimba, &wrong.canonical_locus)
            .unwrap_err()
            .contains("different native Expression")
    );
    let mut wrong = b;
    wrong.retained_entity_refs = vec!["expression:foreign:entity:wrong".into()];
    assert!(
        prepare_atlas_transition(registry, &a, &wrong, MFace::Bimba, &wrong.canonical_locus)
            .unwrap_err()
            .contains("different native Expression")
    );
}

#[test]
fn actual_native_m3_command_projects_form_and_clock_without_label_selection() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut state =
        m3_state::M3State::new(serde_json::from_value(fixture["request"].clone()).unwrap())
            .unwrap();
    let subject = state.snapshot()["subject_ref"].as_str().unwrap().to_owned();
    let initial = m3_material_binding(&state, &subject, state.generation()).unwrap();
    let command: m3_state::M3Command =
        serde_json::from_value(fixture["commands"][0].clone()).unwrap();
    let receipt = state.apply(command).unwrap();
    let after = m3_material_binding(&state, &subject, state.generation()).unwrap();
    assert_eq!(after["form"], receipt.after["form"]);
    assert_eq!(after["clock"], receipt.after["clock"]);
    assert_ne!(initial["form"], after["form"]);
    assert_eq!(after["aperture"]["total_lenses"], 18);
    assert!(m3_material_binding(&state, "wrong:subject", state.generation()).is_err());
    assert!(m3_material_binding(&state, &subject, state.generation() - 1).is_err());
}

#[test]
fn unsafe_material_and_duplicate_output_identity_are_rejected() {
    let registry = m_tree::native_current_m_registry();
    let mut input = request("#3", MFace::Bimba);
    input.manifestations[0].material = json!({"script":"run arbitrary scene code"});
    assert!(resolve_manifestation(registry, input).is_err());
    let mut input = request("#3", MFace::Bimba);
    input.manifestations.push(input.manifestations[0].clone());
    assert!(resolve_manifestation(registry, input).is_err());
}
