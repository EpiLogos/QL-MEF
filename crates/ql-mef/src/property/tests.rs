use super::*;
use crate::m_tree::native_m_registry;
use std::cell::RefCell;

fn pin(name: &str) -> SourcePin {
    SourcePin {
        reference: name.into(),
        revision: "r1".into(),
        use_mode: SourceUse::Current,
    }
}
struct Gate {
    deny: bool,
    stale: RefCell<Option<String>>,
}
impl NativePropertyAdmission for Gate {
    fn check_source(&self, source: &SourcePin) -> Result<()> {
        require(
            self.stale.borrow().as_deref() != Some(source.reference.as_str()),
            "source unavailable or changed",
        )
    }
    fn admit(&self, _: &str, _: &Value) -> Result<SourcePin> {
        require(!self.deny, "native owner denied")?;
        Ok(pin("native/authority/receipt"))
    }
}
fn gate() -> Gate {
    Gate {
        deny: false,
        stale: RefCell::new(None),
    }
}
fn subject(owner: PropertyOwner) -> Subject {
    let registry = native_m_registry();
    Subject {
        registry_revision: registry.manifest().registry_revision.clone(),
        owner,
        id: if owner == PropertyOwner::Node {
            registry.root(0).unwrap().id
        } else {
            registry.manifest().relations[0].id
        },
    }
}
fn definition(owner: PropertyOwner) -> Definition {
    Definition {
        reference: VersionRef {
            identity: "property/source/meaning".into(),
            revision: 1,
        },
        declaration: pin("source/schema"),
        applicability_basis: pin("source/applicability"),
        spec: SourcePropertySpec {
            key: "q_4_meaning".into(),
            coordinate_home: "S2".into(),
            owner,
            value_type: PropertyType::String,
            cardinality: Cardinality::One,
            disclosure: Disclosure::Protected,
            source_family: "source-meaning".into(),
            indexed: false,
            compatibility: false,
        },
        applies_to: vec![subject(owner)],
        aliases: vec!["meaning".into()],
        unit: None,
        domain: vec![],
        embedding_dimensions: None,
    }
}
fn draft(id: &str, owner: PropertyOwner) -> Draft {
    Draft {
        identity: id.into(),
        definition: definition(owner).reference,
        subject: subject(owner),
        value: json!("original"),
        role: ValueRole::Rich,
        standing: Standing::SourceDeclared,
        sources: vec![pin("source/claim")],
        producer: pin("producer/author"),
        result: pin("source/result"),
        warrant: "source-qualified statement, not universal canon".into(),
        valid_from_unix_ms: None,
        valid_until_unix_ms: None,
        derivation: None,
    }
}
fn field(owner: PropertyOwner, gate: &Gate) -> PropertyField<'static> {
    let mut field = PropertyField::new(native_m_registry());
    field
        .define("define", definition(owner), None, gate)
        .unwrap();
    field
}

#[test]
fn independent_relation_assertions_never_overwrite_each_other() {
    let gate = gate();
    let mut field = field(PropertyOwner::Relationship, &gate);
    let first = field
        .publish(
            "first",
            draft("claim/a", PropertyOwner::Relationship),
            None,
            &gate,
        )
        .unwrap();
    let mut opposite = draft("claim/b", PropertyOwner::Relationship);
    opposite.value = json!("contrary qualification");
    let second = field.publish("second", opposite, None, &gate).unwrap();
    assert_ne!(first.reference.identity, second.reference.identity);
    let readings = field
        .assertions_for(&subject(PropertyOwner::Relationship), &gate)
        .unwrap();
    assert_eq!(readings.len(), 2);
    assert_eq!(
        field.read(&first.reference, &gate).unwrap().content.value,
        "original"
    );
}

#[test]
fn compact_expands_exactly_and_tampering_or_truncation_is_refused() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let receipt = field
        .publish(
            "publish",
            draft("claim/a", PropertyOwner::Node),
            None,
            &gate,
        )
        .unwrap();
    let compact = field.compact(&receipt.reference, 100, &gate).unwrap();
    assert_eq!(compact.role, ValueRole::Rich); // q_4_ does not select office #4.
    let encoded = serde_json::to_string(&compact).unwrap();
    let restored: Compact = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        field.expand(&restored, &gate).unwrap().reference,
        receipt.reference
    );
    let mut tampered = compact;
    tampered.standing = Standing::Reviewed;
    assert!(field.expand(&tampered, &gate).is_err());
    assert!(field.compact(&receipt.reference, 2, &gate).is_err());
}

