//! Independent source-to-consumer mutations of a real native relation plan.
//! Source-derived public receipts must retain face, dependency and branch when
//! replayed. These tests invoke production compile/prepare_request/execute;
//! they are proposed red trials, not source-string checks or executed verdicts.
use ql_mef::m2_condition::CorrespondenceRole;
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_relation_plan::{
    M2MefPhase, M2RelationPlan, M2RelationPlanContext, PreparedM2Relation, source_field,
};
use serde_json::Value;

fn prepared() -> (M2Request, PreparedM2Relation) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let value: Value = serde_json::from_slice(
        &std::fs::read(root.join("../../fixtures/kernel/m2-relation-plan-input-v1.json")).unwrap(),
    )
    .unwrap();
    let mut request: M2Request = serde_json::from_value(value["nativeRequest"].clone()).unwrap();
    let mut context: M2RelationPlanContext =
        serde_json::from_value(value["relationContext"].clone()).unwrap();
    request.condition.as_mut().unwrap().maqam_index = 3;
    request.condition.as_mut().unwrap().active_mef_condition = 36;
    request.mef_conditions.push(36);
    request.vimarsha.as_mut().unwrap().lens = 6;
    context.requested_maqam_index = Some(3);
    context.musical.maqam_ref = Some("#2-4.3-0-4".into());
    context.source_coordinates = [
        "#2-0",
        "#2-1",
        "#2-4.3-0-4",
        "#2-5-5",
        "#2-5-0/1-1",
        "#2-2-2-5-5",
    ]
    .map(String::from)
    .to_vec();
    let prepared = M2RelationPlan::compile(&request, context, source_field()).unwrap();
    assert_eq!(
        prepared
            .frame
            .condition
            .as_ref()
            .unwrap()
            .source_path
            .as_ref()
            .unwrap()
            .maqam_coordinate,
        "#2-4.3-0-4"
    );
    assert!(
        prepared
            .plan
            .source_receipts
            .operations
            .iter()
            .all(|op| op.mef_phase == M2MefPhase::Prime)
    );
    assert_eq!(
        prepared
            .plan
            .prepare_request(&request, source_field())
            .unwrap()
            .execute()
            .unwrap()
            .condition
            .as_ref()
            .unwrap()
            .drive
            .audio_octet_hz,
        prepared
            .frame
            .condition
            .as_ref()
            .unwrap()
            .drive
            .audio_octet_hz
    );
    (request, prepared)
}

#[test]
fn v293_wrong_retained_mef_face_cannot_replay_as_the_same_source_plan() {
    let (request, prepared) = prepared();
    let mut wrong = prepared.plan;
    assert!(!wrong.source_receipts.operations.is_empty());
    for op in &mut wrong.source_receipts.operations {
        op.mef_phase = M2MefPhase::Direct;
    }
    assert!(
        wrong.prepare_request(&request, source_field()).is_err(),
        "prime native plan accepted wrong retained direct face"
    );
}

#[test]
fn v293_lost_consumed_typed_relations_cannot_be_presented_as_source_qualified_replay() {
    let (request, prepared) = prepared();
    let mut wrong = prepared.plan;
    assert!(
        wrong
            .source_receipts
            .relation_sets
            .iter()
            .any(|set| set.kind == "TONIC_PLANETARY_RESONANCE")
    );
    assert!(
        wrong
            .source_receipts
            .relation_sets
            .iter()
            .any(|set| set.kind == "PLANETARY_RESONANCE")
    );
    wrong.source_receipts.relation_sets.clear();
    assert!(
        wrong.prepare_request(&request, source_field()).is_err(),
        "plan replay accepted deleted consumed typed source relations"
    );
}

#[test]
fn v293_different_valid_execution_branch_cannot_keep_the_original_tonic_plan_receipt() {
    let (request, prepared) = prepared();
    let mut wrong = prepared.plan;
    assert_eq!(wrong.situated.native_alignment.planet_coordinate, "#2-5-5");
    wrong.execution.condition_input.role = CorrespondenceRole::Dominant;
    assert!(
        wrong.prepare_request(&request, source_field()).is_err(),
        "changed native execution branch accepted with original tonic source plan"
    );
}
