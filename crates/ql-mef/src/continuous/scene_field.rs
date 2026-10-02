//! The integrated scene played on the M1 torus (QL-MEF #135): the dated sky's
//! planets sound as surface modes of the one body, each at its ecliptic degree.
//!
//! Source laws carried here, each named in the influence reading:
//! - rest geometry is the native M1 torus `m1::torus` (R = 16/9, r = 1, #1-5-1);
//!   its large circle is the clock, θ = λ (#254 D6);
//! - each voice is a planet the map tunes (M2-5, `m_2_5_interval_from_root`),
//!   sounded through the continuous owner's sky bus at M1's root times its just
//!   ratio (#254 D13);
//! - its surface shape is the M2′ double-Fourier Chladni term (L = 2π) with
//!   mode numbers `nodal_quartet[i % 4].{m, n}` from the event's Vimarśā reading
//!   and phase `(address72 + i + 1)·π/36`, anchored at the planet's longitude on
//!   the large circle and displaced along the surface normal;
//! - a planet's motion, or a changed quartet or 72-address, re-reads the shapes
//!   of the SAME voices through the explicit K8 shape replacement.
//!
//! Scale, damping, strike amplitude and output gain have no source table. They
//! are declared material policy with that standing, never presented as source.
use std::f64::consts::PI;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::coupled::{CoupledBasis, CoupledFieldSession, CoupledInput, SkyFrequencyBinding};
use super::{ClockInput, FieldInput, FieldSample, FieldUnits, LiftInput};
use crate::m1_engine::M1Engine;
use crate::m2_engine::{
    CarrierWeight, ContinuousMode, MaterialFibre, PhysicalParameter, ResonatorState,
};
use crate::nara::{
    EventBasisRefs, PersonalConstitution, PersonalEventInput, PersonalFieldInstance,
    PersonalFieldState,
};

pub const CONFIG: &str = "ql.scene-expression-config/v1";
pub const PROVIDER: &str = "ql.scene-torus-provider/v1";
pub const INFLUENCE: &str = "ql.expression-influence/v1";
/// The planets the map tunes, in scalar-degree order: Sun, Venus, Mercury,
/// Moon, Saturn, Jupiter, Mars, then Neptune and Pluto.
pub const PLANETS: [&str; 9] = [
    "#2-5-0/1", "#2-5-2", "#2-5-3", "#2-5-4", "#2-5-5", "#2-5-6", "#2-5-7", "#2-5-8", "#2-5-9",
];
pub const VOICES: usize = PLANETS.len();
const CONSTITUENT: &str = "#1-5-1";
const MAX_SAMPLES: usize = 262_144 / VOICES;
const MATERIAL_STANDING: &str = "declared-material-policy: no source table fixes presentation scale, damping, strike amplitude or output gain (QL-MEF #135)";

/// Sampling of the one torus surface (declared; PPS §3 gives 128×64 as a typical render mesh).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneGeometry {
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
pub struct SceneMaterial {
    pub damping_per_second: f64,
    /// Modal amplitude (metres) applied by an explicit strike.
    pub strike_metres: f64,
    pub audio_gain_per_metre: f64,
    /// Whether a determinant event re-excites the voices from `strike_metres`.
    pub strike_on_event: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneField {
    pub subject_ref: String,
    pub sample_rate: u32,
    pub clock: ClockInput,
    pub driver_numerator: u32,
    pub driver_denominator: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneConfig {
    pub schema: String,
    pub instance_ref: String,
    /// Complete coupled M1/M2/M3 event. The provider supplies the resonator and
    /// frequency bindings; a caller-supplied one is refused, never merged.
    pub basis: CoupledInput,
    pub field: SceneField,
    pub geometry: SceneGeometry,
    pub material: SceneMaterial,
    /// One subject's Nara constitution (K10), when this owner is that person's
    /// reception of the event. Its receiver inputs arrive separately.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reception: Option<PersonalConstitution>,
}

/// One voice's source-read surface term.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Voice {
    pub planet_ref: &'static str,
    /// The planet's ecliptic longitude: its place on the torus's large circle.
    pub longitude_radians: f64,
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
        return Err(format!("scene {label} must be finite and positive"));
    }
    Ok(())
}

