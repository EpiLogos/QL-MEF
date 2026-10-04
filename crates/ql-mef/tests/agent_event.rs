//! Integration through the public production event owner and real kernel.
use ql_mef::*;

use agent_event::{
    AgentEvent, DecisionResponse, ProjectionRequest, SemanticHead, admit_decision, event_basis,
    project_event, value_digest,
};
use serde_json::{Value, json};

fn event(facts: &[(&str, Value)]) -> AgentEvent {
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../fixtures/agent-decision/v1/event-v1.json"
    ))
    .unwrap();
    value["observed"] = json!(
        facts
            .iter()
            .map(|(field, value)| json!({"field":field,"value":value,
        "origin":"observed","basis_refs":[value_ref()]}))
            .collect::<Vec<_>>()
    );
    serde_json::from_value(value).unwrap()
}

fn value_ref() -> &'static str {
    "ql:fixture:agent-decision/v1/semantic-specimen.txt"
}

fn project(facts: &[(&str, Value)], heads: &[SemanticHead]) -> agent_event::EventProjection {
    project_event(ProjectionRequest {
        event: event(facts),
        requested_heads: heads.into(),
    })
    .unwrap()
}

fn value<'a>(facts: &'a Value, field: &str) -> &'a Value {
    &facts
        .as_array()
        .unwrap()
        .iter()
        .find(|fact| fact["field"] == field)
        .unwrap()["value"]
}

#[test]
fn explicit_prime_lens_bypasses_semantic_head_and_uses_native_pitch() {
    let result = project(
        &[
            ("lens", json!("L2'")),
            ("local-position", json!(3)),
            ("coordinate-face", json!("direct")),
            ("musical-basis", json!("chromatic")),
        ],
        &[SemanticHead::Lens],
    );
    assert!(result.decision_head_ids.is_empty());
    assert!(result.frame["unresolved"].as_array().unwrap().is_empty());
    assert!(result.determination.get("provider").is_none());
    assert_eq!(result.determination["learned"], json!([]));
    assert_eq!(
        value(&result.determination["derived"], "lens-face"),
        "night"
    );
    assert_eq!(
        value(&result.determination["derived"], "absolute-position"),
        5
    );
    assert_eq!(value(&result.harmonic["harmonic"], "lens-anchor"), 5);
    assert_eq!(value(&result.harmonic["harmonic"], "pitch-class"), 11);
}

#[test]
fn explicit_state_cannot_be_repaired_into_another_face() {
    let result = project_event(ProjectionRequest {
        event: event(&[("lens", json!("L2'")), ("lens-face", json!("day"))]),
        requested_heads: vec![SemanticHead::Lens],
    });
    assert!(result.unwrap_err().contains("contradicts kernel"));
}

#[test]
fn typed_refs_retain_current_revision_and_refuse_stale_lenses() {
    assert_eq!(
        value(
            &project(&[("lens", json!("mef:lens:L2'@1"))], &[]).determination["derived"],
            "lens-face"
        ),
        "night"
    );
    assert!(
        project_event(ProjectionRequest {
            event: event(&[("lens", json!("mef:lens:L2'@999"))]),
            requested_heads: vec![]
        })
        .is_err()
    );
}

#[test]
fn absolute_and_local_rotation_use_the_same_native_owner() {
    for lens in LensId::ALL {
        for n in 0..6 {
            let local = ql_core::QlPosition::new(n).unwrap();
            let rotation = MefRotation::new(lens, local);
            assert_eq!(
                MefRotation::from_absolute(lens, rotation.absolute_position()).local_position(),
                local
            );
            let result = project(
                &[
                    ("lens", json!(lens.code())),
                    (
                        "absolute-position",
                        json!(rotation.absolute_position().value()),
                    ),
                ],
                &[],
            );
            assert_eq!(value(&result.determination["derived"], "local-position"), n);
        }
    }
    assert!(
        project_event(ProjectionRequest {
            event: event(&[
                ("lens", json!("L2")),
                ("local-position", json!(3)),
                ("absolute-position", json!(0))
            ]),
            requested_heads: vec![]
        })
        .is_err()
    );
}

