//! Conformance for `ql.pole.rotational-profile/v1` — the FR 2.3.14
//! rotational machinery port (capability M3-C13, branch M3-3).
//!
//! Three grounds are pinned against each other: the vendor C table
//! (`M3_ROTATIONAL_PROFILE[64]` + `M3_PAIR_MATRIX[16]`, parsed straight out
//! of `vendor/epi-kernel/reference/src/m3.c` — zero transcription), the
//! machine-readable fixture (`fixtures/pole/rotational-profile-v1.tsv`) and
//! the executable module (`src/pole/rotational.rs`). The generation law is
//! checked against the vendor algorithm exhaustively over all 64 codons,
//! and the pose layer (`ql.pole` 472-pose surface) is bridged to the profile.
//!
//! Until the wire-up (`pub mod rotational;` in `src/pole/mod.rs`) the module
//! compiles unmodified inside this test crate through pole-shaped shims that
//! re-export the very same `ql_core` types.

mod codon {
    pub use ql_core::{Codon64, PairIndex16};
}
mod nucleotide {
    pub use ql_core::Nucleotide;
}
#[path = "../src/pole/rotational.rs"]
mod rotational;

use ql_core::{Codon64, Nucleotide, PairIndex16, QlError, ROTATIONAL_STATE_TOTAL, RotationalPose};
use rotational::{
    NO_PAIR, NO_PAIRING, ROTATIONAL_TABLE_ENTRIES, RotationalPolarity, RotationalProfile,
    RotationalStateType, generate_rotational_states, pair_difference, pair_sum, rotational_profile,
    wc_anticodon,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/rotational-profile-v1.tsv");
const M3_C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m3.c");

fn nuc(letter: &str) -> u8 {
    match letter {
        "A" => 0,
        "T" => 1,
        "C" => 2,
        "G" => 3,
        other => panic!("vendor nucleotide letter {other}"),
    }
}

fn codon_bits(a: u8, b: u8, c: u8) -> u8 {
    (a << 4) | (b << 2) | c
}

fn class_name(class: ql_core::CodonClass) -> &'static str {
    match class {
        ql_core::CodonClass::PerfectPalindromic => "perfect-palindromic",
        ql_core::CodonClass::ImperfectPalindromic => "imperfect-palindromic",
        ql_core::CodonClass::NonPalindromicNonDual => "non-palindromic-non-dual",
        ql_core::CodonClass::Dual => "dual",
    }
}

fn type_name(state_type: RotationalStateType) -> &'static str {
    match state_type {
        RotationalStateType::NonDualInitiated => "non-dual-initiated",
        RotationalStateType::FullRotational => "full-rotational",
    }
}

fn optional(field: &str) -> Option<u8> {
    if field == "-" {
        None
    } else {
        Some(field.parse().unwrap_or_else(|_| panic!("index {field}")))
    }
}

/// The semantic identity of the profile contract.
#[test]
fn contract_identity_is_versioned() {
    assert_eq!(
        rotational::POLE_ROTATIONAL_PROFILE_REF,
        "ql.pole.rotational-profile/v1"
    );
}

/// The fixture rows are the executable profile: every row agrees field for
/// field, and the 64 rows classify, count and link exactly as the module
/// does.
#[test]
fn fixture_rows_match_the_executable_profile() {
    let mut rows = 0;
    let mut seven = 0;
    let mut eight = 0;
    let mut paired = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields[0], "profile");
        rows += 1;
        let codon = Codon64::new(fields[1].parse().expect("codon address"));
        let symbols: String = codon.nucleotides().iter().map(|n| n.symbol()).collect();
        assert_eq!(symbols, fields[2], "codon symbols at {row}");
        assert_eq!(class_name(codon.classify()), fields[3], "{row}");
        let profile = rotational_profile(codon);
        assert_eq!(
            profile.state_count(),
            fields[4].parse::<u8>().unwrap(),
            "{row}"
        );
        assert_eq!(type_name(profile.state_type()), fields[5], "{row}");
        let anchor_a = optional(fields[6]);
        let anchor_b = optional(fields[7]);
        let paired_codon = optional(fields[8]);
        assert_eq!(
            profile.anchor_pair_a().map(PairIndex16::index),
            anchor_a,
            "anchor a at {row}"
        );
        assert_eq!(
            profile.anchor_pair_b().map(PairIndex16::index),
            anchor_b,
            "anchor b at {row}"
        );
        assert_eq!(
            profile.paired_codon().map(Codon64::address),
            paired_codon,
            "paired codon at {row}"
        );
        match profile.state_count() {
            7 => seven += 1,
            _ => eight += 1,
        }
        if paired_codon.is_some() {
            paired += 1;
        }
    }
    assert_eq!(rows, 64, "the fixture must carry all 64 codon rows");
    assert_eq!((seven, eight, paired), (40, 24, 16));
}

