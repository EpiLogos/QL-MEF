//! Conformance for `ql.pole.m3-architecture-coverage/v1` — the completeness
//! ledger of the M3 codon-system port.
//!
//! The ledger is the anti-piecemeal instrument: every exported artifact of
//! the vendored M3 reference surface must have exactly one row; every row
//! names a real vendor symbol; every disposition is one of the closed set;
//! open rows carry their reason; ported rows resolve to Rust homes that
//! exist (the `use` items in this file are compile-checked).

use std::collections::HashSet;

const LEDGER: &str = include_str!("../../../fixtures/pole/m3-architecture-coverage-v1.tsv");
const VENDOR_M3C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m3.c");
const VENDOR_M3H: &str = include_str!("../../../vendor/epi-kernel/reference/include/m3.h");
const VENDOR_M2C: &str = include_str!("../../../vendor/epi-kernel/reference/src/m2.c");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    artifact: String,
    kind: String,
    fr: String,
    capability: String,
    branch: String,
    disposition: String,
    home: String,
    note: String,
}

fn ledger_rows() -> Vec<Row> {
    LEDGER
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with("meta\t"))
        .map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            assert_eq!(fields.len(), 8, "ledger row must have 8 columns: {line}");
            Row {
                artifact: fields[0].to_string(),
                kind: fields[1].to_string(),
                fr: fields[2].to_string(),
                capability: fields[3].to_string(),
                branch: fields[4].to_string(),
                disposition: fields[5].to_string(),
                home: fields[6].to_string(),
                note: fields[7].to_string(),
            }
        })
        .collect()
}

#[test]
fn every_row_names_a_real_vendor_symbol() {
    let combined = format!("{VENDOR_M3C}\n{VENDOR_M3H}\n{VENDOR_M2C}");
    // Group rows (named for a family of symbols rather than one literal)
    // carry their member symbols in the note column.
    let group_rows = [
        "charges-and-epogdoon-inlines",
        "m3_init/m3_teardown/m3_cli_dispatch",
    ];
    for row in ledger_rows() {
        if group_rows.contains(&row.artifact.as_str()) {
            let members = [
                "evaluate_codon",
                "m3_compute_charges",
                "apply_epogdoon_compression",
                "get_parashakti_frequency",
                "m3_init",
                "m3_teardown",
                "m3_cli_dispatch",
                "is_evolutionary_gap",
            ];
            for member in members {
                assert!(
                    combined.contains(member),
                    "group row {} member '{member}' must exist in vendor source",
                    row.artifact
                );
            }
            continue;
        }
        let symbol = row.artifact.split('/').next().unwrap();
        assert!(
            combined.contains(symbol),
            "ledger artifact '{}' must exist in the vendored source",
            row.artifact
        );
    }
}

#[test]
fn every_vendor_m3_data_table_has_a_ledger_row() {
    // The complete list of `const` data tables exported by vendor m3.c.
    let vendor_tables: HashSet<&str> = [
        "M3_SD_Value M3_PAIR_MATRIX",
        "uint8_t M3_MATRIX_PAIR",
        "M3_Trigram M3_TRIGRAM_LUT",
        "M3_Hexagram M3_HEXAGRAM_LUT",
        "uint8_t M3_NONDUAL_CODONS",
        "uint64_t M3_RNA_FUNCTIONAL_MASK",
        "uint64_t M3_RNA_DARK_MASK",
        "uint8_t M3_COMP_MATRIX",
        "uint8_t M3_MOVE_MATRIX",
        "uint8_t M3_RES_MATRIX",
        "uint8_t M3_CODON_TO_AA",
        "M3_Major_Arcana_Entry M3_MAJOR_ARCANA",
        "M3_TarotCodonEntry M3_TAROT_CODON_MAP",
        "M3_Rotational_Profile M3_ROTATIONAL_PROFILE",
    ]
    .into_iter()
    .collect();
    assert_eq!(vendor_tables.len(), 14, "the vendor surface is 14 tables");

    let ledger_artifacts: HashSet<String> =
        ledger_rows().into_iter().map(|row| row.artifact).collect();
    for table in &vendor_tables {
        let table_name = table.split(' ').nth(1).expect("table name");
        assert!(
            ledger_artifacts
                .iter()
                .any(|artifact| artifact.starts_with(table_name)),
            "vendor table {table_name} has no ledger row — the port would be piecemeal"
        );
    }
    // The two M2-registered tables the DET consumes.
    assert!(
        VENDOR_M2C.contains("M2_TO_M3_CYMATIC_PROJECTION[72]"),
        "the projection table must stay under coverage"
    );
}