#[test]
fn relation_overlap_remains_two_candidates_and_two_completions() {
    let result = project(
        &[
            ("source-position", json!(2)),
            ("target-position", json!(3)),
            ("completion-degree", json!("D3")),
            ("lens", json!("L0")),
            ("musical-basis", json!("chromatic")),
        ],
        &[],
    );
    let relations = value(&result.determination["derived"], "relation-candidates");
    assert_eq!(
        relations,
        &json!([{"family":"A","pair_index":1,"reversed":false},
        {"family":"C","pair_index":2,"reversed":false}])
    );
    let frames = value(&result.determination["derived"], "completion-candidates")
        .as_array()
        .unwrap();
    assert_eq!(frames.len(), 2);
    assert_eq!(
        frames[0]["operator_ref"],
        "ql:structural:2.0.0:field:A:1:D3"
    );
    assert_eq!(
        frames[1]["operator_ref"],
        "ql:structural:2.0.0:field:C:2:D3"
    );
    assert_eq!(
        value(&result.harmonic["harmonic"], "completion-pitches")[0]["pitches"],
        json!([4, 6, 5, 7])
    );
}

#[test]
fn reversal_changes_traversal_direction_and_preserves_pair_identity() {
    let result = project(
        &[
            ("source-position", json!(3)),
            ("target-position", json!(2)),
            ("completion-degree", json!("D2")),
            ("d2-expansion", json!("source")),
        ],
        &[],
    );
    let relations = value(&result.determination["derived"], "relation-candidates");
    assert_eq!(
        relations[0],
        json!({"family":"A","pair_index":1,"reversed":true})
    );
    let frames = value(&result.determination["derived"], "completion-candidates");
    assert_eq!(frames[0]["expansion_side"], "right");
    assert_eq!(
        frames[0]["coordinates"][2],
        json!({"position":3,"face":"conjugate"})
    );
}