/// The vendor table, parsed straight out of the C source (the `R7`/`R8P`/`R8`
/// macro calls of `M3_ROTATIONAL_PROFILE[64]`, `m3.c:405-425`, with the
/// `RCOD`/`RPAIR` arguments decoded textually), decoded through the module's
/// own raw-record decoder and compared entry for entry with the executable
/// table. This is the zero-transcription pin: any drift on either side fails.
#[test]
fn vendor_profile_table_is_pinned_entry_for_entry() {
    let anchor = "M3_ROTATIONAL_PROFILE[64] = {";
    let start = M3_C.find(anchor).expect("profile anchor") + anchor.len();
    let end = M3_C[start..].find("};").expect("profile close");
    let block = &M3_C[start..start + end];

    let mut raw: [(u8, bool, u8, u8, u8); 64] = [(0, false, 0, 0, 0); 64];
    let mut seen = [false; 64];
    for kind in ["R7(", "R8P(", "R8("] {
        for (at, _) in block.match_indices(kind) {
            let close = at + block[at..].find(')').expect("argument list close");
            let letters: Vec<u8> = block[at + kind.len()..close]
                .split(',')
                .map(|piece| nuc(piece.trim()))
                .collect();
            let codon = match kind {
                "R7(" => {
                    assert_eq!(letters.len(), 7, "R7 arguments at offset {at}");
                    raw[codon_bits(letters[0], letters[1], letters[2]) as usize] = (
                        7,
                        true,
                        (letters[3] << 2) | letters[4],
                        (letters[5] << 2) | letters[6],
                        NO_PAIRING,
                    );
                    codon_bits(letters[0], letters[1], letters[2])
                }
                "R8P(" => {
                    assert_eq!(letters.len(), 6, "R8P arguments at offset {at}");
                    raw[codon_bits(letters[0], letters[1], letters[2]) as usize] = (
                        8,
                        false,
                        NO_PAIR,
                        NO_PAIR,
                        codon_bits(letters[3], letters[4], letters[5]),
                    );
                    codon_bits(letters[0], letters[1], letters[2])
                }
                _ => {
                    assert_eq!(letters.len(), 3, "R8 arguments at offset {at}");
                    raw[codon_bits(letters[0], letters[1], letters[2]) as usize] =
                        (8, false, NO_PAIR, NO_PAIR, NO_PAIRING);
                    codon_bits(letters[0], letters[1], letters[2])
                }
            };
            assert!(!seen[codon as usize], "duplicate profile entry at {at}");
            seen[codon as usize] = true;
        }
    }
    assert!(seen.iter().all(|seen| *seen), "all 64 entries parsed");

    let mut seven = 0;
    let mut eight = 0;
    let mut anchored = 0;
    let mut paired = 0;
    for address in 0u8..64 {
        let (count, non_dual_initiated, a, b, p) = raw[address as usize];
        let state_type = if non_dual_initiated {
            RotationalStateType::NonDualInitiated
        } else {
            RotationalStateType::FullRotational
        };
        let vendor = RotationalProfile::try_from_raw(count, state_type, a, b, p)
            .unwrap_or_else(|err| panic!("vendor entry {address} must decode: {err:?}"));
        let executable = rotational_profile(Codon64::new(address));
        assert_eq!(vendor, executable, "profile entry {address}");
        match executable.state_count() {
            7 => seven += 1,
            _ => eight += 1,
        }
        if executable.anchor_pair_a().is_some() {
            anchored += 1;
        }
        if executable.paired_codon().is_some() {
            paired += 1;
        }
    }
    // The m3_verify counts (m3.c:785-790): 40 seven-state, 24 eight-state,
    // 40 anchored, 16 pair links.
    assert_eq!((seven, eight, anchored, paired), (40, 24, 40, 16));
}

