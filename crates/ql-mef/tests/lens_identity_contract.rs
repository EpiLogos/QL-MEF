//! Lens identity and relations survive different walks and native encodings.
//! These are compatibility witnesses, not a new registry or prescribed walk.
//! See docs/kernel-rebuild/LENS-IDENTITY-AND-ENCODINGS.md.
use std::collections::BTreeMap;

use ql_mef::m1::traverse_json;
use ql_mef::m1_engine::{Basis, EngineConfig, M1Engine};
use ql_mef::m2::{Reading72, Register72};
use ql_mef::{LensFace, LensId, LensRef, SublensRef};
use serde_json::{Value, json};

// Explicit witnesses keep a change to ALL from silently changing the expected
// protocol meaning as well. Columns: M1 slot, identity, retained M2 MEF row.
const ENCODINGS: [(u8, LensId, u8); 12] = [
    (0, LensId::L0, 0),
    (1, LensId::L0Prime, 6),
    (2, LensId::L1, 1),
    (3, LensId::L1Prime, 7),
    (4, LensId::L2, 2),
    (5, LensId::L2Prime, 8),
    (6, LensId::L3, 3),
    (7, LensId::L3Prime, 9),
    (8, LensId::L4, 4),
    (9, LensId::L4Prime, 10),
    (10, LensId::L5, 5),
    (11, LensId::L5Prime, 11),
];

#[test]
fn positional_relations_are_distinct_involutions_not_successive_list_entries() {
    for (_, lens, _) in ENCODINGS {
        let twin = lens.conjugate_twin();
        let complement = lens.same_face_complement();
        let mobius = lens.mobius_partner();
        assert_eq!(twin.index(), lens.index());
        assert_ne!(twin.face(), lens.face());
        assert_eq!(complement.index(), 5 - lens.index());
        assert_eq!(complement.face(), lens.face());
        assert_eq!(mobius.index(), complement.index());
        assert_eq!(mobius.face(), twin.face());
        assert_eq!(twin.conjugate_twin(), lens);
        assert_eq!(complement.same_face_complement(), lens);
        assert_eq!(mobius.mobius_partner(), lens);
        assert_eq!(twin.same_face_complement(), mobius);
        assert_eq!(complement.conjugate_twin(), mobius);
        assert_ne!(twin, complement);
        assert_ne!(complement, mobius);
        assert_ne!(twin, mobius);
        for related in [twin, complement, mobius] {
            assert_eq!(related.square(), lens.square());
        }
        let reference = LensRef::canonical(lens);
        assert_eq!(reference.to_string().parse::<LensRef>().unwrap(), reference);
        assert_eq!(lens.code().parse::<LensId>().unwrap(), lens);
    }
}

#[test]
fn changing_the_walk_does_not_change_the_relational_field() {
    let interleaved = LensId::ALL;
    let grouped = [
        LensId::L0,
        LensId::L1,
        LensId::L2,
        LensId::L3,
        LensId::L4,
        LensId::L5,
        LensId::L0Prime,
        LensId::L1Prime,
        LensId::L2Prime,
        LensId::L3Prime,
        LensId::L4Prime,
        LensId::L5Prime,
    ];
    // A deliberately different visit order; its rank must never become a lens ID.
    let another_walk = [
        LensId::L4Prime,
        LensId::L1,
        LensId::L0Prime,
        LensId::L5,
        LensId::L2Prime,
        LensId::L3,
        LensId::L1Prime,
        LensId::L4,
        LensId::L5Prime,
        LensId::L0,
        LensId::L3Prime,
        LensId::L2,
    ];
    let mut fields = Vec::new();
    for walk in [interleaved, grouped, another_walk] {
        let mut field = BTreeMap::new();
        for lens in walk {
            for local_position in 0..6 {
                let sublens = SublensRef::canonical(lens, local_position).unwrap();
                let received = Reading72::from_sublens(sublens).mef_sublens().unwrap();
                assert_eq!(received, sublens);
                let received_lens = received.lens().lens();
                let previous = field.insert(
                    received.to_string(),
                    (
                        received_lens.index(),
                        received_lens.face(),
                        received_lens.conjugate_twin().code(),
                        received_lens.same_face_complement().code(),
                        received_lens.mobius_partner().code(),
                        received.position().value(),
                    ),
                );
                assert!(previous.is_none());
            }
        }
        assert_eq!(field.len(), 72);
        fields.push(field);
    }
    assert_eq!(fields[0], fields[1]);
    assert_eq!(fields[0], fields[2]);
}

