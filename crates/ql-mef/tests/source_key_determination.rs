use ql_mef::MFace;
use ql_mef::continuous::coupled::CoupledBasis;
use ql_mef::m1_engine::{Basis, M1Engine};
use ql_mef::m2_condition::{CorrespondenceRole, correspondence_field};
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_tuning_sources::*;
use ql_mef::music_determination::*;
use ql_mef::performance_audio::{PreparedPerformanceBinding, prepare_native_performance};
use ql_mef::source_key_determination::*;
use ql_mef::{ContextFrameId, LensId, MCoordinate, MusicalBasis};
use serde_json::{Value, json};

#[path = "support/retained_performance.rs"]
mod retained_performance;

struct NativeFixture {
    prepared: PreparedPerformanceBinding,
    original_m2: M2Request,
    writer: MCoordinate,
}
impl NativeFixture {
    fn new() -> Self {
        let input = retained_performance::preparation();
        let original_m2 = input.coupled.compose().unwrap().m2_input;
        let prepared = prepare_native_performance(input).unwrap();
        let writer = ql_mef::m_tree::native_current_m_registry()
            .coordinate("#2-1", MFace::Pratibimba)
            .unwrap();
        Self {
            prepared,
            original_m2,
            writer,
        }
    }
    fn consumer(&self) -> SparseMusicalConsumer<'_> {
        SparseMusicalConsumer {
            basis: self.prepared.native_basis(),
            writer: &self.writer,
            phase: self.prepared.targets().determination.identity().tick12() / 6,
            condition: Some(SparseConditionConsumer {
                producer_input: &self.original_m2,
                plan: self.prepared.relation_plan(),
            }),
        }
    }
    fn input(&self, collection: TuningSourceCollection) -> SparseKeyPreparation<'_> {
        SparseKeyPreparation {
            determination: self.prepared.targets().determination.clone(),
            collection,
            reduction: seven_keys(),
            octet: None,
            requirement: SourcePitchRequirement::DeclaredAvailable,
            consumer: self.consumer(),
        }
    }
    fn condition_collection(&self, spelled: bool) -> TuningSourceCollection {
        let condition = &self.prepared.relation_plan().execution.condition_input;
        let producer = if spelled {
            source_spelled_condition_collection
        } else {
            retained_condition_collection
        };
        producer(condition.maqam_index, condition.role, &fundamental()).unwrap()
    }
}
fn provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:explicit-active-source-keys".into(),
        source_ref: "reference:performer-key-degree-assignments".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
fn fundamental() -> Fundamental {
    Fundamental::new(220.0, provenance()).unwrap()
}
fn seven_keys() -> ActiveKeyReduction {
    // An explicit supplied key policy; these particular keys are not derived
    // from a source maqam's tuning, nearest pitch class or conventional layout.
    ActiveKeyReduction {
        provenance: provenance(),
        assignments: [0, 2, 4, 5, 7, 9, 11]
            .into_iter()
            .enumerate()
            .map(|(degree, key)| KeyDegreeAssignment {
                key,
                source_degree: degree as u16,
                octave: 0,
            })
            .collect(),
    }
}
fn one_key(key: u8, degree: u16, octave: i8) -> ActiveKeyReduction {
    ActiveKeyReduction {
        provenance: provenance(),
        assignments: vec![KeyDegreeAssignment {
            key,
            source_degree: degree,
            octave,
        }],
    }
}
fn explicit_octet() -> SourceOctetReduction {
    SourceOctetReduction {
        provenance: provenance(),
        assignments: (0..8)
            .map(|component| ComponentDegreeAssignment {
                component,
                source_degree: u16::from(component),
                octave: 0,
            })
            .collect(),
    }
}
fn available(target: SourceKeyTarget) -> SparseNoteTarget {
    match target {
        SourceKeyTarget::Available(target) => *target,
        SourceKeyTarget::Unavailable(t) => panic!("{}", t.reason()),
    }
}

