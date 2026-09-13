//! AW1 rich properties on the existing native Bimba node/relation identities.
//!
//! This is an operational value layer, not another coordinate/C registry or a
//! second graph service. Native source/graph owners retain persistence, grants,
//! transactions and retention. A field mutation below changes this owned value;
//! it does not write Neo4j, human source or canonical C vocabulary implicitly.
mod model;
pub use model::*;

use std::collections::{BTreeMap, BTreeSet};
use serde_json::{Value, json};
use crate::m_tree::MRegistry;
use model::{bounded, reference, require};

/// Implement with the existing native source/authority owner. Admission is a
/// read-only decision over the complete operation; it must not execute a write.
/// Current pins require current revision; Historical pins require exact retained
/// source. Merely echoing the requested pin is not an observation.
pub trait NativePropertyAdmission {
    fn check_source(&self, source: &SourcePin) -> Result<()>;
    fn admit(&self, operation: &str, payload: &Value) -> Result<SourcePin>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyReceipt {
    pub operation: String,
    pub reference: VersionRef,
    pub admission: SourcePin,
    pub replayed: bool,
}

/// The borrowed native registry alone decides structural existence. Definitions
/// retain source identities and explicit applicability; a q_* name never seats
/// a value in an office by spelling. The mutable collection is an in-process
/// rich graph value, suitable for native-owner persistence, not a new database.
pub struct PropertyField<'a> {
    registry: &'a MRegistry,
    definitions: BTreeMap<String, Vec<Definition>>,
    assertions: BTreeMap<String, Vec<Assertion>>,
    operations: BTreeMap<String, (Value, PropertyReceipt)>,
}
impl<'a> PropertyField<'a> {
    pub fn new(registry: &'a MRegistry) -> Self {
        Self { registry, definitions: BTreeMap::new(), assertions: BTreeMap::new(), operations: BTreeMap::new() }
    }
    fn admission<G: NativePropertyAdmission>(&self, operation: &str, payload: &Value, gate: &G) -> Result<SourcePin> {
        let result = gate.admit(operation, payload)?;
        result.validate()?;
        gate.check_source(&result)?;
        Ok(result)
    }
    fn replay<G: NativePropertyAdmission>(&self, operation: &str, payload: &Value, gate: &G) -> Result<Option<PropertyReceipt>> {
        reference(operation)?;
        if let Some((original, receipt)) = self.operations.get(operation) {
            require(original == payload, "operation identity reused for different property content")?;
            self.admission("read-operation", &json!({"operation": operation, "reference": receipt.reference}), gate)?;
            let mut result = receipt.clone();
            result.replayed = true;
            return Ok(Some(result));
        }
        require(self.operations.len() < 16_384, "property operation budget reached; native retention required")?;
        Ok(None)
    }
    fn remember(&mut self, operation: &str, payload: Value, receipt: PropertyReceipt) -> PropertyReceipt {
        self.operations.insert(operation.into(), (payload, receipt.clone()));
        receipt
    }
    fn definition(&self, reference: &VersionRef) -> Result<&Definition> {
        reference.validate()?;
        self.definitions.get(&reference.identity).and_then(|v| v.iter().find(|d| d.reference == *reference))
            .ok_or_else(|| "unknown property definition or definition revision".into())
    }
    fn assertion(&self, reference: &VersionRef) -> Result<&Assertion> {
        reference.validate()?;
        self.assertions.get(&reference.identity).and_then(|v| v.iter().find(|a| a.reference == *reference))
            .ok_or_else(|| "unknown assertion or assertion revision".into())
    }
    pub fn define<G: NativePropertyAdmission>(&mut self, operation: &str, definition: Definition, expected: Option<u64>, gate: &G) -> Result<PropertyReceipt> {
        let payload = json!({"kind":"define", "definition":definition, "expected":expected});
        if let Some(result) = self.replay(operation, &payload, gate)? { return Ok(result); }
        definition.validate(self.registry)?;
        let previous = self.definitions.get(&definition.reference.identity).and_then(|v| v.last());
        require(previous.map(|d| d.reference.revision) == expected, "stale property definition revision")?;
        require(definition.reference.revision == expected.unwrap_or(0).checked_add(1).ok_or("revision overflow")?,
            "property definition revision must advance once")?;
        if let Some(previous) = previous {
            require(previous.spec.key == definition.spec.key && previous.spec.owner == definition.spec.owner,
                "a stable property identity cannot be rebound to another key or owner kind")?;
        }
        for pin in [&definition.declaration, &definition.applicability_basis] { gate.check_source(pin)?; }
        let admission = self.admission("define", &payload, gate)?;
        let receipt = PropertyReceipt { operation: operation.into(), reference: definition.reference.clone(), admission, replayed: false };
        self.definitions.entry(definition.reference.identity.clone()).or_default().push(definition);
        Ok(self.remember(operation, payload, receipt))
    }
    fn validate_draft<G: NativePropertyAdmission>(&self, draft: &Draft, gate: &G) -> Result<()> {
        reference(&draft.identity)?;
        reference(&draft.warrant)?;
        draft.subject.validate(self.registry)?;
        let definition = self.definition(&draft.definition)?;
        require(self.definitions[&draft.definition.identity].last().map(|d| &d.reference) == Some(&draft.definition),
            "new assertion uses a superseded property definition")?;
        require(definition.applies_to.contains(&draft.subject), "property is not applicable to this exact subject")?;
        definition.validate_value(&draft.value)?;
        require(!draft.sources.is_empty() && draft.sources.len() <= MAX_REFERENCES, "property assertion needs bounded source support")?;
        require(draft.standing != Standing::Reviewed, "reviewed standing requires an explicit Recognition operation")?;
        if let (Some(a), Some(b)) = (draft.valid_from_unix_ms, draft.valid_until_unix_ms) {
            require(a <= b, "reversed property validity interval")?;
        }
        let mut pins = BTreeSet::new();
        for pin in draft.sources.iter().chain([&draft.producer, &draft.result, &definition.declaration, &definition.applicability_basis]) {
            pin.validate()?;
            gate.check_source(pin)?;
        }
        for pin in &draft.sources {
            require(pins.insert((&pin.reference, &pin.revision)), "duplicate property source support")?;
        }
        match (&draft.role, &draft.derivation) {
            (ValueRole::GraphDerived, Some(graph)) => {
                require(draft.standing == Standing::Derived || draft.standing == Standing::Proposed,
                    "graph-derived reading must retain derived/proposed origin")?;
                graph.graph.validate()?;
                graph.algorithm.validate()?;
                gate.check_source(&graph.graph)?;
                gate.check_source(&graph.algorithm)?;
                bounded(&graph.parameters)?;
                require(!graph.included_nodes.is_empty()
                    && graph.included_nodes.len() + graph.excluded_nodes.len() <= 16_384,
                    "graph derivation requires bounded explicit projection membership")?;
                let mut nodes = BTreeSet::new();
                for id in graph.included_nodes.iter().chain(&graph.excluded_nodes) {
                    require(self.registry.node(*id).is_some() && nodes.insert(*id),
                        "unknown, duplicate or included-and-excluded graph subject")?;
                }
                require(draft.subject.owner != PropertyOwner::Node || graph.included_nodes.contains(&draft.subject.id),
                    "derived subject is not inside its graph projection")?;
            }
            (ValueRole::GraphDerived, None) => return Err("graph-derived property requires actual graph/algorithm provenance".into()),
            (_, Some(_)) => return Err("graph derivation cannot be disguised as an intrinsic/quintessential value".into()),
            _ => {}
        }
        Ok(())
    }
    pub fn publish<G: NativePropertyAdmission>(&mut self, operation: &str, draft: Draft, expected: Option<u64>, gate: &G) -> Result<PropertyReceipt> {
        let payload = json!({"kind":"publish", "draft":draft, "expected":expected});
        if let Some(result) = self.replay(operation, &payload, gate)? { return Ok(result); }
        self.validate_draft(&draft, gate)?;
        let previous = self.assertions.get(&draft.identity).and_then(|v| v.last());
        require(previous.map(|a| a.reference.revision) == expected, "stale assertion revision")?;
        if let Some(previous) = previous {
            require(previous.content.subject == draft.subject
                && previous.content.definition.identity == draft.definition.identity,
                "assertion identity cannot move to a different subject or property")?;
        }
        let reference = VersionRef { identity: draft.identity.clone(), revision: expected.unwrap_or(0).checked_add(1).ok_or("revision overflow")? };
        let previous = previous.map(|a| a.reference.clone());
        let admission = self.admission("publish", &payload, gate)?;
        let assertion = Assertion { reference: reference.clone(), content: draft, previous,
            restored_from: None, recognition: None, admission: admission.clone() };
        self.assertions.entry(reference.identity.clone()).or_default().push(assertion);
        let receipt = PropertyReceipt { operation: operation.into(), reference, admission, replayed: false };
        Ok(self.remember(operation, payload, receipt))
    }
    /// Reviewing a value is not promotion of structural vocabulary into C. The
    /// original producer, graph derivation and all contrary assertions survive.
    pub fn recognise<G: NativePropertyAdmission>(&mut self, operation: &str, current: &VersionRef, recognition: SourcePin, gate: &G) -> Result<PropertyReceipt> {
        let payload = json!({"kind":"recognise", "current":current, "recognition":recognition});
        if let Some(result) = self.replay(operation, &payload, gate)? { return Ok(result); }
        let mut next = self.assertion(current)?.clone();
        require(self.assertions[&current.identity].last().map(|a| &a.reference) == Some(current), "Recognition uses a superseded assertion")?;
        require(next.content.standing != Standing::Reviewed, "assertion is already reviewed")?;
        self.validate_draft(&next.content, gate)?;
        recognition.validate()?;
        gate.check_source(&recognition)?;
        let admission = self.admission("recognise", &payload, gate)?;
        next.reference.revision = current.revision.checked_add(1).ok_or("revision overflow")?;
        next.previous = Some(current.clone());
        next.content.standing = Standing::Reviewed;
        next.recognition = Some(recognition);
        next.admission = admission.clone();
        let reference = next.reference.clone();
        self.assertions.get_mut(&current.identity).ok_or("assertion disappeared")?.push(next);
        let receipt = PropertyReceipt { operation: operation.into(), reference, admission, replayed: false };
        Ok(self.remember(operation, payload, receipt))
    }
    /// Restores content as a NEW proposed revision. Historical Recognition is
    /// retained on its old revision, never laundered into approval of a new act.
    pub fn rollback<G: NativePropertyAdmission>(&mut self, operation: &str, current: &VersionRef, restore: &VersionRef, gate: &G) -> Result<PropertyReceipt> {
        let payload = json!({"kind":"rollback", "current":current, "restore":restore});
        if let Some(result) = self.replay(operation, &payload, gate)? { return Ok(result); }
        require(current.identity == restore.identity && restore.revision < current.revision,
            "rollback must select an earlier revision of the same assertion")?;
        require(self.assertions.get(&current.identity).and_then(|v| v.last()).map(|a| &a.reference) == Some(current),
            "rollback expected revision is stale")?;
        let mut next = self.assertion(restore)?.clone();
        next.content.standing = Standing::Proposed;
        self.validate_draft(&next.content, gate)?;
        let admission = self.admission("rollback", &payload, gate)?;
        next.reference.revision = current.revision.checked_add(1).ok_or("revision overflow")?;
        next.previous = Some(current.clone());
        next.restored_from = Some(restore.clone());
        next.recognition = None;
        next.admission = admission.clone();
        let reference = next.reference.clone();
        self.assertions.get_mut(&current.identity).ok_or("assertion disappeared")?.push(next);
        let receipt = PropertyReceipt { operation: operation.into(), reference, admission, replayed: false };
        Ok(self.remember(operation, payload, receipt))
    }
    pub fn read<G: NativePropertyAdmission>(&self, reference: &VersionRef, gate: &G) -> Result<&Assertion> {
        let assertion = self.assertion(reference)?;
        self.admission("read", &json!({"assertion":assertion, "definition":self.definition(&assertion.content.definition)?}), gate)?;
        Ok(assertion)
    }
    fn compact_value(&self, assertion: &Assertion, maximum_bytes: usize) -> Result<Compact> {
        require(serde_json::to_vec(&assertion.content.value).map_err(|e| e.to_string())?.len() <= maximum_bytes,
            "compact value exceeds budget; open rich assertion instead of truncating it")?;
        Ok(Compact { version: PROPERTY_VERSION.into(), assertion: assertion.reference.clone(),
            definition: assertion.content.definition.clone(), subject: assertion.content.subject.clone(),
            value: assertion.content.value.clone(), role: assertion.content.role,
            standing: assertion.content.standing, support: assertion.content.result.clone() })
    }
    pub fn compact<G: NativePropertyAdmission>(&self, reference: &VersionRef, maximum_bytes: usize, gate: &G) -> Result<Compact> {
        self.compact_value(self.read(reference, gate)?, maximum_bytes)
    }
    pub fn expand<G: NativePropertyAdmission>(&self, compact: &Compact, gate: &G) -> Result<&Assertion> {
        let rich = self.read(&compact.assertion, gate)?;
        require(self.compact_value(rich, MAX_VALUE_BYTES)? == *compact, "compact property differs from its exact rich assertion")?;
        Ok(rich)
    }
    pub fn assertions_for<G: NativePropertyAdmission>(&self, subject: &Subject, gate: &G) -> Result<Vec<&Assertion>> {
        subject.validate(self.registry)?;
        self.admission("list", &json!({"subject":subject}), gate)?;
        self.assertions.values().filter_map(|v| v.last()).filter(|a| a.content.subject == *subject)
            .map(|a| self.read(&a.reference, gate)).collect()
    }
    /// Ambiguous aliases return every independently source-owned candidate.
    pub fn definitions_named<G: NativePropertyAdmission>(&self, name: &str, gate: &G) -> Result<Vec<&Definition>> {
        reference(name)?;
        let values: Vec<_> = self.definitions.values().filter_map(|v| v.last())
            .filter(|d| d.reference.identity == name || d.spec.key == name || d.aliases.iter().any(|a| a == name)).collect();
        for definition in &values { self.admission("read-definition", &json!(definition), gate)?; }
        Ok(values)
    }
}

#[cfg(test)]
mod tests;
