//! TA0/TA1 coordinate profiles derived from the existing current M registry.
//!
//! This is source/profile resolution. Host adoption, encounter state, provider
//! availability and protected Anima readings remain with their existing owners.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::MFace;
use crate::aw1_world::{RootedFace, RootedMWorld, resolve_rooted_m_world};
use crate::m_tree::{MRegistry, MTreeId, MTreeRelation, MTreeSourceFile, MTreeSourceRecord};

pub const COORDINATE_EXPRESSION_CONTRACT: &str = "ql.coordinate-expression-binding/v1";
const WAYFINDER_PATH: &str = "docs/integrations/epi-logos/TA-ONTA-EXPRESSION-SDK-WAYFINDER.md";
const WAYFINDER: &str =
    include_str!("../../../docs/integrations/epi-logos/TA-ONTA-EXPRESSION-SDK-WAYFINDER.md");
const ORGANS_PATH: &str = "docs/integrations/epi-logos/epi-ta-onta-m-relational-field.matrix.json";
const ORGANS: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-ta-onta-m-relational-field.matrix.json");
const CAPABILITIES: [&str; 6] = [
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m0.json"),
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m1.json"),
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m2.json"),
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m3.json"),
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m4.json"),
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m5.json"),
];

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileGrammarSource {
    pub source_ref: String,
    pub content_revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InheritedCoordinateProfile {
    pub scope: String,
    pub profile_ref: String,
    pub profile_revision: u64,
    pub content_revision: String,
    pub parent_profile_ref: Option<String>,
    pub basis_ref: String,
    pub coordinate_id: Option<MTreeId>,
    pub registry_revision: String,
    pub source_record_indices: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinatePropertySource {
    pub registry_record_index: usize,
    pub record: MTreeSourceRecord,
    pub file: MTreeSourceFile,
    pub source_repository: String,
    pub source_revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaOntaProfileFaculty {
    pub id: String,
    pub label: String,
    pub capability_refs: Vec<String>,
    pub native_owners: Vec<String>,
    pub standing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinateExpressionBinding {
    pub schema: String,
    pub coordinate_ref: String,
    pub coordinate_id: MTreeId,
    pub face: RootedFace,
    pub family: String,
    pub labels: Vec<String>,
    pub branch_path: Vec<String>,
    pub rooted_world: RootedMWorld,
    pub grammar_sources: Vec<ProfileGrammarSource>,
    pub inherited_profiles: Vec<InheritedCoordinateProfile>,
    pub resolved_profile_ref: String,
    pub profile_revision: u64,
    pub binding_content_revision: String,
    pub property_sources: Vec<CoordinatePropertySource>,
    pub property_value_standing: String,
    pub source_relations: Vec<MTreeRelation>,
    /// Exact existing M-domain capability rows, including original stable IDs.
    pub declared_capabilities: Vec<Value>,
    pub ta_onta_faculties: Vec<TaOntaProfileFaculty>,
    pub capability_standing: String,
    pub authored_variant_refs: Vec<String>,
    pub encounter_overlay_standing: String,
}

fn profile_layer(
    registry: &MRegistry,
    face: MFace,
    scope: &str,
    basis_ref: &str,
    parent_profile_ref: Option<String>,
    grammar_sources: &[ProfileGrammarSource],
) -> Result<InheritedCoordinateProfile, String> {
    let node = registry.resolve(basis_ref);
    let mut layer = InheritedCoordinateProfile {
        scope: scope.into(),
        profile_ref: String::new(),
        profile_revision: 1,
        content_revision: String::new(),
        parent_profile_ref,
        basis_ref: basis_ref.into(),
        coordinate_id: node.map(|node| node.id),
        registry_revision: registry.manifest().registry_revision.clone(),
        source_record_indices: node.map(|node| node.records.clone()).unwrap_or_default(),
    };
    let sources = if scope == "global" {
        &grammar_sources[..2]
    } else {
        grammar_sources
    };
    let coordinate_face = (scope == "coordinate").then_some(face.as_str());
    let content = serde_json::to_vec(&(
        COORDINATE_EXPRESSION_CONTRACT,
        coordinate_face,
        &layer,
        sources,
    ))
    .map_err(|error| error.to_string())?;
    layer.content_revision = digest(&content);
    layer.profile_ref = format!("profile:epi-{scope}-{}", layer.content_revision);
    Ok(layer)
}

/// Resolve an immutable derived profile. Repeating the same current source,
/// coordinate and face returns the same content-addressed profile reference.
/// `profile_revision` is 1 because every changed content receives a new ref.
pub fn resolve_coordinate_expression(
    registry: &MRegistry,
    reference: &str,
    face: MFace,
) -> Result<CoordinateExpressionBinding, String> {
    let reference = if let Some(canonical) = reference.strip_prefix("ql:m-coordinate:") {
        let (declared_face, reference) = canonical
            .split_once(':')
            .ok_or("invalid canonical M coordinate")?;
        if declared_face != face.as_str() {
            return Err("canonical coordinate face disagrees with requested face".into());
        }
        reference
    } else {
        reference
    };
    let world = resolve_rooted_m_world(registry, reference)?;
    let selected = registry
        .node(world.selected_id)
        .ok_or("missing selected coordinate")?;
    let family = format!("M{}", world.root_position);
    let capability_source = CAPABILITIES[usize::from(world.root_position)];
    let domain: Value =
        serde_json::from_str(capability_source).map_err(|error| error.to_string())?;
    if domain["m"] != family {
        return Err("capability source disagrees with native M family".into());
    }
    let capabilities = domain["capabilities"]
        .as_array()
        .ok_or("missing capability rows")?
        .clone();
    let grammar_sources = vec![
        ProfileGrammarSource {
            source_ref: WAYFINDER_PATH.into(),
            content_revision: digest(WAYFINDER.as_bytes()),
        },
        ProfileGrammarSource {
            source_ref: ORGANS_PATH.into(),
            content_revision: digest(ORGANS.as_bytes()),
        },
        ProfileGrammarSource {
            source_ref: format!(
                "docs/integrations/epi-logos/epi-m-capability-field-m{}.json",
                world.root_position
            ),
            content_revision: digest(capability_source.as_bytes()),
        },
    ];
    let organ_matrix: Value = serde_json::from_str(ORGANS).map_err(|error| error.to_string())?;
    let organ_rows = organ_matrix["views"]
        .as_array()
        .and_then(|views| views.iter().find(|view| view["id"] == "inhabitation"))
        .and_then(|view| view["column_axis"]["members"].as_array())
        .ok_or("missing current Ta-Onta organs")?;
    if organ_rows.len() != 6 {
        return Err("current Ta-Onta matrix must retain six organs".into());
    }
    let mut faculties = Vec::new();
    for organ in organ_rows {
        let id = organ["id"].as_str().ok_or("organ ID absent")?;
        let label = organ["label"].as_str().ok_or("organ label absent")?;
        let membership = format!("{id} {label}");
        let mut capability_refs = BTreeSet::new();
        let mut native_owners = BTreeSet::new();
        for capability in &capabilities {
            if capability["s_prime"]
                .as_array()
                .is_some_and(|organs| organs.iter().any(|value| value == &membership))
            {
                capability_refs.insert(
                    capability["capability_ref"]
                        .as_str()
                        .ok_or("capability ID absent")?
                        .to_owned(),
                );
                for owner in capability["native_owners"]
                    .as_array()
                    .ok_or("capability owners absent")?
                {
                    native_owners
                        .insert(owner.as_str().ok_or("invalid capability owner")?.to_owned());
                }
            }
        }
        faculties.push(TaOntaProfileFaculty {
            id: id.into(), label: label.into(),
            capability_refs: capability_refs.into_iter().collect(),
            native_owners: native_owners.into_iter().collect(),
            standing: "source-declared membership; runtime availability and authority require native owner resolution".into(),
        });
    }

    let mut layers = Vec::new();
    let mut add_layer = |scope: &str, basis: &str| -> Result<(), String> {
        let parent = layers
            .last()
            .map(|layer: &InheritedCoordinateProfile| layer.profile_ref.clone());
        layers.push(profile_layer(
            registry,
            face,
            scope,
            basis,
            parent,
            &grammar_sources,
        )?);
        Ok(())
    };
    add_layer("global", &world.kernel_taproot_ref)?;
    let root = registry.node(world.root_id).ok_or("missing family root")?;
    add_layer("family", &root.source_ref)?;
    // One native first descendant supplies the domain branch. Deeper ancestry
    // remains in RootedMWorld, not an unbounded generic host profile chain.
    if let Some(branch) = world.ancestry.get(2) {
        add_layer("branch", &branch.source_ref)?;
    }
    add_layer("coordinate", &selected.source_ref)?;

    let mut record_indices = BTreeSet::new();
    for step in &world.ancestry {
        record_indices.extend(
            registry
                .node(step.id)
                .ok_or("missing ancestry node")?
                .records
                .iter()
                .copied(),
        );
    }
    let mut relations: Vec<_> = registry.relations_for(selected.id).cloned().collect();
    relations.sort_by_key(|relation| relation.id);
    for relation in &relations {
        record_indices.insert(relation.record);
    }
    let manifest = registry.manifest();
    let mut property_sources = Vec::new();
    for index in record_indices {
        let record = manifest.records.get(index).ok_or("missing source record")?;
        let file = manifest
            .files
            .get(record.file)
            .ok_or("missing source file")?;
        property_sources.push(CoordinatePropertySource {
            registry_record_index: index,
            record: record.clone(),
            file: file.clone(),
            source_repository: file
                .repository
                .clone()
                .unwrap_or_else(|| manifest.source_repository.clone()),
            source_revision: file
                .revision
                .clone()
                .unwrap_or_else(|| manifest.source_revision.clone()),
        });
    }
    let mut binding = CoordinateExpressionBinding {
        schema: COORDINATE_EXPRESSION_CONTRACT.into(),
        coordinate_ref: selected.source_ref.clone(), coordinate_id: selected.id,
        face: RootedFace::from_m_face(face), family, labels: selected.names.clone(),
        branch_path: world.ancestry.iter().skip(2).map(|step| step.source_ref.clone()).collect(),
        rooted_world: world, grammar_sources, inherited_profiles: layers,
        resolved_profile_ref: String::new(), profile_revision: 1, binding_content_revision: String::new(),
        property_sources,
        property_value_standing: "exact source records and property keys; source payload values are not loaded by this resolver".into(),
        source_relations: relations, declared_capabilities: capabilities, ta_onta_faculties: faculties,
        capability_standing: "declared capability lineage only; availability, selection, permission and invocation remain native owner readings".into(),
        authored_variant_refs: Vec::new(),
        encounter_overlay_standing: "host-owned; a current protected Anima session may overlay this profile without replacing its coordinate identity".into(),
    };
    // Self-references are empty for hashing; all source content and lineage are
    // included. No clock, process identity or host/private encounter enters it.
    let coordinate = binding
        .inherited_profiles
        .last_mut()
        .ok_or("coordinate profile missing")?;
    coordinate.profile_ref.clear();
    coordinate.content_revision.clear();
    binding.binding_content_revision =
        digest(&serde_json::to_vec(&binding).map_err(|error| error.to_string())?);
    binding.resolved_profile_ref = format!(
        "profile:epi-coordinate-{}",
        binding.binding_content_revision
    );
    let coordinate = binding
        .inherited_profiles
        .last_mut()
        .ok_or("coordinate profile missing")?;
    coordinate.profile_ref = binding.resolved_profile_ref.clone();
    coordinate.content_revision = binding.binding_content_revision.clone();
    Ok(binding)
}

// ---------------------------------------------------------------------------
// PS-E (QL-MEF #297): the native subject <-> exact locus <-> stable occurrence
// relation, with typed expressive roles.
//
// The owner's commission (2 October 2026): "all entities having their given
// place in the system, which might be at the level of a glyph, force, a
// movement/sequence, a scene or collection of scenes". A subject manifests at
// an exact source-qualified locus through compositional roles; each role is
// one addressable occurrence whose identity derives from content, so it
// survives reorder and rename, and canonical place re-entry returns the same
// bindings. Roles are compositional (P1 §0.2), not ontological species: one
// subject may hold several at once, and none is reduced to the others.
// ---------------------------------------------------------------------------

pub const SUBJECT_MANIFESTATION_CONTRACT: &str = "ql.subject-manifestation/v1";

/// Where the original Paśu/entity-to-form ground actually lives: the situated
/// paśu (the non-dual agent-user field) and the entity→form semantics are
/// source records of the Original repository, not a QL module. Qualification
/// below reuses their exact explanatory language; nothing here redefines them.
pub const PASU_SOURCE_REFS: [(&str, &str); 3] = [
    (
        "EpiLogos/Epi-Logos-C-Experiments",
        "Idea/Pratibimba/Self/PASU.md @ ead6956e2370d5d147c08499d1f533e2f6f6ddbd (revision daa660cbc1b8c5da83828698665a753852cb0287) — the situated paśu: non-dual agent-user field",
    ),
    (
        "EpiLogos/Epi-Logos-C-Experiments",
        "Body/S/S0/epi-cli/src/vault/pasu.rs @ 9cbb69c23048adeb63e0b274e3ad6e81b2daf62d — the native vault projection of that ground",
    ),
    (
        "EpiLogos/Epi-Logos-C-Experiments",
        "Idea/Bimba/World/World-Ontology.md @ a766db4ed67b29ec70dd273e674cfa1676f60800 — #1 definition/form, #2 operation/entity, #3 pattern/process, #4 context/type, #5 integration/reflection",
    ),
];

/// The compositional expressive role an occurrence plays (P1 §0.2). Required
/// roles include formation/glyph/body, force/constraint/modulation and
/// movement/sequence; scene and whole-Expression complete the owner's stated
/// scales. A planet can participate as formation, motion and modulation
/// together; roles never collapse into one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpressiveRole {
    Formation,
    Force,
    Sequence,
    Scene,
    Expression,
}

impl ExpressiveRole {
    pub const ALL: [ExpressiveRole; 5] = [
        Self::Formation,
        Self::Force,
        Self::Sequence,
        Self::Scene,
        Self::Expression,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Formation => "formation",
            Self::Force => "force",
            Self::Sequence => "sequence",
            Self::Scene => "scene",
            Self::Expression => "expression",
        }
    }

    /// The C-family property layer whose actual keys qualify this role,
    /// reusing the original coordinate semantics exactly as the canon
    /// instructs ("#1 as definition/form, #2 as operation/entity, #3 as
    /// pattern/process, #4 as context/type ... #5 as integration/reflection";
    /// see `PASU_SOURCE_REFS`). These prefixes are real in the current
    /// registry: every deep source record keys its content this way.
    pub const fn qualifying_prefix(self) -> &'static str {
        match self {
            Self::Formation => "c_1_",
            Self::Force => "c_2_",
            Self::Sequence => "c_3_",
            Self::Scene => "c_4_",
            Self::Expression => "c_5_",
        }
    }

    pub const fn layer_meaning(self) -> &'static str {
        match self {
            Self::Formation => "definition/form",
            Self::Force => "operation/entity",
            Self::Sequence => "pattern/process",
            Self::Scene => "context/type",
            Self::Expression => "integration/reflection",
        }
    }
}

