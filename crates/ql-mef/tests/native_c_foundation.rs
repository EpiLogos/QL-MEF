//! K1 executes native C, then checks its complete stream against current Rust.
//! This is a test adapter, not a second production address or M registry.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use ql_core::{
    CanonicalCrossPass, D2CrossPassKind, HOLOGRAPHIC_KERNEL_CONTRACT_VERSION, KernelRelationId,
    QlCoordinate, QlFace, QlFamily, QlPosition, RelationFamily, VakFamily, VakInstruction,
    canonical_cross_pass_d2, canonical_cross_pass_d3,
};
use ql_mef::{
    ContextFrameId, LensId, MefGrain, MefRotation, MefUnitFace, MusicalBasis, SublensRef,
    pitch_at_lens,
};

const COUNTS: &str = include_str!("../../../fixtures/kernel/native-c-foundation-counts-v1.tsv");

fn family(code: &str) -> QlFamily {
    QlFamily::ALL
        .into_iter()
        .find(|f| f.code() == code)
        .unwrap()
}

fn face(code: &str) -> QlFace {
    [QlFace::Direct, QlFace::Conjugate]
        .into_iter()
        .find(|f| f.kernel_code() == code)
        .unwrap()
}

fn relation(code: &str) -> KernelRelationId {
    KernelRelationId::ALL
        .into_iter()
        .find(|r| r.as_str() == code)
        .unwrap()
}

fn position(value: &str) -> QlPosition {
    QlPosition::new(value.parse().unwrap()).unwrap()
}

fn label(f: QlFamily, c: QlCoordinate) -> String {
    let prefix = if f == QlFamily::None { "#" } else { f.code() };
    let prime = if c.face == QlFace::Conjugate { "'" } else { "" };
    format!("{prefix}{}{prime}", c.position.value())
}

fn target(r: KernelRelationId, f: QlFamily, c: QlCoordinate, other: QlFamily) -> String {
    use KernelRelationId::*;
    let mut to = c;
    let mut to_family = f;
    match r {
        // These are structural identity references, not claims of an executed
        // nested traversal, source lookup, or parameterised CF invocation.
        PositionIdentity | ContextFrame | Nesting | Branching | SourceProvenance => {}
        FamilySamePosition => to_family = other,
        LensAnchor => to_family = QlFamily::L,
        CrossSamePosition => to.face = c.face.conjugate(),
        MirrorComplement => to.position = c.position.complement(),
        PairA | PairB | PairC | ConjugateInvarianceA | ConjugateInvarianceB
        | ConjugateInvarianceC => {
            let pair_family = match r {
                PairA | ConjugateInvarianceA => RelationFamily::A,
                PairB | ConjugateInvarianceB => RelationFamily::B,
                _ => RelationFamily::C,
            };
            let invariant = matches!(
                r,
                ConjugateInvarianceA | ConjugateInvarianceB | ConjugateInvarianceC
            );
            if invariant && c.face != QlFace::Conjugate {
                return "-".into();
            }
            let CanonicalCrossPass::D3 { pairs, .. } = canonical_cross_pass_d3(pair_family) else {
                unreachable!()
            };
            to.position = pairs
                .into_iter()
                .find_map(|[a, b]| {
                    if a.position == c.position {
                        Some(b.position)
                    } else if b.position == c.position {
                        Some(a.position)
                    } else {
                        None
                    }
                })
                .unwrap();
        }
        CrossTransform | CrossRequire | CrossComplete => {
            let kind = match r {
                CrossTransform => D2CrossPassKind::Transform,
                CrossRequire => D2CrossPassKind::Require,
                _ => D2CrossPassKind::Complete,
            };
            let CanonicalCrossPass::D2 { coordinates, .. } =
                canonical_cross_pass_d2(kind, c.position)
            else {
                unreachable!()
            };
            to.position = coordinates[1].position;
            to.face = c.face.conjugate();
        }
        PositionSuccessor => {
            if c.position.value() == 5 {
                return "-".into();
            }
            to.position = QlPosition::new(c.position.value() + 1).unwrap();
        }
        MobiusReturn => {
            if c.position.value() != 5 {
                return "-".into();
            }
            to.position = QlPosition::new(0).unwrap();
            to.face = c.face.conjugate();
        }
        VakCpf | VakCt | VakCp | VakCfp | VakCs => return "-".into(),
    }
    label(to_family, to)
}