#[test]
fn seven_explicit_source_keys_are_real_targets_and_five_inactive_keys_have_no_event_target() {
    let fixture = NativeFixture::new();
    let source = fixture.condition_collection(false);
    let targets = SparseKeyTargets::prepare(fixture.input(source.clone())).unwrap();
    let expected = ql_mef::m2::maqam_pitches(9, 220.0).unwrap();
    let mut active = 0;
    for key in 0..12 {
        let target = targets
            .key_target(key, 0, &format!("touch:row/0/key/{key}"))
            .unwrap();
        if let Some(assignment) = seven_keys().assignments.into_iter().find(|a| a.key == key) {
            active += 1;
            let target = available(target);
            assert_eq!(
                target.note().hertz,
                expected[usize::from(assignment.source_degree)]
            );
            assert!(target.note().exact_ratio.is_none());
            assert_eq!(
                target.note().identity,
                *fixture.prepared.targets().determination.identity()
            );
            assert_eq!(
                target.note().source_coordinate,
                *fixture.prepared.targets().determination.coordinate()
            );
            assert_eq!(
                target.source_entry(),
                source.entry(assignment.source_degree).unwrap()
            );
            assert_eq!(
                target.collection().standing(),
                &CollectionStanding::RetainedApproximation
            );
            let wire = target.receipt().unwrap();
            assert_eq!(wire["source_degree"], assignment.source_degree);
            assert_eq!(
                wire["native_target"]["hertz"],
                expected[usize::from(assignment.source_degree)]
            );
            assert_eq!(wire["exact_ratio"], Value::Null);
            let high = available(targets.key_target(key, 1, "touch:row/5/member/b").unwrap());
            assert_eq!(high.note().hertz, target.note().hertz * 2.0);
            assert_eq!(high.applied_octave_ratio(), ExactRatio::new(2, 1).unwrap());
            assert_ne!(high.note().touch_ref, target.note().touch_ref);
        } else {
            assert!(target.note().is_none());
            let wire = target.receipt().unwrap();
            assert_eq!(wire["available"], false);
            assert_eq!(wire["reason"], "source-key-unassigned");
            assert!(wire.get("native_target").is_none());
            assert!(wire.get("hertz").is_none());
        }
    }
    assert_eq!(active, 7);
    assert_eq!(
        targets.nodal_quartet(),
        fixture.prepared.targets().determination.nodal_quartet()
    );
    assert!(
        targets
            .audio_octet_targets(0)
            .unwrap_err()
            .contains("mapping-unavailable")
    );
    targets
        .validate_coupled_consumer(fixture.consumer(), &source)
        .unwrap();
}

#[test]
fn complete_actual_m2_path_is_required_and_other_valid_maqam_or_role_cannot_replace_it() {
    let fixture = NativeFixture::new();
    let current = fixture.condition_collection(false);
    let targets = SparseKeyTargets::prepare(fixture.input(current.clone())).unwrap();
    assert_eq!(current.source_receipt()["correspondence"]["maqam_index"], 9);
    assert_eq!(current.source_receipt()["correspondence"]["role"], "tonic");
    let other =
        retained_condition_collection(3, CorrespondenceRole::Tonic, &fundamental()).unwrap();
    assert!(
        SparseKeyTargets::prepare(fixture.input(other.clone()))
            .unwrap_err()
            .contains("maqam/role")
    );
    assert!(
        targets
            .validate_coupled_consumer(fixture.consumer(), &other)
            .is_err()
    );
    let mut input = fixture.input(current.clone());
    input.consumer.condition = None;
    assert!(
        SparseKeyTargets::prepare(input)
            .unwrap_err()
            .contains("relation-plan replay")
    );
    let mut damaged_plan = fixture.prepared.relation_plan().clone();
    damaged_plan.execution.condition_input.maqam_index = 3;
    let mut input = fixture.input(current);
    input.consumer.condition = Some(SparseConditionConsumer {
        producer_input: &fixture.original_m2,
        plan: &damaged_plan,
    });
    assert!(SparseKeyTargets::prepare(input).is_err());
    let mut missing = 0;
    for index in 0..72 {
        for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
            if correspondence_field().rule(index, role).is_none() {
                missing += 1;
                assert!(retained_condition_collection(index, role, &fundamental()).is_err());
            }
        }
    }
    assert_eq!(missing, 17);
}

