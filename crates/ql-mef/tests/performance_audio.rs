//! Actual native M1 -> joined M2/B -> K per-key targets -> P body preparation.
//! Metric material/tuning below have explicit Reference standing; they are not
//! authentic maqam pitches, a mocked engine or measured physical construction.
use ql_core::{ConjugationDegree, RelationFamily};
use ql_mef::MFace;
use ql_mef::continuous::coupled::{CoupledInput, HarmonicSource, REQUEST};
use ql_mef::m1_engine::EngineConfig;
use ql_mef::m2_engine::M2Request;
use ql_mef::m2_relation_plan::M2RelationPlanContext;
use ql_mef::m3_engine::{M3NodeKind, native_m3_engine};
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::music_determination::{
    ExactRatio, Fundamental, RelationSelection, TuningPolicy, TuningProvenance, TuningStanding,
};
use ql_mef::performance_audio::*;
use ql_mef::physical_body::*;
use serde_json::{Value, json};
fn tuning_provenance() -> TuningProvenance {
    TuningProvenance {
        policy_ref: "reference:explicit-rational-tuning".into(),
        source_ref: "reference:performer-score".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    }
}
fn physical_provenance(name: &str) -> PhysicalProvenance {
    PhysicalProvenance {
        reference: name.into(),
        revision: "1".into(),
        source_ref: "reference:analytic-bar".into(),
        standing: PhysicalStanding::Reference,
    }
}
fn preparation() -> PerformancePreparationInput {
    let m1_seed: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(m1_seed["config"].clone()).unwrap();
    let b: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-relation-plan-input-v1.json"
    ))
    .unwrap();
    let m2: M2Request = serde_json::from_value(b["nativeRequest"].clone()).unwrap();
    let context: M2RelationPlanContext =
        serde_json::from_value(b["relationContext"].clone()).unwrap();
    let m3_seed: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_seed["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m1.revision = "11".into();
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    let state = M3State::new(m3.clone()).unwrap();
    let form = native_m3_engine()
        .node(
            M3NodeKind::Codon,
            usize::from(state.fold().codon().address()),
        )
        .unwrap();
    let physical = BodyPreparationRequest {
        expected_m3_generation: state.generation(),
        body_revision: 1,
        preparation_ref: "reference:performance/body-preparation".into(),
        state_ref: "reference:performance/body-state".into(),
        geometry: MetricGeometry {
            provenance: physical_provenance("reference:metric-axial-bar"),
            family: BodyFamily::AxialTruss,
            nodes: vec![
                PhysicalNode {
                    identity: 1,
                    constituent: form.source_ref.clone(),
                    rest_metres: [0.0; 3],
                    additional_mass_kg: 0.0,
                    fixed: [true; 3],
                },
                PhysicalNode {
                    identity: 2,
                    constituent: "#3-0".into(),
                    rest_metres: [1.0, 0.0, 0.0],
                    additional_mass_kg: 0.0,
                    fixed: [false, true, true],
                },
            ],
            edges: vec![PhysicalEdge {
                first: 0,
                second: 1,
                section_m2: 1e-4,
                prestress_newtons: 0.0,
            }],
        },
        material: PhysicalMaterial {
            provenance: physical_provenance("reference:elastic-material"),
            young_modulus_pa: 1e6,
            density_kg_per_m3: 2.0,
            damping_alpha_per_second: 4.0,
            damping_beta_seconds: 0.0,
        },
        sample_rate: 48000,
        exciter: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: vec![0.0, 1.0],
        },
        pickup: SpatialProjection {
            axis: [1.0, 0.0, 0.0],
            node_weights: vec![0.0, 1.0],
        },
        pickup_linear_per_metre: 1000.0,
        max_force_newtons: 10.0,
        max_impulse_newton_seconds: 0.01,
        max_displacement_metres: 0.1,
    };
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
    PerformancePreparationInput {
        coupled: CoupledInput {
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
        },
        relation_context: context,
        source_face: MFace::Pratibimba,
        physical_face: MFace::Pratibimba,
        excitation: ExcitationPolicy {
            policy_ref: "proposal:D30/native-root-and-octet/v1".into(),
            standing: "agent-proposed-implementation-policy".into(),
            scaling: OctetScaling::NoteRelativeToReference,
            reference_hertz: 220.0,
            root_linear: 0.5,
            octet_linear: 0.5,
            weights: [0.125; 8],
        },
        relation: RelationSelection {
            family: RelationFamily::C,
            pair_index: 1,
            degree: ConjugationDegree::D3,
            expansion_side: None,
        },
        fundamental: Fundamental::new(220.0, tuning_provenance()).unwrap(),
        fundamental_scaling: FundamentalScaling::NativeM1HarmonicRatio,
        tuning: TuningPolicy::ExactPitchRatios {
            ratios: pairs.map(|(n, d)| ExactRatio::new(n, d).unwrap()),
            provenance: tuning_provenance(),
        },
        require_authentic_condition_tuning: false,
        physical,
        instance_ref: "expression:retained/current".into(),
        receipt_ref: "receipt:native/performance-admission".into(),
        touches: (0..12)
            .map(|key| KeyTouch {
                key,
                register: 0,
                member: u64::from(key) + 1,
                touch: u64::from(key) + 1,
                touch_ref: format!("touch:janko/{key}"),
            })
            .collect(),
    }
}
#[test]
fn actual_producers_join_all_twelve_keys_without_changing_body_spectrum_or_bus_roles() {
    let prepared = prepare_native_performance(preparation()).unwrap();
    assert_eq!(prepared.notes().len(), 12);
    assert_eq!(
        prepared.determination()["audio_octet_hz"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(
        prepared.determination()["nodal_quartet"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    let mut pitches = std::collections::BTreeSet::new();
    for note in prepared.notes() {
        assert!(pitches.insert(note["pitch_class"].as_u64().unwrap()));
        assert_eq!(
            note["source_coordinate"],
            prepared.determination()["m1_coordinate"]
        );
        assert_eq!(note["source_face"], 1);
        assert_eq!(note["identity"], prepared.determination()["identity"]);
        assert!(note["exact_ratio"].as_bool().unwrap());
        assert_eq!(
            note["position"].as_u64().unwrap(),
            note["key"].as_u64().unwrap() / 2
        );
    }
    assert_eq!(
        prepared.relation_plan().execution.tuning_standing,
        "unavailable-complete-source-intervals-absent;no-retained-fallback"
    );
    assert!(
        prepared
            .relation_plan()
            .execution
            .intended_tuning_hz
            .is_none()
    );
    assert_eq!(
        prepared.physical_body().request().geometry.edges[0].section_m2,
        1e-4
    );
    assert_eq!(
        prepared.physical_body().request().material.young_modulus_pa,
        1e6
    );
    prepared
        .validate_native_consumers(prepared.native_basis(), prepared.physical_body())
        .unwrap();
}
#[test]
fn musical_source_changes_affect_excitation_targets_while_physical_preparation_remains_independent()
{
    let a = prepare_native_performance(preparation()).unwrap();
    let mut different = preparation();
    different.coupled.harmonic_source = HarmonicSource::CanonicalBasis { index: 0 };
    let b = prepare_native_performance(different).unwrap();
    assert_ne!(a.notes()[0]["hertz"], b.notes()[0]["hertz"]);
    assert_eq!(a.physical_body().request(), b.physical_body().request());
    assert_ne!(
        a.determination()["audio_octet_hz"],
        b.determination()["audio_octet_hz"]
    );
    let mut lens = preparation();
    lens.coupled.m1.lens12 = 1;
    let c = prepare_native_performance(lens).unwrap();
    assert_ne!(a.notes()[0]["pitch_class"], c.notes()[0]["pitch_class"]);
    assert_eq!(a.physical_body().request(), c.physical_body().request());
}
#[test]
fn real_disconnected_and_stale_producer_outputs_fail_receiver_replay() {
    let prepared = prepare_native_performance(preparation()).unwrap();
    let mut wrong = prepared.native_basis().clone();
    wrong.m2["vimarsha"]["reading"]["audio_octet_hz"][0] = json!(440);
    assert!(
        prepared
            .validate_native_consumers(&wrong, prepared.physical_body())
            .is_err()
    );
    let mut wrong = prepared.native_basis().clone();
    wrong.input.m1.revision = "12".into();
    assert!(
        prepared
            .validate_native_consumers(&wrong, prepared.physical_body())
            .is_err()
    );
    let mut wrong = prepared.native_basis().clone();
    wrong.m2_input.stamp.identity.event_ref = "event:other".into();
    assert!(
        prepared
            .validate_native_consumers(&wrong, prepared.physical_body())
            .is_err()
    );
    let mut other = preparation();
    other.physical.state_ref = "body:independent-shader".into();
    let other = prepare_native_performance(other).unwrap();
    assert!(
        prepared
            .validate_native_consumers(prepared.native_basis(), other.physical_body())
            .is_err()
    );
}
#[test]
fn unavailable_authentic_condition_tuning_is_refused_and_never_relabelled_as_reference() {
    let mut authentic = preparation();
    authentic.require_authentic_condition_tuning = true;
    assert!(
        prepare_native_performance(authentic)
            .unwrap_err()
            .contains("source-unavailable")
    );
    let prepared = prepare_native_performance(preparation()).unwrap();
    assert!(
        prepared
            .relation_plan()
            .execution
            .intended_tuning_hz
            .is_none()
    );
    assert_eq!(
        prepared.targets().tuning_policy.provenance().standing,
        TuningStanding::Reference
    );
}
#[test]
fn repeated_touches_keep_native_note_identity_and_exact_registers() {
    let mut input = preparation();
    input.touches = vec![
        KeyTouch {
            key: 5,
            register: -1,
            member: 6,
            touch: 30,
            touch_ref: "touch:row/a".into(),
        },
        KeyTouch {
            key: 5,
            register: -1,
            member: 6,
            touch: 31,
            touch_ref: "touch:row/b".into(),
        },
        KeyTouch {
            key: 5,
            register: 1,
            member: 18,
            touch: 32,
            touch_ref: "touch:high/a".into(),
        },
    ];
    let prepared = prepare_native_performance(input).unwrap();
    assert_eq!(prepared.notes()[0]["hertz"], prepared.notes()[1]["hertz"]);
    assert_ne!(prepared.notes()[0]["touch"], prepared.notes()[1]["touch"]);
    assert_eq!(
        prepared.notes()[2]["hertz"].as_f64().unwrap(),
        prepared.notes()[0]["hertz"].as_f64().unwrap() * 4.0
    );
    let mut duplicate = preparation();
    duplicate.touches[1].touch = 1;
    assert!(prepare_native_performance(duplicate).is_err());
    let mut mismatch = preparation();
    mismatch.touches[1].member = 1;
    assert!(prepare_native_performance(mismatch).is_err());
    let mut unknown = preparation();
    unknown.touches[0].key = 12;
    assert!(prepare_native_performance(unknown).is_err());
}
#[test]
fn retuning_body_modes_cross_event_wrong_rast_and_stale_physical_sources_refuse() {
    let mut retune = preparation();
    retune
        .coupled
        .frequency_bindings
        .push(ql_mef::continuous::coupled::FrequencyBinding {
            mode_ref: "mode:legacy".into(),
            octet_index: 0,
        });
    assert!(
        prepare_native_performance(retune)
            .unwrap_err()
            .contains("cannot retune")
    );
    let mut event = preparation();
    event.coupled.m1.event_ref = "event:other".into();
    assert!(prepare_native_performance(event).is_err());
    let mut rast = preparation();
    rast.relation_context.requested_maqam_index = Some(3);
    assert!(prepare_native_performance(rast).is_err());
    let mut stale = preparation();
    stale.physical.expected_m3_generation += 1;
    assert!(prepare_native_performance(stale).is_err());
    let mut detached = preparation();
    detached.physical.geometry.nodes[0].constituent = "#3-0".into();
    assert!(prepare_native_performance(detached).is_err());
}
#[test]
fn actual_vimarsha_determinant_changes_octet_with_fixed_keys_metric_body_and_policy() {
    let mut input = preparation();
    input.fundamental_scaling = FundamentalScaling::AbsoluteReference;
    let baseline = prepare_native_performance(input).unwrap();
    let mut input = preparation();
    input.fundamental_scaling = FundamentalScaling::AbsoluteReference;
    input.coupled.harmonic_source = HarmonicSource::CanonicalBasis { index: 0 };
    let changed = prepare_native_performance(input).unwrap();
    assert_ne!(
        baseline.determination()["audio_octet_hz"],
        changed.determination()["audio_octet_hz"]
    );
    assert_eq!(
        baseline.determination()["nodal_quartet"],
        changed.determination()["nodal_quartet"]
    );
    assert_eq!(
        baseline.determination()["excitation"],
        changed.determination()["excitation"]
    );
    assert_eq!(
        serde_json::to_value(baseline.physical_body()).unwrap(),
        serde_json::to_value(changed.physical_body()).unwrap()
    );
    for (a, b) in baseline.notes().iter().zip(changed.notes()) {
        assert_eq!(a, b); // K root/reference/touch/source/phase all fixed.
    }
    // Explicit opt-in finite native producer fixture for the paired C++ trial.
    // The caller owns destination custody. This is test evidence, not a store.
    if let Some(destination) = std::env::var_os("QL_PERFORMANCE_PACKET_OUTPUT") {
        let destination = std::path::PathBuf::from(destination);
        assert!(
            destination.is_dir(),
            "caller must admit an existing output directory"
        );
        for (name, binding) in [("baseline", &baseline), ("changed", &changed)] {
            std::fs::write(
                destination.join(format!("{name}.packet.json")),
                serde_json::to_vec(binding).unwrap(),
            )
            .unwrap();
            std::fs::write(
                destination.join(format!("{name}.basis.json")),
                serde_json::to_vec(binding.native_basis()).unwrap(),
            )
            .unwrap();
        }
    }
}
#[test]
fn d30_excitation_policy_refuses_invalid_units_weights_and_unbounded_gains() {
    let mut input = preparation();
    input.excitation.reference_hertz = f64::NAN;
    assert!(prepare_native_performance(input).is_err());
    let mut input = preparation();
    input.excitation.octet_linear = 0.75;
    assert!(prepare_native_performance(input).is_err());
    let mut input = preparation();
    input.excitation.weights[0] = 0.5;
    assert!(prepare_native_performance(input).is_err());
    let mut input = preparation();
    input.excitation.policy_ref.clear();
    assert!(prepare_native_performance(input).is_err());
}
