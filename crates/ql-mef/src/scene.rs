//! One integrated M1–M2–M3 event for the instrument: the dated sky placed on
//! the clock, each planet voiced by the M2 synth, the form advanced by M1, and
//! the Nara centres the sky reaches.
//!
//! - M2-5 is the sky: each body sits at its ecliptic degree (θ = λ) with the
//!   clock's seed there (`scene_sky`, `m3_inscription`).
//! - The M2 synth voices every planet the map tunes at M1's root times its just
//!   ratio (`m2_sky`); retrograde reverses its modulation direction, and its
//!   daily motion sets the modulation rate against the Spanda beat.
//! - M1's ring state at (tick12, cycle) advances the form (`spanda_field`).
//! - Each chakra centre under the Sun (`#2-5-0/1-1..7`) receives the planets
//!   that resonate with it (`PLANETARY_RESONANCE`); Earth `#2-5-0/1-0` observes.
//!
//! Every magnitude that is a choice, not a map fact, is a named tunable (D30).

use crate::continuous::coupled::SKY_ROOT_HZ;
use crate::m2_sky::just_ratio;
use crate::scene_sky::{PlacedBody, place};
use crate::spanda_field::{HkbParams, ring_codon_advance};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const SCENE: &str = "ql.scene/v1";
pub const WORLD_REQUEST: &str = "ql.scene-world-request/v1";
pub const WORLD: &str = "ql.scene-world/v1";

/// Starting instrument settings are authored choices. Source geometry, ratios,
/// form transcription and reception routes still come from their native owners.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StartRecipe {
    pub tick12: u8,
    pub cycle: u64,
    pub aperture: u8,
    pub harmonic_basis_index: u8,
    pub maqam_index: u8,
    pub lens12: u8,
    pub context_frame: u8,
    /// Native held-state continuation. Absence chooses the opening ring form;
    /// consumers pass the admitted owner readback verbatim, never infer it.
    pub m3_address: Option<u8>,
    pub m3_pose: Option<u8>,
    pub m3_clock_steps: Option<u64>,
    pub matrix_axis: Option<u8>,
    pub rna: Option<bool>,
    pub continuous_clock: Option<crate::continuous::ClockInput>,
}

