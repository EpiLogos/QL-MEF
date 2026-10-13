//! `ql music modes --json`: the kernel's mode catalogue served as one versioned,
//! byte-stable reading. Every value is a kernel value; the CLI only projects it.
//! Shem and Ficino planetary scales are named absent, never invented.

use std::collections::BTreeMap;
use std::process::Command;

use ql_mef::{LensId, ModeKind, MusicalBasis, mode_tonic_landscape};

fn modes(args: &[&str]) -> std::process::Output {
    let bin = env!("CARGO_BIN_EXE_ql");
    Command::new(bin)
        .args(["music", "modes"].iter().chain(args))
        .output()
        .expect("ql music modes should run")
}

fn modes_json() -> serde_json::Value {
    let output = modes(&["--json"]);
    assert!(
        output.status.success(),
        "ql music modes --json failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("valid JSON output")
}

fn basis_name(basis: MusicalBasis) -> &'static str {
    match basis {
        MusicalBasis::Chromatic => "chromatic",
        MusicalBasis::Fifths => "fifths",
    }
}

fn mode_name(mode: ModeKind) -> &'static str {
    match mode {
        ModeKind::Ionian => "ionian",
        ModeKind::Dorian => "dorian",
        ModeKind::Phrygian => "phrygian",
        ModeKind::Lydian => "lydian",
        ModeKind::Mixolydian => "mixolydian",
        ModeKind::Aeolian => "aeolian",
        ModeKind::Locrian => "locrian",
    }
}

#[test]
fn the_reading_is_versioned_and_byte_stable() {
    let first = modes(&["--json"]);
    let second = modes(&["--json"]);
    assert!(first.status.success());
    assert_eq!(first.stdout, second.stdout, "two runs disagree");
    let value: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(value["schema"], "ql.music-modes/v1");
    assert!(
        !String::from_utf8_lossy(&first.stdout).contains("timestamp"),
        "no timestamps in a reading"
    );
}

#[test]
fn lens_cf_rows_are_the_kernel_landscape_for_every_defined_basis() {
    let value = modes_json();
    let rows = value["lens_cf"].as_array().unwrap();
    // Two bases exist in MusicalBasis::ALL; each carries 12 lenses x 7 modes.
    assert_eq!(MusicalBasis::ALL.len(), 2);
    assert_eq!(rows.len(), 168);

    let mut expected: BTreeMap<(String, String, String), serde_json::Value> = BTreeMap::new();
    for basis in MusicalBasis::ALL {
        for instance in mode_tonic_landscape(basis) {
            let lens = instance.lens;
            expected.insert(
                (
                    basis_name(basis).to_string(),
                    lens_code(lens),
                    mode_name(instance.mode).to_string(),
                ),
                serde_json::json!({
                    "pitches": instance.pitches.map(u64::from),
                    "tonic": u64::from(instance.tonic),
                    "scale_beneath_tonic": u64::from(instance.scale_beneath_tonic),
                    "context_frame": instance.context_frame.code(),
                }),
            );
        }
    }
    assert_eq!(
        expected.len(),
        168,
        "the kernel's landscape keys are distinct"
    );

    let mut seen = BTreeMap::new();
    for row in rows {
        let key = (
            row["anchor_basis"].as_str().unwrap().to_string(),
            row["lens"].as_str().unwrap().to_string(),
            row["mode"].as_str().unwrap().to_string(),
        );
        let want = expected
            .get(&key)
            .unwrap_or_else(|| panic!("row {key:?} is not a kernel instance"));
        assert_eq!(row["pitches"], want["pitches"], "{key:?}");
        assert_eq!(row["tonic"], want["tonic"], "{key:?}");
        assert_eq!(
            row["scale_beneath_tonic"], want["scale_beneath_tonic"],
            "{key:?}"
        );
        assert_eq!(row["context_frame"], want["context_frame"], "{key:?}");
        assert_eq!(row["tonic"], row["pitches"][0], "tonic is degree one");
        assert_eq!(row["kind"], "lens-cf");
        assert!(
            row["tuning"].is_null(),
            "lens/CF rows declare no ratio per step"
        );
        assert!(seen.insert(key, ()).is_none(), "duplicate row");
    }
    assert_eq!(seen.len(), 168);

    // Exact intervals travel with the row: successive steps of the served pitches.
    for row in rows {
        let pitches: Vec<u64> = row["pitches"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p.as_u64().unwrap())
            .collect();
        let steps: Vec<u64> = (0..7)
            .map(|i| (pitches[(i + 1) % 7] + 12 - pitches[i]) % 12)
            .collect();
        let served: Vec<u64> = row["steps_semitones"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_u64().unwrap())
            .collect();
        assert_eq!(served, steps, "{}", row["key"]);
    }
}

