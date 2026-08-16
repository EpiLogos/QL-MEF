use ql_wiki::{
    CrossWikiTraversalRequest, ExternalObservation, MappingOrigin, MetaBinding,
    MetaKnowledgeProjection, MetaPortal, PortalScope, RecognitionDecision, RecognitionLedger,
    RecognitionState, RefractionStatus, RegistryDisclosureProvider, StaticForeignResolver,
    TargetAvailability, WikiRefractionEngine, WikiRefractionError, WikiRefractionRequest,
    WikiStructuralField, apply_recognised_amendment, parse_okf_wiki,
};
use serde_json::json;

const QL_SPACE: &str = include_str!("../../../fixtures/qw1/meta-wiki/ql-structural-space.md");
const QL_FAMILY_A: &str = include_str!("../../../fixtures/qw1/meta-wiki/relation-family-a.md");
const QL_L1: &str = include_str!("../../../fixtures/qw1/meta-wiki/mef-l1.md");
const QL_L3: &str = include_str!("../../../fixtures/qw4/meta-wiki/mef-l3.md");
const RESEARCH_SPACE: &str = include_str!("../../../fixtures/qw4/research-wiki/research-space.md");
const RESEARCH_LOCAL: &str =
    include_str!("../../../fixtures/qw4/research-wiki/question-local-space.md");
const RESEARCH_QUESTION: &str =
    include_str!("../../../fixtures/qw4/research-wiki/question-node.md");
const RESEARCH_EVIDENCE: &str =
    include_str!("../../../fixtures/qw4/research-wiki/evidence-node.md");
const RESEARCH_FRAME: &str = include_str!("../../../fixtures/qw4/research-wiki/d3-frame.md");
const REFRACTION: &str = include_str!("../../../fixtures/qw4/research-refraction-request.json");
const BINDINGS: &str = include_str!("../../../fixtures/qw4/portal-bindings.json");

fn meta_documents() -> Vec<ql_wiki::OkfWikiDocument> {
    [QL_SPACE, QL_FAMILY_A, QL_L1, QL_L3]
        .into_iter()
        .map(|source| parse_okf_wiki(source).unwrap())
        .collect()
}

fn bindings() -> Vec<MetaBinding> {
    serde_json::from_str(BINDINGS).unwrap()
}

fn both_scopes() -> PortalScope {
    PortalScope::new(
        ["scope:glade".to_owned(), "scope:research".to_owned()],
        true,
    )
}

#[test]
fn research_canvas_is_a_distinct_open_wiki_profile_with_recursive_node_as_whole() {
    let space = parse_okf_wiki(RESEARCH_SPACE).unwrap();
    let local = parse_okf_wiki(RESEARCH_LOCAL).unwrap();
    let question = parse_okf_wiki(RESEARCH_QUESTION).unwrap();
    let evidence = parse_okf_wiki(RESEARCH_EVIDENCE).unwrap();
    let frame = parse_okf_wiki(RESEARCH_FRAME).unwrap();

    for document in [&space, &local, &question, &evidence, &frame] {
        assert_eq!(document.wiki.profile, "okf-wiki/v1");
        assert_eq!(document.ql_mef_profile(), None);
        assert_eq!(document.okf["research_profile"], "research-canvas/wiki/v1");
    }
    assert_eq!(question.wiki.string("type"), Some("Research Question"));
    assert_eq!(
        question.wiki.string("local_space_ref"),
        Some("research:space:question-local")
    );
    assert_eq!(
        local.wiki.string("anchor_ref"),
        Some("research:node:question")
    );
    assert_eq!(
        space.wiki.refs("child_space_refs"),
        vec!["research:space:question-local"]
    );
    assert_eq!(
        frame.wiki.canonical_ref,
        "research:frame:transition-anomaly"
    );
    assert_eq!(frame.wiki.revision, 5);
    assert_eq!(
        evidence.wiki.raw["research_extension"]["observation_count"],
        5
    );
}

