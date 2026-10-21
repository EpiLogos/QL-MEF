//! `ql context-frame read`: the External Context Frame reading through the
//! real front-door binary. It reads a supplied sixfold mapping and a
//! structural probe, and answers exactly what `read_external_context_frame`
//! answers. It confers nothing on the target, and refusals come in the
//! library's own words.
use ql_mef::{
    ContextFrameReadingOrigin, ContextFrameStructuralProbe, ExternalSixfoldMapping, MefGrain,
    MefUnitFace, read_external_context_frame,
};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn request(probe: Value) -> Value {
    json!({
        "target_ref": "target/external-sixfold",
        "mapping_source_ref": "source/external-sixfold/v1",
        "external_position_refs": ["a", "b", "c", "d", "e", "f"],
        "probe": probe,
        "provider_ref": "provider/ql-mef",
        "origin": "derived",
        "evidence_refs": ["evidence/external-structure/1"]
    })
}

/// `ql context-frame read - --json` with the request on stdin.
fn read(request: &Value) -> (bool, String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["context-frame", "read", "-", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn reading(request: &Value) -> Value {
    let (ok, stdout, stderr) = read(request);
    assert!(ok, "{stderr}");
    serde_json::from_str(&stdout).unwrap()
}

#[test]
fn an_exact_probe_reads_the_named_frame_the_library_reads() {
    let answer = reading(&request(
        json!({"local_position": 3, "unit_face": "power", "grain": "inner-four"}),
    ));
    assert_eq!(
        answer["contract"],
        json!("ql.mef.context-frame-reading/1.0.0")
    );
    assert_eq!(
        answer["status"],
        json!({"kind": "exact", "frame": {"code": "CF5", "expression": "(4.0/1-4.4/5)",
            "name": "Fractal-Doubling Executive"}})
    );
    assert_eq!(answer["runtime_authority"], json!(false));
    assert_eq!(answer["origin"], json!("derived"));
    assert_eq!(
        answer["probe"],
        json!({"local_position": 3, "unit_face": "power", "grain": "inner-four"})
    );
    assert_eq!(
        answer["evidence_refs"],
        json!(["evidence/external-structure/1"])
    );
    // The same reading through the library: digest and operator version agree.
    let mapping = ExternalSixfoldMapping::new(
        "target/external-sixfold",
        "source/external-sixfold/v1",
        ["a", "b", "c", "d", "e", "f"].map(str::to_string),
    )
    .unwrap();
    let library = read_external_context_frame(
        &mapping,
        ContextFrameStructuralProbe::new(3, Some(MefUnitFace::Power), Some(MefGrain::InnerFour))
            .unwrap(),
        "provider/ql-mef",
        ContextFrameReadingOrigin::Derived,
        vec!["evidence/external-structure/1".into()],
    )
    .unwrap();
    assert_eq!(answer["mapping_digest"], json!(library.mapping_digest()));
    assert_eq!(
        answer["operator_version"],
        json!(library.operator_version())
    );
    assert_eq!(
        answer["external_position_refs"],
        json!(["a", "b", "c", "d", "e", "f"])
    );
}

#[test]
fn partial_ambiguous_and_absent_readings_are_named_never_guessed() {
    let partial = reading(&request(json!({"local_position": 2, "unit_face": "name"})));
    assert_eq!(partial["status"]["kind"], json!("partial"));
    assert_eq!(partial["status"]["frame"]["code"], json!("CF3"));
    assert_eq!(partial["status"]["missing_face"], json!(false));
    assert_eq!(partial["status"]["missing_grain"], json!(true));
    assert_eq!(partial["probe"]["grain"], Value::Null);

    let ambiguous = reading(&request(json!({"local_position": 2})));
    assert_eq!(ambiguous["status"]["kind"], json!("ambiguous"));
    let codes: Vec<&str> = ambiguous["status"]["frames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|frame| frame["code"].as_str().unwrap())
        .collect();
    assert_eq!(codes, ["CF3", "CF4"]);

    let none = reading(&request(json!({"local_position": 0, "unit_face": "power"})));
    assert_eq!(none["status"], json!({"kind": "no-reading"}));
}

#[test]
fn refusals_are_the_librarys_own_words() {
    let mut duplicate = request(json!({"local_position": 1}));
    duplicate["external_position_refs"] = json!(["a", "a", "c", "d", "e", "f"]);
    let (ok, _, stderr) = read(&duplicate);
    assert!(!ok);
    assert!(
        stderr.contains("DUPLICATE_EXTERNAL_POSITION_REF:a"),
        "{stderr}"
    );

    let (ok, _, stderr) = read(&request(json!({"local_position": 6})));
    assert!(!ok);
    assert!(stderr.contains("INVALID_POSITION:6"), "{stderr}");

    let mut unknown = request(json!({"local_position": 1}));
    unknown["authority"] = json!("granted");
    let (ok, _, stderr) = read(&unknown);
    assert!(!ok, "an unknown field is refused, not ignored");
    assert!(stderr.contains("unknown field `authority`"), "{stderr}");

    let mut empty = request(json!({"local_position": 1}));
    empty["provider_ref"] = json!(" ");
    let (ok, _, stderr) = read(&empty);
    assert!(!ok);
    assert!(stderr.contains("EMPTY_PROVIDER_REF"), "{stderr}");
}

#[test]
fn a_request_file_reads_the_same_as_stdin_and_plain_text_says_what_it_is() {
    let dir = std::env::temp_dir().join(format!("ql-cf-read-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("request.json");
    let body = request(json!({"local_position": 5, "unit_face": "power", "grain": "outer-two"}));
    std::fs::write(&path, body.to_string()).unwrap();
    let from_file = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["context-frame", "read"])
        .arg(&path)
        .arg("--json")
        .output()
        .unwrap();
    assert!(from_file.status.success());
    let from_file: Value = serde_json::from_slice(&from_file.stdout).unwrap();
    assert_eq!(from_file, reading(&body));
    assert_eq!(from_file["status"]["frame"]["code"], json!("CF7"));
    let plain = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["context-frame", "read"])
        .arg(&path)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&plain.stdout);
    assert!(text.contains("exact: CF7"), "{text}");
    assert!(text.contains("descriptive evidence only"), "{text}");
    let usage = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["context-frame", "read"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&usage.stderr)
            .contains("usage: ql context-frame read <request.json|-> [--json]")
    );
    let _ = std::fs::remove_dir_all(&dir);
}
