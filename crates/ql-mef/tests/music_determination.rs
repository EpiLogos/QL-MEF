use ql_core::{ConjugationDegree, ExpansionSide, QlFace, RelationFamily};
use ql_mef::m1_engine::{Basis, EngineConfig, M1Engine};
use ql_mef::m2_engine::{EventIdentity, InputStamp};
use ql_mef::music_determination::*;
use ql_mef::{ContextFrameId, LensId, MFace, MusicalBasis, PoleIdentity};

fn relation() -> RelationSelection {
    RelationSelection {
        family: RelationFamily::C,
        pair_index: 1,
        degree: ConjugationDegree::D3,
        expansion_side: None,
    }
}
fn identity() -> PoleIdentity {
    PoleIdentity::new("event:physical-music", 7, 7, 570).unwrap()
}
fn selection(basis: MusicalBasis, lens: LensId, cf: ContextFrameId) -> MusicalDetermination {
    MusicalDetermination::new(
        identity(),
        "M1-4.4′",
        MFace::Pratibimba,
        basis,
        lens,
        cf,
        relation(),
    )
    .unwrap()
}
fn provenance(standing: TuningStanding) -> TuningProvenance {
    TuningProvenance {
        policy_ref: "tuning:performer-rational-targets".into(),
        source_ref: "score:physical-music/session-1".into(),
        revision: "7".into(),
        standing,
    }
}
fn fundamental() -> Fundamental {
    Fundamental::new(256.0, provenance(TuningStanding::External)).unwrap()
}
fn exact_policy() -> TuningPolicy {
    // Declared Pythagorean pitch targets, independent of the chosen QL basis.
    // This is a supplied tuning, not a replacement universal theory registry.
    let pairs = [
        (1, 1),
        (256, 243),
        (9, 8),
        (32, 27),
        (81, 64),
        (4, 3),
        (729, 512),
        (3, 2),
        (128, 81),
        (27, 16),
        (16, 9),
        (243, 128),
    ];
    TuningPolicy::ExactPitchRatios {
        ratios: pairs.map(|(n, d)| ExactRatio::new(n, d).unwrap()),
        provenance: provenance(TuningStanding::External),
    }
}
fn engine(basis: MusicalBasis, lens: LensId, cf: usize) -> M1Engine {
    M1Engine::new(EngineConfig {
        event_ref: "event:physical-music".into(),
        subject_coordinate: "M1-4.4".into(),
        selected_coordinate: "M1-4.4".into(),
        revision: "7".into(),
        cycle: "1".into(),
        tick12: 7,
        family: 5,
        row12: 3,
        col12: 4,
        flowering_substage: 4,
        lens12: lens.slot(),
        context_frame: cf as u8 + 1,
        basis: match basis {
            MusicalBasis::Chromatic => Basis::Chromatic,
            MusicalBasis::Fifths => Basis::Fifths,
        },
    })
    .unwrap()
}

