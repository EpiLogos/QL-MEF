//! Source-qualified score of the selected complete native Act performance.
//! C owns page decoding, original application validation and Act/Scene CAS.
//! This compiler checks their exact retained bytes and current native source,
//! and returns a prospective inscription. It grants no lease or playback.
use crate::continuous::coupled::CoupledBasis;
use crate::continuous::performance::PerformanceOwner;
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use crate::musical_performance_return::MusicalPerformanceReturn;
use crate::musical_performance_score::{MusicalPerformanceScore, ScoreKeys, ScoreSource};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "ql.musical-performance-score/v2";
const PART_BYTES: usize = 4 * 1024 * 1024;
const MAX_PAGES: usize = 4096;

/// Implemented by the existing C Act delivery owner. Read each original page
/// under the SAME selection. No QL codec, persistent score store or clock.
pub trait NativeScorePages {
    fn read_page(&mut self, index: usize) -> Result<Value, String>;
}

/// The real native owners remain borrowed throughout compilation. Serialized
/// source inputs or witnesses cannot deserialize this currentness binding.
pub struct RetainedScoreSource<'a> {
    pub owner: &'a PerformanceOwner,
    pub current: &'a CoupledBasis,
    pub receiving: &'a NativePerformanceReceivingSource,
    pub original_return: &'a MusicalPerformanceReturn,
    pub source_sample: u64,
}
impl RetainedScoreSource<'_> {
    fn source(&self) -> Result<ScoreSource<'_>, String> {
        let current =
            self.receiving
                .prepare_current(self.owner, self.current, self.source_sample)?;
        if current.snapshot()? != self.owner.source_assets()["current_receiving"] {
            return Err("native score receiving payload is stale or disconnected".into());
        }
        let keys = match self.owner.source_key_consumer(self.current)? {
            Some((targets, consumer)) => ScoreKeys::Sparse { targets, consumer },
            None => ScoreKeys::Architectural,
        };
        Ok(ScoreSource {
            prepared: self.owner.binding(),
            original_return: self.original_return,
            keys,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NativePageScore {
    pub part: Value,
    pub decoded_sha256: String,
    pub decoded_length: String,
    pub kind: String,
    pub applications: usize,
    pub original_inputs: usize,
    pub unknown_requested_times: usize,
    pub first_applied_ordinal: Option<String>,
    pub last_applied_ordinal: Option<String>,
    pub transport_epoch: Option<String>,
    /// Derived from the original C-qualified continued page. These are stream
    /// high-water ordinals, never new applications or rewritten admission IDs.
    pub restored_from_epoch: Option<String>,
    pub restored_applied_high_water: Option<String>,
    pub restored_input_high_water: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NativeScoreCustody {
    pub selection: Value,
    pub selected_performance_sha256: String,
    pub source_parts: Vec<Value>,
    pub recording_pages: Vec<NativePageScore>,
    pub pending_reservations: Value,
    pub checkpoints: Vec<Value>,
    pub original_episodes: Value,
}
fn array(value: &Value) -> Result<&[Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "native score array absent".into())
}
fn counter(value: &Value) -> Result<u64, String> {
    let text = value
        .as_str()
        .ok_or("native canonical sample/ordinal absent")?;
    let out = text.parse::<u64>().map_err(|e| e.to_string())?;
    if out.to_string() != text {
        return Err("native noncanonical sample/ordinal".into());
    }
    Ok(out)
}
fn sha(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn canonical_part(
    witness: &Value,
    kind: &str,
    revision: &str,
    value: &Value,
) -> Result<(), String> {
    let bytes = witness["canonical_part_bytes"]
        .as_str()
        .ok_or("original native part bytes absent")?;
    // Exact original typed bytes, before Value can reorder native fields.
    if bytes.is_empty()
        || bytes.len() > 8 * 1024 * 1024
        || witness["reading"]["availability"] != "available"
        || witness["reading"]["revision"] != revision
        || witness["reading"]["ref"] != sha(bytes.as_bytes())
    {
        return Err("native part bytes/identity differ".into());
    }
    let part: Value = serde_json::from_str(bytes).map_err(|e| e.to_string())?;
    if part.as_object().is_none_or(|v| v.len() != 2)
        || part["kind"] != kind
        || part["value"] != *value
    {
        return Err("native part lost original selected value".into());
    }
    Ok(())
}
fn selection(manifest: &Value) -> Value {
    json!({"act_ref":manifest["act_ref"],"act_revision":manifest["act_revision"],
        "act_digest":manifest["act_digest"],"edition_position":manifest["edition_position"],
        "expanded_document_sha256":manifest["expanded_document_sha256"],
        "scene_ref":manifest["scene_ref"],"scene_revision":manifest["scene_revision"],
        "performance_digest":manifest["performance_digest"]})
}
fn page_score(
    manifest: &Value,
    sources: &[RetainedScoreSource<'_>],
    witness: &Value,
    index: usize,
    page: &Value,
    streams: &mut BTreeMap<u64, (u64, u64)>,
) -> Result<NativePageScore, String> {
    if page["schema"] != "oi.expression-native-page-delivery/v1"
        || selection(page) != selection(manifest)
        || page["witness"] != *witness
        || witness["page_index"].as_u64() != Some(index as u64)
        || page["original_encoded_page"] != manifest["performance"]["native_recordings"][index]
    {
        return Err("native page changed selected Act/Scene/edition/performance".into());
    }
    canonical_part(
        witness,
        "native_recording",
        "oi.expression-native-recording-page/v1",
        &page["original_encoded_page"],
    )?;
    let bytes = page["canonical_decoded_bytes"]
        .as_str()
        .ok_or("original native decoded bytes absent")?;
    if bytes.is_empty()
        || bytes.len() > PART_BYTES
        || counter(&witness["decoded_length"])? != bytes.len() as u64
        || witness["decoded_sha256"] != sha(bytes.as_bytes())
        || serde_json::from_str::<Value>(bytes).map_err(|e| e.to_string())? != page["decoded"]
    {
        return Err("native recording page lost exact canonical decoded bytes".into());
    }
    let decoded = &page["decoded"];
    let kind = decoded["kind"]
        .as_str()
        .ok_or("native recording kind absent")?;
    let mut out = NativePageScore {
        part: witness["reading"].clone(),
        decoded_sha256: witness["decoded_sha256"]
            .as_str()
            .ok_or("decoded SHA absent")?
            .into(),
        decoded_length: witness["decoded_length"]
            .as_str()
            .ok_or("decoded length absent")?
            .into(),
        kind: kind.into(),
        applications: 0,
        original_inputs: 0,
        unknown_requested_times: 0,
        first_applied_ordinal: None,
        last_applied_ordinal: None,
        transport_epoch: None,
        restored_from_epoch: None,
        restored_applied_high_water: None,
        restored_input_high_water: None,
    };
    if kind == "terminated" {
        // Original before/after checkpoints, reservations and actual reason
        // remain in this exact C-validated page part, never relabelled played.
        if !decoded["value"].is_object() {
            return Err("native reservation termination absent".into());
        }
        return Ok(out);
    }
    if kind == "continued" {
        // C's complete selected Performance has already validated the exact
        // proof, saved queues, full restitution and observer FIFO drain. Keep
        // the original encoded part/decoded bytes above; QL owns no C codec or
        // continuation fingerprint. Register only its original stream start.
        let proof = &decoded["value"];
        if proof["schema"] != "oi.expression-native-reservation-continuation/v1" {
            return Err("original native reservation continuation absent".into());
        }
        let saved_epoch = counter(&proof["saved"]["management"]["transport_epoch"])?;
        let epoch = counter(&proof["transport_ack"]["epoch"])?;
        let applied = counter(&proof["saved"]["audio"]["applied_application_ordinal"])?;
        let input = counter(&proof["saved"]["management"]["input_history"]["last_ordinal"])?;
        if saved_epoch == 0
            || epoch <= saved_epoch
            || streams.insert(epoch, (applied, input)).is_some()
        {
            return Err("native continuation stream epoch duplicated or disconnected".into());
        }
        out.transport_epoch = Some(epoch.to_string());
        out.restored_from_epoch = Some(saved_epoch.to_string());
        out.restored_applied_high_water = Some(applied.to_string());
        out.restored_input_high_water = Some(input.to_string());
        return Ok(out);
    }
    let batch = &decoded["value"];
    if kind != "applied" || batch["schema"] != "oi.expression-native-recording-batch/v1" {
        return Err("unknown native recording content".into());
    }
    let basis_index = usize::try_from(
        batch["basis"]
            .as_u64()
            .ok_or("recorded native basis absent")?,
    )
    .map_err(|e| e.to_string())?;
    let basis = array(&manifest["performance"]["bases"])?
        .get(basis_index)
        .ok_or("recorded native basis lost")?;
    let state = &batch["state"];
    let epoch = counter(&state["transport_epoch"])?;
    let mut ordinal = counter(&batch["previous_applied_application_ordinal"])?;
    let mut input_ordinal = counter(&batch["previous_input_ordinal"])?;
    if epoch == 0
        || state["basis_digest"] != basis["content_digest"]
        || state["identity"] != basis["identity"]
        || state["recording"]["failure"] != 0
        || counter(&state["recording"]["dropped_applications"])? != 0
        || streams
            .get(&epoch)
            .is_some_and(|old| *old != (ordinal, input_ordinal))
    {
        return Err("native score lost original source/stream or retained recording loss".into());
    }
    let receipts = array(&batch["receipts"])?;
    let journal = array(&batch["input_journal"])?;
    if receipts.len() > 256 || journal.len() > 256 {
        return Err("original native page bound exceeded".into());
    }
    for receipt in receipts {
        let app = &receipt["application"];
        ordinal = ordinal
            .checked_add(1)
            .ok_or("native application ordinal exhausted")?;
        let admitted = counter(&app["admitted_sample"])?;
        let applied = counter(&app["applied_sample"])?;
        if app["schema"] != "ql.performance-applied-event/v2"
            || counter(&app["applied_application_ordinal"])? != ordinal
            || counter(&app["sequence"])? == 0
            || applied < admitted
            || applied >= counter(&app["committed_cursor"])?
            || app["identity"] != basis["audio_determination"]["identity"]
            || app["body_revision"] != basis["audio_determination"]["body_revision"]
            || app["preparation_ref"] != basis["prepared_body"]["request"]["preparation_ref"]
            || app["state_ref"] != basis["prepared_body"]["request"]["state_ref"]
        {
            return Err("native score lost original applied source/body/time/ordinal".into());
        }
        match app.get("requested_sample") {
            Some(value) => {
                let requested = counter(value)?;
                if requested > admitted || (requested < admitted && app["late_admitted"] != true) {
                    return Err("native score lost original requested timing".into());
                }
            }
            None => out.unknown_requested_times += 1,
        }
        qualify_original_input(sources, manifest, receipt, journal)?;
        if out.first_applied_ordinal.is_none() {
            out.first_applied_ordinal = Some(ordinal.to_string());
        }
        out.last_applied_ordinal = Some(ordinal.to_string());
    }
    for entry in journal {
        input_ordinal = input_ordinal
            .checked_add(1)
            .ok_or("native input ordinal exhausted")?;
        if counter(&entry["ordinal"])? != input_ordinal {
            return Err("native score lost original input journal".into());
        }
    }
    if ordinal != counter(&state["applied_high_water"])? {
        return Err("native score lost trailing native application".into());
    }
    streams.insert(epoch, (ordinal, input_ordinal));
    out.applications = receipts.len();
    out.original_inputs = journal.len();
    out.transport_epoch = Some(epoch.to_string());
    Ok(out)
}

fn qualify_original_input(
    sources: &[RetainedScoreSource<'_>],
    manifest: &Value,
    receipt: &Value,
    journal: &[Value],
) -> Result<(), String> {
    let app = &receipt["application"];
    let kind = app["kind"]
        .as_u64()
        .ok_or("original native operation kind absent")?;
    let applied = app["applied"]
        .as_bool()
        .ok_or("native application status absent")?;
    if ![0, 1, 3].contains(&kind) {
        if !receipt["original_input"].is_null() {
            return Err("non-touch native application borrowed an original input".into());
        }
        return Ok(());
    }
    let matches = journal
        .iter()
        .filter(|entry| {
            entry["native_sequence"] == app["sequence"]
                && entry["change"] == if applied { 2 } else { 3 }
                && entry["operation"] == kind
        })
        .collect::<Vec<_>>();
    if matches.is_empty() && !applied && receipt["original_input"].is_null() {
        return Ok(());
    }
    if matches.len() != 1 || receipt["original_input"] != *matches[0] {
        return Err("native score lost the exact original applied input journal entry".into());
    }
    let original = matches[0];
    if original["input_ref"].as_str().is_none_or(str::is_empty)
        || counter(&original["target"]["touch"])? == 0
        || (kind != 0 && original["target"]["touch"] != app["touch"])
        || (app["has_note"] == true && app["note"] != original["target"])
    {
        return Err("native score original input/touch differs from its application".into());
    }
    if applied {
        // A held input can still belong to an earlier retained source basis.
        // Try only the complete actual retained owners, never an inferred or
        // deserialized pitch policy, and regenerate the original native wire.
        let target = &original["target"];
        let actual = sources
            .iter()
            .find_map(|source| {
                source
                    .owner
                    .qualify_recorded_note(source.current, target, target)
                    .ok()
            })
            .ok_or("played native target has no actual retained K/B/source producer")?;
        if kind == 0 && !receipt["performed_event"].is_null() {
            let event = array(&receipt["performed_event"])?;
            if event.len() != 5 {
                return Err("original performed note event lost operands".into());
            }
            let args = array(&event[4]["n"])?;
            if args.len() != 6 {
                return Err("original performed note lost native note operands".into());
            }
            let index = usize::try_from(args[2].as_u64().ok_or("performed source pitch absent")?)
                .map_err(|e| e.to_string())?;
            let pitch = array(&manifest["performance"]["pitches"])?
                .get(index)
                .ok_or("original performed source pitch lost")?;
            let ratio = if actual["exact_ratio"] == true {
                json!({"numerator":actual["ratio_numerator"],"denominator":actual["ratio_denominator"]})
            } else {
                Value::Null
            };
            if args[0] != actual["touch"]
                || args[1] != actual["member"]
                || pitch["basis"] != event[3]
                || pitch["key"] != actual["key"]
                || pitch["register"] != actual["register_octave"]
                || pitch["source_coordinate"] != actual["source_coordinate"]
                || pitch["source_prime"] != json!(actual["source_face"] == 1)
                || pitch["pitch_class"] != actual["pitch_class"]
                || pitch["hertz"] != actual["hertz"]
                || pitch["fundamental_hz"] != actual["fundamental_hz"]
                || pitch["tuning_ref"] != actual["tuning_ref"]
                || pitch["exact_ratio"] != ratio
                || args[4] != actual["phase_sin"]
                || args[5] != actual["phase_cos"]
            {
                return Err(
                    "native played note projection detached from its complete original source"
                        .into(),
                );
            }
        }
    }
    Ok(())
}

/// Read ALL original pages; compilation aborts on a changed selection or lost
/// producer. C commits the returned score through the same Scene/Act CAS only
/// after the current source/worker/lease transaction independently succeeds.
pub fn compile_retained_score(
    edition_generation: u64,
    sources: &[RetainedScoreSource<'_>],
    manifest: &Value,
    pages: &mut impl NativeScorePages,
) -> Result<MusicalPerformanceScore, String> {
    let performance = &manifest["performance"];
    let original_bytes = manifest["canonical_performance_bytes"]
        .as_str()
        .ok_or("complete original typed native performance bytes absent")?;
    if original_bytes.is_empty()
        || original_bytes.len() > 8 * 1024 * 1024
        || manifest["selected_performance_sha256"] != sha(original_bytes.as_bytes())
        || serde_json::from_str::<Value>(original_bytes).map_err(|e| e.to_string())? != *performance
    {
        return Err(
            "selected native performance lost complete original bytes or recording prefix".into(),
        );
    }
    let prepared = sources
        .iter()
        .map(RetainedScoreSource::source)
        .collect::<Result<Vec<_>, _>>()?;
    if manifest["schema"] != "oi.expression-performance-delivery/v1"
        || manifest["performance_digest"] != performance["content_digest"]
        || ![
            "oi.expression-performance/v1",
            "oi.expression-performance/v2",
            "oi.expression-performance/v3",
        ]
        .contains(&performance["schema"].as_str().unwrap_or(""))
    {
        return Err("native selected score delivery contract differs".into());
    }
    let bases = array(&performance["bases"])?;
    let assets = array(&performance["native_sources"])?;
    let witnesses = array(&manifest["native_source_parts"])?;
    if bases.is_empty()
        || bases.len() > 256
        || bases.len() != sources.len()
        || assets.len() != sources.len()
        || witnesses.len() != assets.len()
    {
        return Err("native score lost a complete source asset".into());
    }
    let mut source_parts = Vec::new();
    let mut source_bases = BTreeSet::new();
    for (index, (asset, witness)) in assets.iter().zip(witnesses).enumerate() {
        canonical_part(
            witness,
            "native_source",
            "oi.expression-performance-source-asset/v1",
            asset,
        )?;
        let matching = bases
            .iter()
            .enumerate()
            .filter(|(_, basis)| basis["content_digest"] == asset["basis_digest"])
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err("native score asset basis missing or ambiguous".into());
        }
        let basis_index = matching[0];
        let source = &sources[basis_index];
        if witness["source_index"].as_u64() != Some(index as u64)
            || asset["schema"] != "oi.expression-performance-source-asset/v1"
            || !source_bases.insert(basis_index)
            || asset["identity"] != bases[basis_index]["identity"]
            || asset["context"] != bases[basis_index]["context"]
            || asset["native_bundle"] != *source.owner.source_assets()
        {
            return Err("native score asset detached from actual current native producer".into());
        }
        source_parts
            .push(json!({"basis":basis_index,"source_index":index,"reading":witness["reading"]}));
    }
    let episodes = array(&manifest["original_episodes"])?;
    if episodes.len() != bases.len() {
        return Err("native score lost original episode return".into());
    }
    for (index, (episode, basis)) in episodes.iter().zip(bases).enumerate() {
        if *episode
            != json!({"basis_index":index,"basis_digest":basis["content_digest"],
            "identity":basis["identity"],"context":basis["context"],"original_episode":basis["m4_episode"]})
        {
            return Err("native score episode/identity/context transferred or rewritten".into());
        }
    }
    let witnesses = array(&manifest["native_recording_parts"])?;
    // The original C v2 serde contract omits empty successor collections.
    // Preserve that absence; it does not imply any recorded application.
    let recordings = match performance.get("native_recordings") {
        None => &[][..],
        Some(value) => array(value)?,
    };
    if witnesses.len() > MAX_PAGES || witnesses.len() != recordings.len() {
        return Err("native score lost original recording pages".into());
    }
    let mut streams = BTreeMap::new();
    let mut recording_pages = Vec::with_capacity(witnesses.len());
    for (index, witness) in witnesses.iter().enumerate() {
        recording_pages.push(page_score(
            manifest,
            sources,
            witness,
            index,
            &pages.read_page(index)?,
            &mut streams,
        )?);
    }
    let checkpoints = array(&performance["checkpoints"])?
        .iter()
        .enumerate()
        .map(|(index, value)| {
            json!({"index":index,"checkpoint_ref":value["checkpoint_ref"],
            "content_digest":value["content_digest"],"sample":value["sample"]})
        })
        .collect();
    let custody = NativeScoreCustody {
        selection: selection(manifest),
        selected_performance_sha256: manifest["selected_performance_sha256"]
            .as_str()
            .ok_or("complete selected native SHA absent")?
            .into(),
        source_parts,
        recording_pages,
        pending_reservations: performance
            .get("native_reservations")
            .cloned()
            .unwrap_or_else(|| json!([])),
        checkpoints,
        original_episodes: manifest["original_episodes"].clone(),
    };
    crate::musical_performance_score::compile_current_native_score(
        manifest["expression_ref"]
            .as_str()
            .ok_or("native Expression reference absent")?,
        edition_generation,
        &prepared,
        performance,
        custody,
    )
}
