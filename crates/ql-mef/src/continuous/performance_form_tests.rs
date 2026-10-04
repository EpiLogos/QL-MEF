//! Real native producer tests. No fabricated reply provides a positive.
use super::super::native_source_support;
use super::*;

fn form(current: &CoupledBasis) -> AuthoredNativePhysicalEdit {
    let original = current.input.m3_commands.last().unwrap();
    AuthoredNativePhysicalEdit::Form {
        actor_ref: "native:source-form/author".into(),
        cause_ref: "native:source-form/change-line-and-pose".into(),
        occurrence_unix_ms: original.occurrence_unix_ms + 1,
        receipt_unix_ms: original.receipt_unix_ms + 1,
        operations: vec![
            M3Operation::ChangeLine { line: 0 },
            M3Operation::SetPose { pose: 2 },
        ],
    }
}
#[test]
fn actual_form_command_prepares_a_new_twelve_node_body_with_original_m1_m2_mef() {
    let (current, config) = native_source_support::config(false);
    let owner =
        PerformanceOwner::prepare(&current, "expression:source-form/owner", config).unwrap();
    let (after, configuration, receipt) =
        prepare_source(&owner, &current, &form(&current)).unwrap();
    assert_eq!(receipt.as_ref().unwrap().status, "applied");
    assert_eq!(receipt.as_ref().unwrap().before, current.m3);
    assert_eq!(receipt.as_ref().unwrap().after, after.m3);
    assert_eq!(
        serde_json::to_value(&after.input.m1).unwrap(),
        serde_json::to_value(&current.input.m1).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&after.input.m2).unwrap(),
        serde_json::to_value(&current.input.m2).unwrap()
    );
    assert_eq!(after.input.source_receipts, current.input.source_receipts);
    assert_eq!(
        after.input.m3_commands.len(),
        current.input.m3_commands.len() + 1
    );
    validate_form_lineage(&current, &after).unwrap();
    let prepared =
        PerformanceOwner::prepare(&after, "expression:source-form/owner", configuration).unwrap();
    prepared
        .binding()
        .validate_source_form_consumer(&replay(&after).unwrap())
        .unwrap();
    assert_eq!(
        prepared
            .binding()
            .physical_body()
            .request()
            .geometry
            .nodes
            .len(),
        12
    );
    assert_eq!(
        owner
            .binding()
            .physical_body()
            .request()
            .geometry
            .nodes
            .len(),
        12
    );
    assert_ne!(
        prepared.binding().physical_body().request().geometry,
        owner.binding().physical_body().request().geometry
    );
    assert_eq!(
        prepared.config.controls.body_revision,
        owner.config.controls.body_revision + 1
    );
    assert_eq!(
        prepared.config.controls.state_ref,
        owner.config.controls.state_ref
    );
    assert_eq!(
        prepared.config.controls.sample_rate,
        owner.config.controls.sample_rate
    );
    assert_eq!(prepared.config.physical_face, owner.config.physical_face);
    assert_eq!(
        owner.source_assets()["original_native_input"],
        serde_json::to_value(&current.input).unwrap()
    );
    // Pure preparation cannot publish a source, native pulse or new owner.
    assert!(owner.reading().is_none());
    owner.validate_current(&current).unwrap();
    assert!(owner.validate_current(&after).is_err());
}
#[test]
fn actual_material_edit_changes_physics_without_changing_source_form_or_tuning_policy() {
    let (current, config) = native_source_support::config(true);
    let owner =
        PerformanceOwner::prepare(&current, "expression:source-material/owner", config).unwrap();
    let mut material = owner.config.controls.material.clone();
    material.young_modulus_pa *= 4.;
    let edit = AuthoredNativePhysicalEdit::Material {
        cause_ref: "native:source-material/stiffness".into(),
        material,
    };
    let (after, configuration, receipt) = prepare_source(&owner, &current, &edit).unwrap();
    assert!(receipt.is_none());
    assert_eq!(
        serde_json::to_value(&after).unwrap(),
        serde_json::to_value(&current).unwrap()
    );
    let prepared =
        PerformanceOwner::prepare(&after, "expression:source-material/owner", configuration)
            .unwrap();
    assert_eq!(
        prepared.binding().physical_body().request().geometry,
        owner.binding().physical_body().request().geometry
    );
    assert_eq!(
        prepared.binding().physical_body().source_coordinate(),
        owner.binding().physical_body().source_coordinate()
    );
    assert_eq!(prepared.binding().notes(), owner.binding().notes());
    let admitted = prepare_material_update(
        owner.binding().physical_body(),
        &replay(&after).unwrap(),
        owner.config.controls.body_revision,
        prepared.config.controls.material.clone(),
        prepared.config.controls.body_revision,
        prepared.config.controls.preparation_ref.clone(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(admitted).unwrap(),
        serde_json::to_value(prepared.binding().physical_body()).unwrap()
    );
    assert_eq!(
        prepared
            .binding()
            .physical_body()
            .request()
            .material
            .young_modulus_pa,
        4. * owner
            .binding()
            .physical_body()
            .request()
            .material
            .young_modulus_pa
    );
}
#[test]
fn lost_descendant_and_disconnected_m1_m2_or_source_cannot_be_form_lineage() {
    let (current, config) = native_source_support::config(false);
    let owner =
        PerformanceOwner::prepare(&current, "expression:source-form/owner", config).unwrap();
    let (after, _, _) = prepare_source(&owner, &current, &form(&current)).unwrap();
    let mut lost = after.input.clone();
    lost.m3_commands.remove(0);
    assert!(
        lost.compose().is_err()
            || validate_form_lineage(&current, &lost.compose().unwrap()).is_err()
    );
    let mut m1 = after.clone();
    m1.input.m1.tick12 = (m1.input.m1.tick12 + 1) % 12;
    assert!(validate_form_lineage(&current, &m1).is_err());
    let mut m2 = after.clone();
    m2.input.m2.at_unix_ms += 1;
    assert!(validate_form_lineage(&current, &m2).is_err());
    let mut source = after.clone();
    source
        .input
        .source_receipts
        .push(json!({"unrelated":"source"}));
    assert!(validate_form_lineage(&current, &source).is_err());
    let mut rewritten = after.clone();
    rewritten.input.m3_commands[0].cause_ref = "native:forged-original".into();
    assert!(validate_form_lineage(&current, &rewritten).is_err());
    owner.validate_current(&current).unwrap();
}
#[test]
fn authored_edit_has_no_body_face_clock_source_or_queue_override() {
    let edit = json!({"kind":"material","cause_ref":"native:source-material/control",
        "material":{"provenance":{"reference":"native:material","revision":"1","source_ref":crate::source_form_body::SOURCE_GEOMETRY_BASIS,"standing":"agent-proposed"},
        "young_modulus_pa":1e6,"density_kg_per_m3":1000.,"damping_alpha_per_second":0.4,"damping_beta_seconds":0.}});
    assert!(serde_json::from_value::<AuthoredNativePhysicalEdit>(edit.clone()).is_ok());
    for name in [
        "physical_face",
        "native_sample",
        "body_revision",
        "expected_generation",
        "native_basis",
        "permission",
        "sequence",
    ] {
        let mut supplied = edit.clone();
        supplied[name] = json!(0);
        assert!(
            serde_json::from_value::<AuthoredNativePhysicalEdit>(supplied).is_err(),
            "{name}"
        );
    }
}

#[test]
fn genuine_world_constructor_owns_original_receipts_after_actual_form_descendant() {
    use crate::musical_performance_return::{ReturnContext, ReturnReference};
    let sky: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v1.json"
    ))
    .unwrap();
    let request: crate::scene::WorldRequest = serde_json::from_value(json!({
        "schema":crate::scene::WORLD_REQUEST,"instance_ref":"expression:source-form/world",
        "event_ref":sky["snapshot_ref"],"subject_ref":"person:source-form/current",
        "texture":[64,64],"units_per_metre":1.,"sky":sky,"start":{"tick12":3,"cycle":7,"aperture":9}
    }))
    .unwrap();
    let produced = crate::scene::world(request.clone()).unwrap();
    let current: super::super::super::coupled::CoupledInput =
        serde_json::from_value(produced["event"].clone()).unwrap();
    let current = current.compose().unwrap();
    let reference = |reference: &str| ReturnReference {
        reference: reference.into(),
        revision: "1".into(),
    };
    let source = NativePerformanceReceivingSource::world_source(
        request,
        ReturnContext {
            kind: "world".into(),
            context: reference("native:source-form/world"),
            receiver: reference("native:source-form/receiver"),
            source_occasion: None,
            protected_state: None,
            consent: None,
            private: false,
            required_assets: vec![],
        },
    )
    .unwrap();
    let (_, mut config) = native_source_support::config(false);
    config.controls.expected_m3_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let mut owner =
        PerformanceOwner::prepare(&current, "expression:source-form/world", config).unwrap();
    let before_receiving = source.admit_current(&mut owner, &current, 0).unwrap();
    let authored = AuthoredNativePhysicalEdit::Form {
        actor_ref: "native:source-form/author".into(),
        cause_ref: "native:source-form/actual-world-descendant".into(),
        occurrence_unix_ms: current.input.m3.occurrence_unix_ms + 1,
        receipt_unix_ms: current.input.m3.receipt_unix_ms + 1,
        operations: vec![
            M3Operation::ChangeLine { line: 0 },
            M3Operation::SetPose { pose: 2 },
        ],
    };
    let (after, config, receipt) = prepare_source(&owner, &current, &authored).unwrap();
    assert_eq!(receipt.unwrap().status, "applied");
    let mut next =
        PerformanceOwner::prepare(&after, "expression:source-form/world", config).unwrap();
    // Only the retained owner transfers its private actual constructor basis.
    next.source_origin = owner.source_origin.clone();
    let after_receiving = source.admit_current(&mut next, &after, 512).unwrap();
    after_receiving
        .validate_current(&source, &next, &after, 512)
        .unwrap();
    assert_eq!(
        after_receiving.source_inputs(),
        before_receiving.source_inputs()
    );
    assert_eq!(after.input.source_receipts, current.input.source_receipts);
    assert_eq!(
        after_receiving.admission().snapshot().unwrap()["operation"]["native_sample"],
        "512"
    );
    assert_eq!(
        after_receiving.admission().snapshot().unwrap()["native_preparation"]["determination"]["body_revision"],
        "2"
    );
    after_receiving
        .context()
        .validate_binding(&current, next.binding())
        .unwrap();
    assert!(
        after_receiving
            .context()
            .validate_binding(&after, next.binding())
            .is_err()
    );
    next.source_origin = after.clone();
    assert!(source.prepare_current(&next, &after, 512).is_err());
    owner.validate_current(&current).unwrap();
    assert!(owner.reading().is_none());
}