#[test]
fn research_frame_consumes_exact_qw0_d3_and_return_contract_without_translation() {
    let frame = parse_okf_wiki(RESEARCH_FRAME).unwrap();
    let structural: WikiStructuralField =
        serde_json::from_value(frame.wiki.raw["research_extension"]["structural_field"].clone())
            .unwrap();
    structural.validate().unwrap();
    assert_eq!(structural.operator_ref, "ql:structural:2.0.0:field:A:1:D3");
    assert_eq!(structural.family.as_deref(), Some("A"));
    assert_eq!(structural.pair_index, Some(1));
    assert_eq!(structural.coordinates.len(), 4);

    let constellations = frame.wiki.raw["constellations"].as_array().unwrap();
    let members = constellations[0]["members"].as_array().unwrap();
    assert_eq!(members.len(), 4);
    assert_eq!(
        constellations[0]["anchor_ref"],
        "research:anchor:transition-whole"
    );
    let return_path = &frame.wiki.raw["research_extension"]["return"];
    assert_eq!(
        return_path["through_anchor_ref"],
        "research:anchor:transition-whole"
    );
    assert_eq!(return_path["target_position"], 0);
    assert_eq!(return_path["ground_kind"], "own");
}

#[test]
fn real_registry_provider_refracts_second_wiki_whole_field_through_multiple_lenses() {
    let request: WikiRefractionRequest = serde_json::from_str(REFRACTION).unwrap();
    request.validate().unwrap();
    let provider = RegistryDisclosureProvider::new();
    let response = WikiRefractionEngine::new(Some(&provider))
        .refract(&request)
        .unwrap();
    assert_eq!(response.status, RefractionStatus::Complete);
    assert_eq!(response.target_ref, "research:frame:transition-anomaly");
    assert_eq!(
        response.target_snapshot_hash,
        "sha256:research-transition-anomaly-v5"
    );
    assert_eq!(response.readings.len(), 3);
    assert!(
        response.readings.iter().all(|reading| reading
            .target_revision
            .as_ref()
            .unwrap()
            .to_string()
            == "5")
    );
    assert!(response.readings.iter().all(|reading| {
        reading.harmonic_field_ref.as_deref() == Some("ql:structural:2.0.0:field:A:1:D3")
    }));
    let disclosures = response
        .readings
        .iter()
        .map(|reading| reading.disclosure.as_str())
        .collect::<Vec<_>>();
    assert!(disclosures.iter().any(|value| value.contains("Processual")));
    assert!(
        disclosures
            .iter()
            .any(|value| value.contains("Archetypal-Numerical"))
    );
    assert!(
        disclosures
            .iter()
            .any(|value| value.contains("Divine Logos"))
    );
    assert!(response.readings.iter().all(|reading| {
        reading.derived_subgraph.vertices.len() == 4
            && reading.derived_subgraph.relations.len() == 1
            && reading.relation_candidates.is_empty()
    }));
}

#[test]
fn overlapping_a_and_c_square_vertices_retain_distinct_field_identity_under_mef() {
    let request_a: WikiRefractionRequest = serde_json::from_str(REFRACTION).unwrap();
    let mut request_c = request_a.clone();
    let field = request_c.target.structural_field.as_mut().unwrap();
    field.operator_ref = "ql:structural:2.0.0:field:C:2:D3".into();
    field.family = Some("C".into());
    field.pair_index = Some(2);
    request_c.validate().unwrap();

    let provider = RegistryDisclosureProvider::new();
    let a = WikiRefractionEngine::new(Some(&provider))
        .refract(&request_a)
        .unwrap();
    let c = WikiRefractionEngine::new(Some(&provider))
        .refract(&request_c)
        .unwrap();
    assert_eq!(
        a.readings[0].derived_subgraph.vertices,
        c.readings[0].derived_subgraph.vertices
    );
    assert_eq!(
        a.readings[0].harmonic_field_ref.as_deref(),
        Some("ql:structural:2.0.0:field:A:1:D3")
    );
    assert_eq!(
        c.readings[0].harmonic_field_ref.as_deref(),
        Some("ql:structural:2.0.0:field:C:2:D3")
    );
    assert_ne!(a.readings[0].operator_refs, c.readings[0].operator_refs);
}

#[test]
fn invalid_second_wiki_operator_is_validation_error_before_provider_state() {
    let mut request: WikiRefractionRequest = serde_json::from_str(REFRACTION).unwrap();
    request
        .target
        .structural_field
        .as_mut()
        .unwrap()
        .operator_ref = "ql:structural:2.0.0:field:B:0:D3".into();
    let result = WikiRefractionEngine::new(None).refract(&request);
    assert!(matches!(
        result,
        Err(WikiRefractionError::InvalidStructuralField(_))
    ));
}

