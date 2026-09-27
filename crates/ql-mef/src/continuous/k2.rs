//! The K² played torus: the retained M1 topological body on which M2's eight
//! Vimarśā voices sound as surface modes (QL-MEF #135).
//!
//! Source laws carried here, each named in the influence reading:
//! - rest geometry is the native M1 torus `m1::torus` (R = 16/9, r = 1, #1-5-1);
//! - each voice `i` sounds `audio_octet_hz[i]` through the accepted K8 binding;
//! - its surface shape is the M2′ double-Fourier Chladni term over toroidal φ and
//!   poloidal θ with L = 2π, mode numbers `nodal_quartet[i % 4].{m, n}`, weight
//!   `f_i / max f` and phase `(address72 + i + 1)·π/36`, displaced along the
//!   surface normal;
//! - a changed nodal quartet or 72-address re-reads the shapes of the SAME voices
//!   through the explicit K8 shape replacement — resident modal state continues.
//!
//! Scale, damping, strike amplitude and output gain have no source table. They
//! are declared material policy with that standing, never presented as source.
use std::f64::consts::PI;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::coupled::{CoupledBasis, CoupledFieldSession, CoupledInput, FrequencyBinding};
use super::{ClockInput, FieldInput, FieldSample, FieldUnits, LiftInput};
use crate::m1_engine::M1Engine;
use crate::m2_engine::{
    CarrierWeight, ContinuousMode, MaterialFibre, PhysicalParameter, ResonatorState,
};

pub const CONFIG: &str = "ql.k2-expression-config/v1";
pub const PROVIDER: &str = "ql.k2-torus-provider/v1";
pub const INFLUENCE: &str = "ql.expression-influence/v1";
pub const VOICES: usize = 8;
const CONSTITUENT: &str = "#1-5-1";
const VOICE_COORDINATE: &str = "#2-1";
const MAX_SAMPLES: usize = 262_144 / VOICES;
const MATERIAL_STANDING: &str = "declared-material-policy: no source table fixes presentation scale, damping, strike amplitude or output gain (QL-MEF #135 design question)";

/// Sampling of the one torus surface. PPS §3 gives 128×64 as the typical mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct K2Geometry {
    /// Toroidal φ samples (the large circle).
    pub longitude_samples: u16,
    /// Poloidal θ samples (the small circle).
    pub latitude_samples: u16,
    /// Declared metres per dimensionless torus unit (r = 1).
    pub metres_per_unit: f64,
    /// K8 clock attachment of every sample: 0 fixed, 1 inscription, 2 lensing.
    pub attachment: u8,
}

/// Declared material policy; its standing travels with every reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct K2Material {
    pub damping_per_second: f64,
    /// Modal amplitude (metres) applied by an explicit strike.
    pub strike_metres: f64,
    pub audio_gain_per_metre: f64,
    /// Whether a determinant event re-excites the voices from `strike_metres`.
    pub strike_on_event: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct K2Field {
    pub subject_ref: String,
    pub sample_rate: u32,
    pub clock: ClockInput,
    pub driver_numerator: u32,
    pub driver_denominator: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct K2Config {
    pub schema: String,
    pub instance_ref: String,
    /// Complete coupled M1/M2/M3 event. The provider supplies the resonator and
    /// frequency bindings; a caller-supplied one is refused, never merged.
    pub basis: CoupledInput,
    pub field: K2Field,
    pub geometry: K2Geometry,
    pub material: K2Material,
}

/// One voice's source-read surface term.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Voice {
    pub frequency_hz: f64,
    pub m: u8,
    pub n: u8,
    pub helix: &'static str,
    pub ql_position: u8,
    pub weight: f64,
    pub phase_radians: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShapeBasis {
    pub shape_ref: String,
    pub address72: u8,
    pub voices: [Voice; VOICES],
}

fn finite_positive(value: f64, label: &str) -> Result<(), String> {
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("K² {label} must be finite and positive"));
    }
    Ok(())
}

