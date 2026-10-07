//! The actual reviewed-data/packing/research pipeline uses the built QL CLI.
//! AIKit owns its provider adapter; this gate needs that exact external source.
use std::path::Path;
use std::process::Command;

#[test]
#[ignore = "requires AIKIT_GLINER_ADAPTER and QL_AGENT_RESEARCH_EVIDENCE_ROOT"]
fn real_native_research_pipeline_preserves_splits_and_refuses_frozen_training() {
    let adapter = std::env::var_os("AIKIT_GLINER_ADAPTER")
        .expect("the actual AIKit GLiNER adapter source is required");
    let evidence = std::env::var_os("QL_AGENT_RESEARCH_EVIDENCE_ROOT")
        .expect("explicit retained native pipeline evidence field is required");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let output = Command::new("python3")
        .arg(root.join("scripts/test-ql-agent-research.py"))
        .arg("--ql")
        .arg(env!("CARGO_BIN_EXE_ql"))
        .arg("--provider-adapter")
        .arg(adapter)
        .arg("--output")
        .arg(&evidence)
        .current_dir(root)
        .output()
        .expect("launch the real native research pipeline");
    assert!(
        output.status.success(),
        "native pipeline failed; evidence retained at {:?}: {}",
        evidence,
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "passed");
    assert_eq!(result["provider_calls"], 0);
    assert_eq!(result["frozen_test_refused"], true);
    assert_eq!(result["deterministic_test_accuracy"], 1.0);
    assert_eq!(result["native_plan_cases"], 9);
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
