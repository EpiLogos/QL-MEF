use ql_mef::m3_state::*;
use serde_json::{Value, json};
use std::{path::Path, process::Command};
fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-v1.json"
    ))
    .unwrap()
}
fn setup() -> (M3State, Vec<M3Command>) {
    let f = fixture();
    (
        M3State::new(serde_json::from_value(f["request"].clone()).unwrap()).unwrap(),
        serde_json::from_value(f["commands"].clone()).unwrap(),
    )
}
#[test]
fn all_three_consumers_share_one_event_generation_clock_and_source_return() {
    let (mut s, commands) = setup();
    let before = s.snapshot();
    assert_eq!(before["clock"]["degree720"], 359);
    let first = s.apply(commands[0].clone()).unwrap();
    let f = s.snapshot();
    assert_eq!(f["form"]["address"], 56);
    assert_eq!(f["clock"]["degree720"], 360);
    assert_eq!(f["clock"]["layer"], 1);
    assert_eq!(f["aperture"]["fibonacci_phase60"], 0);
    assert_eq!(f["subject_ref"], before["subject_ref"]);
    assert_eq!(f["identity"]["profile_generation"], 8);
    assert_eq!(f["bases"], before["bases"]);
    assert_eq!(f["bases"]["m2"]["identity"]["profile_generation"], 7); // Original input generation, not relabelled freshness.
    assert_eq!(first.before, before);
    assert_eq!(first.after, f);
    for consumer in ["epi.cosmic.123", "epi.personal.450", "epi.deep.m3"] {
        assert!(
            f["consumers"]
                .as_array()
                .unwrap()
                .contains(&json!(consumer))
        );
    }
    assert_eq!(
        f["elemental"]["component_order"],
        json!(["Earth", "Fire", "Water", "Air"])
    );
    assert_eq!(f["aperture"]["total_lenses"], 18);
    for command in commands.into_iter().skip(1) {
        s.apply(command).unwrap();
    }
    assert_eq!(s.snapshot()["clock"]["completed_double_covers"], 1);
    let id = serde_json::from_value(s.snapshot()["form"]["codon"]["id"].clone()).unwrap();
    assert!(s.source_node(id).is_some());
}
#[test]
fn replay_preserves_original_occurrence_receipt_and_owner_bases() {
    let (mut a, commands) = setup();
    let (mut b, _) = setup();
    for c in commands {
        let ra = serde_json::to_value(a.apply(c.clone()).unwrap()).unwrap();
        let rb = serde_json::to_value(b.apply(c).unwrap()).unwrap();
        assert_eq!(ra, rb);
    }
    assert_eq!(a.snapshot(), b.snapshot());
    let s = a.snapshot();
    let bases = s["bases"]["owner_readings"].as_array().unwrap();
    for role in [
        "identity-hash",
        "identity-quaternion",
        "live-composition",
        "bioquaternion-reading",
        "human-Day",
        "agent-NOW",
    ] {
        assert!(bases.iter().any(|b| b["role"] == role));
    }
}
#[test]
fn stale_event_subject_generation_and_bad_batches_never_mutate() {
    let (mut s, c) = setup();
    let before = s.snapshot();
    for k in [
        "schema",
        "event_ref",
        "subject_ref",
        "actor_ref",
        "cause_ref",
    ] {
        let mut v = serde_json::to_value(&c[0]).unwrap();
        v[k] = json!("");
        let bad = serde_json::from_value(v).unwrap();
        assert!(s.apply(bad).is_err());
        assert_eq!(s.snapshot(), before);
    }
    let mut bad = c[0].clone();
    bad.expected_generation += 1;
    assert!(s.apply(bad).is_err());
    for operation in [
        M3Operation::SelectForm { address: 64 },
        M3Operation::ChangeLine { line: 6 },
        M3Operation::SetPose { pose: 8 },
        M3Operation::SetAperture { aperture: 16 },
        M3Operation::ApplyMatrix { family: 3 },
        M3Operation::AdvanceClock { steps: u64::MAX },
    ] {
        let mut bad = c[0].clone();
        bad.operations = vec![M3Operation::AdvanceClock { steps: 1 }, operation];
        assert!(s.apply(bad).is_err());
        assert_eq!(s.snapshot(), before);
    }
    let mut bad = c[0].clone();
    bad.operations.clear();
    assert!(s.apply(bad).is_err());
    s.apply(c[0].clone()).unwrap();
    let after = s.snapshot();
    assert!(s.apply(c[0].clone()).is_err());
    assert_eq!(s.snapshot(), after);
}
#[test]
fn matrix_gap_rolls_back_the_whole_batch() {
    let (mut s, c) = setup();
    let before = s.snapshot();
    let mut cmd = c[0].clone();
    cmd.operations = vec![
        M3Operation::SelectForm { address: 5 },
        M3Operation::AdvanceClock { steps: 1 },
        M3Operation::ApplyMatrix { family: 2 },
    ];
    let r = s.apply(cmd).unwrap();
    assert_eq!(r.status, "provisional-unchanged");
    assert_eq!(r.failed_operation, Some(2));
    assert_eq!(r.before, r.after);
    assert_eq!(s.snapshot(), before);
}
#[test]
fn input_rejects_wrong_registry_mixed_generation_and_collapsed_identity_roles() {
    for mutation in 0..5 {
        let mut v = fixture()["request"].clone();
        match mutation {
            0 => v["registry_revision"] = json!("wrong"),
            1 => v["m2_basis"]["identity"]["profile_generation"] = json!(8),
            2 => v["address"] = json!(64),
            3 => v["bases"][1]["role"] = v["bases"][0]["role"].clone(),
            _ => v["clock_steps"] = json!(u64::MAX),
        }
        assert!(M3State::new(serde_json::from_value(v).unwrap()).is_err());
    }
    let mut bad = fixture()["request"].clone();
    bad["unexpected"] = json!(true);
    assert!(serde_json::from_value::<M3Request>(bad).is_err());
}
#[test]
fn every_address_line_change_and_transcription_is_operational() {
    for a in 0..64 {
        for line in 0..6 {
            let mut f = fixture();
            f["request"]["address"] = json!(a);
            f["request"]["pose"] = json!(0);
            let mut s =
                M3State::new(serde_json::from_value(f["request"].clone()).unwrap()).unwrap();
            let mut cmd: M3Command = serde_json::from_value(f["commands"][0].clone()).unwrap();
            cmd.operations = vec![
                M3Operation::ChangeLine { line },
                M3Operation::Transcribe { rna: true },
            ];
            s.apply(cmd).unwrap();
            let v = s.snapshot();
            assert_eq!(v["form"]["address"], a ^ (1 << line));
            assert!(
                !v["transcription"]["sequence"]
                    .as_str()
                    .unwrap()
                    .contains('T')
            );
        }
    }
}
fn run(command: &mut Command) -> Vec<u8> {
    let r = command.output().unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    r.stdout
}
#[test]
fn installed_cpp_consumer_receives_the_same_changed_subject_and_full_form() {
    let (mut state, commands) = setup();
    state.apply(commands[0].clone()).unwrap();
    let packet = state.snapshot();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m3-parent-consumer");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(
        out.join("handoff.json"),
        serde_json::to_vec_pretty(&packet).unwrap(),
    )
    .unwrap();
    run(Command::new("make")
        .current_dir(&root)
        .args(["-C", "c", "install"])
        .arg(format!("BUILD_DIR={}", out.join("build").display()))
        .arg(format!("DESTDIR={}", out.join("install").display()))
        .arg("PREFIX=/ql"));
    let id = |v: &Value| format!("UINT64_C(0x{})", v.as_str().unwrap());
    let values = |v: &Value| {
        v.as_array()
            .unwrap()
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join(",")
    };
    let cpp = format!(
        r#"#include <ql/m3_domain.h>
#include <cassert>
#include <cstring>
int main(){{
 const char *event={event};const char *subject={subject};const unsigned generation={generation};
 assert(std::strcmp(event,"fixture:current-event")==0 && std::strcmp(subject,"fixture:nara-subject")==0 && generation==8);
 QL_M3_Form form{{}};assert(ql_m3_form({address},{pose},{aperture},{phase},{axis},&form)==QL_M3_OK);
 const int angles[3]={{{angles}}};const int velocity[3]={{{velocity}}};const int counts[4]={{{counts}}};
 for(unsigned i=0;i<3;i++){{assert(form.angles_deg10[i]==angles[i]);assert(form.velocities_deg10[i]==velocity[i]);}}
 for(unsigned i=0;i<4;i++)assert(form.elemental_counts[i]==counts[i]);
 QL_M3_Clock clock{{}};assert(ql_m3_clock({steps},&clock)==QL_M3_OK && clock.degree720==360 && clock.layer==1);
 assert(clock.degree_node=={degree} && clock.backbone_node=={backbone});
 auto *node=ql_m3_node(QL_M3_CODON,{address});assert(node && node->id=={codon});
 QL_M3_BackboneProjection p{{}};assert(ql_m3_clock_projection({steps},&p)==QL_M3_OK);assert(p.codon=={proto_codon} && p.hexagram=={proto_hex});
 assert(std::strcmp(ql_m3_domain_revision(),{revision})==0);
 char rna[4];assert(ql_m3_transcribe({address},1,rna)==QL_M3_OK && std::strcmp(rna,{rna})==0);
 // Independent read/deep-open uses the same supplied steps, never starts at 0.
 QL_M3_Clock deep{{}};assert(ql_m3_clock({steps},&deep)==QL_M3_OK && deep.steps==clock.steps);
}}
"#,
        event = packet["identity"]["event_ref"],
        subject = packet["subject_ref"],
        generation = packet["identity"]["profile_generation"],
        address = packet["form"]["address"],
        pose = packet["form"]["pose"],
        aperture = packet["aperture"]["index"],
        phase = packet["aperture"]["fibonacci_phase60"],
        axis = packet["form"]["matrix_axis"],
        angles = values(&packet["form"]["angles_deg10"]),
        velocity = values(&packet["form"]["velocities_deg10"]),
        counts = values(&packet["elemental"]["counts"]),
        steps = packet["clock"]["steps"],
        degree = id(&packet["clock"]["degree"]["id"]),
        backbone = id(&packet["clock"]["backbone"]["id"]),
        codon = id(&packet["form"]["codon"]["id"]),
        proto_codon = id(&packet["clock"]["source_backbone_prototype"]["codon_id"]),
        proto_hex = id(&packet["clock"]["source_backbone_prototype"]["hexagram_id"]),
        revision = packet["domain_revision"],
        rna = packet["transcription"]["sequence"]
    );
    std::fs::write(out.join("consumer.cpp"), cpp).unwrap();
    run(Command::new("c++")
        .args(["-std=c++17", "-Wall", "-Wextra", "-Werror", "-pedantic"])
        .arg(format!("-I{}", out.join("install/ql/include").display()))
        .arg(out.join("consumer.cpp"))
        .arg(out.join("install/ql/lib/libql-mef-c.a"))
        .args(["-lm", "-o"])
        .arg(out.join("consumer")));
    run(Command::new(out.join("consumer")).current_dir("/tmp"));
}