#[test]
fn all_168_native_engine_selections_preserve_coordinates_modes_and_target_identity() {
    let anchor_tables = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [0, 6, 7, 1, 2, 8, 9, 3, 4, 10, 11, 5],
    ];
    let octets = [[2, 4, 6, 8, 3, 5, 7, 9], [7, 2, 9, 4, 1, 8, 3, 10]];
    let quartets = [[0, 10, 1, 11], [0, 11, 6, 5]];
    let ionian = [0, 2, 4, 5, 7, 9, 11];
    let mut seen = std::collections::HashSet::new();
    for (basis_index, basis) in MusicalBasis::ALL.into_iter().enumerate() {
        for (lens_index, lens) in LensId::ALL.into_iter().enumerate() {
            let anchor = anchor_tables[basis_index][lens_index];
            for (mode, cf) in ContextFrameId::ALL.into_iter().enumerate() {
                let native = engine(basis, lens, mode);
                let d = MusicalDetermination::from_engine(&native, MFace::Pratibimba, relation())
                    .unwrap();
                assert!(seen.insert((basis, lens, cf)));
                assert_eq!(d.coordinate().source_ref, "#1-4.4");
                assert_eq!(d.coordinate().face, MFace::Pratibimba);
                assert_eq!(d.identity(), &native.pole_identity().unwrap());
                assert_eq!(d.identity().degree720(), 570);
                assert_eq!(d.lens_anchor().pitch, anchor);
                assert_eq!(d.context_frame(), cf);
                let expected_octet = octets[basis_index].map(|pitch| (pitch + anchor) % 12);
                let expected_quartet = quartets[basis_index].map(|pitch| (pitch + anchor) % 12);
                assert_eq!(
                    d.explicate_octet().positions.map(|p| p.pitch_class),
                    expected_octet
                );
                assert_eq!(
                    d.nodal_quartet().positions.map(|p| p.pitch_class),
                    expected_quartet
                );
                assert_eq!(
                    d.mode().pitches,
                    std::array::from_fn(|index| (anchor + ionian[(mode + index) % 7]) % 12)
                );
                assert_eq!(d.mode().tonic, (anchor + ionian[mode]) % 12);
                let targets = vimarsha_targets(d, fundamental(), exact_policy()).unwrap();
                assert_eq!(targets.audio_octet.map(|p| p.pitch_class), expected_octet);
                assert_eq!(
                    targets.nodal_quartet.positions.map(|p| p.pitch_class),
                    expected_quartet
                );
                for target in targets.audio_octet {
                    assert_eq!(target.hertz, 256.0 * target.exact_ratio.unwrap().as_f64());
                }
                let scale = targets.octave_process_hz();
                assert!(scale.windows(2).all(|pair| pair[0] < pair[1]));
                assert_eq!(scale[7], scale[0] * 2.0);
            }
        }
    }
    assert_eq!(seen.len(), 168);
}

#[test]
fn canonical_twelve_keys_are_native_address_targets_not_eight_bus_indices() {
    let local_fields = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        [0, 6, 7, 1, 2, 8, 9, 3, 4, 10, 11, 5],
    ];
    for (basis_index, basis) in MusicalBasis::ALL.into_iter().enumerate() {
        for lens in LensId::ALL {
            let targets = vimarsha_targets(
                selection(basis, lens, ContextFrameId::ALL[0]),
                fundamental(),
                exact_policy(),
            )
            .unwrap();
            let anchor = targets.determination.lens_anchor().pitch;
            let mut seen = std::collections::HashSet::new();
            for key in 0..12 {
                let low = targets.key_target(key, -1, "touch:a").unwrap();
                let high = targets.key_target(key, 1, "touch:b").unwrap();
                let pitch = (local_fields[basis_index][usize::from(key)] + anchor) % 12;
                assert_eq!(low.pitch_class, pitch);
                assert!(seen.insert(pitch));
                assert_eq!(high.hertz, low.hertz * 4.0);
                assert_eq!(
                    high.exact_ratio.unwrap(),
                    low.exact_ratio
                        .unwrap()
                        .compose(ExactRatio::new(4, 1).unwrap())
                        .unwrap()
                );
                assert_eq!(low.coordinate, high.coordinate);
                assert_eq!(low.identity, *targets.determination.identity());
                assert_eq!(low.source_coordinate, *targets.determination.coordinate());
                assert_eq!(low.source_coordinate.face, MFace::Pratibimba);
                assert_eq!(low.tuning_provenance, *targets.tuning_policy.provenance());
                assert_ne!(low.touch_ref, high.touch_ref);
                assert_eq!(
                    targets.note_target(low.coordinate, -1, "touch:a").unwrap(),
                    low
                );
            }
            assert_eq!(seen.len(), 12);
            assert!(targets.key_target(12, 0, "touch").is_err());
            assert!(targets.key_target(0, 0, "touch\0bad").is_err());
            assert_eq!(targets.audio_octet.len(), 8);
            assert_eq!(targets.nodal_quartet.positions.len(), 4);
        }
    }
}

