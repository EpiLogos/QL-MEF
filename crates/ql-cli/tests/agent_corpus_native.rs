//! The generator and frozen evaluator run the actual public QL executable.
//! This checks structural data, never simulated inference or body acceptance.
use ql_mef::agent_event::{ProjectionRequest, project_event};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Evidence(PathBuf);
impl Evidence {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "ql-native-corpus-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Evidence {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

fn python(arguments: &[&str]) {
    let output = Command::new("python3")
        .current_dir(root())
        .args(arguments)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "actual Python/CLI pipeline failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

fn lines(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[test]
fn actual_corpus_is_reproducible_and_kernel_gates_every_reading_and_counterfactual() {
    let evidence = Evidence::new();
    let first = evidence.0.join("first");
    let second = evidence.0.join("second");
    for output in [&first, &second] {
        python(&[
            "scripts/ql_agent_corpus.py",
            "--ql",
            env!("CARGO_BIN_EXE_ql"),
            "--output",
            output.to_str().unwrap(),
        ]);
    }
    let manifest = read(&first.join("manifest.json"));
    assert_eq!(manifest, read(&second.join("manifest.json")));
    for name in ["suite.jsonl", "native-readings.jsonl", "refusals.jsonl"] {
        assert_eq!(
            std::fs::read(first.join(name)).unwrap(),
            std::fs::read(second.join(name)).unwrap()
        );
    }
    assert_eq!(manifest["provider_calls"], 0);
    assert_eq!(
        manifest["native_label_owners"]["lenses"]
            .as_array()
            .unwrap()
            .len(),
        12
    );
    assert_eq!(
        manifest["native_label_owners"]["context_frames"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    let readings = lines(&first.join("native-readings.jsonl"));
    let cases = lines(&first.join("suite.jsonl"));
    assert_eq!(readings.len(), cases.len());
    assert!(cases.len() > 400);
    let mut families = BTreeMap::new();
    let mut overlap = false;
    let mut missing_side = false;
    for (case, reading) in cases.iter().zip(readings) {
        let family = case["family"].as_str().unwrap();
        assert_eq!(reading["id"], case["id"]);
        if let Some(previous) = families.insert(family, case["split"].clone()) {
            assert_eq!(previous, case["split"], "template leaked across splits");
        }
        let request: ProjectionRequest =
            serde_json::from_value(case["projection"].clone()).unwrap();
        let owner = serde_json::to_value(project_event(request).unwrap()).unwrap();
        assert_eq!(
            owner, reading["projection"],
            "native formal/harmonic owner differs"
        );
        assert!(
            owner["determination"]["learned"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert!(owner["determination"].get("provider").is_none());
        if let Some(relations) = owner["determination"]["derived"]
            .as_array()
            .unwrap()
            .iter()
            .find(|fact| fact["field"] == "relation-candidates")
        {
            let candidates = relations["value"].as_array().unwrap();
            if candidates.len() > 1 || candidates.iter().any(|pair| pair["reversed"] == true) {
                assert_eq!(
                    case["split"], "test",
                    "held-out traversal leaked through another template"
                );
            }
        }
        if family == "relation-overlap" {
            overlap = true;
            let relations = owner["determination"]["derived"]
                .as_array()
                .unwrap()
                .iter()
                .find(|fact| fact["field"] == "relation-candidates")
                .unwrap();
            assert!(relations["value"].as_array().unwrap().len() > 1);
            assert!(owner["decision_head_ids"].as_array().unwrap().is_empty());
        }
        if family.starts_with("completion-missing-side") {
            missing_side = true;
            assert_eq!(owner["determination"]["status"], "unresolved");
            assert!(owner["decision_head_ids"].as_array().unwrap().is_empty());
        }
    }
    assert!(overlap && missing_side);
    let refusals = lines(&first.join("refusals.jsonl"));
    assert_eq!(refusals.len(), 7);
    for refused in refusals {
        let request: ProjectionRequest =
            serde_json::from_value(refused["projection"].clone()).unwrap();
        assert!(project_event(request).unwrap_err().contains("contradict"));
    }
    for split in ["train", "validation", "test"] {
        let output = evidence.0.join(split);
        let suite = first.join("suite.jsonl");
        let mut arguments = vec![
            "scripts/ql_agent_evaluation.py",
            "--ql",
            env!("CARGO_BIN_EXE_ql"),
            "--suite",
            suite.to_str().unwrap(),
        ];
        arguments.extend([
            "--suite-digest",
            manifest["suite_digest"].as_str().unwrap(),
            "--split",
            split,
            "--output",
            output.to_str().unwrap(),
        ]);
        if split != "test" {
            arguments.extend([
                "--provider-command",
                "/nonexistent/ql-corpus-provider-must-not-run",
            ]);
        }
        python(&arguments);
        let metrics = read(&output.join("metrics.json"));
        assert_eq!(metrics["metrics"]["provider_calls"], 0);
        assert_eq!(metrics["metrics"]["exact_determination_accuracy"], 1.0);
        assert_eq!(metrics["metrics"]["deterministic_bypass_rate"], 1.0);
        assert!(metrics["metrics"]["provider_elapsed_p95_seconds"].is_null());
    }
    python(&[
        "-m",
        "unittest",
        "discover",
        "-s",
        "scripts/tests",
        "-p",
        "test_ql_agent_corpus.py",
    ]);
}
