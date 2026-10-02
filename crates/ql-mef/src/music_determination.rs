//! Source-bound musical determination over the existing M, music, MEF and CF owners.
//!
//! V3 II-3.2's inner-position octet, II-4.1's independent five-inner/two-outer
//! diatonic cut, and a seven-degree scale plus octave return are different
//! operations. None supplies a tuning policy implicitly. Exact rational
//! trajectories also remain separate from the finite pitch-class field.
//! M1 owns the oscillator; these targets are for the M2-1 Vimarśā writer.
use ql_core::{ConjugationDegree, ExpansionSide, QlCoordinate, RelationFamily};
use serde::{Deserialize, Deserializer, Serialize};

use crate::m2_engine::InputStamp;
use crate::{
    ContextFrameId, LensId, MCoordinate, MFace, ModeKind, MusicalBasis, MusicalCompletionFrame,
    PoleIdentity, explicate_coordinates, implicate_coordinates, lens_anchor, mode_tonic_instance,
    musical_completion_frame, pitch_at_lens,
};

/// A reduced positive rational. Checked composition cancels before multiplying;
/// neither overflow nor deserialization can turn a ratio into an invalid value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct ExactRatio {
    #[serde(serialize_with = "serialize_exact_integer")]
    numerator: u64,
    #[serde(serialize_with = "serialize_exact_integer")]
    denominator: u64,
}
impl ExactRatio {
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, String> {
        if numerator == 0 || denominator == 0 {
            return Err("a harmonic ratio must be positive".into());
        }
        let divisor = gcd(numerator, denominator);
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }
    pub const fn numerator(self) -> u64 {
        self.numerator
    }
    pub const fn denominator(self) -> u64 {
        self.denominator
    }
    pub const fn reciprocal(self) -> Self {
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }
    pub fn compose(self, other: Self) -> Result<Self, String> {
        let left = gcd(self.numerator, other.denominator);
        let right = gcd(other.numerator, self.denominator);
        Self::new(
            (self.numerator / left)
                .checked_mul(other.numerator / right)
                .ok_or("exact ratio numerator overflow")?,
            (self.denominator / right)
                .checked_mul(other.denominator / left)
                .ok_or("exact ratio denominator overflow")?,
        )
    }
    pub fn divide(self, other: Self) -> Result<Self, String> {
        self.compose(other.reciprocal())
    }
    pub fn pow(self, exponent: i32) -> Result<Self, String> {
        let mut base = if exponent < 0 {
            self.reciprocal()
        } else {
            self
        };
        let mut power = exponent.unsigned_abs();
        let mut result = Self::new(1, 1)?;
        while power != 0 {
            if power & 1 != 0 {
                result = result.compose(base)?;
            }
            power >>= 1;
            if power != 0 {
                base = base.compose(base)?;
            }
        }
        Ok(result)
    }
    pub fn as_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}
impl<'de> Deserialize<'de> for ExactRatio {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Ratio {
            #[serde(deserialize_with = "deserialize_exact_integer")]
            numerator: u64,
            #[serde(deserialize_with = "deserialize_exact_integer")]
            denominator: u64,
        }
        let value = Ratio::deserialize(deserializer)?;
        Self::new(value.numerator, value.denominator).map_err(serde::de::Error::custom)
    }
}
fn serialize_exact_integer<S: serde::Serializer>(
    value: &u64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
fn deserialize_exact_integer<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let text = String::deserialize(deserializer)?;
    let value = text.parse::<u64>().map_err(serde::de::Error::custom)?;
    if value.to_string() != text {
        return Err(serde::de::Error::custom("noncanonical exact ratio integer"));
    }
    Ok(value)
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a
}

