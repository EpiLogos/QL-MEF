//! Native Pi carries the same source-built QL CLI receipts. This explicit
//! integration gate needs the pinned Pi SDK, but ordinary QL use does not.
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct NativeWork(PathBuf);
impl NativeWork {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "ql-pi-native-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for NativeWork {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
#[ignore = "requires AIKIT_PI_SDK_ROOT pointing to the installed Pi 0.84.4 SDK"]
fn pi_registered_tools_invoke_the_actual_ql_owner() {
    let sdk = std::env::var_os("AIKIT_PI_SDK_ROOT")
        .expect("explicit native Pi gate requires AIKIT_PI_SDK_ROOT");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let package = root.join("adapters/pi/ql-agent");
    let work = NativeWork::new();
    let result = Command::new("node")
        .arg(package.join("tests/native-tools.mjs"))
        .arg(&package)
        .arg(&work.0)
        .arg(sdk)
        .arg(root.join("fixtures/agent-decision/v1/event-v1.json"))
        .env("QL_AGENT_BIN", env!("CARGO_BIN_EXE_ql"))
        .current_dir(&work.0)
        .output()
        .expect("launch actual Pi SDK with source-built QL executable");
    assert!(
        result.status.success(),
        "native Pi tools failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let receipt: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(receipt["schema"], "ql.pi-native-tools-proof/v1");
    assert_eq!(receipt["native_owner_success"], true);
    assert_eq!(receipt["model_calls"], 0);
    assert_eq!(receipt["checks"].as_array().unwrap().len(), 13);
    // Keep the exact native carrier checks visible in the hosted gate output.
    println!("{}", String::from_utf8_lossy(&result.stdout));
}
