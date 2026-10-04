//! Source-qualified musical score over the existing Expression event pages.
//! Musical samples/tempo do not advance M3's geometric or civil clock. M3's
//! actual form/transcription and M4's original occasion remain exact sources;
//! this owner adds a musical inscription without inventing a codon classifier.
//! Page payloads remain in native Expression/Act custody, not another store.
use crate::music_determination::NoteTarget;
use crate::musical_performance_return::MusicalPerformanceReturn;
use crate::performance_audio::PreparedPerformanceBinding;
use crate::source_key_determination::{
    SparseConditionConsumer, SparseKeyTargets, SparseMusicalConsumer,
};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "ql.musical-performance-score/v1";
const MAX_EVENTS: usize = 262_144;
const MAX_BASES: usize = 256;
const MAX_PITCHES: usize = 4096;
const MAX_PAGE_EVENTS: usize = 4096;

/// The selected producer is explicit. A missing sparse key is never replaced
/// by the architectural twelve-key policy retained for another instrument.
pub enum ScoreKeys<'a> {
    Architectural,
    Sparse {
        targets: &'a SparseKeyTargets,
        consumer: SparseMusicalConsumer<'a>,
    },
}
pub struct ScoreSource<'a> {
    pub prepared: &'a PreparedPerformanceBinding,
    pub original_return: &'a MusicalPerformanceReturn,
    pub keys: ScoreKeys<'a>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScorePage {
    pub part_ref: String,
    pub events: usize,
    pub first_sample: String,
    pub last_sample: String,
    pub first_sequence: String,
    pub last_sequence: String,
}
/// Native-produced immutable return. Serialized receipts are replay evidence;
/// the native source/Act owners still establish admission and CAS authority.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MusicalPerformanceScore {
    schema: &'static str,
    expression_ref: String,
    performance_ref: String,
    edition_generation: String,
    performance_digest: String,
    expression_performance_revision: String,
    source_bases: Vec<Value>,
    pitch_sources: Vec<Value>,
    pages: Vec<ScorePage>,
    event_count: usize,
    /// Exact authored operands. No notes, routes, body or context are dropped.
    controls: Value,
    original_episode_refs: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_custody: Option<crate::musical_performance_source_score::NativeScoreCustody>,
    content_digest: String,
}
fn digest(v: &Value) -> Result<String, String> {
    Ok(format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(v).map_err(|e| e.to_string())?)
    ))
}
fn counter(v: &Value) -> Result<u64, String> {
    let text = v
        .as_str()
        .ok_or("canonical native score counter required")?;
    let value = text.parse::<u64>().map_err(|e| e.to_string())?;
    if value.to_string() != text {
        return Err("noncanonical native score counter".into());
    }
    Ok(value)
}
fn number(v: &Value, low: f64, high: f64) -> Result<f64, String> {
    let n = v.as_f64().ok_or("native score scalar required")?;
    if !n.is_finite() || !(low..=high).contains(&n) {
        return Err("native score scalar outside source bounds".into());
    }
    Ok(n)
}
fn reference(v: &Value) -> Result<&str, String> {
    let s = v.as_str().ok_or("native score reference required")?;
    if s.trim().is_empty() || s.len() > 4096 || s.chars().any(char::is_control) {
        return Err("invalid native score reference".into());
    }
    Ok(s)
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| "native score array required".into())
}
fn index(v: &Value, bound: usize) -> Result<usize, String> {
    let i = v.as_u64().ok_or("native score index required")?;
    let i = usize::try_from(i).map_err(|e| e.to_string())?;
    if i >= bound {
        return Err("native score target index unavailable".into());
    }
    Ok(i)
}
fn exact(actual: f64, expected: f64) -> bool {
    actual.to_bits() == expected.to_bits()
}
fn qualify_pitch(source: &ScoreSource<'_>, pitch: &Value, ordinal: usize) -> Result<Value, String> {
    let key = u8::try_from(pitch["key"].as_u64().ok_or("source key address required")?)
        .map_err(|e| e.to_string())?;
    let register = i8::try_from(pitch["register"].as_i64().ok_or("source octave required")?)
        .map_err(|e| e.to_string())?;
    let touch = format!("native:score/pitch/{ordinal}");
    let (note, key_source): (NoteTarget, Value) = match &source.keys {
        ScoreKeys::Architectural => (
            source
                .prepared
                .targets()
                .key_target(key, register, &touch)?,
            json!({"owner":"native-architectural-key-policy"}),
        ),
        ScoreKeys::Sparse { targets, .. } => {
            let target = targets.key_target(key, register, &touch)?;
            let note = target
                .note()
                .ok_or("source-required score key/pitch unavailable")?
                .clone();
            (note, target.receipt()?)
        }
    };
    let expected_ratio = note.exact_ratio.map(|r| json!({"numerator":r.numerator().to_string(),"denominator":r.denominator().to_string()})).unwrap_or(Value::Null);
    if pitch["source_coordinate"] != note.source_coordinate.source_ref
        || pitch["source_prime"] != json!(note.source_coordinate.face == crate::MFace::Pratibimba)
        || pitch["pitch_class"] != json!(note.pitch_class)
        || pitch["tuning_ref"] != note.tuning_provenance.policy_ref
        || pitch["exact_ratio"] != expected_ratio
        || !exact(number(&pitch["hertz"], 0.001, 192000.0 * 0.45)?, note.hertz)
        || !exact(
            number(&pitch["fundamental_hz"], 0.001, f64::MAX)?,
            note.fundamental.hertz(),
        )
    {
        return Err("musical score pitch detached from exact selected native source".into());
    }
    Ok(
        json!({"pitch_index":ordinal,"basis":pitch["basis"],"key":key,"register":register,
        "source_coordinate":note.source_coordinate.source_ref,"source_prime":note.source_coordinate.face==crate::MFace::Pratibimba,
        "pitch_class":note.pitch_class,"hertz":note.hertz,"fundamental_hz":note.fundamental.hertz(),
        "exact_ratio":expected_ratio,"tuning_provenance":note.tuning_provenance,"key_source":key_source}),
    )
}
impl MusicalPerformanceScore {
    pub fn snapshot(&self) -> Result<Value, String> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
    pub fn generation(&self) -> Result<u64, String> {
        self.edition_generation
            .parse::<u64>()
            .map_err(|e| e.to_string())
    }
    pub fn pages(&self) -> &[ScorePage] {
        &self.pages
    }
    pub fn pitch_sources(&self) -> &[Value] {
        &self.pitch_sources
    }
    pub fn verify_replay(
        &self,
        sources: &[ScoreSource<'_>],
        performance: &Value,
    ) -> Result<(), String> {
        if self.native_custody.is_some() {
            return Err(
                "native score replay requires the complete selected native Act delivery".into(),
            );
        }
        let actual = compile_score(
            &self.expression_ref,
            self.generation()?,
            sources,
            performance,
        )?;
        if actual != *self {
            return Err(
                "retained musical score differs from exact native page/source replay".into(),
            );
        }
        Ok(())
    }
    /// Normal authored/record/edit/layer/route/automation/transport operations
    /// first produce a prospective native Expression revision; this returns its
    /// source-qualified score before the same Scene/Act CAS commits the edit.
    pub fn reinscribe(
        &self,
        expected_generation: u64,
        sources: &[ScoreSource<'_>],
        prospective: &Value,
    ) -> Result<Self, String> {
        if self.native_custody.is_some() {
            return Err(
                "native score edit requires the complete selected native Act delivery".into(),
            );
        }
        if self.generation()? != expected_generation
            || prospective["performance_ref"] != self.performance_ref
        {
            return Err("stale or foreign native musical score edit".into());
        }
        compile_score(
            &self.expression_ref,
            expected_generation
                .checked_add(1)
                .ok_or("score edition generation exhausted")?,
            sources,
            prospective,
        )
    }
    /// Repeat the complete selected Act/page/source activity. Serialized page
    /// witnesses alone never replace the borrowed actual native owners.
    pub fn verify_native_replay(
        &self,
        sources: &[crate::musical_performance_source_score::RetainedScoreSource<'_>],
        manifest: &Value,
        pages: &mut impl crate::musical_performance_source_score::NativeScorePages,
    ) -> Result<(), String> {
        if self.native_custody.is_none() {
            return Err("legacy score requires its original replay interface".into());
        }
        let actual = crate::musical_performance_source_score::compile_retained_score(
            self.generation()?,
            sources,
            manifest,
            pages,
        )?;
        if actual != *self {
            return Err("complete selected native Act score/source replay differs".into());
        }
        Ok(())
    }
    /// Inscribe a prospective edit delivered by the SAME retained native
    /// Expression/Act owner; that owner separately commits its Scene/Act CAS.
    pub fn reinscribe_native(
        &self,
        expected_generation: u64,
        sources: &[crate::musical_performance_source_score::RetainedScoreSource<'_>],
        manifest: &Value,
        pages: &mut impl crate::musical_performance_source_score::NativeScorePages,
    ) -> Result<Self, String> {
        if self.native_custody.is_none()
            || self.generation()? != expected_generation
            || manifest["expression_ref"] != self.expression_ref
            || manifest["performance"]["performance_ref"] != self.performance_ref
        {
            return Err("stale or foreign complete native score edit".into());
        }
        crate::musical_performance_source_score::compile_retained_score(
            expected_generation
                .checked_add(1)
                .ok_or("score edition generation exhausted")?,
            sources,
            manifest,
            pages,
        )
    }
}
pub fn compile_score(
    expression_ref: &str,
    edition_generation: u64,
    sources: &[ScoreSource<'_>],
    performance: &Value,
) -> Result<MusicalPerformanceScore, String> {
    compile_inner(
        expression_ref,
        edition_generation,
        sources,
        performance,
        None,
        None,
    )
}
pub(crate) fn compile_current_native_score(
    expression_ref: &str,
    edition_generation: u64,
    sources: &[ScoreSource<'_>],
    source_basis_indices: &[usize],
    performance: &Value,
    custody: crate::musical_performance_source_score::NativeScoreCustody,
) -> Result<MusicalPerformanceScore, String> {
    compile_inner(
        expression_ref,
        edition_generation,
        sources,
        performance,
        Some(custody),
        Some(source_basis_indices),
    )
}
fn compile_inner(
    expression_ref: &str,
    edition_generation: u64,
    sources: &[ScoreSource<'_>],
    performance: &Value,
    native_custody: Option<crate::musical_performance_source_score::NativeScoreCustody>,
    source_basis_indices: Option<&[usize]>,
) -> Result<MusicalPerformanceScore, String> {
    reference(&json!(expression_ref))?;
    let schema = performance["schema"].as_str().unwrap_or("");
    if edition_generation == 0
        || if native_custody.is_some() {
            ![
                "oi.expression-performance/v1",
                "oi.expression-performance/v2",
                "oi.expression-performance/v3",
            ]
            .contains(&schema)
        } else {
            schema != "oi.expression-performance/v1"
        }
    {
        return Err("native score/Expression contract mismatch".into());
    }
    let performance_ref = reference(&performance["performance_ref"])?;
    let expression_performance_revision = reference(&performance["content_digest"])?;
    let performance_digest = digest(performance)?;
    let bases = array(&performance["bases"])?;
    let pitches = array(&performance["pitches"])?;
    let layers = array(&performance["layers"])?;
    let rate = performance["sample_rate"]
        .as_u64()
        .ok_or("native audio rate absent")?;
    if !(8000..=192000).contains(&rate)
        || bases.is_empty()
        || bases.len() > MAX_BASES
        || sources.is_empty()
        || sources.len() > MAX_BASES
        || (source_basis_indices.is_none() && sources.len() != bases.len())
        || pitches.len() > MAX_PITCHES
        || layers.is_empty()
        || layers.len() > 64
    {
        return Err("native score source/cardinality/rate bounds differ".into());
    }
    let duration = counter(&performance["duration_samples"])?;
    let ordinary_indices: Vec<_> = (0..bases.len()).collect();
    let indices = source_basis_indices.unwrap_or(&ordinary_indices);
    if indices.len() != sources.len()
        || indices.iter().any(|i| *i >= bases.len())
        || (0..bases.len()).any(|i| !indices.contains(&i))
    {
        return Err("native source epochs lost unique complete musical basis coverage".into());
    }
    let mut source_bases: Vec<Option<Value>> = vec![None; bases.len()];
    let mut original_episode_refs: Vec<Option<Option<String>>> = vec![None; bases.len()];
    for (source, basis_index) in sources.iter().zip(indices) {
        let retained = &bases[*basis_index];
        source.prepared.validate_native_consumers(
            source.prepared.native_basis(),
            source.prepared.physical_body(),
        )?;
        if u64::from(source.prepared.physical_body().request().sample_rate) != rate {
            return Err(
                "native score sample clock differs from actual prepared physical body".into(),
            );
        }
        let actual = source.original_return.expression_basis()?;
        if actual["audio_determination"] != *source.prepared.determination()
            || actual["m1"] != source.prepared.native_basis().m1
            || actual["m2_plan"]
                != serde_json::to_value(source.prepared.relation_plan())
                    .map_err(|e| e.to_string())?
            || actual["m3_score"] != source.prepared.native_basis().m3
            || actual["prepared_body"]
                != serde_json::to_value(source.prepared.physical_body())
                    .map_err(|e| e.to_string())?
        {
            return Err("native score Return detached from actual current preparation".into());
        }
        let mut received = retained.clone();
        reference(&received["content_digest"])?;
        received["content_digest"] = json!("");
        if received != actual {
            return Err(
                "native score lost source form, exact tuning, force or original context".into(),
            );
        }
        let mut output = json!({"basis_digest":retained["content_digest"],"native_identity":actual["identity"],
            "m3_source_score":actual["m3_score"],"m3_source_replay":actual["m3_replay"],
            "original_episode":actual["m4_episode"],"native_return_digest":source.original_return.snapshot()?["content_digest"]});
        match &source.keys {
            ScoreKeys::Architectural => {
                output["key_source"] =
                    json!({"owner":"native-architectural-key-policy","tuning":actual["tuning"]})
            }
            ScoreKeys::Sparse { targets, consumer } => {
                let condition = consumer
                    .condition
                    .as_ref()
                    .map(|c| SparseConditionConsumer {
                        producer_input: c.producer_input,
                        plan: c.plan,
                    });
                targets.validate_coupled_consumer(
                    SparseMusicalConsumer {
                        basis: consumer.basis,
                        writer: consumer.writer,
                        phase: consumer.phase,
                        condition,
                    },
                    targets.collection(),
                )?;
                if consumer.basis.m1 != source.prepared.native_basis().m1
                    || consumer.basis.m2 != source.prepared.native_basis().m2
                    || consumer.basis.m3 != source.prepared.native_basis().m3
                {
                    return Err(
                        "sparse score source detached from current coupled body/form".into(),
                    );
                }
                output["key_source"] = targets.preparation_receipt()?;
            }
        }
        let episode = source
            .original_return
            .original_occasion()
            .map(|o| o.occasion_ref.clone());
        if source_bases[*basis_index]
            .as_ref()
            .is_some_and(|old| *old != output)
            || original_episode_refs[*basis_index]
                .as_ref()
                .is_some_and(|old| *old != episode)
        {
            return Err(
                "same musical basis epochs disagree on original native Return/source/episode"
                    .into(),
            );
        }
        source_bases[*basis_index] = Some(output);
        original_episode_refs[*basis_index] = Some(episode);
    }
    let source_bases = source_bases
        .into_iter()
        .map(|v| v.ok_or_else(|| "original musical basis absent".to_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    let original_episode_refs = original_episode_refs
        .into_iter()
        .map(|v| v.ok_or_else(|| "original musical episode absent".to_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    // All epochs above independently qualified the same immutable musical
    // basis. Musical operands use that basis; complete receiver epochs remain
    // distinct in the native source-part custody, never duplicated bases.
    let by_basis: Vec<&ScoreSource<'_>> = (0..bases.len())
        .map(|basis| {
            sources
                .iter()
                .zip(indices)
                .find(|(_, i)| **i == basis)
                .map(|(source, _)| source)
                .expect("complete musical basis coverage qualified above")
        })
        .collect();
    let pitch_sources = pitches
        .iter()
        .enumerate()
        .map(|(i, p)| qualify_pitch(by_basis[index(&p["basis"], by_basis.len())?], p, i))
        .collect::<Result<Vec<_>, String>>()?;
    let mut pages = Vec::new();
    let mut total = 0usize;
    let mut previous = None;
    let mut sequences = BTreeSet::new();
    let mut touches = BTreeMap::new();
    for page in array(&performance["pages"])? {
        let events = array(&page["events"])?;
        if events.is_empty() || events.len() > MAX_PAGE_EVENTS {
            return Err("native score event page bounds differ".into());
        }
        total = total
            .checked_add(events.len())
            .ok_or("native score event count overflow")?;
        if total > MAX_EVENTS {
            return Err("native score event count exceeds existing Expression allowance".into());
        }
        for event in events {
            let event = array(event)?;
            if event.len() != 5 {
                return Err("native score lost event operands".into());
            }
            let sequence = counter(&event[0])?;
            let sample = counter(&event[1])?;
            let layer = index(&event[2], layers.len())?;
            let basis = index(&event[3], by_basis.len())?;
            if sequence == 0
                || !sequences.insert(sequence)
                || sample > duration
                || previous.is_some_and(|p| (sample, sequence) <= p)
            {
                return Err("native musical event order/identity differs".into());
            }
            previous = Some((sample, sequence));
            qualify_action(
                &event[4],
                layer,
                basis,
                pitches,
                performance,
                &by_basis,
                &mut touches,
            )?;
        }
        let first = array(&events[0])?;
        let last = array(&events[events.len() - 1])?;
        pages.push(ScorePage {
            part_ref: digest(&json!({"kind":"event_page","value":page}))?,
            events: events.len(),
            first_sample: reference(&first[1])?.into(),
            last_sample: reference(&last[1])?.into(),
            first_sequence: reference(&first[0])?.into(),
            last_sequence: reference(&last[0])?.into(),
        });
    }
    let mut controls = performance.clone();
    let object = controls
        .as_object_mut()
        .ok_or("native composition required")?;
    for key in ["bases", "pitches", "pages", "checkpoints", "content_digest"] {
        object.remove(key);
    }
    if native_custody.is_some() {
        // Original native parts stay in C's selected Act. Their exact part
        // identities and stream metadata are retained in native_custody.
        for key in ["native_sources", "native_recordings", "native_reservations"] {
            object.remove(key);
        }
    }
    let mut score = MusicalPerformanceScore {
        schema: if native_custody.is_some() {
            crate::musical_performance_source_score::SCHEMA
        } else {
            SCHEMA
        },
        expression_ref: expression_ref.into(),
        performance_ref: performance_ref.into(),
        edition_generation: edition_generation.to_string(),
        performance_digest,
        expression_performance_revision: expression_performance_revision.into(),
        source_bases,
        pitch_sources,
        pages,
        event_count: total,
        controls,
        original_episode_refs,
        native_custody,
        content_digest: String::new(),
    };
    score.content_digest = digest(&score.snapshot()?)?;
    Ok(score)
}
fn qualify_action(
    action: &Value,
    layer: usize,
    basis: usize,
    pitches: &[Value],
    performance: &Value,
    sources: &[&ScoreSource<'_>],
    touches: &mut BTreeMap<u64, (usize, usize)>,
) -> Result<(), String> {
    let rate = performance["sample_rate"]
        .as_u64()
        .ok_or("native audio rate absent")?;
    let (tag, args) = if let Some(tag) = action.as_str() {
        (tag, None)
    } else {
        let object = action.as_object().ok_or("native score action required")?;
        if object.len() != 1 {
            return Err("unknown or mixed native musical action".into());
        }
        let (tag, value) = object.iter().next().ok_or("native action absent")?;
        (tag.as_str(), Some(value))
    };
    let list = |n| -> Result<&Vec<Value>, String> {
        let a = array(args.ok_or("native action operands absent")?)?;
        if a.len() != n {
            return Err("native musical action lost operands".into());
        }
        Ok(a)
    };
    match tag {
        "n" => {
            let a = list(6)?;
            let touch = counter(&a[0])?;
            let member = counter(&a[1])?;
            let pitch = index(&a[2], pitches.len())?;
            if touch == 0
                || member == 0
                || pitches[pitch]["basis"].as_u64() != Some(basis as u64)
                || touches.insert(touch, (layer, basis)).is_some()
            {
                return Err("native source note lifetime/basis differs".into());
            }
            number(&a[3], 0.0, 1.0)?;
            let s = number(&a[4], -1.0, 1.0)?;
            let c = number(&a[5], -1.0, 1.0)?;
            if (s * s + c * c - 1.0).abs() > 1e-10 {
                return Err("native score phase quadrature differs".into());
            }
            let source = &sources[basis];
            let config = &source.prepared.native_basis().input.m1;
            let carrier = crate::m1_engine::carrier(
                config
                    .cycle
                    .parse()
                    .map_err(|_| "invalid native M1 cycle")?,
                config.tick12,
            )?;
            let phase = if source.prepared.determination()["m1_face"] == 1 {
                carrier.opposite_quadrature
            } else {
                carrier.quadrature
            };
            if !exact(s, phase[1]) || !exact(c, phase[0]) {
                return Err("musical note phase detached from actual M1 carrier".into());
            }
        }
        "o" => {
            let touch = counter(args.ok_or("original touch release absent")?)?;
            if touches.remove(&touch) != Some((layer, basis)) {
                return Err("score release lost original note/source lifetime".into());
            }
        }
        "e" => {
            let a = list(3)?;
            if touches.get(&counter(&a[0])?) != Some(&(layer, basis)) {
                return Err("score expression lost original touch".into());
            }
            number(&a[1], 0.0, 1.0)?;
            number(&a[2], 0.001, rate as f64 * 0.45)?;
        }
        "s" => {
            args.and_then(Value::as_bool)
                .ok_or("native pedal state absent")?;
        }
        "p" | "a" => {
            let a = list(3)?;
            let key = if tag == "p" { "parameters" } else { "routes" };
            let targets = array(&performance[key])?;
            index(&a[0], targets.len())?;
            number(&a[1], -f64::MAX, f64::MAX)?;
            if !a[2].is_null() && touches.get(&counter(&a[2])?) != Some(&(layer, basis)) {
                return Err("note modulation lost original touch".into());
            }
        }
        "k" => {
            crate::musical_performance_source_score::qualify_recorded_contact_action(
                args.ok_or("complete original Contact action absent")?,
                &performance["bases"][basis],
                performance,
            )?;
        }
        "f" => {
            let a = list(3)?;
            let body = &performance["bases"][basis]["prepared_body"];
            if a[0] != body["request"]["preparation_ref"] {
                return Err("source score force lost actual body".into());
            }
            let force = array(&a[1])?;
            if force.len() != 3 {
                return Err("Newton force vector absent".into());
            }
            let mut norm = 0.0;
            for component in force {
                norm += number(component, -f64::MAX, f64::MAX)?.powi(2)
            }
            let max = number(
                &body["request"]["max_force_newtons"],
                f64::MIN_POSITIVE,
                f64::MAX,
            )?;
            if norm.sqrt() > max {
                return Err("source score force exceeds prepared bound".into());
            }
            reference(&a[2]["ref"])?;
            reference(&a[2]["revision"])?;
        }
        "b" | "c" => {
            if args.is_some() || touches.values().any(|(l, _)| *l == layer) {
                return Err(
                    "raw context/basis transfer needs explicit native held-state operation".into(),
                );
            }
        }
        "x" => {
            if args.is_some() {
                return Err("panic operands differ".into());
            }
            touches.retain(|_, (l, _)| *l != layer);
        }
        _ => return Err("unsupported source musical action".into()),
    }
    Ok(())
}
