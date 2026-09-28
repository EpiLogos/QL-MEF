//! The native Spanda field against the C kernel's T2.11 oscillator, executed.

use ql_core::Quat;
use ql_mef::spanda_field::*;
use serde_json::Value;
use std::{path::Path, process::Command};

fn close(c: &Value, key: &str, rust: f64) {
    let expected = c[key].as_f64().unwrap();
    assert!(
        (expected - rust).abs() <= 1e-12 * expected.abs().max(1.0),
        "{key}: C {expected} vs Rust {rust} in {c}"
    );
}

#[test]
fn native_spanda_field_matches_the_c_kernel() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/spanda-field");
    std::fs::create_dir_all(&out).unwrap();
    let exe = out.join("probe");
    let gc = if cfg!(target_os = "macos") {
        "-Wl,-dead_strip"
    } else {
        "-Wl,--gc-sections"
    };
    let build = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .current_dir(&root)
        .args([
            "-std=c11",
            "-D_DEFAULT_SOURCE",
            "-O1",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-ffunction-sections",
            "-fdata-sections",
            "-Ivendor/epi-kernel/reference/include",
            "migration/epi-kernel/spanda-field-probe.c",
            "vendor/epi-kernel/reference/src/m1.c",
            "vendor/epi-kernel/reference/src/m3.c",
            "vendor/epi-kernel/reference/src/kernel.c",
            "vendor/epi-kernel/reference/src/psychoid_numbers.c",
            gc,
            "-lm",
            "-o",
        ])
        .arg(&exe)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let result = Command::new(exe).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );

    let p = HkbParams::default();
    let mut counts = [0usize; 4];
    for line in String::from_utf8(result.stdout).unwrap().lines() {
        let c: Value = serde_json::from_str(line).unwrap();
        let f = |key: &str| c[key].as_f64().unwrap();
        match c["kind"].as_str().unwrap() {
            "params" => {
                assert_eq!(
                    (f("delta_omega"), f("a"), f("b"), f("base_freq_hz")),
                    (p.delta_omega, p.a, p.b, p.base_freq_hz)
                );
            }
            "hkb" => {
                counts[0] += 1;
                let phi = f("phi");
                close(&c, "drift", p.drift(phi));
                close(&c, "potential", p.potential(phi));
                close(&c, "curvature", p.curvature(phi));
                close(&c, "settle", p.settle(phi, 0.01, 2000));
                assert_eq!(
                    c["tick12"].as_u64().unwrap() as u8,
                    tick12_readout(phi),
                    "{c}"
                );
            }
            "wave" => {
                counts[1] += 1;
                let (x, t, s) = (f("x"), f("t"), c["swapped"].as_bool().unwrap());
                close(&c, "pole0", pole_wave(x, t, 0, s));
                close(&c, "pole1", pole_wave(x, t, 1, s));
                close(&c, "superposition", superposition(x, t, s));
                close(&c, "envelope", standing_envelope(x, s));
            }
            "half_turn" => {
                counts[2] += 1;
                let n = c["n"].as_u64().unwrap() as u8;
                assert_eq!(c["value"].as_u64().unwrap() as u8, half_turn_index(n));
            }
            "codon" => {
                counts[3] += 1;
                let q: Vec<f32> = c["q"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap() as f32)
                    .collect();
                let rot = Quat {
                    w: q[0],
                    x: q[1],
                    y: q[2],
                    z: q[3],
                };
                let cycle = c["cycle"].as_u64().unwrap();
                assert_eq!(
                    c["codon"].as_u64().unwrap() as u8,
                    codon_advance(rot, cycle).address(),
                    "{c}"
                );
            }
            other => panic!("unknown C observation {other:?}"),
        }
    }
    assert_eq!(counts, [97, 48, 12, 1728]);
}