impl K2Config {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != CONFIG {
            return Err("unsupported K² expression configuration".into());
        }
        if self.instance_ref.is_empty()
            || self.instance_ref.len() > 2048
            || self.instance_ref.chars().any(char::is_control)
        {
            return Err("invalid K² instance reference".into());
        }
        if self.basis.m2.resonator.is_some()
            || !self.basis.frequency_bindings.is_empty()
            || !self.basis.condition_frequency_bindings.is_empty()
        {
            return Err(
                "the K² provider owns the voices; a supplied resonator or binding is refused"
                    .into(),
            );
        }
        let g = &self.geometry;
        let count = usize::from(g.longitude_samples) * usize::from(g.latitude_samples);
        if g.longitude_samples < 4 || g.latitude_samples < 4 || count > MAX_SAMPLES {
            return Err(format!(
                "K² sampling must be at least 4×4 and at most {MAX_SAMPLES} samples"
            ));
        }
        finite_positive(g.metres_per_unit, "metres per unit")?;
        if g.metres_per_unit > 1000.0 || g.attachment > 2 {
            return Err("K² scale or clock attachment outside the supported range".into());
        }
        let m = &self.material;
        if !m.damping_per_second.is_finite()
            || m.damping_per_second < 0.0
            || m.damping_per_second > 1e6
        {
            return Err("K² damping must be finite and non-negative".into());
        }
        finite_positive(m.strike_metres, "strike amplitude")?;
        if m.strike_metres > 1.0
            || !m.audio_gain_per_metre.is_finite()
            || m.audio_gain_per_metre.abs() > 1e6
        {
            return Err("K² strike or gain outside the supported range".into());
        }
        Ok(())
    }
}

/// The native torus point and its outward unit normal at (θ poloidal, φ toroidal).
fn surface(theta: f64, phi: f64) -> Result<([f64; 3], [f64; 3]), String> {
    let point = crate::m1::torus(theta, phi)?;
    let normal = [
        theta.cos() * phi.cos(),
        theta.cos() * phi.sin(),
        theta.sin(),
    ];
    Ok(([point.x, point.y, point.z], normal))
}

/// M2′ Chladni term on the torus, L = 2π (M2-ARCHITECTURE §5.3.1).
fn chi(voice: &Voice, phi: f64, theta: f64) -> f64 {
    let (m, n) = (f64::from(voice.m), f64::from(voice.n));
    let a = voice.weight * voice.phase_radians.cos();
    let b = voice.weight * voice.phase_radians.sin();
    a * (m * phi / 2.0).sin() * (n * theta / 2.0).sin()
        + b * (m * phi / 2.0).cos() * (n * theta / 2.0).cos()
}

fn angles(geometry: &K2Geometry) -> impl Iterator<Item = (u64, f64, f64)> + '_ {
    let (lon, lat) = (geometry.longitude_samples, geometry.latitude_samples);
    (0..lat).flat_map(move |j| {
        (0..lon).map(move |i| {
            let phi = 2.0 * PI * f64::from(i) / f64::from(lon);
            let theta = 2.0 * PI * f64::from(j) / f64::from(lat);
            (u64::from(j) * u64::from(lon) + u64::from(i), phi, theta)
        })
    })
}

