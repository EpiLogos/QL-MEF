//! Conformance for `ql.pole.coin-value-ground/v1` — the Stage-0 physical-pole
//! contract.
//!
//! The fixture is the machine-readable contract: every row is checked against
//! the executable types, and the executable laws are checked against the
//! fixture. The nucleotide value table is corrected once and for all (owner
//! ratification 2026-09-07): the C reference kernel, its dataset-backed
//! tables and this contract all carry the same parity-consistent table, and
//! the C side is pinned byte-for-byte by test.

use ql_core::{
    AngularGrid, ApertureIndex, Codon64, CoinSum, FibonacciGround, MatrixFamily, Nucleotide,
    PHYSICAL_POLE_FORM_CONTRACT_VERSION, POLE_COIN_CONTRACT_REF, ROTATIONAL_STATE_TOTAL,
    RotationalPose, monoid,
};

const FIXTURE: &str = include_str!("../../../fixtures/pole/physical-pole-coin-contract-v1.tsv");

/// Fixture spelling of the codon classification.
fn class_name(class: ql_core::CodonClass) -> &'static str {
    match class {
        ql_core::CodonClass::PerfectPalindromic => "perfect-palindromic",
        ql_core::CodonClass::ImperfectPalindromic => "imperfect-palindromic",
        ql_core::CodonClass::NonPalindromicNonDual => "non-palindromic-non-dual",
        ql_core::CodonClass::Dual => "dual",
    }
}

#[test]
fn contract_identity_is_versioned() {
    assert_eq!(PHYSICAL_POLE_FORM_CONTRACT_VERSION, "1.0.0");
    assert_eq!(POLE_COIN_CONTRACT_REF, "ql.pole.coin-value-ground/v1");
}

#[test]
fn fixture_nucleotide_table_matches_canonical_values() {
    let mut rows = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[0] != "nucleotide" {
            continue;
        }
        rows += 1;
        let bits = fields[2]
            .parse::<u8>()
            .unwrap_or_else(|_| panic!("nucleotide bits in row: {row}"));
        let nucleotide =
            Nucleotide::try_from(bits).unwrap_or_else(|_| panic!("nucleotide must parse: {row}"));
        assert_eq!(nucleotide.symbol().to_string(), fields[1], "{row}");
        assert_eq!(
            nucleotide.coin_value().value(),
            fields[3].parse::<u8>().unwrap(),
            "{row}"
        );
        assert_eq!(
            format!("{:?}", nucleotide.polarity()).to_lowercase(),
            fields[4],
            "{row}"
        );
        assert_eq!(
            format!("{:?}", nucleotide.mobility()).to_lowercase(),
            fields[5],
            "{row}"
        );
    }
    assert_eq!(rows, 4, "the fixture must carry the four-nucleotide table");
}

#[test]
fn fixture_carries_the_resolution_law() {
    let resolutions: Vec<(u8, u8)> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "resolution")
        .map(|fields| (fields[1].parse().unwrap(), fields[2].parse().unwrap()))
        .collect();
    assert_eq!(resolutions, vec![(6, 7), (9, 8)]);
    for (moving, resting) in resolutions {
        let value = CoinSum::new(moving).expect("resolution source in range");
        let resolved = value
            .resolve_moving()
            .unwrap_or_else(|| panic!("{moving} must be a moving value"));
        let expected_resting = if moving == 6 { moving + 1 } else { moving - 1 };
        assert_eq!(resolved.value(), resting);
        assert_eq!(
            resolved.value(),
            expected_resting,
            "resolution must be a single coin flip 2<->3"
        );
    }
}

