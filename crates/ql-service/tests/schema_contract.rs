#[test]
fn language_neutral_capability_schema_names_the_stable_surface() {
    let schema = include_str!("../../../schemas/q3/service.schema.json");

    for definition in [
        "ProviderState",
        "ProviderClass",
        "Operation",
        "InputLimits",
        "ProviderHealth",
        "ProviderCapabilities",
        "CapabilityDecision",
        "ServiceRequest",
    ] {
        assert!(
            schema.contains(&format!("\"{definition}\"")),
            "missing schema definition: {definition}"
        );
    }

    for state in ["absent", "available", "degraded", "incompatible"] {
        assert!(schema.contains(&format!("\"{state}\"")));
    }
    for operation in ["capabilities", "locate", "refract", "relate", "synthesise"] {
        assert!(schema.contains(&format!("\"{operation}\"")));
    }
    for class in ["formal-kernel", "semantic-refraction"] {
        assert!(schema.contains(&format!("\"{class}\"")));
    }
}