impl ShapeBasis {
    /// Reads the voices from a composed event's own M2 Vimarśā output.
    pub fn from_basis(basis: &CoupledBasis) -> Result<Self, String> {
        let reading = &basis.m2["vimarsha"]["reading"];
        let octet = reading["audio_octet_hz"]
            .as_array()
            .filter(|v| v.len() == VOICES)
            .ok_or("composed event lacks the eight Vimarśā frequencies")?;
        let quartet = reading["nodal_quartet"]
            .as_array()
            .filter(|v| v.len() == 4)
            .ok_or("composed event lacks the Vimarśā nodal quartet")?;
        let address72 = basis.derivation["mef_table_index"]
            .as_u64()
            .filter(|v| *v < 72)
            .ok_or("composed event lacks its 72-address")? as u8;
        let hz: Vec<f64> = octet
            .iter()
            .map(|v| v.as_f64().filter(|f| f.is_finite() && *f > 0.0))
            .collect::<Option<_>>()
            .ok_or("non-finite Vimarśā frequency")?;
        let max = hz.iter().cloned().fold(0.0, f64::max);
        let mut voices = [Voice {
            frequency_hz: 0.0,
            m: 0,
            n: 0,
            helix: "",
            ql_position: 0,
            weight: 0.0,
            phase_radians: 0.0,
        }; VOICES];
        let mut shape_ref = format!("{PROVIDER}:a{address72}");
        for (i, voice) in voices.iter_mut().enumerate() {
            let node = &quartet[i % 4];
            let small = |key: &str| node[key].as_u64().filter(|v| (1..=12).contains(v));
            let (m, n) = (
                small("m").ok_or("nodal m outside 1..12")? as u8,
                small("n").ok_or("nodal n outside 1..12")? as u8,
            );
            let helix = match node["helix"].as_str() {
                Some("bimba") => "bimba",
                Some("pratibimba") => "pratibimba",
                _ => return Err("unknown Vimarśā helix".into()),
            };
            *voice = Voice {
                frequency_hz: hz[i],
                m,
                n,
                helix,
                ql_position: node["ql_position"].as_u64().unwrap_or(0) as u8,
                weight: hz[i] / max,
                phase_radians: f64::from(u16::from(address72) + i as u16 + 1) * PI / 36.0,
            };
            if i < 4 {
                shape_ref.push_str(&format!(":{m}x{n}"));
            }
        }
        Ok(Self {
            shape_ref,
            address72,
            voices,
        })
    }

    /// Nodal lines depend on mode numbers and 72-address only; frequency moves
    /// the weights, which is part of the same re-read.
    pub fn shapes(&self, geometry: &K2Geometry) -> Result<Vec<Vec<[f64; 3]>>, String> {
        angles(geometry)
            .map(|(_, phi, theta)| {
                let (_, normal) = surface(theta, phi)?;
                Ok(self
                    .voices
                    .iter()
                    .map(|voice| {
                        let x = chi(voice, phi, theta);
                        [normal[0] * x, normal[1] * x, normal[2] * x]
                    })
                    .collect())
            })
            .collect()
    }

    pub fn samples(&self, geometry: &K2Geometry) -> Result<Vec<FieldSample>, String> {
        let shapes = self.shapes(geometry)?;
        angles(geometry)
            .zip(shapes)
            .map(|((identity, phi, theta), mode_shapes)| {
                let (point, _) = surface(theta, phi)?;
                Ok(FieldSample {
                    identity,
                    constituent: CONSTITUENT.into(),
                    attachment: geometry.attachment,
                    rest_metres: point.map(|x| x * geometry.metres_per_unit),
                    mode_shapes,
                })
            })
            .collect()
    }
}

