use std::collections::BTreeMap;

use ql_core::{QlFormRef, RelationFamily};
use ql_mef::{
    ClientRef, InputRefRevision, LensId, LensRef, QlProvenance, QlProviderRef, QlReading,
    ResultClass, lens_definition,
};
use ql_semantic::{
    InputLimits, LocateRequest, LocateResult, Operation, ProviderCapabilities, ProviderClass,
    ProviderError, ProviderHealth, QlProvider, RefractRequest, RelateRequest, SemanticDisclosure,
    SemanticReading, SemanticRelationReading, SemanticStatus, SemanticSynthesis, SynthesiseRequest,
};
use ql_wiki::{
    FieldCoordinate, LensSelection, ProviderMode, RefractionStatus, RevisionValue,
    WIKI_REFRACTION_CONTRACT, WikiProvenanceRef, WikiRefractionEngine, WikiRefractionError,
    WikiRefractionRequest, WikiRefractionTarget, WikiStructuralField, WikiSubjectSnapshot,
    WikiTargetKind, WikiTargetRelation,
};
use serde_json::{Map, json};

#[derive(Clone)]
struct LensProvider {
    capabilities: ProviderCapabilities,
}

impl LensProvider {
    fn full(health: ProviderHealth) -> Self {
        Self {
            capabilities: ProviderCapabilities {
                provider: QlProviderRef::new("ql-mef:provider:registry-disclosure", "1.0.0")
                    .unwrap(),
                health,
                classes: vec![ProviderClass::SemanticRefraction],
                supported_forms: vec![QlFormRef::SIXFOLD_V1],
                supported_lenses: LensId::ALL.into_iter().map(LensRef::canonical).collect(),
                operations: vec![Operation::Capabilities, Operation::Refract],
                extension_namespaces: vec!["ql-mef/wiki-refraction/v1".into()],
                deterministic_operations: vec![],
                input_limits: InputLimits {
                    max_relation_subjects: 16,
                    max_synthesis_readings: 12,
                },
                output_schema_versions: vec![WIKI_REFRACTION_CONTRACT.into()],
            },
        }
    }

    fn no_refract() -> Self {
        let mut provider = Self::full(ProviderHealth::available());
        provider.capabilities.operations = vec![Operation::Capabilities];
        provider
    }
}

impl QlProvider for LensProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn locate(&self, _request: LocateRequest) -> Result<LocateResult, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Locate))
    }

    fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ProviderError> {
        if !self.capabilities.supports(Operation::Refract) {
            return Err(ProviderError::UnsupportedOperation(Operation::Refract));
        }
        let lens = lens_definition(request.lens.lens());
        let subject = request.input.target.subject.clone();
        let revision = request.input.revision.clone();
        let mut provenance = QlProvenance::new(
            self.capabilities.provider.clone(),
            "refract",
            vec![InputRefRevision::new(subject.clone(), revision)],
            ResultClass::SemanticStochastic,
        );
        provenance.model = Some("registry-disclosure-fixture".into());
        let mut reading = QlReading::new(
            ClientRef::new(format!(
                "ql-mef:reading:{}:{}",
                subject,
                request.lens.lens().code()
            ))
            .unwrap(),
            request.input.target,
            Some(request.lens),
            SemanticDisclosure {
                text: format!("{} refractive disclosure", lens.name()),
                status: SemanticStatus::Complete,
                confidence_per_mille: Some(900),
            },
            provenance,
        );
        reading.evidence_refs.push(
            ClientRef::new(format!(
                "ql-mef:wiki:node:mef-{}",
                request.lens.lens().code()
            ))
            .unwrap(),
        );
        Ok(reading)
    }

    fn relate(&self, _request: RelateRequest) -> Result<SemanticRelationReading, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Relate))
    }

    fn synthesise(&self, _request: SynthesiseRequest) -> Result<SemanticSynthesis, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Synthesise))
    }
}

fn subject(reference: &str, revision: u64, position: u8, face: &str) -> WikiSubjectSnapshot {
    WikiSubjectSnapshot {
        subject_ref: reference.into(),
        revision: Some(RevisionValue::Integer(revision)),
        position: Some(position),
        face: Some(face.into()),
        extensions: BTreeMap::new(),
    }
}