impl SceneConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != CONFIG {
            return Err("unsupported scene expression configuration".into());
        }
        if self.instance_ref.is_empty()
            || self.instance_ref.len() > 2048
            || self.instance_ref.chars().any(char::is_control)
        {
            return Err("invalid scene instance reference".into());
        }
        if self.basis.m2.resonator.is_some()
            || !self.basis.frequency_bindings.is_empty()
            || !self.basis.condition_frequency_bindings.is_empty()
            || !self.basis.sky_frequency_bindings.is_empty()
        {
            return Err(
                "the scene provider owns the voices; a supplied resonator or binding is refused"
                    .into(),
            );
        }
        let g = &self.geometry;
        let count = usize::from(g.longitude_samples) * usize::from(g.latitude_samples);
        if g.longitude_samples < 4 || g.latitude_samples < 4 || count > MAX_SAMPLES {
            return Err(format!(
                "scene sampling must be at least 4×4 and at most {MAX_SAMPLES} samples"
            ));
        }
        finite_positive(g.metres_per_unit, "metres per unit")?;
        if g.metres_per_unit > 1000.0 || g.attachment > 2 {
            return Err("scene scale or clock attachment outside the supported range".into());
        }
        let m = &self.material;
        if !m.damping_per_second.is_finite()
            || m.damping_per_second < 0.0
            || m.damping_per_second > 1e6
        {
            return Err("scene damping must be finite and non-negative".into());
        }
        finite_positive(m.strike_metres, "strike amplitude")?;
        if let Some(constitution) = &self.reception
            && (constitution.subject_id != self.basis.m3.subject_ref
                || constitution.subject_id != self.field.subject_ref)
        {
            return Err("Nara constitution, M3 and field must name one subject".into());
        }
        if m.strike_metres > 1.0
            || !m.audio_gain_per_metre.is_finite()
            || m.audio_gain_per_metre.abs() > 1e6
        {
            return Err("scene strike or gain outside the supported range".into());
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

/// M2′ Chladni term on the torus, L = 2π (M2-ARCHITECTURE §5.3.1), anchored at
/// the voice's place on the large circle (θ = λ).
fn chi(voice: &Voice, phi: f64, theta: f64) -> f64 {
    let phi = phi - voice.longitude_radians;
    let (m, n) = (f64::from(voice.m), f64::from(voice.n));
    let a = voice.weight * voice.phase_radians.cos();
    let b = voice.weight * voice.phase_radians.sin();
    a * (m * phi / 2.0).sin() * (n * theta / 2.0).sin()
        + b * (m * phi / 2.0).cos() * (n * theta / 2.0).cos()
}

fn angles(geometry: &SceneGeometry) -> impl Iterator<Item = (u64, f64, f64)> + '_ {
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
    /// Reads the voices from a composed event: pitch and place from its sky bus,
    /// surface mode numbers from its own M2 Vimarśā reading.
    pub fn from_basis(basis: &CoupledBasis) -> Result<Self, String> {
        let reading = &basis.m2["vimarsha"]["reading"];
        let quartet = reading["nodal_quartet"]
            .as_array()
            .filter(|v| v.len() == 4)
            .ok_or("composed event lacks the Vimarśā nodal quartet")?;
        let address72 = basis.derivation["mef_table_index"]
            .as_u64()
            .filter(|v| *v < 72)
            .ok_or("composed event lacks its 72-address")? as u8;
        let sky = basis.derivation["sky_voices"]
            .as_array()
            .filter(|v| v.len() == VOICES)
            .ok_or("composed event lacks the sky's voices")?;
        let hz: Vec<f64> = sky
            .iter()
            .map(|v| {
                v["frequency_hz"]
                    .as_f64()
                    .filter(|f| f.is_finite() && *f > 0.0)
            })
            .collect::<Option<_>>()
            .ok_or("non-finite sky frequency")?;
        let max = hz.iter().cloned().fold(0.0, f64::max);
        let mut voices = Vec::with_capacity(VOICES);
        let mut shape_ref = format!("{PROVIDER}:a{address72}");
        for (i, (planet, voice)) in PLANETS.iter().zip(sky).enumerate() {
            if voice["planet_ref"] != *planet {
                return Err("sky voices out of scalar-degree order".into());
            }
            let longitude = voice["longitude_degrees"]
                .as_f64()
                .filter(|l| l.is_finite())
                .ok_or("sky voice lacks its longitude")?;
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
            voices.push(Voice {
                planet_ref: planet,
                longitude_radians: longitude.to_radians(),
                frequency_hz: hz[i],
                m,
                n,
                helix,
                ql_position: node["ql_position"].as_u64().unwrap_or(0) as u8,
                weight: hz[i] / max,
                phase_radians: f64::from(u16::from(address72) + i as u16 + 1) * PI / 36.0,
            });
            // Whole degrees: a planet moving within its degree keeps its shape.
            shape_ref.push_str(&format!(":{m}x{n}@{}", longitude.floor() as i64));
        }
        Ok(Self {
            shape_ref,
            address72,
            voices: voices.try_into().map_err(|_| "voice count")?,
        })
    }

    /// Nodal lines depend on mode numbers and 72-address only; frequency moves
    /// the weights, which is part of the same re-read.
    pub fn shapes(&self, geometry: &SceneGeometry) -> Result<Vec<Vec<[f64; 3]>>, String> {
        angles(geometry)
            .map(|(_, phi, theta)| {
                let (_, normal) = surface(theta, phi)?;
                Ok(self
                    .voices
                    .iter()
                    .map(|voice| {
                        let x = chi(voice, phi, theta);
                        // Dimensionless coefficients travel at 1e-6, far below
                        // what the float32 targets resolve; it halves transfer.
                        [normal[0] * x, normal[1] * x, normal[2] * x]
                            .map(|v| (v * 1e6).round() / 1e6)
                    })
                    .collect())
            })
            .collect()
    }

    pub fn samples(&self, geometry: &SceneGeometry) -> Result<Vec<FieldSample>, String> {
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

fn resonator(
    input: &CoupledInput,
    material: &SceneMaterial,
    fibre: MaterialFibre,
) -> ResonatorState {
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
                mode_ref: format!("scene:planet/{}", PLANETS[i]),
                source_coordinate: PLANETS[i].into(),
                material_fibre: fibre,
                // The condition fixes the element; no carrier within its fibre is
                // preferred by source, so the weight is spread evenly over all 18.
                carrier_weights: (0..18)
                    .map(|c| CarrierWeight {
                        carrier: first + c,
                        weight: 1.0 / 18.0,
                    })
                    .collect(),
                // Bound to the sky by the coupled composer; never sounded as 1 Hz.
                frequency_hz: 1.0,
                amplitude: [material.strike_metres, 0.0],
                excitation: [0.0, 0.0],
                damping_per_second: material.damping_per_second,
                nodal_state_ref: format!("scene:planet/{}/nodal-quartet/{}", PLANETS[i], i % 4),
                antinodal_state_ref: format!("scene:planet/{}/sky", PLANETS[i]),
            })
            .collect(),
    }
}

