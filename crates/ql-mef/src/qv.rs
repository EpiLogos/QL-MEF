//! Source-qualified quintessential-view operations over the existing property field.
//!
//! Historical QV is Epii's per-coordinate self-view: pithy source-backed readings
//! with deeper lens registers, baked into `qv_data.c` by the old `epi core knowing`
//! owner. The `q_*` graph family is the property-side quick-view surface. This
//! module restores that relation without creating a second graph/property store or
//! treating a `q_` spelling as producer authority.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::m_tree::{MRegistry, MTreeId};
use crate::property::{
    Cardinality, Definition, Draft, NativePropertyAdmission, PropertyField, PropertyOwner,
    PropertyType, SourcePin, Standing, ValueRole, VersionRef,
};

pub type Result<T> = std::result::Result<T, String>;

pub const QV_CONTRACT: &str = "ql.qv/v1";
pub const QV_BAKE_CONTRACT: &str = "ql.qv-bake/v1";
pub const QV_SOURCE_REPOSITORY: &str = "EpiLogos/Epi-Logos-C-Experiments";
pub const QV_SOURCE_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
pub const QV_DATA_SOURCE_PATH: &str = "Body/S/S0/epi-lib/src/qv_data.c";
pub const QV_DATA_SOURCE_BLOB: &str = "45001d22ade83f029faaff08726d0f027a5f16c9";
pub const QV_API_SOURCE_PATH: &str = "Body/S/S0/epi-lib/include/m5.h";
pub const QV_PROPERTY_SOURCE_PATH: &str = "Idea/Bimba/Map/datasets/deep-property-map.md";
pub const QV_PITHY_MAX_CHARS: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QvRegister {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl QvRegister {
    pub const ALL: [Self; 6] = [Self::L0, Self::L1, Self::L2, Self::L3, Self::L4, Self::L5];

    pub const fn index(self) -> u8 {
        match self {
            Self::L0 => 0,
            Self::L1 => 1,
            Self::L2 => 2,
            Self::L3 => 3,
            Self::L4 => 4,
            Self::L5 => 5,
        }
    }
}

/// One source-qualified write into an already-defined q_* property identity.
/// Definition creation remains in `PropertyField`; this operation refuses to
/// infer a definition, producer, coordinate or office from the spelling alone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QvWrite {
    pub coordinate_ref: String,
    pub property_key: String,
    pub definition: VersionRef,
    pub assertion_identity: String,
    pub pithy: String,
    pub standing: Standing,
    pub sources: Vec<SourcePin>,
    pub producer: SourcePin,
    pub result: SourcePin,
    pub warrant: String,
    pub valid_from_unix_ms: Option<i64>,
    pub valid_until_unix_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QvReceipt {
    pub version: String,
    pub coordinate_ref: String,
    pub coordinate_id: MTreeId,
    pub registry_revision: String,
    pub q_position: u8,
    pub property_key: String,
    pub definition: VersionRef,
    pub assertion: VersionRef,
    pub producer: SourcePin,
    pub result: SourcePin,
    pub admission: SourcePin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QvReading {
    pub receipt: QvReceipt,
    pub pithy: String,
    pub standing: Standing,
    pub sources: Vec<SourcePin>,
    pub warrant: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QvBakeEntry {
    pub coordinate_ref: String,
    pub coordinate_id: MTreeId,
    pub q_position: u8,
    pub property_key: String,
    pub pithy: String,
    pub assertion: VersionRef,
    pub producer: SourcePin,
    pub result: SourcePin,
    pub admission: SourcePin,
}

/// Deterministic reviewable bake product. It is deliberately not a C writer:
/// #132/K8.0 remains the sole shared C/registry integration writer. A native
/// owner may materialise this exact artifact into its current generated surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QvBakeArtifact {
    pub version: String,
    pub registry_revision: String,
    pub historical_source_repository: String,
    pub historical_source_revision: String,
    pub historical_source_path: String,
    pub historical_source_blob: String,
    pub entries: Vec<QvBakeEntry>,
}

fn require(ok: bool, message: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(message.into()) }
}

fn nonempty(value: &str, what: &str) -> Result<()> {
    require(
        !value.is_empty() && value == value.trim() && !value.contains('\0'),
        what,
    )
}

pub fn q_position(property_key: &str) -> Result<u8> {
    nonempty(property_key, "invalid q_* property key")?;
    let rest = property_key
        .strip_prefix("q_")
        .ok_or("quintessential property must use the source q_<0..5>_<semantic> form")?;
    let (position_text, semantic) = rest
        .split_once('_')
        .ok_or("quintessential property is missing its semantic name")?;
    let position = position_text
        .parse::<u8>()
        .map_err(|_| "quintessential position is not numeric".to_owned())?;
    require(position <= 5, "quintessential position is outside #0..#5")?;
    require(
        position.to_string() == position_text && !semantic.trim().is_empty(),
        "non-canonical quintessential property spelling",
    )?;
    Ok(position)
}

fn qv_definition<G: NativePropertyAdmission>(
    field: &PropertyField<'_>,
    write: &QvWrite,
    gate: &G,
) -> Result<Definition> {
    let position = q_position(&write.property_key)?;
    let definition = field
        .definitions_named(&write.property_key, gate)?
        .into_iter()
        .find(|definition| definition.reference == write.definition)
        .cloned()
        .ok_or("q_* definition/revision is absent or does not own this name")?;
    require(
        definition.spec.key == write.property_key,
        "an alias cannot silently become the canonical q_* producer",
    )?;
    require(
        definition.spec.owner == PropertyOwner::Node
            && definition.spec.value_type == PropertyType::String
            && definition.spec.cardinality == Cardinality::One,
        "QV pithy values require a single string node property",
    )?;
    require(
        q_position(&definition.spec.key)? == position,
        "q_* definition position changed during resolution",
    )?;
    Ok(definition)
}

fn qv_subject(
    registry: &MRegistry,
    coordinate_ref: &str,
) -> Result<(MTreeId, crate::property::Subject)> {
    nonempty(coordinate_ref, "invalid QV coordinate reference")?;
    let node = registry
        .resolve(coordinate_ref)
        .ok_or("QV coordinate is absent from the selected native M registry")?;
    Ok((
        node.id,
        crate::property::Subject {
            registry_revision: registry.manifest().registry_revision.clone(),
            owner: PropertyOwner::Node,
            id: node.id,
        },
    ))
}

/// Write a QV through the existing rich property owner. Explicit source,
/// producer and result identities are mandatory; a q_* spelling never supplies
/// them. Historical <=128-character pithiness is retained as a contract.
pub fn write_qv<G: NativePropertyAdmission>(
    registry: &MRegistry,
    field: &mut PropertyField<'_>,
    operation: &str,
    write: QvWrite,
    expected: Option<u64>,
    gate: &G,
) -> Result<QvReceipt> {
    nonempty(operation, "invalid QV operation identity")?;
    nonempty(&write.assertion_identity, "invalid QV assertion identity")?;
    nonempty(&write.warrant, "QV write requires an explicit warrant")?;
    require(
        !write.pithy.is_empty()
            && write.pithy == write.pithy.trim()
            && write.pithy.chars().count() <= QV_PITHY_MAX_CHARS,
        "QV pithy value must be nonempty, trimmed and <=128 characters",
    )?;
    require(
        !write.sources.is_empty(),
        "QV write requires source support",
    )?;
    write.producer.validate()?;
    write.result.validate()?;
    for source in &write.sources {
        source.validate()?;
    }

    let definition = qv_definition(field, &write, gate)?;
    let (coordinate_id, subject) = qv_subject(registry, &write.coordinate_ref)?;
    require(
        definition.applies_to.contains(&subject),
        "q_* definition is not applicable to the selected Bimba coordinate",
    )?;

    let draft = Draft {
        identity: write.assertion_identity,
        definition: write.definition.clone(),
        subject,
        value: Value::String(write.pithy),
        role: ValueRole::Quintessential,
        standing: write.standing,
        sources: write.sources,
        producer: write.producer.clone(),
        result: write.result.clone(),
        warrant: write.warrant,
        valid_from_unix_ms: write.valid_from_unix_ms,
        valid_until_unix_ms: write.valid_until_unix_ms,
        derivation: None,
    };
    let property_receipt = field.publish(operation, draft, expected, gate)?;

    Ok(QvReceipt {
        version: QV_CONTRACT.into(),
        coordinate_ref: write.coordinate_ref,
        coordinate_id,
        registry_revision: registry.manifest().registry_revision.clone(),
        q_position: q_position(&write.property_key)?,
        property_key: write.property_key,
        definition: write.definition,
        assertion: property_receipt.reference,
        producer: write.producer,
        result: write.result,
        admission: property_receipt.admission,
    })
}

/// Read one exact QV revision and prove that its receipt still names the same
/// coordinate, property definition and explicit producer/result provenance.
pub fn read_qv<G: NativePropertyAdmission>(
    registry: &MRegistry,
    field: &PropertyField<'_>,
    receipt: &QvReceipt,
    gate: &G,
) -> Result<QvReading> {
    require(
        receipt.version == QV_CONTRACT,
        "unsupported QV receipt contract",
    )?;
    require(
        receipt.registry_revision == registry.manifest().registry_revision,
        "QV receipt belongs to another registry revision",
    )?;
    require(
        q_position(&receipt.property_key)? == receipt.q_position,
        "QV receipt property/position mismatch",
    )?;
    let (coordinate_id, subject) = qv_subject(registry, &receipt.coordinate_ref)?;
    require(
        coordinate_id == receipt.coordinate_id,
        "QV coordinate identity drift",
    )?;

    let definition = field
        .definitions_named(&receipt.property_key, gate)?
        .into_iter()
        .find(|definition| definition.reference == receipt.definition)
        .cloned()
        .ok_or("QV definition revision is unavailable")?;
    require(
        definition.spec.key == receipt.property_key && definition.applies_to.contains(&subject),
        "QV definition no longer identifies the recorded property/subject relation",
    )?;

    let assertion = field.read(&receipt.assertion, gate)?;
    require(
        assertion.content.definition == receipt.definition
            && assertion.content.subject == subject
            && assertion.content.role == ValueRole::Quintessential
            && assertion.content.producer == receipt.producer
            && assertion.content.result == receipt.result,
        "QV receipt does not match its exact rich assertion",
    )?;
    let pithy = assertion
        .content
        .value
        .as_str()
        .ok_or("QV assertion is no longer a pithy string")?;
    require(
        !pithy.is_empty() && pithy.chars().count() <= QV_PITHY_MAX_CHARS,
        "QV assertion violates the historical pithy contract",
    )?;

    Ok(QvReading {
        receipt: receipt.clone(),
        pithy: pithy.into(),
        standing: assertion.content.standing,
        sources: assertion.content.sources.clone(),
        warrant: assertion.content.warrant.clone(),
    })
}

/// Bake exact QV readings into a deterministic generated-data artifact. This
/// corresponds to the historical `epi core knowing --bake` boundary while
/// leaving shared C materialisation to its native integration owner.
pub fn bake_qv<G: NativePropertyAdmission>(
    registry: &MRegistry,
    field: &PropertyField<'_>,
    receipts: &[QvReceipt],
    gate: &G,
) -> Result<QvBakeArtifact> {
    require(
        !receipts.is_empty(),
        "QV bake requires at least one reading",
    )?;
    let mut seen = BTreeSet::new();
    let mut entries = Vec::with_capacity(receipts.len());
    for receipt in receipts {
        require(
            seen.insert((receipt.coordinate_id, receipt.property_key.clone())),
            "QV bake contains duplicate coordinate/property identity",
        )?;
        let reading = read_qv(registry, field, receipt, gate)?;
        entries.push(QvBakeEntry {
            coordinate_ref: receipt.coordinate_ref.clone(),
            coordinate_id: receipt.coordinate_id,
            q_position: receipt.q_position,
            property_key: receipt.property_key.clone(),
            pithy: reading.pithy,
            assertion: receipt.assertion.clone(),
            producer: receipt.producer.clone(),
            result: receipt.result.clone(),
            admission: receipt.admission.clone(),
        });
    }
    entries.sort_by(|left, right| {
        left.coordinate_ref
            .cmp(&right.coordinate_ref)
            .then(left.q_position.cmp(&right.q_position))
            .then(left.property_key.cmp(&right.property_key))
    });
    Ok(QvBakeArtifact {
        version: QV_BAKE_CONTRACT.into(),
        registry_revision: registry.manifest().registry_revision.clone(),
        historical_source_repository: QV_SOURCE_REPOSITORY.into(),
        historical_source_revision: QV_SOURCE_REVISION.into(),
        historical_source_path: QV_DATA_SOURCE_PATH.into(),
        historical_source_blob: QV_DATA_SOURCE_BLOB.into(),
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m_tree::native_m_registry;
    use crate::property::{
        Cardinality, Definition, Disclosure, NativePropertyAdmission, PropertyField,
        SourcePropertySpec, SourceUse,
    };
    use std::cell::RefCell;

    fn pin(name: &str) -> SourcePin {
        SourcePin {
            reference: name.into(),
            revision: "r1".into(),
            use_mode: SourceUse::Current,
        }
    }

    struct Gate {
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
            Ok(pin("native/property/admission"))
        }
    }

    fn gate() -> Gate {
        Gate {
            stale: RefCell::new(None),
        }
    }

    fn subject() -> crate::property::Subject {
        let registry = native_m_registry();
        crate::property::Subject {
            registry_revision: registry.manifest().registry_revision.clone(),
            owner: PropertyOwner::Node,
            id: registry.root(0).unwrap().id,
        }
    }

    fn definition(key: &str) -> Definition {
        Definition {
            reference: VersionRef {
                identity: format!("property/{key}"),
                revision: 1,
            },
            declaration: pin("source/deep-property-map"),
            applicability_basis: pin("source/qv-applicability"),
            spec: SourcePropertySpec {
                key: key.into(),
                coordinate_home: "M0".into(),
                owner: PropertyOwner::Node,
                value_type: PropertyType::String,
                cardinality: Cardinality::One,
                disclosure: Disclosure::Internal,
                source_family: "quintessential-quick-view".into(),
                indexed: false,
                compatibility: false,
            },
            aliases: vec![],
            applies_to: vec![subject()],
            unit: None,
            domain: vec![],
            embedding_dimensions: None,
        }
    }

    fn write(key: &str, text: &str) -> QvWrite {
        QvWrite {
            coordinate_ref: "#0".into(),
            property_key: key.into(),
            definition: definition(key).reference,
            assertion_identity: format!("qv/#0/{key}"),
            pithy: text.into(),
            standing: Standing::SourceDeclared,
            sources: vec![pin("source/qv-data")],
            producer: pin("producer/epi-core-knowing"),
            result: pin("result/qv-update"),
            warrant: "source-qualified quintessential reading".into(),
            valid_from_unix_ms: None,
            valid_until_unix_ms: None,
        }
    }

    fn field<'a>(registry: &'a MRegistry, gate: &Gate, key: &str) -> PropertyField<'a> {
        let mut field = PropertyField::new(registry);
        field
            .define("define-qv", definition(key), None, gate)
            .unwrap();
        field
    }

    #[test]
    fn q_prefix_never_substitutes_for_a_canonical_definition_or_producer() {
        let registry = native_m_registry();
        let gate = gate();
        let mut field = PropertyField::new(registry);
        assert!(
            write_qv(
                registry,
                &mut field,
                "write",
                write("q_1_thesis", "ground"),
                None,
                &gate
            )
            .is_err()
        );

        let mut field = self::field(registry, &gate, "q_1_thesis");
        let mut missing_producer = write("q_1_thesis", "ground");
        missing_producer.producer.reference.clear();
        assert!(write_qv(registry, &mut field, "write", missing_producer, None, &gate).is_err());
        assert!(q_position("q_6_thesis").is_err());
        assert!(q_position("q_thesis").is_err());
    }

    #[test]
    fn qv_round_trips_through_the_existing_property_field() {
        let registry = native_m_registry();
        let gate = gate();
        let mut field = field(registry, &gate, "q_1_thesis");
        let receipt = write_qv(
            registry,
            &mut field,
            "write-qv",
            write(
                "q_1_thesis",
                "Ground remembered through an explicit source.",
            ),
            None,
            &gate,
        )
        .unwrap();
        let reading = read_qv(registry, &field, &receipt, &gate).unwrap();
        assert_eq!(
            reading.pithy,
            "Ground remembered through an explicit source."
        );
        assert_eq!(reading.receipt.q_position, 1);
        assert_eq!(
            field.read(&receipt.assertion, &gate).unwrap().content.role,
            ValueRole::Quintessential
        );
    }

    #[test]
    fn bake_is_deterministic_and_preserves_exact_property_evidence() {
        let registry = native_m_registry();
        let gate = gate();
        let mut field = PropertyField::new(registry);
        let first = definition("q_5_integration");
        let second = definition("q_1_thesis");
        field.define("def-5", first, None, &gate).unwrap();
        field.define("def-1", second, None, &gate).unwrap();
        let r5 = write_qv(
            registry,
            &mut field,
            "write-5",
            write("q_5_integration", "Return gathers the selected relation."),
            None,
            &gate,
        )
        .unwrap();
        let r1 = write_qv(
            registry,
            &mut field,
            "write-1",
            write("q_1_thesis", "Definition remains source-qualified."),
            None,
            &gate,
        )
        .unwrap();
        let baked = bake_qv(registry, &field, &[r5.clone(), r1.clone()], &gate).unwrap();
        assert_eq!(baked.version, QV_BAKE_CONTRACT);
        assert_eq!(baked.entries.len(), 2);
        assert_eq!(baked.entries[0].q_position, 1);
        assert_eq!(baked.entries[1].q_position, 5);
        assert_eq!(baked.entries[0].assertion, r1.assertion);
        assert!(bake_qv(registry, &field, &[r1.clone(), r1], &gate).is_err());
    }

    #[test]
    fn stale_source_blocks_read_and_bake_without_mutating_the_assertion() {
        let registry = native_m_registry();
        let gate = gate();
        let mut field = field(registry, &gate, "q_4_context");
        let receipt = write_qv(
            registry,
            &mut field,
            "write",
            write("q_4_context", "A contextual quick view."),
            None,
            &gate,
        )
        .unwrap();
        *gate.stale.borrow_mut() = Some("source/qv-data".into());
        assert!(read_qv(registry, &field, &receipt, &gate).is_err());
        assert!(bake_qv(registry, &field, std::slice::from_ref(&receipt), &gate).is_err());
        *gate.stale.borrow_mut() = None;
        assert_eq!(
            read_qv(registry, &field, &receipt, &gate).unwrap().pithy,
            "A contextual quick view."
        );
    }
}
