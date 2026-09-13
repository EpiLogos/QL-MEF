//! Whole native M1/M2/M3 derivation. The installed native continuation and GPU
//! receive this same producer in kernel-k8-continuous, not a fake worker here.
use ql_mef::continuous::coupled::{
    ConditionFrequencyBinding, CoupledInput, FrequencyBinding, HarmonicSource, REQUEST, REQUEST_V2,
};
use ql_mef::m2_condition::{CorrespondenceRole, TuningPolicy, correspondence_field, condition_pitches};
use ql_mef::m1_engine::{EngineConfig, M1Engine};
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::{LensId, SublensRef};
use serde_json::{Value, json};

fn input() -> CoupledInput {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let mut m1: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    let m2: M2Request = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m2-condition-request-v1.json"
    ))
    .unwrap();
    let m3_fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-v1.json"
    ))
    .unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp
        .identity
        .event_ref
        .clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    CoupledInput {
        schema: REQUEST.into(),
        m1,
        m2,
        m3,
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![],
        condition_frequency_bindings: vec![],
        source_receipts: vec![json!({"standing":"controlled fixture, not a live provider"})],
    }
}

#[test]
fn full_owners_survive_the_actual_join_across_all_lenses_and_context_modes() {
    for (slot, lens) in LensId::ALL.into_iter().enumerate() {
        for cf in 1..=7 {
            let mut request = input();
            request.m1.lens12 = slot as u8;
            request.m1.context_frame = cf;
            let m1 = M1Engine::new(request.m1.clone())
                .unwrap()
                .snapshot()
                .unwrap();
            let m3 = M3State::new(request.m3.clone()).unwrap().snapshot();
            let original = serde_json::to_value(&request).unwrap();
            let result = request.compose().unwrap();
            assert_eq!(result.m1, m1);
            assert_eq!(result.m3, m3);
            assert_eq!(serde_json::to_value(&result.input).unwrap(), original);
            let sublens = SublensRef::canonical(lens, (request.m1.tick12 % 6) as u8).unwrap();
            assert_eq!(result.derivation["mef_sublens"], sublens.to_string());
            assert_eq!(result.m2["condition"]["sublens_ref"], sublens.to_string());
            assert_eq!(
                result.m2_input.vimarsha.as_ref().unwrap().musical_mode,
                cf - 1
            );
            assert_eq!(
                result.m2_input.vimarsha.as_ref().unwrap().pose_ordinal as u64,
                result.m3["form"]["pose_ordinal"].as_u64().unwrap()
            );
            assert_eq!(
                result.m2["modal"]["coefficients"],
                serde_json::to_value(&request.m2.modal_coefficients).unwrap()
            );
            assert_eq!(
                result.m2_input.world_observations.len(),
                request.m2.world_observations.len()
            );
            assert_eq!(
                result.m2["domains"].as_array().unwrap().len(),
                request.m2.execute().unwrap().domains.len()
            );
            assert_eq!(
                serde_json::to_value(request.compose().unwrap()).unwrap(),
                serde_json::to_value(result).unwrap()
            );
        }
    }
}

#[test]
fn every_lawful_pose_and_all_native_harmonic_bases_remain_operative() {
    let mut count = 0;
    for pose in ql_core::all_poses() {
        let mut request = input();
        request.m3.address = pose.codon().address();
        request.m3.pose = pose.slot();
        request.harmonic_source = HarmonicSource::CanonicalBasis {
            index: (count % 8) as u8,
        };
        let result = request.compose().unwrap();
        assert_eq!(
            result.m2["vimarsha"]["reading"]["seed"]["codon"],
            pose.codon().address()
        );
        assert_eq!(
            result.m2["vimarsha"]["reading"]["seed"]["rotation"],
            pose.slot()
        );
        assert_eq!(result.m3["form"]["lawful_field"], 472);
        assert!(result.m3["transcription"]["source"].is_object());
        assert!(result.m3["source_matrix_cells"].is_array());
        count += 1;
    }
    assert_eq!(count, 472);
}

#[test]
fn actual_m1_ratio_and_m3_pose_change_the_returned_native_frequency() {
    let mut request = input();
    request.m1.family = 1;
    request.m1.row12 = 3;
    request.harmonic_source = HarmonicSource::SelectedSourceRow;
    let before = request.compose().unwrap();
    assert_eq!(before.derivation["harmonic_ratio"]["ratio"], json!([4, 3]));
    request.m1.row12 = 7;
    let ratio_changed = request.compose().unwrap();
    assert_ne!(
        before.m2["vimarsha"]["reading"]["audio_octet_hz"],
        ratio_changed.m2["vimarsha"]["reading"]["audio_octet_hz"]
    );
    request.m3.pose = 0;
    let pose_changed = request.compose().unwrap();
    assert_ne!(
        ratio_changed.m2["vimarsha"]["reading"]["audio_octet_hz"],
        pose_changed.m2["vimarsha"]["reading"]["audio_octet_hz"]
    );
    assert_ne!(
        ratio_changed.m2["vimarsha"]["reading"]["nodal_quartet"],
        pose_changed.m2["vimarsha"]["reading"]["nodal_quartet"]
    );
    request.m1.row12 = 2;
    assert!(
        request
            .compose()
            .unwrap_err()
            .contains("no admitted harmonic ratio")
    );
}

