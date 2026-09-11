use ql_core::{ConjugationDegree, QlCoordinate, QlFace, QlPosition};
use ql_mef::m1::*;
use ql_mef::{
    CANONICAL_RATIOS, LensId, MusicalBasis, cardinality_sum, field_cardinality, totality_ratio,
};
use serde_json::Value;
use std::{path::Path, process::Command};

#[test]
fn actual_native_c_matches_rust_and_retained_c_return() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m1-engine");
    std::fs::create_dir_all(&out).unwrap();
    let exe = out.join("probe");
    let build = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .current_dir(&root)
        .args([
            "-std=c11",
            "-O1",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-ffunction-sections",
            "-fdata-sections",
            "-Ic/include",
            "-Ivendor/epi-kernel/reference/include",
            "-Imigration/epi-kernel/m1-return",
            "migration/epi-kernel/m1-engine-probe.c",
            "migration/epi-kernel/m1-return/m1_ananda_projection.c",
            "vendor/epi-kernel/reference/src/m1.c",
            "c/src/m_tree.c",
            "c/src/m1.c",
            "-Wl,--gc-sections",
            "-lm",
            "-o",
        ])
        .arg(&exe)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let result = Command::new(exe).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    std::fs::write(out.join("c-rust.jsonl"), &result.stdout).unwrap();
    let (mut cells, mut descriptors) = (0, 0);
    for line in String::from_utf8(result.stdout).unwrap().lines() {
        let mut c: Value = serde_json::from_str(line).unwrap();
        let u = |name: &str| c[name].as_u64().unwrap() as u32;
        let rust = match c["kind"].as_str() {
            None => {
                cells += 1;
                let cycle = c["clock"]["cycle"].as_str().unwrap().parse().unwrap();
                let tick = c["clock"]["tick12"].as_u64().unwrap() as u32;
                serde_json::to_value(
                    cell(u("family"), u("row12"), u("col12"), cycle, tick).unwrap(),
                )
                .unwrap()
            }
            Some("spanda") => {
                descriptors += 1;
                serde_json::to_value(spanda(u("stage"), u("substage")).unwrap()).unwrap()
            }
            Some("formal") => {
                descriptors += 1;
                serde_json::to_value(formal(u("stage")).unwrap()).unwrap()
            }
            Some("topology") => {
                descriptors += 1;
                let t = topology(u("tick12")).unwrap();
                for (component, expected) in c["legacy_ring_quaternion"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .zip(t.legacy_ring_quaternion)
                {
                    assert_eq!(
                        (component.as_f64().unwrap() as f32).to_bits(),
                        expected.to_bits()
                    );
                }
                c.as_object_mut().unwrap().remove("legacy_ring_quaternion");
                let mut value = serde_json::to_value(t).unwrap();
                value
                    .as_object_mut()
                    .unwrap()
                    .remove("legacy_ring_quaternion");
                value
            }
            other => panic!("unknown C observation {other:?}"),
        };
        c.as_object_mut().unwrap().remove("kind");
        assert_eq!(c, rust, "native observation: {line}");
    }
    assert_eq!(cells, 20736);
    assert_eq!(descriptors, 29);
    let checked = Command::new("python3")
        .current_dir(&root)
        .args(["scripts/check-m1-source.py"])
        .output()
        .unwrap();
    assert!(
        checked.status.success(),
        "{}",
        String::from_utf8_lossy(&checked.stderr)
    );
}

#[test]
fn boundaries_are_checked_not_truncated_or_silently_wrapped() {
    for f in [6, 256, u32::MAX] {
        assert!(cell(f, 0, 0, 0, 0).is_err());
    }
    for n in [12, 256, u32::MAX] {
        assert!(cell(0, n, 0, 0, 0).is_err());
        assert!(cell(0, 0, n, 0, 0).is_err());
        assert!(Clock::new(0, n).is_err());
        assert!(topology(n).is_err());
    }
    assert!(node("#2").is_none());
    assert!(node("#1-4-0").is_none());
    assert_eq!(node("M1-4.0").unwrap().id, node("#1-4.0").unwrap().id);
    assert!(spanda(1, 1).is_err());
    assert!(spanda(4, 6).is_err());
    assert!(formal(6).is_err());
    assert!(torus(f64::NAN, 0.0).is_err());
    assert!(torus(0.0, f64::INFINITY).is_err());
}