#[test]
fn fixture_codon_rows_match_the_executable_64() {
    let mut rows = 0;
    let mut integral = 0u64;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[0] != "codon" {
            continue;
        }
        rows += 1;
        let codon = Codon64::new(fields[1].parse().expect("codon address"));
        let symbols: String = codon.nucleotides().iter().map(|n| n.symbol()).collect();
        assert_eq!(
            symbols,
            fields[2],
            "codon symbols at address {}",
            codon.address()
        );
        assert_eq!(
            codon.pair_xy().index(),
            fields[3].parse::<u8>().unwrap(),
            "{row}"
        );
        assert_eq!(
            codon.pair_yz().index(),
            fields[4].parse::<u8>().unwrap(),
            "{row}"
        );
        assert_eq!(
            codon.codon_sum(),
            fields[5].parse::<u16>().unwrap(),
            "{row}"
        );
        assert_eq!(class_name(codon.classify()), fields[6], "{row}");
        assert_eq!(
            codon.rotational_state_count(),
            fields[7].parse::<u8>().unwrap(),
            "{row}"
        );
        integral += codon.codon_sum() as u64;
    }
    assert_eq!(rows, 64, "the fixture must carry all 64 codon rows");
    assert_eq!(integral, 1440, "the 64 codon sums must total 4 × 360");
}

#[test]
fn fixture_counts_agree_with_the_executable_surface() {
    let counts: Vec<(String, usize)> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "counts")
        .map(|fields| (fields[1].to_string(), fields[2].parse().unwrap()))
        .collect();

    let mut edges = std::collections::HashSet::new();
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        for neighbour in codon.neighbours() {
            edges.insert((codon.address(), neighbour.address()));
        }
    }
    let expected: Vec<(String, usize)> = vec![
        ("adjacency".into(), edges.len()),
        ("poses".into(), ROTATIONAL_STATE_TOTAL),
        ("integral-total".into(), 1440),
        ("pair16".into(), 16),
    ];
    assert_eq!(counts, expected);
}

#[test]
fn fixture_aperture_rows_match_clock_law() {
    let mut rows = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[0] != "aperture" {
            continue;
        }
        rows += 1;
        let aperture = ApertureIndex::new(fields[1].parse().expect("aperture index"))
            .unwrap_or_else(|_| panic!("aperture must parse: {row}"));
        assert_eq!(
            aperture.orientation().reduced().0,
            fields[2].parse::<i32>().unwrap(),
            "{row}"
        );
        assert_eq!(
            aperture.reciprocal().index(),
            fields[3].parse::<u8>().unwrap(),
            "{row}"
        );
    }
    assert_eq!(
        rows, 16,
        "the fixture must carry the sixteen static apertures"
    );
}

#[test]
fn fixture_clock_rows_carry_sixteen_plus_one_and_the_sync_field() {
    let rows: Vec<Vec<&str>> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "clock")
        .collect();

    let ground = rows
        .iter()
        .find(|fields| fields[1] == "fibonacci-ground")
        .expect("fibonacci ground row");
    assert_eq!(ground[2], FibonacciGround::DIVISIONS.to_string());
    assert_eq!(ground[3], "60", "the base quantum must be 6°");

    for (name, grid) in [
        ("sync-base-fibre", AngularGrid::FibonacciBase),
        ("sync-base-aperture", AngularGrid::FibonacciBase),
        ("sync-fibre-aperture", AngularGrid::ElementalFibre),
    ] {
        let row = rows
            .iter()
            .find(|fields| fields[1] == name)
            .unwrap_or_else(|| panic!("missing sync row {name}"));
        let expected = match name {
            "sync-base-fibre" => {
                AngularGrid::FibonacciBase.closures_with(AngularGrid::ElementalFibre)
            }
            "sync-base-aperture" => grid.closures_with(AngularGrid::FormAperture),
            _ => grid.closures_with(AngularGrid::FormAperture),
        };
        assert_eq!(row[2].parse::<i32>().unwrap(), expected, "sync row {name}");
    }
}

#[test]
fn fixture_matrix_rows_bind_the_three_families_to_i_j_k() {
    let rows: Vec<Vec<&str>> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "matrix")
        .collect();
    assert_eq!(rows.len(), 3);
    for fields in rows {
        let family = match fields[1] {
            "complementary" => MatrixFamily::Complementary,
            "moving-resting" => MatrixFamily::MovingResting,
            "same-quality" => MatrixFamily::SameQuality,
            other => panic!("unknown matrix family: {other}"),
        };
        let axis = format!("{:?}", family.axis()).to_lowercase();
        assert_eq!(axis, fields[2], "axis binding for {fields:?}");
    }
}

