//! `ql stage` through the actual executable and the installed native worker:
//! the same field-host route the Expressions application drives, walked by a
//! fresh agent from a plain shell (QL-MEF #296 acceptance A17). Run with
//! `QL_FIELD_WORKER=<installed ql-field-worker> cargo test --test stage_native
//! -- --ignored`.
use serde_json::{Value, json};
use std::process::Command;

fn worker() -> String {
    std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker")
}

fn ql(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ql"))
        .args(args)
        .args(["--worker", &worker()])
        .output()
        .unwrap()
}

fn ql_ok(args: &[&str]) -> Value {
    let out = ql(args);
    assert!(
        out.status.success(),
        "ql {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}

fn fixture() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/kernel/stage-procedure-example-v1.json")
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn the_cli_drives_stage_state_and_a_fixture_evaluation_round_trip() {
    let state = ql_ok(&["stage", "state"]);
    assert_eq!(state["status"], "ok", "{}", state["error"]);
    assert_eq!(state["schema"], "ql.field-host-receipt/v1");
    assert_eq!(state["stage"]["schema"], "ql.stage-state/v1");
    assert_eq!(state["stage"]["subject_ref"], "ql:k2/default-subject");
    assert_eq!(state["stage"]["form"]["address"], json!(7));
    assert_eq!(state["stage"]["slots"]["form"]["owner"], Value::Null);
    assert!(state["field"]["event_ref"].is_string());

    // The committed fixture DATA evaluates through the same binary route: the
    // receipt names the procedure, and the fold plus its generated passage are
    // real determinants on the live worker (the field actually moved).
    let evaluated = ql_ok(&["stage", "evaluate", fixture().to_str().unwrap()]);
    assert_eq!(evaluated["status"], "ok", "{}", evaluated["error"]);
    let stage = &evaluated["stage"];
    assert_eq!(stage["schema"], "ql.stage-receipt/v1");
    assert_eq!(stage["procedure_ref"], json!("ta-onta:stage:cli-fold"));
    assert_eq!(stage["applied"], json!(true));
    assert_eq!(stage["generated_scenes"], json!(2));
    assert_eq!(
        stage["contributions"][0]["key"],
        json!("ta-onta:stage:cli-fold@1/form")
    );
    assert!(!evaluated["field"]["targets"].as_array().unwrap().is_empty());
    assert!(evaluated["influence"]["native_readback"].is_object());

    // Every invocation is one owned exchange over a fresh default scene: the
    // stage ownership lives with the opened host (as in the live-stage E2E's
    // one Driver), so a second process proves the routes, never a persistence
    // the host architecture does not have.
    let state = ql_ok(&["stage", "state"]);
    assert_eq!(state["stage"]["slots"]["form"]["owner"], Value::Null);

    // Unbind names the truth about a fresh host: no binding holds that ref,
    // and the refusal still carries the last acknowledged field.
    let out = ql(&["stage", "unbind", "ta-onta:stage:cli-fold"]);
    assert!(!out.status.success(), "an absent binding must refuse");
    let refused: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(refused["status"], "refused");
    assert!(
        refused["error"]
            .as_str()
            .unwrap()
            .contains("no binding names"),
        "{}",
        refused["error"]
    );

    // Bind drives the determinant route through the binary: the receipt names
    // the admitted determinant and the caller's own budget.
    let bound_path =
        std::env::temp_dir().join(format!("ql-stage-bound-{}.json", std::process::id()));
    std::fs::write(
        &bound_path,
        json!({
            "schema": "ql.stage-procedure/v1",
            "procedure_ref": "ta-onta:stage:cli-bound",
            "revision": 1,
            "subject_ref": "ql:k2/default-subject",
            "trigger": {"trigger": "determinant", "operation": "m1-advance"},
            "selector": [],
            "changes": [{"change": "strike", "mode_ref": "scene:planet/#2-5-4",
                "amplitude": [0.5, 0.0]}]
        })
        .to_string(),
    )
    .unwrap();
    let bound = ql_ok(&[
        "stage",
        "bind",
        bound_path.to_str().unwrap(),
        "--max-evaluations",
        "4",
    ]);
    let _ = std::fs::remove_file(&bound_path);
    assert_eq!(bound["status"], "ok", "{}", bound["error"]);
    assert_eq!(
        bound["stage"]["binding"]["determinant"],
        json!("m1-advance")
    );
    assert_eq!(bound["stage"]["binding"]["max_evaluations"], json!(4));
    assert_eq!(bound["stage"]["binding"]["standing"], json!("active"));

    // Retire on a fresh host is admitted and names exactly what it released:
    // nothing was held, and the receipt says so.
    let retired = ql_ok(&["stage", "retire", "ta-onta:stage:cli-fold"]);
    assert_eq!(retired["status"], "ok");
    assert_eq!(retired["stage"]["retired"], json!([]));
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_refused_evaluation_prints_the_last_acknowledged_field_and_fails() {
    // A foreign-subject procedure passes the CLI's loading validation and is
    // refused by the host after admission: the exit is non-zero and stdout
    // still carries the full receipt with the last acknowledged field.
    let refused_path =
        std::env::temp_dir().join(format!("ql-stage-refused-{}.json", std::process::id()));
    std::fs::write(
        &refused_path,
        json!({
            "schema": "ql.stage-procedure/v1",
            "procedure_ref": "ta-onta:stage:cli-foreign",
            "revision": 1,
            "subject_ref": "person:someone-else",
            "trigger": {"trigger": "invocation"},
            "selector": ["material.damping"],
            "changes": [{"change": "damping", "per_second": 0.5}]
        })
        .to_string(),
    )
    .unwrap();
    let out = ql(&["stage", "evaluate", refused_path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&refused_path);
    assert!(!out.status.success(), "a refused evaluation must fail");
    let receipt: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(receipt["status"], "refused");
    assert!(
        receipt["error"]
            .as_str()
            .unwrap()
            .contains("but the live event belongs to"),
        "{}",
        receipt["error"]
    );
    assert!(
        receipt["field"]["event_ref"].is_string(),
        "the refusal carries the last acknowledged field"
    );
    // The stderr names the cause for the human; stdout stays pure JSON.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("stage evaluate refused"), "{stderr}");
}