fn resonator(input: &CoupledInput, material: &K2Material, fibre: MaterialFibre) -> ResonatorState {
    let first = (fibre.index() * 18) as u8;
    ResonatorState {
        stamp: input.m2.stamp.clone(),
        provider_ref: PROVIDER.into(),
        geometry_ref: format!("{PROVIDER}:m1-torus#1-5-1"),
        material_ref: format!("{PROVIDER}:declared-linear-medium"),
        material_model_ref: "ql.continuous-linear-mode/v1".into(),
        material_parameters: [(
            "damping_per_second".to_owned(),
            PhysicalParameter {
                value: material.damping_per_second,
                unit: "1/s".into(),
                source_ref: MATERIAL_STANDING.into(),
            },
        )]
        .into(),
        modes: (0..VOICES)
            .map(|i| ContinuousMode {
                mode_ref: format!("k2:voice/{i}"),
                source_coordinate: VOICE_COORDINATE.into(),
                material_fibre: fibre,
                // The condition fixes the element; no carrier within its fibre is
                // preferred by source, so the weight is spread evenly over all 18.
                carrier_weights: (0..18)
                    .map(|c| CarrierWeight {
                        carrier: first + c,
                        weight: 1.0 / 18.0,
                    })
                    .collect(),
                // Bound to the octet by the coupled composer; never sounded as 1 Hz.
                frequency_hz: 1.0,
                amplitude: [material.strike_metres, 0.0],
                excitation: [0.0, 0.0],
                damping_per_second: material.damping_per_second,
                nodal_state_ref: format!("k2:voice/{i}/nodal-quartet/{}", i % 4),
                antinodal_state_ref: format!("k2:voice/{i}/octet/{i}"),
            })
            .collect(),
    }
}

/// Completes a caller's event with the provider's voices. The element comes from
/// the event's own M2 condition path; an event without one is refused.
pub fn complete(
    event: &CoupledInput,
    material: &K2Material,
) -> Result<(CoupledInput, CoupledBasis), String> {
    let mut input = event.clone();
    input.frequency_bindings = (0..VOICES)
        .map(|i| FrequencyBinding {
            mode_ref: format!("k2:voice/{i}"),
            octet_index: i as u8,
        })
        .collect();
    input.m2.resonator = Some(resonator(&input, material, MaterialFibre::Earth));
    let first = input.compose()?;
    let fibre: MaterialFibre = serde_json::from_value(
        first.m2["condition"]["source_path"]["material_fibre"].clone(),
    )
    .map_err(
        |_| "K² voices need the active M2 condition's source element; this event admits none",
    )?;
    if fibre == MaterialFibre::Earth {
        return Ok((input, first));
    }
    input.m2.resonator = Some(resonator(&input, material, fibre));
    let basis = input.compose()?;
    Ok((input, basis))
}

pub const BINDING_REQUEST: &str = "ql.k2-binding-request/v1";
pub const BINDING: &str = "oi.native-expression-binding/v1";
const DEFAULT_EVENT: &str = include_str!("../../../../fixtures/kernel/k2-default-event-v1.json");

/// What a consumer asks for: an event (or QL's default starting event), an
/// optional dated sky, and the retained renderer's particle texture.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindingRequest {
    pub schema: String,
    pub instance_ref: String,
    pub texture: [u32; 2],
    pub units_per_metre: f64,
    #[serde(default)]
    pub event: Option<CoupledInput>,
    /// An accepted `ql.sky-snapshot/v1`; its bodies become M2 world observations.
    #[serde(default)]
    pub sky: Option<Value>,
    #[serde(default)]
    pub field: Option<K2Field>,
    #[serde(default)]
    pub geometry: Option<K2Geometry>,
    #[serde(default)]
    pub material: Option<K2Material>,
}

pub fn default_geometry() -> K2Geometry {
    K2Geometry {
        longitude_samples: 128,
        latitude_samples: 64,
        metres_per_unit: 1.0,
        attachment: 1,
    }
}

pub fn default_material() -> K2Material {
    K2Material {
        damping_per_second: 0.35,
        strike_metres: 0.08,
        audio_gain_per_metre: 1.0,
        strike_on_event: true,
    }
}

/// Inscription carries the body at one degree per second (M3's one-degree step
/// at the 1 Hz world clock); lensing keeps its native 9:8 relation.
pub fn default_field(subject_ref: &str) -> K2Field {
    K2Field {
        subject_ref: subject_ref.into(),
        sample_rate: 48_000,
        clock: ClockInput {
            inscription: LiftInput {
                turns: "0".into(),
                half_degrees: 0,
            },
            lensing: LiftInput {
                turns: "0".into(),
                half_degrees: 0,
            },
            grid_origins: [3, 9, 21],
            rate_numerators: ["9".into(), "8".into()],
            rate_denominator: 8,
            rate_remainders: ["0".into(), "0".into()],
            generation: "0".into(),
        },
        driver_numerator: 2,
        driver_denominator: 1,
    }
}

