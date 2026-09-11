//! Execute the #154 M1 → accepted M2 producer join without a parent-local law.
//! These are controlled callers, not desktop or live Epii acceptance.
use ql_mef::m1::traverse_json;
use ql_mef::m1_engine::{Basis, EngineConfig, HarmonicSelection, M1Engine};
use ql_mef::m2::Reading72;
use ql_mef::m2_engine::{EventIdentity, M2Request};
use ql_mef::{LensId, SublensRef};
use serde_json::{Value, json};

#[test]
fn parents_and_deep_transfer_the_same_m1_lens_ratio_generation_and_clock_to_m2() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-parent-consumers-v1.json"
    ))
    .unwrap();
    let mut config: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    config.selected_coordinate = "#1-2-1".into();
    let mut owner = M1Engine::new(config).unwrap();
    for basis in [Basis::Chromatic, Basis::Fifths] {
        for (slot, lens) in LensId::ALL.into_iter().enumerate() {
            owner
                .configure_harmonics(
                    owner.config().revision.parse().unwrap(),
                    HarmonicSelection {
                        family: 1,
                        row12: 3,
                        col12: 4,
                        flowering_substage: 3,
                        lens12: slot as u8,
                        context_frame: 7,
                        basis,
                    },
                )
                .unwrap();
            let shared = owner.snapshot().unwrap();
            let cfg = owner.config();
            let traversal: Value = serde_json::from_str(
                &traverse_json(
                    &json!({
                        "schema":"ql.m1.traversal/v1",
                        "source":{"position6":2,"phase":1},
                        "target":{"position6":3,"phase":1},
                        "pointer":{"source_ref":"fixture:walk/2-prime", "target_ref":"fixture:walk/3-prime",
                            "relation_ref":"fixture:walk/2-3", "relation_roles":["A","C"]},
                        "participation":"source-only", "family":cfg.family,
                        "row12":cfg.row12,"col12":cfg.col12,"cycle":cfg.cycle,
                        "tick12":cfg.tick12,"basis":cfg.basis,"lens12":cfg.lens12
                    })
                    .to_string(),
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(traversal["cell"], shared["cell"]);
            assert_eq!(traversal["lens"], shared["music"]["lens"]);
            let mut received = Vec::new();
            for caller in ["epi.cosmic.123", "epi.personal.450", "epi.deep.m1"] {
                let mut m2: M2Request = serde_json::from_str(include_str!(
                    "../../../fixtures/kernel/m2-condition-request-v1.json"
                ))
                .unwrap();
                let identity = owner.pole_identity().unwrap();
                let event = EventIdentity {
                    event_ref: identity.event_ref().into(),
                    profile_generation: identity.profile_generation(),
                };
                m2.stamp.identity = event.clone();
                m2.tick12 = identity.tick12();
                m2.degree720 = identity.degree720();
                let sublens = SublensRef::canonical(
                    lens,
                    shared["clock"]["position6"].as_u64().unwrap() as u8,
                )
                .unwrap();
                // Native LensId slots are interleaved; retained C MEF rows are
                // grouped. This existing bridge, not raw index copying, owns it.
                let mef = Reading72::from_sublens(sublens);
                assert_eq!(mef.mef_sublens().unwrap(), sublens);
                m2.mef_conditions = vec![mef.index()];
                m2.context_frames = vec![shared["music"]["context_frame"].as_str().unwrap().into()];
                // Do not turn the fixture's zero amplitudes into M1 observations.
                // The accepted joint Vimarsha producer owns the audio/nodal law.
                m2.m1_excitation = None;
                let vim = m2.vimarsha.as_mut().unwrap();
                vim.stamp.identity = event.clone();
                vim.stamp.contract_ref = ql_mef::m1_engine::CONTRACT.into();
                vim.stamp.source_ref = shared["selected_reading"]["source_ref"]
                    .as_str()
                    .unwrap()
                    .into();
                vim.lens = mef.axes()[0];
                vim.musical_mode = cfg.context_frame - 1;
                vim.harmonic_ratio = std::array::from_fn(|i| {
                    u16::try_from(traversal["ratio_evidence"]["ratio"][i].as_u64().unwrap())
                        .unwrap()
                });
                m2.condition.as_mut().unwrap().active_mef_condition = mef.index();
                let frame = m2.execute().unwrap();
                assert_eq!(frame.identity, event, "{caller}");
                let condition = frame.condition.as_ref().unwrap();
                assert_eq!(condition.identity, event);
                assert_eq!(condition.sublens_ref, sublens.to_string());
                assert!(condition.source_path.is_some());
                assert_eq!(
                    frame.vimarsha.as_ref().unwrap().input.harmonic_ratio,
                    [4, 3]
                );
                assert_eq!(frame.degree720, identity.degree720());
                assert_eq!(frame.tick12, identity.tick12());
                received.push(serde_json::to_value(frame).unwrap());
                let mut stale = m2.clone();
                stale
                    .vimarsha
                    .as_mut()
                    .unwrap()
                    .stamp
                    .identity
                    .profile_generation -= 1;
                assert!(stale.execute().is_err());
                // Both input stamps agree; it is the numeric transport's own
                // precision bound, not merely a mismatched stamp, that rejects it.
                let unsafe_generation = 9_007_199_254_740_993;
                m2.stamp.identity.profile_generation = unsafe_generation;
                m2.vimarsha
                    .as_mut()
                    .unwrap()
                    .stamp
                    .identity
                    .profile_generation = unsafe_generation;
                assert!(m2.execute().is_err());
            }
            assert_eq!(received[0], received[1]);
            assert_eq!(received[0], received[2]);
            assert_eq!(owner.snapshot().unwrap(), shared); // reception does not mutate/fork M1
        }
    }
}