/// The pair S/D accessors agree with the regenerated `M3_PAIR_MATRIX`
/// (`m3.c:32-56`), parsed straight out of the C source — the same
/// zero-transcription pattern as the `pole_coin_contract` conformance. Sum =
/// v1+v2, |difference| = |v1−v2| (0 homogeneous), signs = recorded dataset
/// provenance.
#[test]
fn vendor_pair_matrix_matches_the_sd_accessors() {
    let anchor = "M3_PAIR_MATRIX[16] = {";
    let start = M3_C.find(anchor).expect("pair matrix anchor") + anchor.len();
    let end = M3_C[start..].find("};").expect("pair matrix close");

    let mut c_table: [(i16, i16); 16] = [(0, 0); 16];
    let mut found = 0;
    for line in M3_C[start..start + end].lines() {
        let trimmed = line.trim();
        if !(trimmed.starts_with('[') && trimmed.contains('{') && trimmed.contains('}')) {
            continue;
        }
        let close_bracket = trimmed.find(']').expect("index close");
        let index: usize = trimmed[1..close_bracket]
            .trim()
            .parse()
            .expect("pair index");
        let open_brace = trimmed.find('{').expect("entry open");
        let close_brace = trimmed.find('}').expect("entry close");
        let numbers: Vec<i16> = trimmed[open_brace + 1..close_brace]
            .split(',')
            .filter_map(|piece| piece.trim().parse().ok())
            .collect();
        assert_eq!(numbers.len(), 2, "two values per entry: {trimmed}");
        c_table[index] = (numbers[0], numbers[1]);
        found += 1;
    }
    assert_eq!(found, 16, "the C table must carry all 16 entries");

    for a in Nucleotide::ALL {
        for b in Nucleotide::ALL {
            let pair = PairIndex16::from_nucleotides(a, b);
            let (sum, diff) = c_table[pair.index() as usize];
            assert_eq!(pair_sum(pair), sum, "sum of {a}{b}");
            assert_eq!(pair_difference(pair), diff, "difference of {a}{b}");
            // The magnitude law under the canonical table {A=6,T=9,C=8,G=7}.
            let v1 = a.coin_value().value() as i16;
            let v2 = b.coin_value().value() as i16;
            assert_eq!(sum, v1 + v2, "sum law of {a}{b}");
            let magnitude = if a == b { 0 } else { (v1 - v2).abs() };
            assert_eq!(diff.abs(), magnitude, "difference magnitude of {a}{b}");
        }
    }
    // The m3_verify pins (m3.c:676-687).
    assert_eq!(c_table[0], (12, 0), "AA — min sum");
    assert_eq!(c_table[5], (18, 0), "TT — max sum");
    for index in [1, 4, 11, 14] {
        assert_eq!(c_table[index].0, 15, "Watson-Crick sum at {index}");
    }
}

