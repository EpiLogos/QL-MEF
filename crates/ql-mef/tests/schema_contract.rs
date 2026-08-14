#[test]
fn language_neutral_schema_exposes_required_q2_contracts() {
    let schema = include_str!("../../../schemas/q2/contracts.schema.json");
    for definition in [
        "ClientRef",
        "LensRef",
        "SublensRef",
        "ResultClass",
        "QLTarget",
        "QLProviderRef",
        "InputRefRevision",
        "QLProvenance",
        "RefractionContract",
        "QLReading",
        "QLRelationReading",
        "QLSynthesis",
    ] {
        assert!(schema.contains(&format!("\"{definition}\"")), "missing schema definition: {definition}");
    }
    for result_class in ["canonical", "deterministic", "semantic-stochastic", "research"] {
        assert!(schema.contains(&format!("\"{result_class}\"")));
    }
    assert!(schema.contains("^mef:lens:L[0-5]'?@1$"));
    assert!(schema.contains("^mef:sublens:L[0-5]'?\\\\.[0-5]@1$"));
}
