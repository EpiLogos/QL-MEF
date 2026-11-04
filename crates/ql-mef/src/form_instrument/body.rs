//! A form made a body (O-I #852; QL-MEF #288 packet P; DLW manual §21.2 "The
//! body"). A fold, a codon or a hexagram of M3′, or a glyph someone draws, is
//! compiled through the form owners' own types into the body the native field
//! owner (K8) rings:
//!
//! - its points are the form's samples (`form_samples`), by their stable IDs,
//!   at the places the fold's creases put them: site 0 folds the half beyond
//!   the X hinge about it, site 1 the half beyond the Y hinge, and site 2 turns
//!   the whole body in its plane (a pose: it moves the picture, not the sound);
//! - its size is the sheet's declared metric scale;
//! - it is held still where the three hinges cross, the one place no crease
//!   moves (each separate piece of a glyph at its own point nearest it);
//! - it is struck where it is touched and heard at its free tip, the point
//!   farthest from the hold, unless the request names where it is heard;
//! - its modes are read from its own creased lattice: neighbouring points are
//!   joined, and the coupling across a crease is the cosine of the angle
//!   between the two faces, so a fold changes how the halves ring together;
//! - what it is made of is the material's Q factor (the Torus's Damping): each
//!   mode rings out at π·f/Q, a high Q ringing longer, a high partial sooner.
//!
//! The tuning stays the Torus's: every voice stands on the body's first mode
//! and rings the body's partials above it at the body's own ratios. The body
//! sets the partials, their shapes on the body and their ring-down.
//!
//! The form keeps its meaning as a symbol. Its symbolic role ([`FormSymbol`])
//! travels beside its physical role, untouched by it.
//!
//! The preparation is versioned against its exact request: a body prepared for
//! another form, fold or material is refused as stale, never rung.
//!
//! The reading is the `ql.physical-body-preparation/v1` a Scene's performance
//! keeps for its body (O-I's PerformanceBasis `prepared_body`).
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::continuous::scene_field::SceneField;
use crate::continuous::{
    FieldInput, FieldSample, FieldUnits, STRIKE_CAP_METRES, StrikeAtInput, StrikeInput,
};
use crate::form_samples::{SampleBody, SamplePreparation, prepare_samples};
use crate::m2_engine::{
    CarrierWeight, ContinuousMode, InputStamp, MaterialFibre, PhysicalParameter, ResonatorState,
};

pub const BODY_REQUEST: &str = "ql.form-body-request/v1";
pub const BODY_PREPARATION: &str = "ql.physical-body-preparation/v1";
pub const BODY_CONTRACT: &str = "ql.physical-body/v1";
/// The body model: the creased lattice and its modes.
pub const BODY_MODEL: &str = "ql.form-body-creased-lattice/v1";
pub const BODY_PROVIDER: &str = "ql.form-body-provider/v1";
/// The most samples one body is prepared from: its modes are solved whole.
pub const MAX_BODY_SAMPLES: usize = 256;
/// The most modes of the body each voice rings.
pub const MAX_BODY_MODES: u16 = 64;
/// The most modes one body hands the native owner (the Torus's Modes bound).
pub const MAX_RESONATOR_MODES: usize = 4096;
/// The Torus's Damping range, as a Q factor (DLW manual §21.2 Parameters).
pub const Q_FACTOR_RANGE: (f64, f64) = (0.1, 40.0);
/// The constituent every body point stands for: M3's form position (M3-3), the
/// "which" a fold, a codon, a hexagram or a glyph is.
pub const FORM_CONSTITUENT: &str = "#3-3";
/// K8 names a point by an exact JSON integer: at most 2^53 − 1.
pub const IDENTITY_MASK: u64 = (1 << 53) - 1;
pub const BODY_STANDING: &str = "proposed: the creased lattice is the body model this owner declares; no source table fixes a form's stiffness";

pub const EXCITER_LAW: &str = "a struck point gives each voice's body mode amplitude × the voice's weight × the mode's shape at the sample nearest the point, summed over the act";
const DAMPING_LAW: &str = "each mode rings out at π·f/Q";

/// What the form is as a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FormKind {
    Fold,
    Codon,
    Hexagram,
    Glyph,
}

/// The form's symbolic role, kept apart from its physical one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormSymbol {
    pub kind: FormKind,
    pub form_ref: String,
    /// The reading the form was taken from (a fold progress, a codon, a drawing).
    pub source_ref: String,
}

