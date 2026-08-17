#[test]
fn service_schema_exposes_revision_bearing_target_inputs() {
    let schema = include_str!("../../../schemas/q3/service.schema.json");

    for definition in [
        "QlTarget",
        "TargetInput",
        "LocateRequest",
        "RefractRequest",
        "RelateRequest",
    ] {
        assert!(
            schema.contains(&format!("\"{definition}\"")),
            "missing schema definition: {definition}"
        );
    }

    assert!(schema.contains("\"revision\": {\"type\": [\"string\", \"null\"]}"));
    assert!(schema.contains("\"input\": {\"$ref\": \"#/$defs/TargetInput\"}"));
    assert!(schema.contains("\"items\": {\"$ref\": \"#/$defs/TargetInput\"}"));
}
