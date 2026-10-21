//! `ql wiki living-plan`: the Living Wiki refraction plan through the real
//! front-door binary. It answers exactly what `plan_living_wiki_refraction`
//! answers for the stated mode and relevance, and refuses in the library's own
//! words.
use ql_wiki::{LivingWikiMode, LivingWikiRelevance, plan_living_wiki_refraction};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

/// `ql wiki living-plan - [--json]` with the request on stdin.
fn run(request: &Value, json: bool) -> (bool, String, String) {
    let mut args = vec!["wiki", "living-plan", "-"];
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

fn plan(request: &Value) -> Value {
    let (ok, stdout, stderr) = run(request, true);
    assert!(ok, "{stderr}");
    serde_json::from_str(&stdout).unwrap()
}

#[test]
fn the_binarys_plan_is_the_librarys_plan() {
    for (mode, relevance) in [
        (LivingWikiMode::Explain, LivingWikiRelevance::default()),
        (
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                relevant_positions: vec![2, 3],
                requires_context: true,
                full_context_frame: false,
                position_budget: 4,
            },
        ),
        (
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                full_context_frame: true,
                ..LivingWikiRelevance::default()
            },
        ),
        (LivingWikiMode::Ordinary, LivingWikiRelevance::default()),
    ] {
        let expected = plan_living_wiki_refraction(mode, relevance.clone(), None).unwrap();
        let got = plan(&json!({"mode": mode, "relevance": relevance}));
        assert_eq!(got, serde_json::to_value(&expected).unwrap());
    }
    let partial = plan(&json!({"mode": "explain", "relevance":
        {"relevant_positions": [2, 3], "requires_context": true, "position_budget": 4}}));
    assert_eq!(partial["positions"], json!([5, 0, 1, 2]));
    assert_eq!(partial["omitted_relevant_positions"], json!([3, 4]));
    assert_eq!(partial["depth"], "partial");
    assert_eq!(partial["entry"]["is_return"], false);
    assert!(partial["refraction"].is_null());
    let ordinary = plan(&json!({"mode": "ordinary"}));
    assert_eq!(ordinary["positions"], json!([]));
    assert_eq!(ordinary["depth"], "none");
    assert_eq!(ordinary["ordinary_wiki_correctness_requires_ql"], false);
}

#[test]
fn the_plain_reading_names_admitted_and_omitted_positions() {
    let (ok, stdout, stderr) = run(
        &json!({"mode": "explain", "relevance": {"relevant_positions": [2, 3], "requires_context": true, "position_budget": 4}}),
        false,
    );
    assert!(ok, "{stderr}");
    assert!(stdout.contains("explain plan, depth partial: admitted 5 0 1 2; omitted 3 4"));
    assert!(stdout.contains("refraction request none"));
}

#[test]
fn refusals_come_in_the_librarys_words() {
    for (request, words) in [
        (
            json!({"mode": "formal"}),
            "formal Living Wiki depth requires an existing WikiRefractionRequest",
        ),
        (
            json!({"mode": "explain", "relevance": {"position_budget": 2}}),
            "cannot preserve the 5->0->1 entry aperture",
        ),
        (
            json!({"mode": "explain", "relevance": {"relevant_positions": [6], "position_budget": 6}}),
            "Living Wiki relevance position 6 is outside 0..5",
        ),
        (
            json!({"mode": "explain", "surplus": true}),
            "invalid Living Wiki plan request",
        ),
    ] {
        let (ok, _, stderr) = run(&request, true);
        assert!(!ok, "{request}");
        assert!(stderr.contains(words), "{request}: {stderr}");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(["wiki", "plan"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("usage: ql wiki <living-plan|refract> <request.json|-> [--json]")
    );
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
            .contains(&json!("wiki.living-plan"))
    );
}