#[test]
fn actual_coupled_owner_preserves_distinct_source_and_composition_generations() {
    use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
    let seed: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(seed["config"].clone()).unwrap();
    let mut m2: ql_mef::m2_engine::M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_seed: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut m3: ql_mef::m3_state::M3Request =
        serde_json::from_value(m3_seed["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m1.revision = "7".into();
    m2.stamp.identity.profile_generation = 91;
    // M2-owned subordinate stamps must retain its own generation.
    let mut raw = serde_json::to_value(&m2).unwrap();
    fn replace_stamps(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(identity) = map.get_mut("identity") {
                    identity["profile_generation"] = serde_json::json!(91);
                }
                for child in map.values_mut() {
                    replace_stamps(child);
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    replace_stamps(child);
                }
            }
            _ => {}
        }
    }
    replace_stamps(&mut raw);
    m2 = serde_json::from_value(raw).unwrap();
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let input = CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![],
        condition_frequency_bindings: vec![],
        sky_frequency_bindings: vec![],
        source_receipts: vec![],
    };
    let basis = input.compose().unwrap();
    let d = MusicalDetermination::from_engine(
        &M1Engine::new(input.m1.clone()).unwrap(),
        MFace::Pratibimba,
        relation(),
    )
    .unwrap();
    let writer = ql_mef::m_tree::native_current_m_registry()
        .coordinate("#2-1", MFace::Pratibimba)
        .unwrap();
    assert_eq!(basis.m2_input.stamp.identity.profile_generation, 91);
    assert_eq!(d.identity().profile_generation(), 7);
    assert!(
        d.validate_coupled_consumer(&basis, &writer, d.identity().tick12() / 6)
            .is_ok()
    );
    let wrong = ql_mef::m_tree::native_current_m_registry()
        .coordinate("#2-5-5", MFace::Pratibimba)
        .unwrap();
    assert!(
        d.validate_coupled_consumer(&basis, &wrong, d.identity().tick12() / 6)
            .is_err()
    );
    let mut disconnected = basis.clone();
    disconnected.m2_input.stamp.identity.event_ref = "event:other".into();
    assert!(
        d.validate_coupled_consumer(&disconnected, &writer, d.identity().tick12() / 6)
            .is_err()
    );
    let mut stale = basis.clone();
    stale.input.m1.revision = "8".into();
    assert!(
        d.validate_coupled_consumer(&stale, &writer, d.identity().tick12() / 6)
            .is_err()
    );
}