#[test]
fn authentic_absence_and_declared_approximation_remain_distinct_at_native_admission() {
    let fixture = NativeFixture::new();
    let missing = fixture.condition_collection(true);
    let targets = SparseKeyTargets::prepare(fixture.input(missing.clone())).unwrap();
    for key in 0..12 {
        let target = targets.key_target(key, 0, "touch:missing").unwrap();
        assert!(target.note().is_none());
        assert_eq!(target.receipt().unwrap()["available"], false);
    }
    for requirement in [
        SourcePitchRequirement::AuthenticCondition,
        SourcePitchRequirement::SourceAuthoredExact,
    ] {
        let mut input = fixture.input(fixture.condition_collection(false));
        input.requirement = requirement;
        assert!(SparseKeyTargets::prepare(input).is_err());
        let mut input = fixture.input(missing.clone());
        input.requirement = requirement;
        assert!(SparseKeyTargets::prepare(input).is_err());
    }
    for rule in &correspondence_field().rules {
        let original =
            source_spelled_condition_collection(rule.maqam_index, rule.role, &fundamental())
                .unwrap();
        assert!(
            original
                .entries()
                .iter()
                .all(|entry| matches!(entry.pitch, SourcePitchAvailability::Unavailable(_)))
        );
    }
    assert_eq!(correspondence_field().rules.len(), 127);
}

#[test]
fn authored_individual_intervals_keep_original_ratios_octaves_literals_and_earth_absence() {
    let fixture = NativeFixture::new();
    for (reference, n, d) in [
        ("#2-5-0/1", 1, 1),
        ("#2-5-2", 9, 8),
        ("#2-5-3", 5, 4),
        ("#2-5-4", 4, 3),
        ("#2-5-5", 3, 2),
        ("#2-5-6", 5, 3),
        ("#2-5-7", 15, 8),
        ("#2-5-8", 9, 5),
        ("#2-5-9", 9, 4),
    ] {
        let collection = planetary_interval_collection(reference, &fundamental()).unwrap();
        let mut input = fixture.input(collection.clone());
        input.reduction = one_key(10, 0, 1);
        input.requirement = SourcePitchRequirement::SourceAuthoredExact;
        let targets = SparseKeyTargets::prepare(input).unwrap();
        let target = available(targets.key_target(10, -1, "touch:individual").unwrap());
        assert_eq!(
            target.note().exact_ratio,
            Some(ExactRatio::new(n, d).unwrap())
        );
        assert_eq!(
            target.note().hertz,
            220.0 * ExactRatio::new(n, d).unwrap().as_f64()
        );
        assert_eq!(
            target.applied_octave_ratio(),
            ExactRatio::new(1, 1).unwrap()
        );
        assert!(!target.source_entry().octave_return);
        assert_eq!(
            target.source_entry().pitch,
            SourcePitchAvailability::ExactRatio(ExactRatio::new(n, d).unwrap())
        );
        assert_eq!(
            target.receipt().unwrap()["source_receipt"],
            collection.entries()[0].source_receipt
        );
        let mut required = fixture.input(collection);
        required.reduction = one_key(10, 0, 0);
        required.requirement = SourcePitchRequirement::AuthenticCondition;
        assert!(SparseKeyTargets::prepare(required).is_err());
    }
    let pluto = planetary_interval_collection("#2-5-9", &fundamental()).unwrap();
    let mut input = fixture.input(pluto);
    input.reduction = one_key(0, 0, 1);
    let target = available(
        SparseKeyTargets::prepare(input)
            .unwrap()
            .key_target(0, 0, "touch:pluto")
            .unwrap(),
    );
    assert_eq!(
        target.note().exact_ratio,
        Some(ExactRatio::new(9, 2).unwrap())
    );
    let mut input =
        fixture.input(planetary_interval_collection("#2-5-0/1-0", &fundamental()).unwrap());
    input.reduction = one_key(0, 0, 0);
    let absent = SparseKeyTargets::prepare(input)
        .unwrap()
        .key_target(0, 0, "touch:earth")
        .unwrap();
    assert!(absent.note().is_none());
    assert_eq!(
        absent.receipt().unwrap()["reason"],
        "source-has-no-planetary-interval-ratio"
    );
}

