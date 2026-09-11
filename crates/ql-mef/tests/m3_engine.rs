use ql_core::m3_clock::{M3_CLOCK_SOURCE, M3Clock, parse_recorded_clock, recorded_clock};
use ql_core::*;
use ql_mef::m3_engine::{M3Engine, M3NodeKind, native_m3_engine};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

fn execute(command: &mut Command) -> Vec<u8> {
    let result = command.output().expect("execute acceptance tool");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}
fn number(v: &Value, index: usize) -> u8 {
    v[index].as_u64().unwrap().try_into().unwrap()
}

#[test]
fn executed_native_c_equals_rust_across_full_finite_m3_field() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/m3-rust");
    std::fs::create_dir_all(&out).unwrap();
    execute(
        Command::new("python3")
            .current_dir(&root)
            .arg("scripts/generate-m3.py")
            .arg("--out")
            .arg(out.join("m3_data.inc")),
    );
    execute(
        Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()))
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
                "migration/epi-kernel/k7-m3-probe.c",
                "c/src/m3.c",
                "c/src/m_tree.c",
                "-lm",
                "-o",
            ])
            .arg(out.join("probe")),
    );
    let output = execute(&mut Command::new(out.join("probe")));
    std::fs::write(out.join("c-rust-parity.jsonl"), &output).unwrap();
    let engine = native_m3_engine();
    let mut counts = BTreeMap::<String, usize>::new();
    let mut seen = BTreeSet::new();
    for line in String::from_utf8(output).unwrap().lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let kind = v[0].as_str().unwrap();
        *counts.entry(kind.into()).or_default() += 1;
        let index = v[1].as_u64().unwrap() as usize;
        let compound = matches!(
            kind,
            "node"
                | "line"
                | "matrix"
                | "pose"
                | "rotation"
                | "quaternion"
                | "transcription"
                | "partner"
        );
        let key = (
            kind.to_owned(),
            index,
            if compound {
                v[2].to_string()
            } else {
                String::new()
            },
        );
        assert!(seen.insert(key), "duplicate observation: {line}");
        let c = Codon64::new(index as u8);
        let expected = match kind {
            "node" => {
                let node = engine
                    .node(M3NodeKind::ALL[index], v[2].as_u64().unwrap() as usize)
                    .unwrap();
                json!([kind, index, v[2], node.id, node.source_ref])
            }
            "pair" => {
                let pair = PairIndex16::from_index(index as u8).unwrap();
                json!([
                    kind,
                    index,
                    pair_sum(pair.first(), pair.second()),
                    pair_difference(pair.first(), pair.second())
                ])
            }
            "trigram" => {
                let t = Trigram::LUT[index];
                json!([
                    kind,
                    index,
                    t.id,
                    t.binary,
                    t.earlier_heaven,
                    t.later_heaven,
                    t.element,
                    t.family_role,
                    t.degree_anchor
                ])
            }
            "codon" => {
                let sites = c.site_values().map(|s| s.value());
                let charges = c.four_charge();
                json!([
                    kind,
                    index,
                    sites[0],
                    sites[1],
                    sites[2],
                    c.classify() as u8,
                    c.rotational_state_count(),
                    complement(c.address()),
                    nuclear_hexagram(c.address()),
                    wc_anticodon(c).address(),
                    m3_codon_amino_index(c),
                    u8::from(m3_codon_is_rna_capable(c)),
                    charges.pp,
                    charges.mm,
                    charges.mp,
                    charges.pm
                ])
            }
            "line" => json!([
                kind,
                index,
                v[2],
                c.line_change(number(&v, 2)).unwrap().address()
            ]),
            "matrix" => {
                let matrix = number(&v, 2);
                let target = match matrix {
                    0 => Some(complement(c.address())),
                    1 => Some(((c.address() & 7) << 3) | (c.address() >> 3)),
                    2 => resonance_entry(c.address()),
                    _ => panic!("unknown matrix"),
                };
                json!([
                    kind,
                    index,
                    matrix,
                    if target.is_some() { 0 } else { 2 },
                    target.unwrap_or(255)
                ])
            }
            "profile" => {
                let p = rotational_profile(c);
                json!([
                    kind,
                    index,
                    p.state_count(),
                    p.state_type() as u8,
                    p.anchor_pair_a().map(|p| p.index()).unwrap_or(255),
                    p.anchor_pair_b().map(|p| p.index()).unwrap_or(255),
                    p.paired_codon().map(|c| c.address()).unwrap_or(255)
                ])
            }
            "pose" => json!([
                kind,
                index,
                v[2],
                RotationalPose::new(c, number(&v, 2)).unwrap().ordinal()
            ]),
            "rotation" => {
                // C preserves sweep order with an explicit rank field; Rust returns
                // rank order. Join by independent sweep identity, not array position.
                let sweep = number(&v, 2);
                let candidates = generate_rotational_states(c);
                let r = candidates
                    .iter()
                    .find(|r| {
                        r.polarity as u8 == sweep / 4
                            && r.resulting_codon.middle().bits() == sweep % 4
                    })
                    .unwrap();
                json!([
                    kind,
                    index,
                    v[2],
                    r.pair1.index(),
                    r.pair2.index(),
                    r.resulting_codon.address(),
                    r.polarity as u8,
                    r.rotation_slot,
                    r.rotation_degrees,
                    u8::from(r.is_non_dual),
                    r.rotational_value
                ])
            }
            "quaternion" => {
                let q = quat_codon_state(c, number(&v, 2));
                for (i, n) in [q.w, q.x, q.y, q.z].into_iter().enumerate() {
                    assert!(
                        (v[i + 3].as_f64().unwrap() - f64::from(n)).abs() < 0.00001,
                        "{line}"
                    );
                }
                v.clone()
            }
            "transcription" => json!([
                kind,
                index,
                v[2],
                String::from_utf8(M3Engine::transcribe(c, number(&v, 2) == 1).to_vec()).unwrap()
            ]),
            "partner" => {
                let family = [
                    MatrixFamily::Complementary,
                    MatrixFamily::MovingResting,
                    MatrixFamily::SameQuality,
                ][index];
                let nucleotide = Nucleotide::from_bits(number(&v, 2)).unwrap();
                json!([kind, index, v[2], matrix_partner(family, nucleotide).bits()])
            }
            "minor" => {
                let card = &TarotBridge::kernel().minor()[index];
                json!([
                    kind,
                    index,
                    card.suit().bits(),
                    card.pip().value(),
                    card.codon_a().address(),
                    card.codon_b().map(|c| c.address()).unwrap_or(255)
                ])
            }
            "major" => {
                let card = &TarotBridge::kernel().major()[index];
                json!([
                    kind,
                    index,
                    card.name(),
                    card.chromosome_pair(),
                    card.amino_acid_index()
                ])
            }
            "transduce" => json!([
                kind,
                index,
                format!(
                    "{:016x}",
                    transduce_vibration_to_symbol(&[index as u8]).unwrap()
                ),
                apply_epogdoon_compression(index as u8).unwrap(),
                u8::from(is_evolutionary_gap(index as u8).unwrap())
            ]),
            "record" => {
                let mut values = vec![json!(kind), json!(index)];
                values.extend(recorded_clock()[index].fields.map(|field| json!(field)));
                Value::Array(values)
            }
            "clock" => {
                let f = engine.clock(M3Clock::at_steps(index as u64));
                let c = f.clock;
                json!([
                    kind,
                    index,
                    c.degree720(),
                    c.degree360(),
                    c.layer(),
                    c.tick12(),
                    c.decan_phase(),
                    c.polar720(),
                    c.uniform_hexagram_estimate(),
                    c.completed_double_covers(),
                    f.degree.id,
                    f.backbone.id,
                    f.clockwise.id,
                    f.polar.id
                ])
            }
            _ => panic!("unknown native record: {line}"),
        };
        assert_eq!(v, expected, "{kind}[{index}]");
    }
    assert_eq!(
        counts,
        BTreeMap::from(
            [
                ("node", 623),
                ("pair", 16),
                ("trigram", 8),
                ("codon", 64),
                ("line", 384),
                ("matrix", 192),
                ("profile", 64),
                ("pose", 472),
                ("rotation", 512),
                ("quaternion", 512),
                ("transcription", 128),
                ("partner", 12),
                ("minor", 56),
                ("major", 22),
                ("transduce", 72),
                ("record", 360),
                ("clock", 1441),
            ]
            .map(|(k, v)| (k.to_owned(), v))
        )
    );
}