#[test]
fn monoid_identities_stand_where_the_round_trips_touch_them() {
    // 9/8 = 3^2/2^3 — the epogdoon (yang^2/yin^3).
    assert_eq!(monoid::EPOGDOON_NUMERATOR, 9);
    assert_eq!(monoid::EPOGDOON_DENOMINATOR, 8);
    // 72 = 2^3 · 3^2 = 8 · 9 (yin^3 · yang^2); 64 = 2^6 (yin^6).
    assert_eq!(monoid::SEVENTY_TWO, 72);
    assert_eq!(monoid::SIXTY_FOUR, 64);
    // The DET trade: 72 · 8/9 = 64 — yang^2 bought for yin^3.
    assert_eq!(monoid::SEVENTY_TWO * 8 / 9, monoid::SIXTY_FOUR);
    // 2 + 3 = 5 (pentadic aperture); 2 · 3 = 6 (old yin).
    assert_eq!(monoid::PENTAD, 5);
    assert_eq!(monoid::SIX, 6);
}

#[test]
fn poses_are_orientations_of_the_same_form_not_display_flags() {
    // Every (codon, slot) pair is lawful exactly within the dataset-backed
    // state count, and the surface totals 472.
    let mut total = 0;
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        for slot in 0..codon.rotational_state_count() {
            let pose = RotationalPose::new(codon, slot).expect("slot in range");
            assert_eq!(pose.codon(), codon);
            total += 1;
        }
        assert!(RotationalPose::new(codon, codon.rotational_state_count()).is_err());
    }
    assert_eq!(total, 472);
}

#[test]
fn zero_one_exclusion_law_holds_on_the_value_type() {
    // The kernel anchor `# / 0/1 <-> 1/0` keeps 0/1 out of the M3 value
    // space: no coin-sum type admits 0 or 1, and the coin base is {2,3}.
    for excluded in [0u8, 1] {
        assert!(
            CoinSum::new(excluded).is_err(),
            "M3 value space must exclude {excluded}"
        );
    }
    for coin_face in [2u8, 3] {
        assert!(ql_core::CoinFace::from_value(coin_face).is_ok());
    }
    assert!(ql_core::CoinFace::from_value(0).is_err());
    assert!(ql_core::CoinFace::from_value(1).is_err());
}

