//! Real public CLI against the embedded source registry and transactional owner.
//! Controlled command selections are not evidence of a personal session close.
use ql_mef::m3_state::{M3Command, M3Request, M3State};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn invoke(input: &Value) -> Result<Value, String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["kernel", "m3", "-", "--json"])
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
    let output = child.wait_with_output().unwrap();
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    Ok(serde_json::from_slice(&output.stdout).unwrap())
}

fn input() -> Value {
    let revision = ql_mef::m_tree::native_m_registry()
        .manifest()
        .registry_revision
        .clone();
    json!({"request":{
        "schema":"ql.m3-state-request/v1","registry_revision":revision,
        "stamp":{"identity":{"event_ref":"event:controlled-m3-native","profile_generation":0},
            "source_ref":"source:explicit-instrument-selections","contract_ref":"ql.m3-state-request/v1"},
        "subject_ref":"person:controlled-m3-native","occurrence_unix_ms":10,"receipt_unix_ms":11,
        "clock_steps":359,"address":0,"pose":0,"aperture":0,"matrix_axis":0,"rna":false,
        "bases":[],"m2_basis":null},"commands":[]})
}

fn command(generation: u64, operation: Value) -> Value {
    json!({"schema":"ql.m3-command/v1","event_ref":"event:controlled-m3-native",
        "subject_ref":"person:controlled-m3-native","expected_generation":generation,
        "actor_ref":"actor:controlled-native-test","cause_ref":format!("selection:{generation}"),
        "occurrence_unix_ms":20+generation,"receipt_unix_ms":21+generation,"operations":[operation]})
}

#[test]
fn supported_transport_preserves_complete_native_receipts_and_order() {
    let mut input = input();
    input["commands"] = json!([
        command(0, json!({"operation":"select-form","address":56})),
        command(1, json!({"operation":"advance-clock","steps":1})),
        command(2, json!({"operation":"transcribe","rna":true}))
    ]);
    let request: M3Request = serde_json::from_value(input["request"].clone()).unwrap();
    let commands: Vec<M3Command> = serde_json::from_value(input["commands"].clone()).unwrap();
    let mut owner = M3State::new(request).unwrap();
    let receipts: Vec<Value> = commands
        .into_iter()
        .map(|c| serde_json::to_value(owner.apply(c).unwrap()).unwrap())
        .collect();
    let actual = invoke(&input).unwrap();
    assert_eq!(
        actual,
        json!({"state":owner.snapshot(),"receipts":receipts})
    );
    assert_eq!(actual["state"]["clock"]["degree720"], 360);
    assert_eq!(actual["state"]["form"]["address"], 56);
    assert_eq!(
        actual["receipts"][0]["after"],
        actual["receipts"][1]["before"]
    );
    assert_eq!(actual["state"]["identity"]["profile_generation"], 3);
    assert_eq!(actual, invoke(&input).unwrap());
}

#[test]
fn stale_or_cross_subject_commands_cannot_produce_a_reading() {
    for (field, bad) in [
        ("expected_generation", json!(1)),
        ("subject_ref", json!("person:other")),
        ("event_ref", json!("event:other")),
    ] {
        let mut input = input();
        let mut c = command(0, json!({"operation":"change-line","line":0}));
        c[field] = bad;
        input["commands"] = json!([c]);
        assert!(
            invoke(&input)
                .unwrap_err()
                .contains("M3 command contract/event/subject/generation mismatch")
        );
    }
}

#[test]
fn initial_selections_and_source_revision_are_never_defaulted() {
    let mut input = input();
    input["request"]
        .as_object_mut()
        .unwrap()
        .remove("clock_steps");
    assert!(
        invoke(&input)
            .unwrap_err()
            .contains("missing field `clock_steps`")
    );
    let mut input = self::input();
    input["request"]["registry_revision"] = json!("unobserved-source");
    assert!(
        invoke(&input)
            .unwrap_err()
            .contains("M3 contract or registry mismatch")
    );
    let mut input = self::input();
    input["supplied_activity"] = json!("not-this-producer");
    assert!(invoke(&input).unwrap_err().contains("unknown field"));
}