/// One full expected ranked sweep: (valence, bipolar, value, resulting codon,
/// pair1, pair2) per slot, taken from an independent replay of the vendor
/// algorithm.
#[test]
fn generation_law_matches_the_vendor_algorithm_spot_wise() {
    fn check(codon: &str, expected: [(bool, bool, i16, &str, &str, &str); 8]) {
        let address = codon
            .bytes()
            .fold(0u8, |acc, b| (acc << 2) | nuc(&(b as char).to_string()));
        let ranked = generate_rotational_states(Codon64::new(address));
        for (slot, (negative, non_dual, value, result, pair1, pair2)) in expected.iter().enumerate()
        {
            let candidate = &ranked[slot];
            assert_eq!(
                candidate.polarity,
                if *negative {
                    RotationalPolarity::Negative
                } else {
                    RotationalPolarity::Positive
                },
                "{codon} slot {slot} valence"
            );
            assert_eq!(candidate.is_non_dual, *non_dual, "{codon} slot {slot} flag");
            assert_eq!(
                candidate.rotational_value, *value,
                "{codon} slot {slot} value"
            );
            assert_eq!(
                candidate.resulting_codon.to_string(),
                *result,
                "{codon} slot {slot} codon"
            );
            assert_eq!(
                format!("{}{}", candidate.pair1.first(), candidate.pair1.second()),
                *pair1,
                "{codon} slot {slot} pair1"
            );
            assert_eq!(
                format!("{}{}", candidate.pair2.first(), candidate.pair2.second()),
                *pair2,
                "{codon} slot {slot} pair2"
            );
            assert_eq!(candidate.rotation_slot, slot as u8);
            assert_eq!(candidate.rotation_degrees, 45 * slot as u16);
        }
    }

    check(
        "AAA",
        [
            (false, false, 9, "ATA", "AT", "AA"),
            (false, false, 10, "ACA", "AC", "AA"),
            (true, true, 12, "AAA", "AA", "AA"),
            (false, true, 12, "AAA", "AA", "AA"),
            (true, false, 13, "AGA", "AA", "GA"),
            (false, false, 13, "AGA", "AG", "AA"),
            (true, false, 14, "ACA", "AA", "CA"),
            (true, false, 15, "ATA", "AA", "TA"),
        ],
    );
    check(
        "ATA",
        [
            (false, false, 12, "ATA", "AT", "TA"),
            (false, false, 13, "ACA", "AC", "TA"),
            (true, false, 15, "AAA", "AT", "AA"),
            (false, false, 15, "AAA", "AA", "TA"),
            (true, false, 16, "AGA", "AT", "GA"),
            (false, false, 16, "AGA", "AG", "TA"),
            (true, false, 17, "ACA", "AT", "CA"),
            (true, false, 18, "ATA", "AT", "TA"),
        ],
    );
    check(
        "ATG",
        [
            (false, false, 13, "ATG", "AT", "TG"),
            (false, false, 14, "ACG", "AC", "TG"),
            (true, false, 15, "AGG", "AT", "GG"),
            (true, false, 16, "AAG", "AT", "AG"),
            (true, false, 16, "ACG", "AT", "CG"),
            (false, false, 16, "AAG", "AA", "TG"),
            (true, false, 17, "ATG", "AT", "TG"),
            (false, false, 17, "AGG", "AG", "TG"),
        ],
    );
    check(
        "TCG",
        [
            (false, false, 14, "TCG", "TC", "CG"),
            (false, false, 15, "TTG", "TT", "CG"),
            (true, false, 17, "TGG", "TC", "GG"),
            (false, false, 17, "TGG", "TG", "CG"),
            (true, false, 18, "TAG", "TC", "AG"),
            (true, false, 18, "TCG", "TC", "CG"),
            (false, false, 18, "TAG", "TA", "CG"),
            (true, false, 19, "TTG", "TC", "TG"),
        ],
    );
}