#[test]
fn lens_cf_rows_carry_the_kernel_provenance_of_their_operator() {
    let value = modes_json();
    let row = &value["lens_cf"][0];
    let provenance = &row["provenance"];
    assert_eq!(provenance["owner"], "ql-mef");
    assert_eq!(provenance["symbol"], "mode_tonic_landscape");
    assert_eq!(provenance["revision"], ql_mef::MUSICAL_HARMONIC_VERSION);
    assert_eq!(
        provenance["source"]["path"],
        ql_mef::MUSICAL_DERIVATION_SOURCE_PATH
    );
    assert_eq!(
        provenance["source"]["blob"],
        ql_mef::MUSICAL_DERIVATION_SOURCE_BLOB
    );
    let bases = value["bases"].as_array().unwrap();
    assert_eq!(bases.len(), 2);
    assert_eq!(bases[0]["basis"], "chromatic");
    assert_eq!(bases[0]["generator_ratio"]["num"], 9);
    assert_eq!(bases[0]["generator_ratio"]["den"], 8);
    assert_eq!(bases[1]["basis"], "fifths");
    assert_eq!(bases[1]["generator_ratio"]["num"], 3);
    assert_eq!(bases[1]["generator_ratio"]["den"], 2);
}

#[test]
fn source_scales_are_the_quarter_tone_rows_maqam_pitches_serves() {
    let value = modes_json();
    let scales = value["source_scales"].as_array().unwrap();
    assert_eq!(scales.len(), 72);
    let table = ql_mef::m2::catalogue().table("maqam").unwrap();
    for (mode, scale) in scales.iter().enumerate() {
        assert_eq!(scale["key"], format!("maqam/{mode:02}"));
        assert_eq!(scale["family"], "maqam");
        assert!(scale["pitches"].is_null(), "no 12-class coercion");
        assert!(
            scale["tuning"].is_null(),
            "no ratio is claimed for a quarter-tone step"
        );
        assert_eq!(scale["tuning_unit"], "quarter-tone");

        let steps: Vec<u64> = scale["quarter_tone_steps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_u64().unwrap())
            .collect();
        assert_eq!(steps.len(), 7);
        assert_eq!(steps.iter().sum::<u64>(), 24, "one octave in quarter-tones");

        // The steps are the kernel's own catalogue row, unchanged.
        let row = table.row(mode).unwrap();
        assert_eq!(steps, row[2..9].to_vec(), "maqam {mode}");

        // And maqam_pitches serves this row: its ratio at each degree is 2^(q/24).
        let served = ql_mef::m2::maqam_pitches(mode as u8, 1.0).unwrap();
        let from_root: Vec<u64> = scale["quarter_tones_from_root"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| q.as_u64().unwrap())
            .collect();
        assert_eq!(from_root.len(), 8);
        for (degree, quarter_tones) in from_root.iter().enumerate() {
            let want = 2f64.powf(*quarter_tones as f64 / 24.0);
            assert!(
                (served[degree] - want).abs() < 1e-12,
                "maqam {mode} degree {degree}: served {} want {want}",
                served[degree]
            );
        }
        assert_eq!(scale["cents_from_root"][7], 1200, "exact octave in cents");

        // Provenance names the kernel's own operator and the locked source it reads.
        let provenance = &scale["provenance"];
        assert_eq!(provenance["owner"], "ql-mef");
        assert_eq!(provenance["symbol"], "maqam_pitches");
        assert_eq!(
            provenance["registry_revision"],
            ql_mef::m2::catalogue().registry_revision()
        );
        // The O:I side reads every scale's provenance as {owner, symbol, revision}.
        assert_eq!(
            provenance["revision"],
            ql_mef::m2::catalogue().registry_revision()
        );
        assert_eq!(
            provenance["sources"].as_array().unwrap().len(),
            ql_mef::m2::catalogue().sources().len()
        );

        // The family's exact harmonic ratio, as the kernel's ratio table holds it.
        let family = usize::try_from(row[0]).unwrap();
        let ratio = ql_mef::m2::catalogue()
            .table("ratio")
            .unwrap()
            .row(family)
            .unwrap();
        if ratio == [0, 0] {
            assert!(
                scale["family_ratio"].is_null(),
                "family {family} declares no ratio"
            );
        } else {
            assert_eq!(scale["family_ratio"]["num"], ratio[0], "maqam {mode}");
            assert_eq!(scale["family_ratio"]["den"], ratio[1], "maqam {mode}");
        }
    }
}

#[test]
fn shem_and_ficino_are_named_absent_and_never_invented() {
    let value = modes_json();
    let absent = value["sources_absent"].as_array().unwrap();
    let names: Vec<&str> = absent
        .iter()
        .map(|a| a["scale"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["shem", "ficino-planetary"]);
    for entry in absent {
        assert!(!entry["reason"].as_str().unwrap().is_empty());
    }
    // The Shem table holds positions, elements and meanings, and no pitch column.
    let shem = ql_mef::m2::catalogue().table("shem").unwrap();
    assert!(
        shem.columns()
            .iter()
            .all(|c| !c.contains("pitch") && !c.contains("interval") && !c.contains("ratio")),
        "{:?}",
        shem.columns()
    );
    // No served scale is a Shem or Ficino pitch set.
    assert!(
        value["source_scales"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["family"] == "maqam")
    );
}

#[test]
fn unknown_arguments_are_refused_with_usage() {
    let output = modes(&["--lens", "3"]);
    assert_eq!(output.status.code(), Some(2), "unknown flags are refused");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("usage: ql music modes"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn help_lists_the_modes_reading() {
    let bin = env!("CARGO_BIN_EXE_ql");
    let output = Command::new(bin).arg("--help").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ql music modes"), "{stdout}");
}

fn lens_code(lens: LensId) -> String {
    lens.code().to_string()
}