fn selected_activity_input() -> Value {
    let mut input = input();
    input["activity_policy"] = json!("historical-personal-frame-sprite-v1");
    input["request"]["bases"] = json!([
        {"role":"coordinate","reference":"#3-1-1-1","revision":input["request"]["registry_revision"]},
        {"role":"identity-source","reference":"source:controlled-person","revision":"controlled:r1"}]);
    input
}
#[test]
fn explicit_activity_uses_actual_ordered_native_receipts_without_identity_mutation() {
    let mut input = selected_activity_input();
    let empty = invoke(&input).unwrap();
    assert_eq!(empty["activity"]["status"], "unavailable");
    assert!(empty["activity"]["q_activity"].is_null());
    input["commands"] = json!([
        command(0, json!({"operation":"select-form","address":56})),
        command(1, json!({"operation":"select-form","address":1}))
    ]);
    let first = invoke(&input).unwrap();
    assert_eq!(first["activity"]["turn_count"], 2);
    assert_eq!(first["activity"]["packets"][0]["kairos_delta"], 0.);
    assert_eq!(
        first["activity"]["packets"][1]["vak_address"]["ct"],
        json!([
            first["receipts"][0]["after"]["transcription"]["sequence"],
            first["receipts"][1]["after"]["transcription"]["sequence"]
        ])
    );
    assert!(
        first["activity"]["packets"][0]["vak_address"]["cs"]
            .get("recognized")
            .is_none()
    );
    assert_eq!(first["activity"]["legacy_contemplation_close"], false);
    assert_eq!(first["activity"]["identity_mutated"], false);
    assert_eq!(first, invoke(&input).unwrap());
    input["commands"][0]["operations"][0]["address"] = json!(1);
    input["commands"][1]["operations"][0]["address"] = json!(56);
    let second = invoke(&input).unwrap();
    assert_ne!(
        first["activity"]["q_activity"],
        second["activity"]["q_activity"]
    );
    assert_eq!(
        first["state"]["subject_ref"],
        second["state"]["subject_ref"]
    );
    assert_eq!(first["state"]["bases"], second["state"]["bases"]);
}
#[test]
fn provisional_native_gap_never_emits_an_activity_packet() {
    let mut input = selected_activity_input();
    let mut c = command(0, json!({"operation":"select-form","address":5}));
    c["operations"] =
        json!([{ "operation":"select-form","address":5},{"operation":"apply-matrix","family":2}]);
    input["commands"] = json!([c]);
    let result = invoke(&input).unwrap();
    assert_eq!(result["receipts"][0]["status"], "provisional-unchanged");
    assert_eq!(result["activity"]["turn_count"], 0);
    assert!(result["activity"]["q_activity"].is_null());
}

#[test]
fn native_hinge_geometry_preserves_exact_pair_angles_for_all_forms() {
    for address in 0..64 {
        let mut request = input()["request"].clone();
        request["address"] = json!(address);
        let state = M3State::new(serde_json::from_value(request).unwrap())
            .unwrap()
            .snapshot();
        let geometry = &state["form"]["hinge_geometry"];
        assert_eq!(geometry["shared_hinge"], "Y");
        assert_eq!(geometry["points"][1]["xyz"], json!([0., 0., 0.]));
        for (index, (from, to)) in [(0, 1), (1, 2)].into_iter().enumerate() {
            let point =
                |id: usize, axis: usize| geometry["points"][id]["xyz"][axis].as_f64().unwrap();
            let dx = point(to, 0) - point(from, 0);
            let dy = point(to, 1) - point(from, 1);
            let angle = geometry["segments"][index]["angle_deg10"].as_f64().unwrap() / 10.;
            assert!((dx * dx + dy * dy - 1.).abs() < 1e-12);
            assert!((dx - angle.to_radians().cos()).abs() < 1e-12);
            assert!((dy - angle.to_radians().sin()).abs() < 1e-12);
            assert_eq!(
                geometry["segments"][index]["angle_deg10"],
                state["form"]["pair_angles_deg10"][index]
            );
        }
    }
    let actual = invoke(&input()).unwrap();
    assert_eq!(
        actual["state"]["form"]["hinge_geometry"]["schema"],
        "ql.m3-hinge-presentation/v1"
    );
}

#[test]
fn selecting_activity_policy_at_native_generation_never_admits_previous_receipts() {
    let mut input = selected_activity_input();
    input["commands"] = json!([command(0, json!({"operation":"select-form","address":56}))]);
    input["activity_start_generation"] = json!(1);
    let selected = invoke(&input).unwrap();
    assert_eq!(selected["state"]["identity"]["profile_generation"], 1);
    assert_eq!(selected["activity"]["turn_count"], 0);
    assert!(selected["activity"]["q_activity"].is_null());
    input["commands"]
        .as_array_mut()
        .unwrap()
        .push(command(1, json!({"operation":"select-form","address":1})));
    let result = invoke(&input).unwrap();
    assert_eq!(result["activity"]["turn_count"], 1);
    assert_eq!(result["activity"]["packets"][0]["before_generation"], 1);
    assert_eq!(
        result["activity"]["packets"][0]["vak_address"]["ct"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    input["activity_start_generation"] = json!(3);
    assert!(
        invoke(&input)
            .unwrap_err()
            .contains("existing native generation")
    );
}