/// Attaches an accepted sky snapshot as M2 world observations, exactly as the
/// K8 sky provider's own `attach_m2` does. Occurrence follows the receipt.
fn attach_sky(event: &mut CoupledInput, sky: &Value) -> Result<(), String> {
    if sky["schema"] != "ql.sky-snapshot/v1" {
        return Err("sky must be an accepted ql.sky-snapshot/v1".into());
    }
    if sky["source_binding"]["registry_revision"] != json!(event.m2.registry_revision) {
        return Err("sky snapshot and M2 registry revision differ".into());
    }
    let received = sky["receipt_unix_ms"]
        .as_u64()
        .filter(|v| *v <= 9_007_199_254_740_991)
        .ok_or("sky receipt time missing")?;
    let snapshot_ref = sky["snapshot_ref"]
        .as_str()
        .ok_or("sky snapshot_ref missing")?;
    let revision = sky["provider"]["adapter_sha256"]
        .as_str()
        .ok_or("sky adapter digest missing")?;
    let bodies = sky["bodies"].as_array().ok_or("sky bodies missing")?;
    event.m2.world_observations = bodies
        .iter()
        .map(|b| {
            Ok(crate::m2_engine::WorldObservation {
                planet_id: b["native_planet_id"]
                    .as_u64()
                    .filter(|v| *v < 10)
                    .ok_or("bad planet id")? as u8,
                longitude_degrees: b["longitude_degrees"].as_f64().ok_or("bad longitude")?,
                provider_ref: snapshot_ref.into(),
                source_revision: revision.into(),
                observed_at_unix_ms: received,
            })
        })
        .collect::<Result<_, String>>()?;
    event.m2.at_unix_ms = received;
    event.m3.occurrence_unix_ms = received;
    event.m3.receipt_unix_ms = received;
    Ok(())
}

/// A complete `oi.native-expression-binding/v1`: the K² host configuration and
/// a declared presentation. Particle `p` follows sample `p mod N` in both
/// targets — a stated correspondence, not a resampling of the body.
pub fn binding(request: BindingRequest) -> Result<Value, String> {
    if request.schema != BINDING_REQUEST {
        return Err("unsupported K² binding request".into());
    }
    let [width, height] = request.texture;
    let particles = u64::from(width) * u64::from(height);
    if width == 0 || height == 0 || particles > 1_048_576 {
        return Err("retained texture must hold 1..1048576 particles".into());
    }
    if !request.units_per_metre.is_finite()
        || request.units_per_metre <= 0.0
        || request.units_per_metre > 1_000_000.0
    {
        return Err("presentation units per metre must be in (0, 1000000]".into());
    }
    let mut event = match request.event {
        Some(event) => event,
        None => serde_json::from_str(DEFAULT_EVENT).map_err(|e| e.to_string())?,
    };
    if let Some(sky) = &request.sky {
        attach_sky(&mut event, sky)?;
    }
    let subject = event.m3.subject_ref.clone();
    let config = K2Config {
        schema: CONFIG.into(),
        instance_ref: request.instance_ref,
        basis: event,
        field: request.field.unwrap_or_else(|| default_field(&subject)),
        geometry: request.geometry.unwrap_or_else(default_geometry),
        material: request.material.unwrap_or_else(default_material),
    };
    config.validate()?;
    // Compose once here so an unusable event is refused before any owner opens.
    complete(&config.basis, &config.material)?;
    let samples =
        u64::from(config.geometry.longitude_samples) * u64::from(config.geometry.latitude_samples);
    let slots: Vec<u64> = (0..particles).map(|p| p % samples).collect();
    Ok(json!({
        "schema": BINDING,
        "host": config,
        "presentation": {"units_per_metre": request.units_per_metre, "slots_a": slots, "slots_b": slots},
    }))
}