fn d3_target(family: RelationFamily, pair_index: u8) -> WikiRefractionTarget {
    let field = family.pair(pair_index).unwrap().d3();
    let subjects = field
        .coordinates
        .iter()
        .map(|coordinate| {
            subject(
                &format!(
                    "example:node:{}:{}",
                    coordinate.position.value(),
                    coordinate.face.as_str()
                ),
                7,
                coordinate.position.value(),
                coordinate.face.as_str(),
            )
        })
        .collect::<Vec<_>>();
    let coordinates = field
        .coordinates
        .iter()
        .map(|coordinate| FieldCoordinate {
            position: coordinate.position.value(),
            face: coordinate.face.as_str().into(),
        })
        .collect();
    WikiRefractionTarget {
        kind: WikiTargetKind::D3,
        target_ref: format!("example:frame:{}:{}", family.as_str(), pair_index),
        target_frame_ref: Some(format!("example:frame:{}:{}", family.as_str(), pair_index)),
        target_revision: Some(RevisionValue::Integer(12)),
        target_snapshot_hash: format!("sha256:fixture-{}-{pair_index}", family.as_str()),
        provenance: vec![WikiProvenanceRef {
            source_ref: "example:source:constellation".into(),
            source_revision: Some(RevisionValue::String("sha256:source".into())),
            extensions: BTreeMap::new(),
        }],
        subjects,
        relations: vec![WikiTargetRelation {
            from_ref: "example:node:2:direct".into(),
            to_ref: "example:node:3:direct".into(),
            relation: format!("ql:{}:{}:field", family.as_str(), pair_index),
            origin: Some("QL-derived".into()),
            origin_ref: Some(field.operator_ref()),
            provenance: vec![],
            extensions: BTreeMap::new(),
        }],
        structural_field: Some(WikiStructuralField {
            operator_ref: field.operator_ref(),
            family: Some(family.as_str().into()),
            pair_index: Some(pair_index),
            degree: "D3".into(),
            expansion_side: None,
            coordinates,
            provenance: vec![],
        }),
        material: Map::from_iter([("summary".into(), json!("client-owned field material"))]),
        extensions: Map::new(),
    }
}

fn request(
    target: WikiRefractionTarget,
    mode: ProviderMode,
    lenses: &[&str],
) -> WikiRefractionRequest {
    WikiRefractionRequest {
        contract: WIKI_REFRACTION_CONTRACT.into(),
        mode,
        target,
        lenses: lenses
            .iter()
            .map(|lens| LensSelection {
                lens_ref: (*lens).into(),
                sublens_ref: None,
            })
            .collect(),
        context: Map::from_iter([("purpose".into(), json!("comparative reading"))]),
    }
}

#[test]
fn multi_node_d3_frame_yields_separate_mef_readings_without_identity_mutation() {
    let target = d3_target(RelationFamily::A, 1);
    let before = target.clone();
    let provider = LensProvider::full(ProviderHealth::available());
    let response = WikiRefractionEngine::new(Some(&provider))
        .refract(&request(
            target.clone(),
            ProviderMode::Required,
            &["mef:lens:L1@1", "mef:lens:L2'@1", "mef:lens:L5@1"],
        ))
        .unwrap();

    assert_eq!(response.status, RefractionStatus::Complete);
    assert_eq!(response.target_ref, target.target_ref);
    assert_eq!(response.target_snapshot_hash, target.target_snapshot_hash);
    assert_eq!(response.readings.len(), 3);
    assert_eq!(target, before);
    assert_eq!(response.readings[0].target_revision, target.target_revision);
    assert!(
        response
            .readings
            .iter()
            .all(|reading| reading.reading_type == "MEF-derived")
    );
    assert!(response.readings.iter().all(|reading| {
        reading.harmonic_field_ref.as_deref()
            == target
                .structural_field
                .as_ref()
                .map(|field| field.operator_ref.as_str())
    }));
    assert!(
        response
            .readings
            .iter()
            .all(|reading| reading.relation_candidates.is_empty())
    );
    assert!(
        response
            .readings
            .iter()
            .all(|reading| { reading.derived_subgraph.relations[0].origin == "QL-derived" })
    );
}

