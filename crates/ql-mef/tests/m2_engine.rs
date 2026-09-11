use ql_mef::m_tree::{MTreeId, native_m_registry};
use ql_mef::m2::{self, ModalField, Reading72, Register72, TattvaPhase};
use ql_mef::m2_engine::*;
use ql_mef::{ContextFrameId, LensId};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    process::Command,
};

fn input() -> M2Request {
    M2Request::from_json(include_str!(
        "../../../fixtures/kernel/m2-engine-request-v1.json"
    ))
    .unwrap()
}
fn num(s: &str) -> u64 {
    s.parse().unwrap()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-11 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn id(v: Option<MTreeId>) -> u64 {
    v.map(|x| x.as_u64()).unwrap_or(0)
}

#[test]
fn actual_c_and_rust_observations_agree_across_the_field_and_finite_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join(format!("target/m2-parity-{}", std::process::id()));
    std::fs::create_dir_all(&out).unwrap();
    let probe = out.join("probe");
    let built = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
        .current_dir(&root)
        .args([
            "-std=c11",
            "-O2",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
            "scripts/m2-native-probe.c",
            "c/src/m2.c",
            "c/src/m_tree.c",
            "-lm",
            "-o",
        ])
        .arg(&probe)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let output = Command::new(&probe).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let catalogue = m2::catalogue();
    let registry = native_m_registry();
    let holdings = catalogue.coordinates();
    let relation_ids: BTreeSet<_> = m2::m2_relations().map(|r| r.id).collect();
    let values: Vec<_> = (0..72).map(|i| [(i % 7) - 3, (i % 5) - 2]).collect();
    let field = ModalField::new(&values).unwrap();
    let quad = field.quadrature();
    let form = field.form_potential();
    let mut counts = BTreeMap::<&str, usize>::new();
    let mut unique = BTreeSet::new();
    for line in text.lines() {
        let p: Vec<_> = line.split('\t').collect();
        *counts.entry(p[0]).or_default() += 1;
        assert!(unique.insert(line), "duplicate native observation: {line}");
        match p[0] {
            "registry" => assert_eq!(p[1], catalogue.registry_revision()),
            "ground" => assert_eq!(
                p[1..]
                    .iter()
                    .map(|s| s.parse::<i16>().unwrap())
                    .collect::<Vec<_>>(),
                m2::ground_factors()
            ),
            "record" => assert_eq!(
                catalogue
                    .table(p[1])
                    .unwrap()
                    .row(num(p[2]) as usize)
                    .unwrap(),
                p[3..].iter().map(|s| num(s)).collect::<Vec<_>>()
            ),
            "binding" => assert_eq!(
                id(catalogue
                    .table(p[1])
                    .unwrap()
                    .binding(num(p[2]) as usize)
                    .and_then(|r| registry.resolve(r))
                    .map(|n| n.id)),
                u64::from_str_radix(p[3], 16).unwrap()
            ),
            "coordinate" => {
                let n = registry.resolve(p[1]).unwrap();
                assert_eq!(n.root_id, registry.root(2).unwrap().id);
                assert_eq!(n.id.as_u64(), u64::from_str_radix(p[2], 16).unwrap());
                assert_eq!(id(n.parent_id), u64::from_str_radix(p[3], 16).unwrap());
                assert_eq!(
                    holdings
                        .iter()
                        .find(|h| h.id == n.id)
                        .unwrap()
                        .exact_records
                        .len(),
                    num(p[4]) as usize
                );
            }
            "relation" => {
                let r = registry.relation(MTreeId::parse(p[1]).unwrap()).unwrap();
                assert_eq!(r.source_kind, p[2]);
                assert_eq!(id(r.from_id), u64::from_str_radix(p[3], 16).unwrap());
                assert_eq!(id(r.to_id), u64::from_str_radix(p[4], 16).unwrap());
                assert!(relation_ids.contains(&r.id));
            }
            "axes" => {
                let reg = [
                    Register72::Mef,
                    Register72::Tattva,
                    Register72::Decan,
                    Register72::Shem,
                ][num(p[1]) as usize];
                let read = Reading72::new(reg, num(p[2]) as u8).unwrap();
                let axes = [
                    num(p[3]) as u8,
                    num(p[4]) as u8,
                    num(p[5]) as u8,
                    num(p[6]) as u8,
                ];
                assert_eq!(read.axes(), axes);
                assert_eq!(
                    Reading72::from_axes(reg, axes[0], axes[1], axes[2], axes[3]).unwrap(),
                    read
                );
            }
            "signature" => {
                let args = [num(p[1]) as u8, num(p[2]) as u8, num(p[3]) as u8];
                let packed = m2::elemental_signature(args[0], args[1], args[2]).unwrap();
                assert_eq!(packed, num(p[4]) as u8);
                assert_eq!(m2::unpack_signature(packed).unwrap(), args);
            }
            "tattva-step" => {
                let phase = if num(p[2]) == 0 {
                    TattvaPhase::Manifestation
                } else {
                    TattvaPhase::Reabsorption
                };
                let next = m2::tattva_step(num(p[1]) as u8, phase).unwrap();
                assert_eq!(next.is_none(), num(p[3]) == 2);
                assert_eq!(next.unwrap_or(201), num(p[4]) as u8);
            }
            "transforms" => {
                let i = num(p[1]) as u8;
                assert_eq!(m2::decan_to_fibre(i).unwrap(), num(p[2]) as u8);
                assert_eq!(m2::fibre_target(i).unwrap(), num(p[3]) as u8);
                assert_eq!(m2::scalar_compress(i).unwrap(), num(p[4]) as u8);
                assert_eq!(m2::legacy_det(&[i]).unwrap(), num(p[5]));
            }
            "expand" => assert_eq!(m2::scalar_expand(num(p[1]) as u8).unwrap(), num(p[2]) as u8),
            "preempted" => assert_eq!(
                m2::planet_is_preempted(num(p[1]) as u8).unwrap(),
                num(p[2]) != 0
            ),
            "asma-route" => {
                let i = num(p[1]) as u8;
                assert_eq!(m2::asma_is_projective(i).unwrap(), num(p[2]) != 0);
                assert_eq!(m2::asma_is_internal(i).unwrap(), num(p[2]) == 0);
                assert_eq!(
                    m2::digital_root(
                        catalogue
                            .table("asma")
                            .unwrap()
                            .row(usize::from(i))
                            .unwrap()[6]
                    ),
                    num(p[3]) as u8
                );
            }
            "pitch" => near(
                m2::maqam_pitches(num(p[1]) as u8, 220.0).unwrap()[num(p[2]) as usize],
                p[3].parse().unwrap(),
            ),
            "aspect" => {
                let value = m2::aspect(p[1].parse().unwrap(), p[2].parse().unwrap()).unwrap();
                assert_eq!(value.kind.unwrap_or(255), num(p[3]) as u8);
                near(value.angle, p[4].parse().unwrap());
                near(value.orb, p[5].parse().unwrap());
            }
            "quadrature" => assert_eq!(
                quad.coefficients()[num(p[1]) as usize],
                [p[2].parse::<i64>().unwrap(), p[3].parse().unwrap()]
            ),
            "form" => assert_eq!(
                form[num(p[1]) as usize],
                [p[2].parse::<i64>().unwrap(), p[3].parse().unwrap()]
            ),
            "power" => assert_eq!(field.total_power(), num(p[1]) as u128),
            other => panic!("unconsumed native observation {other}"),
        }
    }
    let expected = BTreeMap::from([
        ("registry", 1),
        ("ground", 1),
        ("record", 764),
        ("binding", 764),
        ("coordinate", 597),
        ("relation", 8876),
        ("axes", 288),
        ("signature", 160),
        ("tattva-step", 72),
        ("transforms", 72),
        ("expand", 64),
        ("asma-route", 100),
        ("preempted", 10),
        ("pitch", 576),
        ("aspect", 129600),
        ("quadrature", 72),
        ("form", 64),
        ("power", 1),
    ]);
    assert_eq!(counts, expected);
    let receipt = root.join("target/m2-receipt");
    std::fs::create_dir_all(&receipt).unwrap();
    std::fs::write(receipt.join("c-rust-observed.tsv"), &text).unwrap();
    std::fs::write(
        receipt.join("rust-parity.json"),
        serde_json::to_string_pretty(&json!({"schema":"ql.m2-parity-observation/v1",
        "result":"pass","registry_revision":catalogue.registry_revision(),"counts":counts,
        "standing":"executed-C-versus-Rust-finite-scope; not-live-Neo4j-or-experiential-parity"}))
        .unwrap(),
    )
    .unwrap();
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    std::fs::remove_dir_all(out).unwrap();
}