#[test]
fn clock_boundaries_and_unreconciled_symbolic_values_are_explicit() {
    let engine = native_m3_engine();
    assert_eq!(engine.coordinates().count(), 996);
    for d in 0..360 {
        let a = engine.clock(M3Clock::at_steps(d));
        let b = engine.clock(M3Clock::at_steps(d + 360));
        assert_eq!(a.degree.id, b.degree.id);
        assert_eq!(a.backbone.id, b.backbone.id);
        assert_eq!(a.clock.layer(), 0);
        assert_eq!(b.clock.layer(), 1);
        assert_eq!(a.clock.tick12(), b.clock.tick12());
        assert_eq!(b.clock.decan_phase(), a.clock.decan_phase() + 36);
        assert_eq!(
            a.clock.recorded_projection().reconciled_symbolic_fields(),
            0
        );
        assert!(
            engine
                .source_relations(a.degree.id)
                .any(|r| r.source_kind == "ANCHORED_BY" && r.to_id == Some(a.backbone.id))
        );
    }
    assert_eq!(
        M3Clock::at_steps(719)
            .advance(1)
            .unwrap()
            .completed_double_covers(),
        1
    );
    assert_eq!(M3Clock::at_steps(360).completed_double_covers(), 0);
    assert!(M3Clock::at_steps(u64::MAX).advance(1).is_none());
    assert_eq!(
        engine.clock(M3Clock::at_steps(0)).degree.source_ref,
        "#3-5-5/0-0/360"
    );
    assert!(parse_recorded_clock(&M3_CLOCK_SOURCE.replace("{   1U,", "{0U,")).is_err());
    assert!(parse_recorded_clock(&M3_CLOCK_SOURCE.replace("{   1U,", "{1.5f,")).is_err());
    assert!(
        parse_recorded_clock(
            &M3_CLOCK_SOURCE.replace("CLOCK_DEGREE_LUT[360]", "CLOCK_DEGREE_LUT[359]")
        )
        .is_err()
    );
}
