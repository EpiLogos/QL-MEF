//! Reproducible source promotion; outputs stay in Cargo's build directory.
use std::{env, path::PathBuf, process::Command};

fn main() {
    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory")).join("../..");
    let inputs = [
        "scripts/k8-structure.py",
        "scripts/generate-m-tree.py",
        "c/registry/m-tree-source-v1.json",
        "c/registry/promotions/k8-apertures-v1.json",
        "fixtures/kernel/m-tree-v1.json",
        "fixtures/kernel/k8-structure-receipt-v1.json",
        "c/registry/promotions/sources/25d07a69482fa0cd1655a0ab207085f751572608.md",
    ];
    for input in inputs {
        println!("cargo:rerun-if-changed={}", root.join(input).display());
    }
    println!("cargo:rerun-if-env-changed=PYTHON");
    let python = env::var_os("PYTHON").unwrap_or_else(|| "python3".into());
    let status = Command::new(python)
        .arg(root.join("scripts/k8-structure.py"))
        .arg("--check")
        .arg("--out")
        .arg(env::var_os("OUT_DIR").expect("Cargo output directory"))
        .status()
        .expect("Python 3 is required to compile the reviewed M registry source");
    assert!(
        status.success(),
        "K8 registry promotion failed; refusing a stale generated kernel"
    );
}
