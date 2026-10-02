// V/#293: actual native M2 bus must reach the exported A determination exactly.
// Native source computation, formulas and standing remain at their real owners.
mod support {
    include!("support/retained_performance.rs");
}
use ql_mef::MFace;
use ql_mef::performance_audio::prepare_native_performance;

#[test]
fn v293_actual_audio_preparation_preserves_each_native_m2_frequency_across_the_json_boundary() {
    for face in [MFace::Bimba, MFace::Pratibimba] {
        let mut input = support::preparation();
        input.source_face = face;
        let prepared = prepare_native_performance(input).unwrap();
        prepared
            .validate_native_consumers(prepared.native_basis(), prepared.physical_body())
            .unwrap();
        let writer = &prepared.native_basis().m2["vimarsha"]["reading"]["audio_octet_hz"];
        let admitted = &prepared.determination()["audio_octet_hz"];
        assert_eq!(writer.as_array().unwrap().len(), 8);
        assert_eq!(admitted.as_array().unwrap().len(), 8);
        for slot in 0..8 {
            let expected = writer[slot].as_f64().unwrap();
            let actual = admitted[slot].as_f64().unwrap();
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "actual native M2 slot {slot} changed before A consumer admission for {face:?}"
            );
        }
        // Exercise the actual packet serialization read by the existing C++
        // native_performance_packet floor, rather than only a typed struct.
        let bytes = serde_json::to_vec(&prepared).unwrap();
        let packet: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for slot in 0..8 {
            assert_eq!(
                packet["determination"]["audio_octet_hz"][slot]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                packet["native_basis"]["m2"]["vimarsha"]["reading"]["audio_octet_hz"][slot]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                "actual exported packet detached slot {slot} from its M2 writer for {face:?}"
            );
        }
    }
}