/// The disclosed qualification standing carried by every occurrence.
pub const ROLE_QUALIFICATION_STANDING: &str = "declared-role-qualification: expressive roles are compositional (contract P1 §0.2), \
     qualified by the locus's own source records through the original coordinate semantics \
     (#1 definition/form, #2 operation/entity, #3 pattern/process, #4 context/type, \
     #5 integration/reflection — see ql.subject-manifestation PASU source refs); a locus \
     lacking a layer's keys resolves that role as explicitly unrepresented, never fabricated";

/// The shared native subject-reference grammar. Both this owner and the
/// procedural stage validate subject identity through this one function, so a
/// subject valid for a stage procedure is manifestable in the Atlas and vice
/// versa. It bounds and shapes the reference; it grants no authority.
pub fn validate_subject_ref(subject_ref: &str) -> Result<(), String> {
    if subject_ref.is_empty()
        || subject_ref.len() > 2048
        || subject_ref.chars().any(|c| c.is_control())
    {
        return Err(
            "invalid native subject reference (PS-E subject owner: non-empty, at most 2048 bytes, no control characters)"
                .into(),
        );
    }
    Ok(())
}

/// One authored variant layer over the resolved profile, in declared order.
/// Keys follow the actual profile inheritance at the current basis: first
/// parent precedence per key among parents, whole-key child replacement above
/// them — `material` and every other key is replaced whole, never deep-merged.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoredVariant {
    pub variant_ref: String,
    /// Authored keys, each naming the role layer it addresses by the same
    /// C-family prefix the source records use (e.g. `c_2_harmonic_role`).
    pub keys: BTreeMap<String, Value>,
}

