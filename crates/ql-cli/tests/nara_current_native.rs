//! Actual executable and qualified Kerykeion/Swiss provider. No mock runtime,
//! recorded sky or expected quaternion fixture stands in for calculation.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};
fn invoke(operation: &str, request: &Value) -> std::process::Output {
    let mut p = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["nara", operation, "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    p.stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(request).unwrap())
        .unwrap();
    p.wait_with_output().unwrap()
}
fn sky() -> Value {
    json!({"schema":"ql.sky-request/v1","epoch":"2026-09-27T12:00:00Z","timezone":"UTC","mode":"historical","perspective":"Apparent Geocentric","zodiac":"Tropical","ayanamsha":null,"observer":null,"max_age_seconds":300,"backend_policy":"allow-moshier"})
}
fn profile(name: &str, date: &str) -> Value {
    json!({"schema":"ql.nara-identity-profile/v1","person_ref":format!("controlled:native-current:{name}"),"nara_ref":format!("controlled:nara:current:{name}"),"name":name,"encoding_policy":ql_mef::nara::identity_encoding::EncodingPolicy::default(),"composition_policy":"draft-core-birthdate-decanic-40-60-v1","birth":{"date":date,"time":"12:30:00","precision":"exact","uncertainty_minutes":null,"fold":null,"place":{"label":"London","latitude_degrees":51.5074,"longitude_degrees":-0.1278,"timezone":"Europe/London","source_ref":"controlled:entered-coordinate-source"}},"jungian":null,"gene_keys":null,"human_design":null,"quintessence":null})
}
fn value(output: std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
#[test]
fn same_real_dated_event_composes_two_distinct_native_identities_without_activity() {
    let a_profile = profile("First controlled native current", "1990-06-15");
    let a = value(invoke(
        "personal-current",
        &json!({"schema":"ql.nara-personal-current-request/v1","profile":a_profile,"sky_request":sky()}),
    ));
    let b_profile = profile("Second controlled native current", "2001-11-03");
    let b = value(invoke(
        "personal-current",
        &json!({"schema":"ql.nara-personal-current-request/v1","profile":b_profile,"sky_snapshot":a["transit"]["sky"]}),
    ));
    assert_eq!(a["snapshot_ref"], b["snapshot_ref"]);
    assert_eq!(a["transit"], b["transit"]);
    assert_ne!(a["q_identity"], b["q_identity"]);
    assert_ne!(a["q_identity_transit"], b["q_identity_transit"]);
    for (v, p) in [(&a, &a_profile), (&b, &b_profile)] {
        assert_eq!(&v["identity"]["profile"], p);
        assert_eq!(v["baseline_available"], true);
        assert!(v["q_activity"].is_null());
        assert!(v["q_composed"].is_null());
        assert_eq!(v["activity_status"], "unavailable");
        let a: ql_mef::nara::BioQuaternion =
            serde_json::from_value(v["q_identity"].clone()).unwrap();
        let t: ql_mef::nara::BioQuaternion =
            serde_json::from_value(v["transit"]["q_transit"].clone()).unwrap();
        let signed_dot = (a.w * t.w + a.x * t.x + a.y * t.y + a.z * t.z).clamp(-1., 1.);
        assert!((v["resonance"]["signed_dot"].as_f64().unwrap() - signed_dot).abs() < 1e-13);
        assert!((v["resonance"]["score"].as_f64().unwrap() - signed_dot.abs()).abs() < 1e-13);
        let expected = [
            a.w * t.w - a.x * t.x - a.y * t.y - a.z * t.z,
            a.w * t.x + a.x * t.w + a.y * t.z - a.z * t.y,
            a.w * t.y - a.x * t.z + a.y * t.w + a.z * t.x,
            a.w * t.z + a.x * t.y - a.y * t.x + a.z * t.w,
        ];
        let norm = expected.iter().map(|x| x * x).sum::<f64>().sqrt();
        for (i, k) in ["w", "x", "y", "z"].iter().enumerate() {
            assert!(
                (v["q_identity_transit"][k].as_f64().unwrap() - expected[i] / norm).abs() < 1e-13
            );
        }
    }
    let mut changed = a["transit"]["sky"].clone();
    changed["bodies"][0]["longitude_degrees"] = json!(12.);
    let bad = invoke(
        "personal-current",
        &json!({"schema":"ql.nara-personal-current-request/v1","profile":a_profile,"sky_snapshot":changed}),
    );
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("snapshot digest mismatch"));
}
#[test]
fn real_provider_refuses_stale_current_sky_before_publishing_a_transit() {
    let mut request = sky();
    request["mode"] = json!("current");
    request["epoch"] = json!("2000-01-01T00:00:00Z");
    let result = invoke("transit", &request);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("stale"));
    assert!(result.stdout.is_empty());
}