#[test]
fn eight_inner_positions_are_neither_conventional_octatonic_nor_scale_plus_return() {
    let d = selection(MusicalBasis::Chromatic, LensId::L0, ContextFrameId::Cf1);
    assert_eq!(
        d.explicate_octet().operator,
        CollectionOperator::ExplicateInnerPositions
    );
    assert_eq!(
        d.explicate_octet().positions.map(|p| p.pitch_class),
        [2, 4, 6, 8, 3, 5, 7, 9]
    );
    let mut sorted = d.explicate_octet().positions.map(|p| p.pitch_class);
    sorted.sort();
    assert_eq!(sorted, [2, 3, 4, 5, 6, 7, 8, 9]);
    // A conventional alternating octatonic collection has four gaps1/four2.
    let circular_gaps: Vec<u8> = (0..8)
        .map(|i| (sorted[(i + 1) % 8] + 12 - sorted[i]) % 12)
        .collect();
    assert_ne!(circular_gaps.iter().filter(|gap| **gap == 2).count(), 4);
    assert_eq!(
        d.nodal_quartet().positions.map(|p| p.pitch_class),
        [0, 10, 1, 11]
    );
    assert_eq!(d.mode().pitches, [0, 2, 4, 5, 7, 9, 11]);
    assert_eq!(
        d.diatonic_cut().operator,
        CollectionOperator::ContextFrameDiatonic
    );
    assert_eq!(
        d.diatonic_cut()
            .positions
            .map(|p| p.coordinate.position.value()),
        [0, 1, 2, 2, 3, 4, 5]
    );
    assert_eq!(
        d.diatonic_cut().positions.map(|p| p.coordinate.face),
        [
            QlFace::Direct,
            QlFace::Direct,
            QlFace::Direct,
            QlFace::Conjugate,
            QlFace::Conjugate,
            QlFace::Conjugate,
            QlFace::Conjugate
        ]
    );
    let inner: std::collections::HashSet<_> = d
        .explicate_octet()
        .positions
        .into_iter()
        .map(|p| p.pitch_class)
        .collect();
    assert_eq!(
        d.mode()
            .pitches
            .into_iter()
            .filter(|pitch| inner.contains(pitch))
            .count(),
        5
    );
    let targets = vimarsha_targets(d, fundamental(), exact_policy()).unwrap();
    assert_ne!(targets.audio_octet_hz(), targets.octave_process_hz());
    assert_eq!(targets.octave_process_hz()[0], 256.0);
    assert_eq!(targets.octave_process_hz()[7], 512.0);
}

#[test]
fn source_face_lens_face_and_clock_phase_cannot_collapse_into_one_bit() {
    let direct = MusicalDetermination::new(
        identity(),
        "#1-4.4",
        MFace::Bimba,
        MusicalBasis::Fifths,
        LensId::L2,
        ContextFrameId::Cf5,
        relation(),
    )
    .unwrap();
    let prime = selection(MusicalBasis::Fifths, LensId::L2Prime, ContextFrameId::Cf5);
    assert!(direct.coordinate().same_structural_path(prime.coordinate()));
    assert_ne!(direct.coordinate().face, prime.coordinate().face);
    assert_eq!(direct.identity(), prime.identity());
    assert_eq!(direct.identity().tick12(), 7);
    assert_eq!(direct.lens_anchor().pitch, 2);
    assert_eq!(prime.lens_anchor().pitch, 8);
    assert_ne!(direct.explicate_octet(), prime.explicate_octet());
    assert_eq!(prime.requested_coordinate(), "M1-4.4′");
    assert!(
        MusicalDetermination::new(
            identity(),
            "M1-4.4′",
            MFace::Bimba,
            MusicalBasis::Fifths,
            LensId::L2,
            ContextFrameId::Cf5,
            relation()
        )
        .is_err()
    );
    for reference in ["M1-4.4-65535′", "#1-4-4", "M2-1′"] {
        assert!(
            MusicalDetermination::new(
                identity(),
                reference,
                MFace::Pratibimba,
                MusicalBasis::Fifths,
                LensId::L2Prime,
                ContextFrameId::Cf5,
                relation()
            )
            .is_err()
        );
    }
}

#[test]
fn every_registered_m1_descendant_and_alias_retains_its_exact_source_path() {
    let registry = ql_mef::m_tree::native_current_m_registry();
    let mut visited = 0;
    for node in registry
        .manifest()
        .nodes
        .iter()
        .filter(|node| node.root_position == Some(1))
    {
        for reference in std::iter::once(&node.source_ref).chain(node.aliases.iter()) {
            for face in [MFace::Bimba, MFace::Pratibimba] {
                let d = MusicalDetermination::new(
                    identity(),
                    reference,
                    face,
                    MusicalBasis::Chromatic,
                    LensId::L0,
                    ContextFrameId::Cf1,
                    relation(),
                )
                .unwrap();
                assert_eq!(&d.coordinate().source_ref, &node.source_ref);
                assert_eq!(&d.coordinate().aliases, &node.aliases);
                assert_eq!(d.coordinate().face, face);
                assert!(!d.coordinate().provenance.is_empty());
                assert_eq!(d.coordinate().root, 1);
                visited += 1;
            }
        }
    }
    assert!(
        visited >= 86,
        "all native M1 seats and both source faces must be exercised"
    );
}

