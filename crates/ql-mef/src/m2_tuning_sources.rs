//! Native source collections for explicit sparse key reduction.
//! Sources determine entries; K determines an explicit key/degree assignment.
//! No missing pitch, twelve-key expansion or eight-component mapping is inferred.
use crate::m2::{catalogue, maqam_pitches};
use crate::m2_condition::{
    CorrespondenceRole, TuningPolicy as ConditionTuning, condition_pitches, correspondence_field,
};
use crate::m2_relation_plan::source_field;
use crate::m2_sky;
use crate::music_determination::{
    ExactRatio, Fundamental, MusicalDetermination, TuningPolicy, TuningProvenance, VimarshaTargets,
    rational_trajectory, vimarsha_targets,
};
use serde::Serialize;
use serde_json::{Value, json};

pub const TUNING_SOURCE_COLLECTION_SCHEMA: &str = "ql.tuning-source-collection/v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "availability", content = "value", rename_all = "snake_case")]
pub enum SourcePitchAvailability {
    ExactRatio(ExactRatio),
    /// An explicit approximation in 1/24-octave steps, never an exact ratio.
    RetainedQuarterToneSteps(i16),
    Unavailable(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionStanding {
    RetainedApproximation,
    SourceAuthoredInterval,
    SourceSpelledApproximation,
    NativeRationalOperator,
    ExplicitArchitecturalTuning,
    SourcePitchUnavailable,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TonicProvenance {
    pub hertz: f64,
    pub provenance: TuningProvenance,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningSourceEntry {
    pub original_degree: u16,
    pub source_coordinate: String,
    pub source_prime: bool,
    /// Full native row/property/operator witness, not a digest as authority.
    pub source_receipt: Value,
    pub octave_return: bool,
    pub spelling: Option<String>,
    pub pitch: SourcePitchAvailability,
}
/// Only native producers construct this object. A serialized receipt does not
/// grant source, consent, device or native Act authority.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningSourceCollection {
    schema: String,
    collection_ref: String,
    registry_revision: String,
    source_revision: String,
    standing: CollectionStanding,
    tradition: String,
    variant: String,
    tonic: TonicProvenance,
    entries: Vec<TuningSourceEntry>,
    source_receipt: Value,
}
impl TuningSourceCollection {
    pub fn collection_ref(&self) -> &str {
        &self.collection_ref
    }
    pub fn registry_revision(&self) -> &str {
        &self.registry_revision
    }
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }
    pub fn standing(&self) -> &CollectionStanding {
        &self.standing
    }
    pub fn tradition(&self) -> &str {
        &self.tradition
    }
    pub fn variant(&self) -> &str {
        &self.variant
    }
    pub fn tonic_provenance(&self) -> &TonicProvenance {
        &self.tonic
    }
    pub fn entries(&self) -> &[TuningSourceEntry] {
        &self.entries
    }
    pub fn source_receipt(&self) -> &Value {
        &self.source_receipt
    }
    pub fn entry(&self, original_degree: u16) -> Option<&TuningSourceEntry> {
        self.entries
            .iter()
            .find(|entry| entry.original_degree == original_degree)
    }
    /// The caller holds the independently native-produced collection. Complete
    /// comparison rejects truncated/relabelled JSON before any target is made.
    pub fn verify_retained(&self, retained: &Value) -> Result<(), String> {
        if serde_json::to_value(self).map_err(|e| e.to_string())? != *retained {
            return Err("retained tuning source differs from complete native collection".into());
        }
        Ok(())
    }
}
fn tonic(value: &Fundamental) -> TonicProvenance {
    TonicProvenance {
        hertz: value.hertz(),
        provenance: value.provenance().clone(),
    }
}
fn collection(
    reference: String,
    standing: CollectionStanding,
    tradition: &str,
    variant: &str,
    fundamental: &Fundamental,
    entries: Vec<TuningSourceEntry>,
    receipt: Value,
) -> Result<TuningSourceCollection, String> {
    if entries.is_empty() || entries.len() > 64 {
        return Err("source collection must have 1..64 explicit entries".into());
    }
    // Revalidate the declared reference magnitude/provenance, independent of a
    // source interval's authenticity. Fundamental does not claim a tuning law.
    Fundamental::new(fundamental.hertz(), fundamental.provenance().clone())?;
    let field = source_field();
    Ok(TuningSourceCollection {
        schema: TUNING_SOURCE_COLLECTION_SCHEMA.into(),
        collection_ref: reference,
        registry_revision: field.registry_revision.clone(),
        source_revision: field.source_revision.clone(),
        standing,
        tradition: tradition.into(),
        variant: variant.into(),
        tonic: tonic(fundamental),
        entries,
        source_receipt: receipt,
    })
}
/// Seven source increments plus the explicitly retained return, with complete
/// row/source locks and exact typed correspondence path. Missing paths refuse.
pub fn retained_condition_collection(
    index: u8,
    role: CorrespondenceRole,
    fundamental: &Fundamental,
) -> Result<TuningSourceCollection, String> {
    let field = correspondence_field();
    let rule = field
        .rule(index, role)
        .ok_or("no admitted correspondence source path")?;
    let row = catalogue().table("maqam")?.row(usize::from(index))?;
    let reading = catalogue().reading("maqam", usize::from(index))?;
    let pitches = maqam_pitches(index, fundamental.hertz())?;
    let receipt = json!({"operation":"retained-seven-increments-plus-return",
        "descriptor":reading,"row":row,"retained_source_locks":catalogue().sources(),
        "correspondence":rule,"source_revision":field.source_revision});
    let mut steps = 0u64;
    let mut entries = Vec::with_capacity(8);
    for (degree, expected_hertz) in pitches.into_iter().enumerate() {
        if degree != 0 {
            steps = steps
                .checked_add(row[1 + degree])
                .ok_or("quarter-tone sum overflow")?;
        }
        let steps = i16::try_from(steps)
            .map_err(|_| "quarter-tone source exceeds bounded step representation")?;
        let actual = fundamental.hertz() * 2f64.powf(f64::from(steps) / 24.0);
        if actual != expected_hertz {
            return Err("retained native pitch/row disagreement".into());
        }
        entries.push(TuningSourceEntry {
            original_degree: degree as u16,
            source_coordinate: rule.maqam_coordinate.clone(),
            source_prime: false,
            source_receipt: json!({"collection":receipt,"degree":degree,"steps24":steps}),
            octave_return: degree == 7,
            spelling: None,
            pitch: SourcePitchAvailability::RetainedQuarterToneSteps(steps),
        });
    }
    collection(
        format!("ql:maqam/{index}/{role:?}/retained24tet"),
        CollectionStanding::RetainedApproximation,
        "retained-C-maqam",
        "equal-quarter-tone-source-row",
        fundamental,
        entries,
        receipt,
    )
}
/// Source spelling may remain unavailable. A current path without pitch data
/// carries its full claims/path and eight explicit unavailable entries.
pub fn source_spelled_condition_collection(
    index: u8,
    role: CorrespondenceRole,
    fundamental: &Fundamental,
) -> Result<TuningSourceCollection, String> {
    let field = correspondence_field();
    let rule = field
        .rule(index, role)
        .ok_or("no admitted correspondence source path")?;
    let expected = condition_pitches(
        index,
        role,
        ConditionTuning::BimbaSpelled24Tet,
        fundamental.hertz(),
    )?;
    let receipt = json!({"operation":"source-spelled-quarter-tone-reading", "correspondence":rule,
        "source_revision":field.source_revision,"source_locks":field.source_locks,
        "interval_literal":rule.interval_literal,"spelled_steps24":rule.spelled_steps24});
    let mut entries = Vec::with_capacity(8);
    for degree in 0..8 {
        let pitch = match (&rule.spelled_steps24, &expected) {
            (Some(steps), Some(pitches)) => {
                let step = *steps
                    .get(degree)
                    .ok_or("source-spelled collection lost a degree")?;
                if fundamental.hertz() * 2f64.powf(f64::from(step) / 24.0) != pitches[degree] {
                    return Err("source-spelled pitch/step disagreement".into());
                }
                SourcePitchAvailability::RetainedQuarterToneSteps(i16::from(step))
            }
            (None, None) => SourcePitchAvailability::Unavailable(
                "current-source-has-no-complete-spelled-pitch-set".into(),
            ),
            _ => return Err("source-spelled availability disagreement".into()),
        };
        entries.push(TuningSourceEntry {
            original_degree: degree as u16,
            source_coordinate: rule.maqam_coordinate.clone(),
            source_prime: false,
            source_receipt: json!({"collection":receipt,"degree":degree}),
            octave_return: degree == 7,
            spelling: None,
            pitch,
        });
    }
    let standing = if expected.is_some() {
        CollectionStanding::SourceSpelledApproximation
    } else {
        CollectionStanding::SourcePitchUnavailable
    };
    collection(
        format!("ql:maqam/{index}/{role:?}/source-spelled"),
        standing,
        "Bimba-maqam",
        "literal-spelled-quarter-tone-reading",
        fundamental,
        entries,
        receipt,
    )
}
// Same explicit first parenthesized n:d grammar as the source-owned sky
// projection. It preserves Sun's first1:1 and retains its full1:1/2:1 literal.
fn parse_interval_ratio(literal: &str) -> Option<[u16; 2]> {
    for (start, _) in literal.match_indices('(') {
        let inside = &literal[start + 1..];
        let numerator_len = inside.bytes().take_while(u8::is_ascii_digit).count();
        if numerator_len == 0 || inside.as_bytes().get(numerator_len) != Some(&b':') {
            continue;
        }
        let denominator = &inside[numerator_len + 1..];
        let denominator_len = denominator.bytes().take_while(u8::is_ascii_digit).count();
        if denominator_len == 0 {
            continue;
        }
        return Some([
            inside[..numerator_len].parse().ok()?,
            denominator[..denominator_len].parse().ok()?,
        ]);
    }
    None
}
/// One actual planetary interval. Sun's unison/octave literal is preserved;
/// Pluto9:4 stays unfolded. Earth/Uranus absence never receives a filler ratio.
pub fn planetary_interval_collection(
    reference: &str,
    fundamental: &Fundamental,
) -> Result<TuningSourceCollection, String> {
    let field = source_field();
    let native = field
        .node(reference)
        .ok_or("planetary source coordinate unavailable")?;
    let sky = m2_sky::node(reference).ok_or("planetary sky source unavailable")?;
    if sky.id != format!("{:016x}", native.id.as_u64()) {
        return Err("planetary sky/native identity differs".into());
    }
    let literal = field
        .property(reference, "m_2_5_interval_from_root")
        .cloned();
    if literal != sky.properties.get("m_2_5_interval_from_root").cloned() {
        return Err("planetary interval differs from current source properties".into());
    }
    let parsed = literal
        .as_ref()
        .and_then(Value::as_str)
        .and_then(parse_interval_ratio);
    if parsed != sky.just_ratio {
        return Err("planetary numeric ratio differs from original interval literal".into());
    }
    let pitch = match sky.just_ratio {
        Some([n, d]) => {
            SourcePitchAvailability::ExactRatio(ExactRatio::new(u64::from(n), u64::from(d))?)
        }
        None => {
            SourcePitchAvailability::Unavailable("source-has-no-planetary-interval-ratio".into())
        }
    };
    let receipt = json!({"operation":"individual-unfolded-planetary-interval", "source_node":native,
        "literal":literal,"original_ratio":sky.just_ratio,"map_content_sha256":m2_sky::sky().map_content_sha256});
    let entry = TuningSourceEntry {
        original_degree: 0,
        source_coordinate: reference.into(),
        source_prime: false,
        source_receipt: receipt.clone(),
        octave_return: false,
        spelling: None,
        pitch,
    };
    let standing = if sky.just_ratio.is_some() {
        CollectionStanding::SourceAuthoredInterval
    } else {
        CollectionStanding::SourcePitchUnavailable
    };
    collection(
        format!("ql:planetary-interval/{reference}"),
        standing,
        "source-defined-planetary-intervals",
        "original-unreduced-interval-literal",
        fundamental,
        vec![entry],
        receipt,
    )
}
/// Native diatonic target sequence is separate from the architectural seven
/// address cut and eight antinodal bus. The exact existing tuning is explicit.
pub fn architectural_diatonic_collection(
    targets: &VimarshaTargets,
) -> Result<TuningSourceCollection, String> {
    let replay = vimarsha_targets(
        targets.determination.clone(),
        targets.fundamental.clone(),
        targets.tuning_policy.clone(),
    )?;
    if replay != *targets {
        return Err("diatonic target differs from actual native operator/tuning".into());
    }
    let determination = &targets.determination;
    let coordinates=determination.diatonic_cut().positions.map(|p|json!({"position":p.coordinate.position.value(),"face":p.coordinate.face.as_str(),"pitch_class":p.pitch_class}));
    let receipt = json!({"operation":"context-frame-diatonic-native-targets", "source_coordinate":determination.coordinate().source_ref,
        "source_prime":determination.coordinate().face==crate::MFace::Pratibimba,
        "event_ref":determination.identity().event_ref(),"profile_generation":determination.identity().profile_generation().to_string(),
        "basis":format!("{:?}",determination.basis()),"lens":determination.lens().slot(),
        "context_frame":determination.context_frame().code(),"architectural_diatonic_cut":coordinates,
        "mode_pitches":determination.mode().pitches,"tonic":determination.mode().tonic,
        "tuning_provenance":targets.tuning_policy.provenance()});
    let mut entries = Vec::with_capacity(8);
    for (degree, pitch) in targets
        .diatonic
        .iter()
        .chain(std::iter::once(&targets.octave_return))
        .enumerate()
    {
        let available = match &targets.tuning_policy {
            TuningPolicy::ExactPitchRatios { .. } => SourcePitchAvailability::ExactRatio(
                pitch.exact_ratio.ok_or("exact diatonic ratio absent")?,
            ),
            TuningPolicy::EqualTemperament12 { .. } => {
                SourcePitchAvailability::RetainedQuarterToneSteps(
                    2 * (i16::from(pitch.pitch_class) + 12 * i16::from(pitch.octave)),
                )
            }
        };
        entries.push(TuningSourceEntry { original_degree:degree as u16, source_coordinate:determination.coordinate().source_ref.clone(),
            source_prime:determination.coordinate().face==crate::MFace::Pratibimba,
            source_receipt:json!({"collection":receipt,"degree":degree,"pitch_class":pitch.pitch_class,"octave":pitch.octave,
                "actual_hertz":pitch.hertz,"exact_ratio":pitch.exact_ratio}),octave_return:degree==7,spelling:None,pitch:available });
    }
    collection(
        format!(
            "ql:diatonic/{}/{:?}/{:?}/{}",
            determination.requested_coordinate(),
            determination.basis(),
            determination.lens(),
            determination.context_frame().code()
        ),
        CollectionStanding::ExplicitArchitecturalTuning,
        "QL-native-context-frame",
        "independent-seven-degrees-plus-return",
        &targets.fundamental,
        entries,
        receipt,
    )
}
/// Exact unfolded rational paths retain the selected native generator evidence.
/// The caller supplies the actual desired steps; pitch classes never close the
/// rational path by silently folding or tempering a Pythagorean comma.
pub fn rational_path_collection(
    determination: &MusicalDetermination,
    steps: &[i32],
    fundamental: &Fundamental,
) -> Result<TuningSourceCollection, String> {
    if steps.is_empty() || steps.len() > 64 {
        return Err("rational path must have 1..64 explicit steps".into());
    }
    let mut entries = Vec::with_capacity(steps.len());
    for (degree, step) in steps.iter().enumerate() {
        let path = rational_trajectory(determination.basis(), *step)?;
        entries.push(TuningSourceEntry { original_degree:degree as u16,source_coordinate:determination.coordinate().source_ref.clone(),
            source_prime:determination.coordinate().face==crate::MFace::Pratibimba,
            source_receipt:json!({"operation":"native-unfolded-rational-trajectory","steps":step,
                "source_generator":path.source_generator,"pitch_class":path.pitch_class,"ratio":path.ratio}),
            octave_return:false,spelling:None,pitch:SourcePitchAvailability::ExactRatio(path.ratio) });
    }
    let receipt = json!({"operation":"native-unfolded-rational-path","source_coordinate":determination.coordinate().source_ref,
        "source_prime":determination.coordinate().face==crate::MFace::Pratibimba,"steps":steps,
        "basis":format!("{:?}",determination.basis()),"event_ref":determination.identity().event_ref(),
        "profile_generation":determination.identity().profile_generation().to_string()});
    collection(
        format!(
            "ql:rational-path/{}/{:?}",
            determination.requested_coordinate(),
            determination.basis()
        ),
        CollectionStanding::NativeRationalOperator,
        "QL-native-rational-trajectory",
        "unfolded-explicit-generator-steps",
        fundamental,
        entries,
        receipt,
    )
}
