use std::collections::BTreeMap;

use ql_core::{
    ConstellationGrain, QlFace, QlPosition, QlShape, QlShapeCompression, ShapeBinding,
    StructuralParticipation,
};
use ql_wiki::{
    LensSelection, ProviderMode, RegistryDisclosureProvider, ShapeAwareWikiRefractionEngine,
    WIKI_REFRACTION_CONTRACT, WIKI_SHAPE_BINDING_EXTENSION, WikiProvenanceRef,
    WikiRefractionEngine, WikiRefractionRequest, WikiRefractionTarget, WikiShapeBindingProvenance,
    WikiShapeBindingView, WikiShapeMember, WikiTargetKind, attach_shape_binding,
    shape_binding_from_target,
};
use serde_json::Map;

fn target(reference: &str) -> WikiRefractionTarget {
    WikiRefractionTarget {
        kind: WikiTargetKind::Frame,
        target_ref: reference.into(),
        target_frame_ref: None,
        target_revision: None,
        target_snapshot_hash: "sha256:shape-binding-test".into(),
        provenance: vec![WikiProvenanceRef {
            source_ref: "example:source".into(),
            source_revision: None,
            extensions: BTreeMap::new(),
        }],
        subjects: vec![],
        relations: vec![],
        structural_field: None,
        material: Map::new(),
        extensions: Map::new(),
    }
}

fn compressed_binding(subject_ref: &str) -> WikiShapeBindingView {
    let members = [
        ("example:a", 1_u8),
        ("example:b", 2_u8),
        ("example:c", 3_u8),
    ];
    let disclosed = QlShape::Constellation(ConstellationGrain::ThreeFold123);
    let compression = QlShapeCompression::to_onefold(disclosed).unwrap();
    WikiShapeBindingView {
        subject_ref: subject_ref.into(),
        shape_ref: compression.presented.shape_ref(),
        whole_ref: "example:whole-anchor".into(),
        basis_refs: members.iter().map(|(reference, _)| (*reference).into()).collect(),
        members: members
            .iter()
            .map(|(reference, position)| WikiShapeMember {
                subject_ref: (*reference).into(),
                position: *position,
                face: "direct".into(),
            })
            .collect(),
        derivation_ref: Some(compression.derivation_ref()),
        operator_ref: Some(compression.operator_ref.into()),
        return_refs: vec!["example:return".into()],
        provenance: WikiShapeBindingProvenance {
            caller_ref: "ql-wiki:test".into(),
            source_ref: "example:source".into(),
            standing_ref: "AUTHORED-ARCHITECTURE".into(),
        },
    }
}

fn request(target: WikiRefractionTarget) -> WikiRefractionRequest {
    WikiRefractionRequest {
        contract: WIKI_REFRACTION_CONTRACT.into(),
        mode: ProviderMode::Required,
        target,
        lenses: vec![LensSelection {
            lens_ref: "mef:lens:L1@1".into(),
            sublens_ref: None,
        }],
        context: Map::new(),
    }
}

#[test]
fn core_shape_binding_view_round_trips_without_reowning_geometry() {
    let participation = [
        StructuralParticipation::new("example:a", QlPosition::new(1).unwrap(), QlFace::Direct)
            .unwrap(),
        StructuralParticipation::new("example:b", QlPosition::new(2).unwrap(), QlFace::Direct)
            .unwrap(),
        StructuralParticipation::new("example:c", QlPosition::new(3).unwrap(), QlFace::Direct)
            .unwrap(),
    ];
    let disclosed = QlShape::Constellation(ConstellationGrain::ThreeFold123);
    let compression = QlShapeCompression::to_onefold(disclosed).unwrap();
    let core = ShapeBinding::new(
        "example:subject",
        compression.presented.shape_ref(),
        "example:whole-anchor",
        participation
            .iter()
            .map(|member| member.subject_ref.clone())
            .collect(),
        participation.to_vec(),
        vec![],
        Some(compression.derivation_ref()),
        Some(compression.operator_ref.into()),
        vec!["example:return".into()],
        ql_core::CallerProvenance::new(
            "ql-wiki:test",
            "example:source",
            "AUTHORED-ARCHITECTURE",
        )
        .unwrap(),
    )
    .unwrap();
    let view = WikiShapeBindingView::from_core(&core);
    let rebuilt = view.to_core().unwrap();
    assert_eq!(rebuilt.subject_ref, core.subject_ref);
    assert_eq!(rebuilt.shape_ref, core.shape_ref);
    assert_eq!(rebuilt.whole_ref, core.whole_ref);
    assert_eq!(rebuilt.basis_refs, core.basis_refs);
    assert_eq!(rebuilt.members, core.members);
    assert_eq!(rebuilt.derivation_ref, core.derivation_ref);
    assert_eq!(rebuilt.operator_ref, core.operator_ref);
    assert_eq!(rebuilt.return_refs, core.return_refs);
    assert_eq!(rebuilt.provenance, core.provenance);
}

#[test]
fn compressed_onefold_is_validated_then_carried_into_every_wiki_reading() {
    let mut target = target("example:subject");
    let binding = compressed_binding(&target.target_ref);
    attach_shape_binding(&mut target, &binding).unwrap();
    assert_eq!(shape_binding_from_target(&target).unwrap(), Some(binding.clone()));

    let provider = RegistryDisclosureProvider::new();
    let response = ShapeAwareWikiRefractionEngine::new(Some(&provider))
        .refract(&request(target))
        .unwrap();
    assert_eq!(response.readings.len(), 1);
    let reading = &response.readings[0];
    assert!(reading.ql_form_refs.contains(&binding.shape_ref));
    assert!(
        reading
            .operator_refs
            .contains(binding.operator_ref.as_ref().unwrap())
    );
    assert_eq!(
        reading.extensions.get(WIKI_SHAPE_BINDING_EXTENSION),
        Some(&serde_json::to_value(&binding).unwrap())
    );
}

#[test]
fn aliases_and_false_compression_derivations_fail_closed() {
    let mut target = target("example:subject");
    let mut binding = compressed_binding(&target.target_ref);
    binding.shape_ref = "ql:shape:1.0.0:constellation:partial-conjugate-7".into();
    assert!(attach_shape_binding(&mut target, &binding).is_err());

    let mut target = target("example:subject");
    let mut binding = compressed_binding(&target.target_ref);
    binding.derivation_ref = Some("ql:shape:1.1.0:compression:3-to-1-recognition:from:bogus".into());
    assert!(attach_shape_binding(&mut target, &binding).is_err());
}

#[test]
fn ordinary_wiki_refraction_is_unchanged_when_no_shape_binding_is_present() {
    let provider = RegistryDisclosureProvider::new();
    let request = request(target("example:ordinary"));
    let ordinary = WikiRefractionEngine::new(Some(&provider))
        .refract(&request)
        .unwrap();
    let shape_aware = ShapeAwareWikiRefractionEngine::new(Some(&provider))
        .refract(&request)
        .unwrap();
    assert_eq!(shape_aware, ordinary);
}