#[test]
fn denied_and_stale_sources_leave_no_partial_mutation() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let denied = Gate {
        deny: true,
        stale: RefCell::new(None),
    };
    assert!(
        field
            .publish(
                "denied",
                draft("claim/a", PropertyOwner::Node),
                None,
                &denied
            )
            .is_err()
    );
    *gate.stale.borrow_mut() = Some("source/claim".into());
    assert!(
        field
            .publish("stale", draft("claim/a", PropertyOwner::Node), None, &gate)
            .is_err()
    );
    *gate.stale.borrow_mut() = None;
    assert!(
        field
            .assertions_for(&subject(PropertyOwner::Node), &gate)
            .unwrap()
            .is_empty()
    );
    field
        .publish("stale", draft("claim/a", PropertyOwner::Node), None, &gate)
        .unwrap();
    assert!(
        field
            .read(
                &VersionRef {
                    identity: "claim/a".into(),
                    revision: 1
                },
                &denied
            )
            .is_err()
    );
}

#[test]
fn revision_compare_exchange_replay_and_conflicting_operation_identity() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let original = draft("claim/a", PropertyOwner::Node);
    let first = field
        .publish("write-1", original.clone(), None, &gate)
        .unwrap();
    assert!(
        field
            .publish("write-1", original.clone(), None, &gate)
            .unwrap()
            .replayed
    );
    let mut changed = original;
    changed.value = json!("later");
    assert!(
        field
            .publish("write-1", changed.clone(), None, &gate)
            .is_err()
    );
    assert!(
        field
            .publish("write-2", changed.clone(), None, &gate)
            .is_err()
    );
    let second = field.publish("write-2", changed, Some(1), &gate).unwrap();
    assert_eq!(second.reference.revision, 2);
    assert_eq!(
        field.read(&first.reference, &gate).unwrap().content.value,
        "original"
    );
}

#[test]
fn review_and_rollback_preserve_history_without_reusing_old_authority() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let first = field
        .publish("create", draft("claim/a", PropertyOwner::Node), None, &gate)
        .unwrap();
    let reviewed = field
        .recognise("review", &first.reference, pin("recognition/human"), &gate)
        .unwrap();
    assert_eq!(
        field
            .read(&reviewed.reference, &gate)
            .unwrap()
            .content
            .standing,
        Standing::Reviewed
    );
    let rolled = field
        .rollback("rollback", &reviewed.reference, &first.reference, &gate)
        .unwrap();
    let new = field.read(&rolled.reference, &gate).unwrap();
    assert_eq!(new.reference.revision, 3);
    assert_eq!(new.restored_from, Some(first.reference.clone()));
    assert_eq!(new.content.standing, Standing::Proposed);
    assert!(new.recognition.is_none());
    assert!(
        field
            .read(&reviewed.reference, &gate)
            .unwrap()
            .recognition
            .is_some()
    );
    assert!(
        field
            .recognise(
                "stale-review",
                &first.reference,
                pin("recognition/human"),
                &gate
            )
            .is_err()
    );
}

#[test]
fn no_source_home_scope_inference_and_no_assertion_rebinding() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let mut outside = draft("claim/a", PropertyOwner::Node);
    outside.subject.id = native_m_registry().root(1).unwrap().id;
    assert!(field.publish("outside", outside, None, &gate).is_err());
    let mut wrong_revision = draft("claim/a", PropertyOwner::Node);
    wrong_revision.subject.registry_revision = "another-graph".into();
    assert!(
        field
            .publish("wrong-revision", wrong_revision, None, &gate)
            .is_err()
    );
    let mut forged_review = draft("claim/a", PropertyOwner::Node);
    forged_review.standing = Standing::Reviewed;
    assert!(
        field
            .publish("forged-review", forged_review, None, &gate)
            .is_err()
    );
}