#[test]
fn less_technology_mapped_lenses_are_first_class_not_tags() {
    let provider = LensProvider::full(ProviderHealth::available());
    let response = WikiRefractionEngine::new(Some(&provider))
        .refract(&request(
            d3_target(RelationFamily::B, 0),
            ProviderMode::Required,
            &["mef:lens:L0'@1", "mef:lens:L3@1", "mef:lens:L5'@1"],
        ))
        .unwrap();
    let disclosures = response
        .readings
        .iter()
        .map(|reading| reading.disclosure.as_str())
        .collect::<Vec<_>>();
    assert!(
        disclosures
            .iter()
            .any(|text| text.contains("Archetypal-Numerical"))
    );
    assert!(disclosures.iter().any(|text| text.contains("Processual")));
    assert!(disclosures.iter().any(|text| text.contains("Divine Logos")));
}

#[test]
fn all_nine_d3_square_identities_validate_and_same_vertices_different_family_stay_distinct() {
    for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
        for pair_index in 0..3 {
            d3_target(family, pair_index).validate().unwrap();
        }
    }
    let a = d3_target(RelationFamily::A, 1);
    let c = d3_target(RelationFamily::C, 2);
    let a_field = a.structural_field.as_ref().unwrap();
    let c_field = c.structural_field.as_ref().unwrap();
    let mut a_coordinates = a_field.coordinates.clone();
    let mut c_coordinates = c_field.coordinates.clone();
    a_coordinates.sort_by_key(|coordinate| (coordinate.position, coordinate.face.clone()));
    c_coordinates.sort_by_key(|coordinate| (coordinate.position, coordinate.face.clone()));
    assert_eq!(a_coordinates, c_coordinates);
    assert_ne!(a_field.operator_ref, c_field.operator_ref);
}

#[test]
fn node_frame_pair_d1_d2_d3_and_space_targets_are_valid_target_surfaces() {
    let provider = LensProvider::full(ProviderHealth::available());
    let mut targets = vec![];

    for kind in [
        WikiTargetKind::NodeLocal,
        WikiTargetKind::Frame,
        WikiTargetKind::Space,
    ] {
        let mut target = d3_target(RelationFamily::A, 0);
        target.kind = kind;
        target.structural_field = None;
        target.target_ref = format!("example:{kind:?}:target");
        targets.push(target);
    }

    let pair = RelationFamily::A.pair(0).unwrap();
    let pair_coords = [pair.left, pair.right]
        .into_iter()
        .map(|position| FieldCoordinate {
            position: position.value(),
            face: "direct".into(),
        })
        .collect();
    let mut pair_target = d3_target(RelationFamily::A, 0);
    pair_target.kind = WikiTargetKind::Pair;
    pair_target.structural_field = Some(WikiStructuralField {
        operator_ref: pair.operator_ref(),
        family: Some("A".into()),
        pair_index: Some(0),
        degree: "pair".into(),
        expansion_side: None,
        coordinates: pair_coords,
        provenance: vec![],
    });
    targets.push(pair_target);

    let mut d1_target = d3_target(RelationFamily::A, 0);
    d1_target.kind = WikiTargetKind::D1;
    d1_target.structural_field = Some(WikiStructuralField {
        operator_ref: "ql:structural:2.0.0:conjugation:D1:position-0".into(),
        family: None,
        pair_index: None,
        degree: "D1".into(),
        expansion_side: None,
        coordinates: vec![
            FieldCoordinate {
                position: 0,
                face: "direct".into(),
            },
            FieldCoordinate {
                position: 0,
                face: "conjugate".into(),
            },
        ],
        provenance: vec![],
    });
    targets.push(d1_target);

    for (kind, side) in [(WikiTargetKind::D2, "left"), (WikiTargetKind::D2, "right")] {
        let field = pair.d2(if side == "left" {
            ql_core::ExpansionSide::Left
        } else {
            ql_core::ExpansionSide::Right
        });
        let mut target = d3_target(RelationFamily::A, 0);
        target.kind = kind;
        target.structural_field = Some(WikiStructuralField {
            operator_ref: field.operator_ref(),
            family: Some("A".into()),
            pair_index: Some(0),
            degree: "D2".into(),
            expansion_side: Some(side.into()),
            coordinates: field
                .coordinates
                .iter()
                .map(|coordinate| FieldCoordinate {
                    position: coordinate.position.value(),
                    face: coordinate.face.as_str().into(),
                })
                .collect(),
            provenance: vec![],
        });
        targets.push(target);
    }

    targets.push(d3_target(RelationFamily::C, 2));

    for target in targets {
        let response = WikiRefractionEngine::new(Some(&provider))
            .refract(&request(target, ProviderMode::Required, &["mef:lens:L4@1"]))
            .unwrap();
        assert_eq!(response.readings.len(), 1);
    }
}

