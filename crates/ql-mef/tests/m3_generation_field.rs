//! The kernel's generation law reproduces the 192 cells of the map's M3-3-2
//! layer exactly, as carried by `fixtures/kernel/m3-generation-field-v1.json`
//! (generated from the live map by `migration/bimba-map/generate_m3_generation_cells.py`).

use ql_core::{CodonClass, MatrixFamily, Trigram, generate};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/kernel/m3-generation-field-v1.json");

const TRIGRAM_NAMES: [&str; 8] = ["Qian", "Kun", "Zhen", "Xun", "Kan", "Li", "Gen", "Dui"];

/// Trigram name -> kernel line pattern (the LUT is id-indexed by these names).
fn pattern(name: &str) -> u8 {
    let id = TRIGRAM_NAMES
        .iter()
        .position(|n| *n == name)
        .unwrap_or_else(|| panic!("trigram {name}"));
    Trigram::LUT[id].binary
}

fn family(matrix: u64) -> MatrixFamily {
    MatrixFamily::ALL[matrix as usize - 1]
}

fn class_name(class: CodonClass) -> &'static str {
    match class {
        CodonClass::PerfectPalindromic => "perfect_palindromic",
        CodonClass::ImperfectPalindromic => "imperfect_palindromic",
        CodonClass::NonPalindromicNonDual => "non_palindromic_non_dual",
        CodonClass::Dual => "dual",
    }
}

fn text(cell: &Value, key: &str) -> String {
    cell[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key}"))
        .to_string()
}

#[test]
fn the_kernel_law_reproduces_all_192_fixture_cells() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let cells = fixture["cells"].as_array().unwrap();
    assert_eq!(cells.len(), 192);
    for cell in cells {
        let matrix = cell["matrix"].as_u64().unwrap();
        let lower = pattern(&text(cell, "lower_trigram"));
        let upper = pattern(&text(cell, "upper_trigram"));
        let coordinate = text(cell, "coordinate");
        let got = generate(family(matrix), (upper << 3) | lower);
        assert_eq!(got.family.matrix_number() as u64, matrix, "{coordinate}");
        let pair = |p: ql_core::PairIndex16| format!("{}{}", p.first(), p.second());
        assert_eq!(
            pair(got.lower_pair),
            text(cell, "lower_pair"),
            "{coordinate}"
        );
        assert_eq!(
            pair(got.upper_pair),
            text(cell, "upper_pair"),
            "{coordinate}"
        );
        assert_eq!(
            got.positive.to_string(),
            text(cell, "positive"),
            "{coordinate}"
        );
        assert_eq!(
            got.negative.to_string(),
            text(cell, "negative"),
            "{coordinate}"
        );
        assert_eq!(
            got.is_non_dual(),
            cell["non_dual"].as_bool().unwrap(),
            "{coordinate}"
        );
        assert_eq!(
            class_name(got.class),
            text(cell, "cell_class"),
            "{coordinate}"
        );
        // The map's hexagram coordinate is M3-1-<lower id>-<upper id> in the
        // kernel's trigram ids: the line patterns identify the same trigrams.
        let hexagram = text(cell, "hexagram");
        let expected = format!(
            "M3-1-{}-{}",
            Trigram::from_binary(lower).id,
            Trigram::from_binary(upper).id
        );
        assert_eq!(hexagram, expected, "{coordinate}");
    }
}

#[test]
fn the_fixture_trigram_pair_table_is_the_kernel_table() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    for (m, family) in MatrixFamily::ALL.into_iter().enumerate() {
        let table = &fixture["trigram_pairs"][(m + 1).to_string()];
        for name in TRIGRAM_NAMES {
            let p = ql_core::trigram_pair(family, pattern(name));
            assert_eq!(
                format!("{}{}", p.first(), p.second()),
                table[name].as_str().unwrap(),
                "matrix {} {name}",
                m + 1
            );
        }
    }
}