#[test]
fn no_cross_event_unknown_mode_invalid_pose_or_silent_binding_fallback() {
    let mut request = input();
    request.m1.event_ref = "another-event".into();
    assert!(request.compose().is_err());
    request = input();
    request.m3.pose = 8;
    assert!(request.compose().is_err());
    request = input();
    request.harmonic_source = HarmonicSource::CanonicalBasis { index: 8 };
    assert!(request.compose().is_err());
    request = input();
    request.frequency_bindings.push(FrequencyBinding {
        mode_ref: "unknown".into(),
        octet_index: 0,
    });
    assert!(request.compose().is_err());
    let mut raw = serde_json::to_value(input()).unwrap();
    raw["hidden_policy"] = json!(true);
    assert!(serde_json::from_value::<CoupledInput>(raw).is_err());
}


/// Supplied material modes keep their own spatial and physical provenance.
/// One Vimarsha voice, eight correspondence voices and one unbound mode share
/// one resonator. This is not a measured acoustic eigensystem.
fn musical_input() -> CoupledInput {
    let mut request = input();
    request.schema = REQUEST_V2.into();
    let modes: Vec<Value> = (0..10)
        .map(|i| json!({
            "mode_ref":format!("controlled:musical-mode/{i}"),
            "source_coordinate":"#2-2-2-5-5", "material_fibre":"earth",
            "carrier_weights":[{"carrier":i,"weight":1.0}],
            "frequency_hz":137.0, "amplitude":[0.002,0.001], "excitation":[0.001,0.0],
            "damping_per_second":0.25, "nodal_state_ref":"controlled:nodes",
            "antinodal_state_ref":"controlled:antinodes"
        }))
        .collect();
    request.m2.resonator = Some(serde_json::from_value(json!({
        "stamp":request.m2.stamp, "provider_ref":"controlled:musical-projection",
        "geometry_ref":"controlled:modal-basis", "material_ref":"controlled:linear-medium",
        "material_model_ref":"ql.continuous-linear-mode/v1", "material_parameters":{},
        "modes":modes
    })).unwrap());
    request.frequency_bindings = vec![FrequencyBinding {
        mode_ref: "controlled:musical-mode/0".into(), octet_index: 3,
    }];
    request.condition_frequency_bindings = (0..8)
        .map(|i| ConditionFrequencyBinding {
            mode_ref: format!("controlled:musical-mode/{}", i + 1), pitch_index: i,
        })
        .collect();
    request
}

#[test]
fn v2_binds_both_tunings_over_every_present_source_path_without_replacing_vimarsha() {
    let field = correspondence_field();
    assert_eq!(field.rules.len(), 127, "source coverage changed; review, do not sample");
    for rule in &field.rules {
        for tuning in [TuningPolicy::Retained24Tet, TuningPolicy::BimbaSpelled24Tet] {
            let mut request = musical_input();
            let condition = request.m2.condition.as_mut().unwrap();
            condition.maqam_index = rule.maqam_index;
            condition.role = rule.role;
            condition.tuning = tuning;
            condition.tonic_hz = 220.0;
            let expected = condition_pitches(rule.maqam_index, rule.role, tuning, 220.0).unwrap();
            let Some(expected) = expected else {
                assert!(request.compose().unwrap_err().contains("condition pitch unavailable"));
                continue;
            };
            let basis = request.compose().unwrap();
            let modes = &basis.m2_input.resonator.as_ref().unwrap().modes;
            for (i, frequency) in expected.iter().enumerate() {
                assert_eq!(modes[i + 1].frequency_hz, *frequency);
                assert_eq!(basis.m2["condition"]["musical"]["pitches_hz"][i], *frequency);
            }
            assert_eq!(modes[0].frequency_hz,
                basis.m2["vimarsha"]["reading"]["audio_octet_hz"][3].as_f64().unwrap());
            assert_eq!(modes[9].frequency_hz, 137.0, "unbound mode was rewritten");
            assert_eq!(basis.m2["condition"]["source_path"], serde_json::to_value(rule).unwrap());
            assert_eq!(basis.m2["condition"]["source_revision"], field.source_revision);
            assert_eq!(basis.m2["condition"]["musical"]["tuning"], serde_json::to_value(tuning).unwrap());
            assert_eq!(basis.m2_input.world_observations.len(), request.m2.world_observations.len());
            assert_eq!(basis.m2_input.modal_coefficients, request.m2.modal_coefficients);
            assert_eq!(serde_json::to_value(&basis.input).unwrap(), serde_json::to_value(&request).unwrap());
            assert_eq!(basis.derivation["condition_frequency_bindings"],
                serde_json::to_value(&request.condition_frequency_bindings).unwrap());
            for (original, current) in request.m2.resonator.as_ref().unwrap().modes.iter().zip(modes) {
                let mut unchanged = serde_json::to_value(original).unwrap();
                unchanged["frequency_hz"] = json!(current.frequency_hz);
                assert_eq!(unchanged, serde_json::to_value(current).unwrap());
            }
        }
    }
}

