use ql_mef::all_lens_definitions;

#[test]
fn stable_registry_does_not_promote_unsettled_harmonic_or_generated_role_fields() {
    let forbidden = ["tonic", "diatonic", "harmonic", "36-state", "64-state", "generated-role"];
    let stable_text = all_lens_definitions()
        .iter()
        .flat_map(|lens| std::iter::once(lens.name()).chain(lens.sublens_labels()))
        .collect::<Vec<_>>()
        .join("\n")
        .to_lowercase();

    for term in forbidden {
        assert!(!stable_text.contains(term), "stable registry leaked research term: {term}");
    }
}
