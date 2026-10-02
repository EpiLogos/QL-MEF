use ql_core::{ConjugationDegree, RelationFamily};
use ql_mef::m2_condition::{CorrespondenceRole, correspondence_field};
use ql_mef::m2_tuning_sources::*;
use ql_mef::music_determination::*;
use ql_mef::{ContextFrameId, LensId, MFace, MusicalBasis, PoleIdentity};
use serde_json::json;
fn provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:declared-root".into(),
        source_ref: "controlled:root".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
fn root() -> Fundamental {
    Fundamental::new(220.0, provenance()).unwrap()
}
fn determination(basis: MusicalBasis) -> MusicalDetermination {
    MusicalDetermination::new(
        PoleIdentity::new("event:source-collection", 7, 7, 570).unwrap(),
        "M1-4.4′",
        MFace::Pratibimba,
        basis,
        LensId::ALL[0],
        ContextFrameId::ALL[0],
        RelationSelection {
            family: RelationFamily::C,
            pair_index: 1,
            degree: ConjugationDegree::D3,
            expansion_side: None,
        },
    )
    .unwrap()
}
#[test]
fn every_available_path_retains_real_row_and_all_authentic_absences() {
    let field = correspondence_field();
    for rule in &field.rules {
        let retained = retained_condition_collection(rule.maqam_index, rule.role, &root()).unwrap();
        let expected = ql_mef::m2::maqam_pitches(rule.maqam_index, 220.0).unwrap();
        assert_eq!(retained.entries().len(), 8);
        assert_eq!(
            retained.standing(),
            &CollectionStanding::RetainedApproximation
        );
        assert_eq!(
            retained.source_receipt()["descriptor"]["index"],
            rule.maqam_index
        );
        assert_eq!(
            retained.source_receipt()["correspondence"],
            serde_json::to_value(rule).unwrap()
        );
        for (degree, entry) in retained.entries().iter().enumerate() {
            assert_eq!(entry.original_degree, degree as u16);
            assert_eq!(entry.octave_return, degree == 7);
            assert_eq!(entry.source_coordinate, rule.maqam_coordinate);
            let SourcePitchAvailability::RetainedQuarterToneSteps(steps) = entry.pitch else {
                panic!("retained approximation lost")
            };
            assert_eq!(220.0 * 2f64.powf(f64::from(steps) / 24.0), expected[degree]);
        }
        let missing =
            source_spelled_condition_collection(rule.maqam_index, rule.role, &root()).unwrap();
        assert_eq!(
            missing.standing(),
            &CollectionStanding::SourcePitchUnavailable
        );
        assert!(
            missing
                .entries()
                .iter()
                .all(|entry| matches!(entry.pitch, SourcePitchAvailability::Unavailable(_)))
        );
        assert_eq!(missing.source_receipt()["interval_literal"], "");
        assert_eq!(missing.source_receipt()["spelled_steps24"], json!(null));
    }
    assert_eq!(field.rules.len(), 127);
}
#[test]
fn all_seventeen_missing_relations_refuse_before_retained_pitch_selection() {
    let mut missing = 0;
    for index in 0..72 {
        for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
            if correspondence_field().rule(index, role).is_some() {
                continue;
            }
            missing += 1;
            assert!(
                retained_condition_collection(index, role, &root())
                    .unwrap_err()
                    .contains("source path")
            );
            assert!(source_spelled_condition_collection(index, role, &root()).is_err());
        }
    }
    assert_eq!(missing, 17);
}
#[test]
fn actual_rast_row_nine_is_retained_and_never_substituted_for_another_row() {
    let row = retained_condition_collection(9, CorrespondenceRole::Tonic, &root()).unwrap();
    let increments: Vec<_> = row
        .entries()
        .windows(2)
        .map(|pair| match (&pair[0].pitch, &pair[1].pitch) {
            (
                SourcePitchAvailability::RetainedQuarterToneSteps(a),
                SourcePitchAvailability::RetainedQuarterToneSteps(b),
            ) => b - a,
            _ => panic!("wrong source domain"),
        })
        .collect();
    assert_eq!(increments, vec![4, 3, 3, 4, 4, 3, 3]);
    assert!(row.entries().iter().all(|entry| entry.spelling.is_none()));
    let other = retained_condition_collection(3, CorrespondenceRole::Tonic, &root()).unwrap();
    assert!(
        row.verify_retained(&serde_json::to_value(other).unwrap())
            .is_err()
    );
}
#[test]
fn planetary_individual_ratios_preserve_pluto_and_suns_original_literal() {
    for (reference, ratio) in [
        ("#2-5-0/1", [1, 1]),
        ("#2-5-2", [9, 8]),
        ("#2-5-3", [5, 4]),
        ("#2-5-4", [4, 3]),
        ("#2-5-5", [3, 2]),
        ("#2-5-6", [5, 3]),
        ("#2-5-7", [15, 8]),
        ("#2-5-8", [9, 5]),
        ("#2-5-9", [9, 4]),
    ] {
        let collection = planetary_interval_collection(reference, &root()).unwrap();
        assert_eq!(collection.entries().len(), 1);
        assert_eq!(
            collection.entries()[0].pitch,
            SourcePitchAvailability::ExactRatio(ExactRatio::new(ratio[0], ratio[1]).unwrap())
        );
        assert!(!collection.entries()[0].octave_return);
        assert_eq!(collection.source_receipt()["original_ratio"], json!(ratio));
        assert!(
            collection.source_receipt()["source_node"]["canonical_properties"]
                .as_str()
                .unwrap()
                .contains("m_2_5_interval_from_root")
        );
    }
    let sun = planetary_interval_collection("#2-5-0/1", &root()).unwrap();
    assert_eq!(sun.source_receipt()["literal"], "Unison/Octave (1:1/2:1)");
    let earth = planetary_interval_collection("#2-5-0/1-0", &root()).unwrap();
    assert!(matches!(
        earth.entries()[0].pitch,
        SourcePitchAvailability::Unavailable(_)
    ));
    assert!(planetary_interval_collection("#2-5-uranus", &root()).is_err());
}
#[test]
fn complete_native_collection_comparison_detects_pitch_receipt_path_and_relation_loss() {
    let native = retained_condition_collection(9, CorrespondenceRole::Tonic, &root()).unwrap();
    let exact = serde_json::to_value(&native).unwrap();
    native.verify_retained(&exact).unwrap();
    for case in 0..5 {
        let mut lost = exact.clone();
        match case {
            0 => {
                lost["entries"].as_array_mut().unwrap().remove(2);
            }
            1 => {
                lost["entries"][2]["pitch"] = json!({"availability":"exact_ratio","value":{"numerator":"1","denominator":"1"}})
            }
            2 => lost["source_receipt"]["correspondence"]["musical_relations"] = json!([]),
            3 => lost["entries"][0]["source_coordinate"] = json!("#2-4"),
            _ => lost["standing"] = json!("source_authored_interval"),
        };
        assert!(native.verify_retained(&lost).is_err(), "loss case{case}");
    }
}
#[test]
fn native_rational_path_does_not_temper_or_fold_twelve_fifths() {
    let d = determination(MusicalBasis::Fifths);
    let path = rational_path_collection(&d, &[0, 1, 12, -1], &root()).unwrap();
    assert_eq!(
        path.entries()[2].pitch,
        SourcePitchAvailability::ExactRatio(ExactRatio::new(531441, 4096).unwrap())
    );
    assert_eq!(path.entries()[2].source_receipt["pitch_class"], 0);
    assert_eq!(
        path.entries()[3].pitch,
        SourcePitchAvailability::ExactRatio(ExactRatio::new(2, 3).unwrap())
    );
    assert!(
        path.entries()
            .iter()
            .all(|entry| entry.source_prime && !entry.octave_return)
    );
    assert!(rational_path_collection(&d, &[], &root()).is_err());
    assert!(rational_path_collection(&d, &[i32::MAX], &root()).is_err());
}
#[test]
fn actual_diatonic_target_operator_stays_separate_from_octet_and_return() {
    let policy = TuningPolicy::EqualTemperament12 {
        provenance: provenance(),
    };
    let mut targets =
        vimarsha_targets(determination(MusicalBasis::Chromatic), root(), policy).unwrap();
    let source = architectural_diatonic_collection(&targets).unwrap();
    assert_eq!(source.entries().len(), 8);
    assert_eq!(
        source.standing(),
        &CollectionStanding::ExplicitArchitecturalTuning
    );
    assert_eq!(
        source.source_receipt()["architectural_diatonic_cut"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    for (index, entry) in source.entries().iter().enumerate() {
        let SourcePitchAvailability::RetainedQuarterToneSteps(steps) = entry.pitch else {
            panic!("explicit temperament missing")
        };
        let actual = if index == 7 {
            targets.octave_return.hertz
        } else {
            targets.diatonic[index].hertz
        };
        assert!((220.0 * 2f64.powf(f64::from(steps) / 24.0) - actual).abs() < actual * 1e-14);
        assert_eq!(entry.octave_return, index == 7);
    }
    targets.diatonic[2].hertz += 1.0;
    assert!(architectural_diatonic_collection(&targets).is_err());
}