/// Re-issues an event under the next M2 generation. Every stamp carrying the
/// same M2 identity moves together; an event already newer is left alone.
fn next_generation(event: &CoupledInput, applied: u64) -> Result<CoupledInput, String> {
    let identity = &event.m2.stamp.identity;
    if identity.profile_generation > applied {
        return Ok(event.clone());
    }
    let next = applied.checked_add(1).ok_or("M2 generation exhausted")?;
    let old = serde_json::to_value(identity).map_err(|e| e.to_string())?;
    fn walk(value: &mut Value, old: &Value, next: u64) {
        match value {
            Value::Object(map) => {
                if map.get("identity") == Some(old) {
                    map.insert(
                        "identity".into(),
                        json!({"event_ref": old["event_ref"], "profile_generation": next}),
                    );
                }
                for child in map.values_mut() {
                    walk(child, old, next);
                }
            }
            Value::Array(items) => items.iter_mut().for_each(|item| walk(item, old, next)),
            _ => {}
        }
    }
    let mut value = serde_json::to_value(event).map_err(|e| e.to_string())?;
    walk(&mut value, &old, next);
    serde_json::from_value(value).map_err(|e| e.to_string())
}

/// One live K² instrument over one native coupled owner.
pub struct K2Instrument {
    instance_ref: String,
    geometry: K2Geometry,
    material: K2Material,
    session: CoupledFieldSession,
    shape: ShapeBasis,
    event: CoupledInput,
}

impl K2Instrument {
    pub fn open(worker: &Path, config: K2Config, timeout: Duration) -> Result<Self, String> {
        config.validate()?;
        let (input, basis) = complete(&config.basis, &config.material)?;
        let shape = ShapeBasis::from_basis(&basis)?;
        let field = FieldInput {
            subject_ref: config.field.subject_ref.clone(),
            sample_rate: config.field.sample_rate,
            clock: config.field.clock.clone(),
            driver_numerator: config.field.driver_numerator,
            driver_denominator: config.field.driver_denominator,
            units: FieldUnits {
                amplitude: "m".into(),
                excitation: "m/s".into(),
                shape: "dimensionless".into(),
                position: "m".into(),
                audio: "linear".into(),
            },
            audio_gains: vec![config.material.audio_gain_per_metre; VOICES],
            samples: shape.samples(&config.geometry)?,
            shape_ref: Some(shape.shape_ref.clone()),
        };
        let session = CoupledFieldSession::open(worker, input.clone(), field, timeout)?;
        Ok(Self {
            instance_ref: config.instance_ref,
            geometry: config.geometry,
            material: config.material,
            session,
            shape,
            event: input,
        })
    }

    pub fn instance_ref(&self) -> &str {
        &self.instance_ref
    }
    pub fn session(&self) -> &CoupledFieldSession {
        &self.session
    }
    pub fn session_mut(&mut self) -> &mut CoupledFieldSession {
        &mut self.session
    }
    pub fn shape(&self) -> &ShapeBasis {
        &self.shape
    }
    /// The caller's event, without the provider-owned voices.
    pub fn event(&self) -> CoupledInput {
        let mut event = self.event.clone();
        event.m2.resonator = None;
        event.frequency_bindings.clear();
        event
    }