#[test]
fn actual_native_m3_activity_recomposes_same_identity_and_event_without_touching_natal() {
    let current = value(invoke(
        "personal-current",
        &json!({
        "schema":"ql.nara-personal-current-request/v1",
        "profile":profile("Activity native source", "1990-06-15"),"sky_request":sky()}),
    ));
    let registry = ql_mef::m_tree::native_m_registry()
        .manifest()
        .registry_revision
        .clone();
    let mut m3 = json!({"activity_policy":"historical-personal-frame-sprite-v1",
        "request":{"schema":"ql.m3-state-request/v1","registry_revision":registry,
            "stamp":{"identity":{"event_ref":current["snapshot_ref"],"profile_generation":0},
                "source_ref":current["input_revision"],"contract_ref":"ql.nara-personal-current/v1"},
            "subject_ref":current["person_ref"],"occurrence_unix_ms":10,"receipt_unix_ms":11,
            "clock_steps":359,"address":0,"pose":0,"aperture":0,"matrix_axis":0,"rna":false,
            "bases":[{"role":"coordinate","reference":"#3-1-1-1","revision":registry},
                {"role":"identity-source","reference":"controlled:source:activity","revision":current["input_revision"]}],"m2_basis":null},
        "commands":[]});
    let operation = |generation: u64, address: u8| {
        json!({"schema":"ql.m3-command/v1",
        "event_ref":current["snapshot_ref"],"subject_ref":current["person_ref"],
        "expected_generation":generation,"actor_ref":"controlled:native-owner","cause_ref":"controlled:explicit-selection",
        "occurrence_unix_ms":20+generation*1800000,"receipt_unix_ms":21+generation*1800000,
        "operations":[{"operation":"select-form","address":address}]})
    };
    let recompose = |m3: &Value| {
        value(invoke(
            "personal-recompose",
            &json!({
        "schema":"ql.nara-personal-recompose-request/v1","current":current,"m3_input":m3}),
        ))
    };
    let empty = recompose(&m3);
    assert_eq!(empty["activity_status"], "unavailable");
    assert!(empty["q_activity"].is_null());
    m3["commands"] = json!([operation(0, 56)]);
    let first = recompose(&m3);
    m3["commands"] = json!([operation(0, 56), operation(1, 1)]);
    let second = recompose(&m3);
    assert_eq!(first["activity"]["turn_count"], 1);
    assert_eq!(second["activity"]["turn_count"], 2);
    assert_ne!(first["q_activity"], second["q_activity"]);
    assert_ne!(first["q_composed"], second["q_composed"]);
    for output in [&first, &second] {
        assert_eq!(output["activity_status"], "available");
        for field in [
            "identity",
            "transit",
            "q_identity",
            "q_identity_transit",
            "snapshot_ref",
        ] {
            assert_eq!(output[field], current[field]);
        }
        let baseline: ql_mef::nara::BioQuaternion =
            serde_json::from_value(current["q_identity_transit"].clone()).unwrap();
        let activity: ql_mef::nara::BioQuaternion =
            serde_json::from_value(output["q_activity"].clone()).unwrap();
        assert_eq!(
            output["q_composed"],
            json!(baseline.compose(activity).unwrap())
        );
    }
    m3["request"]["subject_ref"] = json!("person:wrong");
    assert!(!invoke("personal-recompose", &json!({"schema":"ql.nara-personal-recompose-request/v1","current":current,"m3_input":m3})).status.success());
}
