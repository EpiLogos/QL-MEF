//! Same-generation valid native alternatives detect disconnected admission.
#[path = "support/retained_performance.rs"]
mod producer;
use ql_mef::nara_performance_receiving::*;
use ql_mef::performance_audio::*;
use ql_mef::performance_receiving_admission::*;
use serde_json::{Value, json};
fn context() -> ReceivingContext {
    ReceivingContext {
        kind: ContextKind::World,
        context: Reference {
            reference: "expression:world".into(),
            revision: "1".into(),
        },
        receiver: Reference {
            reference: "expression:world/receiver".into(),
            revision: "1".into(),
        },
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    }
}
fn input(prepared: &PreparedPerformanceBinding) -> ReceivingPreparation<'_> {
    ReceivingPreparation {
        prepared,
        context: context(),
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    }
}
fn native_alternatives() -> Vec<(&'static str, PreparedPerformanceBinding)> {
    let mut branch = producer::preparation();
    branch.coupled.m1.selected_coordinate = "#1-5-2".into();
    let mut clock = producer::preparation();
    clock.coupled.m1.tick12 = (clock.coupled.m1.tick12 + 1) % 12;
    let mut pose = producer::preparation();
    // Same source and post-command generation, different actual native pose/
    // clock snapshot. This is a real owner command, not a patched receipt.
    pose.coupled.m3_commands[0]
        .operations
        .push(ql_mef::m3_state::M3Operation::AdvanceClock { steps: 1 });
    vec![("branch", branch), ("clock", clock), ("pose", pose)]
        .into_iter()
        .map(|(name, request)| (name, prepare_native_performance(request).unwrap()))
        .collect()
}
fn fixture() -> Value {
    let prepared = prepare_native_performance(producer::preparation()).unwrap();
    let definition = prepare_native_receiving(input(&prepared)).unwrap();
    let admitted = prepare_native_receiving_admission(
        &definition,
        input(&prepared),
        prepared.native_basis(),
        0,
    )
    .unwrap();
    admitted
        .validate_current(&definition, input(&prepared), prepared.native_basis(), 0)
        .unwrap();
    let original = admitted.snapshot().unwrap();
    let alternatives: Vec<Value> = native_alternatives()
        .into_iter()
        .map(|(name, after)| {
            assert_eq!(
                after.determination()["identity"],
                prepared.determination()["identity"]
            );
            assert_eq!(
                after.native_basis().m3["identity"]["profile_generation"],
                prepared.native_basis().m3["identity"]["profile_generation"]
            );
            assert_ne!(
                serde_json::to_value(after.native_basis()).unwrap(),
                original["native_basis"]
            );
            assert!(
                definition.validate_sources(input(&after)).is_err(),
                "{name} must not reuse original N definition"
            );
            assert!(
                admitted
                    .validate_current(&definition, input(&after), after.native_basis(), 0)
                    .is_err()
            );
            assert!(
                prepare_native_receiving_admission(
                    &definition,
                    input(&prepared),
                    after.native_basis(),
                    0
                )
                .is_err()
            );
            // A separate valid native definition proves this is a valid alternative.
            let own_definition = prepare_native_receiving(input(&after)).unwrap();
            let own = prepare_native_receiving_admission(
                &own_definition,
                input(&after),
                after.native_basis(),
                0,
            )
            .unwrap();
            json!({"kind":name,"admission":own.snapshot().unwrap()})
        })
        .collect();
    json!({"schema":"ql.performance-receiving-admission-fixture/v1",
           "baseline":original,"alternatives":alternatives})
}
#[test]
fn privately_native_receiving_admission_rejects_other_valid_branch_clock_and_pose() {
    let packet = fixture();
    assert_eq!(packet["alternatives"].as_array().unwrap().len(), 3);
}
#[test]
#[ignore = "requires current native floor performance_receiving_admission binary"]
fn actual_native_cpp_admission_rejects_valid_disconnected_sources() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let binary = std::env::var("QL_NATIVE_RECEIVING_ADMISSION_TEST").expect("native floor binary");
    let bytes = serde_json::to_vec(&fixture()).unwrap();
    assert!(bytes.len() < 16 * 1024 * 1024);
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
