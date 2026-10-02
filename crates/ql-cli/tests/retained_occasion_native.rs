//! Replay the actual saved ordinary-entry occasion through the real CLI and
//! embedded provider. The positive inputs are retained native owner artifacts,
//! not a fabricated snapshot or transport. Scope: admission and native dated
//! computation, not installed rendering, acoustic output or personal history.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn artifact(name: &str) -> Value {
    let path = std::env::var(name).expect("actual retained native artifact is required");
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn invoke(owner: &str, operation: &str, input: &Value) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args([owner, operation, "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(input).unwrap())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn accepted(out: Output) -> Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
fn refused(out: Output, reason: &str) {
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains(reason),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
fn receipt(value: &Value, sky: &Value) {
    let a = &value["sky_admission"];
    assert_eq!(a["schema"], "ql.sky-admission/v1");
    assert_eq!(a["purpose"], "retained-occasion");
    assert_eq!(a["snapshot_ref"], sky["snapshot_ref"]);
    assert_eq!(a["original_mode"], "current");
    assert_eq!(a["epoch_utc"], sky["epoch_utc"]);
    assert_eq!(a["receipt_utc"], sky["receipt_utc"]);
    assert_eq!(a["fresh_current_attested"], false);
    assert_eq!(
        a["validation"],
        "immutable-snapshot-and-current-native-source"
    );
    assert!(
        a["standing"]
            .as_str()
            .unwrap()
            .contains("no fresh-current attestation")
    );
}

#[test]
#[ignore = "Requires QL_RETAINED_WORLD_FILE and both QL_RETAINED_IDENTITY_{A,B}_FILE actual native owner artifacts"]
fn actual_aged_current_origin_reopens_same_cosmic_and_two_personal_dated_bases() {
    let prepared = artifact("QL_RETAINED_WORLD_FILE");
    let world = &prepared["response"]["outcome"]["data"]["source"]["world"];
    assert_eq!(world["schema"], "ql.scene-world/v1");
    let sky = &world["sky"];
    assert_eq!(sky["request"]["mode"], "current");
    let original = serde_json::to_vec(sky).unwrap();
    let compose = json!({"schema":"ql.scene-request/v1", "sky_snapshot":sky,
        "tick12":world["starting_recipe"]["settings"]["tick12"],
        "cycle":world["starting_recipe"]["settings"]["cycle"]});
    refused(
        invoke("scene", "compose", &compose),
        "not fresh current sky",
    );
    let mut retained = compose.clone();
    retained["snapshot_purpose"] = json!("retained-occasion");
    let scene = accepted(invoke("scene", "compose", &retained));
    receipt(&scene, sky);
    let tick = compose["tick12"].as_u64().unwrap() as u8;
    let cycle = compose["cycle"].as_u64().unwrap();
    let expected_scene = ql_mef::scene::compose(sky, tick, cycle, Default::default()).unwrap();
    let mut without_admission = scene.clone();
    without_admission
        .as_object_mut()
        .unwrap()
        .remove("sky_admission");
    assert_eq!(
        without_admission, expected_scene,
        "admission cannot change native scene computation"
    );
    let mut world_input = json!({"schema":"ql.scene-world-request/v1",
        "instance_ref":world["instance_ref"], "event_ref":world["event_ref"],
        "subject_ref":world["subject_ref"], "sky":sky,
        "texture":prepared["request"]["request"]["request"]["texture"],
        "units_per_metre":prepared["request"]["request"]["request"]["units_per_metre"],
        "start":world["starting_recipe"]["settings"]});
    refused(
        invoke("scene", "world", &world_input),
        "not fresh current sky",
    );
    let expected_world =
        ql_mef::scene::world(serde_json::from_value(world_input.clone()).unwrap()).unwrap();
    world_input["snapshot_purpose"] = json!("retained-occasion");
    let reopened = accepted(invoke("scene", "world", &world_input));
    receipt(&reopened, sky);
    let mut no_receipt = reopened.clone();
    no_receipt.as_object_mut().unwrap().remove("sky_admission");
    assert_eq!(
        no_receipt, expected_world,
        "retained purpose changes no native determinant/output"
    );
    assert_eq!(reopened["event_ref"], sky["snapshot_ref"]);
    assert_eq!(serde_json::to_vec(&reopened["sky"]).unwrap(), original);
    assert_eq!(reopened["basis"], reopened["binding"]["native_basis"]);

    let mut readings = Vec::new();
    for name in ["QL_RETAINED_IDENTITY_A_FILE", "QL_RETAINED_IDENTITY_B_FILE"] {
        let saved = artifact(name);
        let profile = &saved["reading"]["profile"];
        assert_eq!(profile["schema"], "ql.nara-identity-profile/v1");
        let mut input = json!({"schema":"ql.nara-personal-current-request/v1",
            "profile":profile, "sky_snapshot":sky});
        refused(
            invoke("nara", "personal-current", &input),
            "not fresh current sky",
        );
        input["snapshot_purpose"] = json!("retained-occasion");
        let personal = accepted(invoke("nara", "personal-current", &input));
        receipt(&personal, sky);
        assert_eq!(personal["person_ref"], profile["person_ref"]);
        assert_eq!(personal["nara_ref"], profile["nara_ref"]);
        assert_eq!(
            serde_json::to_vec(&personal["transit"]["sky"]).unwrap(),
            original
        );
        assert_eq!(
            personal["transit"],
            ql_mef::nara::current::transit(Some(sky)).unwrap()
        );
        let native_baseline =
            ql_mef::nara::current::personal_current(&personal["identity"], &personal["transit"])
                .unwrap();
        let mut no_admission = personal.clone();
        no_admission
            .as_object_mut()
            .unwrap()
            .remove("sky_admission");
        assert_eq!(no_admission, native_baseline);
        assert!(personal["q_activity"].is_null());
        assert!(personal["q_composed"].is_null());
        input.as_object_mut().unwrap().remove("sky_snapshot");
        input["sky_request"] = sky["request"].clone();
        refused(
            invoke("nara", "personal-current", &input),
            "requires an existing exact sky snapshot",
        );
        readings.push(personal);
    }
    assert_ne!(readings[0]["person_ref"], readings[1]["person_ref"]);
    assert_ne!(readings[0]["q_identity"], readings[1]["q_identity"]);
    assert_ne!(
        readings[0]["q_identity_transit"],
        readings[1]["q_identity_transit"]
    );
    assert_eq!(readings[0]["transit"], readings[1]["transit"]);
    let mut bad = retained.clone();
    bad["sky_snapshot"]["bodies"][0]["longitude_degrees"] = json!(12.);
    refused(invoke("scene", "compose", &bad), "snapshot digest mismatch");
    bad = retained.clone();
    bad["sky_snapshot"] = serde_json::from_slice(include_bytes!(
        "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
    ))
    .unwrap();
    refused(
        invoke("scene", "compose", &bad),
        "native source binding is stale",
    );
    bad = retained.clone();
    bad["snapshot_purpose"] = json!("not-a-purpose");
    refused(invoke("scene", "compose", &bad), "unknown variant");
    bad = retained.clone();
    bad["sky_request"] = sky["request"].clone();
    refused(invoke("scene", "compose", &bad), "exactly one");
    bad = retained;
    bad.as_object_mut().unwrap().remove("sky_snapshot");
    bad["sky_request"] = sky["request"].clone();
    refused(
        invoke("scene", "compose", &bad),
        "requires an existing exact sky snapshot",
    );
    world_input["event_ref"] = json!("sha256:wrong-event");
    refused(
        invoke("scene", "world", &world_input),
        "admitted sky snapshot reference",
    );
}