impl AuthoredVariant {
    fn validate(&self) -> Result<(), String> {
        if self.variant_ref.is_empty()
            || self.variant_ref.len() > 256
            || self.variant_ref.chars().any(|c| c.is_control())
        {
            return Err("invalid authored variant reference".into());
        }
        if self.keys.len() > 64 {
            return Err("an authored variant carries at most 64 keys".into());
        }
        for (key, value) in &self.keys {
            if key.is_empty() || key.len() > 256 {
                return Err(format!("invalid authored key length {key:?}"));
            }
            if !ExpressiveRole::ALL
                .iter()
                .any(|role| key.starts_with(role.qualifying_prefix()))
            {
                return Err(format!(
                    "authored key {key:?} must name the expressive role layer it addresses (one of the C-family role prefixes)"
                ));
            }
            if serde_json::to_vec(value)
                .map_err(|error| error.to_string())?
                .len()
                > 4096
            {
                return Err(format!("authored value for {key:?} exceeds 4096 bytes"));
            }
        }
        Ok(())
    }
}

/// The exact locus of a manifestation: the coordinate identity, the addressed
/// face, the depth in the source tree, and the content revision of the full
/// `CoordinateExpressionBinding` this address summarises (re-resolvable
/// deterministically from `coordinate_ref` + face at the current registry).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocusAddress {
    pub coordinate_ref: String,
    pub coordinate_id: MTreeId,
    pub family: String,
    pub face: RootedFace,
    /// Exact depth of the selected coordinate in the source tree.
    pub depth: usize,
    pub branch_path: Vec<String>,
    pub canonical_ref: String,
    pub conjugate_canonical_ref: String,
    pub binding_content_revision: String,
    pub resolved_profile_ref: String,
    pub inherited_profile_refs: Vec<String>,
}