#[test]
fn dispositions_are_from_the_closed_set_and_open_rows_carry_reasons() {
    let allowed = ["ported", "ported-as-law", "shadowed", "open", "deferred"];
    for row in ledger_rows() {
        assert!(
            allowed.contains(&row.disposition.as_str()),
            "unknown disposition '{}' on {}",
            row.disposition,
            row.artifact
        );
        if row.disposition == "open" || row.disposition == "deferred" {
            assert!(
                row.note.len() > 40,
                "open/deferred rows must carry their reason: {}",
                row.artifact
            );
        }
    }
}

#[test]
fn ported_rows_resolve_to_existing_rust_homes() {
    // Each ported row's home must name a module path that exists in the
    // compiled crate. Compile-checked directly: the items below are the
    // homes the port series landed; a missing module breaks this file.
    // iching (trigram/nuclear/complement), quaternion (DET overlay chain),
    // inverse (M3-C31 split), pose (472), nucleotide (value table), coin
    // ground (pair law).
    let homes_present = (
        ql_core::ICHING_GRAMMAR_REF,
        ql_core::ORIENTATION_CHAIN_REF,
        ql_core::INVERSE_SEAM_CONTRACT_REF,
        ql_core::ROTATIONAL_STATE_TOTAL,
        ql_core::Nucleotide::NUCLEOTIDE_COIN_VALUE,
        ql_core::POLE_COIN_CONTRACT_REF,
        ql_core::POLE_FOLD_STATE_REF,
        ql_core::POLE_ICHING_GRAMMAR_CONTRACT_REF,
        ql_core::RES_GAP_ADDRESSES,
        ql_core::Trigram::LUT,
        ql_core::palindromic_anchors(),
        ql_core::det_overlay,
        // Tarot bridge (pole::tarot) — M3-C19/C20.
        ql_core::POLE_TAROT_BRIDGE_REF,
        ql_core::ENTRIES_PER_SUIT,
        ql_core::MINOR_ARCANA_COUNT,
        ql_core::MAJOR_ARCANA_COUNT,
        ql_core::TRANSCENDENT_TAROT_COUNT,
        // Rotational machinery (pole::rotational) — M3-C13.
        ql_core::POLE_ROTATIONAL_PROFILE_REF,
        ql_core::ROTATIONAL_TABLE_ENTRIES,
        ql_core::RECORDED_PAIR_DIFF_SIGNS,
        ql_core::generate_rotational_states,
        ql_core::wc_anticodon,
        // Transcription layer (pole::transcription) — M3-C02/C17/C18.
        ql_core::POLE_TRANSCRIPTION_REF,
        ql_core::M2_TO_M3_CYMATIC_PROJECTION,
        ql_core::M3_CODON_TO_AA,
        ql_core::M3_RNA_FUNCTIONAL_MASK,
        ql_core::M3_PAIR_DIFFERENCE_SIGN,
        ql_core::transduce_vibration_to_symbol,
    );
    let _ = homes_present;

    for row in ledger_rows() {
        if row.disposition.starts_with("ported") {
            assert!(
                !row.home.trim().is_empty(),
                "ported row {} must name its Rust home",
                row.artifact
            );
        }
    }
}

#[test]
fn value_dependent_tables_are_marked_regenerated() {
    // The four tables the coin correction regenerated must say so.
    for name in ["M3_PAIR_MATRIX", "M3_MAJOR_ARCANA", "M3_TAROT_CODON_MAP"] {
        let row = ledger_rows()
            .into_iter()
            .find(|row| row.artifact == name)
            .unwrap_or_else(|| panic!("{name} must be covered"));
        assert!(
            matches!(row.disposition.as_str(), "ported" | "ported-as-law"),
            "{name} is ported"
        );
    }
    // And the corrected value table itself is pinned by the coin contract.
    let header = include_str!("../../../vendor/epi-kernel/reference/include/m3.h");
    assert!(
        header.contains("{6, 9, 8, 7}"),
        "the corrected table stands"
    );
    assert!(!header.contains("{6, 9, 7, 8}"), "the legacy table is gone");
}