#[test]
fn octave_return_and_complete_separate_component_mapping_are_explicit_and_never_cyclic_filled() {
    let fixture = NativeFixture::new();
    let source = fixture.condition_collection(false);
    let mut input = fixture.input(source.clone());
    input.octet = Some(explicit_octet());
    input.reduction = one_key(8, 7, -1);
    let targets = SparseKeyTargets::prepare(input).unwrap();
    let note = available(targets.key_target(8, 0, "touch:explicit-return").unwrap());
    assert_eq!(note.note().hertz, 220.0);
    assert!(note.source_entry().octave_return);
    let octet = targets.audio_octet_targets(0).unwrap();
    let expected = ql_mef::m2::maqam_pitches(9, 220.0).unwrap();
    for (index, target) in octet.iter().enumerate() {
        assert_eq!(target.hertz(), expected[index]);
        assert_eq!(target.assignment().source_degree, index as u16);
        assert_eq!(target.component(), index as u8);
        assert!(target.exact_ratio().is_none());
        assert_eq!(
            target.receipt()["source_entry"]["octave_return"],
            index == 7
        );
    }
    assert_eq!(targets.nodal_quartet().positions.len(), 4);
    assert_eq!(
        targets.nodal_quartet(),
        &fixture.prepared.targets().nodal_quartet
    );
    for case in 0..3 {
        let mut octet = explicit_octet();
        match case {
            0 => {
                octet.assignments.pop();
            }
            1 => octet.assignments[7].component = 0,
            _ => octet.assignments[7].source_degree = 99,
        };
        let mut input = fixture.input(source.clone());
        input.octet = Some(octet);
        assert!(SparseKeyTargets::prepare(input).is_err());
    }
    let mut input = fixture.input(fixture.condition_collection(true));
    input.octet = Some(explicit_octet());
    assert!(SparseKeyTargets::prepare(input).is_err());
}

#[test]
fn actual_joined_independent_revisions_reject_stale_wrong_face_wrong_branch_and_disconnected_receipts()
 {
    let fixture = NativeFixture::new();
    let collection = fixture.condition_collection(false);
    let targets = SparseKeyTargets::prepare(fixture.input(collection.clone())).unwrap();
    assert_ne!(
        targets.determination().identity().profile_generation(),
        targets.m2_generation()
    );
    targets
        .validate_coupled_consumer(fixture.consumer(), &collection)
        .unwrap();
    let registry = ql_mef::m_tree::native_current_m_registry();
    for writer in [
        registry.coordinate("#2-1", MFace::Bimba).unwrap(),
        registry.coordinate("#2-5-5", MFace::Pratibimba).unwrap(),
        MCoordinate::parse_source("#2-1", MFace::Pratibimba).unwrap(),
    ] {
        let mut consumer = fixture.consumer();
        consumer.writer = &writer;
        assert!(
            targets
                .validate_coupled_consumer(consumer, &collection)
                .is_err()
        );
    }
    let mut consumer = fixture.consumer();
    consumer.phase ^= 1;
    assert!(
        targets
            .validate_coupled_consumer(consumer, &collection)
            .is_err()
    );
    for case in 0..4 {
        let mut basis = fixture.prepared.native_basis().clone();
        match case {
            0 => basis.input.m1.revision = "12".into(),
            1 => basis.m2_input.stamp.identity.profile_generation += 1,
            2 => basis.m2_input.stamp.identity.event_ref = "event:detached".into(),
            _ => basis.input.source_receipts.clear(),
        };
        let mut consumer = fixture.consumer();
        consumer.basis = &basis;
        assert!(
            targets
                .validate_coupled_consumer(consumer, &collection)
                .is_err(),
            "case{case}"
        );
    }
}