#[test]
fn abc_families_remain_separate_from_two_three_four_coordinate_completion() {
    for basis in MusicalBasis::ALL {
        for lens in LensId::ALL {
            for family in [RelationFamily::A, RelationFamily::B, RelationFamily::C] {
                for pair_index in 0..3 {
                    for (degree, side, count) in [
                        (ConjugationDegree::D1, None, 2),
                        (ConjugationDegree::D2, Some(ExpansionSide::Left), 3),
                        (ConjugationDegree::D2, Some(ExpansionSide::Right), 3),
                        (ConjugationDegree::D3, None, 4),
                    ] {
                        let d = MusicalDetermination::new(
                            identity(),
                            "#1-4.4",
                            MFace::Pratibimba,
                            basis,
                            lens,
                            ContextFrameId::Cf5,
                            RelationSelection {
                                family,
                                pair_index,
                                degree,
                                expansion_side: side,
                            },
                        )
                        .unwrap();
                        assert_eq!(d.completion().family, family);
                        assert_eq!(d.completion().coordinates.len(), count);
                        assert_eq!(d.completion().pitches.len(), count);
                        assert_eq!(
                            d.completion()
                                .coordinates
                                .iter()
                                .filter(|c| c.face == QlFace::Conjugate)
                                .count(),
                            count - 2
                        );
                    }
                }
            }
        }
    }
    let invalid = RelationSelection {
        degree: ConjugationDegree::D2,
        expansion_side: None,
        ..relation()
    };
    assert!(
        MusicalDetermination::new(
            identity(),
            "#1-4.4",
            MFace::Pratibimba,
            MusicalBasis::Chromatic,
            LensId::L0,
            ContextFrameId::Cf1,
            invalid
        )
        .is_err()
    );
}

#[test]
fn rational_trajectories_do_not_obey_tempered_pitch_class_closure() {
    let fourth = ExactRatio::new(4, 3).unwrap();
    let fifth = ExactRatio::new(3, 2).unwrap();
    assert_eq!(
        fourth.compose(fifth).unwrap(),
        ExactRatio::new(2, 1).unwrap()
    );
    let trajectory = rational_trajectory(MusicalBasis::Fifths, 12).unwrap();
    assert_eq!(trajectory.pitch_class, 0);
    assert_eq!(trajectory.ratio, ExactRatio::new(531441, 4096).unwrap());
    let seven_octaves = ExactRatio::new(2, 1).unwrap().pow(7).unwrap();
    assert_ne!(trajectory.ratio, seven_octaves);
    assert_eq!(
        trajectory.ratio.divide(seven_octaves).unwrap(),
        ExactRatio::new(531441, 524288).unwrap()
    );
    let cents = 1200.0
        * trajectory
            .ratio
            .divide(seven_octaves)
            .unwrap()
            .as_f64()
            .log2();
    assert!((cents - 23.460010384649).abs() < 1e-9);
    let down = rational_trajectory(MusicalBasis::Fifths, -12).unwrap();
    assert_eq!(down.ratio, trajectory.ratio.reciprocal());
    assert_eq!(
        rational_trajectory(MusicalBasis::Chromatic, 6)
            .unwrap()
            .pitch_class,
        0
    );
    assert_ne!(
        rational_trajectory(MusicalBasis::Chromatic, 6)
            .unwrap()
            .ratio,
        ExactRatio::new(2, 1).unwrap()
    );
    assert!(rational_trajectory(MusicalBasis::Fifths, i32::MIN).is_err());
    let big = ExactRatio::new(u64::MAX, 2).unwrap();
    assert_eq!(
        big.compose(big.reciprocal()).unwrap(),
        ExactRatio::new(1, 1).unwrap()
    );
    assert!(big.compose(big).is_err());
    assert!(
        serde_json::from_str::<ExactRatio>("{\"numerator\":\"1\",\"denominator\":\"0\"}").is_err()
    );
    assert_eq!(
        serde_json::from_str::<ExactRatio>("{\"numerator\":\"4\",\"denominator\":\"6\"}").unwrap(),
        ExactRatio::new(2, 3).unwrap()
    );
    let full_precision = ExactRatio::new(u64::MAX, 1).unwrap();
    let wire = serde_json::to_string(&full_precision).unwrap();
    assert_eq!(
        serde_json::from_str::<ExactRatio>(&wire).unwrap(),
        full_precision
    );
    assert!(
        serde_json::from_str::<ExactRatio>("{\"numerator\":\"04\",\"denominator\":\"6\"}").is_err()
    );
}