/// The generation law is total, ranked ascending (slot 0 = the lowest
/// rotational value, negative valence before positive within equal values)
/// and always sweeps the middle site over the whole alphabet.
#[test]
fn generation_law_is_total_ranked_and_sweeping() {
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let ranked = generate_rotational_states(codon);
        assert_eq!(ranked.len(), ROTATIONAL_TABLE_ENTRIES);

        let mut slots: Vec<u8> = ranked.iter().map(|c| c.rotation_slot).collect();
        slots.sort_unstable();
        assert_eq!(
            slots,
            (0..8).collect::<Vec<_>>(),
            "slot permutation at {codon}"
        );
        for candidate in &ranked {
            assert_eq!(
                candidate.rotation_degrees,
                45 * u16::from(candidate.rotation_slot),
                "degrees at {codon}"
            );
        }
        for window in ranked.windows(2) {
            assert!(
                window[0].rotational_value <= window[1].rotational_value,
                "ascending value at {codon}"
            );
            if window[0].rotational_value == window[1].rotational_value {
                let rank = |p: RotationalPolarity| match p {
                    RotationalPolarity::Negative => 0,
                    RotationalPolarity::Positive => 1,
                };
                assert!(
                    rank(window[0].polarity) <= rank(window[1].polarity),
                    "negative precedes positive within equal values at {codon}"
                );
            }
        }

        // The middle-site sweep: resulting codons are (X, swept, Z).
        let mut middles = std::collections::HashSet::new();
        for candidate in &ranked {
            assert_eq!(candidate.resulting_codon.outer(), codon.outer());
            assert_eq!(candidate.resulting_codon.inner(), codon.inner());
            middles.insert(candidate.resulting_codon.middle().bits());
        }
        assert_eq!(middles.len(), 4, "full alphabet sweep at {codon}");

        // The hinge: both valences emit the input codon at swept == middle.
        let hinge: Vec<_> = ranked
            .iter()
            .filter(|c| c.resulting_codon == codon)
            .collect();
        assert_eq!(hinge.len(), 2, "two hinge candidates at {codon}");
        assert!(
            hinge
                .iter()
                .any(|c| c.polarity == RotationalPolarity::Negative)
        );
        assert!(
            hinge
                .iter()
                .any(|c| c.polarity == RotationalPolarity::Positive)
        );
        for candidate in &hinge {
            assert_eq!(candidate.pair1, codon.pair_xy(), "hinge pairs at {codon}");
            assert_eq!(candidate.pair2, codon.pair_yz(), "hinge pairs at {codon}");
        }

        // Bipolar flags: negative fires iff Y == Z (at swept == X), positive
        // fires iff X == Y (at swept == Z).
        let negative_flags = ranked
            .iter()
            .filter(|c| c.polarity == RotationalPolarity::Negative && c.is_non_dual)
            .count();
        let positive_flags = ranked
            .iter()
            .filter(|c| c.polarity == RotationalPolarity::Positive && c.is_non_dual)
            .count();
        assert_eq!(
            (negative_flags == 1),
            codon.middle() == codon.inner(),
            "negative bipolar flag at {codon}"
        );
        assert_eq!(
            (positive_flags == 1),
            codon.outer() == codon.middle(),
            "positive bipolar flag at {codon}"
        );
    }
}

/// THE KEY DISCOVERY, TESTED AND REFUTED AS STATED: the eight candidates do
/// NOT span 7 (non-dual) or 8 (dual) distinct resulting codons. Both
/// valences land on `(X, swept, Z)`, so every codon — dual or not — spans
/// exactly FOUR distinct resulting codons × two valences = eight distinct
/// (codon, valence) candidates. The 7/8 profile split is dataset provenance
/// (the profile table), not a distinctness count of the generation. The one
/// literal collapse is the perfect-palindrome bipolar state, where both
/// valences emit the identical candidate record.
#[test]
fn discovery_four_distinct_codons_not_seven_or_eight() {
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let ranked = generate_rotational_states(codon);
        let distinct_codons: std::collections::HashSet<_> =
            ranked.iter().map(|c| c.resulting_codon).collect();
        let distinct_valenced: std::collections::HashSet<_> = ranked
            .iter()
            .map(|c| (c.resulting_codon, c.polarity))
            .collect();
        assert_eq!(distinct_codons.len(), 4, "distinct codons at {codon}");
        assert_eq!(distinct_valenced.len(), 8, "distinct valenced at {codon}");

        if codon.is_palindromic() && codon.outer() == codon.middle() {
            // Perfect palindrome: the bipolar hinge candidate is emitted
            // identically by both valences — 7 distinct candidate records.
            let records: std::collections::HashSet<_> = ranked
                .iter()
                .map(|c| (c.pair1, c.pair2, c.resulting_codon, c.rotational_value))
                .collect();
            assert_eq!(records.len(), 7, "bipolar collapse at {codon}");
        }
    }
    // Counterexample pinned: ATA (imperfect palindrome, non-dual, 7 lawful
    // states) still shows 8 distinct candidate records — the profile count
    // is not a distinctness count anywhere but the perfect palindromes.
    let ata = generate_rotational_states(Codon64::new(0x04));
    let records: std::collections::HashSet<_> = ata
        .iter()
        .map(|c| (c.pair1, c.pair2, c.resulting_codon, c.rotational_value))
        .collect();
    assert_eq!(records.len(), 8);
}