#[test]
fn definition_changes_invalidate_new_uses_but_keep_original_expansion() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let first = field
        .publish(
            "publish",
            draft("claim/a", PropertyOwner::Node),
            None,
            &gate,
        )
        .unwrap();
    let compact = field.compact(&first.reference, 100, &gate).unwrap();
    let mut newer = definition(PropertyOwner::Node);
    newer.reference.revision = 2;
    newer.declaration.revision = "r2".into();
    field
        .define("new-definition", newer, Some(1), &gate)
        .unwrap();
    assert!(
        field
            .publish(
                "stale-definition",
                draft("claim/b", PropertyOwner::Node),
                None,
                &gate
            )
            .is_err()
    );
    assert_eq!(
        field
            .expand(&compact, &gate)
            .unwrap()
            .content
            .definition
            .revision,
        1
    );
}

#[test]
fn ambiguous_property_handles_stay_plural() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let mut another = definition(PropertyOwner::Node);
    another.reference.identity = "property/other-owner/meaning".into();
    field
        .define("other-definition", another, None, &gate)
        .unwrap();
    assert_eq!(field.definitions_named("meaning", &gate).unwrap().len(), 2);
}

#[test]
fn all_scalar_types_cardinalities_and_time_errors_are_distinguished() {
    let mut spec = definition(PropertyOwner::Node).spec;
    for (kind, good) in [
        (PropertyType::String, json!("text")),
        (PropertyType::StringList, json!(["a", "b"])),
        (PropertyType::Integer, json!(-4)),
        (PropertyType::Float, json!(1.25)),
        (PropertyType::Boolean, json!(false)),
        (
            PropertyType::DateTime,
            json!("2024-02-29T09:30:00.12+01:00"),
        ),
        (PropertyType::JsonString, json!("{\"meaning\":true}")),
        (PropertyType::Embedding, json!([0.0, 1.0])),
    ] {
        spec.value_type = kind;
        spec.cardinality = Cardinality::One;
        spec.validate_value(&good, Some(2)).unwrap();
        assert!(spec.validate_value(&Value::Null, Some(2)).is_err());
        spec.cardinality = Cardinality::Many;
        let many = if matches!(kind, PropertyType::StringList | PropertyType::Embedding) {
            good
        } else {
            json!([good])
        };
        spec.validate_value(&many, Some(2)).unwrap();
    }
    spec.value_type = PropertyType::DateTime;
    spec.cardinality = Cardinality::One;
    for invalid in [
        "2025-02-29T09:30:00Z",
        "2026-01-01T25:00:00Z",
        "2026-01-01T00:00:00",
        "2026-01-01T00:00:00+29:00",
        "2026-01-01T00:00:60Z",
    ] {
        assert!(spec.validate_value(&json!(invalid), None).is_err());
    }
}

#[test]
fn quintessential_and_graph_derived_roles_require_explicit_producer_basis() {
    let gate = gate();
    let mut field = field(PropertyOwner::Node, &gate);
    let mut reading = draft("reading/q", PropertyOwner::Node);
    reading.role = ValueRole::Quintessential;
    reading.standing = Standing::Proposed;
    reading.producer = pin("producer/defined-synthesis");
    let receipt = field.publish("synthesis", reading, None, &gate).unwrap();
    assert_eq!(
        field.read(&receipt.reference, &gate).unwrap().content.role,
        ValueRole::Quintessential
    );
    let mut derived = draft("reading/derived", PropertyOwner::Node);
    derived.role = ValueRole::GraphDerived;
    derived.standing = Standing::Derived;
    assert!(
        field
            .publish("no-graph", derived.clone(), None, &gate)
            .is_err()
    );
    derived.derivation = Some(GraphDerivation {
        graph: pin("graph/projection"),
        algorithm: pin("algorithm/degree"),
        parameters: json!({"orientation":"in"}),
        included_nodes: vec![subject(PropertyOwner::Node).id],
        excluded_nodes: vec![],
    });
    let receipt = field
        .publish("with-graph", derived.clone(), None, &gate)
        .unwrap();
    let reviewed = field
        .recognise(
            "review-graph",
            &receipt.reference,
            pin("recognition/graph"),
            &gate,
        )
        .unwrap();
    assert_eq!(
        field.read(&reviewed.reference, &gate).unwrap().content.role,
        ValueRole::GraphDerived
    );
    derived.identity = "reading/excluded".into();
    derived
        .derivation
        .as_mut()
        .unwrap()
        .excluded_nodes
        .push(subject(PropertyOwner::Node).id);
    assert!(
        field
            .publish("mixed-projection", derived, None, &gate)
            .is_err()
    );
}