#[test]
fn missing_d2_side_is_an_unresolved_input_and_never_a_decision_head() {
    let result = project(
        &[
            ("source-position", json!(0)),
            ("target-position", json!(1)),
            ("completion-degree", json!("D2")),
        ],
        &[],
    );
    assert_eq!(result.determination["status"], "unresolved");
    assert_eq!(result.frame["unresolved"][0]["id"], "input-d2-expansion");
    assert!(result.decision_head_ids.is_empty());
    assert_eq!(
        value(&result.determination["derived"], "completion-candidates")
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn missing_endpoint_stays_unresolved_without_guessing_a_position() {
    let result = project(&[("source-position", json!(4))], &[]);
    assert_eq!(result.determination["status"], "unresolved");
    assert_eq!(result.frame["unresolved"][0]["field"], "target-position");
    assert!(result.decision_head_ids.is_empty());
}

#[test]
fn noncanonical_pair_does_not_acquire_an_invented_family() {
    let result = project(
        &[("source-position", json!(0)), ("target-position", json!(2))],
        &[],
    );
    assert_eq!(
        value(&result.determination["derived"], "relation-candidates"),
        &json!([])
    );
    assert_eq!(
        value(&result.determination["derived"], "completion-candidates"),
        &json!([])
    );
}

#[test]
fn context_frame_completion_precedes_head_creation_and_harmonics() {
    let result = project(
        &[
            ("context-local-position", json!(2)),
            ("context-unit-face", json!("power")),
            ("lens", json!("L0")),
            ("musical-basis", json!("chromatic")),
        ],
        &[SemanticHead::ContextFrame],
    );
    assert_eq!(
        value(&result.determination["derived"], "context-frame"),
        "CF4"
    );
    assert_eq!(
        value(&result.determination["derived"], "context-grain"),
        "inner-four"
    );
    assert_eq!(
        value(&result.harmonic["harmonic"], "context-frame-pitch"),
        5
    );
    assert!(result.frame["unresolved"].as_array().unwrap().is_empty());
    assert!(result.decision_head_ids.is_empty());
}

#[test]
fn context_frame_ambiguity_preserves_the_kernel_candidate_set() {
    let result = project(
        &[("context-local-position", json!(2))],
        &[SemanticHead::ContextFrame],
    );
    let head = &result.frame["unresolved"][0];
    assert_eq!(
        head["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|label| label["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["CF3", "CF4"]
    );
    assert_eq!(head["ambiguity_policy"], "preserve-candidates");
    assert_eq!(head["cardinality"]["max"], 2);
    assert_eq!(
        result.frame["constraints"][0]["allowed"]["label_ids"],
        json!(["CF3", "CF4"])
    );
}

#[test]
fn recognized_invalid_fields_refuse_even_when_their_projection_is_unused() {
    for (field, invalid) in [
        ("coordinate-face", json!("invalid")),
        ("completion-degree", json!("D9")),
        ("musical-basis", json!("invented")),
        ("local-position", json!(6)),
        ("context-unit-face", json!("wrong")),
        ("context-grain", json!("wrong")),
        ("context-frame", json!("CF8")),
    ] {
        assert!(
            project_event(ProjectionRequest {
                event: event(&[(field, invalid)]),
                requested_heads: vec![]
            })
            .is_err(),
            "{field}"
        );
    }
}

#[test]
fn wrong_context_frame_cannot_overrule_supplied_structure() {
    assert!(
        project_event(ProjectionRequest {
            event: event(&[
                ("context-frame", json!("CF3")),
                ("context-unit-face", json!("power"))
            ]),
            requested_heads: vec![]
        })
        .is_err()
    );
}

#[test]
fn callable_operation_labels_follow_native_faculty_membership() {
    let result = project(&[("faculty", json!("#3"))], &[SemanticHead::Operation]);
    assert_eq!(
        result.frame["unresolved"][0]["labels"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        result.frame["unresolved"][0]["labels"][0]["id"],
        "representation.bind"
    );
    assert!(
        project_event(ProjectionRequest {
            event: event(&[("operation", json!("ql-techne-reading"))]),
            requested_heads: vec![]
        })
        .is_err()
    );
    assert!(
        project_event(ProjectionRequest {
            event: event(&[
                ("faculty", json!("#2")),
                ("operation", json!("nara.journey.open"))
            ]),
            requested_heads: vec![]
        })
        .is_err()
    );
    let inverse = project(
        &[("operation", json!("nara.journey.open"))],
        &[SemanticHead::Faculty],
    );
    assert_eq!(value(&inverse.determination["derived"], "faculty"), "#4");
    assert!(inverse.decision_head_ids.is_empty());
}

#[test]
fn simultaneous_faculty_and_operation_heads_have_native_combination_constraints() {
    let result = project(&[], &[SemanticHead::Faculty, SemanticHead::Operation]);
    let tuples = result.frame["constraints"][0]["allowed_tuples"]
        .as_array()
        .unwrap();
    assert!(tuples.iter().any(|row| row[0]["label_ids"] == json!(["#2"])
        && row[1]["label_ids"] == json!(["bimba.neighborhood"])));
    assert!(
        !tuples.iter().any(|row| row[0]["label_ids"] == json!(["#2"])
            && row[1]["label_ids"] == json!(["nara.journey.open"]))
    );
}

#[test]
fn source_generation_and_material_changes_fence_reuse_but_body_bindings_do_not() {
    let original = event(&[]);
    let basis = event_basis(&original).unwrap();
    assert_eq!(
        basis,
        "sha256:03f36b162a07c8e916ed7c0c74cec325f2a5309ea706eba9b5235b8035b131e2"
    );
    let mut changed = original.clone();
    changed.bindings = None;
    assert_eq!(event_basis(&changed).unwrap(), basis);
    changed.generation += 1;
    assert_ne!(event_basis(&changed).unwrap(), basis);
    let mut changed = original.clone();
    changed.source_basis[0].revision.push_str("-next");
    assert_ne!(event_basis(&changed).unwrap(), basis);
    let mut changed = original;
    changed.material.text.push_str(" more");
    assert_ne!(event_basis(&changed).unwrap(), basis);
}

#[test]
fn supplied_null_optional_fields_are_refused_without_normalization() {
    let original = serde_json::to_value(event(&[("lens", json!("L2"))])).unwrap();
    for path in ["bindings", "body_ref", "rule_ref"] {
        let mut invalid = original.clone();
        match path {
            "bindings" => invalid["bindings"] = Value::Null,
            "body_ref" => invalid["bindings"]["body_ref"] = Value::Null,
            _ => invalid["observed"][0]["rule_ref"] = Value::Null,
        }
        assert!(
            serde_json::from_value::<AgentEvent>(invalid).is_err(),
            "{path}"
        );
    }
}

#[test]
fn canonical_numbers_and_unicode_match_independently_generated_python_vectors() {
    let vectors: Value = serde_json::from_str(include_str!(
        "../../../fixtures/agent-decision/v1/digest-vectors.json"
    ))
    .unwrap();
    for row in vectors["vectors"].as_array().unwrap() {
        assert_eq!(
            value_digest(&row["value"]).unwrap(),
            row["digest"].as_str().unwrap(),
            "{}",
            row["value"]
        );
    }
}

#[test]
fn missing_semantic_material_never_requests_a_model() {
    let mut event = event(&[]);
    event.material.text.clear();
    let result = project_event(ProjectionRequest {
        event,
        requested_heads: vec![SemanticHead::Lens],
    })
    .unwrap();
    assert_eq!(result.determination["status"], "unresolved");
    assert!(result.decision_head_ids.is_empty());
}

#[test]
fn partial_lens_state_restricts_candidates_and_completes_unique_readings() {
    let fixed = project(
        &[("lens-position", json!(2)), ("lens-face", json!("night"))],
        &[SemanticHead::Lens],
    );
    assert_eq!(value(&fixed.determination["derived"], "lens"), "L2'");
    assert!(fixed.decision_head_ids.is_empty());
    let partial = project(&[("lens-position", json!(2))], &[SemanticHead::Lens]);
    assert_eq!(
        partial.frame["unresolved"][0]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["L2", "L2'"]
    );
    let inverse = project(
        &[("lens-complement", json!("mef:lens:L5'@1"))],
        &[SemanticHead::Lens],
    );
    assert_eq!(value(&inverse.determination["derived"], "lens"), "L0'");
    for (field, v) in [
        ("lens-position", json!(6)),
        ("lens-face", json!("twilight")),
        ("lens-conjugate", json!("L7")),
    ] {
        assert!(
            project_event(ProjectionRequest {
                event: event(&[(field, v)]),
                requested_heads: vec![]
            })
            .is_err()
        );
    }
}

#[test]
fn completion_without_a_structural_basis_remains_unresolved() {
    let missing = project(&[("completion-degree", json!("D3"))], &[]);
    assert_eq!(missing.determination["status"], "unresolved");
    assert!(missing.decision_head_ids.is_empty());
    assert!(
        project_event(ProjectionRequest {
            event: event(&[("d2-expansion", json!("source"))]),
            requested_heads: vec![]
        })
        .is_err()
    );
    assert!(
        project_event(ProjectionRequest {
            event: event(&[
                ("completion-degree", json!("D1")),
                ("d2-expansion", json!("source"))
            ]),
            requested_heads: vec![]
        })
        .is_err()
    );
}

#[test]
fn explicit_relation_selection_is_checked_without_erasing_overlap() {
    let fixed = project(
        &[
            ("source-position", json!(2)),
            ("target-position", json!(3)),
            ("relation-family", json!("C")),
            ("pair-index", json!(2)),
            ("completion-degree", json!("D3")),
        ],
        &[],
    );
    assert_eq!(
        value(&fixed.determination["derived"], "relation-candidates")
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        value(&fixed.determination["derived"], "completion-candidates")
            .as_array()
            .unwrap()
            .len(),
        1
    );
    for (family, index) in [("B", 2), ("C", 1), ("A", 3)] {
        assert!(
            project_event(ProjectionRequest {
                event: event(&[
                    ("source-position", json!(2)),
                    ("target-position", json!(3)),
                    ("relation-family", json!(family)),
                    ("pair-index", json!(index))
                ]),
                requested_heads: vec![]
            })
            .is_err()
        );
    }
    let pair = project(
        &[
            ("relation-family", json!("C")),
            ("pair-index", json!(2)),
            ("completion-degree", json!("D3")),
        ],
        &[],
    );
    let completion = &value(&pair.determination["derived"], "completion-candidates")[0];
    assert_eq!(
        completion["operator_ref"],
        "ql:structural:2.0.0:field:C:2:D3"
    );
    assert!(completion.get("reversed").is_none());
    assert_eq!(pair.determination["status"], "determined");
}

#[test]
fn harmonic_projection_refuses_conflicting_observations_and_uses_native_ratios() {
    let facts = [
        ("lens", json!("L0")),
        ("local-position", json!(2)),
        ("coordinate-face", json!("direct")),
        ("musical-basis", json!("fifths")),
    ];
    let result = project(&facts, &[]);
    let ratio = MusicalBasis::Fifths.generator_ratio();
    assert_eq!(
        value(&result.harmonic["harmonic"], "basis-generator-ratio"),
        &json!({"numerator":ratio.numerator(),"denominator":ratio.denominator()})
    );
    let mut wrong = facts.to_vec();
    wrong.push(("pitch-class", json!(3)));
    assert!(
        project_event(ProjectionRequest {
            event: event(&wrong),
            requested_heads: vec![]
        })
        .is_err()
    );
}

fn decision(request: &ProjectionRequest, proposals: Value) -> DecisionResponse {
    let projection = project_event(request.clone()).unwrap();
    serde_json::from_value(json!({"schema":"ql.agent-decision-response/v1",
        "event_basis_digest":projection.frame["event_basis_digest"],"frame_digest":value_digest(&projection.frame).unwrap(),
        "kernel_basis":projection.frame["kernel_basis"],"outcome":"answered",
        "provider":{"provider_ref":"test:controlled-discriminator","model_ref":"test:controlled-selections",
            "model_revision":"1","runtime_revision":"1"},"proposals":proposals})).unwrap()
}

fn request(facts: &[(&str, Value)], heads: &[SemanticHead]) -> ProjectionRequest {
    ProjectionRequest {
        event: event(facts),
        requested_heads: heads.to_vec(),
    }
}

#[test]
fn impossible_model_label_is_retained_and_refused_without_repair() {
    let request = request(&[], &[SemanticHead::Lens]);
    let response = decision(
        &request,
        json!([{"head_id":"semantic-lens","label_ids":["L99"],"spans":[],"confidence":0.99}]),
    );
    let admission = admit_decision(request, response.clone()).unwrap();
    assert_eq!(admission.admission_status, "refused");
    assert_eq!(admission.response, response);
    let determination = &admission.projection.determination;
    assert_eq!(determination["learned"][0]["label_ids"], json!(["L99"]));
    assert_eq!(
        determination["refused_candidates"][0]["label_ids"],
        json!(["L99"])
    );
    assert_eq!(determination["validated"], json!([]));
}

#[test]
fn inconsistent_faculty_operation_is_refused_by_native_combination() {
    let request = request(&[], &[SemanticHead::Faculty, SemanticHead::Operation]);
    let response = decision(
        &request,
        json!([
        {"head_id":"semantic-faculty","label_ids":["#2"],"spans":[]},
        {"head_id":"semantic-operation","label_ids":["nara.journey.open"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert_eq!(admission.projection.determination["status"], "refused");
    assert_eq!(
        admission.projection.determination["refused_candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(admission.projection.determination["validated"], json!([]));
}

#[test]
fn accepted_lens_uses_same_completion_harmonics_and_retains_learned_standing() {
    let request = request(
        &[
            ("local-position", json!(3)),
            ("coordinate-face", json!("direct")),
            ("musical-basis", json!("chromatic")),
        ],
        &[SemanticHead::Lens],
    );
    let original = project_event(request.clone()).unwrap();
    let response = decision(
        &request,
        json!([{"head_id":"semantic-lens","label_ids":["L2'"],"spans":[]}]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert_eq!(admission.admission_status, "admitted");
    assert_eq!(
        admission.projection.determination["observed"],
        original.determination["observed"]
    );
    assert_eq!(
        admission.projection.determination["validated"][0]["origin"],
        "learned"
    );
    assert_eq!(
        value(
            &admission.projection.determination["derived"],
            "absolute-position"
        ),
        5
    );
    assert_eq!(
        value(&admission.projection.harmonic["harmonic"], "pitch-class"),
        11
    );
    assert_eq!(admission.projection.determination["status"], "determined");
}

#[test]
fn accepted_ambiguity_produces_separate_native_harmonic_readings() {
    let request = request(
        &[
            ("lens-position", json!(2)),
            ("musical-basis", json!("chromatic")),
        ],
        &[SemanticHead::Lens],
    );
    let response = decision(
        &request,
        json!([{"head_id":"semantic-lens","label_ids":["L2","L2'"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert_eq!(
        admission.projection.determination["validated"][0]["value"],
        json!(["L2", "L2'"])
    );
    let readings = value(
        &admission.projection.harmonic["harmonic"],
        "candidate-projections",
    );
    assert_eq!(readings.as_array().unwrap().len(), 2);
    assert_eq!(value(&readings[0]["harmonic"], "lens-anchor"), 4);
    assert_eq!(value(&readings[1]["harmonic"], "lens-anchor"), 5);
}

#[test]
fn stale_response_cannot_change_newer_event_or_retire_ordinary_ql() {
    let original = request(&[], &[SemanticHead::Lens]);
    let response = decision(
        &original,
        json!([{"head_id":"semantic-lens","label_ids":["L2"],"spans":[]} ]),
    );
    let mut newer = original.clone();
    newer.event.generation += 1;
    let stale = admit_decision(newer, response.clone()).unwrap();
    assert_eq!(stale.admission_status, "stale");
    assert_eq!(stale.projection.determination["validated"], json!([]));
    assert_eq!(stale.response, response);
    let bypass = admit_decision(
        request(&[("lens", json!("L2'"))], &[SemanticHead::Lens]),
        response,
    )
    .unwrap();
    assert_eq!(bypass.admission_status, "bypassed");
    assert!(bypass.projection.determination.get("provider").is_none());
    assert_eq!(
        value(&bypass.projection.determination["derived"], "lens-face"),
        "night"
    );
}

#[test]
fn unavailable_and_abstention_preserve_unresolved_standing() {
    let request = request(&[], &[SemanticHead::Lens]);
    let mut response = decision(&request, json!([]));
    response.outcome = agent_event::DecisionOutcome::Unavailable;
    response.reason = Some("local classifier not installed".into());
    let unavailable = admit_decision(request.clone(), response).unwrap();
    assert_eq!(
        unavailable.projection.determination["status"],
        "unavailable"
    );
    assert_eq!(unavailable.projection.determination["learned"], json!([]));
    let abstain = admit_decision(
        request.clone(),
        decision(
            &request,
            json!([{"head_id":"semantic-lens","label_ids":[],"spans":[],"confidence":1.0}]),
        ),
    )
    .unwrap();
    assert_eq!(abstain.projection.determination["status"], "unresolved");
    assert_eq!(abstain.projection.determination["validated"], json!([]));
}

#[test]
fn forged_unicode_evidence_and_missing_provider_are_refused() {
    let mut request = request(&[], &[SemanticHead::Lens]);
    request.event.material.text = "α😀QL".into();
    let response = decision(
        &request,
        json!([{"head_id":"semantic-lens","label_ids":["L2"],"spans":[{
        "material_ref":request.event.material.r#ref,"revision":request.event.material.revision,
        "start":1,"end":2,"text":"😀"}]}]),
    );
    assert_eq!(
        admit_decision(request.clone(), response.clone())
            .unwrap()
            .admission_status,
        "admitted"
    );
    let mut forged = response.clone();
    forged.proposals[0].spans[0].text = "Q".into();
    let refused = admit_decision(request.clone(), forged.clone()).unwrap();
    assert_eq!(refused.admission_status, "refused");
    assert_eq!(refused.response, forged);
    assert_eq!(refused.projection.determination["validated"], json!([]));
    let mut anonymous = response;
    anonymous.provider = None;
    assert_eq!(
        admit_decision(request, anonymous).unwrap().admission_status,
        "refused"
    );
}

#[test]
fn half_a_connected_decision_cannot_become_operative() {
    let request = request(&[], &[SemanticHead::Faculty, SemanticHead::Operation]);
    let response = decision(
        &request,
        json!([{"head_id":"semantic-operation","label_ids":["bimba.neighborhood"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert_eq!(admission.projection.determination["validated"], json!([]));
    assert_eq!(
        admission.projection.determination["unresolved"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn observed_rotation_and_harmonics_determine_before_classification() {
    let result = project(
        &[
            ("local-position", json!(2)),
            ("absolute-position", json!(4)),
            ("lens-face", json!("night")),
            ("coordinate-face", json!("direct")),
            ("musical-basis", json!("chromatic")),
        ],
        &[SemanticHead::Lens],
    );
    assert!(result.decision_head_ids.is_empty());
    assert_eq!(value(&result.determination["derived"], "lens"), "L2'");
    let result = project(
        &[
            ("lens", json!("L0")),
            ("musical-basis", json!("chromatic")),
            ("context-frame-pitch", json!(5)),
        ],
        &[SemanticHead::ContextFrame],
    );
    assert!(result.decision_head_ids.is_empty());
    assert_eq!(
        value(&result.determination["derived"], "context-frame"),
        "CF4"
    );
}

#[test]
fn failed_post_admission_completion_returns_exact_attributable_refusal() {
    let request = request(
        &[
            ("musical-basis", json!("chromatic")),
            ("context-frame-pitch", json!(0)),
        ],
        &[SemanticHead::Lens, SemanticHead::ContextFrame],
    );
    let response = decision(
        &request,
        json!([
        {"head_id":"semantic-lens","label_ids":["L2"],"spans":[]},
        {"head_id":"semantic-context-frame","label_ids":["CF1"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response.clone()).unwrap();
    assert_eq!(admission.admission_status, "refused");
    assert_eq!(admission.response, response);
    assert_eq!(admission.projection.determination["validated"], json!([]));
    assert_eq!(
        admission.projection.determination["refused_candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn accepted_operation_completes_abstained_faculty_and_missing_lens_is_discharged() {
    let request = request(&[], &[SemanticHead::Faculty, SemanticHead::Operation]);
    let response = decision(
        &request,
        json!([
        {"head_id":"semantic-faculty","label_ids":[],"spans":[]},
        {"head_id":"semantic-operation","label_ids":["bimba.neighborhood"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert_eq!(admission.projection.determination["status"], "determined");
    assert_eq!(admission.projection.determination["unresolved"], json!([]));
    assert_eq!(
        value(&admission.projection.determination["derived"], "faculty"),
        "#2"
    );
    assert_eq!(
        admission.projection.determination["validated"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let request = crate::request(
        &[("musical-basis", json!("chromatic"))],
        &[SemanticHead::Lens],
    );
    let response = decision(
        &request,
        json!([{"head_id":"semantic-lens","label_ids":["L2"],"spans":[]} ]),
    );
    let admission = admit_decision(request, response).unwrap();
    assert!(admission.projection.missing_inputs.is_empty());
}

#[test]
fn constellation_shape_and_compression_use_native_structural_members() {
    let members = |positions: &[u8]| {
        json!({"anchor_ref":"anchor:event","members":positions.iter().map(|n|
        json!({"subject_ref":format!("member:{n}"),"position":n,"face":"direct"})).collect::<Vec<_>>()})
    };
    let result = project(
        &[
            ("constellation", members(&[1, 2, 3])),
            ("lens", json!("L0")),
            ("musical-basis", json!("chromatic")),
        ],
        &[],
    );
    let shape = ql_core::QlShape::Constellation(ql_core::ConstellationGrain::ThreeFold123);
    assert_eq!(
        value(&result.determination["derived"], "shape"),
        &json!(shape.shape_ref())
    );
    assert_eq!(
        value(&result.determination["derived"], "shape-fold-count"),
        3
    );
    assert_eq!(
        value(&result.determination["derived"], "shape-compression")["recognition_superset"],
        ql_core::QlShape::Constellation(ql_core::ConstellationGrain::FourFold1234).shape_ref()
    );
    assert_eq!(
        value(&result.harmonic["harmonic"], "constellation-pitches")
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["pitch_class"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![2, 4, 6]
    );
    let mut invalid = members(&[1]);
    invalid["members"][0]["face"] = json!("conjugate");
    assert!(project_event(request(&[("constellation", invalid)], &[])).is_err());
    let mut invalid = members(&[1]);
    invalid["members"][0]["fabricated"] = json!(true);
    assert!(project_event(request(&[("constellation", invalid)], &[])).is_err());
}

#[test]
fn anonymous_native_constellation_retains_grain_without_an_invented_shape_ref() {
    for positions in [vec![0], vec![0, 1, 2]] {
        let members = json!({"anchor_ref":"anchor:event","members":positions.iter().map(|n|
            json!({"subject_ref":format!("member:{n}"),"position":n,"face":"direct"})).collect::<Vec<_>>()});
        let result = project(&[("constellation", members.clone())], &[]);
        assert!(
            !result.determination["derived"]
                .as_array()
                .unwrap()
                .iter()
                .any(|fact| fact["field"] == "shape")
        );
        let native = ql_core::StructuralConstellation::new(
            "anchor:event",
            positions
                .into_iter()
                .map(|n| {
                    ql_core::StructuralParticipation::new(
                        format!("member:{n}"),
                        ql_core::QlPosition::new(n).unwrap(),
                        ql_core::QlFace::Direct,
                    )
                    .unwrap()
                })
                .collect(),
            vec![],
        )
        .unwrap();
        assert_eq!(
            value(&result.determination["derived"], "constellation-grain"),
            &json!({"kind":native.grain().as_str(),"direct":native.members.len(),"conjugate":0})
        );
        assert!(
            project_event(request(
                &[
                    ("constellation", members.clone()),
                    (
                        "shape",
                        json!(
                            ql_core::QlShape::Constellation(ql_core::ConstellationGrain::TwoFold)
                                .shape_ref()
                        )
                    )
                ],
                &[]
            ))
            .is_err()
        );
        assert!(
            project_event(request(
                &[
                    ("constellation", members),
                    ("shape", json!("ql:shape:1.0.0:constellation:other"))
                ],
                &[]
            ))
            .is_err()
        );
    }
}

#[test]
fn known_pair_completes_missing_endpoint_before_d2_input_checks() {
    let result = project(
        &[
            ("relation-family", json!("C")),
            ("pair-index", json!(0)),
            ("source-position", json!(5)),
            ("completion-degree", json!("D2")),
            ("d2-expansion", json!("source")),
        ],
        &[],
    );
    assert_eq!(
        value(&result.determination["derived"], "target-position"),
        0
    );
    assert_eq!(result.determination["status"], "determined");
    assert!(result.missing_inputs.is_empty());
    assert_eq!(
        value(&result.determination["derived"], "completion-candidates")[0]["expansion_side"],
        "right"
    );
    assert!(
        project_event(request(
            &[
                ("relation-family", json!("C")),
                ("pair-index", json!(0)),
                ("source-position", json!(2))
            ],
            &[]
        ))
        .is_err()
    );
}
