//! `ql wiki refract`: one Wiki refraction request through the real front-door
//! binary. It answers exactly what the library's shape-aware engine answers over
//! the MEF registry provider (the composition `ql techne reading` runs with its
//! one pinned lens), through the lenses and sublenses the caller names, and
//! refuses in the library's own words.
use ql_wiki::{RegistryDisclosureProvider, ShapeAwareWikiRefractionEngine, WikiRefractionRequest};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

const REQUEST: &str = include_str!("../../../fixtures/qw2/wiki-refraction-request.json");

/// `ql wiki refract - [--json]` with the request on stdin.
fn run(request: &Value, json: bool) -> (bool, String, String) {
    let mut args = vec!["wiki", "refract", "-"];
    if json {
        args.push("--json");
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(&args)
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

fn library(request: &Value) -> Value {
    let request: WikiRefractionRequest = serde_json::from_value(request.clone()).unwrap();
    let provider = RegistryDisclosureProvider::new();
    let response = ShapeAwareWikiRefractionEngine::new(Some(&provider))
        .refract(&request)
        .unwrap();
    serde_json::to_value(response).unwrap()
}

#[test]
fn the_binarys_refraction_is_the_librarys_through_the_lenses_named() {
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    let (ok, stdout, stderr) = run(&request, true);
    assert!(ok, "{stderr}");
    let got: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(got, library(&request));
    assert_eq!(got["contract"], "ql-mef/wiki-refraction/v1");
    assert_eq!(got["status"], "complete");
    let lenses: Vec<_> = got["readings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|reading| reading["lens_ref"].clone())
        .collect();
    assert_eq!(
        lenses,
        [
            json!("mef:lens:L1@1"),
            json!("mef:lens:L2'@1"),
            json!("mef:lens:L5'@1")
        ]
    );
    assert_eq!(got["readings"][1]["sublens_ref"], "mef:sublens:L2'.3@1");

    // Another lens and sublens, asked optionally: the library's reading through them.
    let mut other = request.clone();
    other["mode"] = json!("optional");
    other["lenses"] = json!([{"lens_ref": "mef:lens:L5@1", "sublens_ref": "mef:sublens:L5.2@1"}]);
    let (ok, stdout, stderr) = run(&other, true);
    assert!(ok, "{stderr}");
    let got: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(got, library(&other));
    assert_eq!(got["readings"][0]["lens_ref"], "mef:lens:L5@1");
    assert_eq!(got["readings"][0]["sublens_ref"], "mef:sublens:L5.2@1");

    // Disabled: no lens is read.
    let mut disabled = request.clone();
    disabled["mode"] = json!("disabled");
    disabled["lenses"] = json!([]);
    let (ok, stdout, stderr) = run(&disabled, true);
    assert!(ok, "{stderr}");
    let got: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(got, library(&disabled));
    assert_eq!(got["status"], "disabled");
}

#[test]
fn the_plain_reading_names_each_lens() {
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    let (ok, stdout, stderr) = run(&request, false);
    assert!(ok, "{stderr}");
    assert!(
        stdout.starts_with("complete refraction of example:wiki:frame:decision-17: 3 reading(s)")
    );
    assert!(stdout.contains("  mef:lens:L2'@1 mef:sublens:L2'.3@1 ("));
}

#[test]
fn refusals_come_in_the_librarys_words() {
    let request: Value = serde_json::from_str(REQUEST).unwrap();
    let mut none = request.clone();
    none["lenses"] = json!([]);
    let mut foreign = request.clone();
    foreign["contract"] = json!("ql-mef/wiki-refraction/v0");
    let mut bad_lens = request.clone();
    bad_lens["lenses"] = json!([{"lens_ref": "mef:lens:L9@1"}]);
    for (request, words) in [
        (none, "enabled refraction requires at least one lens"),
        (foreign, "ql-mef/wiki-refraction/v0"),
        (bad_lens, "invalid lens selection: UNKNOWN_LENS"),
        (
            json!({"mode": "required"}),
            "invalid Wiki refraction request",
        ),
    ] {
        let (ok, _, stderr) = run(&request, true);
        assert!(!ok, "{request}");
        assert!(stderr.contains(words), "{request}: {stderr}");
    }
}

#[test]
fn capabilities_name_the_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["capabilities", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        document["commands"]
            .as_array()
            .unwrap()
            .contains(&json!("wiki.refract"))
    );
}