/// What the body is made of.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyMaterial {
    pub fibre: MaterialFibre,
    /// The Torus's Damping: a high value rings longer.
    pub q_factor: f64,
    /// The Torus's Strike: how strongly velocity excites the body, 0..1.
    pub strike_share: f64,
    /// Where the constants came from: "declared", or the tattva read.
    pub source_ref: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyRequest {
    pub schema: String,
    pub event_ref: String,
    pub subject_ref: String,
    /// The M3 generation the form was read under.
    pub source_generation: u64,
    pub form: FormSymbol,
    /// The form's sheet: a sheet carrier for a fold, a glyph mask for a drawn
    /// glyph, a codon or a hexagram.
    pub samples: SamplePreparation,
    /// The fold's commanded crease angles, tenths of a degree, by M3 site
    /// (a fold progress's `site_angles_deg10`).
    pub crease_deg10: [i32; 3],
    pub material: BodyMaterial,
    /// How many of the body's modes each voice rings.
    pub modes: u16,
    pub sample_rate: u32,
    /// Where the body is heard, as (u, v) of its sheet; absent, at its free tip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pickup_uv: Option<[f64; 2]>,
}

/// One point of the body: a sample of the form where the fold put it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BodyPoint {
    pub sample_id: u64,
    /// The name the native owner knows the point by: the sample ID's low 53
    /// bits (K8 takes an exact JSON integer), unique within the body.
    pub identity: u64,
    pub layer: u16,
    /// Its rest coordinate on the sheet.
    pub uv: [f64; 2],
    pub rest_metres: [f64; 3],
    /// The normal of its face after the creases.
    pub normal: [f64; 3],
    /// Its mass share: the form's coverage there.
    pub mass: f64,
    /// Held still: where the hinges cross.
    pub held: bool,
}

/// One mode of the body.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BodyMode {
    pub index: usize,
    pub eigenvalue: f64,
    /// Its frequency over the first mode's: each voice rings it at this ratio.
    pub ratio: f64,
    /// Its shape at each point, in point order: 0 where held, peak 1.
    pub shape: Vec<f64>,
}

/// One voice the body rings, as the tuning sounds it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodyVoice {
    pub mode_ref: String,
    pub source_coordinate: String,
    pub frequency_hz: f64,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct FormBody {
    request: BodyRequest,
    sheet: SampleBody,
    points: Vec<BodyPoint>,
    edges: usize,
    modes: Vec<BodyMode>,
    pickup: usize,
    preparation_ref: String,
    state_ref: String,
}

fn text(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        return Err(format!(
            "a body's {label} must be 1..=2048 characters with no control character"
        ));
    }
    Ok(())
}

fn sha256(value: &impl Serialize) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).expect("serialisable"))
    )
}

/// The preparation reference a request names: the digest of the exact request.
pub fn preparation_ref(request: &BodyRequest) -> String {
    format!("ql:form-body:{}", sha256(request))
}

impl BodyRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != BODY_REQUEST {
            return Err(format!("a form body is prepared from {BODY_REQUEST}"));
        }
        text(&self.event_ref, "event_ref")?;
        text(&self.subject_ref, "subject_ref")?;
        text(&self.form.form_ref, "form_ref")?;
        text(&self.form.source_ref, "form source_ref")?;
        text(&self.material.source_ref, "material source_ref")?;
        if !(1..=MAX_BODY_MODES).contains(&self.modes) {
            return Err(format!(
                "a body rings 1..={MAX_BODY_MODES} modes for each voice"
            ));
        }
        let (low, high) = Q_FACTOR_RANGE;
        let q = self.material.q_factor;
        if !q.is_finite() || q < low || q > high {
            return Err(format!(
                "a body's Q factor must be within {low} to {high}: it is the Torus's Damping"
            ));
        }
        let strike = self.material.strike_share;
        if !strike.is_finite() || !(0.0..=1.0).contains(&strike) {
            return Err("a body's Strike must be a share within 0 to 1".into());
        }
        if self.sample_rate == 0 {
            return Err("a body rings at a positive sample rate".into());
        }
        if self.crease_deg10.iter().any(|a| a.abs() > 1800) {
            return Err("a crease turns at most 180° either way".into());
        }
        if let Some(uv) = self.pickup_uv
            && uv.iter().any(|x| !x.is_finite() || !(0.0..1.0).contains(x))
        {
            return Err(
                "where a body is heard is a point (u, v) of its sheet, each in [0, 1)".into(),
            );
        }
        let count = self.samples.allocation_count();
        if count > MAX_BODY_SAMPLES {
            return Err(format!(
                "a body is prepared from at most {MAX_BODY_SAMPLES} samples; this form allocates {count}: prepare its body at a coarser resolution"
            ));
        }
        Ok(())
    }
}

