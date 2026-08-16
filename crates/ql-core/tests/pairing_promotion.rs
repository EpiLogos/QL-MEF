use ql_core::PAIRING_GRAMMAR_VERSION;

#[test]
fn pairing_promotion_manifest_keeps_formal_and_research_boundaries_explicit() {
    let manifest = include_str!("../../../fixtures/q6/pairing-promotion-v1.json");

    assert_eq!(PAIRING_GRAMMAR_VERSION, "1.0.0");
    assert!(manifest.contains("\"capability\": \"ql.pairing\""));
    assert!(manifest.contains("\"status\": \"specified-formal-structure\""));
    assert!(manifest.contains("\"automaticTraversal\": false"));
    assert!(manifest.contains("\"grammarSelection\": \"deterministic\""));
    assert!(manifest.contains("\"invocationPolicy\": \"external-experimental-policy\""));
    assert!(manifest.contains("\"entries\": 9"));
    assert!(manifest.contains("\"orientedStructures\": 8"));
    assert!(manifest.contains("\"unorderedAddressTetrads\": 7"));

    for research_extension in [
        "ql.state64.runtime-semantics",
        "ql.epogdoon.retained-difference-metric",
        "ql.topology.control-rules",
        "ql.mef.context-operational-roles",
    ] {
        assert!(manifest.contains(research_extension));
    }

    assert!(manifest.contains("\"factoryIssue\": 108"));
    assert!(manifest.contains("\"factoryPullRequest\": 130"));
    assert!(manifest.contains("\"factoryHead\": \"a654c62f68b82236061986d9215b23257fe53b17\""));
    assert!(
        manifest.contains(
            "\"factoryPairingGrammarBlob\": \"0d6aa49197dd4d06646ea5a5fb094c03b10a74e4\""
        )
    );
}
