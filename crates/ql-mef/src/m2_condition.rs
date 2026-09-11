//! Joint M2 music/colour/material producer over exact Bimba relation paths.
//!
//! Source statements, retained-C playback, explicit note-spelling tuning,
//! presentation palette and supplied physical observations stay distinguishable.
//! All output belongs to the engine's event/generation; no shader owns lookup.
use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::m_tree::{MTreeId, native_m_registry};
use crate::m2::{self, Reading72, Register72, SourceLock};
use crate::m2_engine::{
    EventIdentity, InputStamp, M2Frame, M2Request, MaterialFibre, ResonatorState, WorldReading,
};
use crate::m2_vimarsha::VimarshaReading;

pub const CONDITION_CONTRACT: &str = "ql.m2-condition/v1";
pub const CORRESPONDENCE_CONTRACT: &str = "ql.m2-correspondences/v1";
pub const SOURCE_FIELD: &str = include_str!("../../../fixtures/kernel/m2-correspondences-v1.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrespondenceRole {
    Tonic,
    Dominant,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TuningPolicy {
    Retained24Tet,
    BimbaSpelled24Tet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceClaim {
    pub coordinate: String,
    pub node_id: MTreeId,
    pub property: String,
    pub literal: String,
    pub path: String,
    pub pointer: String,
    pub sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRelation {
    pub id: MTreeId,
    pub relation_ref: String,
    pub kind: String,
    pub from_coordinate: String,
    pub to_coordinate: String,
    pub path: String,
    pub sha256: String,
    pub record_index: usize,
    pub payload_sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrespondenceRule {
    pub maqam_index: u8,
    pub role: CorrespondenceRole,
    pub maqam_coordinate: String,
    pub maqam_node_id: MTreeId,
    pub maqam_name: String,
    pub planet_index: u8,
    pub planet_coordinate: String,
    pub planet_node_id: MTreeId,
    pub chakra_index: u8,
    pub chakra_coordinate: String,
    pub chakra_node_id: MTreeId,
    pub tattva_coordinate: Option<String>,
    pub tattva_node_id: Option<MTreeId>,
    pub element_literal: String,
    pub material_fibre: Option<MaterialFibre>,
    pub yantra_literal: String,
    pub colour_name: Option<String>,
    pub planetary_mode_literal: String,
    pub interval_literal: String,
    pub spelled_steps24: Option<Vec<u8>>,
    pub musical_relations: Vec<SourceRelation>,
    pub planetary_relations: Vec<SourceRelation>,
    pub claims: Vec<SourceClaim>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrespondenceField {
    pub schema: String,
    pub registry_revision: String,
    pub source_revision: String,
    pub source_repository: String,
    pub standing: String,
    pub source_locks: Vec<SourceLock>,
    pub rules: Vec<CorrespondenceRule>,
    pub gaps: Vec<String>,
}
impl CorrespondenceField {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let field: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let registry = native_m_registry();
        let manifest = registry.manifest();
        if field.schema != CORRESPONDENCE_CONTRACT
            || field.registry_revision != manifest.registry_revision
            || field.source_revision != manifest.source_revision
            || field.source_repository != manifest.source_repository
        {
            return Err("stale or unsupported correspondence source".into());
        }
        for lock in &field.source_locks {
            if !manifest
                .files
                .iter()
                .any(|f| f.path == lock.path && f.sha256 == lock.sha256)
            {
                return Err("unlocked correspondence source".into());
            }
        }
        let mut seen = BTreeSet::new();
        for rule in &field.rules {
            if rule.maqam_index >= 72
                || rule.planet_index >= 10
                || rule.chakra_index >= 8
                || !seen.insert((rule.maqam_index, rule.role as u8))
            {
                return Err("duplicate or invalid correspondence rule".into());
            }
            for (reference, id) in [
                (&rule.maqam_coordinate, rule.maqam_node_id),
                (&rule.planet_coordinate, rule.planet_node_id),
                (&rule.chakra_coordinate, rule.chakra_node_id),
            ] {
                if registry.resolve(reference).map(|n| n.id) != Some(id) {
                    return Err("correspondence node identity mismatch".into());
                }
            }
            if rule
                .tattva_coordinate
                .as_ref()
                .and_then(|r| registry.resolve(r))
                .map(|n| n.id)
                != rule.tattva_node_id
            {
                return Err("correspondence tattva identity mismatch".into());
            }
            for (table, index, reference) in [
                ("maqam", rule.maqam_index, &rule.maqam_coordinate),
                ("planet", rule.planet_index, &rule.planet_coordinate),
                ("chakra", rule.chakra_index, &rule.chakra_coordinate),
            ] {
                if m2::catalogue().table(table)?.binding(index as usize) != Some(reference.as_str())
                {
                    return Err("correspondence retained binding mismatch".into());
                }
            }
            let expected_kind = match rule.role {
                CorrespondenceRole::Tonic => "TONIC_PLANETARY_RESONANCE",
                CorrespondenceRole::Dominant => "DOMINANT_PLANETARY_RESONANCE",
            };
            for (relations, from, to, kind) in [
                (
                    &rule.musical_relations,
                    &rule.maqam_coordinate,
                    &rule.planet_coordinate,
                    expected_kind,
                ),
                (
                    &rule.planetary_relations,
                    &rule.planet_coordinate,
                    &rule.chakra_coordinate,
                    "PLANETARY_RESONANCE",
                ),
            ] {
                if relations.is_empty() {
                    return Err("missing source relation path".into());
                }
                for link in relations {
                    let r = manifest
                        .relations
                        .iter()
                        .find(|r| r.id == link.id)
                        .ok_or("missing registered relation")?;
                    let record = &manifest.records[r.record];
                    let file = &manifest.files[record.file];
                    if r.from_ref.as_ref() != Some(from)
                        || r.to_ref.as_ref() != Some(to)
                        || r.source_kind != kind
                        || link.kind != kind
                        || &link.from_coordinate != from
                        || &link.to_coordinate != to
                        || link.relation_ref != r.relation_ref
                        || link.path != file.path
                        || link.sha256 != file.sha256
                        || link.record_index != record.record_index
                        || link.payload_sha256 != record.payload_sha256
                    {
                        return Err("correspondence relation/provenance mismatch".into());
                    }
                }
            }
            if let Some(steps) = &rule.spelled_steps24 {
                if steps.len() != 8
                    || steps[0] != 0
                    || steps[7] != 24
                    || !steps.windows(2).all(|w| w[0] < w[1])
                {
                    return Err("invalid explicit spelled tuning".into());
                }
            }
            if rule.claims.len() != 4 {
                return Err("missing correspondence property claims".into());
            }
            for claim in &rule.claims {
                let node = registry
                    .resolve(&claim.coordinate)
                    .ok_or("missing source claim node")?;
                if node.id != claim.node_id
                    || !node.records.iter().any(|index| {
                        let record = &manifest.records[*index];
                        let file = &manifest.files[record.file];
                        file.path == claim.path
                            && file.sha256 == claim.sha256
                            && record
                                .property_keys
                                .contains(&format!("filteredProps.{}", claim.property))
                            && claim.pointer
                                == format!(
                                    "/{}/filteredProps/{}",
                                    record.record_index, claim.property
                                )
                    })
                {
                    return Err("invalid source property claim".into());
                }
            }
        }
        Ok(field)
    }
    pub fn rule(&self, index: u8, role: CorrespondenceRole) -> Option<&CorrespondenceRule> {
        self.rules
            .iter()
            .find(|r| r.maqam_index == index && r.role == role)
    }
}
pub fn correspondence_field() -> &'static CorrespondenceField {
    static FIELD: OnceLock<CorrespondenceField> = OnceLock::new();
    FIELD.get_or_init(|| {
        CorrespondenceField::from_json(SOURCE_FIELD).expect("compiled source paths verified")
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedPaletteEntry {
    pub name: String,
    /// Explicit renderer policy, linear-light RGBA; never inferred from Hz.
    pub linear_rgba: [f64; 4],
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderPalette {
    pub stamp: InputStamp,
    pub policy_ref: String,
    pub entries: Vec<NamedPaletteEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2ConditionInput {
    pub maqam_index: u8,
    pub role: CorrespondenceRole,
    pub active_mef_condition: u8,
    pub tuning: TuningPolicy,
    pub tonic_hz: f64,
    pub palette: Option<RenderPalette>,
}
impl M2ConditionInput {
    pub(crate) fn validate(&self, request: &M2Request) -> Result<(), String> {
        if self.maqam_index >= 72
            || self.active_mef_condition >= 72
            || !self.tonic_hz.is_finite()
            || self.tonic_hz <= 0.0
            || self.tonic_hz > 1_000_000.0
        {
            return Err("invalid joint M2 condition input".into());
        }
        let vim = request
            .vimarsha
            .as_ref()
            .ok_or("joint M2 condition requires active Vimarsha drive")?;
        if vim.lens != self.active_mef_condition / 6
            || request.tick12 % 6 != self.active_mef_condition % 6
            || !request.mef_conditions.contains(&self.active_mef_condition)
        {
            return Err("joint condition and MEF/Vimarsha selection disagree".into());
        }
        if let Some(palette) = &self.palette {
            if palette.stamp.identity != request.stamp.identity
                || palette.policy_ref.trim().is_empty()
                || palette.stamp.source_ref.trim().is_empty()
                || palette.stamp.contract_ref.trim().is_empty()
                || palette.entries.len() > 32
            {
                return Err("stale, unbounded or unattributed palette".into());
            }
            let mut names = BTreeSet::new();
            for entry in &palette.entries {
                if entry.name.trim().is_empty()
                    || !names.insert(&entry.name)
                    || entry
                        .linear_rgba
                        .iter()
                        .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
                {
                    return Err("invalid or duplicate linear RGBA palette entry".into());
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ConditionMusic {
    pub tuning: TuningPolicy,
    pub tonic_hz: f64,
    pub pitches_hz: Vec<f64>,
    pub standing: String,
    pub retained_c_planet_index: u8,
}
#[derive(Debug, Clone, Serialize)]
pub struct ConditionColour {
    pub source_name: Option<String>,
    pub linear_rgba: Option<[f64; 4]>,
    pub palette: Option<RenderPalette>,
    pub standing: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct M2Condition {
    pub schema: String,
    pub identity: EventIdentity,
    pub registry_revision: String,
    pub source_revision: String,
    pub correspondence_ref: String,
    pub active_mef_condition: u8,
    pub sublens_ref: String,
    pub musical: ConditionMusic,
    pub drive: VimarshaReading,
    pub source_path: Option<CorrespondenceRule>,
    pub colour: ConditionColour,
    pub physical_material: Option<ResonatorState>,
    pub world: Option<WorldReading>,
    pub provider_standing: String,
    pub m3_form_potential_ref: String,
    pub m3_scalar_from_mef: u8,
    pub m3_legacy_mask_from_mef_decimal: String,
    pub m3_transform_policies: Vec<String>,
    pub gaps: Vec<String>,
}

pub fn condition_pitches(
    index: u8,
    role: CorrespondenceRole,
    tuning: TuningPolicy,
    tonic: f64,
) -> Result<Option<[f64; 8]>, String> {
    if index >= 72 || !tonic.is_finite() || tonic <= 0.0 {
        return Err("invalid condition pitch input".into());
    }
    match tuning {
        TuningPolicy::Retained24Tet => m2::maqam_pitches(index, tonic).map(Some),
        TuningPolicy::BimbaSpelled24Tet => {
            let steps = correspondence_field()
                .rule(index, role)
                .and_then(|r| r.spelled_steps24.as_ref());
            let Some(steps) = steps else {
                return Ok(None);
            };
            let pitches = std::array::from_fn(|i| tonic * 2f64.powf(f64::from(steps[i]) / 24.0));
            if pitches.iter().any(|p| !p.is_finite()) {
                return Err("condition pitch overflow".into());
            }
            Ok(Some(pitches))
        }
    }
}
pub(crate) fn resolve(input: &M2ConditionInput, frame: &M2Frame) -> Result<M2Condition, String> {
    let field = correspondence_field();
    let rule = field.rule(input.maqam_index, input.role).cloned();
    let mut gaps = Vec::new();
    if rule.is_none() {
        gaps.push("No unique source-held maqam/planetary/chakral path for this role".into());
    }
    let pitches = condition_pitches(input.maqam_index, input.role, input.tuning, input.tonic_hz)?;
    if pitches.is_none() {
        gaps.push("Selected source tuning is unsupported; no retained-tuning fallback".into());
    }
    let source_name = rule.as_ref().and_then(|r| r.colour_name.clone());
    let rgba = source_name
        .as_ref()
        .and_then(|name| {
            input
                .palette
                .as_ref()
                .and_then(|p| p.entries.iter().find(|e| &e.name == name))
        })
        .map(|e| e.linear_rgba);
    let colour_standing = if source_name.is_none() {
        "unavailable-no-explicit-source-colour"
    } else if rgba.is_none() {
        "source-name-present-no-palette-conversion"
    } else {
        "source-name-with-explicit-linear-rgba-policy"
    };
    if source_name.is_none() {
        gaps.push("No explicit admitted colour name in this source path".into());
    }
    let world = rule
        .as_ref()
        .and_then(|r| {
            frame
                .world
                .iter()
                .find(|w| w.observation.planet_id == r.planet_index)
        })
        .cloned();
    let physical = frame.resonator.clone();
    let provider_standing = if world.is_some() {
        "supplied-observation-retained-with-age-and-source"
    } else {
        "unavailable-no-world-provider-for-selected-planet"
    };
    let mef = Reading72::new(Register72::Mef, input.active_mef_condition)?;
    let drive = frame
        .vimarsha
        .as_ref()
        .ok_or("missing joint condition drive")?
        .reading
        .clone();
    Ok(M2Condition {
        schema: CONDITION_CONTRACT.into(),
        identity: frame.identity.clone(),
        registry_revision: field.registry_revision.clone(),
        source_revision: field.source_revision.clone(),
        correspondence_ref: CORRESPONDENCE_CONTRACT.into(),
        active_mef_condition: input.active_mef_condition,
        sublens_ref: mef.mef_sublens()?.to_string(),
        musical: ConditionMusic {
            tuning: input.tuning,
            tonic_hz: input.tonic_hz,
            pitches_hz: pitches.map(Vec::from).unwrap_or_default(),
            standing: match input.tuning {
                TuningPolicy::Retained24Tet => {
                    "retained-C-24tet-reading-not-source-interval-equivalence"
                }
                TuningPolicy::BimbaSpelled24Tet => {
                    "explicit-Bimba-note-spelling-24tet-policy-not-performance-authenticity"
                }
            }
            .into(),
            retained_c_planet_index: m2::catalogue()
                .table("maqam")?
                .row(input.maqam_index as usize)?[9] as u8,
        },
        drive,
        source_path: rule,
        colour: ConditionColour {
            source_name,
            linear_rgba: rgba,
            palette: input.palette.clone(),
            standing: colour_standing.into(),
        },
        physical_material: physical,
        world,
        provider_standing: provider_standing.into(),
        m3_form_potential_ref: "ql.m2-engine/v1#/modal/form_potential".into(),
        m3_scalar_from_mef: m2::scalar_compress(input.active_mef_condition)?,
        m3_legacy_mask_from_mef_decimal: m2::legacy_det(&[input.active_mef_condition])?.to_string(),
        m3_transform_policies: vec![
            "scalar-floor-8i/9-from-explicit-MEF-index".into(),
            "retained-OR-mask-from-explicit-MEF-index".into(),
            "I4-tensor-T18to16-from-EFWA-modal-coefficients;16->0,17->8".into(),
        ],
        gaps,
    })
}
