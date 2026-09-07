//! Conformance for the ported M3 RES matrix (`ql.pole.fold-state/v1`, the
//! Same-Quality k axis) — the dataset-structural resonance table copied
//! VERBATIM from `vendor/epi-kernel/reference/src/m3.c` (`M3_RES_MATRIX`,
//! 56 admitted entries + 8 evolutionary gaps).
//!
//! Two facts fully determine the table against the C literal and are pinned
//! here: the 8 gap addresses are exactly the recorded ones, and every
//! admitted entry is the recorded (identity) partner. Together with the
//! fold-state behaviour tests this is the executable conformance of the
//! port; the gap set is also pinned to its trigram-row structure, which is
//! a property of the six-bit address alone and therefore independent of the
//! nucleotide value table.

use ql_core::{
    ApertureIndex, ApplyOutcome, Codon64, FoldState, M3_RES_MATRIX, MatrixAxis, MatrixFamily,
    RES_ADMITTED_COUNT, RES_GAP_ADDRESSES, RESONANCE_GAP, is_resonance_gap, resonance_entry,
};

/// The exact trigram crossings the kernel comments record for the 8 gaps:
/// `(address, upper trigram bits, lower trigram bits, crossing)`, where the
/// hexagram id is `upper<<3 | lower` over the same six bits.
const GAP_CROSSINGS: [(u8, u8, u8, &str); 8] = [
    (0x05, 0b000, 0b101, "Kun/Li"),
    (0x15, 0b010, 0b101, "Kan/Li"),
    (0x1A, 0b011, 0b010, "Dui/Kan"),
    (0x22, 0b100, 0b010, "Gen/Kan"),
    (0x2A, 0b101, 0b010, "Li/Kan"),
    (0x35, 0b110, 0b101, "Xun/Li"),
    (0x3A, 0b111, 0b010, "Qian/Kan"),
    (0x3D, 0b111, 0b101, "Qian/Li"),
];

#[test]
fn the_eight_gaps_are_exactly_the_recorded_addresses() {
    let gaps: Vec<u8> = (0u8..64)
        .filter(|a| M3_RES_MATRIX[*a as usize] == RESONANCE_GAP)
        .collect();
    assert_eq!(
        gaps, RES_GAP_ADDRESSES,
        "gap set differs from the kernel record"
    );
    assert_eq!(gaps.len(), 8);
    assert_eq!(64 - gaps.len(), RES_ADMITTED_COUNT, "56 admitted + 8 gaps");
    for address in 0u8..64 {
        assert_eq!(
            is_resonance_gap(address),
            gaps.contains(&address),
            "gap predicate at {address:#04x}"
        );
        assert_eq!(
            resonance_entry(address).is_none(),
            gaps.contains(&address),
            "lookup optionality at {address:#04x}"
        );
    }
}

#[test]
fn gap_addresses_decode_to_the_recorded_trigram_crossings() {
    for (address, upper, lower, crossing) in GAP_CROSSINGS {
        assert!(RES_GAP_ADDRESSES.contains(&address), "{crossing} recorded");
        // The hexagram id IS the address bits: upper<<3 | lower.
        assert_eq!(
            Codon64::new(address).hexagram_id(),
            address,
            "hexagram id of {crossing}"
        );
        assert_eq!(address >> 3, upper, "{crossing}: upper trigram bits");
        assert_eq!(address & 0x07, lower, "{crossing}: lower trigram bits");
    }
    // Structural law of the recorded gap set: every gap has a Kan (Water,
    // 010) or Li (Fire, 101) LOWER trigram, and Qian (111) is the only
    // upper trigram appearing twice.
    for (address, _, lower, crossing) in GAP_CROSSINGS {
        assert!(
            lower == 0b010 || lower == 0b101,
            "{crossing}: gap {address:#04x} must sit on a Water/Fire lower trigram"
        );
    }
    let uppers: Vec<u8> = GAP_CROSSINGS.iter().map(|(_, u, _, _)| *u).collect();
    assert_eq!(
        uppers.iter().filter(|u| **u == 0b111).count(),
        2,
        "Qian doubled"
    );
}

#[test]
fn the_zhen_row_is_entirely_admitted() {
    // Row 1 (upper=Zhen=001): addresses 0x08..=0x0F — the kernel comment
    // records "all valid"; this is the one fully-admitted trigram row.
    for address in 0x08..=0x0F {
        assert!(!is_resonance_gap(address), "Zhen row admits {address:#04x}");
    }
}

#[test]
fn admitted_entries_are_the_recorded_identity_partners() {
    // The recorded data: every admitted address is its own resonance
    // partner. Together with the exact gap set this reproduces the C
    // literal byte for byte.
    let mut admitted = 0usize;
    for address in 0u8..64 {
        match resonance_entry(address) {
            Some(partner) => {
                admitted += 1;
                assert_eq!(partner, address, "recorded partner of {address:#04x}");
                // Closure: an admitted entry never lands on a gap.
                assert!(
                    !is_resonance_gap(partner),
                    "partner of {address:#04x} admitted"
                );
            }
            None => assert_eq!(M3_RES_MATRIX[address as usize], RESONANCE_GAP),
        }
    }
    assert_eq!(admitted, RES_ADMITTED_COUNT);
}

#[test]
fn iterated_lookup_stays_defined_and_fixed_on_the_recorded_data() {
    // The RES matrix is NOT claimed to be an involution as a semantic law;
    // what the recorded data shows is tested: for every admitted address x
    // the image res[x] is itself admitted, so res[res[x]] is well-defined —
    // and because the recorded entries are the identity, res[res[x]] == x.
    for address in 0u8..64 {
        if let Some(first) = resonance_entry(address) {
            let second = resonance_entry(first)
                .unwrap_or_else(|| panic!("image of {address:#04x} must be resolvable"));
            assert_eq!(second, address, "iterated lookup at {address:#04x}");
        }
    }
}

#[test]
fn fold_state_same_quality_is_provisional_at_gaps_and_applied_elsewhere() {
    let aperture = ApertureIndex::new(6).unwrap();
    for address in 0u8..64 {
        let state = FoldState::from_codon(Codon64::new(address), aperture, 21);
        // Not an error in either branch: the k axis is executable, the gap
        // is a typed provisional outcome (kernel STATUS_PROVISIONAL).
        let outcome = state.apply_matrix(MatrixFamily::SameQuality).unwrap();
        if is_resonance_gap(address) {
            assert_eq!(outcome, ApplyOutcome::Provisional, "gap {address:#04x}");
        } else {
            let next = outcome
                .applied()
                .unwrap_or_else(|| panic!("admitted {address:#04x} must apply"));
            assert_eq!(next.codon().address(), address);
            assert_eq!(next.active_matrix_axis(), MatrixAxis::K);
            assert_eq!(next.aperture16(), aperture);
            assert_eq!(next.fibonacci_phase60(), 21);
        }
    }
    // The one gap of the Kun row, by name: Kun/Li 0x05 stays provisional.
    assert_eq!(
        FoldState::from_codon(Codon64::new(0x05), aperture, 0)
            .apply_matrix(MatrixFamily::SameQuality)
            .unwrap(),
        ApplyOutcome::Provisional
    );
}
