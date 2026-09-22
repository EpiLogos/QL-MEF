//! Source-qualified Epi-Logos Prime-QL faculty surface.
//!
//! This module does not create another coordinate registry. It composes the
//! current MRegistry, source-locked Anuttara language table, Nara contracts and
//! bounded formal operations into one callable domain surface for Actuation.
//! #0/1 belongs to the constituted acting body; this module exposes #0..#5.

use crate::{
    m_tree::native_current_m_registry,
    nara::{activity::NaraActivityLog, BioQuaternion, ConsentState, SourceRevision},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const EPI_AGENT_CONSTITUTION_VERSION: &str = "ql.epi-logos-agent-constitution/v1";
pub const EPI_AGENT_INVOCATION_VERSION: &str = "ql.epi-logos-agent-invocation/v1";

const ANUTTARA_LANGUAGE: &str =
    include_str!("../../../data/epi-bimba-map/anuttara-language-map.md");
const BIMBA_SOURCE_LOCK: &str = include_str!("../../../data/epi-bimba-map/source-lock.json");
const CAPABILITY_FIELD: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field.json");
const CAPABILITY_M0: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m0.json");
const CAPABILITY_M1: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m1.json");
const CAPABILITY_M2: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m2.json");
const CAPABILITY_M3: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m3.json");
const CAPABILITY_M4: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m4.json");
const CAPABILITY_M5: &str =
    include_str!("../../../docs/integrations/epi-logos/epi-m-capability-field-m5.json");

fn parsed_json(source: &str, label: &str) -> Value {
    serde_json::from_str(source).unwrap_or_else(|error| panic!("{label} is invalid JSON: {error}"))
}

fn syntax_row_count() -> usize {
    ANUTTARA_LANGUAGE
        .lines()
        .filter(|line| line.starts_with("| `M0"))
        .count()
}

fn source_lock() -> Value {
    parsed_json(BIMBA_SOURCE_LOCK, "Bimba source lock")
}

fn capability_document(position: u8) -> Value {
    let source = match position {
        0 => CAPABILITY_M0,
        1 => CAPABILITY_M1,
        2 => CAPABILITY_M2,
        3 => CAPABILITY_M3,
        4 => CAPABILITY_M4,
        5 => CAPABILITY_M5,
        _ => unreachable!("validated position"),
    };
    parsed_json(source, "Epi capability domain")
}

pub fn constitution() -> Value {
    let registry = native_current_m_registry();
    let manifest = registry.manifest();
    json!({
        "schema": EPI_AGENT_CONSTITUTION_VERSION,
        "whole": {
            "coordinate": "#0/1",
            "name": "Prime-QL constituted agent",
            "standing": "acting-body-owned-by-Actuation",
            "distinct_from": "#0",
            "components": [
                "Prime recursion",
                "QL recurrence",
                "Relational Logos orientation",
                "QL-formed operative/reflective tooling",
                "situated native World"
            ]
        },
        "faculties": [
            {"position":"#0","name":"Anuttara","operations":["anuttara.read"],"source_owner":"QL-MEF","optional_instruments":["jev","ananda-m1-2","ebm"]},
            {"position":"#1","name":"Paramaśiva","operations":["tda.vietoris-rips"],"source_owner":"QL-MEF","optional_instruments":["external-tda-provider"]},
            {"position":"#2","name":"Paraśakti","operations":["bimba.neighborhood"],"source_owner":"QL-MEF","optional_instruments":["neo4j-cypher-apoc","neo4j-gds","learned-graph-representations"]},
            {"position":"#3","name":"Mahāmāyā","operations":["representation.bind","ql-techne-reading"],"source_owner":"QL-MEF/O:I","optional_instruments":["cross-modal-retrieval","learned-process-pathways"]},
            {"position":"#4","name":"Nara","operations":["nara.activity.validate","nara.elemental-map"],"source_owner":"QL-MEF","identity":"M4/M4′","s_prime":"S4′ Anima"},
            {"position":"#5","name":"Epii","operations":["logos.return"],"source_owner":"QL-MEF","identity":"M5/M5′","s_prime":"S5′ Aletheia"}
        ],
        "source": {
            "repository": manifest.source_repository,
            "revision": manifest.source_revision,
            "dataset_tree": manifest.source_dataset_tree,
            "registry_revision": manifest.registry_revision,
            "syntax_rows": syntax_row_count(),
            "m0_source_coordinates": registry.manifest().nodes.iter().filter(|node| node.root_position == Some(0)).count(),
            "source_relations": manifest.relations.len(),
            "lock": source_lock()
        },
        "capability_field": parsed_json(CAPABILITY_FIELD, "Epi capability field"),
        "standing": {
            "native_operations": "callable-current-code",
            "optional_instruments": "availability-must-be-resolved-by-their-native-owner",
            "training": "not-authorised-by-mode-selection",
            "canonical_mutation": false
        }
    })
}

pub fn faculty(position: u8) -> Result<Value, String> {
    if position > 5 {
        return Err("Epi faculty position must be 0..5".into());
    }
    let constitution = constitution();
    Ok(json!({
        "schema": "ql.epi-logos-agent-faculty/v1",
        "position": format!("#{position}"),
        "constitution": constitution["faculties"][usize::from(position)].clone(),
        "capability_field": capability_document(position),
        "source_revision": constitution["source"]["revision"].clone(),
        "canonical_mutation": false
    }))
}

fn markdown_cell(value: &str) -> String {
    let trimmed = value.trim();
    let unquoted = trimmed
        .strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .unwrap_or(trimmed);
    unquoted.replace("\\|", "|")
}

fn anuttara_language_row(reference: &str) -> Option<Value> {
    let wanted = reference
        .strip_prefix('#')
        .map(|tail| format!("M{tail}"))
        .unwrap_or_else(|| reference.to_owned());
    for line in ANUTTARA_LANGUAGE.lines() {
        if !line.starts_with("| `M0") {
            continue;
        }
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(markdown_cell)
            .collect::<Vec<_>>();
        if cells.len() < 8 || cells[0] != wanted {
            continue;
        }
        return Some(json!({
            "coordinate": cells[0],
            "name": cells[1],
            "symbol": cells[2],
            "primary_designation": cells[3],
            "complete_formulation": cells[4],
            "formulation_breakdown": cells[5],
            "metaphysical_names": cells[6],
            "description": cells[7]
        }));
    }
    None
}

pub fn anuttara_read(reference: &str, max_relations: usize) -> Result<Value, String> {
    if reference.trim().is_empty() {
        return Err("Anuttara reference must be non-empty".into());
    }
    if !(1..=512).contains(&max_relations) {
        return Err("max_relations must be 1..512".into());
    }
    let row = anuttara_language_row(reference)
        .ok_or_else(|| format!("unknown Anuttara language reference {reference}"))?;
    let registry = native_current_m_registry();
    let node = registry.resolve(reference);
    let relations = node
        .map(|node| {
            registry
                .relations_for(node.id)
                .take(max_relations)
                .map(|relation| serde_json::to_value(relation).expect("M relation serialises"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(json!({
        "schema": "ql.anuttara-source-reading/v1",
        "language": row,
        "bimba_node": node.map(|node| serde_json::to_value(node).expect("M node serialises")),
        "relations": relations,
        "relation_count_returned": relations.len(),
        "relation_limit": max_relations,
        "source": {
            "repository": registry.manifest().source_repository,
            "revision": registry.manifest().source_revision,
            "registry_revision": registry.manifest().registry_revision,
            "language_map": "data/epi-bimba-map/anuttara-language-map.md"
        },
        "standing": if node.is_some() {"source-qualified-language-plus-bimba-relations"} else {"formal-layer-language-row-no-bimba-node"},
        "canonical_mutation": false
    }))
}

pub fn bimba_neighborhood(reference: &str, max_relations: usize) -> Result<Value, String> {
    if !(1..=2048).contains(&max_relations) {
        return Err("max_relations must be 1..2048".into());
    }
    let registry = native_current_m_registry();
    let node = registry
        .resolve(reference)
        .ok_or_else(|| format!("unknown Bimba coordinate {reference}"))?;
    let mut rows = registry
        .relations_for(node.id)
        .map(|relation| serde_json::to_value(relation).expect("M relation serialises"))
        .collect::<Vec<_>>();
    let total = rows.len();
    rows.truncate(max_relations);
    Ok(json!({
        "schema":"ql.bimba-neighborhood/v1",
        "node": serde_json::to_value(node).expect("M node serialises"),
        "relations": rows,
        "total_relations": total,
        "truncated": total > max_relations,
        "algorithm":"exact-source-adjacency",
        "reasoning_support":"none-inferred",
        "source_revision":registry.manifest().source_revision,
        "canonical_mutation":false
    }))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TdaRequest {
    pub metric: String,
    pub complex: String,
    pub coefficients: u8,
    pub max_homology_dimension: u8,
    pub max_scale: f64,
    pub distances: Vec<Vec<f64>>,
    pub source_basis: Value,
}

#[derive(Debug, Clone)]
struct Simplex {
    vertices: Vec<usize>,
    dimension: u8,
    filtration: f64,
}

fn xor_sets(left: &BTreeSet<usize>, right: &BTreeSet<usize>) -> BTreeSet<usize> {
    left.symmetric_difference(right).copied().collect()
}

fn validate_tda(request: &TdaRequest) -> Result<(), String> {
    if request.metric != "precomputed" {
        return Err("TDA metric must be precomputed".into());
    }
    if request.complex != "vietoris-rips" {
        return Err("TDA complex must be vietoris-rips".into());
    }
    if request.coefficients != 2 {
        return Err("TDA currently supports only coefficient field F2".into());
    }
    if request.max_homology_dimension > 1 {
        return Err("TDA currently supports persistent H0 and H1".into());
    }
    if !request.max_scale.is_finite() || request.max_scale < 0.0 {
        return Err("TDA max_scale must be finite and non-negative".into());
    }
    let n = request.distances.len();
    if !(1..=48).contains(&n) {
        return Err("TDA distance matrix must contain 1..48 points".into());
    }
    if request.source_basis.is_null() {
        return Err("TDA source_basis must be explicit".into());
    }
    for (row_index, row) in request.distances.iter().enumerate() {
        if row.len() != n {
            return Err("TDA distance matrix must be square".into());
        }
        for (column_index, value) in row.iter().copied().enumerate() {
            if !value.is_finite() || value < 0.0 {
                return Err("TDA distances must be finite and non-negative".into());
            }
            if row_index == column_index && value.abs() > 1e-12 {
                return Err("TDA distance matrix diagonal must be zero".into());
            }
            if (value - request.distances[column_index][row_index]).abs() > 1e-9 {
                return Err("TDA distance matrix must be symmetric".into());
            }
        }
    }
    Ok(())
}

pub fn persistent_homology(request: TdaRequest) -> Result<Value, String> {
    validate_tda(&request)?;
    let n = request.distances.len();
    let mut simplices = Vec::new();
    for vertex in 0..n {
        simplices.push(Simplex {
            vertices: vec![vertex],
            dimension: 0,
            filtration: 0.0,
        });
    }
    for left in 0..n {
        for right in (left + 1)..n {
            let filtration = request.distances[left][right];
            if filtration <= request.max_scale {
                simplices.push(Simplex {
                    vertices: vec![left, right],
                    dimension: 1,
                    filtration,
                });
            }
        }
    }
    if request.max_homology_dimension >= 1 {
        for a in 0..n {
            for b in (a + 1)..n {
                for c in (b + 1)..n {
                    let filtration = request.distances[a][b]
                        .max(request.distances[a][c])
                        .max(request.distances[b][c]);
                    if filtration <= request.max_scale {
                        simplices.push(Simplex {
                            vertices: vec![a, b, c],
                            dimension: 2,
                            filtration,
                        });
                    }
                }
            }
        }
    }
    simplices.sort_by(|left, right| {
        left.filtration
            .total_cmp(&right.filtration)
            .then(left.dimension.cmp(&right.dimension))
            .then(left.vertices.cmp(&right.vertices))
    });
    let by_vertices = simplices
        .iter()
        .enumerate()
        .map(|(index, simplex)| (simplex.vertices.clone(), index))
        .collect::<BTreeMap<_, _>>();

    let mut reduced = Vec::<BTreeSet<usize>>::with_capacity(simplices.len());
    let mut pivot_to_column = BTreeMap::<usize, usize>::new();
    let mut births = vec![false; simplices.len()];
    let mut death_for_birth = BTreeMap::<usize, usize>::new();

    for (index, simplex) in simplices.iter().enumerate() {
        let mut column = BTreeSet::new();
        if simplex.dimension > 0 {
            for omit in 0..simplex.vertices.len() {
                let mut face = simplex.vertices.clone();
                face.remove(omit);
                let face_index = by_vertices
                    .get(&face)
                    .copied()
                    .ok_or_else(|| "TDA complex lost a simplex face".to_string())?;
                if face_index >= index {
                    return Err("TDA filtration ordering placed a face after its coface".into());
                }
                column.insert(face_index);
            }
        }
        while let Some(low) = column.iter().next_back().copied() {
            let Some(previous) = pivot_to_column.get(&low).copied() else {
                break;
            };
            column = xor_sets(&column, &reduced[previous]);
        }
        if column.is_empty() {
            births[index] = true;
        } else {
            let low = *column.iter().next_back().expect("non-empty reduced column");
            pivot_to_column.insert(low, index);
            death_for_birth.insert(low, index);
        }
        reduced.push(column);
    }

    let intervals = births
        .iter()
        .enumerate()
        .filter(|(index, born)| **born && simplices[*index].dimension <= request.max_homology_dimension)
        .map(|(birth_index, _)| {
            let birth = &simplices[birth_index];
            let death_index = death_for_birth.get(&birth_index).copied();
            let death = death_index.map(|index| simplices[index].filtration);
            json!({
                "dimension": birth.dimension,
                "birth": birth.filtration,
                "death": death,
                "persistence": death.map(|value| value - birth.filtration),
                "birth_simplex": birth.vertices,
                "death_simplex": death_index.map(|index| simplices[index].vertices.clone())
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "schema":"ql.tda-persistence/v1",
        "metric":request.metric,
        "complex":request.complex,
        "coefficients":request.coefficients,
        "max_scale":request.max_scale,
        "max_homology_dimension":request.max_homology_dimension,
        "source_basis":request.source_basis,
        "point_count":n,
        "simplex_count":simplices.len(),
        "intervals":intervals,
        "algorithm":"vietoris-rips-boundary-matrix-reduction-f2",
        "standing":"deterministic-native-result",
        "canonical_topology_claim":false
    }))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementReading {
    pub contribution_strength: f64,
    pub confidence: Option<f64>,
}

impl ElementReading {
    fn validate(&self) -> Result<(), String> {
        if !self.contribution_strength.is_finite() {
            return Err("element contribution strength must be finite".into());
        }
        if self
            .confidence
            .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
        {
            return Err("element confidence must be finite and within 0..=1".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaraElementalRequest {
    pub fire: Option<ElementReading>,
    pub water: Option<ElementReading>,
    pub earth: Option<ElementReading>,
    pub air: Option<ElementReading>,
    pub source: SourceRevision,
    pub consent: ConsentState,
}

pub fn nara_elemental_map(request: NaraElementalRequest) -> Result<Value, String> {
    let mut unknown = Vec::new();
    for (label, reading) in [
        ("fire", request.fire.as_ref()),
        ("water", request.water.as_ref()),
        ("earth", request.earth.as_ref()),
        ("air", request.air.as_ref()),
    ] {
        match reading {
            Some(reading) => reading.validate()?,
            None => unknown.push(label),
        }
    }
    if request.source.source_ref.trim().is_empty()
        || request.source.revision.trim().is_empty()
        || request.source.standing_ref.trim().is_empty()
    {
        return Err("Nara elemental source revision must be explicit".into());
    }
    let raw = match (&request.earth, &request.fire, &request.water, &request.air) {
        (Some(earth), Some(fire), Some(water), Some(air)) => Some(BioQuaternion {
            w: earth.contribution_strength,
            x: fire.contribution_strength,
            y: water.contribution_strength,
            z: air.contribution_strength,
        }),
        _ => None,
    };
    let normalized = raw.map(BioQuaternion::normalized).transpose()?;
    Ok(json!({
        "schema":"ql.nara-elemental-reading/v1",
        "source":request.source,
        "consent":request.consent,
        "contributions":{
            "fire":request.fire,
            "water":request.water,
            "earth":request.earth,
            "air":request.air
        },
        "unknown_elements":unknown,
        "mapping":{
            "source_order":["Fire","Water","Earth","Air"],
            "quaternion":{"w":"Earth","x":"Fire","y":"Water","z":"Air"},
            "jungian_throughline":{
                "Feeling":"Water/A",
                "Intuition":"Fire/T(U)",
                "Sensation":"Earth/C",
                "Thinking":"Air/G"
            }
        },
        "raw_quaternion":raw,
        "normalized_quaternion":normalized,
        "effect_authority_granted":false,
        "standing":"typed-observation-mapping; contribution-strength-distinct-from-confidence"
    }))
}

pub fn validate_nara_activity(value: Value) -> Result<Value, String> {
    let log: NaraActivityLog =
        serde_json::from_value(value).map_err(|error| format!("invalid Nara activity log: {error}"))?;
    log.validate()?;
    Ok(json!({
        "schema":"ql.nara-activity-validation/v1",
        "subject_id":log.subject_id,
        "occurrence_count":log.occurrences.len(),
        "thought_consumption_count":log.thought_consumptions.len(),
        "source_refs":log.occurrences.iter().map(|row|row.source.source_ref.clone()).collect::<Vec<_>>(),
        "standing":"validated-source-spans-and-protection-contract",
        "identity_mutation":false
    }))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationBindingRequest {
    pub subject_ref: String,
    pub formal_state_ref: String,
    pub source_refs: Vec<String>,
    pub representation_ref: String,
    pub modality: String,
    pub asset_refs: Vec<String>,
    pub rendering_revision: String,
    pub operation_revision: String,
    pub temporal_basis_ref: String,
    pub exact_transformation: bool,
}

pub fn bind_representation(request: RepresentationBindingRequest) -> Result<Value, String> {
    for (label, value) in [
        ("subject_ref", request.subject_ref.as_str()),
        ("formal_state_ref", request.formal_state_ref.as_str()),
        ("representation_ref", request.representation_ref.as_str()),
        ("modality", request.modality.as_str()),
        ("rendering_revision", request.rendering_revision.as_str()),
        ("operation_revision", request.operation_revision.as_str()),
        ("temporal_basis_ref", request.temporal_basis_ref.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(format!("representation {label} must be non-empty"));
        }
    }
    if request.source_refs.is_empty() {
        return Err("representation binding requires source_refs".into());
    }
    Ok(json!({
        "schema":"ql.mahamaya-representation-binding/v1",
        "subject_ref":request.subject_ref,
        "formal_state_ref":request.formal_state_ref,
        "source_refs":request.source_refs,
        "representation_ref":request.representation_ref,
        "modality":request.modality,
        "asset_refs":request.asset_refs,
        "rendering_revision":request.rendering_revision,
        "operation_revision":request.operation_revision,
        "temporal_basis_ref":request.temporal_basis_ref,
        "exact_transformation":request.exact_transformation,
        "standing":"source/form/representation/temporal-binding",
        "semantic_equivalence_inferred":false,
        "canonical_mutation":false
    }))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogosReturnRequest {
    pub inquiry_ref: String,
    #[serde(rename = "T")]
    pub t: String,
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "T_prime")]
    pub t_prime: String,
    #[serde(rename = "C_prime")]
    pub c_prime: String,
    pub source_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub practice_ref: Option<String>,
    pub fresh_participant_ref: Option<String>,
}

pub fn logos_return(request: LogosReturnRequest) -> Result<Value, String> {
    for (label, value) in [
        ("inquiry_ref", request.inquiry_ref.as_str()),
        ("T", request.t.as_str()),
        ("C", request.c.as_str()),
        ("T_prime", request.t_prime.as_str()),
        ("C_prime", request.c_prime.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(format!("Logos cycle {label} must be non-empty"));
        }
    }
    if request.source_refs.is_empty() || request.evidence_refs.is_empty() {
        return Err("Logos Return requires source_refs and evidence_refs".into());
    }
    let uptake_claimed = request.practice_ref.is_some() && request.fresh_participant_ref.is_some();
    Ok(json!({
        "schema":"ql.epii-logos-return/v1",
        "inquiry_ref":request.inquiry_ref,
        "cycle":{
            "T":request.t,
            "C":request.c,
            "T_prime":request.t_prime,
            "C_prime":request.c_prime
        },
        "source_refs":request.source_refs,
        "evidence_refs":request.evidence_refs,
        "practice_ref":request.practice_ref,
        "fresh_participant_ref":request.fresh_participant_ref,
        "fresh_uptake_reference_present":uptake_claimed,
        "learning_demonstrated":false,
        "standing":"performed-return-envelope; later-held-out-uptake-must-be-observed-separately",
        "promotion_authority_granted":false
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constitution_keeps_whole_and_zero_distinct() {
        let value = constitution();
        assert_eq!(value["whole"]["coordinate"], "#0/1");
        assert_eq!(value["whole"]["distinct_from"], "#0");
        assert_eq!(value["faculties"].as_array().unwrap().len(), 6);
        assert_eq!(value["faculties"][4]["identity"], "M4/M4′");
        assert_eq!(value["faculties"][5]["identity"], "M5/M5′");
        assert_eq!(value["faculties"][4]["s_prime"], "S4′ Anima");
        assert_eq!(value["faculties"][5]["s_prime"], "S5′ Aletheia");
        assert_eq!(value["source"]["syntax_rows"], 109);
    }

    #[test]
    fn anuttara_reading_joins_formulation_and_declared_relations() {
        let reading = anuttara_read("M0-2-9", 128).unwrap();
        assert_eq!(reading["language"]["coordinate"], "M0-2-9");
        assert!(reading["language"]["complete_formulation"].as_str().unwrap().contains("Paramesvara"));
        assert!(reading["bimba_node"].is_object());
        assert!(reading["relations"].is_array());
        assert_eq!(reading["canonical_mutation"], false);
    }

    #[test]
    fn persistence_distinguishes_one_cycle_until_a_triangle_fills_it() {
        let result = persistent_homology(TdaRequest {
            metric: "precomputed".into(),
            complex: "vietoris-rips".into(),
            coefficients: 2,
            max_homology_dimension: 1,
            max_scale: 2.0,
            distances: vec![
                vec![0.0, 1.0, 1.0],
                vec![1.0, 0.0, 1.0],
                vec![1.0, 1.0, 0.0],
            ],
            source_basis: json!({"ref":"fixture:triangle","revision":"r1"}),
        }).unwrap();
        let intervals = result["intervals"].as_array().unwrap();
        assert!(intervals.iter().any(|row| row["dimension"] == 1 && row["birth"] == 1.0 && row["death"] == 1.0));
    }

    #[test]
    fn missing_element_is_unknown_not_zero() {
        let reading = nara_elemental_map(NaraElementalRequest {
            fire: Some(ElementReading { contribution_strength: 1.0, confidence: Some(0.8) }),
            water: None,
            earth: Some(ElementReading { contribution_strength: 2.0, confidence: Some(0.6) }),
            air: Some(ElementReading { contribution_strength: 3.0, confidence: None }),
            source: SourceRevision { source_ref:"central:activity/1".into(), revision:"r1".into(), standing_ref:"observed".into() },
            consent: ConsentState::Granted,
        }).unwrap();
        assert_eq!(reading["unknown_elements"], json!(["water"]));
        assert!(reading["normalized_quaternion"].is_null());
        assert_eq!(reading["effect_authority_granted"], false);
    }
}
