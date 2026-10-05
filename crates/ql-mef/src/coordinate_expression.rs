//! TA0/TA1 coordinate profiles derived from the existing current M registry.
//!
//! This is source/profile resolution. Host adoption, encounter state, provider
//! availability and protected Anima readings remain with their existing owners.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;
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

struct CoordinateCopyCounter {
    remaining: usize,
}
impl std::io::Write for CoordinateCopyCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.remaining = self.remaining.checked_sub(bytes.len()).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "native Source coordinate construction",
            )
        })?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl CoordinateCopyCounter {
    fn value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), String> {
        serde_json::to_writer(&mut *self, value)
            .map_err(|_| "Native Source coordinate construction exceeds its reserved cap".into())
    }
}

/// Resource-bounded sibling of the actual pure native resolver. The registry
/// and embedded grammars remain the sole source. A byte ceiling issues no
/// binding/Scene/clock/Source capability and never changes an ordinary call.
pub fn resolve_coordinate_expression_bounded(
    registry: &MRegistry,
    reference: &str,
    face: MFace,
    cap: usize,
) -> Result<CoordinateExpressionBinding, String> {
    if cap == 0 || cap > 16 * 1024 * 1024 {
        return Err("Private Source coordinate cap is absent or exceeds ordinary Nara".into());
    }
    let canonical = if let Some(canonical) = reference.strip_prefix("ql:m-coordinate:") {
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
    let selected = registry
        .resolve(canonical)
        .ok_or("unknown rooted M coordinate")?;
    let root = selected
        .root_position
        .ok_or("source M coordinate has no root position")?;
    let capability = CAPABILITIES
        .get(usize::from(root))
        .ok_or("native coordinate family is absent")?;
    let mut copies = CoordinateCopyCounter {
        remaining: cap
            .checked_mul(32)
            .ok_or("Native coordinate copy cap overflow")?,
    };
    // Borrowed embedded grammars cover parsing, capability/faculty copies and
    // hashing before their first Value/Vec allocation. Source bodies stay in
    // their existing native registry; no file or caller payload replaces them.
    copies.value(&(
        WAYFINDER, ORGANS, ORGANS, ORGANS, capability, capability, capability, capability,
    ))?;
    copies.value(&(
        reference,
        reference,
        reference,
        reference,
        &registry.manifest().registry_revision,
        &registry.manifest().source_repository,
        &registry.manifest().source_revision,
    ))?;
    let mut cursor = Some(selected.id);
    let mut visited = 0usize;
    while let Some(id) = cursor {
        visited = visited
            .checked_add(1)
            .ok_or("native coordinate ancestry overflow")?;
        if visited > registry.manifest().nodes.len() {
            return Err("broken M ancestry".into());
        }
        let node = registry.node(id).ok_or("broken M ancestry")?;
        // Full node strings/records and repeated rooted/profile projections
        // precede all native owned ancestry/binding/property-source copies.
        copies.value(&(node, node, node, node, node, node, node, node))?;
        for index in &node.records {
            let record = registry
                .manifest()
                .records
                .get(*index)
                .ok_or("missing source record")?;
            let file = registry
                .manifest()
                .files
                .get(record.file)
                .ok_or("missing source file")?;
            copies.value(&(record, record, file, file))?;
        }
        cursor = node.parent_id;
    }
    for relation in registry.relations_for(selected.id) {
        let record = registry
            .manifest()
            .records
            .get(relation.record)
            .ok_or("missing source record")?;
        let file = registry
            .manifest()
            .files
            .get(record.file)
            .ok_or("missing source file")?;
        copies.value(&(relation, relation, record, record, file, file))?;
    }
    for binding in registry
        .manifest()
        .bindings
        .iter()
        .filter(|binding| binding.coordinate_id == Some(selected.id))
    {
        copies.value(&(binding, binding))?;
    }
    copies.value(&[0_u8; 4096])?;
    resolve_coordinate_expression(registry, reference, face)
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
}

#[cfg(test)]
mod source_coordinate_copy_budget_tests {
    use super::*;
    #[test]
    fn bounded_resolution_uses_actual_registry_and_retains_each_face_and_source() {
        let registry = crate::m_tree::native_current_m_registry();
        for face in [MFace::Bimba, MFace::Pratibimba] {
            let actual = resolve_coordinate_expression(registry, "#3", face).unwrap();
            let bounded =
                resolve_coordinate_expression_bounded(registry, "#3", face, 16 * 1024 * 1024)
                    .unwrap();
            assert_eq!(
                serde_json::to_value(&actual).unwrap(),
                serde_json::to_value(&bounded).unwrap()
            );
            assert!(resolve_coordinate_expression_bounded(registry, "#3", face, 1).is_err());
            assert!(resolve_coordinate_expression_bounded(registry, "#3", face, 0).is_err());
        }
        assert!(
            resolve_coordinate_expression_bounded(
                registry,
                "ql:m-coordinate:pratibimba:#3",
                MFace::Bimba,
                16 * 1024 * 1024
            )
            .is_err()
        );
    }
}