/// The court-reflection law of the profile is the middle<->inner swap —
/// NOT the Watson-Crick anticodon. All 16 `R8P` links are involutive
/// Y↔Z reflections, and `wc_anticodon` agrees with none of them (0/16 —
/// a documented discrepancy between the two reflection laws: the WC
/// anticodon complements every site and reverses outer/inner, the court
/// link permutes the two inner sites of the dual codon). The 8 unlinked
/// dual codons are closed under the swap.
#[test]
fn court_reflection_is_the_swap_not_the_wc_anticodon() {
    let mut links = 0;
    let mut agreements = 0;
    let mut unpaired = Vec::new();
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let profile = rotational_profile(codon);
        let swap = Codon64::new(
            (codon.outer().bits() << 4) | (codon.inner().bits() << 2) | codon.middle().bits(),
        );
        if let Some(paired) = profile.paired_codon() {
            links += 1;
            assert_eq!(paired, swap, "court link is the Y<->Z swap at {codon}");
            let partner = rotational_profile(paired);
            assert_eq!(
                partner.paired_codon(),
                Some(codon),
                "court link is involutive at {codon}"
            );
            assert_eq!(partner.state_count(), 8);
            if wc_anticodon(codon) == paired {
                agreements += 1;
            }
        } else if codon.classify() == ql_core::CodonClass::Dual {
            unpaired.push(swap);
        }
    }
    assert_eq!(links, 16, "the 16 court-reflection links");
    assert_eq!(agreements, 0, "wc_anticodon never equals the court link");
    for swap in unpaired {
        let profile = rotational_profile(swap);
        assert!(
            profile.paired_codon().is_none(),
            "unpaired closure at {swap}"
        );
    }
}

/// The pose bridge: the profile state count IS the classifier count, the
/// ranked candidates ARE the pose content of slots 0..state_count, and the
/// lawful surface totals 472. (The "distinct candidates == state_count"
/// reading is refuted by `discovery_four_distinct_codons_not_seven_or_eight`;
/// the profile gates how many of the eight ranked orientations the codon
/// lawfully carries.)
#[test]
fn profile_gates_the_pose_surface() {
    let mut lawful = 0;
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        let profile = rotational_profile(codon);
        assert_eq!(
            profile.state_count(),
            codon.rotational_state_count(),
            "profile count at {codon}"
        );
        let ranked = generate_rotational_states(codon);
        for slot in 0..ROTATIONAL_TABLE_ENTRIES as u8 {
            let pose = RotationalPose::new(codon, slot);
            if slot < profile.state_count() {
                let pose = pose.unwrap_or_else(|err| panic!("slot {slot} at {codon}: {err:?}"));
                assert_eq!(pose.codon(), codon);
                assert_eq!(ranked[slot as usize].rotation_slot, slot);
                assert_eq!(ranked[slot as usize].rotation_degrees, 45 * u16::from(slot));
                lawful += 1;
            } else {
                assert!(pose.is_err(), "slot {slot} beyond the count at {codon}");
            }
        }
    }
    assert_eq!(lawful, 472);
    assert_eq!(lawful, ROTATIONAL_STATE_TOTAL);
    assert_eq!(ql_core::all_poses().count(), ROTATIONAL_STATE_TOTAL);
}