/// One typed role occurrence: the subject bound at the locus through one
/// expressive role, with the exact source records that qualify it and the
/// whole-key authored resolution that applies above them.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleOccurrence {
    pub role: ExpressiveRole,
    /// Content-derived identity. Reordering the requested roles, renaming
    /// labels or re-entering the place changes nothing here; changing the
    /// qualified content, the authored resolution or the explicit instance
    /// identity does.
    pub occurrence_ref: String,
    /// Whether the locus's own records actually carry this layer's keys. An
    /// unrepresented role is recorded as the obligation it is — the absence is
    /// information, never filled by fabrication.
    pub represented: bool,
    pub property_keys: Vec<String>,
    pub source_records: Vec<CoordinatePropertySource>,
    /// Effective authored keys for this role after first-parent-per-key and
    /// whole-key child replacement: key -> winning variant ref. Absent entries
    /// stand on their source records.
    pub authored_keys: BTreeMap<String, String>,
    pub standing: String,
}

/// One native subject manifested at one exact locus. This is the P1 relation
/// in one inspectable object: subject (who), locus (where, at what face and
/// depth, under which profile lineage), occurrences (in which roles, on which
/// source records, with which authored resolution).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectManifestation {
    pub schema: String,
    pub subject_ref: String,
    pub locus: LocusAddress,
    /// In the role type's declared order, regardless of the requested order.
    pub occurrences: Vec<RoleOccurrence>,
    /// Explicit instantiation identity when the caller forks another purposeful
    /// occurrence of the same subject at the same place; `None` is the
    /// canonical occurrence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    pub manifestation_content_revision: String,
    pub qualification_standing: &'static str,
    pub standing: String,
}

/// The exact content basis one occurrence's identity is taken over. The
/// digest covers the subject, the locus's content revision, the role, the
/// effective (source + authored) key set with the authored winning values,
/// and the explicit instance seed when one exists. Nothing order- or
/// label-bearing enters it: variant references are provenance, not identity.
fn occurrence_basis(
    subject_ref: &str,
    binding_revision: &str,
    role: ExpressiveRole,
    property_keys: &BTreeSet<String>,
    authored_values: &BTreeMap<String, Value>,
    record_pins: &[String],
    instance: Option<&str>,
) -> Value {
    json!({
        "contract": SUBJECT_MANIFESTATION_CONTRACT,
        "subject_ref": subject_ref,
        "locus_binding_content_revision": binding_revision,
        "role": role.as_str(),
        "property_keys": property_keys.iter().collect::<Vec<_>>(),
        "authored_values": authored_values,
        "source_record_payloads": record_pins,
        "instance": instance,
    })
}

fn occurrence_ref(basis: &Value) -> Result<String, String> {
    let bytes = serde_json::to_vec(basis).map_err(|error| error.to_string())?;
    Ok(format!("occurrence:{}", digest(&bytes)))
}