/// Completes a caller's event with the provider's voices. The element comes from
/// the event's own M2 condition path; an event without one is refused. `hint` is
/// the element last seen (a tick never changes the condition), so one compose
/// usually suffices.
pub fn complete(
    event: &CoupledInput,
    material: &SceneMaterial,
    hint: MaterialFibre,
) -> Result<(CoupledInput, CoupledBasis), String> {
    let mut input = event.clone();
    input.sky_frequency_bindings = PLANETS
        .iter()
        .map(|planet| SkyFrequencyBinding {
            mode_ref: format!("scene:planet/{planet}"),
            planet_ref: (*planet).into(),
        })
        .collect();
    input.m2.resonator = Some(resonator(&input, material, hint));
    let first = input.compose()?;
    let fibre: MaterialFibre = serde_json::from_value(
        first.m2["condition"]["source_path"]["material_fibre"].clone(),
    )
    .map_err(
        |_| "scene voices need the active M2 condition's source element; this event admits none",
    )?;
    if fibre == hint {
        return Ok((input, first));
    }
    input.m2.resonator = Some(resonator(&input, material, fibre));
    let basis = input.compose()?;
    Ok((input, basis))
}

pub const BINDING_REQUEST: &str = "ql.scene-binding-request/v1";
pub const BINDING: &str = "oi.native-expression-binding/v1";
const DEFAULT_EVENT: &str = include_str!("../../../../fixtures/kernel/scene-default-event-v2.json");

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
    pub field: Option<SceneField>,
    #[serde(default)]
    pub geometry: Option<SceneGeometry>,
    #[serde(default)]
    pub material: Option<SceneMaterial>,
    /// Optional Nara constitution for a personal reception of this event.
    #[serde(default)]
    pub reception: Option<PersonalConstitution>,
}

