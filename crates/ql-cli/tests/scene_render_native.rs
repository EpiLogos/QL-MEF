//! `ql scene render` through the actual executable and the installed native
//! worker: the played/sounding scene rendered offline to a real WAV file a
//! player opens, then exactly replayed. Run with `QL_FIELD_WORKER=<installed
//! ql-field-worker> cargo test --test scene_render_native -- --ignored`.
use serde_json::{Value, json};
use std::process::Command;

fn worker() -> String {
    std::env::var("QL_FIELD_WORKER").expect("QL_FIELD_WORKER must name the worker")
}

/// The DEFAULT event needs no sky: a minimal binding request is complete.
fn request_path() -> std::path::PathBuf {
    let request = json!({
        "schema": "ql.scene-binding-request/v1",
        "instance_ref": "test:render",
        "texture": [8, 8],
        "units_per_metre": 1.0,
    });
    let path = std::env::temp_dir().join(format!(
        "ql-scene-render-request-{}.json",
        std::process::id()
    ));
    std::fs::write(&path, serde_json::to_vec(&request).unwrap()).unwrap();
    path
}

fn render(request: &std::path::Path, wav: &std::path::Path, worker: &str) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_ql"))
        .arg("scene")
        .arg("render")
        .arg(request)
        .args(["--worker", worker, "--seconds", "0.5", "--out"])
        .arg(wav)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
#[ignore = "requires the installed ql-field-worker"]
fn a_played_scene_renders_offline_to_a_byte_identical_wav() {
    let worker = worker();
    let request = request_path();
    let (first, second) = (
        std::env::temp_dir().join(format!("ql-scene-render-a-{}.wav", std::process::id())),
        std::env::temp_dir().join(format!("ql-scene-render-b-{}.wav", std::process::id())),
    );
    let receipt = render(&request, &first, &worker);
    assert_eq!(receipt["schema"], "ql.scene-render/v1");
    assert_eq!(receipt["sample_rate"], 48_000);
    assert_eq!(receipt["seconds"], 0.5);
    assert_eq!(receipt["worker"], worker);
    assert_eq!(receipt["out"], first.to_str().unwrap());
    let frames = receipt["frames"].as_u64().unwrap();
    assert_eq!(frames, 24_000, "ceil(0.5 s at 48 kHz)");
    assert_eq!(
        receipt["blocks"],
        frames.div_ceil(8192),
        "whole 8192-frame worker blocks"
    );
    // The WAV a player opens: canonical header, exact data length, non-silence.
    let bytes = std::fs::read(&first).unwrap();
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WAVE");
    assert_eq!(&bytes[12..16], b"fmt ");
    assert_eq!(&bytes[36..40], b"data");
    assert_eq!(
        u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]),
        (frames * 2) as u32,
        "the data chunk holds exactly frames*2 bytes"
    );
    assert_eq!(bytes.len(), 44 + frames as usize * 2);
    assert_eq!(
        u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        (bytes.len() - 8) as u32
    );
    assert!(
        bytes[44..]
            .chunks_exact(2)
            .any(|s| i16::from_le_bytes([s[0], s[1]]) != 0),
        "the rendered take is silent"
    );
    let sha = receipt["pcm_sha256"].as_str().unwrap();
    assert_eq!(sha.len(), 64, "hex sha256 of the PCM bytes");
    // Exact replay determinism: a second identical render is byte-identical.
    let replay = render(&request, &second, &worker);
    assert_eq!(replay["pcm_sha256"], receipt["pcm_sha256"]);
    assert_eq!(replay["frames"], receipt["frames"]);
    assert_eq!(replay["blocks"], receipt["blocks"]);
    assert_eq!(
        std::fs::read(&second).unwrap(),
        bytes,
        "two renders of one binding differ"
    );
    let _ = std::fs::remove_file(&request);
    let _ = std::fs::remove_file(&first);
    let _ = std::fs::remove_file(&second);
}