impl Default for StartRecipe {
    fn default() -> Self {
        Self {
            tick12: 0,
            cycle: 0,
            aperture: 9,
            harmonic_basis_index: 0,
            maqam_index: 3,
            lens12: 11,
            context_frame: 7,
            m3_address: None,
            m3_pose: None,
            m3_clock_steps: None,
            matrix_axis: None,
            rna: None,
            continuous_clock: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldRequest {
    pub schema: String,
    pub instance_ref: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub sky: Value,
    pub texture: [u32; 2],
    pub units_per_metre: f64,
    #[serde(default)]
    pub start: StartRecipe,
    #[serde(default)]
    pub geometry: Option<crate::continuous::scene_field::SceneGeometry>,
    #[serde(default)]
    pub material: Option<crate::continuous::scene_field::SceneMaterial>,
}

/// One source-owned constructor: never a copy of the embedded demonstration
/// event. The dated sky, actual M1 ring and M3 state yield both the composition
/// reading and the continuous binding under the supplied subject/occasion.
pub fn world(request: WorldRequest) -> Result<Value, String> {
    use crate::continuous::coupled::{CoupledInput, HarmonicSource};
    use crate::continuous::scene_field::{self, BindingRequest};
    use crate::m1_engine::{Basis, EngineConfig};
    use crate::m2_condition::{CorrespondenceRole, M2ConditionInput, TuningPolicy};
    use crate::m2_engine::{EventIdentity, InputStamp, M2Request};
    use crate::m3_state::M3Request;
    if request.schema != WORLD_REQUEST {
        return Err("unsupported scene world request".into());
    }
    for reference in [
        &request.instance_ref,
        &request.event_ref,
        &request.subject_ref,
    ] {
        if reference.trim().is_empty()
            || reference.len() > 2048
            || reference.chars().any(char::is_control)
            || reference == "ql:k2/default-event"
            || reference == "ql:k2/default-subject"
        {
            return Err(
                "a world requires exact bounded instance, event and subject references".into(),
            );
        }
    }
    // The CLI additionally validates the provider receipt through its owner.
    // Library consumers retain full ten-body identity/provenance validation.
    crate::nara::current::transit(Some(&request.sky))?;
    if request.sky["snapshot_ref"].as_str() != Some(request.event_ref.as_str()) {
        return Err("the personal world event must be the admitted sky snapshot reference".into());
    }
    let start = &request.start;
    let clock = crate::m1::Clock::new(start.cycle, u32::from(start.tick12))?;
    let steps = start
        .cycle
        .checked_mul(360)
        .and_then(|n| n.checked_add(u64::from(clock.degree360)))
        .filter(|n| *n <= crate::m2_engine::MAX_EXACT_JSON_INTEGER)
        .ok_or("starting clock exceeds exact native range")?;
    let time = request.sky["receipt_unix_ms"]
        .as_u64()
        .ok_or("sky receipt missing")?;
    let occurrence = request.sky["epoch_unix_ms"]
        .as_u64()
        .ok_or("sky epoch exceeds the native unsigned occurrence range")?;
    let revision = crate::m_tree::native_m_registry()
        .manifest()
        .registry_revision
        .clone();
    let identity = EventIdentity {
        event_ref: request.event_ref.clone(),
        profile_generation: 1,
    };
    let stamp = |part: &str, contract: &str| InputStamp {
        identity: identity.clone(),
        source_ref: format!("{}#{part}", request.event_ref),
        contract_ref: contract.into(),
    };
    let m2_stamp = stamp("m2", crate::m2_engine::REQUEST_SCHEMA);
    let mut event = CoupledInput {
        schema: crate::continuous::coupled::REQUEST_V2.into(),
        m1: EngineConfig {
            event_ref: request.event_ref.clone(),
            subject_coordinate: "#1".into(),
            selected_coordinate: "#1-2-5".into(),
            revision: "0".into(),
            cycle: start.cycle.to_string(),
            tick12: u32::from(start.tick12),
            family: 5,
            row12: 3,
            col12: 4,
            flowering_substage: 3,
            lens12: start.lens12,
            context_frame: start.context_frame,
            basis: Basis::Fifths,
        },
        m2: M2Request {
            schema: crate::m2_engine::REQUEST_SCHEMA.into(),
            registry_revision: revision.clone(),
            stamp: m2_stamp.clone(),
            at_unix_ms: time,
            tick12: start.tick12,
            degree720: clock.degree720 as u16,
            modal_coefficients: vec![[0, 0]; 72],
            mef_conditions: vec![],
            context_frames: vec![],
            selections: vec![],
            condition: None,
            m1_excitation: None,
            vimarsha: None,
            world_observations: vec![],
            resonator: None,
        },
        m3: M3Request {
            schema: crate::m3_state::REQUEST_SCHEMA.into(),
            registry_revision: revision,
            stamp: stamp("m3-form", crate::m3_state::REQUEST_SCHEMA),
            subject_ref: request.subject_ref.clone(),
            occurrence_unix_ms: occurrence,
            receipt_unix_ms: time,
            clock_steps: start.m3_clock_steps.unwrap_or(steps),
            address: start
                .m3_address
                .unwrap_or_else(|| ring_codon_advance(start.tick12, start.cycle).address()),
            pose: start.m3_pose.unwrap_or(0),
            aperture: start.aperture,
            matrix_axis: start.matrix_axis.unwrap_or(0),
            rna: start.rna.unwrap_or(false),
            bases: vec![],
            m2_basis: Some(m2_stamp),
        },
        m3_commands: vec![],
        harmonic_source: HarmonicSource::CanonicalBasis {
            index: start.harmonic_basis_index,
        },
        frequency_bindings: vec![],
        condition_frequency_bindings: vec![],
        sky_frequency_bindings: vec![],
        source_receipts: vec![request.sky.clone()],
    };
    scene_field::attach_sky(&mut event, &request.sky)?;
    // Derive the active MEF/Vimarsha through the coupled owner before admitting
    // the selected M2 material condition. A condition cannot invent its drive.
    event.m2 = event.compose()?.m2_input;
    let active = event
        .m2
        .vimarsha
        .as_ref()
        .ok_or("native Vimarsha missing")?
        .lens
        * 6
        + event.m2.tick12 % 6;
    event.m2.condition = Some(M2ConditionInput {
        maqam_index: start.maqam_index,
        role: CorrespondenceRole::Tonic,
        active_mef_condition: active,
        tuning: TuningPolicy::BimbaSpelled24Tet,
        tonic_hz: SKY_ROOT_HZ,
        palette: None,
    });
    let mut field = scene_field::default_field(&request.subject_ref);
    field.clock.inscription.turns = (steps / 360).to_string();
    field.clock.inscription.half_degrees = (clock.degree360 * 2) as u16;
    if let Some(held_clock) = &start.continuous_clock {
        field.clock = held_clock.clone();
    }
    let binding = scene_field::binding(BindingRequest {
        schema: scene_field::BINDING_REQUEST.into(),
        instance_ref: request.instance_ref.clone(),
        texture: request.texture,
        units_per_metre: request.units_per_metre,
        event: Some(event.clone()),
        sky: Some(request.sky.clone()),
        field: Some(field),
        geometry: request.geometry,
        material: request.material,
        reception: None,
    })?;
    Ok(
        json!({"schema": WORLD, "instance_ref": request.instance_ref,
        "event_ref": request.event_ref, "subject_ref": request.subject_ref,
        "snapshot_ref": request.sky["snapshot_ref"], "sky": request.sky,
        "starting_recipe": {"settings":request.start,
            "standing":"authored initial instrument settings; native laws and source correspondences are not tunables",
            "m1_source_cell": {"family":5,"row12":3,"col12":4,"flowering_substage":3,"basis":"fifths"},
            "material_initial_state":"zero modal coefficients; voices are struck by declared SceneMaterial"},
        "event": binding["native_basis"]["input"], "basis": binding["native_basis"],
        "scene": binding["scene"], "native_readback": binding["native_readback"],
        "registers": register_catalogue()?,
        "current_form": binding["native_readback"]["form_process"],
        "native_owner_sources": {
            "constructor": source_ref("crates/ql-mef/src/scene.rs", &source_hash(include_str!("scene.rs"))),
            "coupled": source_ref("crates/ql-mef/src/continuous/coupled.rs", &source_hash(include_str!("continuous/coupled.rs"))),
            "field": source_ref("crates/ql-mef/src/continuous/scene_field.rs", &source_hash(include_str!("continuous/scene_field.rs")))
        },
        "binding": binding}),
    )
}

fn source_hash(source: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("sha256:{:x}", Sha256::digest(source.as_bytes()))
}

fn source_ref(reference: &str, revision: &str) -> Value {
    json!({"ref":reference,"revision":revision,"availability":"available"})
}

fn qualify_reference(reference: &str, revision: &str) -> Value {
    let exact_revision = if reference.starts_with("crates/ql-mef/src/m2.rs") {
        source_hash(include_str!("m2.rs"))
    } else if reference == "crates/ql-core/src/pole/aperture.rs" {
        source_hash(include_str!("../../ql-core/src/pole/aperture.rs"))
    } else if reference == "c/registry/promotions/k8-apertures-v1.json"
        || reference.starts_with("#2-0-")
    {
        source_hash(include_str!(
            "../../../c/registry/promotions/k8-apertures-v1.json"
        ))
    } else {
        revision.into()
    };
    source_ref(reference, &exact_revision)
}

fn source_row(reference: &str, angle: Option<f64>) -> Result<Value, String> {
    let content = crate::bimba_content::native_bimba_content()?.coordinate(reference)?;
    let identity = &content["identity"];
    let props = &identity["properties"];
    let title = props["c_1_name"]
        .as_str()
        .or_else(|| props["c_1_description"].as_str())
        .unwrap_or(reference);
    let mut row = json!({"reading":{"ref":identity["canonical_ref"],
            "revision":content["source_revision"],"availability":"available"},
        "coordinate_ref":identity["native_coordinate"],"title":title,
        "source_refs":[source_ref(identity["full_source_ref"].as_str().ok_or("source ref absent")?,
            content["source_revision"].as_str().ok_or("source revision absent")?)],"standing":"canonical"});
    if let Some(degrees) = angle {
        row["display"] = json!({"angle_degrees":degrees,"label":title,"standing":"canonical"});
    }
    Ok(row)
}

/// Complete finite registers, retaining different denominators and qualified
/// source subjects. The renderer can group their overview but cannot erase or
/// replace a register with repeated placeholder rows.
pub fn register_catalogue() -> Result<Value, String> {
    use crate::m3_engine::{M3NodeKind, native_m3_engine};
    let engine = native_m3_engine();
    let mut degree = Vec::with_capacity(360);
    for i in 0..360 {
        degree.push(source_row(
            &engine
                .node(M3NodeKind::Degree, i)
                .ok_or("degree source missing")?
                .source_ref,
            Some(i as f64),
        )?);
    }
    let mut backbone = Vec::with_capacity(24);
    for i in 0..24 {
        let node = engine
            .node(M3NodeKind::Backbone, i)
            .ok_or("backbone source missing")?;
        let start = (0..360)
            .find(|&d| {
                engine
                    .clock(ql_core::m3_clock::M3Clock::at_steps(d))
                    .backbone
                    .id
                    == node.id
            })
            .ok_or("governor has no actual degree arc")?;
        backbone.push(source_row(&node.source_ref, Some(start as f64))?);
    }
    let mut decan = Vec::with_capacity(36);
    for i in 0..36 {
        let degrees = i as f64 * 10.0;
        let route = crate::m2::decan_planet_route(degrees)?;
        decan.push(source_row(&route.decan_coordinate, Some(degrees))?);
    }
    let mut codon = Vec::with_capacity(64);
    for i in 0..64 {
        let node = engine
            .node(M3NodeKind::Codon, i)
            .ok_or("codon source missing")?;
        let mut row = source_row(&node.source_ref, None)?;
        let source = crate::m3_source::native_m3_source()
            .node(node.id)
            .ok_or("codon full source missing")?;
        row["triplet"] = source.properties["p_3_sequence"].clone();
        row["display"] = json!({"angle_degrees":i as f64 * 360.0 / 64.0,
            "label":row["triplet"],"standing":"authored-source-order"});
        codon.push(row);
    }
    let revision = crate::m_tree::native_current_m_registry()
        .manifest()
        .source_revision
        .clone();
    let mut skin = Vec::with_capacity(72);
    for i in 0..72 {
        let reading = crate::m2::Reading72::new(crate::m2::Register72::Mef, i)?;
        let sublens = reading.mef_sublens()?;
        let def = crate::lens_definition(sublens.lens().lens());
        let title = format!("{} · {}", def.name(), sublens);
        skin.push(json!({"reading":{"ref":format!("ql:m2-reading:mef/{i}"),"revision":revision,"availability":"available"},
            "coordinate_ref":"#2-1","title":title,"source_refs":["#2-1",sublens.to_string(),"crates/ql-mef/src/m2.rs#Reading72"],
            "standing":"source-backed-correspondence","native_index":i,"native_axes":reading.axes(),
            "register":"MEF12×6; not Decan72 or a physical eigenmode count",
            "display":{"angle_degrees":f64::from(i)*5.0,"label":title,"standing":"authored-source-order"}}));
    }
    let mut aperture = Vec::with_capacity(18);
    for i in 0..16 {
        let native = ql_core::ApertureIndex::new(i).map_err(|e| e.to_string())?;
        let pair = i.min(15 - i);
        let title = format!(
            "{}° × {}",
            f64::from(native.division_deg10()) / 10.0,
            3600 / native.division_deg10()
        );
        aperture.push(json!({"reading":{"ref":format!("ql:m3-aperture/static/{i}"),"revision":revision,"availability":"available"},
            "coordinate_ref":format!("#2-0-2-{pair}"),"title":title,
            "source_refs":[format!("#2-0-2-{pair}"),"c/registry/promotions/k8-apertures-v1.json","crates/ql-core/src/pole/aperture.rs"],
            "standing":"source-backed-correspondence","native_index":i,"division_deg10":native.division_deg10(),"reciprocal_index":native.reciprocal().index(),
            "display":{"angle_degrees":f64::from(native.orientation().0)/10.0,"label":title,"standing":"canonical"}}));
    }
    for (role, reference, title, slots, quantum) in [
        (
            "fibonacci",
            "#2-0-1",
            "Fibonacci/Pisano ground · 60 × 6°",
            60,
            6.0,
        ),
        (
            "void",
            "#2-0-0",
            "Anuttara void ring · 16 × 22.5°",
            16,
            22.5,
        ),
    ] {
        aperture.push(json!({"reading":{"ref":format!("ql:m3-aperture/{role}"),"revision":revision,"availability":"available"},
            "coordinate_ref":reference,"title":title,"source_refs":[reference,"c/registry/promotions/k8-apertures-v1.json","crates/ql-core/src/pole/aperture.rs"],
            "standing":"source-backed-correspondence","role":role,"positions":slots,"quantum_degrees":quantum,
            "selection_standing":"ground/render register; not a seventeenth/eighteenth static divisor selection"}));
    }
    for rows in [&mut skin, &mut aperture] {
        for row in rows {
            let qualified = row["source_refs"]
                .as_array()
                .ok_or("register source refs absent")?
                .iter()
                .map(|r| qualify_reference(r.as_str().expect("authored source ref"), &revision))
                .collect::<Vec<_>>();
            row["source_refs"] = json!(qualified);
        }
    }
    Ok(
        json!({"schema":"ql.scene-register-catalogue/v1","source_revision":revision,
        "degree":degree,"backbone":backbone,"decan":decan,"codon":codon,"skin":skin,"aperture":aperture}),
    )
}

pub fn current_form(
    basis: &crate::continuous::coupled::CoupledBasis,
    instance_ref: &str,
) -> Result<Value, String> {
    let codon = basis.m3["form"]["codon"]["ref"]
        .as_str()
        .ok_or("current source codon missing")?;
    let hexagram = basis.m3["form"]["hexagram"]["ref"]
        .as_str()
        .ok_or("current source hexagram missing")?;
    let source = crate::bimba_content::native_bimba_content()?.coordinate(hexagram)?;
    let reading = source_row(codon, None)?;
    let mut form_sources = reading["source_refs"]
        .as_array()
        .ok_or("codon source references missing")?
        .clone();
    form_sources.push(source_ref(
        source["identity"]["full_source_ref"]
            .as_str()
            .ok_or("hexagram source reference missing")?,
        source["source_revision"]
            .as_str()
            .ok_or("hexagram source revision missing")?,
    ));
    Ok(json!({"reading":reading,"coordinate_ref":codon,
        "process_subject_ref":format!("ql:scene-form:{instance_ref}"),
        "canonical_subject_ref":reading["reading"]["ref"],"current_reading":reading["reading"],
        "triplet":basis.m3["transcription"]["sequence"],"hexagram_coordinate_ref":hexagram,
        "hexagram_glyph":source["identity"]["properties"]["c_1_symbol"],
        "hexagram_title":source["identity"]["title"],"hexagram_source_ref":source["identity"]["full_source_ref"],
        "hexagram_reading":{"ref":hexagram,"revision":source["source_revision"],"availability":"available"},
        "source_refs":form_sources,
        "source_rule":"M3 native codon transcription and source YIELDS_CODON hexagram correspondence; live hinge is its native form geometry",
        "native_form":basis.m3["form"],"native_hinge":basis.m3["form"]["hinge_geometry"],
        "event_ref":basis.input.m1.event_ref,"subject_ref":basis.input.m3.subject_ref}))
}

/// Read a scene from the exact basis being installed in the field owner. An
/// independently composed tick/ratio/form cannot stand in for this event.
pub fn from_basis(
    snapshot: &Value,
    basis: &crate::continuous::coupled::CoupledBasis,
) -> Result<Value, String> {
    let ratio = &basis.derivation["harmonic_ratio"]["ratio"];
    let num = ratio[0]
        .as_u64()
        .and_then(|n| u16::try_from(n).ok())
        .ok_or("native harmonic numerator missing")?;
    let den = ratio[1]
        .as_u64()
        .and_then(|n| u16::try_from(n).ok())
        .ok_or("native harmonic denominator missing")?;
    let tick = u8::try_from(basis.input.m1.tick12).map_err(|_| "native tick missing")?;
    let cycle = basis
        .input
        .m1
        .cycle
        .parse()
        .map_err(|_| "native cycle missing")?;
    let mut scene = compose(
        snapshot,
        tick,
        cycle,
        SceneTuning {
            m1_harmonic_ratio: [num, den],
            ..SceneTuning::default()
        },
    )?;
    scene["event_ref"] = json!(basis.input.m1.event_ref);
    scene["subject_ref"] = json!(basis.input.m3.subject_ref);
    scene["profile_generation"] = json!(basis.input.m2.stamp.identity.profile_generation);
    scene["form"] = json!({"codon":basis.m3["form"]["address"],
        "native":basis.m3["form"], "advanced_by":"actual admitted M3 state"});
    Ok(scene)
}

/// The scene's tunables. Map facts are not here; only choices are (D30).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneTuning {
    /// M1's harmonic ratio scaling the root (the carrier's own ratio).
    pub m1_harmonic_ratio: [u16; 2],
    /// The root the ratios multiply; the octet's C3.
    pub root_hz: f64,
    /// Modulation rate per degree/day of a planet's motion, in Spanda beats.
    /// Default: the Moon's mean daily motion (13.176°/day) is one beat, so the
    /// fastest body pulses at the Spanda beat and the outer planets breathe.
    pub beats_per_degree_per_day: f64,
}

impl Default for SceneTuning {
    fn default() -> Self {
        Self {
            m1_harmonic_ratio: [1, 1],
            root_hz: SKY_ROOT_HZ,
            beats_per_degree_per_day: 1.0 / 13.176,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Voice {
    pub planet_ref: String,
    pub just_ratio: [u16; 2],
    pub frequency_hz: f64,
    /// +1 direct, −1 retrograde: the direction the modulation turns.
    pub direction: i8,
    pub modulation_hz: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SceneBody {
    #[serde(flatten)]
    pub placed: PlacedBody,
    pub voice: Option<Voice>,
}

/// Compose the scene from an accepted sky snapshot and M1's (tick12, cycle).
pub fn compose(
    snapshot: &Value,
    tick12: u8,
    cycle: u64,
    tuning: SceneTuning,
) -> Result<Value, String> {
    let [num, den] = tuning.m1_harmonic_ratio;
    if tick12 >= 12
        || num == 0
        || den == 0
        || !tuning.root_hz.is_finite()
        || tuning.root_hz <= 0.0
        || !tuning.beats_per_degree_per_day.is_finite()
        || tuning.beats_per_degree_per_day < 0.0
    {
        return Err("invalid scene tick or tuning".into());
    }
    let hkb = HkbParams::default();
    let root = tuning.root_hz * f64::from(num) / f64::from(den);
    let placement = place(snapshot)?;
    let bodies: Vec<SceneBody> = placement
        .bodies
        .into_iter()
        .map(|placed| {
            let voice = placed.planet_ref.as_deref().and_then(|planet| {
                just_ratio(planet).map(|ratio| Voice {
                    planet_ref: planet.to_string(),
                    just_ratio: ratio,
                    frequency_hz: root * f64::from(ratio[0]) / f64::from(ratio[1]),
                    direction: if placed.retrograde { -1 } else { 1 },
                    modulation_hz: hkb.base_freq_hz
                        * tuning.beats_per_degree_per_day
                        * placed.speed_deg_per_day.abs(),
                })
            });
            SceneBody { placed, voice }
        })
        .collect();
    let centres: Vec<Value> = (1..=7)
        .map(|i| {
            let centre = format!("#2-5-0/1-{i}");
            let receiving: Vec<&str> = bodies
                .iter()
                .filter(|b| b.placed.resonant_chakra_ref.as_deref() == Some(centre.as_str()))
                .map(|b| b.placed.body.as_str())
                .collect();
            json!({"centre_ref":centre, "anatomy":crate::m2_sky::node(&centre)
                .and_then(|n| n.properties.get("c_2_anatomical_location").cloned()),
                "receiving":receiving})
        })
        .collect();
    let form = ring_codon_advance(tick12, cycle);
    Ok(json!({
        "schema": SCENE,
        "snapshot_ref": placement.snapshot_ref,
        "epoch_utc": placement.epoch_utc,
        "observer_ref": placement.observer_ref,
        "m1": {"tick12": tick12, "cycle": cycle, "hkb": hkb, "root_hz": root},
        "form": {"codon": form.address(), "advanced_by": "spanda_field::ring_codon_advance"},
        "bodies": bodies,
        "centres": centres,
        "tuning": tuning,
        "sources": {
            "sky_revision": crate::m2_sky::sky().map_content_sha256,
            "registry_revision": crate::m2_sky::sky().registry_revision,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sky() -> Value {
        serde_json::from_str(include_str!(
            "../../../fixtures/kernel/sky-snapshot-2026-09-28-v1.json"
        ))
        .unwrap()
    }

    #[test]
    fn the_dated_sky_composes_one_scene() {
        let scene = compose(&sky(), 3, 7, SceneTuning::default()).unwrap();
        let bodies = scene["bodies"].as_array().unwrap();
        assert_eq!(bodies.len(), 10);
        let by = |name: &str| bodies.iter().find(|b| b["body"] == name).unwrap();
        // Sun 1:1 at the root; Saturn 3:2, retrograde.
        assert_eq!(by("Sun")["voice"]["frequency_hz"], json!(SKY_ROOT_HZ));
        assert_eq!(by("Saturn")["voice"]["just_ratio"], json!([3, 2]));
        assert_eq!(by("Saturn")["voice"]["direction"], json!(-1));
        // Uranus has no map node, so no voice; it still sits on the clock.
        assert!(by("Uranus")["voice"].is_null());
        assert_eq!(by("Uranus")["seed"]["decan_name"], "Gemini Decan 1");
        // Seven centres; Saturn reaches Muladhara, the Sun Sahasrara.
        let centres = scene["centres"].as_array().unwrap();
        assert_eq!(centres.len(), 7);
        assert_eq!(centres[0]["receiving"], json!(["Saturn"]));
        assert_eq!(centres[5]["receiving"], json!(["Moon", "Neptune"]));
        assert_eq!(centres[6]["receiving"], json!(["Sun", "Pluto"]));
        assert_eq!(
            scene["form"]["codon"],
            json!(ring_codon_advance(3, 7).address())
        );
    }

    #[test]
    fn m1_moves_the_root_and_form_but_not_the_sky() {
        let a = compose(&sky(), 3, 7, SceneTuning::default()).unwrap();
        let tuned = SceneTuning {
            m1_harmonic_ratio: [3, 2],
            ..SceneTuning::default()
        };
        let b = compose(&sky(), 9, 7, tuned).unwrap();
        for (x, y) in a["bodies"]
            .as_array()
            .unwrap()
            .iter()
            .zip(b["bodies"].as_array().unwrap())
        {
            assert_eq!(x["seed"], y["seed"], "M1 does not move the sky");
            if let (Some(f), Some(g)) = (
                x["voice"]["frequency_hz"].as_f64(),
                y["voice"]["frequency_hz"].as_f64(),
            ) {
                assert!(
                    (g / f - 1.5).abs() < 1e-12,
                    "the whole sky transposes with M1's ratio"
                );
            }
        }
        assert_ne!(a["form"], b["form"], "the ring state advances the form");
    }
}