#[test]
fn grouped_mef_carrier_and_interleaved_lenses_roundtrip_without_changing_frames() {
    let mut refs = BTreeSet::new();
    for i in 0..72 {
        let reading = Reading72::new(Register72::Mef, i).unwrap();
        let s = reading.mef_sublens().unwrap();
        assert_eq!(s.lens().lens().index(), i / 6 % 6);
        assert_eq!(s.lens().lens().slot() % 2, u8::from(i >= 36));
        assert_eq!(Reading72::from_sublens(s), reading);
        assert!(refs.insert(s.to_string()));
    }
    assert_eq!(refs.len(), 72);
    assert_eq!(ContextFrameId::ALL.len(), 7);
    for frame in ContextFrameId::ALL {
        for lens in LensId::ALL {
            let r = m2::context_condition(frame, lens);
            assert_eq!(r.mef_sublens().unwrap().lens().lens(), lens);
            assert_eq!(
                r.index() % 6,
                frame.canonical_selection().local_position().value()
            );
        }
    }
    assert!(
        Reading72::new(Register72::Shem, 0)
            .unwrap()
            .mef_sublens()
            .is_err()
    );
}
#[test]
fn carriers_are_distinct_and_three_transforms_are_not_substituted() {
    let mef = Reading72::new(Register72::Mef, 65).unwrap();
    let decan = mef.reinterpret_carrier(Register72::Decan);
    assert_eq!(mef.index(), decan.index());
    assert_ne!(mef.axes(), decan.axes());
    assert_ne!(mef, decan);
    assert_eq!(m2::scalar_compress(65).unwrap(), 57);
    assert_eq!(m2::fibre_target(65).unwrap(), 59);
    assert_eq!(m2::legacy_det(&[65]).unwrap(), 1 << 8);
    for i in 0..72 {
        assert_eq!(m2::fibre_target(i).unwrap() / 16, i / 18);
    }
    assert_eq!(m2::decan_to_fibre(0).unwrap(), 18); // Fire -> EFWA Fire, not Earth
    for i in 0..36 {
        let a = m2::situated_decan(f64::from(i) * 10.0 + 0.1, false).unwrap();
        let b = m2::situated_decan(f64::from(i) * 10.0 + 0.1, true).unwrap();
        assert_eq!(a.index() + 1, b.index());
    }
}
#[test]
fn incomplete_or_conflicting_correspondences_remain_visible_in_the_full_field() {
    let c = m2::catalogue();
    assert_eq!(c.coordinates().len(), 597);
    let counts: Vec<_> = c
        .tables()
        .iter()
        .map(|t| {
            (
                t.name(),
                (0..t.rows().len())
                    .filter(|i| t.binding(*i).is_some())
                    .count(),
            )
        })
        .collect();
    assert_eq!(
        counts,
        vec![
            ("carrier", 0),
            ("mef", 0),
            ("tattva", 34),
            ("decan", 73),
            ("planet", 9),
            ("chakra", 8),
            ("shem", 72),
            ("ratio", 0),
            ("maqam", 72),
            ("station", 15),
            ("asma", 66),
            ("mantra", 50),
            ("element", 5),
            ("det", 0),
            ("resonance", 0),
            ("routing", 0)
        ]
    );
    assert_eq!(c.table("planet").unwrap().binding(3), Some("#2-5-2"));
    assert!(c.table("planet").unwrap().binding(7).is_none());
    assert_eq!(c.table("station").unwrap().binding(6), Some("#2-4.2-5-0"));
    assert_eq!(c.table("asma").unwrap().binding(2), Some("#2-4.0-0/1-0-0"));
    assert_eq!(c.table("asma").unwrap().row(2).unwrap()[6], 62); // retained assertion, not Bimba's 90
    assert!(
        !m2::causal_resonances("#2-1")
            .unwrap()
            .iter()
            .any(|r| r.source_kind != "CAUSAL_RESONANCE")
    );
    assert!(m2::causal_resonances("#3").is_err());
    for t in c.tables() {
        for i in 0..t.rows().len() {
            let r = c.reading(t.name(), i).unwrap();
            assert_eq!(r.standing, m2::RETAINED_STANDING);
            m2::linked_readings(t.name(), i).unwrap();
        }
    }
}
#[test]
fn bounded_modal_operations_preserve_quadrature_not_coherent_fold_power() {
    let mut values = vec![[0, 0]; 72];
    values[0] = [m2::MAX_COMPONENT, 0];
    values[16] = [m2::MAX_COMPONENT, 0];
    let field = ModalField::new(&values).unwrap();
    let q = field.quadrature();
    assert_eq!(q.total_power(), field.total_power());
    assert_eq!(q.quadrature().quadrature().quadrature(), field);
    assert_eq!(field.form_potential()[0], [2 * m2::MAX_COMPONENT, 0]);
    let form_power: u128 = field
        .form_potential()
        .iter()
        .map(|[re, im]| {
            re.unsigned_abs() as u128 * re.unsigned_abs() as u128
                + im.unsigned_abs() as u128 * im.unsigned_abs() as u128
        })
        .sum();
    assert_ne!(form_power, field.total_power());
    for i in 0..72 {
        let mut basis = vec![[0, 0]; 72];
        basis[i] = [1, -1];
        let f = ModalField::new(&basis).unwrap();
        assert_eq!(
            f.form_potential()[usize::from(m2::fibre_target(i as u8).unwrap())],
            [1, -1]
        );
    }
    values[0] = [i64::MIN, 0];
    assert!(ModalField::new(&values).is_err());
    values[0] = [m2::MAX_COMPONENT + 1, 0];
    assert!(ModalField::new(&values).is_err());
    assert!(ModalField::new(&values[..71]).is_err());
}
#[test]
fn invalid_finite_inputs_fail_instead_of_aliasing_another_coordinate() {
    for reg in [
        Register72::Mef,
        Register72::Tattva,
        Register72::Decan,
        Register72::Shem,
    ] {
        assert!(Reading72::new(reg, 72).is_err());
        assert!(Reading72::from_axes(reg, 0, 0, 0, 255).is_err());
    }
    assert!(m2::tattva_step(36, TattvaPhase::Manifestation).is_err());
    assert!(m2::elemental_signature(5, 0, 0).is_err());
    assert!(m2::unpack_signature(7).is_err());
    assert!(m2::legacy_det(&[72]).is_err());
    assert!(m2::asma_is_projective(100).is_err());
    assert!(m2::scalar_expand(64).is_err());
    for a in [f64::NAN, f64::INFINITY, -1.0, 360.0] {
        assert!(m2::aspect(a, 0.0).is_err());
        assert!(m2::situated_decan(a, false).is_err());
    }
    for a in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MAX] {
        assert!(m2::maqam_pitches(0, a).is_err());
    }
    assert_eq!(m2::digital_root(u64::MAX), 6);
}
#[test]
fn engine_emits_every_domain_with_event_provenance_and_no_invented_resonator() {
    let request = input();
    let frame = request.execute().unwrap();
    assert_eq!(frame.schema, m2::ENGINE_CONTRACT);
    assert_eq!(frame.identity, request.stamp.identity);
    assert_eq!(frame.domains.len(), 16);
    assert_eq!(frame.structural_coordinate_count, 597);
    assert_eq!(frame.structural_relation_count, 8876);
    assert_eq!(frame.modal.coefficients.len(), 72);
    assert_eq!(frame.modal.form_potential.len(), 64);
    assert_eq!(frame.numerical_ground.field72, 72);
    assert_eq!(frame.numerical_ground.base36, 36);
    assert_eq!(
        frame.continuous_standing,
        "unavailable-no-physical-solver-implied"
    );
    assert!(frame.resonator.is_none());
    assert_eq!(frame.world.len(), 2);
    assert_eq!(frame.aspects.len(), 1);
    assert_eq!(frame.world[0].age_ms, 100);
    assert!(
        frame
            .world
            .iter()
            .all(|w| w.standing.contains("not-authenticated"))
    );
    let mut all = request.clone();
    all.selections = m2::catalogue()
        .tables()
        .iter()
        .flat_map(|t| {
            (0..t.rows().len()).map(move |i| Selection {
                table: t.name().into(),
                index: i,
            })
        })
        .collect();
    assert_eq!(all.execute().unwrap().selected_descriptors.len(), 764);
    assert!(M2Request::from_json(&serde_json::to_string(&request).unwrap()).is_ok());
    let mut value = serde_json::to_value(request).unwrap();
    value["invented_field"] = json!(true);
    assert!(M2Request::from_json(&value.to_string()).is_err());
}
fn with_resonator() -> M2Request {
    let mut r = input();
    r.resonator = Some(ResonatorState {
        stamp: r.stamp.clone(),
        provider_ref: "test:solver".into(),
        geometry_ref: "test:geometry".into(),
        material_ref: "test:material".into(),
        material_model_ref: "test:constitutive-model".into(),
        material_parameters: BTreeMap::new(),
        modes: vec![ContinuousMode {
            mode_ref: "test:mode-1".into(),
            source_coordinate: "#2-2".into(),
            material_fibre: MaterialFibre::Earth,
            carrier_weights: vec![CarrierWeight {
                carrier: 0,
                weight: 1.0,
            }],
            frequency_hz: 220.0,
            amplitude: [1.0, 0.0],
            excitation: [0.1, 0.0],
            damping_per_second: 0.01,
            nodal_state_ref: "test:nodes".into(),
            antinodal_state_ref: "test:antinodes".into(),
        }],
    });
    r
}
#[test]
fn physical_modes_remain_supplied_projected_state_not_seventy_two_assumed_eigenmodes() {
    let r = with_resonator();
    let f = r.execute().unwrap();
    assert_eq!(f.resonator.unwrap().modes.len(), 1);
    assert_eq!(
        f.continuous_standing,
        "provider-supplied-not-experientially-verified"
    );
    let mut many = r.clone();
    for i in 1..80 {
        let mut mode = many.resonator.as_ref().unwrap().modes[0].clone();
        mode.mode_ref = format!("test:mode-{}", i + 1);
        many.resonator.as_mut().unwrap().modes.push(mode);
    }
    assert_eq!(many.execute().unwrap().resonator.unwrap().modes.len(), 80);
}
#[test]
fn stale_nonfinite_cross_event_and_cross_element_provider_data_are_rejected() {
    let r = with_resonator();
    let mut v = serde_json::to_value(&r).unwrap();
    let cases: Vec<(&str, Value)> = vec![
        ("/registry_revision", json!("stale")),
        ("/stamp/identity/event_ref", json!("different:event")),
        ("/resonator/stamp/identity/profile_generation", json!(999)),
        ("/resonator/geometry_ref", json!("")),
        ("/resonator/modes/0/source_coordinate", json!("#3")),
        ("/resonator/modes/0/material_fibre", json!("fire")),
        ("/resonator/modes/0/frequency_hz", json!(-1)),
        ("/resonator/modes/0/damping_per_second", json!(-1)),
        ("/resonator/modes/0/carrier_weights/0/carrier", json!(72)),
        ("/tick12", json!(12)),
        ("/degree720", json!(720)),
        ("/context_frames/0", json!("(4/5/0)")),
        ("/world_observations/0/observed_at_unix_ms", json!(999999)),
        ("/world_observations/0/planet_id", json!(10)),
    ];
    for (path, bad) in cases {
        let original = v.pointer(path).unwrap().clone();
        *v.pointer_mut(path).unwrap() = bad;
        assert!(M2Request::from_json(&v.to_string()).is_err(), "{path}");
        *v.pointer_mut(path).unwrap() = original;
    }
    let mut bad = r.clone();
    bad.resonator.as_mut().unwrap().modes[0].amplitude[0] = f64::NAN;
    assert!(bad.validate().is_err());
    let mut duplicate = r.clone();
    let mode = duplicate.resonator.as_ref().unwrap().modes[0].clone();
    duplicate.resonator.as_mut().unwrap().modes.push(mode);
    assert!(duplicate.validate().is_err());
    let mut duplicate = r.clone();
    duplicate.mef_conditions.push(duplicate.mef_conditions[0]);
    assert!(duplicate.validate().is_err());
    let mut bad = r;
    bad.world_observations[0].longitude_degrees = f64::INFINITY;
    assert!(bad.validate().is_err());
}
#[test]
fn malformed_catalogue_cannot_create_coordinates_or_launder_source_standing() {
    let original: Value = serde_json::from_str(m2::RETAINED_C_JSON).unwrap();
    for (path, bad) in [
        ("/registry_revision", json!("stale")),
        ("/standing", json!("verified")),
        ("/tables/2/bindings/0", json!("#3")),
        ("/tables/2/bindings/0", json!("#2-2-1-3")),
        ("/tables/2/rows/0/0", json!(999999)),
    ] {
        let mut v = original.clone();
        *v.pointer_mut(path).unwrap() = bad;
        assert!(m2::Catalogue::from_json(&v.to_string()).is_err(), "{path}");
    }
}