/// Unfolded exact generator path, before any external temperament or octave fold.
/// Twelve fifths return to pitch class zero but carry 531441/4096, not 128/1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RationalTrajectory {
    pub basis: MusicalBasis,
    pub steps: i32,
    pub source_generator: crate::m1::RatioEvidence,
    pub ratio: ExactRatio,
    pub pitch_class: u8,
}
pub fn rational_trajectory(basis: MusicalBasis, steps: i32) -> Result<RationalTrajectory, String> {
    let ratios = crate::m1::ratio_basis()?;
    let source_generator = ratios[match basis {
        MusicalBasis::Chromatic => 6,
        MusicalBasis::Fifths => 3,
    }]
    .clone();
    let ratio = ExactRatio::new(
        u64::from(source_generator.ratio[0]),
        u64::from(source_generator.ratio[1]),
    )?
    .pow(steps)?;
    let pitch_class =
        (i64::from(steps) * i64::from(basis.generator_semitones())).rem_euclid(12) as u8;
    Ok(RationalTrajectory {
        basis,
        steps,
        source_generator,
        ratio,
        pitch_class,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionOperator {
    /// Both faces' local positions 1..4; not the conventional octatonic scale.
    ExplicateInnerPositions,
    /// Both faces' local positions 0 and 5; boundary addresses, not audio voices.
    ImplicateOuterPositions,
    /// The canonical seven CF selections; five inner and two outer positions.
    ContextFrameDiatonic,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchitecturalPitch {
    pub coordinate: QlCoordinate,
    pub pitch_class: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalCollection<const N: usize> {
    pub operator: CollectionOperator,
    pub positions: [ArchitecturalPitch; N],
}

/// A performance selection reuses the accepted A/B/C and D completion types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationSelection {
    pub family: RelationFamily,
    pub pair_index: u8,
    pub degree: ConjugationDegree,
    pub expansion_side: Option<ExpansionSide>,
}

/// Private state prevents a caller from changing one identity after admission.
/// The source-coordinate face and the temporal/Spanda phase are independent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicalDetermination {
    identity: PoleIdentity,
    requested_coordinate: String,
    coordinate: MCoordinate,
    basis: MusicalBasis,
    lens: LensId,
    context_frame: ContextFrameId,
    completion: MusicalCompletionFrame,
    explicate: PositionalCollection<8>,
    implicate: PositionalCollection<4>,
    diatonic: PositionalCollection<7>,
    mode: crate::ModeTonicInstance,
}
impl MusicalDetermination {
    pub fn new(
        identity: PoleIdentity,
        reference: &str,
        face: MFace,
        basis: MusicalBasis,
        lens: LensId,
        context_frame: ContextFrameId,
        relation: RelationSelection,
    ) -> Result<Self, String> {
        nonempty(identity.event_ref(), "musical event")?;
        let registry = crate::m_tree::native_current_m_registry();
        // A terminal prime is a face declaration, never a request for a parent.
        let source = if let Some(source) = reference
            .strip_suffix('\'')
            .or_else(|| reference.strip_suffix('′'))
        {
            if face != MFace::Pratibimba {
                return Err("prime source spelling conflicts with Bimba face".into());
            }
            source
        } else {
            reference
        };
        let node = registry
            .resolve(source)
            .ok_or("unknown exact musical source coordinate")?;
        if node.root_position != Some(1) {
            return Err("musical oscillator subject must be an exact M1 coordinate".into());
        }
        let coordinate = registry.coordinate(source, face)?;
        let completion = musical_completion_frame(
            basis,
            lens,
            relation.family,
            relation.pair_index,
            relation.degree,
            relation.expansion_side,
        )
        .map_err(|error| error.to_string())?;
        let pitches = |coordinates: Vec<QlCoordinate>| -> Vec<ArchitecturalPitch> {
            coordinates
                .into_iter()
                .map(|coordinate| ArchitecturalPitch {
                    coordinate,
                    pitch_class: pitch_at_lens(basis, lens, coordinate),
                })
                .collect()
        };
        let explicate = PositionalCollection {
            operator: CollectionOperator::ExplicateInnerPositions,
            positions: pitches(explicate_coordinates())
                .try_into()
                .map_err(|_| "native explicate cardinality drift")?,
        };
        let implicate = PositionalCollection {
            operator: CollectionOperator::ImplicateOuterPositions,
            positions: pitches(implicate_coordinates())
                .try_into()
                .map_err(|_| "native implicate cardinality drift")?,
        };
        let mode_kind = ModeKind::ALL
            .into_iter()
            .find(|mode| mode.context_frame() == context_frame)
            .ok_or("Context Frame has no accepted musical mode")?;
        let mode = mode_tonic_instance(basis, lens, mode_kind);
        let canonical = ContextFrameId::ALL.map(|frame| frame.canonical_selection());
        let diatonic = PositionalCollection {
            operator: CollectionOperator::ContextFrameDiatonic,
            positions: std::array::from_fn(|index| {
                let selected = canonical[(mode_kind.index() + index) % 7];
                let face = match selected.unit_face() {
                    crate::MefUnitFace::Name => ql_core::QlFace::Direct,
                    crate::MefUnitFace::Power => ql_core::QlFace::Conjugate,
                };
                ArchitecturalPitch {
                    coordinate: QlCoordinate::new(selected.local_position(), face),
                    pitch_class: mode.pitches[index],
                }
            }),
        };
        Ok(Self {
            identity,
            requested_coordinate: reference.into(),
            coordinate,
            basis,
            lens,
            context_frame,
            completion,
            explicate,
            implicate,
            diatonic,
            mode,
        })
    }
    /// Read the admitted M1 state. No duplicate clock or loose lens encoding.
    pub fn from_engine(
        engine: &crate::m1_engine::M1Engine,
        face: MFace,
        relation: RelationSelection,
    ) -> Result<Self, String> {
        let config = engine.config();
        let basis = match config.basis {
            crate::m1_engine::Basis::Chromatic => MusicalBasis::Chromatic,
            crate::m1_engine::Basis::Fifths => MusicalBasis::Fifths,
        };
        let lens = *LensId::ALL
            .get(usize::from(config.lens12))
            .ok_or("invalid native M1 lens")?;
        let context_frame = *ContextFrameId::ALL
            .get(
                usize::from(config.context_frame)
                    .checked_sub(1)
                    .ok_or("invalid native M1 CF")?,
            )
            .ok_or("invalid native M1 CF")?;
        Self::new(
            engine.pole_identity()?,
            &config.selected_coordinate,
            face,
            basis,
            lens,
            context_frame,
            relation,
        )
    }
    pub fn identity(&self) -> &PoleIdentity {
        &self.identity
    }
    pub fn requested_coordinate(&self) -> &str {
        &self.requested_coordinate
    }
    pub fn coordinate(&self) -> &MCoordinate {
        &self.coordinate
    }
    pub const fn basis(&self) -> MusicalBasis {
        self.basis
    }
    pub const fn lens(&self) -> LensId {
        self.lens
    }
    pub const fn context_frame(&self) -> ContextFrameId {
        self.context_frame
    }
    pub fn completion(&self) -> &MusicalCompletionFrame {
        &self.completion
    }
    pub fn explicate_octet(&self) -> &PositionalCollection<8> {
        &self.explicate
    }
    pub fn nodal_quartet(&self) -> &PositionalCollection<4> {
        &self.implicate
    }
    pub fn diatonic_cut(&self) -> &PositionalCollection<7> {
        &self.diatonic
    }
    pub fn mode(&self) -> &crate::ModeTonicInstance {
        &self.mode
    }
    pub fn lens_anchor(&self) -> crate::LensAnchor {
        lens_anchor(self.basis, self.lens)
    }
    /// A retained M2 read must consume this event and generation exactly.
    pub fn validate_consumer(
        &self,
        stamp: &InputStamp,
        coordinate: &MCoordinate,
        phase: u8,
    ) -> Result<(), String> {
        nonempty(&stamp.source_ref, "M2 input source")?;
        nonempty(&stamp.contract_ref, "M2 input contract")?;
        if stamp.identity.event_ref != self.identity.event_ref()
            || stamp.identity.profile_generation != self.identity.profile_generation()
        {
            return Err("disconnected or stale M2 musical consumer".into());
        }
        if stamp.source_ref != crate::m2_vimarsha::SOURCE
            || stamp.contract_ref != crate::m2_vimarsha::POLICY
        {
            return Err(
                "musical consumer lacks the accepted native Vimarśā source/contract binding".into(),
            );
        }
        let expected =
            crate::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
        if coordinate != &expected {
            return Err(
                "musical target writer must be the exact source-backed M2-1′ Vimarśā coordinate"
                    .into(),
            );
        }
        if phase != self.identity.tick12() / 6 {
            return Err("Vimarśā temporal phase is disconnected from the M1 clock".into());
        }
        Ok(())
    }
    /// Control-thread admission of the actual joined owner result. M2's
    /// composition generation is retained separately from M1's source revision.
    /// Recomposition authenticates derivation coherence, not host authority.
    /// This operation allocates and must never run in an audio callback.
    pub fn validate_coupled_consumer(
        &self,
        basis: &crate::continuous::coupled::CoupledBasis,
        coordinate: &MCoordinate,
        phase: u8,
    ) -> Result<(), String> {
        let expected =
            crate::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
        if coordinate != &expected || phase != self.identity.tick12() / 6 {
            return Err("joined musical writer coordinate/face/phase mismatch".into());
        }
        let m1 = crate::m1_engine::M1Engine::new(basis.input.m1.clone())?;
        if m1.pole_identity()? != self.identity || m1.snapshot()? != basis.m1 {
            return Err("stale or disconnected joined M1 source".into());
        }
        let config = m1.config();
        let native_basis = match config.basis {
            crate::m1_engine::Basis::Chromatic => MusicalBasis::Chromatic,
            crate::m1_engine::Basis::Fifths => MusicalBasis::Fifths,
        };
        if native_basis != self.basis || LensId::ALL[usize::from(config.lens12)] != self.lens {
            return Err("joined musical basis/lens mismatch".into());
        }
        let native_coordinate = crate::m_tree::native_current_m_registry()
            .coordinate(&config.selected_coordinate, self.coordinate.face)?;
        if native_coordinate != self.coordinate {
            return Err("joined musical source branch mismatch".into());
        }
        let replay = basis.input.compose()?;
        if replay.m1 != basis.m1
            || replay.m2 != basis.m2
            || replay.m3 != basis.m3
            || replay.m3_receipts != basis.m3_receipts
            || replay.derivation != basis.derivation
            || serde_json::to_value(&replay.m2_input).map_err(|e| e.to_string())?
                != serde_json::to_value(&basis.m2_input).map_err(|e| e.to_string())?
        {
            return Err("joined consumer differs from native current composition".into());
        }
        if basis.derivation["context_frame"].as_str() != Some(self.context_frame.code())
            || basis.m2_input.stamp.identity.event_ref != self.identity.event_ref()
            || basis.m2_input.tick12 != self.identity.tick12()
            || basis.m2_input.degree720 != self.identity.degree720()
        {
            return Err("joined event/clock/Context Frame mismatch".into());
        }
        Ok(())
    }
}

/// D30: supplied magnitudes and policies always carry their declared standing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TuningStanding {
    SourceAuthored,
    Ratified,
    Reference,
    AgentProposed,
    External,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TuningProvenance {
    pub policy_ref: String,
    pub source_ref: String,
    pub revision: String,
    pub standing: TuningStanding,
}
impl TuningProvenance {
    fn validate(&self) -> Result<(), String> {
        nonempty(&self.policy_ref, "tuning policy")?;
        nonempty(&self.source_ref, "tuning source")?;
        nonempty(&self.revision, "tuning revision")
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Fundamental {
    hertz: f64,
    provenance: TuningProvenance,
}
impl Fundamental {
    /// Reference pitch-class zero; a lens change does not replace its provenance.
    pub fn new(hertz: f64, provenance: TuningProvenance) -> Result<Self, String> {
        positive_finite(hertz, "fundamental Hz")?;
        provenance.validate()?;
        Ok(Self { hertz, provenance })
    }
    pub const fn hertz(&self) -> f64 {
        self.hertz
    }
    pub fn provenance(&self) -> &TuningProvenance {
        &self.provenance
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TuningPolicy {
    /// Caller-supplied ordered pitch-class targets in [1,2), anchored at 1/1.
    /// This preserves just/non-tempered targets rather than manufacturing them
    /// from a pitch-class label or a matrix digital root.
    ExactPitchRatios {
        ratios: [ExactRatio; 12],
        provenance: TuningProvenance,
    },
    /// Explicit external approximation. It never changes an exact trajectory.
    EqualTemperament12 { provenance: TuningProvenance },
}
impl TuningPolicy {
    fn validate(&self) -> Result<(), String> {
        self.provenance().validate()?;
        if let Self::ExactPitchRatios { ratios, .. } = self {
            if ratios[0] != ExactRatio::new(1, 1)? {
                return Err("pitch-class zero must be the exact unison reference".into());
            }
            for (index, ratio) in ratios.iter().enumerate() {
                if u128::from(ratio.numerator) >= 2 * u128::from(ratio.denominator) {
                    return Err("pitch-class ratio must be below its octave return".into());
                }
                if index > 0 {
                    let previous = ratios[index - 1];
                    if u128::from(previous.numerator) * u128::from(ratio.denominator)
                        >= u128::from(ratio.numerator) * u128::from(previous.denominator)
                    {
                        return Err("pitch-class ratio targets must be strictly increasing".into());
                    }
                }
            }
        } else if self.provenance().standing != TuningStanding::External {
            return Err("12-TET approximation must declare External standing".into());
        }
        Ok(())
    }
    pub fn provenance(&self) -> &TuningProvenance {
        match self {
            Self::ExactPitchRatios { provenance, .. } | Self::EqualTemperament12 { provenance } => {
                provenance
            }
        }
    }
    fn target(
        &self,
        pitch_class: u8,
        octave: u8,
        fundamental: &Fundamental,
    ) -> Result<TunedPitch, String> {
        let (multiplier, exact_ratio) = match self {
            Self::ExactPitchRatios { ratios, .. } => {
                let ratio = ratios[usize::from(pitch_class)]
                    .compose(ExactRatio::new(2, 1)?.pow(i32::from(octave))?)?;
                (ratio.as_f64(), Some(ratio))
            }
            Self::EqualTemperament12 { .. } => (
                2.0_f64.powf(f64::from(pitch_class) / 12.0 + f64::from(octave)),
                None,
            ),
        };
        let hertz = fundamental.hertz * multiplier;
        positive_finite(hertz, "target Hz")?;
        Ok(TunedPitch {
            pitch_class,
            octave,
            exact_ratio,
            hertz,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TunedPitch {
    pub pitch_class: u8,
    pub octave: u8,
    pub exact_ratio: Option<ExactRatio>,
    pub hertz: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct VimarshaTargets {
    pub determination: MusicalDetermination,
    pub fundamental: Fundamental,
    pub tuning_policy: TuningPolicy,
    pub audio_octet: [TunedPitch; 8],
    /// Addresses and roles only: a nodal boundary is not a ninth audio target.
    pub nodal_quartet: PositionalCollection<4>,
    pub diatonic: [TunedPitch; 7],
    pub octave_return: TunedPitch,
}
impl VimarshaTargets {
    /// Canonical Jankó key order is the existing chromatic QL address field:
    /// direct/conjugate positions interleave. Basis and lens act on that address,
    /// rather than indexing the eightfold audio bus. A repeated touch is a
    /// distinct excitation token and does not change the canonical note.
    pub fn key_target(&self, key: u8, register: i8, touch_ref: &str) -> Result<NoteTarget, String> {
        if key >= 12 {
            return Err("canonical Jankó key must be in 0..12".into());
        }
        let face = if key % 2 == 0 {
            ql_core::QlFace::Direct
        } else {
            ql_core::QlFace::Conjugate
        };
        self.note_target(
            QlCoordinate::new(
                ql_core::QlPosition::new(key / 2).map_err(|error| error.to_string())?,
                face,
            ),
            register,
            touch_ref,
        )
    }
    pub fn note_target(
        &self,
        coordinate: QlCoordinate,
        register: i8,
        touch_ref: &str,
    ) -> Result<NoteTarget, String> {
        nonempty(touch_ref, "note touch")?;
        self.tuning_policy.validate()?;
        let pitch_class = pitch_at_lens(
            self.determination.basis,
            self.determination.lens,
            coordinate,
        );
        let octave_ratio = ExactRatio::new(2, 1)?.pow(i32::from(register))?;
        let (exact_ratio, multiplier) = match &self.tuning_policy {
            TuningPolicy::ExactPitchRatios { ratios, .. } => {
                let ratio = ratios[usize::from(pitch_class)].compose(octave_ratio)?;
                (Some(ratio), ratio.as_f64())
            }
            TuningPolicy::EqualTemperament12 { .. } => (
                None,
                2.0_f64.powf(f64::from(pitch_class) / 12.0 + f64::from(register)),
            ),
        };
        let hertz = self.fundamental.hertz * multiplier;
        positive_finite(hertz, "note target Hz")?;
        Ok(NoteTarget {
            identity: self.determination.identity.clone(),
            source_coordinate: self.determination.coordinate.clone(),
            coordinate,
            register,
            touch_ref: touch_ref.into(),
            pitch_class,
            exact_ratio,
            hertz,
            fundamental: self.fundamental.clone(),
            tuning_provenance: self.tuning_policy.provenance().clone(),
        })
    }
    pub fn audio_octet_hz(&self) -> [f64; 8] {
        self.audio_octet.map(|pitch| pitch.hertz)
    }
    /// A scale's seven degrees plus its return; never the oscillator bus octet.
    pub fn octave_process_hz(&self) -> [f64; 8] {
        std::array::from_fn(|index| {
            if index == 7 {
                self.octave_return.hertz
            } else {
                self.diatonic[index].hertz
            }
        })
    }
}
/// Per-note M1 excitation handed through the admitted M2 determination. This
/// carries its source face independently of the temporal phase in identity.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteTarget {
    pub identity: PoleIdentity,
    pub source_coordinate: MCoordinate,
    pub coordinate: QlCoordinate,
    pub register: i8,
    pub touch_ref: String,
    pub pitch_class: u8,
    pub exact_ratio: Option<ExactRatio>,
    pub hertz: f64,
    pub fundamental: Fundamental,
    pub tuning_provenance: TuningProvenance,
}
/// Compute authoritative targets once for M2. A sounding consumer reads these
/// values; it must not regenerate a renderer-local 12-TET table.
pub fn vimarsha_targets(
    determination: MusicalDetermination,
    fundamental: Fundamental,
    tuning_policy: TuningPolicy,
) -> Result<VimarshaTargets, String> {
    tuning_policy.validate()?;
    let mut audio = Vec::with_capacity(8);
    for position in determination.explicate.positions {
        audio.push(tuning_policy.target(position.pitch_class, 0, &fundamental)?);
    }
    let tonic = determination.mode.tonic;
    let mut diatonic = Vec::with_capacity(7);
    for pitch in determination.mode.pitches {
        diatonic.push(tuning_policy.target(pitch, u8::from(pitch < tonic), &fundamental)?);
    }
    let octave_return = tuning_policy.target(tonic, 1, &fundamental)?;
    Ok(VimarshaTargets {
        nodal_quartet: determination.implicate.clone(),
        determination,
        fundamental,
        tuning_policy,
        audio_octet: audio
            .try_into()
            .map_err(|_| "audio target cardinality drift")?,
        diatonic: diatonic
            .try_into()
            .map_err(|_| "diatonic target cardinality drift")?,
        octave_return,
    })
}
fn nonempty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 4096 || value.contains('\0') {
        return Err(format!("bounded {label} reference required"));
    }
    Ok(())
}
fn positive_finite(value: f64, label: &str) -> Result<(), String> {
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("positive finite {label} required"));
    }
    Ok(())
}
