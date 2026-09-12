//! Whole native M1/M2/M3 derivation. The installed native continuation and GPU
//! receive this same producer in kernel-k8-continuous, not a fake worker here.
use ql_mef::continuous::coupled::{CoupledInput, FrequencyBinding, HarmonicSource, REQUEST};
use ql_mef::m1_engine::{EngineConfig, M1Engine};
use ql_mef::m2_engine::M2Request;
use ql_mef::m3_state::{M3Request, M3State};
use ql_mef::{LensId, SublensRef};
use serde_json::{Value, json};

fn input() -> CoupledInput {
    let fixture: Value = serde_json::from_str(include_str!("../../../fixtures/kernel/m1-engine-v1.request.json")).unwrap();
    let mut m1: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    let m2: M2Request = serde_json::from_str(include_str!("../../../fixtures/kernel/m2-condition-request-v1.json")).unwrap();
    let m3_fixture: Value = serde_json::from_str(include_str!("../../../fixtures/kernel/m3-parent-consumer-v1.json")).unwrap();
    let mut m3: M3Request = serde_json::from_value(m3_fixture["request"].clone()).unwrap();
    m1.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.stamp.identity.event_ref.clone_from(&m2.stamp.identity.event_ref);
    m3.m2_basis.as_mut().unwrap().identity = m3.stamp.identity.clone();
    CoupledInput {
        schema: REQUEST.into(), m1, m2, m3, m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis { index: 3 },
        frequency_bindings: vec![], source_receipts: vec![json!({"standing":"controlled fixture, not a live provider"})],
    }
}

#[test]
fn full_owners_survive_the_actual_join_across_all_lenses_and_context_modes() {
    for (slot, lens) in LensId::ALL.into_iter().enumerate() {
        for cf in 1..=7 {
            let mut request = input();
            request.m1.lens12 = slot as u8;
            request.m1.context_frame = cf;
            let m1 = M1Engine::new(request.m1.clone()).unwrap().snapshot().unwrap();
            let m3 = M3State::new(request.m3.clone()).unwrap().snapshot();
            let original = serde_json::to_value(&request).unwrap();
            let result = request.compose().unwrap();
            assert_eq!(result.m1, m1);
            assert_eq!(result.m3, m3);
            assert_eq!(serde_json::to_value(&result.input).unwrap(), original);
            let sublens = SublensRef::canonical(lens, (request.m1.tick12 % 6) as u8).unwrap();
            assert_eq!(result.derivation["mef_sublens"], sublens.to_string());
            assert_eq!(result.m2["condition"]["sublens_ref"], sublens.to_string());
            assert_eq!(result.m2_input.vimarsha.as_ref().unwrap().musical_mode, cf - 1);
            assert_eq!(result.m2_input.vimarsha.as_ref().unwrap().pose_ordinal as u64, result.m3["form"]["pose_ordinal"].as_u64().unwrap());
            assert_eq!(result.m2["modal"]["coefficients"], serde_json::to_value(&request.m2.modal_coefficients).unwrap());
            assert_eq!(result.m2_input.world_observations.len(), request.m2.world_observations.len());
            assert_eq!(result.m2["domains"].as_array().unwrap().len(), request.m2.execute().unwrap().domains.len());
            assert_eq!(serde_json::to_value(request.compose().unwrap()).unwrap(), serde_json::to_value(result).unwrap());
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
        request.harmonic_source = HarmonicSource::CanonicalBasis { index: (count % 8) as u8 };
        let result = request.compose().unwrap();
        assert_eq!(result.m2["vimarsha"]["reading"]["seed"]["codon"], pose.codon().address());
        assert_eq!(result.m2["vimarsha"]["reading"]["seed"]["rotation"], pose.slot());
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
    assert_ne!(before.m2["vimarsha"]["reading"]["audio_octet_hz"], ratio_changed.m2["vimarsha"]["reading"]["audio_octet_hz"]);
    request.m3.pose = 0;
    let pose_changed = request.compose().unwrap();
    assert_ne!(ratio_changed.m2["vimarsha"]["reading"]["audio_octet_hz"], pose_changed.m2["vimarsha"]["reading"]["audio_octet_hz"]);
    assert_ne!(ratio_changed.m2["vimarsha"]["reading"]["nodal_quartet"], pose_changed.m2["vimarsha"]["reading"]["nodal_quartet"]);
    request.m1.row12 = 2;
    assert!(request.compose().unwrap_err().contains("no admitted harmonic ratio"));
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
    request.frequency_bindings.push(FrequencyBinding { mode_ref: "unknown".into(), octet_index: 0 });
    assert!(request.compose().is_err());
    let mut raw = serde_json::to_value(input()).unwrap();
    raw["hidden_policy"] = json!(true);
    assert!(serde_json::from_value::<CoupledInput>(raw).is_err());
}