#[test]
fn wire_integers_do_not_roundtrip_through_lossy_javascript_numbers() {
    let reading = m2::catalogue().reading("det", 63).unwrap();
    let wire = serde_json::to_value(&reading).unwrap();
    assert_eq!(wire["fields"]["mask"].as_str(), Some("9223372036854775808"));
    let roundtrip: m2::DescriptorReading = serde_json::from_value(wire).unwrap();
    assert_eq!(roundtrip.fields["mask"], 1_u64 << 63);
    let mut request = input();
    request.stamp.identity.profile_generation = MAX_EXACT_JSON_INTEGER + 1;
    assert!(request.validate().is_err());
    let mut request = input();
    request.at_unix_ms = MAX_EXACT_JSON_INTEGER + 1;
    assert!(request.validate().is_err());
}

#[test]
fn active_vimarsha_is_stamped_and_uses_the_shared_pose_and_distinct_musical_mode() {
    let mut request = input();
    let frame = request.execute().unwrap();
    let reading = frame.vimarsha.unwrap();
    assert_eq!(reading.input.stamp.identity, frame.identity);
    assert_eq!(reading.reading.source_coordinate, "#2-1");
    assert!(
        reading
            .reading
            .audio_octet_hz
            .iter()
            .all(|v| v.is_finite() && *v > 0.0)
    );
    for n in &reading.reading.nodal_quartet {
        assert!([0, 5].contains(&n.ql_position));
        assert!((1..=12).contains(&n.m));
    }
    let good = request.vimarsha.clone().unwrap();
    for field in 0..6 {
        let mut bad = good.clone();
        match field {
            0 => bad.stamp.identity.profile_generation += 1,
            1 => bad.pose_ordinal = 472,
            2 => bad.musical_mode = 7,
            3 => bad.lens = 12,
            4 => bad.harmonic_ratio = [1, 0],
            _ => bad.pose_source_ref.clear(),
        }
        request.vimarsha = Some(bad);
        assert!(request.execute().is_err());
    }
}

#[test]
fn decan_elements_follow_the_retained_throughline_not_shared_numeric_ids() {
    let catalogue = m2::catalogue();
    for i in 0..73 {
        let links = m2::linked_readings("decan", i).unwrap();
        let element = links.iter().find(|r| r.table == "element").unwrap();
        let source_element = catalogue.table("decan").unwrap().row(i).unwrap()[0];
        assert_eq!(element.fields["decan_element"], source_element);
        assert_eq!(element.index, [2, 4, 1, 3, 0][source_element as usize]);
        assert_eq!(links.len(), if i == 72 { 1 } else { 2 });
    }
}