#[test]
fn three_registers_and_quintessence_contributors_remain_distinct() {
    let a = cell(0, 3, 4, 0, 0).unwrap();
    assert_eq!(
        (a.raw, a.digit_root, a.decimal10),
        (Some(12), Some(3), Some(2))
    );
    let a = cell(5, 3, 4, 0, 0).unwrap();
    assert!(!a.scalar_valid);
    assert_eq!((a.raw, a.digit_root, a.decimal10), (None, None, None));
    assert_eq!(a.raw_terms, [12, 13, 25, -1, 1]);
    assert_eq!(a.dr_terms, [3, 4, 7, 9, 1]);
    assert_eq!(a.decimal_terms, Some([2, 3, 5, 9, 1]));
    assert_eq!(cell(5, 11, 11, 0, 0).unwrap().decimal_terms, None);
}

#[test]
fn phase_fiber_spinor_and_legacy_ring_are_separate() {
    for tick in 0..12 {
        let a = Clock::new(0, tick).unwrap();
        let b = Clock::new(1, tick).unwrap();
        let c = Clock::new(2, tick).unwrap();
        assert_eq!(a.phase, b.phase);
        assert_ne!(a.hopf_fiber, b.hopf_fiber);
        assert_eq!(a.spinor(), c.spinor());
        assert!((a.spinor().w + b.spinor().w).abs() < 1e-6);
        assert!((a.spinor().x + b.spinor().x).abs() < 1e-6);
        assert_eq!(
            a.position6,
            Clock::new(0, a.conjugate_tick12).unwrap().position6
        );
    }
    assert_eq!(Clock::new(u64::MAX, 11).unwrap().degree720, 690);
    assert_eq!(
        serde_json::to_value(Clock::new(u64::MAX, 0).unwrap()).unwrap()["cycle"],
        u64::MAX.to_string()
    );
    assert_eq!(Clock::new(0, 6).unwrap().position6, 0);
    assert_eq!(topology(6).unwrap().legacy_return_stage, 5);
    assert_ne!(
        Clock::new(0, 6).unwrap().spinor().w,
        topology(6).unwrap().legacy_ring_quaternion[0]
    );
}