    /// Recompose from a changed event: frequencies continue (or are struck), and
    /// a re-read nodal quartet/72-address reshapes the same voices explicitly.
    pub fn replace(&mut self, event: &CoupledInput, strike: bool) -> Result<Value, String> {
        let event = next_generation(event, self.event.m2.stamp.identity.profile_generation)?;
        let (input, basis) = complete(&event, &self.material)?;
        let shape = ShapeBasis::from_basis(&basis)?;
        let mut field = self.session.replace_field_state(input.clone(), strike)?;
        self.event = input;
        if shape.shape_ref != self.shape.shape_ref || shape.voices != self.shape.voices {
            let shapes = shape.shapes(&self.geometry)?;
            field = self.session.replace_shapes(&shape.shape_ref, shapes)?;
            self.shape = shape;
        }
        Ok(field)
    }

    /// M1's own advance action, then the whole event is re-read.
    pub fn m1_advance(&mut self, ticks: u64) -> Result<Value, String> {
        if ticks == 0 || ticks > 1_000_000 {
            return Err("M1 advance must be 1..1000000 ticks".into());
        }
        let mut event = self.event();
        let mut m1 = M1Engine::new(event.m1.clone())?;
        let revision: u64 = event
            .m1
            .revision
            .parse()
            .map_err(|_| "invalid M1 revision")?;
        m1.advance(revision, ticks)?;
        event.m1 = m1.config().clone();
        self.replace(&event, self.material.strike_on_event)
    }

    pub fn set_axis(&mut self, axis: u8, phase: LiftInput) -> Result<Value, String> {
        self.session.set_axis_field(axis, phase)
    }