#[test]
fn actual_targets_retain_rational_tuning_and_require_explicit_temperament() {
    let d = selection(MusicalBasis::Chromatic, LensId::L0, ContextFrameId::Cf1);
    let just = vimarsha_targets(d.clone(), fundamental(), exact_policy()).unwrap();
    assert_eq!(
        just.diatonic.map(|p| p.exact_ratio.unwrap()),
        [
            (1, 1),
            (9, 8),
            (81, 64),
            (4, 3),
            (3, 2),
            (27, 16),
            (243, 128)
        ]
        .map(|(n, d)| ExactRatio::new(n, d).unwrap())
    );
    let tempered = vimarsha_targets(
        d.clone(),
        fundamental(),
        TuningPolicy::EqualTemperament12 {
            provenance: provenance(TuningStanding::External),
        },
    )
    .unwrap();
    assert!(
        tempered
            .audio_octet
            .iter()
            .all(|pitch| pitch.exact_ratio.is_none())
    );
    assert_ne!(just.audio_octet_hz(), tempered.audio_octet_hz());
    assert_eq!(just.determination, tempered.determination);
    assert!(
        vimarsha_targets(
            d.clone(),
            fundamental(),
            TuningPolicy::EqualTemperament12 {
                provenance: provenance(TuningStanding::SourceAuthored),
            }
        )
        .is_err()
    );
    let invalid = [ExactRatio::new(1, 1).unwrap(); 12];
    assert!(
        vimarsha_targets(
            d,
            fundamental(),
            TuningPolicy::ExactPitchRatios {
                ratios: invalid,
                provenance: provenance(TuningStanding::External),
            }
        )
        .is_err()
    );
    assert!(Fundamental::new(f64::NAN, provenance(TuningStanding::External)).is_err());
    assert!(Fundamental::new(f64::MAX, provenance(TuningStanding::External)).is_ok());
    assert!(
        vimarsha_targets(
            selection(MusicalBasis::Chromatic, LensId::L0, ContextFrameId::Cf1),
            Fundamental::new(f64::MAX, provenance(TuningStanding::External)).unwrap(),
            exact_policy()
        )
        .is_err()
    );
}