#[test]
fn disabled_optional_and_required_modes_have_distinct_failure_semantics() {
    let target = d3_target(RelationFamily::A, 1);
    let disabled = WikiRefractionEngine::new(None)
        .refract(&request(target.clone(), ProviderMode::Disabled, &[]))
        .unwrap();
    assert_eq!(disabled.status, RefractionStatus::Disabled);
    assert!(disabled.readings.is_empty());

    let optional = WikiRefractionEngine::new(None)
        .refract(&request(
            target.clone(),
            ProviderMode::Optional,
            &["mef:lens:L1@1"],
        ))
        .unwrap();
    assert_eq!(optional.status, RefractionStatus::Unavailable);
    assert!(optional.readings.is_empty());

    let required = WikiRefractionEngine::new(None).refract(&request(
        target.clone(),
        ProviderMode::Required,
        &["mef:lens:L1@1"],
    ));
    assert!(matches!(
        required,
        Err(WikiRefractionError::ProviderRequired(_))
    ));

    let no_refract = LensProvider::no_refract();
    let optional = WikiRefractionEngine::new(Some(&no_refract))
        .refract(&request(
            target.clone(),
            ProviderMode::Optional,
            &["mef:lens:L1@1"],
        ))
        .unwrap();
    assert_eq!(optional.status, RefractionStatus::Unavailable);

    let required = WikiRefractionEngine::new(Some(&no_refract)).refract(&request(
        target,
        ProviderMode::Required,
        &["mef:lens:L1@1"],
    ));
    assert!(matches!(
        required,
        Err(WikiRefractionError::ProviderRequired(_))
    ));
}

#[test]
fn invalid_structural_field_is_validation_error_even_when_provider_is_optional_or_absent() {
    let mut target = d3_target(RelationFamily::A, 1);
    target.structural_field.as_mut().unwrap().operator_ref =
        "ql:structural:2.0.0:field:C:2:D3".into();
    let result = WikiRefractionEngine::new(None).refract(&request(
        target,
        ProviderMode::Optional,
        &["mef:lens:L1@1"],
    ));
    assert!(matches!(
        result,
        Err(WikiRefractionError::InvalidStructuralField(_))
    ));
}

#[test]
fn mismatched_sublens_is_validation_error_before_provider_invocation() {
    let mut request = request(
        d3_target(RelationFamily::A, 1),
        ProviderMode::Optional,
        &["mef:lens:L1@1"],
    );
    request.lenses[0].sublens_ref = Some("mef:sublens:L4.2@1".into());
    assert!(matches!(
        WikiRefractionEngine::new(None).refract(&request),
        Err(WikiRefractionError::InvalidLens(_))
    ));
}

#[test]
fn degraded_provider_can_serve_advertised_refraction_and_status_remains_inspectable() {
    let provider = LensProvider::full(ProviderHealth::degraded("semantic model is cold-starting"));
    let response = WikiRefractionEngine::new(Some(&provider))
        .refract(&request(
            d3_target(RelationFamily::B, 1),
            ProviderMode::Optional,
            &["mef:lens:L2@1"],
        ))
        .unwrap();
    assert_eq!(response.status, RefractionStatus::Degraded);
    assert_eq!(response.readings.len(), 1);
    assert_eq!(response.readings[0].provider.health, "degraded");
}

#[test]
fn language_neutral_request_fixture_round_trips_without_project_ontology() {
    let raw = include_str!("../../../fixtures/qw2/wiki-refraction-request.json");
    let request: WikiRefractionRequest = serde_json::from_str(raw).unwrap();
    request.validate().unwrap();
    assert_eq!(request.target.target_ref, "example:wiki:frame:decision-17");
    assert_eq!(
        request
            .target
            .structural_field
            .as_ref()
            .unwrap()
            .family
            .as_deref(),
        Some("A")
    );
    assert!(!raw.to_lowercase().contains("glade"));
    assert!(!raw.to_lowercase().contains("aikit"));
    assert!(!raw.contains("Bimba Graph"));
}
