//! Execute the full recovered definition vocabulary through current operations.
//! Admission here is a controlled native-owner test fixture, not a live grant.
use ql_mef::m_tree::native_m_registry;
use ql_mef::property::*;
use serde_json::{Value, json};

struct FixtureGate;
impl NativePropertyAdmission for FixtureGate {
    fn check_source(&self, source: &SourcePin) -> Result<()> { source.validate() }
    fn admit(&self, _: &str, _: &Value) -> Result<SourcePin> { Ok(pin("fixture/authorised", "1")) }
}
fn pin(reference: &str, revision: &str) -> SourcePin {
    SourcePin { reference: reference.into(), revision: revision.into(), use_mode: SourceUse::Historical }
}
fn sample(spec: &SourcePropertySpec, dimensions: usize) -> Value {
    let value = match spec.value_type {
        PropertyType::String => json!("source-qualified value"),
        PropertyType::StringList => json!(["source-qualified value"]),
        PropertyType::Integer => json!(1), PropertyType::Float => json!(1.25),
        PropertyType::Boolean => json!(true),
        PropertyType::DateTime => json!("2026-09-13T00:00:00Z"),
        PropertyType::JsonString => json!("{\"source\":\"retained\"}"),
        PropertyType::Embedding => json!(vec![0.0; dimensions]),
    };
    if spec.cardinality == Cardinality::Many && !matches!(spec.value_type, PropertyType::StringList | PropertyType::Embedding) { json!([value]) } else { value }
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("pass the decompressed original property source JSON")?;
    let source: Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let columns = source["columns"].as_array().ok_or("missing source columns")?;
    let rows = source["properties"].as_array().ok_or("missing property definitions")?;
    if rows.len() != 194 { return Err("the full original 194-definition vocabulary is required".into()); }
    let registry = native_m_registry();
    let mut field = PropertyField::new(registry);
    let mut relationship_count = 0;
    for (index, row) in rows.iter().enumerate() {
        let values = row.as_array().ok_or("invalid source property row")?;
        if values.len() != columns.len() { return Err("source property row/column mismatch".into()); }
        let object: serde_json::Map<String, Value> = columns.iter().zip(values).map(|(k,v)| Ok((k.as_str().ok_or("invalid column")?.into(), v.clone()))).collect::<std::result::Result<_, &str>>()?;
        let spec: SourcePropertySpec = serde_json::from_value(Value::Object(object))?;
        let dimensions = source["embedding_dimensions"].as_u64().ok_or("missing original embedding dimensions")? as usize;
        let value = sample(&spec, dimensions);
        spec.validate_value(&value, Some(dimensions))?;
        if spec.validate_value(&Value::Null, Some(dimensions)).is_ok() { return Err("null type mismatch admitted".into()); }
        let subject = Subject { registry_revision: registry.manifest().registry_revision.clone(), owner: spec.owner,
            id: if spec.owner == PropertyOwner::Node {registry.root(0).ok_or("M0 absent")?.id} else {
                relationship_count += 1; registry.manifest().relations.first().ok_or("no native relation")?.id } };
        let identity = format!("original-property/{:?}/{}", spec.owner, spec.key);
        let definition = Definition { reference:VersionRef {identity, revision:1},
            declaration:pin(source["source"]["path"].as_str().ok_or("source path absent")?, source["source"]["git_blob"].as_str().ok_or("source blob absent")?),
            applicability_basis:pin("fixture/explicit-native-subject", "1"),
            applies_to:vec![subject.clone()], aliases:vec![], unit:None, domain:vec![],
            embedding_dimensions: if spec.value_type == PropertyType::Embedding {Some(dimensions)} else {None}, spec };
        let reference = definition.reference.clone();
        field.define(&format!("define/{index}"), definition, None, &FixtureGate)?;
        let draft = Draft {identity:format!("assertion/{index}"), definition:reference, subject, value,
            role:ValueRole::Rich, standing:Standing::Proposed, sources:vec![pin("fixture/value-source", "1")],
            producer:pin("fixture/producer", "1"), result:pin("fixture/result", "1"), warrant:"controlled current Rust value operation, not source applicability or live graph proof".into(),
            valid_from_unix_ms:None, valid_until_unix_ms:None, derivation:None};
        let receipt = field.publish(&format!("publish/{index}"), draft, None, &FixtureGate)?;
        let compact = field.compact(&receipt.reference, MAX_VALUE_BYTES, &FixtureGate)?;
        if field.expand(&compact, &FixtureGate)?.reference != receipt.reference {return Err("lost source assertion".into());}
    }
    if relationship_count != 30 { return Err("relationship vocabulary coverage changed".into()); }
    println!("{}", json!({"definition_operations":rows.len(), "node_definitions":164, "relationship_definitions":relationship_count,
        "typed_negative_cases":rows.len(), "compact_expansions":rows.len(), "standing":"controlled-native-property-operations", "live_neo4j":false, "canonical_vocabulary_promotion":false}));
    Ok(())
}
