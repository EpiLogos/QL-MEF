//! The installed `ql music janko` controller read: the projection comes from
//! the accepted musical object, and the CLI output agrees with the native
//! invariants (one substrate, three touch-points, lens anchoring).

use std::process::Command;

fn janko(args: &[&str]) -> serde_json::Value {
    let bin = env!("CARGO_BIN_EXE_ql");
    let output = Command::new(bin)
        .args(["music", "janko"].iter().chain(args))
        .output()
        .unwrap_or_else(|e| panic!("ql music janko {args:?} should run: {e}"));
    assert!(
        output.status.success(),
        "ql music janko {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("valid JSON output")
}

#[test]
fn the_surface_window_discloses_one_full_period_over_the_substrate() {
    let surface = janko(&["window", "--json"]);
    assert_eq!(surface["schema"], "ql.janko-surface/v1");
    assert_eq!(surface["rows"], 6);
    assert_eq!(surface["column_period"], 6);
    assert_eq!(surface["touch_points"], 3);
    let rows = surface["surface"].as_array().unwrap();
    assert_eq!(rows.len(), 6);
    let mut counts = [0_u8; 12];
    for row in rows {
        let keys = row["keys"].as_array().unwrap();
        assert_eq!(keys.len(), 6);
        for key in keys {
            let class = key["sounding_pitch_class"].as_u64().unwrap();
            counts[usize::try_from(class).unwrap()] += 1;
            assert_eq!(
                key["coordinate"]["face"].as_str().unwrap(),
                key["direct_prime_face"].as_str().unwrap()
            );
        }
    }
    for (pitch_class, count) in counts.into_iter().enumerate() {
        assert_eq!(count, 3, "pitch class {pitch_class}");
    }
}

#[test]
fn one_key_projects_its_class_coordinate_and_fifths_overlay() {
    let key = janko(&["key", "--row", "1", "--column", "3", "--json"]);
    assert_eq!(key["sounding_pitch_class"], 7);
    assert_eq!(key["pitch_name"], "G");
    assert_eq!(key["whole_tone_row_family"], 1);
    assert_eq!(key["repeated_touch_point"], 0);
    assert_eq!(key["coordinate"]["face"], "prime");
    assert_eq!(key["fifths_overlay"]["face"], "direct");
    // The same key under a lens anchor is its own chromatic transposition.
    let anchored = janko(&[
        "key", "--row", "1", "--column", "3", "--lens", "3", "--json",
    ]);
    let plain = key["sounding_pitch_class"].as_u64().unwrap();
    let lensed = anchored["sounding_pitch_class"].as_u64().unwrap();
    let tonic = anchored["lens_tonic_pitch"].as_u64().unwrap();
    assert_eq!((plain + tonic) % 12, lensed);
}

#[test]
fn touch_points_name_three_keys_of_one_family_and_reject_foreign_pitch() {
    let points = janko(&["touch-points", "--pitch", "4", "--json"]);
    let keys = points["touch_points"].as_array().unwrap();
    assert_eq!(keys.len(), 3);
    let family = keys[0]["whole_tone_row_family"].as_u64().unwrap();
    let coordinate = keys[0]["coordinate"].clone();
    for (index, key) in keys.iter().enumerate() {
        assert_eq!(key["sounding_pitch_class"], 4);
        assert_eq!(key["whole_tone_row_family"], family);
        assert_eq!(key["repeated_touch_point"], index as u64);
        assert_eq!(key["coordinate"], coordinate);
    }
    let bin = env!("CARGO_BIN_EXE_ql");
    let output = Command::new(bin)
        .args(["music", "janko", "touch-points", "--pitch", "99"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "a foreign pitch is refused");
}

#[test]
fn help_lists_the_music_group() {
    let bin = env!("CARGO_BIN_EXE_ql");
    let output = Command::new(bin).arg("--help").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Music (controller projections"), "{stdout}");
    assert!(stdout.contains("ql music janko"), "{stdout}");
}
