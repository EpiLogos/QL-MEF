use ql_mef::m_tree::{MTreeId, native_m_registry};
use ql_mef::m1::{self, Clock};
use ql_mef::m1_engine::*;
use serde_json::{Value, json};
use std::{collections::BTreeSet, path::Path, process::Command};

fn config() -> EngineConfig {
    EngineConfig {
        event_ref: "test:event:M1".into(),
        subject_coordinate: "#1".into(),
        selected_coordinate: "#1".into(),
        revision: "0".into(),
        cycle: "0".into(),
        tick12: 0,
        family: 0,
        row12: 3,
        col12: 4,
        flowering_substage: 0,
        lens12: 0,
        context_frame: 1,
        basis: Basis::Chromatic,
    }
}
fn numeric_equal(a: &Value, b: &Value, tolerance: f64) {
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (x, y) in a.iter().zip(b) {
                numeric_equal(x, y, tolerance);
            }
        }
        (Value::Number(a), Value::Number(b)) => {
            let a = a.as_f64().unwrap();
            let b = b.as_f64().unwrap();
            assert!((a - b).abs() <= tolerance, "{a} != {b}");
        }
        _ => assert_eq!(a, b),
    }
}
#[test]
fn actual_c_source_relation_and_state_match_independent_rust() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m1-state");
    std::fs::create_dir_all(&out).unwrap();
    let exe = out.join("probe");
    let result = Command::new(std::env::var("CC").unwrap_or("cc".into()))
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
            "vendor/epi-kernel/reference/src/m1.c",
            "vendor/epi-kernel/reference/src/psychoid_numbers.c",
            "migration/epi-kernel/m1-state-probe.c",
            "c/src/m_tree.c",
            "c/src/m1.c",
            "c/src/m1_state.c",
            "c/src/kernel.c",
            "c/src/primitive.c",
            "-Wl,--gc-sections",
            "-lm",
            "-o",
        ])
        .arg(&exe)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let run = Command::new(&exe).output().unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    std::fs::write(out.join("c-rust.jsonl"), &run.stdout).unwrap();
    let registry = native_m_registry();
    let mut counts = [0; 11];
    let mut seen = BTreeSet::new();
    let mut seen_nodes = BTreeSet::new();
    for line in String::from_utf8(run.stdout).unwrap().lines() {
        let mut c: Value = serde_json::from_str(line).unwrap();
        let kind = c["kind"].as_str().unwrap().to_owned();
        c.as_object_mut().unwrap().remove("kind");
        let u = |k: &str| c[k].as_u64().unwrap() as u32;
        match kind.as_str() {
            "source" => {
                counts[0] += 1;
                assert_eq!(
                    c,
                    json!(source_cell(u("family"), u("row12"), u("col12")).unwrap())
                );
            }
            "node" => {
                counts[1] += 1;
                let n = m1::node(c["source_ref"].as_str().unwrap()).unwrap();
                assert!(seen_nodes.insert(n.id));
                assert_eq!(
                    c,
                    json!({"coordinate":n.id,"source_ref":n.source_ref,"parent_id":n.parent_id,"children":n.children,"records":n.records})
                );
            }
            "reflection" => {
                counts[2] += 1;
                let subject = MTreeId::parse(c["subject"].as_str().unwrap()).unwrap();
                assert_eq!(c, json!(reflection(subject, u("phase")).unwrap()));
            }
            "relation" => {
                counts[3] += 1;
                let subject = MTreeId::parse(c["subject"].as_str().unwrap()).unwrap();
                let id = MTreeId::parse(c["value"]["id"].as_str().unwrap()).unwrap();
                assert!(seen.insert((subject, id)));
                let actual = relations(subject).unwrap();
                assert!(actual.iter().any(|r| r.id == id));
                assert_eq!(c["value"], json!(registry.relation(id).unwrap()));
            }
            "grammar" => {
                counts[4] += 1;
                assert_eq!(c, json!(grammar().unwrap()));
            }
            "rotor" => {
                counts[5] += 1;
                let r = rotor(c["a"].as_f64().unwrap(), c["b"].as_f64().unwrap()).unwrap();
                assert_eq!(c["coordinate"], json!(r.coordinate));
                numeric_equal(&c["quaternion"], &json!(r.quaternion), 3e-7);
            }
            "carrier" => {
                counts[6] += 1;
                let r = carrier(c["cycle"].as_str().unwrap().parse().unwrap(), u("tick")).unwrap();
                let r = json!(r);
                for field in [
                    "coordinates",
                    "spinor",
                    "quadrature",
                    "opposite_quadrature",
                    "m2_carrier_count",
                    "genus",
                    "explicate_edges",
                    "identification_slots",
                ] {
                    numeric_equal(
                        &c[field],
                        &r[field],
                        if field == "spinor" { 3e-7 } else { 2e-14 },
                    );
                }
            }
            "torus" => {
                counts[8] += 1;
                let t = m1::torus(c["a"].as_f64().unwrap(), c["b"].as_f64().unwrap()).unwrap();
                assert_eq!(c["coordinate"], json!(t.coordinate));
                for (field, value) in [("x", t.x), ("y", t.y), ("z", t.z)] {
                    numeric_equal(&c[field], &json!(value), 2e-14);
                }
            }
            "finite" => {
                counts[9] += 1;
                assert_eq!(
                    c,
                    json!(
                        finite_field(
                            u("left_position"),
                            u("right_position"),
                            u("direct_word"),
                            u("prime_word")
                        )
                        .unwrap()
                    )
                );
            }
            "traits" => {
                counts[10] += 1;
                assert_eq!(c, json!(source_traits().unwrap()));
            }
            "advance" => {
                counts[7] += 1;
                let r = advance_clock(
                    c["cycle"].as_str().unwrap().parse().unwrap(),
                    u("tick"),
                    c["delta"].as_str().unwrap().parse().unwrap(),
                );
                assert_eq!(c["ok"], r.is_ok());
                if let Ok(r) = r {
                    assert_eq!(c["next_cycle"], r.cycle.to_string());
                    assert_eq!(c["next_tick"], r.tick12);
                }
            }
            _ => panic!("unexpected observation {kind}"),
        }
    }
    let expected_nodes: BTreeSet<_> = registry
        .manifest()
        .nodes
        .iter()
        .filter(|n| n.root_position == Some(1))
        .map(|n| n.id)
        .collect();
    let expected_relations: BTreeSet<_> = expected_nodes
        .iter()
        .flat_map(|n| relations(*n).unwrap().into_iter().map(move |r| (*n, r.id)))
        .collect();
    assert_eq!(seen_nodes, expected_nodes);
    assert_eq!(seen, expected_relations);
    assert_eq!(counts, [864, 43, 86, 1072, 1, 2401, 48, 384, 625, 4096, 1]);
    let result = Command::new("python3")
        .current_dir(&root)
        .args(["scripts/check-m1-literals.py", "--observations"])
        .arg(out.join("c-rust.jsonl"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn every_k4_coordinate_returns_its_own_source_and_computation() {
    let registry = native_m_registry();
    let mut engine = M1Engine::new(config()).unwrap();
    let mut visits = 0;
    for n in registry
        .manifest()
        .nodes
        .iter()
        .filter(|n| n.root_position == Some(1))
    {
        let revision = engine.config().revision.parse().unwrap();
        engine.select(revision, &n.source_ref).unwrap();
        let s = engine.snapshot().unwrap();
        visits += 1;
        assert_eq!(s["selected_reading"]["coordinate"], json!(n.id));
        assert_eq!(s["selected_reading"]["source_records"], json!(n.records));
        assert_eq!(s["config"]["event_ref"], "test:event:M1");
        assert_eq!(s["clock"]["cycle"], "0");
        assert_eq!(s["config"]["family"], 0);
        let body = &s["selected_reading"]["content"];
        if n.source_ref.starts_with("#1-2-") {
            let family = n.source_ref[5..6].parse::<u32>().unwrap();
            assert_eq!(body["projection"]["family"], family);
            assert_eq!(body["source"]["family"], family);
            let cell = m1::cell(family, 3, 4, 0, 0).unwrap();
            assert_eq!(body["projection"], json!(cell));
        }
        if n.source_ref.starts_with("#1-3-") {
            assert_eq!(body["stage"], n.source_ref[5..6].parse::<u32>().unwrap());
        }
        if n.source_ref.starts_with("#1-4.") {
            assert_eq!(
                body["formal"]["stage"],
                n.source_ref[5..6].parse::<u32>().unwrap()
            );
        }
    }
    assert_eq!(visits, 43);
    for invalid in [
        "#2",
        "#1-4-0",
        "#1-3-4.0/1/2/3/4",
        "M1'",
        "M1-3-4.(4.0/1-4.4/5)",
    ] {
        assert!(coordinate_operation(invalid).is_err());
    }
}
#[test]
fn literal_and_numeric_registers_survive_as_distinct_exact_values() {
    for f in 0..6 {
        for r in 0..12 {
            for c in 0..12 {
                let source = source_cell(f, r, c).unwrap();
                assert!(!source.raw_literal.is_empty());
                assert!(!source.digit_root_literal.is_empty());
            }
        }
    }
    assert_eq!(source_cell(3, 3, 4).unwrap().digit_root_literal, "1");
    assert_eq!(m1::cell(3, 3, 4, 0, 0).unwrap().digit_root, Some(9));
    assert_eq!(source_cell(2, 10, 0).unwrap().digit_root_literal, "0/1");
    assert_eq!(m1::cell(2, 10, 0, 0, 0).unwrap().digit_root, Some(1));
    let source = source_cell(5, 3, 4).unwrap();
    assert!(source.raw_literal.contains('/'));
    assert!(!m1::cell(5, 3, 4, 0, 0).unwrap().scalar_valid);
    assert!(source_cell(256, 0, 0).is_err());
    assert!(source_cell(0, 12, 0).is_err());
}
#[test]
fn clock_carrier_and_opposition_keep_four_phase_fibre_combinations() {
    let mut states = BTreeSet::new();
    for cycle in 0..2 {
        for tick in 0..12 {
            let r = carrier(cycle, tick).unwrap();
            states.insert((r.clock.phase, r.clock.hopf_fiber));
            let q = r.spinor;
            assert!((q.iter().map(|v| v * v).sum::<f32>() - 1.).abs() < 1e-6);
            assert!((r.quadrature.iter().map(|v| v * v).sum::<f64>() - 1.).abs() < 1e-14);
            assert_eq!(r.opposite_quadrature, r.quadrature.map(|v| -v));
            let next = carrier(cycle + 1, tick).unwrap();
            for (a, b) in q.into_iter().zip(next.spinor) {
                assert!((a + b).abs() < 1e-6);
            }
            let second = carrier(cycle + 2, tick).unwrap();
            assert_eq!(r.spinor, second.spinor);
        }
    }
    assert_eq!(states.len(), 4);
    assert!(rotor(f64::NAN, 0.).is_err());
    assert!(carrier(0, 12).is_err());
    assert_eq!(advance_clock(0, 11, 1).unwrap(), Clock::new(1, 0).unwrap());
}
#[test]
fn state_changes_are_atomic_revision_checked_and_event_preserving() {
    let mut e = M1Engine::new(config()).unwrap();
    let initial = e.snapshot().unwrap();
    assert!(e.advance(1, 1).is_err());
    assert!(e.select(0, "#2").is_err());
    assert_eq!(initial, e.snapshot().unwrap());
    e.advance(0, 13).unwrap();
    assert_eq!(e.config().cycle, "1");
    assert_eq!(e.config().tick12, 1);
    assert_eq!(e.config().revision, "1");
    let identity = e.pole_identity().unwrap();
    assert_eq!(identity.event_ref(), "test:event:M1");
    assert_eq!(identity.degree720(), 390);
    e.configure_harmonics(
        1,
        HarmonicSelection {
            family: 5,
            row12: 11,
            col12: 11,
            flowering_substage: 3,
            lens12: 11,
            context_frame: 7,
            basis: Basis::Fifths,
        },
    )
    .unwrap();
    let s = e.snapshot().unwrap();
    assert_eq!(s["config"]["revision"], "2");
    assert_eq!(s["music"]["context_frame"], "CF7");
    assert_eq!(s["cell"]["scalar_valid"], false);
    assert!(
        e.configure_harmonics(
            2,
            HarmonicSelection {
                family: 256,
                row12: 0,
                col12: 0,
                flowering_substage: 0,
                lens12: 0,
                context_frame: 1,
                basis: Basis::Chromatic
            }
        )
        .is_err()
    );
    assert_eq!(s, e.snapshot().unwrap());
    let mut cfg = config();
    cfg.revision = u64::MAX.to_string();
    let mut e = M1Engine::new(cfg).unwrap();
    let s = e.snapshot().unwrap();
    assert!(e.advance(u64::MAX, 1).is_err());
    assert!(e.select(u64::MAX, "#1-0").is_err());
    assert_eq!(s, e.snapshot().unwrap());
    let mut cfg = config();
    cfg.cycle = u64::MAX.to_string();
    cfg.tick12 = 11;
    let mut e = M1Engine::new(cfg).unwrap();
    let s = e.snapshot().unwrap();
    assert!(e.advance(0, 1).is_err());
    assert_eq!(s, e.snapshot().unwrap());
}
#[test]
fn json_transport_round_trips_and_rejects_invented_identity_or_authority() {
    let mut request = json!({"schema":CONTRACT,"config":config(),"action":{"kind":"advance","expected_revision":"0","ticks":"24"}});
    let result: Value = serde_json::from_str(&engine_json(&request.to_string()).unwrap()).unwrap();
    assert_eq!(result["config"]["cycle"], "2");
    assert_eq!(result["config"]["revision"], "1");
    request["config"] = result["config"].clone();
    request["action"] = json!({"kind":"select","expected_revision":"1","coordinate":"#1-2-5"});
    let result: Value = serde_json::from_str(&engine_json(&request.to_string()).unwrap()).unwrap();
    assert_eq!(
        result["selected_reading"]["content"]["projection"]["family"],
        5
    );
    request["action"]["expected_revision"] = json!("0");
    assert!(engine_json(&request.to_string()).is_err());
    request["action"] = Value::Null;
    for field in ["revision", "cycle"] {
        let mut r = request.clone();
        r["config"][field] = json!(7);
        assert!(engine_json(&r.to_string()).is_err());
        r["config"][field] = json!("18446744073709551616");
        assert!(engine_json(&r.to_string()).is_err());
    }
    request["grant_root_access"] = json!(true);
    assert!(engine_json(&request.to_string()).is_err());
}

#[test]
fn each_configured_basis_lens_and_context_frame_uses_the_existing_tonal_derivation() {
    for basis in [Basis::Chromatic, Basis::Fifths] {
        for (i, lens) in ql_mef::LensId::ALL.into_iter().enumerate() {
            for mode in ql_mef::ModeKind::ALL {
                let mut cfg = config();
                cfg.basis = basis;
                cfg.lens12 = i as u8;
                cfg.context_frame = mode.index() as u8 + 1;
                let s = M1Engine::new(cfg).unwrap().snapshot().unwrap();
                let native_basis = match basis {
                    Basis::Chromatic => ql_mef::MusicalBasis::Chromatic,
                    Basis::Fifths => ql_mef::MusicalBasis::Fifths,
                };
                let expected = ql_mef::mode_tonic_instance(native_basis, lens, mode);
                assert_eq!(s["music"]["context_frame"], expected.context_frame.code());
                assert_eq!(s["music"]["pitches"], json!(expected.pitches));
                assert_eq!(s["music"]["tonic"], expected.tonic);
            }
        }
    }
}

#[test]
fn finite_state_field_retains_independent_words_and_existing_sixfold_order() {
    let mut pairs = BTreeSet::new();
    let mut faces = BTreeSet::new();
    let mut relations = BTreeSet::new();
    for a in 0..64 {
        for b in 0..64 {
            let v = finite_field(a % 6, b % 6, a, b).unwrap();
            pairs.insert(v.pair_index);
            faces.insert(v.direct_face_index);
            faces.insert(v.prime_face_index);
            relations.insert(v.relation_index);
            assert_eq!(v.bitwise_complement, 63 - a);
            assert_eq!(v.prime_word, b);
        }
    }
    assert_eq!(pairs.len(), 4096);
    assert_eq!(faces.len(), 128);
    assert_eq!(relations.len(), 36);
    for n in [64, 256, u32::MAX] {
        assert!(finite_field(0, 0, n, 0).is_err());
        assert!(finite_field(0, 0, 0, n).is_err());
    }
    assert!(finite_field(6, 0, 0, 0).is_err());
    assert!(finite_field(0, 6, 0, 0).is_err());
}