#[test]
fn retained_preparation_cannot_drop_source_pitch_ratio_policy_face_or_octet_availability() {
    let fixture = NativeFixture::new();
    let source = fixture.condition_collection(false);
    let targets = SparseKeyTargets::prepare(fixture.input(source)).unwrap();
    let retained = targets.preparation_receipt().unwrap();
    targets.verify_retained(&retained).unwrap();
    for case in 0..6 {
        let mut changed = retained.clone();
        match case {
            0 => {
                changed["source_collection"]["entries"][0]["pitch"] = json!({"availability":"exact_ratio","value":{"numerator":"1","denominator":"1"}})
            }
            1 => {
                changed["source_collection"]["source_receipt"]["correspondence"]["claims"] =
                    json!([])
            }
            2 => changed["reduction"]["assignments"][0]["source_degree"] = json!(7),
            3 => changed["native_source_coordinate"]["face"] = json!("bimba"),
            4 => changed["octet_reduction"] = json!({"inferred":"cycle-seven"}),
            _ => changed["key_availability"][1]["available"] = json!(true),
        };
        assert!(targets.verify_retained(&changed).is_err(), "case{case}");
    }
}

#[test]
fn all_168_actual_basis_lens_context_selections_keep_source_degrees_separate_from_architectural_keys()
 {
    let fixture = NativeFixture::new();
    let mut count = 0;
    for basis in MusicalBasis::ALL {
        for lens in LensId::ALL {
            for (frame, cf) in ContextFrameId::ALL.into_iter().enumerate() {
                let mut input = fixture.prepared.native_basis().input.clone();
                input.m1.basis = match basis {
                    MusicalBasis::Chromatic => Basis::Chromatic,
                    MusicalBasis::Fifths => Basis::Fifths,
                };
                input.m1.lens12 = lens.slot();
                input.m1.context_frame = frame as u8 + 1;
                let joined: CoupledBasis = input.compose().unwrap();
                let engine = M1Engine::new(input.m1).unwrap();
                let determination = MusicalDetermination::from_engine(
                    &engine,
                    MFace::Pratibimba,
                    retained_performance::preparation().relation,
                )
                .unwrap();
                assert_eq!(determination.context_frame(), cf);
                let actual = vimarsha_targets(
                    determination.clone(),
                    fundamental(),
                    fixture.prepared.targets().tuning_policy.clone(),
                )
                .unwrap();
                let source = architectural_diatonic_collection(&actual).unwrap();
                let consumer = SparseMusicalConsumer {
                    basis: &joined,
                    writer: &fixture.writer,
                    phase: determination.identity().tick12() / 6,
                    condition: None,
                };
                let targets = SparseKeyTargets::prepare(SparseKeyPreparation {
                    determination,
                    collection: source,
                    reduction: seven_keys(),
                    octet: None,
                    requirement: SourcePitchRequirement::DeclaredAvailable,
                    consumer,
                })
                .unwrap();
                for assignment in &targets.reduction().assignments {
                    let note = available(
                        targets
                            .key_target(assignment.key, 0, "touch:all-native-selections")
                            .unwrap(),
                    );
                    assert_eq!(
                        note.note().hertz,
                        actual.diatonic[usize::from(assignment.source_degree)].hertz
                    );
                    assert_eq!(
                        note.note().exact_ratio,
                        actual.diatonic[usize::from(assignment.source_degree)].exact_ratio
                    );
                    assert_eq!(
                        note.note().pitch_class,
                        ql_mef::pitch_at_lens(basis, lens, note.note().coordinate)
                    );
                    assert_eq!(
                        note.source_entry().original_degree,
                        assignment.source_degree
                    );
                }
                assert_eq!(targets.nodal_quartet(), &actual.nodal_quartet);
                assert!(targets.audio_octet_targets(0).is_err());
                count += 1;
            }
        }
    }
    assert_eq!(count, 168);
}