fn rotate(point: &mut [f64; 3], (a, b): (usize, usize), angle: f64) {
    let (sin, cos) = angle.sin_cos();
    let (x, y) = (point[a], point[b]);
    point[a] = x * cos - y * sin;
    point[b] = x * sin + y * cos;
}

/// The creases applied to one rest point and its face's normal, in site order.
fn crease(rest: [f64; 3], deg10: [i32; 3]) -> ([f64; 3], [f64; 3]) {
    let angle = |site: usize| f64::from(deg10[site]) * PI / 1800.0;
    let (mut point, mut normal) = (rest, [0.0, 0.0, 1.0]);
    // Site 0 (X): the half beyond the X hinge folds about it, (y, z).
    if rest[1] > 0.0 {
        rotate(&mut point, (1, 2), angle(0));
        rotate(&mut normal, (1, 2), angle(0));
    }
    // Site 1 (Y): the half beyond the Y hinge folds about it, (z, x).
    if rest[0] > 0.0 {
        rotate(&mut point, (2, 0), angle(1));
        rotate(&mut normal, (2, 0), angle(1));
    }
    // Site 2 (Z): the whole body turns in its plane, (x, y).
    rotate(&mut point, (0, 1), angle(2));
    rotate(&mut normal, (0, 1), angle(2));
    (point, normal)
}