#[test]
fn native_encodings_round_trip_identity_and_local_position_not_bare_integers() {
    for (m1_slot, lens, m2_row) in ENCODINGS {
        assert_eq!(LensId::ALL[usize::from(m1_slot)], lens);
        assert_eq!(lens.slot(), m1_slot);
        assert_eq!(lens.index(), m2_row % 6);
        assert_eq!(
            lens.face(),
            if m2_row < 6 {
                LensFace::Day
            } else {
                LensFace::Night
            }
        );
        for local_position in 0..6 {
            let expected = SublensRef::canonical(lens, local_position).unwrap();
            let encoded = Reading72::from_sublens(expected);
            assert_eq!(encoded.register(), Register72::Mef);
            assert_eq!(encoded.axes(), [m2_row, local_position, 0, 0]);
            let received = Reading72::from_axes(Register72::Mef, m2_row, local_position, 0, 0)
                .unwrap()
                .mef_sublens()
                .unwrap();
            assert_eq!(received, expected);
        }
    }
    // Both are legal encodings of different identities: copying 1 is not a bridge.
    let from_m1 = SublensRef::canonical(LensId::ALL[1], 0).unwrap();
    let from_m2 = Reading72::from_axes(Register72::Mef, 1, 0, 0, 0)
        .unwrap()
        .mef_sublens()
        .unwrap();
    assert_eq!(from_m1.lens().lens(), LensId::L0Prime);
    assert_eq!(from_m2.lens().lens(), LensId::L1);
    assert_ne!(from_m1, from_m2);
    assert_eq!(Reading72::from_sublens(from_m1).axes()[0], 6);
    assert!(Reading72::new(Register72::Mef, 72).is_err());
    assert!(Reading72::from_axes(Register72::Mef, 12, 0, 0, 0).is_err());
    assert!(Reading72::from_axes(Register72::Mef, 0, 6, 0, 0).is_err());
    assert!(
        Reading72::new(Register72::Tattva, 0)
            .unwrap()
            .mef_sublens()
            .is_err()
    );
}

#[test]
fn m1_slot_one_intentionally_selects_l0_prime_independently_of_clock_and_basis() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m1-engine-v1.request.json"
    ))
    .unwrap();
    let original: EngineConfig = serde_json::from_value(fixture["config"].clone()).unwrap();
    for basis in [Basis::Chromatic, Basis::Fifths] {
        for tick12 in [0, 7] {
            for cycle in ["0", "1"] {
                let mut cfg = original.clone();
                cfg.lens12 = 1;
                cfg.tick12 = tick12;
                cfg.cycle = cycle.into();
                cfg.basis = basis;
                let owner = M1Engine::new(cfg.clone()).unwrap();
                let snapshot = owner.snapshot().unwrap();
                assert_eq!(snapshot["music"]["lens"], "L0'");
                assert_eq!(snapshot["config"]["lens12"], 1);
                let traversal: Value = serde_json::from_str(&traverse_json(&json!({
                    "schema":"ql.m1.traversal/v1",
                    "source":{"position6":2,"phase":1},
                    "target":{"position6":3,"phase":1},
                    "pointer":{"source_ref":"fixture:2-prime","target_ref":"fixture:3-prime",
                        "relation_ref":"fixture:walk","relation_roles":["A","C"]},
                    "participation":"both", "family":cfg.family, "row12":cfg.row12,
                    "col12":cfg.col12, "cycle":cfg.cycle, "tick12":cfg.tick12,
                    "basis":cfg.basis, "lens12":cfg.lens12
                }).to_string()).unwrap()).unwrap();
                assert_eq!(traversal["lens"], "L0'");
                assert_eq!(traversal["cell"], snapshot["cell"]);
                assert_eq!(owner.snapshot().unwrap(), snapshot);
            }
        }
    }
}