/// 64×64 resolves the highest nodal order (m, n ≤ 12: at most 12 crossings a
/// turn) with every per-block transfer half that of PPS's 128×64 render mesh.
pub fn default_geometry() -> SceneGeometry {
    SceneGeometry {
        longitude_samples: 64,
        latitude_samples: 64,
        metres_per_unit: 1.0,
        attachment: 1,
    }
}

pub fn default_material() -> SceneMaterial {
    SceneMaterial {
        damping_per_second: 0.35,
        strike_metres: 0.08,
        audio_gain_per_metre: 1.0,
        strike_on_event: true,
    }
}

/// Authored fine-motion driver: one degree per second with a 9:8 phase relation.
/// This transition/display rate is distinct from the admitted M1 action's
/// source-defined thirty-degree tick; world() initializes the source inscription.
pub fn default_field(subject_ref: &str) -> SceneField {
    SceneField {
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
/// K8 sky provider's own `attach_m2` does. Occurrence retains the observation
/// epoch; acquisition/receipt remains a distinct time and exact source receipt.
pub(crate) fn attach_sky(event: &mut CoupledInput, sky: &Value) -> Result<(), String> {
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
    let occurrence = sky["epoch_unix_ms"]
        .as_u64()
        .filter(|v| *v <= 9_007_199_254_740_991)
        .ok_or("sky epoch exceeds the native unsigned occurrence range")?;
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
    event.m3.occurrence_unix_ms = occurrence;
    event.m3.receipt_unix_ms = received;
    // Retain the actual receipt that supplied these observations. Keeping an
    // unrelated embedded sky receipt makes two dated fields appear joined.
    event
        .source_receipts
        .retain(|receipt| receipt["schema"] != "ql.sky-snapshot/v1");
    event.source_receipts.push(sky.clone());
    Ok(())
}

/// Current geometric successor T² plus its distinct source clocks. The
/// continuous lift is display/transition state; aperture index changes only
/// through an admitted M3 operation. None is a renderer-local clock.
pub fn native_readback(basis: &CoupledBasis, field_clock: &Value, instance_ref: &str) -> Value {
    let mut form_process =
        crate::scene::current_form(basis, instance_ref).unwrap_or_else(|error| {
            json!({"process_subject_ref":format!("ql:scene-form:{instance_ref}"),
            "canonical_subject_ref":basis.m3["form"]["codon"]["ref"],
            "current_reading":{"ref":basis.m3["form"]["codon"]["ref"],"availability":"unavailable"},
            "source_error":error})
        });
    form_process["instance_ref"] = json!(instance_ref);
    // A native worker's clock also discloses source loci and double-cover
    // readings. Keep those verbatim separately; the continuation input is the
    // exact accepted ClockInput shape, containing no derived renderer fields.
    let input_clock = json!({
        "inscription":{"turns":field_clock["inscription"]["turns"],"half_degrees":field_clock["inscription"]["half_degrees"]},
        "lensing":{"turns":field_clock["lensing"]["turns"],"half_degrees":field_clock["lensing"]["half_degrees"]},
        "grid_origins":field_clock["grid_origins"],"rate_numerators":field_clock["rate_numerators"],
        "rate_denominator":field_clock["rate_denominator"],"rate_remainders":field_clock["rate_remainders"],
        "generation":field_clock["generation"]});
    let continuation_start = match basis.input.harmonic_source {
        super::coupled::HarmonicSource::CanonicalBasis { index } => json!({
            "tick12":basis.input.m1.tick12,"cycle":basis.input.m1.cycle.parse::<u64>().ok(),
            "aperture":basis.m3["aperture"]["index"],"harmonic_basis_index":index,
            "maqam_index":basis.input.m2.condition.as_ref().map(|c|c.maqam_index),
            "lens12":basis.input.m1.lens12,"context_frame":basis.input.m1.context_frame,
            "m3_address":basis.m3["form"]["address"],"m3_pose":basis.m3["form"]["pose"],
            "m3_clock_steps":basis.m3["clock"]["steps"],"matrix_axis":basis.m3["form"]["matrix_axis"],
            "rna":basis.m3["transcription"]["rna"],"continuous_clock":input_clock}),
        super::coupled::HarmonicSource::SelectedSourceRow => json!({"availability":"unavailable",
            "reason":"opening world recipe does not represent a selected-source-row harmonic; retain the complete native event instead"}),
    };
    json!({"schema":"ql.scene-source-reading/v1",
        "event_ref":basis.input.m1.event_ref, "subject_ref":basis.input.m3.subject_ref,
        "profile_generation":basis.input.m2.stamp.identity.profile_generation,
        "m1_revision":basis.m1["config"]["revision"], "m3_generation":basis.m3["identity"]["profile_generation"],
        "m1_clock":basis.m1["clock"], "m1_carrier":basis.m1["carrier"],
        "m3_clock":basis.m3["clock"], "form":basis.m3["form"],
        "selected_aperture":basis.m3["aperture"], "continuous_clock":input_clock,
        "continuous_clock_native":field_clock,
        "form_process":form_process,"continuation_start":continuation_start,
        "clock_semantics":{"source":"docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md#11-m3-5--the-clock-as-totalised-symbolic-cosmic-form",
            "geometric_successor":"inscription-circle × lens-circle",
            "selected_aperture":"action-only; continuous lensing is transition/display",
            "m1_advance":"native M1 ring and M3 inscription advance30 degrees/tick; aperture and sky are invariant"}})
}

/// A complete `oi.native-expression-binding/v1`: the scene host configuration and
/// a declared presentation. Particle `p` follows sample `p mod N` in both
/// targets — a stated correspondence, not a resampling of the body.
pub fn binding(request: BindingRequest) -> Result<Value, String> {
    if request.schema != BINDING_REQUEST {
        return Err("unsupported scene binding request".into());
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
    let config = SceneConfig {
        schema: CONFIG.into(),
        instance_ref: request.instance_ref,
        basis: event,
        field: request.field.unwrap_or_else(|| default_field(&subject)),
        geometry: request.geometry.unwrap_or_else(default_geometry),
        material: request.material.unwrap_or_else(default_material),
        reception: request.reception,
    };
    config.validate()?;
    // Compose once here so an unusable event is refused before any owner opens.
    let (_, basis) = complete(&config.basis, &config.material, MaterialFibre::Earth)?;
    let sky = config
        .basis
        .source_receipts
        .iter()
        .find(|r| r["schema"] == "ql.sky-snapshot/v1");
    let scene = sky
        .map(|snapshot| crate::scene::from_basis(snapshot, &basis))
        .transpose()?;
    let reading = native_readback(&basis, &json!(config.field.clock), &config.instance_ref);
    let samples =
        u64::from(config.geometry.longitude_samples) * u64::from(config.geometry.latitude_samples);
    let slots: Vec<u64> = (0..particles).map(|p| p % samples).collect();
    Ok(json!({
        "schema": BINDING,
        "host": config,
        "scene": scene,
        "native_readback": reading,
        "native_basis": basis,
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

/// One live scene instrument over one native coupled owner.
pub struct SceneInstrument {
    instance_ref: String,
    geometry: SceneGeometry,
    material: SceneMaterial,
    session: CoupledFieldSession,
    shape: ShapeBasis,
    event: CoupledInput,
    personal: Option<PersonalFieldInstance>,
}

impl SceneInstrument {
    pub fn open(worker: &Path, config: SceneConfig, timeout: Duration) -> Result<Self, String> {
        config.validate()?;
        let (input, basis) = complete(&config.basis, &config.material, MaterialFibre::Earth)?;
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
        let personal = config
            .reception
            .map(PersonalFieldInstance::new)
            .transpose()?;
        let session = CoupledFieldSession::open(worker, input.clone(), field, timeout)?;
        Ok(Self {
            instance_ref: config.instance_ref,
            geometry: config.geometry,
            material: config.material,
            session,
            shape,
            event: input,
            personal,
        })
    }

    /// The element the voices currently sound in.
    fn fibre(&self) -> MaterialFibre {
        self.event
            .m2
            .resonator
            .as_ref()
            .and_then(|r| r.modes.first())
            .map_or(MaterialFibre::Earth, |m| m.material_fibre)
    }

    /// The exact event basis a personal reception must cite.
    pub fn event_basis(&self) -> Result<EventBasisRefs, String> {
        EventBasisRefs::from_basis(self.session.current_basis())
    }

    /// Receives supplied seven-centre inputs against the current event without
    /// advancing or replacing the material field. Stale/cross-event is refused.
    pub fn receive_personal(
        &mut self,
        input: PersonalEventInput,
    ) -> Result<PersonalFieldState, String> {
        let personal = self
            .personal
            .as_mut()
            .ok_or("this scene owner carries no Nara constitution")?;
        personal.receive(self.session.current_basis(), input)
    }

    /// The last reception and whether it belongs to the exact current event;
    /// a determinant event never relabels an older reception as current.
    pub fn personal_reading(&self) -> Result<Value, String> {
        let personal = self
            .personal
            .as_ref()
            .ok_or("this scene owner carries no Nara constitution")?;
        let event = self.event_basis()?;
        let current = personal
            .current()
            .is_some_and(|state| state.event == event && state.subject_id == event.subject_ref);
        Ok(
            json!({"schema":"ql.scene-nara-reception/v1", "event": event,
            "state": personal.current(), "current": current}),
        )
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
        event.sky_frequency_bindings.clear();
        event
    }

    /// Recompose from a changed event: frequencies continue (or are struck), and
    /// a re-read nodal quartet/72-address reshapes the same voices explicitly.
    pub fn replace(&mut self, event: &CoupledInput, strike: bool) -> Result<Value, String> {
        let event = next_generation(event, self.event.m2.stamp.identity.profile_generation)?;
        let (input, basis) = complete(&event, &self.material, self.fibre())?;
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
        // The form advances from M1's live ring state, never from the bare tick
        // (spanda_field::ring_codon_advance; M3 SpandaAdvance).
        let cycle: u64 = event.m1.cycle.parse().map_err(|_| "invalid M1 cycle")?;
        let tick = u8::try_from(event.m1.tick12 % 12).map_err(|_| "invalid M1 tick")?;
        event.m3.address = crate::spanda_field::ring_codon_advance(tick, cycle).address();
        // D5: no separate stale inscription trajectory. M3's existing clock
        // owner preserves the unwrapped double cover while advancing30° per
        // admitted M1 tick. Aperture, observer/sky and harmonic selection stay.
        let degrees = ticks.checked_mul(30).ok_or("M1 clock advance overflow")?;
        event.m3.clock_steps = ql_core::m3_clock::M3Clock::at_steps(event.m3.clock_steps)
            .advance(degrees)
            .filter(|clock| clock.steps() <= crate::m2_engine::MAX_EXACT_JSON_INTEGER)
            .ok_or("M3 inscription advance exceeds exact native range")?
            .steps();
        self.replace(&event, self.material.strike_on_event)?;
        // The receiving continuous inscription follows the actual native M3
        // clock at this admitted determinant. Fine motion between events is
        // still the separately declared time driver; lens phase is invariant.
        self.session.set_axis_field(
            0,
            LiftInput {
                turns: (event.m3.clock_steps / 360).to_string(),
                half_degrees: ((event.m3.clock_steps % 360) * 2) as u16,
            },
        )
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
            "m3_generation": basis.m3["identity"]["profile_generation"],
            "shape_ref": self.shape.shape_ref,
            "address72": self.shape.address72,
            "voices": self.shape.voices,
            "geometry": self.geometry,
            "material": self.material,
            "material_standing": MATERIAL_STANDING,
            "native_readback": native_readback(basis, &field["clock"], &self.instance_ref),
            "effects": [
                {"determinant":"the dated sky (ql-sky snapshot → M2 world observations)",
                 "through":"coupled sky bus: M2-5 m_2_5_interval_from_root × M1 root",
                 "effect":"voice i = planet PLANETS[i]: PCM pitch and modal oscillation of the torus surface",
                 "units":"Hz", "range":"root × 1..9/4", "timing":"per determinant event; continuous integration between",
                 "consumer":"K8 C++ modal owner → native PCM and GPU target texture",
                 "warrant":"source-defined (map just octave; owner ruling #254 D13)"},
                {"determinant":"planet longitude λ",
                 "through":"θ = λ on the torus's large circle",
                 "effect":"where voice i's surface term is anchored on the body",
                 "units":"radians", "range":"0..2π", "timing":"re-read per event via explicit K8 shape replacement",
                 "consumer":"K8 target samples → GPU spring targets",
                 "warrant":"owner ruling #254 D6 (the torus is the clock)"},
                {"determinant":"m1 tick12 / lens12 / context frame / m3 pose",
                 "through":"m2.vimarsha.reading.nodal_quartet[i % 4].{m,n} and derivation.mef_table_index",
                 "effect":"nodal lines and phase of voice i's surface term (the M2 cymatic skin)",
                 "units":"mode numbers; radians", "range":"1..12; (address72 + i + 1)·π/36", "timing":"per determinant event",
                 "consumer":"K8 target samples",
                 "warrant":"source-defined (M2-ARCHITECTURE §5.3.1)"},
                {"determinant":"m1 ring state (tick12, cycle)",
                 "through":"spanda_field::ring_codon_advance → m3.address",
                 "effect":"the M3 form the event carries",
                 "units":"codon address", "range":"0..63", "timing":"per M1 advance",
                 "consumer":"M3 owner → Vimarśā pose → surface skin",
                 "warrant":"source-defined (C kernel spanda_codon_advance, parity-tested)"},
                {"determinant":"m1 torus #1-5-1",
                 "through":"m1::torus(θ, φ), R = 16/9, r = 1",
                 "effect":"rest body of every sample",
                 "units":"torus units × metres_per_unit", "range":"|x|,|y| ≤ 25/9, |z| ≤ 1", "timing":"fixed per instance",
                 "consumer":"K8 rest samples → GPU targets",
                 "warrant":"source-defined (KERNEL-M1-ENGINE-CONTRACT)"},
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
            planet_ref: "#2-5-0/1",
            longitude_radians: 0.0,
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
        let geometry = SceneGeometry {
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
        let geometry = SceneGeometry {
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

#[cfg(test)]
mod scene_tests {
    use super::*;

    #[test]
    fn a_voice_is_anchored_at_its_planets_longitude() {
        let base = Voice {
            planet_ref: "#2-5-4",
            longitude_radians: 0.0,
            frequency_hz: 220.0,
            m: 3,
            n: 2,
            helix: "bimba",
            ql_position: 0,
            weight: 1.0,
            phase_radians: 0.4,
        };
        let moved = Voice {
            longitude_radians: 1.0,
            ..base
        };
        // The same term, carried round the large circle by the longitude.
        assert!((chi(&base, 0.3, 1.2) - chi(&moved, 1.3, 1.2)).abs() < 1e-15);
    }

    #[test]
    fn a_historical_sky_replaces_the_receipt_and_preserves_epoch_vs_acquisition() {
        let sky: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v2.json"
        ))
        .unwrap();
        let mut event: CoupledInput = serde_json::from_str(DEFAULT_EVENT).unwrap();
        let old = event
            .source_receipts
            .iter()
            .find(|r| r["schema"] == "ql.sky-snapshot/v1")
            .unwrap()
            .clone();
        assert_ne!(old["snapshot_ref"], sky["snapshot_ref"]);
        assert_ne!(sky["epoch_unix_ms"], sky["receipt_unix_ms"]);
        let preserved =
            json!({"schema":"controlled:other-source/v1","ref":"test:qualified-other-source"});
        event.source_receipts.push(preserved.clone());
        attach_sky(&mut event, &sky).unwrap();
        let receipts: Vec<_> = event
            .source_receipts
            .iter()
            .filter(|r| r["schema"] == "ql.sky-snapshot/v1")
            .collect();
        assert_eq!(receipts, vec![&sky]);
        assert!(event.source_receipts.contains(&preserved));
        assert_eq!(
            event.m3.occurrence_unix_ms,
            sky["epoch_unix_ms"].as_u64().unwrap()
        );
        assert_eq!(
            event.m3.receipt_unix_ms,
            sky["receipt_unix_ms"].as_u64().unwrap()
        );
        assert!(
            event
                .m2
                .world_observations
                .iter()
                .all(|o| o.provider_ref == sky["snapshot_ref"].as_str().unwrap())
        );
    }

    #[test]
    fn historical_embedded_event_is_refused_against_the_current_source_sky() {
        let sky: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/kernel/sky-snapshot-world-2026-09-28-v2.json"
        ))
        .unwrap();
        let result = binding(BindingRequest {
            schema: BINDING_REQUEST.into(),
            instance_ref: "test:scene".into(),
            texture: [8, 8],
            units_per_metre: 1.0,
            // The preserved original stale fixture remains a real negative;
            // the current embedded successor is no longer the stale subject.
            event: Some(
                serde_json::from_str(include_str!(
                    "../../../../fixtures/kernel/scene-default-event-v1.json"
                ))
                .unwrap(),
            ),
            sky: Some(sky),
            field: None,
            geometry: None,
            material: None,
            reception: None,
        });
        assert!(result.unwrap_err().contains("registry revision differ"));
    }
    #[test]
    fn default_without_new_acquisition_still_contains_the_exact_dated_scene() {
        let event: CoupledInput = serde_json::from_str(DEFAULT_EVENT).unwrap();
        let sky = event
            .source_receipts
            .iter()
            .find(|r| r["schema"] == "ql.sky-snapshot/v1")
            .unwrap();
        assert_eq!(sky["source_binding"]["sun_role"], "solar-parent");
        assert_eq!(sky["epoch_utc"], "2026-09-15T13:46:21Z");
        assert_eq!(event.m1.tick12, 7);
        assert_eq!(event.m1.cycle, "1");
        assert_eq!(event.m3.address, 7);
        assert_eq!(event.m3.pose, 6);
        assert_eq!(event.m3.aperture, 2);
        assert_eq!(event.m3.clock_steps, 359);
        let reading = binding(BindingRequest {
            schema: BINDING_REQUEST.into(),
            instance_ref: "test:qualified-default".into(),
            texture: [8, 8],
            units_per_metre: 120.0,
            event: None,
            sky: None,
            field: None,
            geometry: None,
            material: None,
            reception: None,
        })
        .unwrap();
        assert_eq!(reading["scene"]["bodies"].as_array().unwrap().len(), 10);
        assert_eq!(
            reading["native_basis"]["derivation"]["sky_voices"]
                .as_array()
                .unwrap()
                .len(),
            9
        );
        assert_eq!(reading["scene"]["snapshot_ref"], sky["snapshot_ref"]);
        assert_eq!(reading["host"]["basis"], json!(event));
    }
}