#[test]
fn unfolded_rational_source_path_is_not_pitch_class_closed_and_wrong_source_face_refuses() {
    let fixture = NativeFixture::new();
    let mut input = fixture.prepared.native_basis().input.clone();
    input.m1.basis = Basis::Fifths;
    let joined = input.compose().unwrap();
    let engine = M1Engine::new(input.m1).unwrap();
    let d = MusicalDetermination::from_engine(
        &engine,
        MFace::Pratibimba,
        retained_performance::preparation().relation,
    )
    .unwrap();
    let source = rational_path_collection(&d, &[0, 1, 12, -1], &fundamental()).unwrap();
    let preparation = SparseKeyPreparation {
        determination: d.clone(),
        collection: source.clone(),
        reduction: one_key(0, 2, -7),
        octet: None,
        requirement: SourcePitchRequirement::DeclaredAvailable,
        consumer: SparseMusicalConsumer {
            basis: &joined,
            writer: &fixture.writer,
            phase: d.identity().tick12() / 6,
            condition: None,
        },
    };
    let target = available(
        SparseKeyTargets::prepare(preparation)
            .unwrap()
            .key_target(0, 0, "touch:comma")
            .unwrap(),
    );
    assert_eq!(
        target.source_entry().pitch,
        SourcePitchAvailability::ExactRatio(ExactRatio::new(531441, 4096).unwrap())
    );
    assert_eq!(
        target.note().exact_ratio,
        Some(ExactRatio::new(531441, 524288).unwrap())
    );
    assert_ne!(target.note().hertz, 220.0);
    assert_eq!(target.source_entry().source_receipt["pitch_class"], 0);
    let wrong = MusicalDetermination::from_engine(
        &engine,
        MFace::Bimba,
        retained_performance::preparation().relation,
    )
    .unwrap();
    let wrong_source = rational_path_collection(&wrong, &[0, 1, 12, -1], &fundamental()).unwrap();
    assert!(
        SparseKeyTargets::prepare(SparseKeyPreparation {
            determination: d.clone(),
            collection: wrong_source,
            reduction: one_key(0, 2, 0),
            octet: None,
            requirement: SourcePitchRequirement::DeclaredAvailable,
            consumer: SparseMusicalConsumer {
                basis: &joined,
                writer: &fixture.writer,
                phase: d.identity().tick12() / 6,
                condition: None
            }
        })
        .is_err()
    );
}

#[test]
fn malformed_duplicate_unbounded_and_overflowed_operations_produce_no_target() {
    let fixture = NativeFixture::new();
    for case in 0..5 {
        let mut input = fixture.input(fixture.condition_collection(false));
        match case {
            0 => input
                .reduction
                .assignments
                .push(input.reduction.assignments[0]),
            1 => input.reduction.assignments[0].key = 12,
            2 => input.reduction.assignments[0].source_degree = 99,
            3 => input.reduction.provenance.policy_ref.push('\0'),
            _ => input.reduction.assignments[0].octave = 127,
        };
        assert!(SparseKeyTargets::prepare(input).is_err(), "case{case}");
    }
    let targets =
        SparseKeyTargets::prepare(fixture.input(fixture.condition_collection(false))).unwrap();
    assert!(targets.key_target(12, 0, "touch:invalid").is_err());
    assert!(targets.key_target(0, 127, "touch:invalid").is_err());
    assert!(targets.key_target(0, 0, "touch:invalid\0").is_err());
    let immense = Fundamental::new(f64::MAX, provenance()).unwrap();
    let collection = planetary_interval_collection("#2-5-9", &immense).unwrap();
    let mut input = fixture.input(collection);
    input.reduction = one_key(0, 0, 0);
    assert!(SparseKeyTargets::prepare(input).is_err());
}
