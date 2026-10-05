use super::native_source_support as native_source;
use super::*;
use std::io::Write;
use std::process::{Command, Stdio};
fn wire(owner: &PerformanceOwner) -> Value {
    // One immutable actual-owner serialization supplies the two required
    // original packet operands. No cross-owner or cross-invocation cache.
    let packet = owner.native_packet().unwrap();
    json!({"schema":CONTROL,"operation":"prepare","session_ref":owner.config.session_ref,
        "packet":packet,"current_source_packet":packet,
        "actual_native_basis":owner.binding().native_basis(),"m1_pratibimba":owner.config.source_face==1,
        "physical_pratibimba":owner.config.physical_face==1,
        "body_source":{"kind":"sourceForm","recipe_ref":owner.config.recipe.provenance.reference,
            "validated_m3_generation":owner.config.controls.expected_m3_generation.to_string()}})
}
#[test]
#[ignore = "requires actual native source/Management wire consumer qualified by the parent"]
fn actual_valid_other_bodies_cannot_replace_resident_source_reply() {
    let progress = ActualSourceReplyProgress(std::time::Instant::now());
    progress.mark("current-source-configuration begin");
    let (current, config) = native_source::config(true);
    progress.mark("current-source-configuration complete");
    progress.mark("resident-owner prepare begin");
    let mut resident =
        PerformanceOwner::prepare(&current, "expression:retained/current", config.clone()).unwrap();
    progress.mark("resident prepare complete");
    let mut alternate = config.clone();
    alternate.controls.body_revision = 2;
    alternate.controls.preparation_ref =
        "controlled:source-performance/valid-other-preparation".into();
    alternate.controls.state_ref = "controlled:source-performance/valid-other-resident".into();
    progress.mark("other_body prepare begin");
    let other_body =
        PerformanceOwner::prepare(&current, "expression:retained/current", alternate).unwrap();
    progress.mark("other_body prepare complete");
    let mut alternate = config.clone();
    alternate.physical_face = 0;
    progress.mark("other_face prepare begin");
    let other_face =
        PerformanceOwner::prepare(&current, "expression:retained/current", alternate).unwrap();
    progress.mark("other_face prepare complete");
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
    progress.mark("other_generation prepare begin");
    let other_generation =
        PerformanceOwner::prepare(&changed, "expression:retained/current", alternate).unwrap();
    progress.mark("other_generation prepare complete");
    let mut pose = current.input.clone();
    pose.m3_commands[0]
        .operations
        .push(crate::m3_state::M3Operation::AdvanceClock { steps: 1 });
    let pose = pose.compose().unwrap();
    progress.mark("other_clock prepare begin");
    let other_clock =
        PerformanceOwner::prepare(&pose, "expression:retained/current", config.clone()).unwrap();
    progress.mark("other_clock prepare complete");
    // AdvanceClock changes genuine source time but can leave the current pose
    // and numerical body unchanged. Separately exercise an actual SetPose.
    let mut rotated = current.input.clone();
    let pose_index = current.m3["form"]["pose"].as_u64().unwrap();
    let pose_count = current.m3["form"]["state_count"].as_u64().unwrap();
    assert!(pose_count > 1);
    let next_pose = u8::try_from((pose_index + 1) % pose_count).unwrap();
    rotated
        .m3_commands
        .last_mut()
        .unwrap()
        .operations
        .push(crate::m3_state::M3Operation::SetPose { pose: next_pose });
    let rotated = rotated.compose().unwrap();
    progress.mark("other_pose prepare begin");
    let other_pose =
        PerformanceOwner::prepare(&rotated, "expression:retained/current", config).unwrap();
    progress.mark("other_pose prepare complete");
    let owners = [
        &resident,
        &other_body,
        &other_face,
        &other_generation,
        &other_clock,
        &other_pose,
    ];

    let binary =
        std::env::var("QL_NATIVE_SOURCE_REPLY_TEST").expect("actual native reply driver required");
    let mut replies = Vec::with_capacity(6);
    for (index, owner) in owners.iter().enumerate() {
        progress.mark(&format!("case-{index} original-packet serialization begin"));
        let input = json!({"schema":"ql.source-reply-detecting-input/v1","case_index":index,"case":wire(owner)});
        let bytes = serde_json::to_vec(&input).unwrap();
        let destination = before_source_case(index, &bytes);
        progress.mark(&format!(
            "case-{index} original-packet serialization complete bytes={}",
            bytes.len()
        ));
        assert!(
            bytes.len() <= 16 * 1024 * 1024,
            "actual source case input bound"
        );
        progress.mark(&format!("case-{index} actual-native-child begin"));
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
        progress.mark(&format!(
            "case-{index} actual-native-child complete code={:?}",
            result.status.code()
        ));
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
    assert_eq!(replies.len(), 6);
    // All five original cases and the additional real pose are independently
    // valid complete native preparations; their source differences must refuse.
    // Rejecting it below is current resident identity, never malformed JSON.
    for (index, (owner, reply)) in owners.iter().zip(&replies).enumerate() {
        progress.mark(&format!(
            "case-{index} complete-source-reply-validation begin"
        ));
        owner.validate_reply(reply).unwrap();
        progress.mark(&format!(
            "case-{index} complete-source-reply-validation complete"
        ));
    }
    // Detect lost or contradictory roles using the complete actual
    // native preparation; no fabricated reply supplies a positive.
    for reply in &replies {
        assert_eq!(
            reply["reading"]["consumer_roles"]["legacy_mode_frequency_remapping"],
            "retired-in-this-explicit-physical-projection"
        );
    }
    let mut lost_role = replies[0].clone();
    lost_role["reading"]["consumer_roles"]
        .as_object_mut()
        .unwrap()
        .remove("legacy_mode_frequency_remapping");
    assert_eq!(
        resident.validate_reply(&lost_role).unwrap_err(),
        "native performance source consumer roles disconnected"
    );
    let mut wrong_role = replies[0].clone();
    wrong_role["reading"]["consumer_roles"]["legacy_mode_frequency_remapping"] =
        json!("operative-frequency-remapping");
    assert!(resident.validate_reply(&wrong_role).is_err());
    let mut wrong_projection = replies[0].clone();
    wrong_projection["reading"]["consumer_roles"]["physical"] =
        json!("reference-metric-scalar-excitation");
    assert!(resident.validate_reply(&wrong_projection).is_err());
    resident.validate_reply(&replies[0]).unwrap();
    resident.last = Some(replies[0].clone());
    for reply in &replies[1..] {
        assert!(resident.validate_reply(reply).is_err());
    }
    // A genuine physical pose change preserves the entire native M1 carrier
    // and changes its coupled MEF determination, including tuning and nodes.
    assert_eq!(rotated.m1, current.m1);
    assert_ne!(
        other_pose.binding().determination(),
        resident.binding().determination()
    );
    assert_eq!(
        replies[4]["reading"]["physical"]["eigenbasis_identity"],
        replies[0]["reading"]["physical"]["eigenbasis_identity"]
    );
    assert_ne!(
        replies[4]["payload"]["body_descriptor"]["physical_preparation"]["clock"],
        replies[0]["payload"]["body_descriptor"]["physical_preparation"]["clock"]
    );
    assert_ne!(
        replies[5]["reading"]["physical"]["eigenbasis_identity"],
        replies[0]["reading"]["physical"]["eigenbasis_identity"]
    );
    assert_ne!(
        replies[5]["payload"]["body_descriptor"]["physical_preparation"]["request"]["geometry"],
        replies[0]["payload"]["body_descriptor"]["physical_preparation"]["request"]["geometry"]
    );
    progress.mark("all-six-native-cases-and-complete-original-detectors complete");
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

// Host-only test diagnostics: never a native sample, admission or timing grant.
struct ActualSourceReplyProgress(std::time::Instant);
impl ActualSourceReplyProgress {
    fn mark(&self, stage: &str) {
        let utc_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("host diagnostic UTC clock")
            .as_millis();
        // Direct stderr survives a job timeout even when this unchanged
        // ordinary detector runs without --nocapture.
        writeln!(
            std::io::stderr().lock(),
            "native-source-reply-stage stage={stage} utc_unix_ms={utc_ms} elapsed_ms={}",
            self.0.elapsed().as_millis()
        )
        .expect("retain actual host source-reply progress");
    }
}