#[test]
fn retained_ratios_join_current_matheme_without_a_new_music_grammar() {
    for (e, r) in ratio_basis().unwrap().iter().zip(CANONICAL_RATIOS) {
        assert_eq!(e.ratio, [r.numerator(), r.denominator()]);
    }
    assert_eq!(totality_ratio(), CANONICAL_RATIOS[5]);
    assert_eq!(field_cardinality(), 72);
    assert_eq!(cardinality_sum(), 137);
    assert!(source_ratio(0, 3).unwrap().is_none());
    assert!(source_ratio(5, 9).unwrap().is_none());
    assert!(matches!(
        ratio_basis().unwrap()[7].derivation,
        RatioDerivation::Composition { .. }
    ));
    let mut big = source_ratio(0, 9).unwrap().unwrap();
    big.ratio = [u32::MAX, 1];
    assert!(big.clone().compose(big).is_err());
    let mut a = source_ratio(0, 9).unwrap().unwrap();
    a.ratio = [u32::MAX, 2];
    assert_eq!(a.clone().compose(a.reciprocal()).unwrap().ratio, [1, 1]);
}
fn q(position: u8, face: QlFace) -> QlCoordinate {
    QlCoordinate::new(QlPosition::new(position).unwrap(), face)
}
fn request(source: QlCoordinate, target: QlCoordinate) -> TraversalRequest {
    TraversalRequest {
        source,
        target,
        pointer: PointerEvidence {
            source_ref: "S2:source".into(),
            target_ref: "S2:target".into(),
            relation_ref: "pointer:observed".into(),
            relation_roles: vec!["retained-role".into()],
        },
        family: 1,
        row12: 7,
        col12: 1,
        cycle: 1,
        tick12: 6,
        participation: ConjugateParticipation::None,
        basis: MusicalBasis::Chromatic,
        lens: LensId::L0,
    }
}
#[test]
fn every_actual_walk_retains_all_families_direction_and_completion() {
    let mut observations = 0;
    for face in [QlFace::Direct, QlFace::Conjugate] {
        for from in 0..6 {
            for to in 0..6 {
                for basis in MusicalBasis::ALL {
                    for lens in LensId::ALL {
                        for participation in [
                            ConjugateParticipation::None,
                            ConjugateParticipation::SourceOnly,
                            ConjugateParticipation::TargetOnly,
                            ConjugateParticipation::Both,
                        ] {
                            let mut r = request(q(from, face), q(to, face));
                            r.basis = basis;
                            r.lens = lens;
                            r.participation = participation;
                            let event = traverse(r.clone()).unwrap();
                            observations += 1;
                            let mut changed = r;
                            changed.family = 5;
                            changed.row12 = 11;
                            changed.col12 = 11;
                            assert_eq!(
                                event.candidates,
                                traverse(changed).unwrap().candidates,
                                "cell evidence must never select the walked relation"
                            );
                            for candidate in event.candidates {
                                let size = match participation {
                                    ConjugateParticipation::None => 2,
                                    ConjugateParticipation::Both => 4,
                                    _ => 3,
                                };
                                assert_eq!(candidate.frame.coordinates.len(), size);
                                assert_eq!(candidate.frame.pitches.len(), size);
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(observations, 6912);
    let event = traverse(request(q(2, QlFace::Direct), q(3, QlFace::Direct))).unwrap();
    assert_eq!(event.candidates.len(), 2);
    assert_eq!(event.ratio.unwrap().ratio, [16, 9]);
    assert!(
        traverse(request(q(2, QlFace::Direct), q(3, QlFace::Conjugate)))
            .unwrap()
            .candidates
            .is_empty()
    );
    let mut bad = request(q(0, QlFace::Direct), q(1, QlFace::Direct));
    bad.pointer.relation_ref = " ".into();
    assert!(traverse(bad).is_err());
}
#[test]
fn reversed_d2_keeps_expansion_on_the_observed_endpoint() {
    let mut a = request(q(2, QlFace::Direct), q(3, QlFace::Direct));
    a.participation = ConjugateParticipation::SourceOnly;
    let mut b = request(q(3, QlFace::Direct), q(2, QlFace::Direct));
    b.participation = ConjugateParticipation::TargetOnly;
    let a = traverse(a).unwrap();
    let b = traverse(b).unwrap();
    for (a, b) in a.candidates.iter().zip(&b.candidates) {
        assert_eq!(a.frame.degree, ConjugationDegree::D2);
        assert_eq!(a.frame.expansion_side, b.frame.expansion_side);
        assert_eq!(a.frame.coordinates, b.frame.coordinates);
    }
}

#[test]
fn json_boundary_keeps_full_precision_and_rejects_malformed_requests() {
    let request = serde_json::json!({"schema":"ql.m1.traversal/v1","source":{"position6":2,"phase":0},
        "target":{"position6":3,"phase":0},"pointer":{"source_ref":"S2:2","target_ref":"S2:3","relation_ref":"walk:2-3","relation_roles":["A","C"]},
        "family":5,"row12":3,"col12":4,"cycle":u64::MAX.to_string(),"tick12":7,
        "participation":"both","basis":"fifths","lens12":11});
    let output: Value =
        serde_json::from_str(&traverse_json(&request.to_string()).unwrap()).unwrap();
    assert_eq!(output["cell"]["clock"]["cycle"], u64::MAX.to_string());
    assert_eq!(output["cell"]["raw"], Value::Null);
    assert_eq!(
        output["cell"]["decimal_terms"],
        serde_json::json!([2, 3, 5, 9, 1])
    );
    assert_eq!(output["pointer"], request["pointer"]);
    assert_eq!(output["pointer_evidence_status"], "caller-supplied");
    assert_eq!(output["candidates"].as_array().unwrap().len(), 2);
    assert_eq!(
        output["candidates"][0]["coordinates"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    for (key, value) in [
        ("cycle", serde_json::json!(18446744073709551615u64)),
        ("cycle", serde_json::json!("18446744073709551616")),
        ("cycle", serde_json::json!("+1")),
        ("tick12", serde_json::json!(12)),
        ("family", serde_json::json!(256)),
        ("lens12", serde_json::json!(12)),
        ("schema", serde_json::json!("other")),
        ("unknown", serde_json::json!(true)),
    ] {
        let mut invalid = request.clone();
        invalid[key] = value;
        assert!(traverse_json(&invalid.to_string()).is_err(), "{invalid}");
    }
    let mut invalid = request;
    invalid["source"]["position6"] = serde_json::json!(6);
    assert!(traverse_json(&invalid.to_string()).is_err());
}
