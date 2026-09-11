use ql_core::m3_clock::M3Clock;
use ql_core::*;
use ql_mef::m3_engine::native_m3_engine;
use ql_mef::m3_source::native_m3_source;
use serde_json::{Value, json};
use std::{path::Path, process::Command};
fn run(c: &mut Command) -> Vec<u8> {
    let r = c.output().unwrap();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    r.stdout
}
fn form_values(kind: &str, index: usize, extra: usize, f: &FoldState) -> Value {
    let c = f.codon();
    let ap = f.aperture16();
    let g = f.geometry();
    let sites = f.sites();
    let elements = c
        .nucleotides()
        .map(|n| [2usize, 1, 0, 3][n.bits() as usize]);
    let mut counts = [0u8; 4];
    for e in elements {
        counts[e] += 1;
    }
    let mut row = vec![
        json!(kind),
        json!(index),
        json!(extra),
        json!(c.address()),
        json!(f.rotational_index()),
        json!(f.state_count()),
        json!(ap.index()),
        json!(ap.reciprocal().index()),
        json!(f.active_matrix_axis() as u8),
        json!(f.rotational_pose().ordinal()),
        json!(f.fibonacci_phase60()),
        json!(g.pair_xy.index()),
        json!(g.pair_yz.index()),
        json!(g.hinge.bits()),
    ];
    row.extend(sites.iter().map(|s| json!(s.signed_angle)));
    row.extend(sites.iter().map(|s| json!(s.angular_velocity)));
    row.extend([
        json!(g.pair_angle_xy().0),
        json!(g.pair_angle_yz().0),
        json!(ap.division_deg10()),
        json!(ap.complement_deg10()),
        json!(ap.orientation().0),
    ]);
    row.extend(elements.map(|e| json!(e)));
    row.extend(counts.map(|e| json!(e)));
    json!(row)
}
#[test]
fn source_reading_retains_full_depth_and_multivalued_genetics() {
    let s = native_m3_source();
    assert_eq!(s.nodes().len(), 996);
    assert_eq!(s.relations().len(), 4891);
    assert_eq!(s.discrepancies().len(), 563);
    let atg = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::T, Nucleotide::G);
    assert_eq!(
        s.genetic(atg)
            .translations
            .iter()
            .filter(|r| r.kind == "TRANSLATES_TO")
            .count(),
        2
    );
    assert_eq!(
        (0..64)
            .filter(|c| s.genetic(Codon64::new(*c)).rna.is_some())
            .count(),
        37
    );
    assert!(
        s.relations()
            .iter()
            .any(|r| r.kind == "USES_Pair" && r.to_id.is_none())
    );
}
#[test]
fn native_source_field_and_all_lawful_forms_equal_rust() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m3-domain");
    std::fs::create_dir_all(&out).unwrap();
    run(Command::new("python3")
        .current_dir(&root)
        .args(["scripts/generate-m3.py", "--out"])
        .arg(out.join("m3_data.inc")));
    run(Command::new("python3")
        .current_dir(&root)
        .args(["scripts/m3-domain.py", "--c-output"])
        .arg(out.join("m3_domain_data.inc")));
    run(Command::new("cc")
        .current_dir(&root)
        .args([
            "-std=c11",
            "-O1",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
        ])
        .arg(format!("-I{}", out.display()))
        .args([
            "migration/epi-kernel/k7-m3-domain-probe.c",
            "c/src/m3_domain.c",
            "c/src/m3.c",
            "c/src/m_tree.c",
            "-lm",
            "-o",
        ])
        .arg(out.join("probe")));
    let bytes = run(&mut Command::new(out.join("probe")));
    std::fs::write(out.join("c-rust.jsonl"), &bytes).unwrap();
    let source = native_m3_source();
    let engine = native_m3_engine();
    let forms: Vec<_> = all_poses()
        .flat_map(|p| (0..16).map(move |a| (p, a)))
        .enumerate()
        .map(|(i, (p, a))| {
            let mut f =
                FoldState::from_codon(p.codon(), ApertureIndex::new(a).unwrap(), (i % 60) as u16);
            f.set_rotational_index(p.slot()).unwrap();
            f.set_matrix_axis(MatrixFamily::ALL[i % 3].axis());
            f
        })
        .collect();
    let mut counts = std::collections::BTreeMap::new();
    let mut seen = std::collections::BTreeSet::new();
    for line in std::str::from_utf8(&bytes).unwrap().lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let k = v[0].as_str().unwrap();
        *counts.entry(k.to_string()).or_insert(0usize) += 1;
        let index = v[1].as_u64().unwrap_or(0) as usize;
        let extra = v[2].as_u64().unwrap_or(0) as usize;
        assert!(seen.insert((k.to_string(), index, extra)));
        let expected = match k {
            "revision" => json!([k, source.revision()]),
            "source-node" => {
                let n = &source.nodes()[index];
                json!([k, index, n.id, n.record, n.role])
            }
            "source-edge" => {
                let e = &source.relations()[index];
                json!([
                    k,
                    index,
                    e.id,
                    e.from_id
                        .map(|x| format!("{:016x}", x.as_u64()))
                        .unwrap_or("0000000000000000".into()),
                    e.to_id
                        .map(|x| format!("{:016x}", x.as_u64()))
                        .unwrap_or("0000000000000000".into()),
                    e.record,
                    e.kind,
                    e.properties
                ])
            }
            "cell" => {
                let c = &source.matrix_cells()[index];
                let mut r = vec![
                    json!(k),
                    json!(index),
                    json!(c.id),
                    json!(source.resolve(&c.hexagram_ref).unwrap().id),
                    json!(c.resolves_relation),
                    json!(c.family),
                    json!(c.address),
                ];
                r.extend(
                    c.pair_relations
                        .iter()
                        .chain(&c.codon_relations)
                        .map(|id| json!(id)),
                );
                json!(r)
            }
            "projection" => {
                let clock = engine.clock(M3Clock::at_steps(index as u64));
                let p = source.backbone(clock.backbone.id).unwrap();
                json!([
                    k,
                    index,
                    p.id,
                    p.codon_id,
                    p.hexagram_id,
                    p.codon_address,
                    p.hexagram_address,
                    p.source_record
                ])
            }
            "active" => json!([
                k,
                index,
                extra,
                quat_active_state(
                    quat_codon_state(Codon64::new(index as u8), extra as u8),
                    Codon64::new(index as u8)
                )
            ]),
            "form" => form_values(k, index, 0, &forms[index]),
            "applied" | "gap" => match forms[index].apply_matrix(MatrixFamily::ALL[extra]).unwrap()
            {
                ApplyOutcome::Applied(f) => form_values("applied", index, extra, &f),
                ApplyOutcome::Provisional => json!(["gap", index, extra]),
            },
            _ => panic!("unexpected record"),
        };
        assert_eq!(v, expected, "{k}/{index}/{extra}");
    }
    assert_eq!(counts.get("active"), Some(&512));
    assert_eq!(counts.get("source-node"), Some(&996));
    assert_eq!(counts.get("source-edge"), Some(&4891));
    assert_eq!(counts.get("cell"), Some(&184));
    assert_eq!(counts.get("projection"), Some(&1441));
    assert_eq!(counts.get("form"), Some(&(472 * 16)));
    assert_eq!(counts["applied"] + counts["gap"], 472 * 16 * 3);
}
