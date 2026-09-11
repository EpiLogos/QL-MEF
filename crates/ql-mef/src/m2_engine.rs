//! Stable M2 state/refs for continuous modal/material embodiment (K6 -> K8/K9).
//!
//! This is a producer boundary, not a bell solver. Native 72 carriers need not
//! equal the number of physical eigenmodes. A provider maps its modes explicitly
//! to carriers and supplies geometry/material, nodal/antinodal and damping facts.
//! Supplied observations are never relabelled as authenticated/live/verified.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m_tree::native_m_registry;
use crate::m2::{self, DescriptorReading, ModalField, Reading72, Register72};
use crate::{ContextFrameId, PoleIdentity, lens_definition};

pub const MAX_EXACT_JSON_INTEGER: u64 = 9_007_199_254_740_991;
pub const REQUEST_SCHEMA: &str = "ql.m2-engine-request/v1";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventIdentity {
    pub event_ref: String,
    pub profile_generation: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputStamp {
    pub identity: EventIdentity,
    pub source_ref: String,
    pub contract_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub table: String,
    pub index: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M1Excitation {
    pub stamp: InputStamp,
    pub topology_ref: String,
    pub audio_octet: [f64; 8],
    pub nodal_quartet: [f64; 4],
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VimarshaInput {
    pub stamp: InputStamp,
    pub lens: u8,
    pub musical_mode: u8,
    pub harmonic_ratio: [u16; 2],
    pub pose_ordinal: u16,
    pub pose_source_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VimarshaFrame {
    pub input: VimarshaInput,
    pub reading: crate::m2_vimarsha::VimarshaReading,
}
impl VimarshaInput {
    fn read(&self, tick12: u8) -> Result<VimarshaFrame, String> {
        nonempty(&self.pose_source_ref, "Vimarsha pose source")?;
        let pose = ql_core::all_poses()
            .nth(self.pose_ordinal as usize)
            .ok_or("invalid shared M3 pose ordinal")?;
        Ok(VimarshaFrame {
            input: self.clone(),
            reading: crate::m2_vimarsha::read_from_pose(
                tick12,
                self.lens,
                self.musical_mode,
                self.harmonic_ratio,
                pose,
            )?,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldObservation {
    pub planet_id: u8,
    pub longitude_degrees: f64,
    pub provider_ref: String,
    pub source_revision: String,
    pub observed_at_unix_ms: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialFibre {
    Earth,
    Fire,
    Water,
    Air,
}
impl MaterialFibre {
    pub const fn index(self) -> usize {
        match self {
            Self::Earth => 0,
            Self::Fire => 1,
            Self::Water => 2,
            Self::Air => 3,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierWeight {
    pub carrier: u8,
    pub weight: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalParameter {
    pub value: f64,
    pub unit: String,
    pub source_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuousMode {
    pub mode_ref: String,
    pub source_coordinate: String,
    pub material_fibre: MaterialFibre,
    pub carrier_weights: Vec<CarrierWeight>,
    pub frequency_hz: f64,
    pub amplitude: [f64; 2],
    pub excitation: [f64; 2],
    pub damping_per_second: f64,
    pub nodal_state_ref: String,
    pub antinodal_state_ref: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResonatorState {
    pub stamp: InputStamp,
    pub provider_ref: String,
    pub geometry_ref: String,
    pub material_ref: String,
    pub material_model_ref: String,
    pub material_parameters: BTreeMap<String, PhysicalParameter>,
    pub modes: Vec<ContinuousMode>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2Request {
    pub condition: Option<crate::m2_condition::M2ConditionInput>,
    pub schema: String,
    pub registry_revision: String,
    pub stamp: InputStamp,
    pub at_unix_ms: u64,
    pub tick12: u8,
    pub degree720: u16,
    /// EFWA order, 18 coefficients per fibre, exact integer kernel units.
    pub modal_coefficients: Vec<[i64; 2]>,
    pub mef_conditions: Vec<u8>,
    pub context_frames: Vec<String>,
    pub selections: Vec<Selection>,
    pub m1_excitation: Option<M1Excitation>,
    pub vimarsha: Option<VimarshaInput>,
    pub world_observations: Vec<WorldObservation>,
    pub resonator: Option<ResonatorState>,
}
#[derive(Debug, Clone, Serialize)]
pub struct MefReading {
    pub carrier_index: u8,
    pub sublens_ref: String,
    pub lens_name: String,
    pub source_scope: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct ContextReading {
    pub code: String,
    pub expression: String,
    pub condition_indices: Vec<u8>,
}
#[derive(Debug, Clone, Serialize)]
pub struct WorldReading {
    pub observation: WorldObservation,
    pub standing: String,
    pub age_ms: u64,
    pub planetary_power: DescriptorReading,
    pub light_decan: DescriptorReading,
    pub shadow_decan: DescriptorReading,
}
#[derive(Debug, Clone, Serialize)]
pub struct PlanetaryAspect {
    pub from_planet: u8,
    pub to_planet: u8,
    pub value: m2::Aspect,
}
#[derive(Debug, Clone, Serialize)]
pub struct DomainHolding {
    pub name: String,
    pub scope: String,
    pub retained_records: usize,
    pub exact_coordinate_bindings: usize,
    pub selected_records: usize,
    pub standing: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct ModalState {
    pub contract_ref: String,
    pub carrier_order: [MaterialFibre; 4],
    pub coefficient_unit: String,
    pub coefficients: Vec<[i64; 2]>,
    pub quadrature: Vec<[i64; 2]>,
    pub form_potential: Vec<[i64; 2]>,
    pub power_decimal: String,
    pub transformation: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct NumericGround {
    pub phi_polynomial: [i8; 3],
    pub phi: f64,
    pub base36: u32,
    pub field72: u32,
    pub form64: u32,
    pub retained_whole: u32,
    pub third_spanda: u32,
    pub elemental_aperture: [u32; 3],
}
#[derive(Debug, Clone, Serialize)]
pub struct M2Frame {
    pub condition: Option<crate::m2_condition::M2Condition>,
    pub schema: String,
    pub registry_revision: String,
    pub ledger_revision: String,
    pub identity: EventIdentity,
    pub input_stamp: InputStamp,
    pub at_unix_ms: u64,
    pub tick12: u8,
    pub degree720: u16,
    pub source_locks: Vec<m2::SourceLock>,
    pub structural_coordinate_count: usize,
    pub structural_relation_count: usize,
    pub numerical_ground: NumericGround,
    pub domains: Vec<DomainHolding>,
    pub mef: Vec<MefReading>,
    pub context_frames: Vec<ContextReading>,
    pub selected_descriptors: Vec<DescriptorReading>,
    pub linked_descriptors: Vec<DescriptorReading>,
    pub modal: ModalState,
    pub m1_excitation: Option<M1Excitation>,
    pub vimarsha: Option<VimarshaFrame>,
    pub world: Vec<WorldReading>,
    pub aspects: Vec<PlanetaryAspect>,
    pub resonator: Option<ResonatorState>,
    pub continuous_standing: String,
}
fn nonempty(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} must carry a real reference"))
    } else {
        Ok(())
    }
}
fn stamp(value: &InputStamp, identity: &EventIdentity) -> Result<(), String> {
    nonempty(&value.source_ref, "source_ref")?;
    nonempty(&value.contract_ref, "contract_ref")?;
    nonempty(&value.identity.event_ref, "event_ref")?;
    if value.identity.profile_generation > MAX_EXACT_JSON_INTEGER {
        return Err("profile generation exceeds exact JSON integer range".into());
    }
    if &value.identity != identity {
        return Err("cross-event or stale profile generation".into());
    }
    Ok(())
}
fn finite(values: impl IntoIterator<Item = f64>) -> Result<(), String> {
    if values.into_iter().all(f64::is_finite) {
        Ok(())
    } else {
        Err("non-finite provider state".into())
    }
}
fn validate_resonator(state: &ResonatorState, identity: &EventIdentity) -> Result<(), String> {
    stamp(&state.stamp, identity)?;
    for (label, value) in [
        ("provider_ref", &state.provider_ref),
        ("geometry_ref", &state.geometry_ref),
        ("material_ref", &state.material_ref),
        ("material_model_ref", &state.material_model_ref),
    ] {
        nonempty(value, label)?;
    }
    for (name, p) in &state.material_parameters {
        nonempty(name, "material parameter name")?;
        nonempty(&p.unit, "parameter unit")?;
        nonempty(&p.source_ref, "parameter source")?;
        finite([p.value])?;
    }
    if state.modes.is_empty() || state.modes.len() > 4096 {
        return Err("resonator needs 1..4096 supplied eigenmodes".into());
    }
    let registry = native_m_registry();
    let root = registry.root(2).ok_or("missing M2 root")?.id;
    let mut ids = BTreeSet::new();
    for mode in &state.modes {
        nonempty(&mode.mode_ref, "mode_ref")?;
        if !ids.insert(&mode.mode_ref) {
            return Err("duplicate eigenmode identity".into());
        }
        let node = registry
            .resolve(&mode.source_coordinate)
            .ok_or("unresolved eigenmode source coordinate")?;
        if node.root_id != root {
            return Err("eigenmode anchor must be an actual M2 coordinate".into());
        }
        finite([
            mode.frequency_hz,
            mode.damping_per_second,
            mode.amplitude[0],
            mode.amplitude[1],
            mode.excitation[0],
            mode.excitation[1],
        ])?;
        if mode.frequency_hz <= 0.0 || mode.damping_per_second < 0.0 {
            return Err("invalid frequency or damping".into());
        }
        nonempty(&mode.nodal_state_ref, "nodal_state_ref")?;
        nonempty(&mode.antinodal_state_ref, "antinodal_state_ref")?;
        if mode.carrier_weights.is_empty() || mode.carrier_weights.len() > 18 {
            return Err("an elemental mode needs 1..18 explicit carrier weights".into());
        }
        let mut carriers = BTreeSet::new();
        for weight in &mode.carrier_weights {
            finite([weight.weight])?;
            if weight.carrier >= 72
                || usize::from(weight.carrier / 18) != mode.material_fibre.index()
                || !carriers.insert(weight.carrier)
            {
                return Err("duplicate or cross-element eigenmode weight".into());
            }
        }
    }
    Ok(())
}
impl M2Request {
    pub fn from_json(json: &str) -> Result<Self, String> {
        if json.len() > 32 * 1024 * 1024 {
            return Err("M2 request exceeds 32 MiB".into());
        }
        let request: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        request.validate()?;
        Ok(request)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != REQUEST_SCHEMA
            || self.registry_revision != native_m_registry().manifest().registry_revision
        {
            return Err("unsupported M2 request or stale registry revision".into());
        }
        if self.at_unix_ms > MAX_EXACT_JSON_INTEGER {
            return Err("event time exceeds exact JSON integer range".into());
        }
        stamp(&self.stamp, &self.stamp.identity)?;
        PoleIdentity::new(
            &self.stamp.identity.event_ref,
            self.stamp.identity.profile_generation,
            self.tick12,
            self.degree720,
        )
        .map_err(|e| e.to_string())?;
        ModalField::new(&self.modal_coefficients)?;
        if self.selections.len() > 764
            || self.mef_conditions.len() > 72
            || self.context_frames.len() > 7
        {
            return Err("duplicate/unbounded selections".into());
        }
        let mut selections = BTreeSet::new();
        for selection in &self.selections {
            m2::catalogue()
                .table(&selection.table)?
                .row(selection.index)?;
            if !selections.insert((&selection.table, selection.index)) {
                return Err("duplicate descriptor selection".into());
            }
        }
        let mut conditions = BTreeSet::new();
        for i in &self.mef_conditions {
            Reading72::new(Register72::Mef, *i)?;
            if !conditions.insert(*i) {
                return Err("duplicate MEF condition".into());
            }
        }
        let mut frames = BTreeSet::new();
        for code in &self.context_frames {
            if !ContextFrameId::ALL.iter().any(|f| f.code() == code) || !frames.insert(code) {
                return Err("noncanonical or duplicate Context Frame".into());
            }
        }
        if let Some(input) = &self.m1_excitation {
            stamp(&input.stamp, &self.stamp.identity)?;
            nonempty(&input.topology_ref, "topology_ref")?;
            finite(input.audio_octet)?;
            finite(input.nodal_quartet)?;
        }
        if let Some(input) = &self.vimarsha {
            stamp(&input.stamp, &self.stamp.identity)?;
            input.read(self.tick12)?;
        }
        let mut planets = BTreeSet::new();
        if self.world_observations.len() > 10 {
            return Err("planet count exceeds mod-10 (Earth is separate ground)".into());
        }
        for world in &self.world_observations {
            if world.planet_id >= 10 || !planets.insert(world.planet_id) {
                return Err("unknown or duplicate planet ID".into());
            }
            m2::situated_decan(world.longitude_degrees, false)?;
            nonempty(&world.provider_ref, "world provider")?;
            nonempty(&world.source_revision, "world source revision")?;
            if world.observed_at_unix_ms > self.at_unix_ms {
                return Err("world observation is after the requested event".into());
            }
        }
        if let Some(condition) = &self.condition {
            condition.validate(self)?;
        }
        if let Some(state) = &self.resonator {
            validate_resonator(state, &self.stamp.identity)?;
        }
        Ok(())
    }
    pub fn execute(&self) -> Result<M2Frame, String> {
        self.validate()?;
        let catalogue = m2::catalogue();
        let field = ModalField::new(&self.modal_coefficients)?;
        let mef = self
            .mef_conditions
            .iter()
            .map(|i| {
                let sublens = Reading72::new(Register72::Mef, *i)?.mef_sublens()?;
                Ok(MefReading {
                    carrier_index: *i,
                    sublens_ref: sublens.to_string(),
                    lens_name: lens_definition(sublens.lens().lens()).name().into(),
                    source_scope: "#2-1".into(),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let context_frames = self
            .context_frames
            .iter()
            .map(|code| {
                let frame = *ContextFrameId::ALL
                    .iter()
                    .find(|f| f.code() == code)
                    .expect("validated canonical frame");
                ContextReading {
                    code: code.clone(),
                    expression: frame.expression().into(),
                    condition_indices: crate::LensId::ALL
                        .iter()
                        .map(|lens| m2::context_condition(frame, *lens).index())
                        .collect(),
                }
            })
            .collect();
        let selected_descriptors = self
            .selections
            .iter()
            .map(|s| catalogue.reading(&s.table, s.index))
            .collect::<Result<Vec<_>, _>>()?;
        let mut linked_descriptors = Vec::new();
        let mut seen = BTreeSet::new();
        for selection in &self.selections {
            for reading in m2::linked_readings(&selection.table, selection.index)? {
                if seen.insert((reading.table.clone(), reading.index)) {
                    linked_descriptors.push(reading);
                }
            }
        }
        let domains = catalogue
            .tables()
            .iter()
            .map(|table| DomainHolding {
                name: table.name().into(),
                scope: table.scope().into(),
                retained_records: table.rows().len(),
                exact_coordinate_bindings: (0..table.rows().len())
                    .filter(|i| table.binding(*i).is_some())
                    .count(),
                selected_records: self
                    .selections
                    .iter()
                    .filter(|s| s.table == table.name())
                    .count(),
                standing: m2::RETAINED_STANDING.into(),
            })
            .collect();
        let world = self
            .world_observations
            .iter()
            .map(|o| {
                Ok(WorldReading {
                    observation: o.clone(),
                    standing: "supplied-observation-not-authenticated-live-evidence".into(),
                    age_ms: self.at_unix_ms - o.observed_at_unix_ms,
                    planetary_power: catalogue.reading("planet", usize::from(o.planet_id))?,
                    light_decan: catalogue.reading(
                        "decan",
                        usize::from(m2::situated_decan(o.longitude_degrees, false)?.index()),
                    )?,
                    shadow_decan: catalogue.reading(
                        "decan",
                        usize::from(m2::situated_decan(o.longitude_degrees, true)?.index()),
                    )?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut aspects = Vec::new();
        for (i, a) in self.world_observations.iter().enumerate() {
            for b in &self.world_observations[i + 1..] {
                aspects.push(PlanetaryAspect {
                    from_planet: a.planet_id,
                    to_planet: b.planet_id,
                    value: m2::aspect(a.longitude_degrees, b.longitude_degrees)?,
                });
            }
        }
        let continuous_standing = if self.resonator.is_some() {
            "provider-supplied-not-experientially-verified"
        } else {
            "unavailable-no-physical-solver-implied"
        }
        .into();
        let mut frame = M2Frame {
            condition: None,
            schema: m2::ENGINE_CONTRACT.into(),
            registry_revision: catalogue.registry_revision().into(),
            ledger_revision: crate::m_ledger::native_m_ledger()?.ledger_revision,
            identity: self.stamp.identity.clone(),
            input_stamp: self.stamp.clone(),
            at_unix_ms: self.at_unix_ms,
            tick12: self.tick12,
            degree720: self.degree720,
            source_locks: catalogue.sources().to_vec(),
            structural_coordinate_count: m2::m2_coordinates().count(),
            structural_relation_count: m2::m2_relations().count(),
            numerical_ground: NumericGround {
                phi_polynomial: [-1, -1, 1],
                phi: (1.0 + 5f64.sqrt()) / 2.0,
                base36: crate::self_register(),
                field72: crate::field_cardinality(),
                form64: crate::binary_register(),
                retained_whole: crate::RETAINED_ONE,
                third_spanda: crate::cardinality_sum(),
                elemental_aperture: [100, 5, 20],
            },
            domains,
            mef,
            context_frames,
            selected_descriptors,
            linked_descriptors,
            modal: ModalState {
                contract_ref: crate::TEMPLATEURE_FIELD_CONTRACT_REF.into(),
                carrier_order: [
                    MaterialFibre::Earth,
                    MaterialFibre::Fire,
                    MaterialFibre::Water,
                    MaterialFibre::Air,
                ],
                coefficient_unit: "exact-integer-kernel-units; max |component|=1000000".into(),
                coefficients: field.coefficients().to_vec(),
                quadrature: field.quadrature().coefficients().to_vec(),
                form_potential: field.form_potential().to_vec(),
                power_decimal: field.total_power().to_string(),
                transformation:
                    "I4 tensor T18->16; coherent fold 16->0,17->8; not scalar/legacy-OR DET".into(),
            },
            m1_excitation: self.m1_excitation.clone(),
            vimarsha: self
                .vimarsha
                .as_ref()
                .map(|i| i.read(self.tick12))
                .transpose()?,
            world,
            aspects,
            resonator: self.resonator.clone(),
            continuous_standing,
        };
        frame.condition = self
            .condition
            .as_ref()
            .map(|input| crate::m2_condition::resolve(input, &frame))
            .transpose()?;
        Ok(frame)
    }
}