#[test]
fn every_absent_path_is_refused_even_when_retained_tuning_could_supply_a_tone() {
    let mut absent = 0;
    for index in 0..72 {
        for role in [CorrespondenceRole::Tonic, CorrespondenceRole::Dominant] {
            if correspondence_field().rule(index, role).is_some() { continue; }
            absent += 1;
            for tuning in [TuningPolicy::Retained24Tet, TuningPolicy::BimbaSpelled24Tet] {
                let mut request = musical_input();
                let condition = request.m2.condition.as_mut().unwrap();
                condition.maqam_index = index;
                condition.role = role;
                condition.tuning = tuning;
                assert!(request.compose().unwrap_err().contains("no admitted correspondence source path"));
            }
        }
    }
    assert_eq!(absent, 17);
}

#[test]
fn changed_correspondence_tonic_changes_only_its_explicit_bus() {
    let mut request = musical_input();
    let before = request.compose().unwrap();
    request.m2.condition.as_mut().unwrap().tonic_hz *= 1.25;
    let changed = request.compose().unwrap();
    assert_eq!(before.m1, changed.m1);
    assert_eq!(before.m3, changed.m3);
    assert_eq!(before.m2["vimarsha"], changed.m2["vimarsha"]);
    let old = &before.m2_input.resonator.as_ref().unwrap().modes;
    let new = &changed.m2_input.resonator.as_ref().unwrap().modes;
    assert_eq!(old[0].frequency_hz, new[0].frequency_hz);
    assert_eq!(old[9].frequency_hz, new[9].frequency_hz);
    for i in 1..9 {
        assert!((new[i].frequency_hz / old[i].frequency_hz - 1.25).abs() < 1e-12);
    }
}

#[test]
fn dual_bus_validation_has_no_aliases_fallback_or_legacy_reinterpretation() {
    let original = musical_input();
    for case in 0..8 {
        let mut request = original.clone();
        match case {
            0 => request.schema = REQUEST.into(),
            1 => request.condition_frequency_bindings[0].mode_ref = "unknown".into(),
            2 => request.condition_frequency_bindings[0].pitch_index = 8,
            3 => request.condition_frequency_bindings[0].mode_ref = request.frequency_bindings[0].mode_ref.clone(),
            4 => request.condition_frequency_bindings.push(request.condition_frequency_bindings[0].clone()),
            5 => request.m2.condition = None,
            6 => request.m2.resonator = None,
            7 => request.schema = "ql.coupled-event-request/v3".into(),
            _ => unreachable!(),
        }
        assert!(request.compose().is_err(), "case {case} was accepted");
    }
    let mut raw = serde_json::to_value(&original).unwrap();
    raw["condition_frequency_bindings"][0]["fallback_hz"] = json!(440.0);
    assert!(serde_json::from_value::<CoupledInput>(raw).is_err());
    let mut legacy = original;
    legacy.schema = REQUEST.into();
    legacy.condition_frequency_bindings.clear();
    let encoded = serde_json::to_value(&legacy).unwrap();
    assert!(encoded.get("condition_frequency_bindings").is_none());
    let decoded: CoupledInput = serde_json::from_value(encoded.clone()).unwrap();
    let basis = decoded.compose().unwrap();
    assert_eq!(serde_json::to_value(&basis.input).unwrap(), encoded);
    assert!(basis.derivation.get("condition_frequency_bindings").is_none());
    assert!(basis.derivation.get("musical_sources").is_none());
    assert_eq!(basis.m2_input.resonator.as_ref().unwrap().modes[1].frequency_hz, 137.0);
    assert_eq!(serde_json::to_value(&basis).unwrap(), serde_json::to_value(legacy.compose().unwrap()).unwrap());
}