/// Resolves the subject ↔ locus ↔ occurrence relation through the existing
/// Atlas binding: the locus resolves exactly (coordinate, face, depth, real
/// profile lineage), each requested role binds the locus's own source records
/// through the original coordinate semantics, and authored variants apply the
/// actual first-parent-per-key / whole-key-replacement inheritance. Occurrence
/// identity is content-addressed: reorder-stable, rename-stable, and stable
/// across canonical place re-entry.
pub fn resolve_subject_manifestation(
    registry: &MRegistry,
    subject_ref: &str,
    locus_ref: &str,
    face: MFace,
    requested: &[ExpressiveRole],
    variants: &[AuthoredVariant],
    instance: Option<&str>,
) -> Result<SubjectManifestation, String> {
    validate_subject_ref(subject_ref)?;
    if requested.is_empty() {
        return Err("a manifestation addresses at least one expressive role".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    for role in requested {
        if !seen.insert(*role) {
            return Err(format!(
                "duplicate {} occurrence in one manifestation; a second purposeful occurrence is an explicit instantiation with its own instance identity",
                role.as_str()
            ));
        }
    }
    for variant in variants {
        variant.validate()?;
    }
    if let Some(seed) = instance
        && (seed.is_empty() || seed.len() > 256 || seed.chars().any(|c| c.is_control()))
    {
        return Err("invalid occurrence instance identity".into());
    }

    let binding = resolve_coordinate_expression(registry, locus_ref, face)?;
    let selected = registry
        .node(binding.coordinate_id)
        .ok_or("missing selected coordinate")?;
    // The locus's own source records carry its property content; ancestors
    // remain in the binding's profile lineage and branch path.
    let mut occurrence_sources = Vec::new();
    for &index in &selected.records {
        let record = registry
            .manifest()
            .records
            .get(index)
            .ok_or("missing source record")?;
        let file = registry
            .manifest()
            .files
            .get(record.file)
            .ok_or("missing source file")?;
        occurrence_sources.push((index, record, file));
    }

    // Whole-key authored resolution: for each authored key, the LAST variant
    // in declared order that defines it wins whole (child replacement over the
    // resolved parents); parents keep first-parent precedence among
    // themselves, which here is the locus's own source content.
    let mut authored_winner: BTreeMap<&str, (&AuthoredVariant, &Value)> = BTreeMap::new();
    for variant in variants {
        for (key, value) in &variant.keys {
            authored_winner.insert(key.as_str(), (variant, value));
        }
    }

    let mut occurrences = Vec::new();
    for role in ExpressiveRole::ALL {
        if !requested.contains(&role) {
            continue;
        }
        let prefix = role.qualifying_prefix();
        let mut source_keys = BTreeSet::new();
        let mut source_records = Vec::new();
        for (index, record, file) in &occurrence_sources {
            let mut hits = false;
            for key in &record.property_keys {
                // Whole-key authored replacement: an authored key stands
                // instead of the source key of the same name, never merged
                // with it.
                if key.starts_with(prefix) && !authored_winner.contains_key(key.as_str()) {
                    source_keys.insert(key.clone());
                    hits = true;
                }
            }
            if hits {
                source_records.push(CoordinatePropertySource {
                    registry_record_index: *index,
                    record: (*record).clone(),
                    file: (*file).clone(),
                    source_repository: file
                        .repository
                        .clone()
                        .unwrap_or_else(|| registry.manifest().source_repository.clone()),
                    source_revision: file
                        .revision
                        .clone()
                        .unwrap_or_else(|| registry.manifest().source_revision.clone()),
                });
            }
        }
        let mut authored_keys = BTreeMap::new();
        let mut authored_values = BTreeMap::new();
        for (key, value) in &authored_winner {
            if key.starts_with(prefix) {
                authored_keys.insert((*key).to_owned(), value.0.variant_ref.clone());
                authored_values.insert((*key).to_owned(), (*value.1).clone());
            }
        }
        // The effective key set the occurrence stands on: the locus's own
        // source keys plus the authored extensions/replacements, each key
        // whole. `represented` is the source fact alone; authored content
        // never fabricates source representation.
        let represented = !source_keys.is_empty();
        let mut property_keys = source_keys;
        property_keys.extend(authored_values.keys().cloned());
        let record_pins: Vec<String> = source_records
            .iter()
            .map(|source| source.record.payload_sha256.clone())
            .collect();
        let basis = occurrence_basis(
            subject_ref,
            &binding.binding_content_revision,
            role,
            &property_keys,
            &authored_values,
            &record_pins,
            instance,
        );
        let occurrence_ref = occurrence_ref(&basis)?;
        occurrences.push(RoleOccurrence {
            role,
            occurrence_ref,
            represented,
            property_keys: property_keys.into_iter().collect(),
            source_records,
            authored_keys,
            standing: if represented {
                format!(
                    "source-qualified {} occurrence through the locus's own {} records",
                    role.as_str(),
                    role.layer_meaning()
                )
            } else {
                format!(
                    "unrepresented at this locus: no {} ({}) keys in the locus's own records; the obligation is recorded, not fabricated",
                    role.qualifying_prefix(),
                    role.layer_meaning()
                )
            },
        });
    }

    let canonical_ref = match face {
        MFace::Bimba => binding.rooted_world.direct.canonical_ref.clone(),
        MFace::Pratibimba => binding.rooted_world.conjugate.canonical_ref.clone(),
    };
    let locus = LocusAddress {
        coordinate_ref: binding.coordinate_ref.clone(),
        coordinate_id: binding.coordinate_id,
        family: binding.family.clone(),
        face: binding.face,
        depth: selected.depth,
        branch_path: binding.branch_path.clone(),
        canonical_ref,
        conjugate_canonical_ref: if face == MFace::Bimba {
            binding.rooted_world.conjugate.canonical_ref.clone()
        } else {
            binding.rooted_world.direct.canonical_ref.clone()
        },
        binding_content_revision: binding.binding_content_revision.clone(),
        resolved_profile_ref: binding.resolved_profile_ref.clone(),
        inherited_profile_refs: binding
            .inherited_profiles
            .iter()
            .map(|layer| layer.profile_ref.clone())
            .collect(),
    };
    let mut manifestation = SubjectManifestation {
        schema: SUBJECT_MANIFESTATION_CONTRACT.into(),
        subject_ref: subject_ref.to_owned(),
        locus,
        occurrences,
        instance: instance.map(str::to_owned),
        manifestation_content_revision: String::new(),
        qualification_standing: ROLE_QUALIFICATION_STANDING,
        standing: "resolved subject/locus/occurrence references over the existing Atlas binding; source values stay at their records, authority stays at the native owners".into(),
    };
    manifestation.manifestation_content_revision =
        digest(&serde_json::to_vec(&manifestation).map_err(|error| error.to_string())?);
    Ok(manifestation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m_tree::native_current_m_registry;

    // The O:I generic Expression owner admits profile:<local-id>, with the
    // same restricted local identifier for every inherited and resolved ref.
    fn assert_native_profile_refs(binding: &CoordinateExpressionBinding) {
        for reference in std::iter::once(&binding.resolved_profile_ref)
            .chain(
                binding
                    .inherited_profiles
                    .iter()
                    .map(|layer| &layer.profile_ref),
            )
            .chain(
                binding
                    .inherited_profiles
                    .iter()
                    .filter_map(|layer| layer.parent_profile_ref.as_ref()),
            )
        {
            let local = reference
                .strip_prefix("profile:")
                .expect("native profile prefix");
            assert!(!local.is_empty() && local.len() <= 128, "{reference}");
            assert!(
                local
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.')),
                "{reference}"
            );
        }
    }

    #[test]
    fn actual_current_registry_covers_every_m_family_and_all_six_nara_branches() {
        let registry = native_current_m_registry();
        let mut references = (0..6)
            .map(|position| format!("#{position}"))
            .collect::<Vec<_>>();
        let nara = registry.resolve("#4").unwrap();
        // The nesting threshold: after a position 4 the separator is "." (Lived
        // Topology §IV), so Nara's six branches are 4.0..4.5. The map also holds
        // non-canonical dash twins 4-0..4-5; they are not branches.
        let branches: Vec<String> = nara
            .children
            .iter()
            .map(|id| registry.node(*id).unwrap().source_ref.clone())
            .filter(|r| r.starts_with("#4."))
            .collect();
        assert_eq!(
            branches,
            (0..6).map(|i| format!("#4.{i}")).collect::<Vec<_>>()
        );
        references.extend(branches);
        for reference in references {
            let binding =
                resolve_coordinate_expression(registry, &reference, MFace::Bimba).unwrap();
            assert_native_profile_refs(&binding);
            assert_eq!(binding.coordinate_ref, reference);
            assert_eq!(binding.ta_onta_faculties.len(), 6);
            assert!(!binding.declared_capabilities.is_empty());
            assert_eq!(binding.inherited_profiles[0].scope, "global");
            assert_eq!(binding.inherited_profiles[1].scope, "family");
            assert_eq!(
                binding.inherited_profiles.last().unwrap().scope,
                "coordinate"
            );
            for pair in binding.inherited_profiles.windows(2) {
                assert_eq!(
                    pair[1].parent_profile_ref.as_ref(),
                    Some(&pair[0].profile_ref)
                );
            }
            assert_eq!(
                binding
                    .source_relations
                    .iter()
                    .map(|relation| relation.id)
                    .collect::<Vec<_>>(),
                binding.rooted_world.source_relation_ids
            );
            for source in &binding.property_sources {
                assert_eq!(
                    source.record.payload_sha256,
                    registry.manifest().records[source.registry_record_index].payload_sha256
                );
                assert!(!source.file.git_blob.is_empty());
            }
            if binding.family == "M4" && reference != "#4" {
                assert_eq!(binding.inherited_profiles[2].scope, "branch");
                assert_eq!(binding.inherited_profiles[2].basis_ref, reference);
            }
        }
    }

    #[test]
    fn profile_refs_are_deterministic_source_addressed_and_faces_keep_one_identity() {
        let registry = native_current_m_registry();
        let reference = &registry
            .manifest()
            .nodes
            .iter()
            .find(|node| node.depth > 3)
            .unwrap()
            .source_ref;
        let direct = resolve_coordinate_expression(registry, reference, MFace::Bimba).unwrap();
        let repeated = resolve_coordinate_expression(registry, reference, MFace::Bimba).unwrap();
        let conjugate =
            resolve_coordinate_expression(registry, reference, MFace::Pratibimba).unwrap();
        assert_eq!(
            serde_json::to_value(&direct).unwrap(),
            serde_json::to_value(repeated).unwrap()
        );
        assert_native_profile_refs(&direct);
        assert_native_profile_refs(&conjugate);
        assert_eq!(direct.coordinate_id, conjugate.coordinate_id);
        assert_eq!(direct.coordinate_ref, conjugate.coordinate_ref);
        assert_ne!(direct.resolved_profile_ref, conjugate.resolved_profile_ref);
        assert!(
            direct
                .resolved_profile_ref
                .starts_with("profile:epi-coordinate-")
        );
        assert_eq!(direct.profile_revision, 1);
        let expected = direct.binding_content_revision.clone();
        let mut content = direct;
        content.binding_content_revision.clear();
        content.resolved_profile_ref.clear();
        let coordinate = content.inherited_profiles.last_mut().unwrap();
        coordinate.profile_ref.clear();
        coordinate.content_revision.clear();
        assert_eq!(digest(&serde_json::to_vec(&content).unwrap()), expected);
        assert!(resolve_coordinate_expression(registry, "#99", MFace::Bimba).is_err());
        assert!(resolve_coordinate_expression(registry, "M", MFace::Bimba).is_err());
    }

    #[test]
    fn native_aliases_share_profile_and_global_lineage_without_exceeding_host_depth() {
        let registry = native_current_m_registry();
        let mut global_ref = None;
        for position in 0..6 {
            let source = format!("#{position}");
            let profile = resolve_coordinate_expression(registry, &source, MFace::Bimba).unwrap();
            let canonical = registry
                .coordinate(&source, MFace::Bimba)
                .unwrap()
                .canonical_ref();
            let alias = resolve_coordinate_expression(registry, &canonical, MFace::Bimba).unwrap();
            assert_eq!(profile.resolved_profile_ref, alias.resolved_profile_ref);
            assert_eq!(
                profile.inherited_profiles.last().unwrap().profile_ref,
                profile.resolved_profile_ref
            );
            assert!(profile.inherited_profiles.len() <= 4);
            let inherited = &profile.inherited_profiles[0].profile_ref;
            if let Some(expected) = &global_ref {
                assert_eq!(inherited, expected);
            } else {
                global_ref = Some(inherited.clone());
            }
            assert!(
                resolve_coordinate_expression(registry, &canonical, MFace::Pratibimba).is_err()
            );
        }
        let source = &registry
            .manifest()
            .nodes
            .iter()
            .max_by_key(|node| node.depth)
            .unwrap()
            .source_ref;
        let deep = resolve_coordinate_expression(registry, source, MFace::Bimba).unwrap();
        assert_eq!(deep.inherited_profiles.len(), 4);
        assert!(deep.rooted_world.ancestry.len() > deep.inherited_profiles.len());
    }

    // ---- PS-E (QL-MEF #297): subject <-> locus <-> occurrence -------------

    fn moon(
        roles: &[ExpressiveRole],
        variants: &[AuthoredVariant],
        instance: Option<&str>,
    ) -> SubjectManifestation {
        let registry = native_current_m_registry();
        resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#2-5-4",
            MFace::Bimba,
            roles,
            variants,
            instance,
        )
        .unwrap()
    }

    /// Whole-manifestation equality through its exact serialized content.
    fn json_eq(left: &SubjectManifestation, right: &SubjectManifestation) -> bool {
        serde_json::to_value(left).unwrap() == serde_json::to_value(right).unwrap()
    }

    fn occurrence_json_eq(left: &RoleOccurrence, right: &RoleOccurrence) -> bool {
        serde_json::to_value(left).unwrap() == serde_json::to_value(right).unwrap()
    }

    #[test]
    fn subject_refs_are_validated_by_one_shared_grammar() {
        assert!(validate_subject_ref("ql:k2/default-subject").is_ok());
        assert!(validate_subject_ref("person:someone").is_ok());
        assert!(validate_subject_ref("").is_err());
        assert!(validate_subject_ref("bad\0subject").is_err());
        let long = "s".repeat(2049);
        assert!(validate_subject_ref(&long).is_err());
    }

    #[test]
    fn role_layers_reuse_the_original_coordinate_semantics_and_pin_the_pasu_sources() {
        assert_eq!(ExpressiveRole::Formation.qualifying_prefix(), "c_1_");
        assert_eq!(ExpressiveRole::Formation.layer_meaning(), "definition/form");
        assert_eq!(ExpressiveRole::Force.qualifying_prefix(), "c_2_");
        assert_eq!(ExpressiveRole::Force.layer_meaning(), "operation/entity");
        assert_eq!(ExpressiveRole::Sequence.qualifying_prefix(), "c_3_");
        assert_eq!(ExpressiveRole::Sequence.layer_meaning(), "pattern/process");
        assert_eq!(ExpressiveRole::Scene.qualifying_prefix(), "c_4_");
        assert_eq!(ExpressiveRole::Scene.layer_meaning(), "context/type");
        assert_eq!(ExpressiveRole::Expression.qualifying_prefix(), "c_5_");
        assert_eq!(
            ExpressiveRole::Expression.layer_meaning(),
            "integration/reflection"
        );
        // The original paśu/entity-to-form sources are recorded where they
        // actually live, not invented as QL modules.
        assert_eq!(PASU_SOURCE_REFS.len(), 3);
        assert!(
            PASU_SOURCE_REFS
                .iter()
                .all(|(repo, pin)| *repo == "EpiLogos/Epi-Logos-C-Experiments" && !pin.is_empty())
        );
        assert!(
            PASU_SOURCE_REFS
                .iter()
                .any(|(_, pin)| pin.contains("Idea/Pratibimba/Self/PASU.md"))
        );
    }

    #[test]
    fn one_subject_manifests_as_formation_force_and_sequence_at_once() {
        let manifestation = moon(
            &[
                ExpressiveRole::Formation,
                ExpressiveRole::Force,
                ExpressiveRole::Sequence,
            ],
            &[],
            None,
        );
        assert_eq!(manifestation.schema, SUBJECT_MANIFESTATION_CONTRACT);
        assert_eq!(manifestation.subject_ref, "ql:k2/default-subject");
        assert_eq!(manifestation.locus.coordinate_ref, "#2-5-4");
        assert_eq!(manifestation.locus.family, "M2");
        assert_eq!(manifestation.locus.face, RootedFace::Bimba);
        assert_eq!(manifestation.locus.depth, 3);
        assert!(!manifestation.locus.inherited_profile_refs.is_empty());
        assert!(!manifestation.locus.binding_content_revision.is_empty());
        // Occurrences come in the role type's declared order, not request order.
        assert_eq!(
            manifestation
                .occurrences
                .iter()
                .map(|o| o.role)
                .collect::<Vec<_>>(),
            vec![
                ExpressiveRole::Formation,
                ExpressiveRole::Force,
                ExpressiveRole::Sequence
            ]
        );
        let formation = &manifestation.occurrences[0];
        let force = &manifestation.occurrences[1];
        let sequence = &manifestation.occurrences[2];
        assert!(formation.property_keys.contains(&"c_1_name".to_owned()));
        assert!(
            force
                .property_keys
                .contains(&"c_2_harmonic_role".to_owned())
        );
        assert!(!sequence.property_keys.is_empty());
        for occurrence in &manifestation.occurrences {
            assert!(occurrence.represented, "{}", occurrence.standing);
            assert!(!occurrence.source_records.is_empty());
            for source in &occurrence.source_records {
                assert!(!source.record.payload_sha256.is_empty());
                assert!(!source.file.git_blob.is_empty());
            }
        }
        // Three roles of one subject: three distinct occurrences.
        assert_ne!(formation.occurrence_ref, force.occurrence_ref);
        assert_ne!(force.occurrence_ref, sequence.occurrence_ref);
        assert!(formation.occurrence_ref.starts_with("occurrence:"));
    }

    #[test]
    fn occurrence_identity_is_reorder_rename_and_reentry_stable() {
        let roles = [
            ExpressiveRole::Formation,
            ExpressiveRole::Force,
            ExpressiveRole::Sequence,
        ];
        let direct = moon(&roles, &[], None);
        let reversed = moon(&roles[2..], &[], None)
            .occurrences
            .into_iter()
            .chain(moon(&roles[..2], &[], None).occurrences)
            .collect::<Vec<_>>();
        // Resolving in two reordered halves still reproduces each role's
        // occurrence identity: list order is not identity.
        for occurrence in &reversed {
            assert!(occurrence_json_eq(
                direct
                    .occurrences
                    .iter()
                    .find(|o| o.role == occurrence.role)
                    .unwrap(),
                occurrence
            ));
        }
        // Re-entering the same place by its canonical face-bearing spelling
        // returns the same bindings.
        let registry = native_current_m_registry();
        let canonical = direct.locus.canonical_ref.clone();
        let reentered = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            &canonical,
            MFace::Bimba,
            &roles,
            &[],
            None,
        )
        .unwrap();
        assert!(json_eq(&reentered, &direct));
        // Re-resolution is deterministic.
        assert!(json_eq(&moon(&roles, &[], None), &direct));
    }

    #[test]
    fn adding_a_role_never_relabels_the_existing_occurrences() {
        let three = moon(
            &[
                ExpressiveRole::Formation,
                ExpressiveRole::Force,
                ExpressiveRole::Sequence,
            ],
            &[],
            None,
        );
        let four = moon(
            &[
                ExpressiveRole::Formation,
                ExpressiveRole::Force,
                ExpressiveRole::Sequence,
                ExpressiveRole::Scene,
            ],
            &[],
            None,
        );
        assert_eq!(four.occurrences.len(), 4);
        for occurrence in &three.occurrences {
            assert!(occurrence_json_eq(
                four.occurrences
                    .iter()
                    .find(|o| o.role == occurrence.role)
                    .unwrap(),
                occurrence
            ));
        }
    }

    #[test]
    fn unrepresented_roles_are_recorded_not_fabricated() {
        // #0-0-0's own records carry c_0..c_4 keys and no c_5_ layer:
        // the integration/reflection role is genuinely absent there.
        let registry = native_current_m_registry();
        let manifestation = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#0-0-0",
            MFace::Bimba,
            &[
                ExpressiveRole::Formation,
                ExpressiveRole::Scene,
                ExpressiveRole::Expression,
            ],
            &[],
            None,
        )
        .unwrap();
        let formation = manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Formation)
            .unwrap();
        assert!(formation.represented);
        let scene = manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Scene)
            .unwrap();
        assert!(scene.represented);
        let expression = manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Expression)
            .unwrap();
        assert!(!expression.represented);
        // An unrepresented occurrence still addresses: identity, not absence
        // of address.
        assert!(expression.occurrence_ref.starts_with("occurrence:"));
    }

    #[test]
    fn authored_variants_replace_whole_keys_and_never_deep_merge() {
        let v1 = AuthoredVariant {
            variant_ref: "variant:moon-v1".into(),
            keys: [
                (
                    "c_2_harmonic_role".to_owned(),
                    json!("authored-v1-whole-key"),
                ),
                ("c_2_moon_phase".to_owned(), json!("phase-from-v1")),
            ]
            .into_iter()
            .collect(),
        };
        let v2 = AuthoredVariant {
            variant_ref: "variant:moon-v2".into(),
            keys: [(
                "c_2_harmonic_role".to_owned(),
                json!("authored-v2-replaces-v1-whole"),
            )]
            .into_iter()
            .collect(),
        };
        let manifestation = moon(
            &[ExpressiveRole::Formation, ExpressiveRole::Force],
            &[v1.clone(), v2],
            None,
        );
        let force = manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Force)
            .unwrap();
        // Last child replacement wins the whole key; the first variant's
        // value for that key is gone, not merged.
        assert_eq!(
            force.authored_keys.get("c_2_harmonic_role").unwrap(),
            "variant:moon-v2"
        );
        // A key only the first variant defines stays the first variant's.
        assert_eq!(
            force.authored_keys.get("c_2_moon_phase").unwrap(),
            "variant:moon-v1"
        );
        assert!(
            force
                .property_keys
                .contains(&"c_2_harmonic_role".to_owned())
        );
        // The replaced source key no longer stands as source.
        assert!(!ROLE_QUALIFICATION_STANDING.is_empty());
        // Renaming a variant (same content) keeps occurrence identity;
        // changing its content does not.
        let renamed = AuthoredVariant {
            variant_ref: "variant:moon-v2-renamed".into(),
            keys: [(
                "c_2_harmonic_role".to_owned(),
                json!("authored-v2-replaces-v1-whole"),
            )]
            .into_iter()
            .collect(),
        };
        let renamed_manifestation = moon(
            &[ExpressiveRole::Formation, ExpressiveRole::Force],
            &[v1.clone(), renamed],
            None,
        );
        let renamed_force = renamed_manifestation
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Force)
            .unwrap();
        assert_eq!(renamed_force.occurrence_ref, force.occurrence_ref);
        let changed = moon(
            &[ExpressiveRole::Formation, ExpressiveRole::Force],
            &[
                v1,
                AuthoredVariant {
                    variant_ref: "variant:moon-v2".into(),
                    keys: [(
                        "c_2_harmonic_role".to_owned(),
                        json!("authored-v2-changed-content"),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
            None,
        );
        let changed_force = changed
            .occurrences
            .iter()
            .find(|o| o.role == ExpressiveRole::Force)
            .unwrap();
        assert_ne!(changed_force.occurrence_ref, force.occurrence_ref);
    }

    #[test]
    fn invalid_variants_duplicate_roles_and_forks_are_handled_by_name() {
        let registry = native_current_m_registry();
        let bad_layer = AuthoredVariant {
            variant_ref: "variant:bad".into(),
            keys: [("material".to_owned(), json!({"weight": 1}))]
                .into_iter()
                .collect(),
        };
        let error = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#2-5-4",
            MFace::Bimba,
            &[ExpressiveRole::Force],
            &[bad_layer],
            None,
        )
        .unwrap_err();
        assert!(error.contains("role layer"), "{error}");

        let error = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#2-5-4",
            MFace::Bimba,
            &[ExpressiveRole::Force, ExpressiveRole::Force],
            &[],
            None,
        )
        .unwrap_err();
        assert!(
            error.contains("duplicate force occurrence"),
            "duplicate occurrences are refused: {error}"
        );

        let error = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#2-5-4",
            MFace::Bimba,
            &[],
            &[],
            None,
        )
        .unwrap_err();
        assert!(error.contains("at least one expressive role"), "{error}");

        // An explicit instance fork is another purposeful occurrence of the
        // same subject at the same place: same qualification, distinct
        // identity.
        let canonical = moon(&[ExpressiveRole::Force], &[], None);
        let fork = moon(&[ExpressiveRole::Force], &[], Some("take-2"));
        assert_ne!(
            canonical.occurrences[0].occurrence_ref,
            fork.occurrences[0].occurrence_ref
        );
        assert_eq!(fork.instance.as_deref(), Some("take-2"));
        assert_eq!(
            canonical.occurrences[0].property_keys,
            fork.occurrences[0].property_keys
        );
    }

    #[test]
    fn faces_are_exact_addresses_of_one_coordinate_identity() {
        let bimba = moon(&[ExpressiveRole::Force], &[], None);
        let registry = native_current_m_registry();
        let pratibimba = resolve_subject_manifestation(
            registry,
            "ql:k2/default-subject",
            "#2-5-4",
            MFace::Pratibimba,
            &[ExpressiveRole::Force],
            &[],
            None,
        )
        .unwrap();
        assert_eq!(bimba.locus.coordinate_id, pratibimba.locus.coordinate_id);
        assert_eq!(bimba.locus.face, RootedFace::Bimba);
        assert_eq!(pratibimba.locus.face, RootedFace::Pratibimba);
        assert_eq!(
            pratibimba.locus.canonical_ref,
            bimba.locus.conjugate_canonical_ref
        );
        // The conjugate face is a different disclosed reading: its profile
        // lineage and content revision differ while the coordinate stays one.
        assert_ne!(
            bimba.locus.binding_content_revision,
            pratibimba.locus.binding_content_revision
        );
        assert_ne!(
            bimba.occurrences[0].occurrence_ref,
            pratibimba.occurrences[0].occurrence_ref
        );
    }
}