    /// Where each acting influence comes from, what it drives and on what warrant.
    pub fn influence(&self) -> Value {
        let basis = self.session.current_basis();
        let m1 = &basis.m1;
        let field = self.session.last_field();
        json!({
            "schema": INFLUENCE,
            "instance_ref": self.instance_ref,
            "event_ref": field["event_ref"],
            "subject_ref": field["subject_ref"],
            "generation": field["generation"],
            "samples_elapsed": field["samples_elapsed"],
            "m1_revision": m1["config"]["revision"],
            "m3_generation": basis.m3["generation"],
            "shape_ref": self.shape.shape_ref,
            "address72": self.shape.address72,
            "voices": self.shape.voices,
            "geometry": self.geometry,
            "material": self.material,
            "material_standing": MATERIAL_STANDING,
            "effects": [
                {"determinant":"m1.clock.tick12 / m1.config.lens12 / m1.config.context_frame / m3.form.pose",
                 "through":"m2.vimarsha.reading.audio_octet_hz[i]",
                 "effect":"voice i frequency: PCM pitch and modal oscillation of the torus surface",
                 "units":"Hz", "range":"~98..4000", "timing":"per determinant event; continuous integration between",
                 "consumer":"K8 C++ modal owner → native PCM and GPU target texture",
                 "warrant":"source-defined (Vimarśā reader; K8 frequency binding)"},
                {"determinant":"same as above plus m3 codon/rotation",
                 "through":"m2.vimarsha.reading.nodal_quartet[i % 4].{m,n}",
                 "effect":"nodal lines of voice i on the torus surface (double-Fourier Chladni, L = 2π)",
                 "units":"mode numbers", "range":"1..12", "timing":"re-read per determinant event via explicit K8 shape replacement",
                 "consumer":"K8 target samples → GPU spring targets",
                 "warrant":"source-defined mode numbers (M2-ARCHITECTURE §5.3.1); torus reading of the plate term is source-directed"},
                {"determinant":"m1.config.lens12 + tick12 % 6 (MEF sublens)",
                 "through":"derivation.mef_table_index (address72)",
                 "effect":"phase (a/b balance) of every voice's surface term",
                 "units":"radians", "range":"(address72 + i + 1)·π/36", "timing":"per determinant event",
                 "consumer":"K8 target samples",
                 "warrant":"source-defined (M2-ARCHITECTURE §5.3.1; meaning-packet phase law)"},
                {"determinant":"m1 torus #1-5-1",
                 "through":"m1::torus(θ, φ), R = 16/9, r = 1",
                 "effect":"rest body of every sample",
                 "units":"torus units × metres_per_unit", "range":"|x|,|y| ≤ 25/9, |z| ≤ 1", "timing":"fixed per instance",
                 "consumer":"K8 rest samples → GPU targets",
                 "warrant":"source-defined (KERNEL-M1-ENGINE-CONTRACT)"},
                {"determinant":"coupled clock inscription/lensing",
                 "through":"K8 native C clock",
                 "effect":"rotation of attached samples about the torus axis",
                 "units":"half-degrees", "range":"0..719 plus winding", "timing":"continuous with audio samples",
                 "consumer":"K8 write_targets",
                 "warrant":"source-defined (K8 structural contract)"},
                {"determinant":"material policy",
                 "through":"damping, strike, gain, metres_per_unit",
                 "effect":"decay, loudness and visible amplitude",
                 "units":"1/s, m, 1/m, m", "range":"declared", "timing":"per instance",
                 "consumer":"K8 modal owner",
                 "warrant":"declared policy — not source"}
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voice(m: u8, n: u8, weight: f64, phase: f64) -> Voice {
        Voice {
            frequency_hz: 220.0,
            m,
            n,
            helix: "bimba",
            ql_position: 0,
            weight,
            phase_radians: phase,
        }
    }

    #[test]
    fn chladni_term_is_the_m2_plate_law_at_l_two_pi() {
        let v = voice(3, 5, 0.5, 0.3);
        let (phi, theta) = (1.1_f64, 2.3_f64);
        let expect = 0.5 * 0.3_f64.cos() * (1.5 * phi).sin() * (2.5 * theta).sin()
            + 0.5 * 0.3_f64.sin() * (1.5 * phi).cos() * (2.5 * theta).cos();
        assert!((chi(&v, phi, theta) - expect).abs() < 1e-15);
    }

    #[test]
    fn rest_body_is_the_native_m1_torus_and_shapes_are_normal() {
        let geometry = K2Geometry {
            longitude_samples: 8,
            latitude_samples: 4,
            metres_per_unit: 0.5,
            attachment: 1,
        };
        let basis = ShapeBasis {
            shape_ref: "t".into(),
            address72: 0,
            voices: [voice(1, 2, 1.0, 0.0); VOICES],
        };
        let samples = basis.samples(&geometry).unwrap();
        assert_eq!(samples.len(), 32);
        for (k, s) in samples.iter().enumerate() {
            assert_eq!(s.identity, k as u64);
            let (phi, theta) = (
                2.0 * PI * (k % 8) as f64 / 8.0,
                2.0 * PI * (k / 8) as f64 / 4.0,
            );
            let t = crate::m1::torus(theta, phi).unwrap();
            assert!((s.rest_metres[0] - 0.5 * t.x).abs() < 1e-12);
            assert!((s.rest_metres[2] - 0.5 * t.z).abs() < 1e-12);
            // Displacement is parallel to the outward normal.
            let n = [
                theta.cos() * phi.cos(),
                theta.cos() * phi.sin(),
                theta.sin(),
            ];
            let d = s.mode_shapes[0];
            let cross = [
                n[1] * d[2] - n[2] * d[1],
                n[2] * d[0] - n[0] * d[2],
                n[0] * d[1] - n[1] * d[0],
            ];
            assert!(cross.iter().all(|c| c.abs() < 1e-12));
        }
    }

    #[test]
    fn different_mode_numbers_move_different_nodal_lines() {
        let geometry = K2Geometry {
            longitude_samples: 32,
            latitude_samples: 16,
            metres_per_unit: 1.0,
            attachment: 0,
        };
        let shape = |m, n| ShapeBasis {
            shape_ref: "t".into(),
            address72: 0,
            voices: [voice(m, n, 1.0, 0.7); VOICES],
        };
        assert_ne!(
            shape(2, 3).shapes(&geometry).unwrap(),
            shape(4, 3).shapes(&geometry).unwrap()
        );
    }
}