#[test]
fn disabled_optional_required_modes_remain_identical_for_second_wiki() {
    let base: WikiRefractionRequest = serde_json::from_str(REFRACTION).unwrap();
    let mut disabled = base.clone();
    disabled.mode = ql_wiki::ProviderMode::Disabled;
    disabled.lenses.clear();
    assert_eq!(
        WikiRefractionEngine::new(None)
            .refract(&disabled)
            .unwrap()
            .status,
        RefractionStatus::Disabled
    );

    let mut optional = base.clone();
    optional.mode = ql_wiki::ProviderMode::Optional;
    assert_eq!(
        WikiRefractionEngine::new(None)
            .refract(&optional)
            .unwrap()
            .status,
        RefractionStatus::Unavailable
    );

    assert!(matches!(
        WikiRefractionEngine::new(None).refract(&base),
        Err(WikiRefractionError::ProviderRequired(_))
    ));
}

#[test]
fn proposed_external_feedback_does_not_mutate_projection_then_recognition_opens_new_route() {
    let mut projection =
        MetaKnowledgeProjection::rebuild(&meta_documents(), &bindings(), 1).unwrap();
    let glade_ref = "okf-concept-epi-as-the-instrument-of-mind-a796";
    let research_ref = "research:frame:transition-anomaly";
    let request = CrossWikiTraversalRequest {
        start_ref: glade_ref.into(),
        relation: None,
        operator_ref: None,
        lens_ref: None,
        max_hops: 10,
    };
    let before = MetaPortal::new(&projection, None)
        .cross_wiki_traverse(&request, &both_scopes())
        .unwrap();
    assert!(
        !before
            .routes
            .iter()
            .any(|route| route.destination_ref == research_ref)
    );

    let relation_count_before = projection.relations.len();
    let mut ledger = RecognitionLedger::default();
    let candidate = ledger
        .propose(ExternalObservation {
            observation_ref: "research:observation:process-structural-gap".into(),
            source_provider_ref: "knowledge:research-canvas".into(),
            source_target_ref: research_ref.into(),
            source_revision: Some("5".into()),
            suggested_from_meta_ref: "ql-mef:wiki:node:mef-l3".into(),
            suggested_to_meta_ref: "ql-mef:wiki:node:relation-family-a".into(),
            suggested_relation: "contextualised-by-structural-field".into(),
            rationale: "five repeated process readings need explicit structural-field context"
                .into(),
            occurrences: 5,
            evidence_refs: vec!["research:node:evidence".into()],
            extensions: serde_json::Map::new(),
        })
        .unwrap();
    assert_eq!(candidate.state, RecognitionState::Proposed);
    assert_eq!(projection.relations.len(), relation_count_before);
    assert!(
        !projection
            .relations
            .iter()
            .any(|relation| { relation.relation == "contextualised-by-structural-field" })
    );

    let recognised = ledger
        .recognise(
            &candidate.candidate_ref,
            RecognitionDecision {
                decision_ref: "ql-mef:decision:recognise-process-structural-gap".into(),
                reviewer_ref: "ql-mef:reviewer:human".into(),
                evidence_refs: vec!["ql-mef:review:qw4".into()],
            },
        )
        .unwrap();
    assert_eq!(
        ledger.candidate(&candidate.candidate_ref).unwrap().state,
        RecognitionState::Recognised
    );
    apply_recognised_amendment(&mut projection, &recognised).unwrap();
    let added = projection
        .relations
        .iter()
        .find(|relation| relation.relation == "contextualised-by-structural-field")
        .unwrap();
    assert_eq!(added.origin, "recognised");
    assert_eq!(
        added.origin_ref.as_deref(),
        Some("ql-mef:decision:recognise-process-structural-gap")
    );

    let after = MetaPortal::new(&projection, None)
        .cross_wiki_traverse(&request, &both_scopes())
        .unwrap();
    let route = after
        .routes
        .iter()
        .find(|route| route.destination_ref == research_ref)
        .unwrap();
    assert!(route.steps.iter().any(|step| {
        step.relation == "contextualised-by-structural-field" && step.origin == "recognised"
    }));
    assert!(
        route
            .steps
            .iter()
            .all(|step| !step.semantic_equivalence_asserted)
    );
}