fn distance(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

/// The eigenpairs of a symmetric matrix (cyclic Jacobi), eigenvalues ascending;
/// each eigenvector is a column of the returned row-major matrix.
fn eigen(mut a: Vec<f64>, n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut v = vec![0.0; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    let scale: f64 = a.iter().map(|x| x * x).sum();
    for _ in 0..64 {
        let mut off = 0.0;
        for p in 0..n {
            for q in p + 1..n {
                off += a[p * n + q] * a[p * n + q];
            }
        }
        if off <= 1e-26 * scale {
            break;
        }
        for p in 0..n {
            for q in p + 1..n {
                let apq = a[p * n + q];
                if apq.abs() <= 1e-300 {
                    continue;
                }
                let theta = (a[q * n + q] - a[p * n + p]) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..n {
                    let (kp, kq) = (a[k * n + p], a[k * n + q]);
                    a[k * n + p] = c * kp - s * kq;
                    a[k * n + q] = s * kp + c * kq;
                }
                for k in 0..n {
                    let (pk, qk) = (a[p * n + k], a[q * n + k]);
                    a[p * n + k] = c * pk - s * qk;
                    a[q * n + k] = s * pk + c * qk;
                }
                for k in 0..n {
                    let (kp, kq) = (v[k * n + p], v[k * n + q]);
                    v[k * n + p] = c * kp - s * kq;
                    v[k * n + q] = s * kp + c * kq;
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| a[i * n + i].total_cmp(&a[j * n + j]).then(i.cmp(&j)));
    let values = order.iter().map(|&i| a[i * n + i]).collect();
    let mut vectors = vec![0.0; n * n];
    for (column, &i) in order.iter().enumerate() {
        for k in 0..n {
            vectors[k * n + column] = v[k * n + i];
        }
    }
    (values, vectors)
}

fn root(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

impl FormBody {
    /// Compiles the form into its body: points, hold, lattice, modes, pickup.
    pub fn prepare(request: BodyRequest) -> Result<Self, String> {
        request.validate()?;
        let sheet = prepare_samples(request.samples.clone())?;
        let resolution = usize::from(sheet.preparation.resolution);
        let mut slots = vec![None; sheet.samples.len()];
        let mut points = Vec::new();
        for (slot, sample) in sheet.samples.iter().enumerate() {
            if sample.coverage <= 0.0 {
                continue;
            }
            let (u, v) = sheet.rest_uv(sample);
            let (place, normal) = crease(sheet.rest_units(sample), request.crease_deg10);
            let scale = sheet.preparation.units.metres_per_unit;
            slots[slot] = Some(points.len());
            points.push(BodyPoint {
                sample_id: sample.sample_id,
                identity: sample.sample_id & IDENTITY_MASK,
                layer: sample.layer_index,
                uv: [u, v],
                rest_metres: place.map(|x| x * scale),
                normal,
                mass: sample.coverage,
                held: false,
            });
        }
        if points.is_empty() {
            return Err("this form leaves no sample to ring: its mask covers nothing".into());
        }
        let mut names = std::collections::BTreeSet::new();
        if let Some(twin) = points.iter().find(|p| !names.insert(p.identity)) {
            return Err(format!(
                "two samples of this form share the native name {}: the body cannot tell them apart",
                twin.identity
            ));
        }
        // The lattice: neighbours on the sheet, and through the layers.
        let per_layer = resolution * resolution;
        let mut edges = Vec::new();
        for (slot, sample) in sheet.samples.iter().enumerate() {
            let Some(i) = slots[slot] else { continue };
            let mut join = |other: usize| {
                if let Some(j) = slots.get(other).copied().flatten() {
                    edges.push((i, j));
                }
            };
            if usize::from(sample.u_index) + 1 < resolution {
                join(slot + 1);
            }
            if usize::from(sample.v_index) + 1 < resolution {
                join(slot + resolution);
            }
            join(slot + per_layer);
        }
        // The points stand in the order the native owner takes them: by name.
        let mut order: Vec<usize> = (0..points.len()).collect();
        order.sort_by_key(|&i| points[i].identity);
        let mut rank = vec![0; points.len()];
        for (r, &i) in order.iter().enumerate() {
            rank[i] = r;
        }
        let mut points: Vec<BodyPoint> = order.iter().map(|&i| points[i].clone()).collect();
        let edges: Vec<(usize, usize)> = edges.iter().map(|&(i, j)| (rank[i], rank[j])).collect();
        // Held where the hinges cross: in each separate piece, its points
        // nearest the sheet's centre.
        let mut parent: Vec<usize> = (0..points.len()).collect();
        for &(i, j) in &edges {
            let (a, b) = (root(&mut parent, i), root(&mut parent, j));
            if a != b {
                parent[a.max(b)] = a.min(b);
            }
        }
        let mut nearest: BTreeMap<usize, f64> = BTreeMap::new();
        for (i, point) in points.iter().enumerate() {
            let piece = root(&mut parent, i);
            let d = distance(point.uv, [0.5, 0.5]);
            let entry = nearest.entry(piece).or_insert(d);
            *entry = entry.min(d);
        }
        for (i, point) in points.iter_mut().enumerate() {
            let piece = root(&mut parent, i);
            if distance(point.uv, [0.5, 0.5]) <= nearest[&piece] + 1e-12 {
                point.held = true;
            }
        }
        let free: Vec<usize> = (0..points.len()).filter(|&i| !points[i].held).collect();
        if free.is_empty() {
            return Err(
                "this form's body is held at every sample: nothing of it is free to ring".into(),
            );
        }
        let mut dof = vec![None; points.len()];
        for (k, &i) in free.iter().enumerate() {
            dof[i] = Some(k);
        }
        // The stiffness of the creased lattice, over the points' masses.
        let n = free.len();
        let mut stiffness = vec![0.0; n * n];
        for &(i, j) in &edges {
            let ni = points[i].normal;
            let nj = points[j].normal;
            let coupling = ni[0] * nj[0] + ni[1] * nj[1] + ni[2] * nj[2];
            if let Some(a) = dof[i] {
                stiffness[a * n + a] += 1.0;
            }
            if let Some(b) = dof[j] {
                stiffness[b * n + b] += 1.0;
            }
            if let (Some(a), Some(b)) = (dof[i], dof[j]) {
                stiffness[a * n + b] -= coupling;
                stiffness[b * n + a] -= coupling;
            }
        }
        let inverse_root: Vec<f64> = free
            .iter()
            .map(|&i| points[i].mass.sqrt().recip())
            .collect();
        for a in 0..n {
            for b in 0..n {
                stiffness[a * n + b] *= inverse_root[a] * inverse_root[b];
            }
        }
        let (values, vectors) = eigen(stiffness, n);
        let count = usize::from(request.modes).min(n);
        let first = values[0];
        if !first.is_finite() || first <= 1e-12 {
            return Err(
                "this form's body has a free piece that is held nowhere: it cannot ring".into(),
            );
        }
        let modes = (0..count)
            .map(|k| {
                let mut shape = vec![0.0; points.len()];
                for (a, &i) in free.iter().enumerate() {
                    shape[i] = vectors[a * n + k] * inverse_root[a];
                }
                let peak = shape.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
                let at = shape
                    .iter()
                    .position(|x| x.abs() >= peak * (1.0 - 1e-12))
                    .unwrap_or(0);
                let sign = shape[at].signum() * peak;
                for x in &mut shape {
                    *x /= sign;
                }
                shape[at] = 1.0;
                BodyMode {
                    index: k,
                    eigenvalue: values[k],
                    ratio: (values[k] / first).sqrt(),
                    shape,
                }
            })
            .collect::<Vec<_>>();
        let pickup = match request.pickup_uv {
            Some(uv) => nearest_point(&points, uv),
            None => {
                let far = points
                    .iter()
                    .map(|p| distance(p.uv, [0.5, 0.5]))
                    .fold(0.0, f64::max);
                points
                    .iter()
                    .position(|p| distance(p.uv, [0.5, 0.5]) >= far - 1e-12)
                    .unwrap_or(0)
            }
        };
        let preparation_ref = preparation_ref(&request);
        let state_ref = format!(
            "ql:form-body-state:{}",
            sha256(
                &json!({"preparation_ref": preparation_ref, "model": BODY_MODEL,
                "points": points, "modes": modes, "pickup": pickup})
            )
        );
        Ok(Self {
            request,
            sheet,
            edges: edges.len(),
            points,
            modes,
            pickup,
            preparation_ref,
            state_ref,
        })
    }

    pub fn request(&self) -> &BodyRequest {
        &self.request
    }
    pub fn points(&self) -> &[BodyPoint] {
        &self.points
    }
    pub fn modes(&self) -> &[BodyMode] {
        &self.modes
    }
    /// The point where the body is heard.
    pub fn pickup(&self) -> usize {
        self.pickup
    }
    pub fn preparation_ref(&self) -> &str {
        &self.preparation_ref
    }
    pub fn state_ref(&self) -> &str {
        &self.state_ref
    }

    /// Refuses a request this body was not prepared for: a stale solve never rings.
    pub fn admit(&self, request: &BodyRequest) -> Result<(), String> {
        let wanted = preparation_ref(request);
        if wanted != self.preparation_ref {
            return Err(format!(
                "this body was prepared for another form, fold or material ({}); prepare it again for {wanted} before it rings",
                self.preparation_ref
            ));
        }
        Ok(())
    }

    /// The point a strike lands on: the one nearest (u, v) = (φ/2π, θ/2π),
    /// the same place of the sheet a strike-at names on the torus.
    pub fn sample_at(&self, point: &StrikeAtInput) -> usize {
        nearest_point(&self.points, [point.phi / TAU, point.theta / TAU])
    }

    fn mode_ref(voice: &BodyVoice, k: usize) -> String {
        format!("{}/body/{k}", voice.mode_ref)
    }

    /// The modes the native owner rings for `voices`, in its order: each voice
    /// through each body mode (`resonator`).
    pub fn mode_refs(&self, voices: &[BodyVoice]) -> Vec<String> {
        voices
            .iter()
            .flat_map(|voice| {
                self.modes
                    .iter()
                    .map(|mode| Self::mode_ref(voice, mode.index))
            })
            .collect()
    }

    fn voices_valid(&self, voices: &[BodyVoice]) -> Result<(), String> {
        if voices.is_empty() {
            return Err("a body rings the voices of a tuning: none were given".into());
        }
        if voices.len() * self.modes.len() > MAX_RESONATOR_MODES {
            return Err(format!(
                "{} voices of {} modes pass the {MAX_RESONATOR_MODES} modes one body may hand the native owner",
                voices.len(),
                self.modes.len()
            ));
        }
        for voice in voices {
            text(&voice.mode_ref, "voice mode_ref")?;
            if !voice.frequency_hz.is_finite() || voice.frequency_hz <= 0.0 {
                return Err(format!(
                    "voice {} has no positive frequency",
                    voice.mode_ref
                ));
            }
            if !voice.weight.is_finite() {
                return Err(format!("voice {} has no finite weight", voice.mode_ref));
            }
            let top = voice.frequency_hz * self.modes.last().map_or(1.0, |m| m.ratio);
            if top >= f64::from(self.request.sample_rate) / 2.0 {
                return Err(format!(
                    "voice {} rings this body's highest mode at {top:.1} Hz, past half the sample rate: ring fewer modes",
                    voice.mode_ref
                ));
            }
        }
        Ok(())
    }

    /// The modes the native owner rings: each voice through each body mode,
    /// at the voice's frequency times the mode's ratio, ringing out at π·f/Q,
    /// at rest until struck.
    pub fn resonator(
        &self,
        stamp: InputStamp,
        voices: &[BodyVoice],
    ) -> Result<ResonatorState, String> {
        self.voices_valid(voices)?;
        let material = &self.request.material;
        let first = (material.fibre.index() * 18) as u8;
        let carriers: Vec<CarrierWeight> = (0..18)
            .map(|c| CarrierWeight {
                carrier: first + c,
                weight: 1.0 / 18.0,
            })
            .collect();
        let modes = voices
            .iter()
            .flat_map(|voice| {
                self.modes.iter().map(|mode| {
                    let frequency_hz = voice.frequency_hz * mode.ratio;
                    ContinuousMode {
                        mode_ref: Self::mode_ref(voice, mode.index),
                        source_coordinate: voice.source_coordinate.clone(),
                        material_fibre: material.fibre,
                        carrier_weights: carriers.clone(),
                        frequency_hz,
                        amplitude: [0.0, 0.0],
                        excitation: [0.0, 0.0],
                        damping_per_second: PI * frequency_hz / material.q_factor,
                        nodal_state_ref: format!("{}/held", self.preparation_ref),
                        antinodal_state_ref: format!(
                            "{}/mode/{}",
                            self.preparation_ref, mode.index
                        ),
                    }
                })
            })
            .collect();
        let parameter = |value: f64, unit: &str| PhysicalParameter {
            value,
            unit: unit.into(),
            source_ref: material.source_ref.clone(),
        };
        Ok(ResonatorState {
            stamp,
            provider_ref: BODY_PROVIDER.into(),
            geometry_ref: self.preparation_ref.clone(),
            material_ref: format!("{BODY_PROVIDER}:declared-linear-medium"),
            material_model_ref: BODY_MODEL.into(),
            material_parameters: [
                ("q_factor".to_owned(), parameter(material.q_factor, "1")),
                (
                    "strike_share".to_owned(),
                    parameter(material.strike_share, "1"),
                ),
            ]
            .into(),
            modes,
        })
    }

    /// The body's points as the native owner takes them: each sample by its
    /// ID at its creased place, with every ringing mode's shape there — the
    /// face's normal times the voice's weight times the mode's shape.
    pub fn samples(&self, voices: &[BodyVoice], attachment: u8) -> Vec<FieldSample> {
        self.points
            .iter()
            .enumerate()
            .map(|(i, point)| FieldSample {
                identity: point.identity,
                constituent: FORM_CONSTITUENT.into(),
                attachment,
                rest_metres: point.rest_metres,
                mode_shapes: voices
                    .iter()
                    .flat_map(|voice| {
                        self.modes.iter().map(move |mode| {
                            let x = voice.weight * mode.shape[i];
                            point.normal.map(|n| (n * x * 1e6).round() / 1e6)
                        })
                    })
                    .collect(),
            })
            .collect()
    }

    /// What each ringing mode gives the ear: its shape where the body is heard.
    pub fn audio_gains(&self, voices: &[BodyVoice], gain_per_metre: f64) -> Vec<f64> {
        voices
            .iter()
            .flat_map(|_| {
                self.modes
                    .iter()
                    .map(move |mode| gain_per_metre * mode.shape[self.pickup])
            })
            .collect()
    }

    /// The whole input the native owner opens the body with, over the scene's
    /// own field: its subject, rate and clock.
    pub fn field_input(
        &self,
        field: &SceneField,
        voices: &[BodyVoice],
        attachment: u8,
        gain_per_metre: f64,
    ) -> Result<FieldInput, String> {
        self.voices_valid(voices)?;
        if field.sample_rate != self.request.sample_rate {
            return Err(format!(
                "this body was prepared at {} Hz; the field runs at {} Hz",
                self.request.sample_rate, field.sample_rate
            ));
        }
        if attachment > 2 {
            return Err("a body's attachment is 0 fixed, 1 inscription or 2 lensing".into());
        }
        Ok(FieldInput {
            subject_ref: field.subject_ref.clone(),
            sample_rate: field.sample_rate,
            clock: field.clock.clone(),
            driver_numerator: field.driver_numerator,
            driver_denominator: field.driver_denominator,
            units: FieldUnits {
                amplitude: "m".into(),
                excitation: "m/s".into(),
                shape: "dimensionless".into(),
                position: "m".into(),
                audio: "linear".into(),
            },
            audio_gains: self.audio_gains(voices, gain_per_metre),
            samples: self.samples(voices, attachment),
            shape_ref: Some(self.state_ref.clone()),
        })
    }

    /// A strike at points of the body, lowered to the named-mode strike the
    /// native owner admits. Refused whole, in words, if a point is outside the
    /// sheet or the impulses pass the material policy.
    pub fn strike_at(
        &self,
        points: &[StrikeAtInput],
        voices: &[BodyVoice],
    ) -> Result<Vec<StrikeInput>, String> {
        StrikeAtInput::validate_act(points)?;
        self.voices_valid(voices)?;
        let mut impulses: Vec<StrikeInput> = voices
            .iter()
            .flat_map(|voice| {
                self.modes.iter().map(move |mode| StrikeInput {
                    mode_ref: Self::mode_ref(voice, mode.index),
                    amplitude: [0.0, 0.0],
                })
            })
            .collect();
        for point in points {
            let at = self.sample_at(point);
            for (v, voice) in voices.iter().enumerate() {
                for (k, mode) in self.modes.iter().enumerate() {
                    let shape = voice.weight * mode.shape[at];
                    let impulse = &mut impulses[v * self.modes.len() + k];
                    impulse.amplitude[0] += point.amplitude[0] * shape;
                    impulse.amplitude[1] += point.amplitude[1] * shape;
                }
            }
        }
        if let Some(over) = impulses.iter().find(|impulse| {
            impulse
                .amplitude
                .iter()
                .any(|v| !v.is_finite() || v.abs() > STRIKE_CAP_METRES)
        }) {
            return Err(format!(
                "the points of this strike-at act sum past the declared {STRIKE_CAP_METRES:.1} metre material policy on {}",
                over.mode_ref
            ));
        }
        Ok(impulses)
    }

    /// The prepared body a Scene's performance keeps (`prepared_body`).
    pub fn reading(&self) -> Value {
        let request = &self.request;
        let pickup = &self.points[self.pickup];
        json!({
            "schema": BODY_PREPARATION,
            "contract": BODY_CONTRACT,
            "event_ref": request.event_ref,
            "subject_ref": request.subject_ref,
            "source_generation": request.source_generation,
            "request": {
                "preparation_ref": self.preparation_ref,
                "state_ref": self.state_ref,
                "body_revision": BODY_MODEL,
                "geometry": {
                    "form_samples": self.sheet.preparation_sha256,
                    "prep_ref": self.sheet.preparation.prep_ref,
                    "crease_deg10": request.crease_deg10,
                    "metres_per_unit": self.sheet.preparation.units.metres_per_unit,
                    "points": self.points.len(),
                    "edges": self.edges,
                    "held": self.points.iter().filter(|p| p.held).map(|p| p.sample_id).collect::<Vec<_>>(),
                },
                "material": {
                    "fibre": request.material.fibre,
                    "q_factor": request.material.q_factor,
                    "strike_share": request.material.strike_share,
                    "source_ref": request.material.source_ref,
                    "damping": DAMPING_LAW,
                },
                "exciter": {"kind": "strike-at", "law": EXCITER_LAW, "cap_metres": STRIKE_CAP_METRES},
                "pickup": {"sample_id": pickup.sample_id, "uv": pickup.uv, "named": request.pickup_uv.is_some()},
                "sample_rate": request.sample_rate,
            },
            "form": request.form,
            "clock": {"sample_rate": request.sample_rate, "source_generation": request.source_generation},
            "units": {"position": "m", "shape": "dimensionless", "ratio": "over the first mode", "damping": "1/s"},
            "modes": self.modes.iter().map(|m| json!({"index": m.index, "ratio": m.ratio, "eigenvalue": m.eigenvalue})).collect::<Vec<_>>(),
            "standing": BODY_STANDING,
        })
    }
}

fn nearest_point(points: &[BodyPoint], uv: [f64; 2]) -> usize {
    let mut best = (f64::INFINITY, 0);
    for (i, point) in points.iter().enumerate() {
        let d = distance(point.uv, uv);
        if d < best.0 - 1e-12 {
            best = (d, i);
        }
    }
    best.1
}