/// The C dataset tables were regenerated with the canonical table (owner
/// ratification 2026-09-07): M3_PAIR_MATRIX (vendor m3.c) and the per-suit
/// integral constants now satisfy the coin law exactly. The C table is
/// parsed from source and every entry is checked: magnitudes follow the
/// value arithmetic; signs are the recorded dataset provenance (M3
/// unresolved item 2) preserved verbatim — no transcription, no drift.
#[test]
fn regenerated_c_dataset_matches_canonical_table() {
    let source = include_str!("../../../vendor/epi-kernel/reference/src/m3.c");
    let anchor = "M3_PAIR_MATRIX[16] = {";
    let start = source.find(anchor).expect("pair matrix anchor") + anchor.len();
    let end = source[start..].find("};").expect("pair matrix close");

    // Parse every `[index] = { sum, diff },` entry line.
    let mut c_table: [(i16, i16); 16] = [(0, 0); 16];
    let mut found = 0;
    for line in source[start..start + end].lines() {
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

    // Recorded dataset signs (M3 unresolved item 2: the class-stable
    // semantics of differenceValue stays open; the signs are provenance
    // preserved verbatim from the previously recorded table). 0 = homogeneous.
    let recorded_signs: [i16; 16] = [
        0,  // AA
        -1, // AT
        -1, // AC
        1,  // AG
        1,  // TA
        0,  // TT
        -1, // TC
        1,  // TG
        1,  // CA
        -1, // CT
        0,  // CC
        1,  // GC
        1,  // GA
        -1, // GT
        -1, // CG
        0,  // GG
    ];

    // Law: sum = v1+v2 always; |diff| = |v1-v2| for mixed pairs, 0 for
    // homogeneous; diff sign = the recorded dataset provenance.
    for a in Nucleotide::ALL {
        for b in Nucleotide::ALL {
            let index = ((a.bits() << 2) | b.bits()) as usize;
            let sum = (v_of(a) + v_of(b)) as i16;
            let magnitude = (v_of(a) as i16 - v_of(b) as i16).abs();
            let expected_diff = recorded_signs[index] * magnitude;
            assert_eq!(c_table[index].0, sum, "sum for pair index {index}");
            assert_eq!(
                c_table[index].1, expected_diff,
                "difference for pair index {index} ({a}{b})"
            );
        }
    }

    // Regenerated per-suit integrals: Cups 84, Wands 96, Pentacles 92,
    // Swords 88 — total 360 unchanged; the C constants were swapped with
    // the table in the same commit so m3_verify() passes.
    let mut suit_totals = [0u32; 4];
    for address in 0u8..64 {
        let codon = Codon64::new(address);
        suit_totals[codon.outer().bits() as usize] += codon.codon_sum() as u32;
    }
    let per_suit: Vec<u32> = suit_totals.iter().map(|raw| raw / 4).collect();
    assert_eq!(per_suit, vec![84, 96, 92, 88]);
}

fn v_of(n: Nucleotide) -> u8 {
    n.coin_value().value()
}

#[test]
fn fixture_tarot_rows_carry_gendering_and_dominant_subdominant() {
    // The suit alignments: gendering (suit polarity = nucleotide polarity)
    // and the dominant/subdominant grade (moving/old = dominant,
    // resting/young = subdominant). The correction of the C/G values is
    // what makes this law hold: Pentacles is the subdominant yin (young
    // yin 8) and Swords the subdominant yang (young yang 7).
    let rows: Vec<Vec<&str>> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "tarot")
        .collect();
    assert_eq!(rows.len(), 4, "the four suits");
    for fields in &rows {
        let nucleotide = match fields[2] {
            "A" => Nucleotide::A,
            "T" => Nucleotide::T,
            "C" => Nucleotide::C,
            _ => Nucleotide::G,
        };
        let element = match fields[3] {
            "water" => "Water",
            "fire" => "Fire",
            "earth" => "Earth",
            _ => "Air",
        };
        let value: u8 = fields[6].parse().unwrap();
        let integral: u32 = fields[7].parse().unwrap();
        // Value parity matches the suit gendering.
        let suit_yin = fields[4] == "yin";
        assert_eq!(value % 2 == 0, suit_yin, "parity/gendering for {fields:?}");
        // Grade follows mobility: moving = dominant, resting = subdominant.
        let grade = fields[5];
        assert_eq!(
            grade.starts_with("moving"),
            nucleotide.mobility() == ql_core::Mobility::Moving,
            "grade/mobility agreement for {fields:?}"
        );
        // The suit integral is the outer-family charge law: 4v + 60.
        assert_eq!(
            integral,
            4 * u32::from(value) + 60,
            "integral for {fields:?}"
        );
        // Total still 360 across the four suits.
        let _ = element;
    }
    let total: u32 = rows
        .iter()
        .map(|fields| fields[7].parse::<u32>().unwrap())
        .sum();
    assert_eq!(total, 360);
}

#[test]
fn fixture_tarot_court_rows_pin_the_dual_codon_gendering() {
    // Kernel court law (m3.h FR 2.3.16): yin suits carry their dual-codon
    // courts at Knight+King, yang suits at Princess(Page)+Queen — the
    // relational (two-codon) courts sit on the opposite gender within each
    // suit polarity. Nucleotide-keyed, invariant under the value table.
    let rows: Vec<Vec<&str>> = FIXTURE
        .lines()
        .filter(|row| !row.starts_with('#'))
        .map(|row| row.split('\t').collect::<Vec<_>>())
        .filter(|fields| fields[0] == "tarot-court")
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][1], "yin-suits");
    assert_eq!(rows[0][2], "knight+king");
    assert_eq!(rows[1][1], "yang-suits");
    assert_eq!(rows[1][2], "princess+queen");
}