#[test]
fn recognised_amendment_can_be_reapplied_after_projection_rebuild_and_reverse_traversal_works() {
    let mut ledger = RecognitionLedger::default();
    let candidate = ledger
        .propose(ExternalObservation {
            observation_ref: "research:observation:process-structural-gap".into(),
            source_provider_ref: "knowledge:research-canvas".into(),
            source_target_ref: "research:frame:transition-anomaly".into(),
            source_revision: Some("5".into()),
            suggested_from_meta_ref: "ql-mef:wiki:node:mef-l3".into(),
            suggested_to_meta_ref: "ql-mef:wiki:node:relation-family-a".into(),
            suggested_relation: "contextualised-by-structural-field".into(),
            rationale: "repeated external evidence".into(),
            occurrences: 5,
            evidence_refs: vec!["research:node:evidence".into()],
            extensions: serde_json::Map::new(),
        })
        .unwrap();
    let recognised = ledger
        .recognise(
            &candidate.candidate_ref,
            RecognitionDecision {
                decision_ref: "ql-mef:decision:recognise-process-structural-gap".into(),
                reviewer_ref: "ql-mef:reviewer:human".into(),
                evidence_refs: vec![],
            },
        )
        .unwrap();
    let serialized = serde_json::to_string(&recognised).unwrap();
    let restored: ql_wiki::RecognisedMetaAmendment = serde_json::from_str(&serialized).unwrap();

    let mut rebuilt = MetaKnowledgeProjection::rebuild(&meta_documents(), &bindings(), 99).unwrap();
    apply_recognised_amendment(&mut rebuilt, &restored).unwrap();
    assert_eq!(rebuilt.projection_version, 99);
    assert!(rebuilt.canonical_refs().contains("ql-mef:wiki:node:mef-l3"));
    assert!(!rebuilt.contains_foreign_object("research:frame:transition-anomaly"));

    let reverse = MetaPortal::new(&rebuilt, None)
        .cross_wiki_traverse(
            &CrossWikiTraversalRequest {
                start_ref: "research:frame:transition-anomaly".into(),
                relation: None,
                operator_ref: None,
                lens_ref: None,
                max_hops: 10,
            },
            &both_scopes(),
        )
        .unwrap();
    assert!(reverse.routes.iter().any(|route| {
        route.destination_ref == "okf-concept-epi-as-the-instrument-of-mind-a796"
    }));
    assert!(
        reverse
            .routes
            .iter()
            .flat_map(|route| route.steps.iter())
            .any(|step| { step.relation == "inverse:contextualised-by-structural-field" })
    );
}

#[test]
fn provider_loss_and_stale_revision_leave_binding_known_without_copying_foreign_content() {
    let projection = MetaKnowledgeProjection::rebuild(&meta_documents(), &bindings(), 3).unwrap();
    let mut resolver = StaticForeignResolver::default();
    resolver.insert(
        Some("knowledge:research-canvas".into()),
        "research:frame:transition-anomaly",
        ql_wiki::ForeignTargetResolution {
            target_ref: "research:frame:transition-anomaly".into(),
            provider_ref: Some("knowledge:research-canvas".into()),
            revision: Some("6".into()),
            availability: TargetAvailability::Unavailable,
            payload: Some(json!({"stale": "must not surface"})),
            notices: vec!["provider offline".into()],
        },
    );
    let portal = MetaPortal::new(&projection, Some(&resolver));
    let response = portal.manifestations(
        "ql-mef:wiki:node:relation-family-a",
        &PortalScope::new(["scope:research".to_owned()], true),
    );
    let target = response.manifestations.first().unwrap();
    assert_eq!(target.availability, TargetAvailability::Unavailable);
    assert_eq!(target.target_revision.as_deref(), Some("5"));
    assert!(target.payload.is_none());
    assert_eq!(target.origin, MappingOrigin::Authored);
    assert!(!projection.contains_foreign_object("research:frame:transition-anomaly"));
    assert!(projection.objects.iter().all(|object| {
        !object.canonical_ref.starts_with("research:")
            && !object.canonical_ref.starts_with("okf-concept-epi-")
    }));
}

#[test]
fn independent_wiki_fixture_contains_no_ql_mef_or_glade_ontology_takeover() {
    let corpus = format!(
        "{RESEARCH_SPACE}\n{RESEARCH_LOCAL}\n{RESEARCH_QUESTION}\n{RESEARCH_EVIDENCE}\n{RESEARCH_FRAME}"
    );
    assert!(corpus.contains("research-canvas/wiki/v1"));
    assert!(!corpus.contains("ql_mef_profile"));
    assert!(!corpus.contains("glade_profile"));
    assert!(!corpus.contains("Bimba Graph"));
}