#[test]
fn producer_target_rejects_disconnected_stale_and_wrong_domain_consumers() {
    let d = MusicalDetermination::from_engine(
        &engine(MusicalBasis::Fifths, LensId::L2Prime, 4),
        MFace::Pratibimba,
        relation(),
    )
    .unwrap();
    let mut stamp = InputStamp {
        identity: EventIdentity {
            event_ref: "event:physical-music".into(),
            profile_generation: 7,
        },
        source_ref: ql_mef::m2_vimarsha::SOURCE.into(),
        contract_ref: ql_mef::m2_vimarsha::POLICY.into(),
    };
    let registry = ql_mef::m_tree::native_current_m_registry();
    let writer = registry.coordinate("M2-1", MFace::Pratibimba).unwrap();
    d.validate_consumer(&stamp, &writer, 1).unwrap();
    stamp.identity.profile_generation = 6;
    assert!(d.validate_consumer(&stamp, &writer, 1).is_err());
    stamp.identity.profile_generation = 7;
    stamp.identity.event_ref = "event:different".into();
    assert!(d.validate_consumer(&stamp, &writer, 1).is_err());
    stamp.identity.event_ref = "event:physical-music".into();
    assert!(
        d.validate_consumer(
            &stamp,
            &registry.coordinate("M1-3", MFace::Pratibimba).unwrap(),
            1
        )
        .is_err()
    );
    assert!(
        d.validate_consumer(
            &stamp,
            &registry.coordinate("#2-5-5", MFace::Pratibimba).unwrap(),
            1
        )
        .is_err()
    );
    assert!(
        d.validate_consumer(
            &stamp,
            &registry.coordinate("M2-1", MFace::Bimba).unwrap(),
            1
        )
        .is_err()
    );
    assert!(d.validate_consumer(&stamp, &writer, 0).is_err());
    let unbacked = ql_mef::MCoordinate::parse_source("#2-1", MFace::Pratibimba).unwrap();
    assert!(d.validate_consumer(&stamp, &unbacked, 1).is_err());
    stamp.contract_ref = "arbitrary:nonempty-contract".into();
    assert!(d.validate_consumer(&stamp, &writer, 1).is_err());
    stamp.contract_ref = ql_mef::m2_vimarsha::POLICY.into();
    stamp.source_ref = "arbitrary:nonempty-source".into();
    assert!(d.validate_consumer(&stamp, &writer, 1).is_err());
    stamp.source_ref = format!("{}\0", ql_mef::m2_vimarsha::SOURCE);
    assert!(d.validate_consumer(&stamp, &writer, 1).is_err());
    let mut invalid_provenance = provenance(TuningStanding::External);
    invalid_provenance.policy_ref.push('\0');
    assert!(Fundamental::new(256.0, invalid_provenance).is_err());
}

#[test]
fn source_native_ananda_phase_and_complete_spinor_return_survive_the_handoff() {
    let engine = engine(MusicalBasis::Fifths, LensId::L2Prime, 4);
    let snapshot = engine.snapshot().unwrap();
    // Inspect actual native producer output, including literal and arithmetic
    // owners, rather than deriving Ananda from the target pitch-class numbers.
    let cell = ql_mef::m1::cell(5, 3, 4, 1, 7).unwrap();
    assert_eq!(cell.raw_terms, [12, 13, 25, -1, 1]);
    assert_eq!(cell.dr_terms, [3, 4, 7, 9, 1]);
    assert_eq!(cell.decimal_terms, Some([2, 3, 5, 9, 1]));
    assert_eq!(
        (cell.raw, cell.digit_root, cell.decimal10),
        (None, None, None)
    );
    assert_eq!(
        ql_mef::m1::cell(5, 11, 11, 1, 7).unwrap().decimal_terms,
        None
    );
    assert_eq!(
        (
            cell.clock.position6,
            cell.clock.phase,
            cell.clock.hopf_fiber,
            cell.clock.degree720
        ),
        (1, 1, 1, 570)
    );
    assert_eq!(snapshot["clock"]["degree720"], 570);
    let d = MusicalDetermination::from_engine(&engine, MFace::Pratibimba, relation()).unwrap();
    assert_eq!(d.identity().degree720(), cell.clock.degree720 as u16);
    let start = ql_mef::m1::Clock::new(0, 0).unwrap().spinor();
    let half = ql_mef::m1::Clock::new(1, 0).unwrap().spinor();
    let returned = ql_mef::m1::Clock::new(2, 0).unwrap().spinor();
    assert_eq!(start, returned);
    assert!((start.w + half.w).abs() < 1e-6);
    assert!((start.x + half.x).abs() < 1e-6);
}
