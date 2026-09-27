//! K3 executable M ledger. The K2 registry alone owns coordinate existence.
//!
//! Profiles share assessments, not capability identity. Missing profile entries
//! never mean COMPLETE; all five parity axes and all strata must be explicit.
//! Source assertions may be unresolved without disappearing. Implementations,
//! promoted canon and readiness claims, in contrast, must resolve and be warranted.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m_tree::{MRegistry, MTreeId, native_m_registry};

pub const M_LEDGER_SCHEMA: &str = "ql.m-ledger/v1";
pub const M_COVERAGE_SCHEMA: &str = "ql.m-coverage/v1";
pub const NATIVE_M_LEDGER: &str = include_str!("../../../fixtures/kernel/m-ledger-v1.json");
pub const AXES: [&str; 5] = [
    "source",
    "coordinate",
    "relation",
    "operational",
    "experiential",
];
pub const STRATA: [&str; 7] = [
    "source",
    "c",
    "rust",
    "cpp",
    "neo4j",
    "application",
    "instrument",
];
const PEERS: [&str; 4] = ["bimba", "c", "rust", "cpp"];
const BINDING_STRATA: [&str; 4] = ["c", "rust", "cpp", "neo4j"];
const READINESS: [&str; 6] = [
    "unassessed",
    "unimplemented",
    "structural-index-only",
    "partial",
    "implemented",
    "verified",
];
const WARRANTS: [&str; 6] = [
    "unassessed",
    "source-declared",
    "inferred",
    "implemented",
    "tested",
    "observed",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactLock {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryLock {
    pub path: String,
    pub sha256: String,
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixLock {
    pub id: String,
    pub data: ArtifactLock,
    pub rationale: ArtifactLock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub status: String,
    pub warrant: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityClaim {
    pub from_peer: String,
    pub to_peer: String,
    pub status: String,
    pub warrant: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub readiness: BTreeMap<String, Claim>,
    pub parity: BTreeMap<String, Vec<ParityClaim>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixSource {
    pub matrix: String,
    pub pointer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationClaim {
    pub relation_ref: String,
    pub standing: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LedgerRow {
    pub id: String,
    pub role: String,
    pub scope: String,
    /// Source assertions, not a private coordinate registry. Separators survive.
    pub coordinates: Vec<String>,
    pub source: Option<MatrixSource>,
    pub assessment: String,
    pub bindings: Vec<String>,
    /// Omitted stratum means unassessed, never not-applicable or complete.
    pub dispositions: BTreeMap<String, String>,
    pub invariants: Vec<String>,
    pub relations: Vec<RelationClaim>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructureAssertion {
    pub coordinate: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub relations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Implementation {
    pub id: String,
    pub stratum: String,
    pub kind: String,
    pub path: String,
    pub symbol: String,
    pub disposition: String,
    pub coordinates: Vec<String>,
    pub relations: Vec<String>,
    pub rationale: String,
    pub structure: Vec<StructureAssertion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub id: String,
    pub kind: String,
    pub artifact: ArtifactLock,
    pub registry_revision: String,
    pub subjects: Vec<String>,
    pub strata: Vec<String>,
    pub axes: Vec<String>,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authority {
    pub peer: String,
    pub reference: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resolution {
    pub target_peer: String,
    pub change: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub authority: Authority,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    pub state: String,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Discrepancy {
    pub id: String,
    pub subjects: Vec<String>,
    pub axis: String,
    pub from_peer: String,
    pub to_peer: String,
    pub state: String,
    pub detail: String,
    pub current_authority: Authority,
    pub proposal: Option<Resolution>,
    pub decision: Option<Decision>,
    /// Proposed structural-canon promotion does not become canon on its own.
    pub promotion: Option<String>,
    pub history: Vec<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MLedger {
    pub schema: String,
    pub ledger_revision: String,
    pub registry: RegistryLock,
    pub matrices: Vec<MatrixLock>,
    pub assessments: BTreeMap<String, Assessment>,
    pub rows: Vec<LedgerRow>,
    pub implementations: Vec<Implementation>,
    pub evidence: Vec<Evidence>,
    pub discrepancies: Vec<Discrepancy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub severity: String,
    pub code: String,
    pub subject: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct Coverage {
    pub schema: &'static str,
    pub ledger_revision: String,
    pub registry_revision: String,
    pub scope: String,
    pub stratum: String,
    pub axis: String,
    pub required_readiness: String,
    pub structural_coordinates: usize,
    pub structural_index_bindings: Vec<String>,
    pub coordinates_without_computational_binding: Vec<String>,
    pub coordinates_without_capability_rows: Vec<String>,
    pub rows: Vec<String>,
    pub blocking_rows: Vec<Finding>,
    pub source_without_implementation_disposition: Vec<String>,
    pub findings: Vec<Finding>,
}

fn finding(severity: &str, code: &str, subject: &str, detail: impl Into<String>) -> Finding {
    Finding {
        severity: severity.into(),
        code: code.into(),
        subject: subject.into(),
        detail: detail.into(),
    }
}

fn nonempty(s: &str) -> bool {
    !s.trim().is_empty()
}
fn hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn path(s: &str) -> bool {
    nonempty(s)
        && !s.starts_with('/')
        && !s.contains('\\')
        && !s.split('/').any(|x| x == ".." || x.is_empty())
}
fn set<'a>(items: impl Iterator<Item = &'a String>) -> BTreeSet<&'a str> {
    items.map(String::as_str).collect()
}

impl MLedger {
    /// Structural/semantic validation is independent of repository I/O. Checkout
    /// hashes and actual observed C/Rust descriptors are verified by m-ledger.py.
    /// This constructor does not authenticate external evidence or signatures.
    pub fn from_json(json: &str, registry: &MRegistry) -> Result<Self, String> {
        let ledger: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let errors = ledger
            .validate(registry)
            .into_iter()
            .filter(|f| f.severity == "error")
            .collect::<Vec<_>>();
        if errors.is_empty() {
            Ok(ledger)
        } else {
            Err(serde_json::to_string(&errors).map_err(|e| e.to_string())?)
        }
    }

    fn evidence_for(
        &self,
        ids: &[String],
        subject: &str,
        stratum: Option<&str>,
        axis: Option<&str>,
        executed: bool,
    ) -> bool {
        !ids.is_empty()
            && ids.iter().all(|id| {
                self.evidence.iter().any(|e| {
                    e.id == *id
                        && e.registry_revision == self.registry.revision
                        && e.subjects.iter().any(|s| s == subject)
                        && stratum.is_none_or(|s| e.strata.iter().any(|v| v == s))
                        && axis.is_none_or(|a| e.axes.iter().any(|v| v == a))
                        && if executed {
                            matches!(e.kind.as_str(), "test-receipt" | "observation")
                                && e.result == "passed"
                        } else {
                            matches!(e.result.as_str(), "present" | "passed" | "accepted")
                        }
                })
            })
    }

    fn row_bindings<'a>(
        &'a self,
        row: &'a LedgerRow,
        stratum: &'a str,
    ) -> impl Iterator<Item = &'a Implementation> {
        self.implementations
            .iter()
            .filter(move |i| i.stratum == stratum && row.bindings.contains(&i.id))
    }

    fn parity_for(&self, row: &LedgerRow, axis: &str, peer: &str) -> bool {
        self.assessments
            .get(&row.assessment)
            .and_then(|a| a.parity.get(axis))
            .is_some_and(|claims| {
                claims.iter().any(|p| {
                    p.status == "equivalent"
                        && (p.from_peer == peer || p.to_peer == peer)
                        && self.evidence_for(
                            &p.evidence,
                            &row.id,
                            Some(peer_stratum(peer)),
                            Some(axis),
                            true,
                        )
                })
            })
    }

    pub fn validate(&self, registry: &MRegistry) -> Vec<Finding> {
        let mut out = Vec::new();
        macro_rules! error {
            ($code:expr, $subject:expr, $detail:expr) => {
                out.push(finding("error", $code, $subject, $detail));
            };
        }
        if self.schema != M_LEDGER_SCHEMA
            || !hash(&self.ledger_revision)
            || !hash(&self.registry.sha256)
            || self.registry.revision != registry.manifest().registry_revision
            || !path(&self.registry.path)
        {
            error!(
                "ledger-contract",
                "ledger", "schema, digest shape or registry lock disagrees"
            );
        }
        for (name, ids) in [
            ("rows", self.rows.iter().map(|r| &r.id).collect::<Vec<_>>()),
            (
                "implementations",
                self.implementations.iter().map(|r| &r.id).collect(),
            ),
            ("evidence", self.evidence.iter().map(|r| &r.id).collect()),
            ("matrices", self.matrices.iter().map(|r| &r.id).collect()),
            (
                "discrepancies",
                self.discrepancies.iter().map(|r| &r.id).collect(),
            ),
        ] {
            if ids.iter().any(|id| !nonempty(id)) || set(ids.iter().copied()).len() != ids.len() {
                error!(
                    "duplicate-or-empty-id",
                    name, "identities must be nonempty and unique"
                );
            }
        }
        for matrix in &self.matrices {
            for artifact in [&matrix.data, &matrix.rationale] {
                if !path(&artifact.path) || !hash(&artifact.sha256) {
                    error!(
                        "matrix-lock",
                        &matrix.id, "invalid matrix/rationale artifact lock"
                    );
                }
            }
        }
        for e in &self.evidence {
            if !path(&e.artifact.path)
                || !hash(&e.artifact.sha256)
                || e.registry_revision != self.registry.revision
                || e.subjects.is_empty()
                || e.subjects.iter().any(|s| !nonempty(s) || s == "*")
                || ![
                    "source",
                    "implementation",
                    "test-receipt",
                    "observation",
                    "review",
                ]
                .contains(&e.kind.as_str())
                || !["present", "passed", "failed", "accepted", "rejected"]
                    .contains(&e.result.as_str())
                || e.strata.iter().any(|s| !STRATA.contains(&s.as_str()))
                || e.axes.iter().any(|a| !AXES.contains(&a.as_str()))
            {
                error!(
                    "evidence-contract",
                    &e.id, "invalid, unscoped or stale evidence"
                );
            }
        }
        for (id, assessment) in &self.assessments {
            if set(assessment.readiness.keys()) != STRATA.into_iter().collect()
                || set(assessment.parity.keys()) != AXES.into_iter().collect()
            {
                error!(
                    "assessment-axes",
                    id, "all seven strata and five parity axes must be explicit"
                );
            }
            for claim in assessment.readiness.values() {
                if !READINESS.contains(&claim.status.as_str())
                    || !WARRANTS.contains(&claim.warrant.as_str())
                {
                    error!(
                        "unsupported-completion-claim",
                        id, "unsupported readiness or warrant; no scalar COMPLETE state exists"
                    );
                }
            }
            for (axis, claims) in &assessment.parity {
                let mut seen = BTreeSet::new();
                for p in claims {
                    let mut pair = [p.from_peer.as_str(), p.to_peer.as_str()];
                    pair.sort();
                    if !PEERS.contains(&p.from_peer.as_str())
                        || !PEERS.contains(&p.to_peer.as_str())
                        || p.from_peer == p.to_peer
                        || !seen.insert(pair)
                        || !["unassessed", "different", "partial", "equivalent"]
                            .contains(&p.status.as_str())
                        || !WARRANTS.contains(&p.warrant.as_str())
                    {
                        error!(
                            "parity-contract",
                            id,
                            format!("invalid or conflicting pair on {axis}")
                        );
                    }
                }
            }
        }
        let row_ids = set(self.rows.iter().map(|r| &r.id));
        let impl_ids = set(self.implementations.iter().map(|i| &i.id));
        let used = self
            .rows
            .iter()
            .flat_map(|r| &r.bindings)
            .collect::<BTreeSet<_>>();
        for implementation in &self.implementations {
            let id = &implementation.id;
            if !BINDING_STRATA.contains(&implementation.stratum.as_str())
                || !["structural-index", "computational"].contains(&implementation.kind.as_str())
                || !path(&implementation.path)
                || !nonempty(&implementation.symbol)
            {
                error!(
                    "implementation-contract",
                    id, "invalid stratum, kind or source binding"
                );
            }
            let disposition_ok = match implementation.disposition.as_str() {
                "coordinate-bound" => !implementation.coordinates.is_empty(),
                "cross-coordinate" => {
                    implementation.coordinates.len() > 1 || !implementation.relations.is_empty()
                }
                "infrastructural" => {
                    implementation.coordinates.is_empty()
                        && implementation.relations.is_empty()
                        && nonempty(&implementation.rationale)
                }
                _ => false,
            };
            if !disposition_ok
                || (!used.contains(id) && implementation.disposition != "infrastructural")
            {
                error!(
                    "orphan-implementation",
                    id,
                    "construct has no valid coordinate/row or explicit infrastructure disposition"
                );
            }
            for coordinate in &implementation.coordinates {
                if registry.resolve(coordinate).is_none() {
                    error!(
                        "implementation-coordinate",
                        id,
                        format!("unresolved exact coordinate {coordinate}")
                    );
                }
            }
            for relation in &implementation.relations {
                if !registry
                    .manifest()
                    .relations
                    .iter()
                    .any(|r| r.relation_ref == *relation)
                {
                    error!(
                        "implementation-relation",
                        id,
                        format!("unresolved relation {relation}")
                    );
                }
            }
            for assertion in &implementation.structure {
                let Some(node) = registry.resolve(&assertion.coordinate) else {
                    error!(
                        "structural-disagreement",
                        id, "assertion has an unknown coordinate"
                    );
                    continue;
                };
                if !implementation
                    .coordinates
                    .iter()
                    .filter_map(|r| registry.resolve(r))
                    .any(|n| n.id == node.id)
                {
                    error!(
                        "structural-assertion-unbound",
                        id, "assertion must belong to this implementation's coordinate disposition"
                    );
                }
                let parent = assertion
                    .parent
                    .as_ref()
                    .and_then(|r| registry.resolve(r))
                    .map(|n| n.id);
                let children = assertion
                    .children
                    .iter()
                    .filter_map(|r| registry.resolve(r))
                    .map(|n| n.id)
                    .collect::<BTreeSet<_>>();
                let relations = registry
                    .relations_for(node.id)
                    .map(|r| r.relation_ref.as_str())
                    .collect::<BTreeSet<_>>();
                if assertion
                    .parent
                    .as_ref()
                    .is_some_and(|r| registry.resolve(r).is_none())
                    || parent != node.parent_id
                    || children.len() != assertion.children.len()
                    || children != node.children.iter().copied().collect()
                    || set(assertion.relations.iter()) != relations
                {
                    error!(
                        "structural-disagreement",
                        id,
                        format!(
                            "parent/children/relations disagree at {}",
                            assertion.coordinate
                        )
                    );
                }
            }
        }
        for row in &self.rows {
            if !nonempty(&row.role) || registry.resolve(&row.scope).is_none() {
                error!("row-scope", &row.id, "role or scope is missing/unresolved");
            }
            if let Some(source) = &row.source {
                if !self.matrices.iter().any(|m| m.id == source.matrix)
                    || !source.pointer.starts_with("/capabilities/")
                {
                    error!(
                        "source-binding",
                        &row.id, "source does not reference a locked capability matrix row"
                    );
                }
            }
            for coordinate in &row.coordinates {
                if registry.resolve(coordinate).is_none() {
                    out.push(finding(
                        "gap",
                        "unresolved-source-coordinate",
                        &row.id,
                        coordinate,
                    ));
                }
            }
            for binding in &row.bindings {
                if !impl_ids.contains(binding.as_str()) {
                    error!("missing-binding", &row.id, binding);
                }
                if let Some(implementation) = self.implementations.iter().find(|i| i.id == *binding)
                {
                    let row_nodes = row
                        .coordinates
                        .iter()
                        .filter_map(|r| registry.resolve(r))
                        .map(|n| n.id)
                        .collect::<BTreeSet<_>>();
                    if implementation.disposition != "infrastructural"
                        && !implementation
                            .coordinates
                            .iter()
                            .filter_map(|r| registry.resolve(r))
                            .any(|n| row_nodes.contains(&n.id))
                        && !implementation
                            .relations
                            .iter()
                            .any(|r| row.relations.iter().any(|c| c.relation_ref == *r))
                    {
                        error!("binding-coordinate-mismatch", &row.id, binding);
                    }
                }
            }
            for (stratum, disposition) in &row.dispositions {
                if !BINDING_STRATA.contains(&stratum.as_str())
                    || !["unassessed", "planned", "unimplemented", "bound"]
                        .contains(&disposition.as_str())
                    || (*disposition == "bound" && self.row_bindings(row, stratum).next().is_none())
                {
                    error!("implementation-disposition", &row.id, stratum);
                }
            }
            if row
                .dependencies
                .iter()
                .any(|id| !row_ids.contains(id.as_str()) || id == &row.id)
            {
                error!("dependency", &row.id, "missing or self dependency");
            }
            let Some(a) = self.assessments.get(&row.assessment) else {
                error!("missing-assessment", &row.id, &row.assessment);
                continue;
            };
            for (stratum, claim) in &a.readiness {
                let executed = claim.status == "verified";
                let asserted = !["unassessed", "unimplemented"].contains(&claim.status.as_str());
                if asserted
                    && (!self.evidence_for(
                        &claim.evidence,
                        &row.id,
                        Some(stratum),
                        if executed {
                            Some(readiness_axis(stratum))
                        } else {
                            None
                        },
                        executed,
                    ) || (executed
                        && !["tested", "observed"].contains(&claim.warrant.as_str())))
                {
                    error!(
                        "unsupported-readiness",
                        &row.id,
                        format!("{stratum}: {} lacks scoped evidence/warrant", claim.status)
                    );
                }
                if ["partial", "implemented"].contains(&claim.status.as_str())
                    && (!self.evidence_for(&claim.evidence, &row.id, Some(stratum), None, false)
                        || !["implemented", "tested", "observed"].contains(&claim.warrant.as_str())
                        || !claim.evidence.iter().all(|id| {
                            self.evidence.iter().any(|e| {
                                e.id == *id
                                    && ["implementation", "test-receipt", "observation"]
                                        .contains(&e.kind.as_str())
                            })
                        }))
                {
                    error!("unsupported-implementation-claim", &row.id, stratum);
                }
                if BINDING_STRATA.contains(&stratum.as_str()) && asserted {
                    let computational =
                        ["partial", "implemented", "verified"].contains(&claim.status.as_str());
                    if !self.row_bindings(row, stratum).any(|i| {
                        if computational {
                            i.kind == "computational"
                        } else {
                            i.kind == "structural-index"
                        }
                    }) {
                        error!(
                            "readiness-without-binding",
                            &row.id,
                            format!(
                                "{stratum}: index presence cannot warrant computational readiness"
                            )
                        );
                    }
                }
                if stratum == "neo4j"
                    && executed
                    && !claim.evidence.iter().any(|id| {
                        self.evidence
                            .iter()
                            .any(|e| e.id == *id && e.kind == "observation")
                    })
                {
                    error!(
                        "unobserved-live-graph",
                        &row.id, "serialized source is not live Neo4j evidence"
                    );
                }
            }
            for (axis, claims) in &a.parity {
                for p in claims {
                    let executed = p.status == "equivalent";
                    if p.status != "unassessed"
                        && (!self.evidence_for(&p.evidence, &row.id, None, Some(axis), executed)
                            || (executed
                                && (!["tested", "observed"].contains(&p.warrant.as_str())
                                    || !self.evidence_for(
                                        &p.evidence,
                                        &row.id,
                                        Some(peer_stratum(&p.from_peer)),
                                        Some(axis),
                                        true,
                                    )
                                    || !self.evidence_for(
                                        &p.evidence,
                                        &row.id,
                                        Some(peer_stratum(&p.to_peer)),
                                        Some(axis),
                                        true,
                                    ))))
                    {
                        error!(
                            "unsupported-parity",
                            &row.id,
                            format!("{axis}: {} <-> {}", p.from_peer, p.to_peer)
                        );
                    }
                }
            }
            for relation in &row.relations {
                if !["semantic", "research", "structural-canon"]
                    .contains(&relation.standing.as_str())
                {
                    error!("relation-standing", &row.id, &relation.standing);
                }
                if relation.standing == "structural-canon"
                    && (!registry
                        .manifest()
                        .relations
                        .iter()
                        .any(|r| r.relation_ref == relation.relation_ref)
                        || !self.parity_for(row, "relation", "c")
                        || !self
                            .row_bindings(row, "c")
                            .any(|i| i.relations.contains(&relation.relation_ref))
                        || !self.discrepancies.iter().any(|d| {
                            d.promotion.as_deref() == Some("structural-canon")
                                && ["accepted", "applied"].contains(&d.state.as_str())
                                && d.axis == "relation"
                                && d.subjects.contains(&row.id)
                        }))
                {
                    error!(
                        "promotion-without-c-parity",
                        &row.id, &relation.relation_ref
                    );
                }
            }
        }
        // Dependency cycles are rejected without recursion, including cycles that
        // do not touch the requested vertical. A blocked cycle cannot disappear.
        let mut pending = self
            .rows
            .iter()
            .map(|r| {
                (
                    r.id.as_str(),
                    r.dependencies
                        .iter()
                        .map(String::as_str)
                        .collect::<BTreeSet<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        loop {
            let ready = pending
                .iter()
                .filter(|(_, deps)| deps.is_empty())
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();
            if ready.is_empty() {
                break;
            }
            for id in &ready {
                pending.remove(id);
            }
            for deps in pending.values_mut() {
                for id in &ready {
                    deps.remove(id);
                }
            }
        }
        if !pending.is_empty() {
            error!(
                "dependency-cycle-or-missing",
                "ledger", "dependencies do not form an executable work graph"
            );
        }
        for d in &self.discrepancies {
            if !PEERS.contains(&d.from_peer.as_str())
                || !PEERS.contains(&d.to_peer.as_str())
                || d.from_peer == d.to_peer
                || !AXES.contains(&d.axis.as_str())
                || d.subjects.is_empty()
                || !nonempty(&d.detail)
                || !authority(&d.current_authority)
            {
                error!(
                    "discrepancy-contract",
                    &d.id, "invalid direction, axis, subject or current authority"
                );
            }
            for subject in &d.subjects {
                if !row_ids.contains(subject.as_str())
                    && registry.resolve(subject).is_none()
                    && !registry
                        .manifest()
                        .relations
                        .iter()
                        .any(|r| r.relation_ref == *subject)
                {
                    error!("discrepancy-subject", &d.id, subject);
                }
            }
            if d.history.first().is_none_or(|h| h.state != "open")
                || d.history.last().is_none_or(|h| h.state != d.state)
                || d.history.iter().any(|h| !nonempty(&h.reference))
                || d.history
                    .windows(2)
                    .any(|w| !transition(&w[0].state, &w[1].state))
            {
                error!(
                    "discrepancy-lifecycle",
                    &d.id,
                    "history must be a referenced, legal open -> proposal -> decision -> application lifecycle"
                );
            }
            if ["proposed", "accepted", "applied"].contains(&d.state.as_str())
                && d.proposal.as_ref().is_none_or(|p| {
                    !PEERS.contains(&p.target_peer.as_str()) || !nonempty(&p.change)
                })
            {
                error!(
                    "missing-resolution-proposal",
                    &d.id, "resolution must name the target peer and actual change"
                );
            }
            if ["accepted", "applied", "rejected"].contains(&d.state.as_str())
                && d.decision.as_ref().is_none_or(|decision| {
                    !authority(&decision.authority)
                        || !self.evidence_for(&decision.evidence, &d.id, None, None, false)
                })
            {
                error!(
                    "missing-resolution-decision",
                    &d.id, "resolution requires an evidenced decision by the named authority"
                );
            }
            if d.promotion.as_ref().is_some_and(|p| {
                p != "structural-canon" || !["coordinate", "relation"].contains(&d.axis.as_str())
            }) {
                error!("promotion-standing", &d.id, "unsupported promotion target");
            }
            if d.state == "applied"
                && d.proposal.as_ref().is_none_or(|p| {
                    !self.evidence_for(
                        &p.evidence,
                        &d.id,
                        Some(peer_stratum(&p.target_peer)),
                        Some(&d.axis),
                        true,
                    )
                })
            {
                error!(
                    "unproved-resolution-application",
                    &d.id, "application requires scoped evidence from the corrected peer"
                );
            }
            if d.promotion.is_some() && ["accepted", "applied"].contains(&d.state.as_str()) {
                for subject in &d.subjects {
                    if self
                        .rows
                        .iter()
                        .find(|r| r.id == *subject)
                        .is_none_or(|row| {
                            !self.parity_for(row, &d.axis, "c")
                                || self.row_bindings(row, "c").next().is_none()
                        })
                    {
                        error!("promotion-without-c-parity", &d.id, subject);
                    }
                }
            }
            if !["applied", "rejected"].contains(&d.state.as_str()) {
                out.push(finding("gap", "open-discrepancy", &d.id, &d.detail));
            }
        }
        out
    }

    /// Join the ledger row to K2 descriptors and retained source/matrix carriers.
    /// Parents, children, record provenance and relations are never copied into a
    /// divergent capability tree. Every returned coordinate is resolved by K2.
    pub fn coordinate_view(
        &self,
        registry: &MRegistry,
        reference: &str,
    ) -> Result<serde_json::Value, String> {
        let node = registry
            .resolve(reference)
            .ok_or_else(|| format!("unknown exact M coordinate: {reference}"))?;
        let mut rows = Vec::new();
        for row in &self.rows {
            if row
                .coordinates
                .iter()
                .filter_map(|r| registry.resolve(r))
                .any(|n| n.id == node.id)
                || (row.coordinates.is_empty()
                    && registry
                        .resolve(&row.scope)
                        .is_some_and(|n| n.id == node.id))
            {
                let matrix = row
                    .source
                    .as_ref()
                    .and_then(|s| self.matrices.iter().find(|m| m.id == s.matrix));
                let bindings = self
                    .implementations
                    .iter()
                    .filter(|i| row.bindings.contains(&i.id))
                    .collect::<Vec<_>>();
                rows.push(serde_json::json!({"row": row, "assessment": self.assessments.get(&row.assessment), "matrix": matrix, "bindings": bindings}));
            }
        }
        let source_records = node.records.iter().map(|index| {
            let record = &registry.manifest().records[*index];
            serde_json::json!({"index": index, "record": record, "file": &registry.manifest().files[record.file]})
        }).collect::<Vec<_>>();
        Ok(
            serde_json::json!({"schema": "ql.m-ledger-view/v1", "ledger_revision": self.ledger_revision,
            "registry_revision": self.registry.revision, "coordinate": node, "source_records": source_records,
            "relations": registry.relations_for(node.id).collect::<Vec<_>>(), "rows": rows}),
        )
    }

    pub fn coverage(
        &self,
        registry: &MRegistry,
        scope: &str,
        stratum: &str,
        axis: &str,
        required: &str,
    ) -> Result<Coverage, String> {
        if !STRATA.contains(&stratum)
            || !AXES.contains(&axis)
            || !["implemented", "verified"].contains(&required)
        {
            return Err("invalid coverage stratum, parity axis or required readiness".into());
        }
        let centre = registry
            .resolve(scope)
            .ok_or_else(|| format!("unknown exact M coordinate: {scope}"))?;
        let mut nodes = BTreeSet::new();
        let mut pending = vec![centre.id];
        while let Some(id) = pending.pop() {
            if nodes.insert(id) {
                pending.extend(registry.children(id).map(|n| n.id));
            }
        }
        let intersects = |coordinates: &[String]| {
            coordinates
                .iter()
                .filter_map(|r| registry.resolve(r))
                .any(|n| nodes.contains(&n.id))
        };
        let mut selected = self
            .rows
            .iter()
            .filter(|r| {
                intersects(&r.coordinates)
                    || registry
                        .resolve(&r.scope)
                        .is_some_and(|n| nodes.contains(&n.id))
            })
            .map(|r| r.id.as_str())
            .collect::<BTreeSet<_>>();
        loop {
            let before = selected.len();
            for row in &self.rows {
                if selected.contains(row.id.as_str()) {
                    selected.extend(row.dependencies.iter().map(String::as_str));
                }
            }
            if selected.len() == before {
                break;
            }
        }
        let rows = self
            .rows
            .iter()
            .filter(|r| selected.contains(r.id.as_str()))
            .collect::<Vec<_>>();
        let relevant = self
            .implementations
            .iter()
            .filter(|i| i.stratum == stratum && intersects(&i.coordinates))
            .collect::<Vec<_>>();
        let computational = relevant
            .iter()
            .filter(|i| i.kind == "computational")
            .flat_map(|i| &i.coordinates)
            .filter_map(|r| registry.resolve(r))
            .map(|n| n.id)
            .collect::<BTreeSet<_>>();
        let accounted = rows
            .iter()
            .filter(|r| r.source.is_some())
            .flat_map(|r| &r.coordinates)
            .filter_map(|r| registry.resolve(r))
            .map(|n| n.id)
            .collect::<BTreeSet<_>>();
        let refs_without = |covered: &BTreeSet<MTreeId>| {
            nodes
                .iter()
                .filter(|id| !covered.contains(id))
                .filter_map(|id| registry.node(*id))
                .map(|n| n.source_ref.clone())
                .collect::<Vec<_>>()
        };
        let mut blocking_rows = Vec::new();
        for row in &rows {
            let status = self
                .assessments
                .get(&row.assessment)
                .and_then(|a| a.readiness.get(stratum))
                .map(|c| c.status.as_str())
                .unwrap_or("unassessed");
            let ready =
                status == "verified" || (required == "implemented" && status == "implemented");
            // Bimba is the peer name for serialized source, not an alias for
            // live Neo4j. Strata without an explicit comparison peer stay
            // blocked; unrelated C/Rust parity cannot establish their parity.
            let peer = if stratum == "source" {
                "bimba"
            } else {
                stratum
            };
            if !ready || !self.parity_for(row, axis, peer) {
                blocking_rows.push(finding("gap", "vertical-blocker", &row.id, format!("{stratum} readiness={status}; required {required} and evidenced {axis} parity")));
            }
        }
        let source_without_implementation_disposition = rows
            .iter()
            .filter(|r| {
                r.source.is_some()
                    && r.dispositions
                        .get(stratum)
                        .is_none_or(|d| d == "unassessed")
            })
            .map(|r| r.id.clone())
            .collect();
        Ok(Coverage {
            schema: M_COVERAGE_SCHEMA,
            ledger_revision: self.ledger_revision.clone(),
            registry_revision: self.registry.revision.clone(),
            scope: centre.source_ref.clone(),
            stratum: stratum.into(),
            axis: axis.into(),
            required_readiness: required.into(),
            structural_coordinates: nodes.len(),
            structural_index_bindings: relevant
                .iter()
                .filter(|i| i.kind == "structural-index")
                .map(|i| i.id.clone())
                .collect(),
            coordinates_without_computational_binding: refs_without(&computational),
            coordinates_without_capability_rows: refs_without(&accounted),
            rows: rows.iter().map(|r| r.id.clone()).collect(),
            blocking_rows,
            source_without_implementation_disposition,
            // Global integrity errors and unscoped orphans remain visible even in
            // a narrow query; this prevents scope filtering from hiding defects.
            findings: self.validate(registry),
        })
    }
}

fn peer_stratum(peer: &str) -> &str {
    if peer == "bimba" { "source" } else { peer }
}
fn readiness_axis(stratum: &str) -> &str {
    match stratum {
        "source" => "source",
        "c" | "neo4j" => "coordinate",
        "instrument" => "experiential",
        _ => "operational",
    }
}
fn authority(a: &Authority) -> bool {
    PEERS.contains(&a.peer.as_str()) && nonempty(&a.reference) && nonempty(&a.reason)
}
fn transition(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("open", "proposed" | "rejected")
            | ("proposed", "accepted" | "rejected")
            | ("accepted", "applied")
    )
}

pub fn native_m_ledger() -> Result<MLedger, String> {
    MLedger::from_json(NATIVE_M_LEDGER, native_m_registry())
}
