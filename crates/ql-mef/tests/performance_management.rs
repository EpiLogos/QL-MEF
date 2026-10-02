#[path = "support/retained_performance.rs"]
mod support;
use ql_mef::m1_engine::Basis;
use ql_mef::performance_audio::{KeyTouch, prepare_native_performance};
use ql_mef::performance_management::{native_janko_catalog, resolve_performance_touch};
use std::collections::BTreeMap;

#[test]
fn all_native_bases_lenses_transpositions_preserve_real_janko_geometry_and_targets() {
    for basis in [Basis::Chromatic, Basis::Fifths] {
        for lens in 0..12 {
            let mut input = support::preparation();
            input.coupled.m1.basis = basis;
            input.coupled.m1.lens12 = lens;
            let binding = prepare_native_performance(input).unwrap();
            let before = binding.determination().clone();
            let physical = serde_json::to_value(binding.physical_body()).unwrap();
            for transpose in 0..12 {
                let cells = native_janko_catalog(&binding, 6, 0, transpose).unwrap();
                assert_eq!(cells.len(), 36);
                let mut copies = BTreeMap::new();
                for c in &cells {
                    let original = if c.row % 2 == 0 {
                        [0, 2, 4, 6, 8, 10]
                    } else {
                        [1, 3, 5, 7, 9, 11]
                    };
                    let semitone = original[usize::from(c.column)] + transpose;
                    assert_eq!(c.pitch_class, semitone % 12);
                    assert_eq!(c.register_octave, (semitone / 12) as i8);
                    let native = binding
                        .targets()
                        .key_target(c.key, c.register_octave, "test:independent-native-touch")
                        .unwrap();
                    assert_eq!(c.hertz, native.hertz);
                    assert_eq!(c.coordinate, native.source_coordinate.source_ref);
                    copies
                        .entry((c.pitch_class, c.register_octave))
                        .or_insert_with(Vec::new)
                        .push(c);
                }
                assert_eq!(copies.len(), 12);
                for (_, three) in copies {
                    assert_eq!(three.len(), 3);
                    assert_eq!(three[0].hertz, three[1].hertz);
                    assert_eq!(three[1].hertz, three[2].hertz);
                    assert_eq!(three[0].key, three[1].key);
                    assert_eq!(three[0].ratio, three[2].ratio);
                    assert_eq!(three[1].row - three[0].row, 2);
                    assert_eq!(three[2].row - three[1].row, 2);
                }
                // Selection does not create a second bus or reset the body.
                assert_eq!(binding.determination(), &before);
                assert_eq!(
                    serde_json::to_value(binding.physical_body()).unwrap(),
                    physical
                );
            }
        }
    }
}

#[test]
fn repeated_independent_native_touches_keep_exact_phase_ratio_and_source_identity() {
    let binding = prepare_native_performance(support::preparation()).unwrap();
    let make = |touch| KeyTouch {
        key: 4,
        register: -1,
        member: 41,
        touch,
        touch_ref: format!("test:native-input/{touch}"),
    };
    let first = resolve_performance_touch(&binding, make(101)).unwrap();
    let second = resolve_performance_touch(&binding, make(102)).unwrap();
    for key in [
        "identity",
        "source_coordinate",
        "source_face",
        "tuning_ref",
        "member",
        "key",
        "position",
        "coordinate_face",
        "register_octave",
        "pitch_class",
        "fundamental_hz",
        "hertz",
        "ratio_numerator",
        "ratio_denominator",
        "exact_ratio",
        "phase_cos",
        "phase_sin",
    ] {
        assert_eq!(
            first[key], second[key],
            "independent touch changed native {key}"
        );
    }
    assert_ne!(first["touch"], second["touch"]);
    assert_ne!(first["touch_ref"], second["touch_ref"]);
    let expected = binding
        .targets()
        .key_target(4, -1, "test:native-input/101")
        .unwrap();
    assert_eq!(first["hertz"].as_f64().unwrap(), expected.hertz);
    assert_eq!(first["identity"], binding.determination()["identity"]);
    assert!(
        resolve_performance_touch(
            &binding,
            KeyTouch {
                key: 12,
                ..make(103)
            }
        )
        .is_err()
    );
    assert!(
        resolve_performance_touch(
            &binding,
            KeyTouch {
                touch: 0,
                ..make(103)
            }
        )
        .is_err()
    );
    assert!(
        resolve_performance_touch(
            &binding,
            KeyTouch {
                touch_ref: "test:bad\0input".into(),
                ..make(103)
            }
        )
        .is_err()
    );
    assert!(native_janko_catalog(&binding, 5, 0, 0).is_err());
    assert!(native_janko_catalog(&binding, 6, 0, 12).is_err());
}