fn check_row(c: &[&str]) {
    match c[0] {
        "contract" => assert_eq!(c, ["contract", HOLOGRAPHIC_KERNEL_CONTRACT_VERSION]),
        "family" => assert_eq!(c[2], family(c[1]).value().to_string()),
        "face" => assert_eq!(c[2], face(c[1]).kernel_value().to_string()),
        "relation" => {
            relation(c[1]);
        }
        // # is native tap-root bedrock, not a Rust sixfold position. Preserve
        // that explicit distinction rather than inventing a Rust position 255.
        "hash" => assert_eq!(c, ["hash", "#", "7", "255", "0"]),
        "hash-edge" => {
            let expected = if matches!(
                relation(c[1]),
                KernelRelationId::PositionIdentity | KernelRelationId::SourceProvenance
            ) {
                "#"
            } else {
                "-"
            };
            assert_eq!(c[2], expected);
        }
        "address" => {
            let coordinate = QlCoordinate::new(position(c[2]), face(c[3]));
            assert_eq!(c[4], label(family(c[1]), coordinate));
            assert_eq!(
                c[5],
                MusicalBasis::Chromatic.pitch_at(coordinate).to_string()
            );
        }
        "edge" => {
            let r = relation(c[1]);
            if r != KernelRelationId::FamilySamePosition {
                assert_eq!(c[5], "NONE");
            }
            let coordinate = QlCoordinate::new(position(c[3]), face(c[4]));
            assert_eq!(c[6], target(r, family(c[2]), coordinate, family(c[5])));
        }
        "mef" => {
            let lens = c[1].parse::<LensId>().unwrap();
            let local = position(c[2]);
            assert_eq!(
                c[3],
                MefRotation::new(lens, local)
                    .absolute_position()
                    .value()
                    .to_string()
            );
            assert_eq!(
                c[4],
                SublensRef::canonical(lens, local.value())
                    .unwrap()
                    .to_string()
            );
            let pitch = pitch_at_lens(
                MusicalBasis::Chromatic,
                lens,
                QlCoordinate::new(local, QlFace::Direct),
            );
            assert_eq!(c[5], pitch.to_string());
        }
        "cf" => {
            let frame = ContextFrameId::ALL
                .into_iter()
                .find(|f| f.code() == c[1])
                .unwrap();
            let coordinate = frame
                .canonical_selection()
                .at_lens(c[2].parse().unwrap())
                .coordinate();
            assert_eq!(c[3], coordinate.local_position().value().to_string());
            assert_eq!(c[4], coordinate.absolute_position().value().to_string());
            assert_eq!(
                c[5],
                match coordinate.unit_face() {
                    MefUnitFace::Name => "name",
                    MefUnitFace::Power => "power",
                }
            );
            assert_eq!(
                c[6],
                match coordinate.grain() {
                    MefGrain::InnerFour => "inner-four",
                    MefGrain::OuterTwo => "outer-two",
                }
            );
            assert_eq!(c[7], frame.expression());
        }
        "vak" => {
            let vak = VakFamily::ALL
                .into_iter()
                .find(|v| v.code() == c[1])
                .unwrap();
            assert_eq!(c[2], vak.value().to_string());
            assert_eq!(c[3], vak.relation_id().as_str());
            assert_eq!(c[4], vak.meaning());
            assert_eq!(c[5], vak.m0_handler_role());
        }
        "instruction" => {
            let vak = VakFamily::ALL
                .into_iter()
                .find(|v| v.value().to_string() == c[1])
                .unwrap();
            assert_eq!(c[2], "255");
            let instruction = VakInstruction::new(
                vak,
                255,
                c[3].parse().unwrap(),
                c[4].parse().unwrap(),
                face(c[5]) == QlFace::Conjugate,
            )
            .unwrap();
            assert_eq!(instruction.face().kernel_code(), c[5]);
            assert_eq!(instruction.relation_id(), vak.relation_id());
        }
        _ => panic!("unknown native parity row: {c:?}"),
    }
}

#[test]
fn native_c_matches_current_rust_foundation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = root.join("target/kernel-native-c");
    std::fs::create_dir_all(&out).unwrap();
    let binary = out.join(format!("foundation-probe-{}", std::process::id()));
    let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
    let compile = Command::new(cc)
        .current_dir(&root)
        .args([
            "-std=c11",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-pedantic",
            "-Ic/include",
            "migration/epi-kernel/k1-foundation-probe.c",
            "c/src/primitive.c",
            "c/src/holographic.c",
            "c/src/kernel.c",
            "-lm",
            "-o",
        ])
        .arg(&binary)
        .output()
        .expect("K1 parity requires a working C11 compiler (CC)");
    assert!(
        compile.status.success(),
        "{}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let run = Command::new(&binary).output().unwrap();
    let _ = std::fs::remove_file(&binary);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let text = String::from_utf8(run.stdout).unwrap();
    let mut counts = BTreeMap::<&str, usize>::new();
    let mut unique = BTreeSet::new();
    for line in text.lines() {
        assert!(unique.insert(line), "duplicate C outcome: {line}");
        let columns: Vec<_> = line.split('\t').collect();
        check_row(&columns);
        *counts.entry(columns[0]).or_default() += 1;
    }
    let expected: BTreeMap<_, _> = COUNTS
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let (kind, count) = l.split_once('\t').unwrap();
            (kind, count.parse::<usize>().unwrap())
        })
        .collect();
    assert_eq!(
        counts, expected,
        "native parity stream must be exhaustive, not a skipped subset"
    );
    std::fs::write(out.join("c-rust-parity.tsv"), &text).unwrap();
    println!("K1 current C/Rust parity: {} unique outcomes", unique.len());
}
