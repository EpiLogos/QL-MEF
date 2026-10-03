use super::*;
use std::io::Write;
use std::process::{Command, Stdio};
#[path = "../../tests/support/retained_source_performance.rs"]
mod native_source;
fn wire(owner: &PerformanceOwner) -> Value {
    json!({"schema":CONTROL,"operation":"prepare","session_ref":owner.config.session_ref,
        "packet":owner.native_packet().unwrap(),"current_source_packet":owner.native_packet().unwrap(),
        "actual_native_basis":owner.binding().native_basis(),"m1_pratibimba":owner.config.source_face==1,
        "physical_pratibimba":owner.config.physical_face==1,
        "body_source":{"kind":"sourceForm","recipe_ref":owner.config.recipe.provenance.reference,
            "validated_m3_generation":owner.config.controls.expected_m3_generation.to_string()}})
}
#[test]
#[ignore = "requires actual native source/Management wire consumer qualified by the parent"]
fn actual_valid_other_bodies_cannot_replace_resident_source_reply() {
    let (current, config) = native_source::config(true);
    let mut resident =
        PerformanceOwner::prepare(&current, "expression:retained/current", config.clone()).unwrap();
    let mut alternate = config.clone();
    alternate.controls.body_revision = 2;
    alternate.controls.preparation_ref =
        "controlled:source-performance/valid-other-preparation".into();
    alternate.controls.state_ref = "controlled:source-performance/valid-other-resident".into();
    let other_body =
        PerformanceOwner::prepare(&current, "expression:retained/current", alternate).unwrap();
    let mut alternate = config.clone();
    alternate.physical_face = 0;
    let other_face =
        PerformanceOwner::prepare(&current, "expression:retained/current", alternate).unwrap();
    let mut changed = current.input.clone();
    let mut source_change = changed.m3_commands[0].clone();
    source_change.expected_generation = current.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    source_change.occurrence_unix_ms += 1;
    source_change.receipt_unix_ms += 1;
    source_change.operations = vec![crate::m3_state::M3Operation::ChangeLine { line: 0 }];
    changed.m3_commands.push(source_change);
    let changed = changed.compose().unwrap();
    let mut alternate = config.clone();
    alternate.controls.expected_m3_generation = changed.m3["identity"]["profile_generation"]
        .as_u64()
        .unwrap();
    let other_generation =
        PerformanceOwner::prepare(&changed, "expression:retained/current", alternate).unwrap();
    let mut pose = current.input.clone();
    pose.m3_commands[0]
        .operations
        .push(crate::m3_state::M3Operation::AdvanceClock { steps: 1 });
    let pose = pose.compose().unwrap();
    let other_pose =
        PerformanceOwner::prepare(&pose, "expression:retained/current", config).unwrap();
    let owners = [
        &resident,
        &other_body,
        &other_face,
        &other_generation,
        &other_pose,
    ];

    let binary =
        std::env::var("QL_NATIVE_SOURCE_REPLY_TEST").expect("actual native reply driver required");
    let mut replies = Vec::with_capacity(5);
    for (index, owner) in owners.iter().enumerate() {
        let input = json!({"schema":"ql.source-reply-detecting-input/v1","case_index":index,"case":wire(owner)});
        let bytes = serde_json::to_vec(&input).unwrap();
        let destination = before_source_case(index, &bytes);
        assert!(
            bytes.len() <= 16 * 1024 * 1024,
            "actual source case input bound"
        );
        let mut child = Command::new(&binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(&bytes).unwrap();
        drop(stdin);
        let result = child.wait_with_output().unwrap();
        preserve_source_case(destination.as_deref(), &result);
        assert!(
            result.status.success(),
            "actual native reply consumer failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let output: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(output["schema"], "ql.actual-native-source-replies/v1");
        assert_eq!(output.as_object().unwrap().len(), 3);
        assert_eq!(output["case_index"], index);
        let actual = output["replies"].as_array().unwrap();
        assert_eq!(actual.len(), 1);
        replies.push(actual[0].clone());
    }
    assert_eq!(replies.len(), 5);
    // Every alternative is an actual valid independently prepared source/body.
    // Rejecting it below is current resident identity, never malformed JSON.
    for (owner, reply) in owners.iter().zip(&replies) {
        owner.validate_reply(reply).unwrap();
    }
    resident.last = Some(replies[0].clone());
    for reply in &replies[1..] {
        assert!(resident.validate_reply(reply).is_err());
    }
    assert_eq!(
        other_pose.binding().determination(),
        resident.binding().determination()
    );
    assert_ne!(
        replies[4]["reading"]["physical"]["eigenbasis_identity"],
        replies[0]["reading"]["physical"]["eigenbasis_identity"]
    );
}

// Exact preexecution source bytes and actual child output are retained before
// size/status/parser/native validators. Every native case keeps its full wire.
fn before_source_case(index: usize, input: &[u8]) -> Option<std::path::PathBuf> {
    let Some(directory) = std::env::var_os("QL_NATIVE_SOURCE_REPLY_EVIDENCE_DIR") else {
        eprintln!(
            "actual_source_case_preexecution index={index} input_bytes={} limit={}",
            input.len(),
            16 * 1024 * 1024
        );
        return None;
    };
    let root = std::path::Path::new(&directory);
    std::fs::create_dir_all(root).unwrap();
    let path = root.join(index.to_string());
    std::fs::create_dir(&path).expect("fresh native source case evidence directory");
    retain_source_case_file(&path, "producer-input.json", input, 16 * 1024 * 1024);
    retain_source_case_file(&path, "native-preexecution.json", &serde_json::to_vec(&json!({"schema":"ql.native-source-reply-preexecution/v1","case_index":index,"actual_input_bytes":input.len(),"stdin_limit_bytes":16*1024*1024,"complete_input":input.len()<=16*1024*1024,"child_spawned":false})).unwrap(),4096);
    Some(path)
}
fn retain_source_case_file(root: &std::path::Path, name: &str, bytes: &[u8], limit: usize) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(name))
        .unwrap();
    file.write_all(&bytes[..bytes.len().min(limit)]).unwrap();
    file.sync_all().unwrap();
    if bytes.len() > limit {
        let mut marker = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(format!("{name}.truncated.json")))
            .unwrap();
        write!(
            marker,
            "{}",
            json!({"complete":false,"actual_bytes":bytes.len(),"retained_prefix_bytes":limit})
        )
        .unwrap();
        marker.sync_all().unwrap();
    }
}
fn preserve_source_case(destination: Option<&std::path::Path>, output: &std::process::Output) {
    if let Some(root) = destination {
        retain_source_case_file(root, "native-stdout.json", &output.stdout, 32 * 1024 * 1024);
        retain_source_case_file(root, "native-stderr.txt", &output.stderr, 4 * 1024 * 1024);
        retain_source_case_file(
            root,
            "native-exit.json",
            &serde_json::to_vec(
                &json!({"success":output.status.success(),"code":output.status.code()}),
            )
            .unwrap(),
            4096,
        );
    }
    assert!(
        output.stdout.len() <= 32 * 1024 * 1024 && output.stderr.len() <= 4 * 1024 * 1024,
        "native source reply output exceeds unchanged transport bound"
    );
}
