//! Dedicated selected-Act operation on the native Manager-owned control pipe.
//! The normal public HostOperation/Exchange surface cannot supply this request.
//! C keeps its native Act reader and pre/post store CAS while servicing pulls.
use super::host::{FieldHost, MAX_HOST_INPUT, MAX_HOST_OUTPUT};
use super::native_act_channel::NativeActOperation;
use super::performance_export::NativeActCheckpoints;
use crate::musical_performance_source_score::NativeScorePages;
use serde::Deserialize;
use serde_json::{Value, json};
use std::cell::RefCell;

pub const ACT_REQUEST: &str = "ql.native-act-owner-request/v1";
const QUERY: &str = "ql.native-act-owner-query/v1";
const ANSWER: &str = "oi.native-act-owner-answer/v1";
const CONTROL: &str = "oi.native-act-owner-control/v1";

fn count(value: &Value) -> Result<u64, String> {
    let text = value.as_str().ok_or("exact native Act counter absent")?;
    let number: u64 = text.parse().map_err(|_| "native Act counter invalid")?;
    if number.to_string() != text {
        return Err("noncanonical native Act counter".into());
    }
    Ok(number)
}
fn bounded(value: &Value, limit: u64) -> Result<(), String> {
    if serde_json::to_vec(value).map_err(|e| e.to_string())?.len() as u64 > limit {
        return Err("native selected Act control payload exceeds existing transport bound".into());
    }
    Ok(())
}

// The real native C WAV consumer renders complete 512-frame blocks and its
// final short block. Its finite command count comes from the admitted interval,
// independently of the unchanged source-page/diagnostic-frame bounds. This
// tracks transport progress only; native committed PCM/source receipts remain
// the renderer authority and are validated separately.
const NATIVE_PCM_BLOCK: u64 = 512;
enum NativeRenderCommand {
    Render(u32),
    Finished,
    Abort,
}
struct NativeRenderCommands {
    remaining_samples: u64,
    remaining_messages: u64,
}
impl NativeRenderCommands {
    fn new(admitted_frames: u64) -> Result<Self, String> {
        if admitted_frames == 0 {
            return Err("native render interval is empty".into());
        }
        Ok(Self {
            remaining_samples: admitted_frames,
            remaining_messages: admitted_frames
                .div_ceil(NATIVE_PCM_BLOCK)
                .checked_add(1)
                .ok_or("native render command count exhausted")?,
        })
    }
    fn messages(&self) -> u64 {
        self.remaining_messages
    }
    fn next(&mut self, control: &Value) -> Result<NativeRenderCommand, String> {
        if self.remaining_messages == 0 {
            return Err("native render exceeded the admitted command count".into());
        }
        self.remaining_messages -= 1;
        match control["operation"].as_str() {
            Some("render") => {
                if control.as_object().map(|v| v.len()) != Some(5) {
                    return Err("native render control contains unowned fields".into());
                }
                let frames = control["frames"]
                    .as_u64()
                    .ok_or("native render frames absent")?;
                if frames == 0 || frames != self.remaining_samples.min(NATIVE_PCM_BLOCK) {
                    return Err(
                        "native render block differs from the admitted complete PCM interval"
                            .into(),
                    );
                }
                self.remaining_samples -= frames;
                Ok(NativeRenderCommand::Render(frames as u32))
            }
            Some("finished") => {
                if control.as_object().map(|v| v.len()) != Some(4) || self.remaining_samples != 0 {
                    return Err(
                        "native render finished before its complete admitted PCM interval".into(),
                    );
                }
                self.remaining_messages = 0;
                Ok(NativeRenderCommand::Finished)
            }
            Some("abort") => {
                if control.as_object().map(|v| v.len()) != Some(4) {
                    return Err("native render abort contains unowned fields".into());
                }
                self.remaining_messages = 0;
                Ok(NativeRenderCommand::Abort)
            }
            _ => Err("unknown native selected Act render consumer operation".into()),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActRequest {
    schema: String,
    pub(crate) instance_ref: String,
    pub(crate) event_ref: String,
    pub(crate) subject_ref: String,
    pub(crate) request_id: String,
    manager_lease: String,
    #[serde(default)]
    edition_generation: String,
    mode: String,
    #[serde(default)]
    from_sample: String,
    #[serde(default)]
    to_sample: String,
    manifest: Value,
    #[serde(default)]
    checkpoint_index: Option<usize>,
    #[serde(default)]
    transaction_ref: Option<String>,
    #[serde(default)]
    field_registration: Option<Value>,
    #[serde(default)]
    source_bootstrap: Option<Value>,
    #[serde(default)]
    procedural_request: Option<Value>,
    #[serde(default)]
    declared_seed: Option<String>,
    #[serde(default)]
    acoustic_configuration: Option<super::performance::AcousticConfiguration>,
    #[serde(default)]
    original_acoustic_boundary: Option<Value>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativePerformanceTimingQuery {
    schema: String,
    binding: crate::procedural_composition::TimingBinding,
    moment: String,
    ordinal: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeRecordingCommandQuery {
    schema: String,
    scene_constructor: Value,
    command: super::performance::PerformanceCommand,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeRecordingOriginQuery {
    schema: String,
    scene_constructor: Value,
}

impl ActRequest {
    pub(crate) fn is_field(&self) -> bool {
        matches!(
            self.mode.as_str(),
            "field-source"
                | "field-descriptor"
                | "source-bootstrap"
                | "source-lifecycle"
                | "acoustic-install"
                | "acoustic-stage"
                | "acoustic-replace"
                | "acoustic-prepare"
                | "performance-source"
                | "performance-descriptor"
                | "performance-timing"
                | "recording-command"
                | "recording-origin"
                | "recording-save-cut"
        )
    }
}

/// Minted only inside this dedicated operation after the native full-source
/// score compiler accepts the complete C-owned selection. No Deserialize,
/// Clone or public constructor can turn retained JSON into a live Act lease.
enum NativeActSource<'a> {
    Performance {
        checkpoint: Option<&'a Value>,
    },
    Field {
        registration: &'a Value,
        retained_source: Option<&'a Value>,
        source_read: Option<&'a Value>,
        world_carrier: &'a Value,
    },
}
pub struct NativeActSourceLease<'a> {
    manager_lease: &'a str,
    request_id: u64,
    manifest: &'a Value,
    parent_qualification: &'a Value,
    source: NativeActSource<'a>,
}
impl NativeActSourceLease<'_> {
    fn field_registration(&self) -> Option<&Value> {
        match self.source {
            NativeActSource::Field { registration, .. } => Some(registration),
            NativeActSource::Performance { .. } => None,
        }
    }
    fn field_world_carrier(&self) -> Result<&Value, String> {
        match self.source {
            NativeActSource::Field { world_carrier, .. } => Ok(world_carrier),
            NativeActSource::Performance { .. } => {
                Err("private native FIELD World carrier absent".into())
            }
        }
    }
    fn retained_field_source(&self) -> Option<&Value> {
        match self.source {
            NativeActSource::Field {
                retained_source, ..
            } => retained_source,
            NativeActSource::Performance { .. } => None,
        }
    }
    pub fn validate_source_assets(
        &self,
        instance_ref: &str,
        actual_source_assets: &Value,
    ) -> Result<(), String> {
        let performance = if self.field_registration().is_some() {
            &self.manifest["scene"]["performance"]
        } else {
            &self.manifest["performance"]
        };
        let sources = performance["native_sources"]
            .as_array()
            .ok_or("native selected source array absent")?;
        if sources.len() != 1
            || sources[0]["identity"]["instance_ref"] != instance_ref
            || sources[0]["native_bundle"] != *actual_source_assets
        {
            return Err("native Act lease has a different full current source owner".into());
        }
        Ok(())
    }
    /// The actual coordinate owner privately issued this whole read while C
    /// held the SAME current Document/Scene. Channel image/peer qualification,
    /// not retained hashes or caller JSON, established that producer origin.
    pub(crate) fn validate_procedural_scene_read(
        &self,
        instance_ref: &str,
        reading: &crate::procedural_source::NativeBootstrapSceneRead,
        contributors: &[crate::procedural_manifestation::NativeSubject],
    ) -> Result<(), String> {
        reading.validate()?;
        validate_closed_scene_manifest(self.manifest)?;
        let issued = match self.source {
            NativeActSource::Field {
                source_read: Some(issued),
                ..
            } => issued,
            _ => return Err("private selected Scene source issuer absent".into()),
        };
        if issued.as_object().map(|o| o.len()) != Some(2)
            || issued["reading"] != serde_json::to_value(reading).map_err(|e| e.to_string())?
        {
            return Err("procedural Scene read differs from complete actual native issuer".into());
        }
        let receipt = &issued["issuer_receipt"];
        let intent = &receipt["original_intent"];
        let coordinate = &receipt["coordinate"];
        let binding = &coordinate["binding"];
        let subject = &coordinate["subject_binding"];
        let world = &binding["rooted_world"];
        let ground = &intent["authorship"]["ground_ref"];
        let face = match intent["authorship"]["ground_face"].as_str() {
            Some("bimba") => "direct",
            Some("pratibimba") => "conjugate",
            _ => return Err("actual native source has no original authored face".into()),
        };
        let mut material = self.manifest["scene"]["presentation"].clone();
        let material_scene = material["scene"]
            .as_object_mut()
            .ok_or("actual native Scene material absent")?;
        material_scene.remove("procedural");
        let doc_hash = self.manifest["expanded_document_sha256"]
            .as_str()
            .and_then(|v| v.strip_prefix("sha256:"))
            .ok_or("native full Document digest absent")?;
        if receipt.as_object().map(|o| o.len()) != Some(9)
            || receipt["schema"] != "oi.native-selected-scene-source-read/v1"
            || receipt["receipt_ref"] != reading.source_read_receipt_ref
            || receipt["document_fingerprint"] != doc_hash
            || receipt["expression_ref"] != self.manifest["expression_ref"]
            || receipt["scene_ref"] != self.manifest["scene_ref"]
            || receipt["document_revision"] != self.manifest["expression_revision"]
            || receipt["native_identity"]["instance_ref"] != instance_ref
            || reading.expression_ref != self.manifest["expression_ref"]
            || reading.scene_ref != self.manifest["scene_ref"]
            || reading.document_revision != self.manifest["expression_revision"]
            || reading.presentation != material
            || intent["scene_ref"] != reading.scene_ref
            || intent["schema"] != "oi.expression-procedural-source-bootstrap-intent/v1"
            || coordinate["schema"] != "oi.nara-coordinate/v1"
            || binding["schema"] != "ql.coordinate-expression-binding/v1"
            || binding["face"] != intent["authorship"]["ground_face"]
            || subject["native_owner"] != "ql-mef"
            || subject["subject_ref"] != *ground
            || world[face]["canonical_ref"] != *ground
            || subject["sources"][0]["ref"] != *ground
            || subject["sources"][0]["revision"] != world["registry_revision"]
            || subject["sources"][0]
                != serde_json::to_value(&reading.locus).map_err(|e| e.to_string())?
            || subject["readings"][0]["ref"] != reading.source_basis.source_ref
            || subject["readings"][0]["revision"] != reading.source_basis.revision
            || subject["readings"][0]["availability"] != "available"
            || binding["resolved_profile_ref"] != reading.source_basis.source_ref
            || binding["binding_content_revision"] != reading.source_basis.revision
        {
            return Err(
                "original full Document/Scene/coordinate/profile/locus/source owner changed".into(),
            );
        }
        let original: Vec<crate::procedural_manifestation::NativeSubject> =
            serde_json::from_value(intent["authorship"]["contributors"].clone())
                .map_err(|e| e.to_string())?;
        if original != contributors {
            return Err("original native source contributors changed".into());
        }
        for contributor in contributors {
            crate::procedural_manifestation::validate_native_subject_basis(
                crate::m_tree::native_current_m_registry(),
                contributor,
            )?;
        }
        Ok(())
    }
    /// Actual nonsounding Field sources, compared against the FULL closed
    /// Scene's original/current source. This never derives a basis from readback.
    pub(crate) fn validate_field_sources(
        &self,
        instance_ref: &str,
        original: &super::coupled::CoupledBasis,
        current: &super::coupled::CoupledBasis,
    ) -> Result<(), String> {
        if self.field_registration().is_none()
            || !matches!(
                self.manifest["schema"].as_str(),
                Some(
                    "oi.expression-native-scene-delivery/v1"
                        | "oi.expression-native-current-scene-delivery/v1"
                )
            )
        {
            return Err("native Act lease has no privately selected Field Scene".into());
        }
        FieldHost::validate_retained_field_sources_with_artifact(
            &self.field_world_carrier()?["presentation"],
            instance_ref,
            original,
            current,
            self.retained_field_source(),
        )
    }
    /// Construct only from the actual Manager opening registration and guarded
    /// native cursor. No instance/control-generation alias or default epoch.
    pub(crate) fn field_timing_binding(
        &self,
        instance_ref: &str,
        original: &super::FieldInput,
        actual_cursor: u64,
    ) -> Result<crate::procedural_composition::TimingBinding, String> {
        let registration = self
            .field_registration()
            .ok_or("native Field timing was not registered by its actual owner")?;
        FieldHost::validate_retained_field_input_artifact(
            self.retained_field_source()
                .ok_or("complete native FIELD source parts not retained in selected Scene")?,
            original,
        )?;
        let text = |key: &str| -> Result<String, String> {
            registration[key]
                .as_str()
                .filter(|text| {
                    !text.is_empty() && text.len() <= 4096 && !text.chars().any(char::is_control)
                })
                .map(str::to_owned)
                .ok_or_else(|| format!("native Field timing {key} registration absent"))
        };
        if registration.as_object().map(|value| value.len()) != Some(6)
            || registration["schema"] != "oi.native-field-timing-registration/v1"
            || registration["instance_ref"] != instance_ref
            || registration["owner_ref"] != self.manager_lease
            || registration["domain"] != "native_field_samples"
            || !registration["time_mapping_ref"].is_null()
            || registration["epoch_ref"] == instance_ref
            || registration["epoch_ref"] == registration["owner_ref"]
        {
            return Err(
                "native Field timing registration changed its actual owner/domain/epoch/mapping"
                    .into(),
            );
        }
        Ok(crate::procedural_composition::TimingBinding {
            owner_ref: text("owner_ref")?,
            domain: text("domain")?,
            epoch_ref: text("epoch_ref")?,
            requested_cursor: actual_cursor,
            time_mapping_ref: None,
        })
    }
    pub(crate) fn validate_field_timing(
        &self,
        instance_ref: &str,
        binding: &crate::procedural_composition::TimingBinding,
        original: &super::FieldInput,
    ) -> Result<(), String> {
        if *binding
            != self.field_timing_binding(instance_ref, original, binding.requested_cursor)?
        {
            return Err("original native Field binding differs from registered actual owner/domain/epoch/mapping".into());
        }
        Ok(())
    }
    /// Exact checkpoint selected by the same native closed reader. This is
    /// required by the private receiving restore owner before issuing Control.
    pub(crate) fn validate_selected_checkpoint(
        &self,
        instance_ref: &str,
        checkpoint_ref: &str,
        wire: &str,
    ) -> Result<(), String> {
        let selected = match self.source {
            NativeActSource::Performance {
                checkpoint: Some(selected),
            } => selected,
            _ => {
                return Err(
                    "native Act lease has no selected performance checkpoint custody".into(),
                );
            }
        };
        if selected["checkpoint"]["identity"]["instance_ref"] != instance_ref
            || selected["checkpoint"]["checkpoint_ref"] != checkpoint_ref
            || selected["canonical_native_wire_bytes"].as_str() != Some(wire)
        {
            return Err("native checkpoint is not the exact same closed Act selection".into());
        }
        Ok(())
    }
    /// Evidence of the privately admitted operation; this Value cannot mint a lease.
    pub fn evidence(&self) -> Value {
        let mut evidence = json!({"schema":"ql.native-act-source-lease-evidence/v1","manager_lease":self.manager_lease,
            "native_parent_qualification":self.parent_qualification,"native_request_id":self.request_id.to_string(),"act_ref":self.manifest["act_ref"],
            "act_revision":self.manifest["act_revision"],"act_digest":self.manifest["act_digest"],
            "edition_position":self.manifest["edition_position"],"expanded_document_sha256":self.manifest["expanded_document_sha256"],
            "scene_ref":self.manifest["scene_ref"],"scene_revision":self.manifest["scene_revision"],
            "performance_digest":self.manifest["performance_digest"],"selected_performance_sha256":self.manifest["selected_performance_sha256"],
            "source_kind":if self.field_registration().is_some(){"field"}else{"performance"},
            "selected_scene_sha256":self.manifest["selected_scene_sha256"],"field_registration":self.field_registration(),
            "expression_ref":self.manifest["expression_ref"],"expression_revision":self.manifest["expression_revision"],
            "source_custody":if self.manifest["schema"]=="oi.expression-native-current-scene-delivery/v1" {"current-document"} else {"recorded-act"}});
        if self.manifest["schema"] == "oi.expression-native-current-scene-delivery/v1" {
            for key in ["act_ref", "act_revision", "act_digest", "edition_position"] {
                evidence
                    .as_object_mut()
                    .expect("native evidence object")
                    .remove(key);
            }
        }
        evidence
    }
    pub fn manager_lease(&self) -> &str {
        self.manager_lease
    }
    pub fn native_request_id(&self) -> u64 {
        self.request_id
    }
    pub fn act_ref(&self) -> &str {
        self.manifest["act_ref"]
            .as_str()
            .expect("native selected Act ref validated")
    }
    pub fn act_revision(&self) -> &Value {
        &self.manifest["act_revision"]
    }
    pub fn act_digest(&self) -> &str {
        self.manifest["act_digest"]
            .as_str()
            .expect("native selected Act digest validated")
    }
    pub fn edition_position(&self) -> &Value {
        &self.manifest["edition_position"]
    }
    pub fn scene_ref(&self) -> &str {
        self.manifest["scene_ref"]
            .as_str()
            .expect("native selected Scene ref validated")
    }
    pub fn scene_revision(&self) -> &Value {
        &self.manifest["scene_revision"]
    }
    pub fn performance_digest(&self) -> &str {
        self.manifest["performance_digest"]
            .as_str()
            .expect("native selected performance digest validated")
    }
    pub fn native_sources(&self) -> &Value {
        &self.manifest["performance"]["native_sources"]
    }
}

struct Pipe<R, W> {
    read: R,
    write: W,
    instance_ref: String,
    request_id: String,
    query: u64,
}
impl<R: FnMut() -> Result<Value, String>, W: FnMut(&Value) -> Result<(), String>> Pipe<R, W> {
    fn send(&mut self, value: &Value) -> Result<(), String> {
        bounded(value, MAX_HOST_OUTPUT as u64)?;
        (self.write)(value)
    }
    fn receive(&mut self) -> Result<Value, String> {
        let value = (self.read)()?;
        bounded(&value, MAX_HOST_INPUT)?;
        Ok(value)
    }
    fn pull(&mut self, kind: &str, index: usize) -> Result<Value, String> {
        if self.query >= 16384 || index >= 8192 {
            return Err("native Act callback exceeds existing finite part/index custody".into());
        }
        self.query = self
            .query
            .checked_add(1)
            .ok_or("native Act pull ordinal exhausted")?;
        let query = self.query.to_string();
        self.send(
            &json!({"schema":QUERY,"instance_ref":self.instance_ref,"request_id":self.request_id,
            "query_ordinal":query,"kind":kind,"index":index}),
        )?;
        let answer = self.receive()?;
        if answer["schema"] != ANSWER
            || answer["instance_ref"] != self.instance_ref
            || answer["request_id"] != self.request_id
            || answer["query_ordinal"] != query
            || answer["kind"] != kind
            || answer["index"] != index
        {
            return Err(
                "native Act page/checkpoint callback changed operation identity or order".into(),
            );
        }
        if answer["available"] != true {
            return Err(format!(
                "native selected Act callback refused: {}",
                answer["error"]
            ));
        }
        Ok(answer["value"].clone())
    }
    fn control(&mut self) -> Result<Value, String> {
        let value = self.receive()?;
        if value["schema"] != CONTROL
            || value["instance_ref"] != self.instance_ref
            || value["request_id"] != self.request_id
        {
            return Err("native selected Act consumer has foreign control custody".into());
        }
        Ok(value)
    }
}
struct Pages<'a, R, W>(&'a RefCell<Pipe<R, W>>);
impl<R: FnMut() -> Result<Value, String>, W: FnMut(&Value) -> Result<(), String>> NativeScorePages
    for Pages<'_, R, W>
{
    fn read_page(&mut self, index: usize) -> Result<Value, String> {
        self.0.borrow_mut().pull("page", index)
    }
}
impl<R: FnMut() -> Result<Value, String>, W: FnMut(&Value) -> Result<(), String>>
    NativeActCheckpoints for Pages<'_, R, W>
{
    fn read_checkpoint(&mut self, index: usize) -> Result<Value, String> {
        self.0.borrow_mut().pull("checkpoint", index)
    }
}

/// Exact typed C transport from its closed current Document or recorded Act.
/// This comparison supplements the already-qualified private producer channel;
/// it is never a public manifest/hash constructor of source authority.
fn validate_closed_scene_manifest(manifest: &Value) -> Result<(), String> {
    use sha2::{Digest, Sha256};
    let current = manifest["schema"] == "oi.expression-native-current-scene-delivery/v1";
    if !current && manifest["schema"] != "oi.expression-native-scene-delivery/v1" {
        return Err("unsupported closed native Scene source custody".into());
    }
    let mut keys = vec![
        "schema",
        "expression_ref",
        "expression_revision",
        "expanded_document_sha256",
        "canonical_document_bytes",
        "scene_ref",
        "scene_revision",
        "scene",
        "canonical_scene_bytes",
        "selected_scene_sha256",
        "field_source_manifest",
    ];
    if !current {
        keys.extend(["act_ref", "act_revision", "act_digest", "edition_position"]);
    }
    let object = manifest
        .as_object()
        .ok_or("native selected Scene manifest absent")?;
    if object.len() != keys.len() || object.keys().any(|k| !keys.contains(&k.as_str())) {
        return Err("native selected Scene contains foreign custody fields".into());
    }
    for key in ["scene_ref", "expression_ref"] {
        if manifest[key]
            .as_str()
            .is_none_or(|s| s.is_empty() || s.len() > 4096 || s.chars().any(char::is_control))
        {
            return Err("actual native Scene identity absent/unbounded".into());
        }
    }
    if manifest["expression_revision"]
        .as_u64()
        .is_none_or(|n| n == 0)
        || manifest["scene_revision"].as_u64().is_none()
    {
        return Err("actual native Document/Scene revisions absent".into());
    }
    if !current
        && (manifest["act_ref"]
            .as_str()
            .is_none_or(|s| s.is_empty() || s.len() > 4096 || s.chars().any(char::is_control))
            || manifest["act_revision"].as_u64().is_none()
            || manifest["edition_position"].as_u64().is_none())
    {
        return Err("actual recorded Act selection absent".into());
    }
    let doc_text = manifest["canonical_document_bytes"]
        .as_str()
        .ok_or("actual native Document text absent")?;
    let scene_text = manifest["canonical_scene_bytes"]
        .as_str()
        .ok_or("actual native Scene text absent")?;
    if doc_text.len() > 8 * 1024 * 1024 || scene_text.len() > 8 * 1024 * 1024 {
        return Err("native Document/Scene exceeds its existing expanded bound".into());
    }
    let doc: Value = serde_json::from_str(doc_text).map_err(|e| e.to_string())?;
    let scene: Value = serde_json::from_str(scene_text).map_err(|e| e.to_string())?;
    let selected: Vec<_> = doc["scenes"]
        .as_array()
        .ok_or("full native Document scenes absent")?
        .iter()
        .filter(|s| s["scene_ref"] == manifest["scene_ref"])
        .collect();
    if doc["expression_ref"] != manifest["expression_ref"]
        || doc["revision"] != manifest["expression_revision"]
        || selected.len() != 1
        || *selected[0] != scene
        || scene != manifest["scene"]
        || scene["scene_ref"] != manifest["scene_ref"]
        || scene["revision"] != manifest["scene_revision"]
        || manifest["expanded_document_sha256"]
            != format!("sha256:{:x}", Sha256::digest(doc_text.as_bytes()))
        || manifest["selected_scene_sha256"]
            != format!("sha256:{:x}", Sha256::digest(scene_text.as_bytes()))
    {
        return Err("complete original typed native Document/Scene custody changed".into());
    }
    Ok(())
}

// Resolve the sole original World from the SAME fully validated closed
// Document. Selected canonical/generated Scenes retain their own source/CAS;
// they do not acquire copied World material or a caller-provided source asset.
fn closed_field_world_carrier(manifest: &Value) -> Result<Value, String> {
    let document: Value = serde_json::from_str(
        manifest["canonical_document_bytes"]
            .as_str()
            .ok_or("native FIELD full Document bytes absent")?,
    )
    .map_err(|error| error.to_string())?;
    let scenes = document["scenes"]
        .as_array()
        .ok_or("native FIELD full Document Scenes absent")?;
    let mut carriers = scenes
        .iter()
        .filter(|scene| scene["presentation"]["scene"]["epiWorld"].is_object());
    let carrier = carriers
        .next()
        .ok_or("native FIELD original World carrier absent")?;
    if carriers.next().is_some() {
        return Err("native FIELD original World carrier is ambiguous".into());
    }
    Ok(carrier.clone())
}
fn source_header(source: &Value) -> Result<Value, String> {
    let object = source
        .as_object()
        .ok_or("full native FIELD source absent")?;
    let field = source["original_field"]
        .as_object()
        .ok_or("actual original FIELD input absent")?;
    let mut header = object
        .iter()
        .filter(|(key, _)| key.as_str() != "original_field")
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<serde_json::Map<_, _>>();
    header.insert(
        "original_field".into(),
        Value::Object(
            field
                .iter()
                .filter(|(key, _)| key.as_str() != "samples")
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        ),
    );
    Ok(Value::Object(header))
}
fn qualify_field_source_parts<
    R: FnMut() -> Result<Value, String>,
    W: FnMut(&Value) -> Result<(), String>,
>(
    manifest: &Value,
    world_carrier: &Value,
    actual: &Value,
    pipe: &RefCell<Pipe<R, W>>,
) -> Result<(), String> {
    use sha2::{Digest, Sha256};
    let asset = &world_carrier["native_field_source"];
    let source = &manifest["field_source_manifest"];
    let samples = actual["original_field"]["samples"]
        .as_array()
        .ok_or("actual full FIELD samples absent")?;
    let part_count = samples.len().div_ceil(512);
    if samples.is_empty() || samples.len() > 65536 || part_count > 128 {
        return Err("actual FIELD source exceeds its existing native sample bound".into());
    }
    let actual_bytes = serde_json::to_vec(actual).map_err(|e| e.to_string())?;
    if actual_bytes.len() > MAX_HOST_OUTPUT
        || source["schema"] != "oi.expression-native-field-source-manifest/v1"
        || asset["schema"] != "oi.expression-native-field-source/v1"
        || source["instance_ref"] != actual["instance_ref"]
        || asset["instance_ref"] != actual["instance_ref"]
        || source["source_sha256"] != format!("sha256:{:x}", Sha256::digest(&actual_bytes))
        || asset["canonical_source_sha256"] != source["source_sha256"]
        || source["source_bytes"].as_u64() != Some(actual_bytes.len() as u64)
        || asset["canonical_source_bytes"] != source["source_bytes"]
        || source["sample_count"].as_u64() != Some(samples.len() as u64)
        || asset["sample_count"] != source["sample_count"]
        || source["part_count"].as_u64() != Some(part_count as u64)
        || source["header"] != source_header(actual)?
    {
        return Err(
            "closed selected FIELD asset differs from actual full original/current source".into(),
        );
    }
    let parts = asset["sample_parts"]
        .as_array()
        .ok_or("complete closed FIELD sample parts absent")?;
    if parts.len() != part_count {
        return Err("closed FIELD source lost a complete original sample part".into());
    }
    for (index, chunk) in samples.chunks(512).enumerate() {
        let part = pipe.borrow_mut().pull("field-part", index)?;
        let encoded = &parts[index];
        let text = part["canonical_decoded_bytes"]
            .as_str()
            .ok_or("C canonical FIELD part bytes absent")?;
        if text.len() > 4 * 1024 * 1024
            || part["schema"] != "oi.expression-native-field-part/v1"
            || part["index"].as_u64() != Some(index as u64)
            || part["first"].as_u64() != Some((index * 512) as u64)
            || encoded["first"] != part["first"]
            || encoded["count"] != part["count"]
            || part["count"].as_u64() != Some(chunk.len() as u64)
            || part["original_encoded_part"] != encoded["encoded"]
            || part["decoded_length"].as_u64() != Some(text.len() as u64)
            || part["decoded_sha256"] != format!("sha256:{:x}", Sha256::digest(text.as_bytes()))
        {
            return Err("original encoded/decoded FIELD part lost exact selected custody".into());
        }
        let selection_keys: &[&str] =
            if manifest["schema"] == "oi.expression-native-current-scene-delivery/v1" {
                &[
                    "expression_ref",
                    "expression_revision",
                    "expanded_document_sha256",
                    "scene_ref",
                    "scene_revision",
                ]
            } else {
                &[
                    "act_ref",
                    "act_revision",
                    "act_digest",
                    "edition_position",
                    "scene_ref",
                    "scene_revision",
                ]
            };
        for &key in selection_keys {
            if part["selection"][key] != manifest[key] {
                return Err("FIELD source part has another actual selected Act/Scene".into());
            }
        }
        if part["selection"]["source_digest"] != source["source_sha256"] {
            return Err("FIELD source part substituted another full source".into());
        }
        let decoded: Vec<Value> = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if decoded.as_slice() != chunk {
            return Err(
                "native full FIELD sample geometry differs from original C decoded source part"
                    .into(),
            );
        }
    }
    Ok(())
}

/// Called by ql-field-host's dedicated pipe branch, never public HostOperation.
/// The pipe is already exclusively held by the native C Manager. Every page
/// answer comes from its closed selected Act reader. The native original
/// receiving factory/Return/body/source is independently replayed by FieldHost.
pub fn serve_native_act_pipe(
    host: &mut FieldHost,
    operation: &mut NativeActOperation,
) -> Result<(), String> {
    let qualification = operation.qualification().clone();
    let value = operation.request().clone();
    let channel = RefCell::new(operation);
    serve_native_act_operation(
        host,
        value,
        || channel.borrow_mut().read_value(),
        |value| channel.borrow_mut().write_value(value),
        &qualification,
    )
}

// Only the actual qualified channel above reaches this factory. No public
// raw Value/callback API, schema discriminator or imported proof can call it.
fn serve_native_act_operation(
    host: &mut FieldHost,
    value: Value,
    read: impl FnMut() -> Result<Value, String>,
    write: impl FnMut(&Value) -> Result<(), String>,
    parent_qualification: &Value,
) -> Result<(), String> {
    bounded(&value, MAX_HOST_INPUT)?;
    let request: ActRequest = serde_json::from_value(value).map_err(|e| e.to_string())?;
    if request.schema != ACT_REQUEST
        || request.manager_lease.is_empty()
        || request.manager_lease.len() > 4096
        || request.manager_lease.chars().any(char::is_control)
        || ![
            "compile",
            "render",
            "readmit",
            "field-source",
            "field-descriptor",
            "source-bootstrap",
            "source-lifecycle",
            "acoustic-install",
            "acoustic-stage",
            "acoustic-replace",
            "acoustic-prepare",
            "performance-source",
            "performance-descriptor",
            "performance-timing",
            "recording-command",
            "recording-origin",
            "recording-save-cut",
        ]
        .contains(&request.mode.as_str())
    {
        return Err("unsupported dedicated native selected Act operation".into());
    }
    if request.declared_seed.is_some() && request.mode != "performance-source" {
        return Err(
            "native source seed belongs only to the explicit private source producer".into(),
        );
    }
    if (request.mode == "acoustic-prepare") != request.acoustic_configuration.is_some()
        || (request.original_acoustic_boundary.is_some()
            && !matches!(
                request.mode.as_str(),
                "acoustic-install" | "acoustic-stage" | "acoustic-replace"
            ))
    {
        return Err(
            "native acoustic input/boundary belongs only to its closed source operation".into(),
        );
    }
    if request.is_field() {
        let pipe = RefCell::new(Pipe {
            read,
            write,
            instance_ref: request.instance_ref.clone(),
            request_id: request.request_id.clone(),
            query: 0,
        });
        let mut recording_diagnostics = super::native_act_diagnostics::NativeDiagnosticSender::new(
            &request.instance_ref,
            &request.request_id,
            |value| pipe.borrow_mut().send(value),
            || pipe.borrow_mut().receive(),
        )?;
        let mut recording_delivery_error = None;
        let outcome = (|| -> Result<Value, String> {
            if !matches!(
                request.manifest["schema"].as_str(),
                Some(
                    "oi.expression-native-scene-delivery/v1"
                        | "oi.expression-native-current-scene-delivery/v1"
                )
            ) || request.checkpoint_index.is_some()
                || request.transaction_ref.is_some()
                || !request.edition_generation.is_empty()
                || !request.from_sample.is_empty()
                || !request.to_sample.is_empty()
            {
                return Err(
                    "native Field operation has a foreign selected Scene or performance operand"
                        .into(),
                );
            }
            validate_closed_scene_manifest(&request.manifest)?;
            let world_carrier = closed_field_world_carrier(&request.manifest)?;
            if !matches!(
                request.mode.as_str(),
                "source-bootstrap" | "source-lifecycle"
            ) {
                host.admit_native_act(&request)?;
            }
            let registration = request
                .field_registration
                .as_ref()
                .ok_or("actual Manager Field epoch registration absent")?;
            let artifact = host.retained_procedural_source_artifact()?;
            // The private host producer emitted the complete actual held tuple.
            // Replay its original inputs, comparing every full output again.
            let replay = |key: &str| -> Result<super::coupled::CoupledBasis, String> {
                let input: super::coupled::CoupledInput =
                    serde_json::from_value(artifact[key]["input"].clone())
                        .map_err(|e| e.to_string())?;
                let basis = input.compose()?;
                if serde_json::to_value(&basis).map_err(|e| e.to_string())? != artifact[key] {
                    return Err(
                        "actual native Field tuple differs from its complete source producer"
                            .into(),
                    );
                }
                Ok(basis)
            };
            let original = replay("original_basis")?;
            let current = replay("current_basis")?;
            let retained_source = if matches!(
                request.mode.as_str(),
                "field-descriptor"
                    | "source-bootstrap"
                    | "source-lifecycle"
                    | "performance-source"
                    | "acoustic-install"
                    | "performance-descriptor"
                    | "performance-timing"
                    | "recording-command"
                    | "recording-origin"
                    | "recording-save-cut"
                    | "acoustic-stage"
                    | "acoustic-replace"
                    | "acoustic-prepare"
            ) {
                qualify_field_source_parts(&request.manifest, &world_carrier, &artifact, &pipe)?;
                Some(&artifact)
            } else {
                // Explicit source observation retains a newly changed current
                // tuple. It grants no timing or material application. Its
                // actual portable original source/owner must still match.
                FieldHost::validate_retained_field_sources_with_artifact(
                    &world_carrier["presentation"],
                    &request.instance_ref,
                    &original,
                    &original,
                    None,
                )?;
                None
            };
            let lease = NativeActSourceLease {
                manager_lease: &request.manager_lease,
                request_id: count(&json!(request.request_id))?,
                manifest: &request.manifest,
                parent_qualification,
                source: NativeActSource::Field {
                    registration,
                    retained_source,
                    source_read: request.source_bootstrap.as_ref(),
                    world_carrier: &world_carrier,
                },
            };
            if retained_source.is_some() {
                lease.validate_field_sources(&request.instance_ref, &original, &current)?;
            }
            if matches!(
                request.mode.as_str(),
                "source-bootstrap" | "source-lifecycle"
            ) {
                let source_read = request
                    .source_bootstrap
                    .as_ref()
                    .ok_or("actual privately issued Scene read absent")?;
                let mut native = request
                    .procedural_request
                    .clone()
                    .ok_or("original scoped procedural request absent")?;
                if native["schema"] != "ql.field-host-request/v1"
                    || native["instance_ref"] != request.instance_ref
                    || native["event_ref"] != request.event_ref
                    || native["subject_ref"] != request.subject_ref
                    || native["request_id"] != request.request_id
                    || native["command"]["operation"] != "procedure"
                    || source_read["issuer_receipt"]["native_identity"]["instance_ref"]
                        != request.instance_ref
                    || source_read["issuer_receipt"]["native_identity"]["event_ref"]
                        != request.event_ref
                    || source_read["issuer_receipt"]["native_identity"]["subject_ref"]
                        != request.subject_ref
                {
                    return Err(
                        "native Scene issuer changed complete original owner/ordinal".into(),
                    );
                }
                let input = &native["command"]["request"]["input"];
                match (
                    request.mode.as_str(),
                    native["command"]["request"]["action"].as_str(),
                ) {
                    ("source-bootstrap", Some("source_bootstrap")) => {
                        if input["authorship"]
                            != source_read["issuer_receipt"]["original_intent"]["authorship"]
                        {
                            return Err("bootstrap changed full original authorship".into());
                        }
                        let bootstrap: crate::procedural_source::NativeSourceBootstrap =
                            serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
                        lease.validate_procedural_scene_read(
                            &request.instance_ref,
                            &bootstrap.scene,
                            &bootstrap.authorship.contributors,
                        )?;
                    }
                    ("source-lifecycle", Some("lifecycle")) => {
                        let lifecycle: crate::procedural_conduct::lifecycle::NativeLifecycleInput =
                            serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
                        let contributors = lifecycle.selected_contributors()?;
                        lease.validate_procedural_scene_read(
                            &request.instance_ref,
                            &lifecycle.reading.scene_read,
                            &contributors,
                        )?;
                    }
                    ("source-lifecycle", Some("lifecycle_cancel")) => {
                        let cancel: crate::procedural_conduct::lifecycle::NativeLifecycleCancel =
                            serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
                        lease.validate_procedural_scene_read(
                            &request.instance_ref,
                            &cancel.scene_read,
                            &cancel.contributors,
                        )?;
                    }
                    _ => {
                        return Err("closed procedural Scene operation changed exact action".into());
                    }
                }
                // Actual E lifecycle/cancel validates the original installed
                // procedure/Runtime/Document journal. No bootstrap is replayed.
                // This consumes the SAME original Host ordinal exactly once.
                let scoped: super::host::HostRequest =
                    serde_json::from_value(native.take()).map_err(|e| e.to_string())?;
                let receipt = host.execute_native_procedure(
                    scoped,
                    &lease,
                    super::performance::NativeTimingMoment::Boundary,
                );
                return Ok(json!({"selection":lease.evidence(),"native_receipt":receipt}));
            }
            if matches!(
                request.mode.as_str(),
                "recording-command" | "recording-origin" | "recording-save-cut"
            ) {
                use sha2::{Digest, Sha256};
                if request.source_bootstrap.is_some()
                    || request.manifest["schema"]
                        != "oi.expression-native-current-scene-delivery/v1"
                {
                    return Err("recording command requires its actual current Document/Scene; no fabricated Act/source issuer".into());
                }
                let (scene_constructor, command) = if matches!(
                    request.mode.as_str(),
                    "recording-origin" | "recording-save-cut"
                ) {
                    let query: NativeRecordingOriginQuery = serde_json::from_value(
                        request
                            .procedural_request
                            .clone()
                            .ok_or("native recording origin query absent")?,
                    )
                    .map_err(|e| e.to_string())?;
                    if query.schema
                        != if request.mode == "recording-save-cut" {
                            "ql.native-scene-recording-save-cut/v1"
                        } else {
                            "ql.native-scene-recording-origin/v1"
                        }
                    {
                        return Err("foreign native recording origin query".into());
                    }
                    (query.scene_constructor, None)
                } else {
                    let query: NativeRecordingCommandQuery = serde_json::from_value(
                        request
                            .procedural_request
                            .clone()
                            .ok_or("native recording command absent")?,
                    )
                    .map_err(|e| e.to_string())?;
                    if query.schema != "ql.native-scene-recording-command/v1" {
                        return Err("foreign native recording command query".into());
                    }
                    (query.scene_constructor, Some(query.command))
                };
                let fact = &scene_constructor;
                let doc_text = request.manifest["canonical_document_bytes"]
                    .as_str()
                    .ok_or("recording complete typed Document bytes absent")?;
                if fact["schema"] != "oi.native-document-scene-constructor/v1"
                    || fact["expression_ref"] != request.manifest["expression_ref"]
                    || fact["scene_ref"] != request.manifest["scene_ref"]
                    || fact["document_revision"] != request.manifest["expression_revision"]
                    || fact["document_sha256"]
                        != format!("{:x}", Sha256::digest(doc_text.as_bytes()))
                    || fact["generation_domain"] != "native-document-scene-construction"
                {
                    return Err("recording lost the actual native Scene constructor/full typed Document custody".into());
                }
                // Evidence comes from the same OS/image-qualified C channel.
                // Only C's actual registered Arc/Map/live owner can admit the
                // edit; this JSON cannot mint that owner.
                let mut selection = lease.evidence();
                selection["scene_constructor"] = scene_constructor;
                if request.mode == "recording-save-cut" {
                    return Ok(match host.capture_native_stopped_recording_cut(&lease) {
                        Ok(cut) => {
                            let delivery = recording_diagnostics
                                .send_receipt("recording.cut_observation", 0, cut.observation())
                                .and_then(|()| {
                                    recording_diagnostics.send_receipt(
                                        "recording.cut_checkpoint",
                                        0,
                                        cut.checkpoint(),
                                    )
                                });
                            if let Err(error) = delivery {
                                recording_delivery_error = Some(error.clone());
                                json!({"schema":"ql.native-scene-recording-save-cut/v1","selection":selection,
                                    "accepted":false,"reason":error,
                                    "host_receipt":host.recording_controller_refusal(&request.request_id,"actual native save cut receipt delivery failed")})
                            } else {
                                json!({"schema":"ql.native-scene-recording-save-cut/v1","selection":selection,"accepted":true,
                                    "host_receipt":host.recording_controller_receipt(&request.request_id,Some(cut.command_receipt()))})
                            }
                        }
                        Err(failure) => {
                            for (index, pulse) in failure.native_receipts.iter().enumerate() {
                                if let Err(error) = recording_diagnostics.send_receipt(
                                    "recording.cut_failure",
                                    index,
                                    pulse,
                                ) {
                                    recording_delivery_error = Some(error);
                                    break;
                                }
                            }
                            json!({"schema":"ql.native-scene-recording-save-cut/v1","selection":selection,"accepted":false,
                                "reason":failure.reason,"host_receipt":host.recording_controller_refusal(&request.request_id,&failure.reason)})
                        }
                    });
                }
                if command.is_none() {
                    return Ok(match host.capture_native_recording_origin(&lease) {
                        Ok(origin) => {
                            json!({"schema":"ql.native-scene-recording-origin/v1","selection":selection,"accepted":true,"native_pulse":origin.native_pulse(),"host_receipt":host.recording_controller_receipt(&request.request_id,None)})
                        }
                        Err(refusal) => {
                            json!({"schema":"ql.native-scene-recording-origin/v1","selection":selection,"accepted":false,"reason":refusal.reason,"native_receipts":refusal.native_receipts,"host_receipt":host.recording_controller_refusal(&request.request_id,&refusal.reason)})
                        }
                    });
                }
                return Ok(
                    match host.capture_native_performance_command(
                        &lease,
                        command.ok_or("recording command absent")?,
                    ) {
                        Ok(captured) => json!({"schema":"ql.native-scene-recording-capture/v1",
                        "selection":selection,"accepted":true,"command_receipt":captured.receipt(),
                        "native_pulse":captured.native_pulse(),"host_receipt":host.recording_controller_receipt(&request.request_id,Some(captured.receipt()))}),
                        Err(refusal) => json!({"schema":"ql.native-scene-recording-capture/v1",
                        "selection":selection,"accepted":false,"reason":refusal.reason,
                        "native_pulse":refusal.native_pulse,"host_receipt":host.recording_controller_refusal(&request.request_id,&refusal.reason)}),
                    },
                );
            }
            if matches!(
                request.mode.as_str(),
                "performance-descriptor" | "performance-timing"
            ) {
                if request.source_bootstrap.is_some() {
                    return Err("native performance timing carries unowned source issuance".into());
                }
                if request.mode == "performance-descriptor" {
                    if request.procedural_request.is_some() {
                        return Err(
                            "native performance descriptor carries a foreign timing query".into(),
                        );
                    }
                    return Ok(match host.procedural_timing_descriptor(&lease) {
                        Ok(prepared) => json!({"selection":lease.evidence(),"available":true,
                            "binding":prepared.binding(),"native_position":prepared.position(),
                            "native_pulse":prepared.native_pulse(),
                            "standing":"actual original native management timing descriptor; observed source/clock boundary only"}),
                        Err(refusal) => json!({"selection":lease.evidence(),"available":false,
                            "reason":refusal.reason(),"native_pulse":refusal.native_pulse()}),
                    });
                }
                let query: NativePerformanceTimingQuery = serde_json::from_value(
                    request
                        .procedural_request
                        .clone()
                        .ok_or("native timing query absent")?,
                )
                .map_err(|e| e.to_string())?;
                if query.schema != "ql.native-performance-timing-query/v1" {
                    return Err("foreign native performance timing query schema".into());
                }
                let ordinal = count(&json!(query.ordinal))?;
                let moment = match (query.moment.as_str(), ordinal) {
                    ("boundary", 0) => super::performance::NativeTimingMoment::Boundary,
                    ("score", n) if n != 0 => super::performance::NativeTimingMoment::Score(n),
                    ("clock", n) if n != 0 => super::performance::NativeTimingMoment::Clock(n),
                    ("applied", n) if n != 0 => super::performance::NativeTimingMoment::Applied(n),
                    _ => return Err("native timing query has no exact owner moment/ordinal".into()),
                };
                // Imported operands create no witness. The SAME actual private
                // Management owner must supply the exact queue/clock/application
                // fact, full source, domain, epoch and current body position.
                return Ok(
                    match host.procedural_timing_witness(query.binding, &lease, moment) {
                        Ok(prepared) => json!({"selection":lease.evidence(),"available":true,
                        "timing_evidence":prepared.witness().evidence(),
                        "native_position":prepared.position(),"native_pulse":prepared.native_pulse(),
                        "standing":"actual native timing observation; serialized evidence cannot mint a witness or consumer acknowledgement"}),
                        Err(refusal) => json!({"selection":lease.evidence(),"available":false,
                        "reason":refusal.reason(),"native_pulse":refusal.native_pulse()}),
                    },
                );
            }
            if request.mode == "performance-source" {
                if request.source_bootstrap.is_some() || request.procedural_request.is_some() {
                    return Err(
                        "native performance source read carries unowned procedural operands".into(),
                    );
                }
                let seed = request
                    .declared_seed
                    .as_ref()
                    .ok_or("native performance source has no original authored declared seed")?;
                let seed = count(&json!(seed))?;
                // This is the SAME activated existing owner and its native
                // receiving/Return producer. No imported source bundle or old
                // fixture can stand in for this private getter's actual result.
                lease.validate_field_sources(&request.instance_ref, &original, &current)?;
                let source = host.retained_performance_source_artifact(seed)?;
                lease.validate_field_sources(&request.instance_ref, &original, &current)?;
                return Ok(
                    json!({"selection":lease.evidence(),"source_artifact":source,"host_receipt":host.recording_controller_receipt(&request.request_id,None),
                    "standing":"same privately borrowed native Scene and actual activated source/Return; no selected Act, rendered output or public disclosure grant"}),
                );
            }
            if matches!(
                request.mode.as_str(),
                "acoustic-prepare" | "acoustic-install" | "acoustic-stage" | "acoustic-replace"
            ) {
                if request.source_bootstrap.is_some() || request.procedural_request.is_some() {
                    return Err(
                        "acoustic source operation carries unowned procedural authority".into(),
                    );
                }
                let sources = request.manifest["scene"]["performance"]["native_sources"]
                    .as_array()
                    .ok_or("closed Scene has no complete native source")?;
                if sources.len() != 1 {
                    return Err("acoustic operation requires exact selected native source".into());
                }
                let configuration = if request.mode == "acoustic-prepare" {
                    request
                        .acoustic_configuration
                        .as_ref()
                        .ok_or("closed authored acoustic configuration absent")?
                        .clone()
                } else {
                    serde_json::from_value::<super::performance::AcousticConfiguration>(
                        sources[0]["native_bundle"]["acoustic_receiving"]["packet"]["configuration"].clone())
                        .map_err(|e|e.to_string())?
                };
                if request.mode == "acoustic-prepare" {
                    if sources[0]["native_bundle"]
                        .get("acoustic_receiving")
                        .is_some()
                    {
                        let candidate = host.prepare_performance_acoustic_update(configuration)?;
                        lease.validate_source_assets(
                            &request.instance_ref,
                            candidate.before_source_assets(),
                        )?;
                        lease.validate_field_sources(&request.instance_ref, &original, &current)?;
                        return Ok(json!({"selection":lease.evidence(),"prepared_acoustic":{
                            "schema":"ql.native-current-scene-acoustic-candidate/v1","change":"geometry-replacement",
                            "configuration":candidate.configuration(),"source_assets":candidate.source_assets(),
                            "native_boundary":candidate.native_boundary()},
                            "standing":"pure stopped same-source receiver replacement; no history/output/source publication or pulse drain"}));
                    }
                    let candidate =
                        host.prepare_performance_acoustic_installation(configuration)?;
                    lease.validate_source_assets(
                        &request.instance_ref,
                        candidate.before_source_assets(),
                    )?;
                    lease.validate_field_sources(&request.instance_ref, &original, &current)?;
                    return Ok(json!({"selection":lease.evidence(),"prepared_acoustic":{
                        "schema":"ql.native-current-scene-acoustic-candidate/v1","change":"initial",
                        "configuration":candidate.configuration(),"source_assets":candidate.source_assets(),
                        "native_boundary":candidate.native_boundary()},
                        "standing":"pure current native source candidate; no output/source publication or pulse drain"}));
                }
                if request.mode == "acoustic-replace" {
                    let candidate = host.prepare_performance_acoustic_update(configuration)?;
                    if request.original_acoustic_boundary.as_ref()
                        != Some(candidate.native_boundary())
                    {
                        return Err("original receiver replacement boundary changed before current Scene application".into());
                    }
                    return Ok(
                        match host.replace_performance_acoustic(&candidate, &lease) {
                            Ok(pulse) => {
                                json!({"selection":lease.evidence(),"accepted":true,"native_pulse":pulse})
                            }
                            Err(refusal) => {
                                json!({"selection":lease.evidence(),"accepted":false,"reason":refusal.reason(),"native_pulse":refusal.native_pulse()})
                            }
                        },
                    );
                }
                let (candidate, preparation_error) =
                    match host.prepare_performance_acoustic_installation(configuration) {
                        Ok(candidate) => (Some(candidate), None),
                        Err(reason) => (None, Some(reason)),
                    };
                if let Some(expected) = &request.original_acoustic_boundary {
                    let candidate = candidate
                        .as_ref()
                        .ok_or("prepared original acoustic source/boundary unavailable")?;
                    if candidate.native_boundary() != expected {
                        return Err("original acoustic preparation boundary changed before current Scene installation".into());
                    }
                }
                let applied = if request.mode == "acoustic-stage" {
                    let candidate = candidate.as_ref().ok_or(
                        "staging requires the exact original pure candidate after actual Scene CAS",
                    )?;
                    host.stage_performance_acoustic_candidate(candidate, &lease)
                } else if let Some(candidate) = candidate.as_ref() {
                    host.install_performance_acoustic_candidate(candidate, &lease)
                } else {
                    // Existing fully selected staged legacy snapshots retain
                    // their native install path; no imported timing is guessed.
                    host.install_performance_acoustic(&lease)
                };
                return Ok(match applied {
                    Ok(pulse) => json!({"selection":lease.evidence(),"accepted":true,
                        "staged":request.mode=="acoustic-stage","native_pulse":pulse}),
                    Err(refusal) => json!({"selection":lease.evidence(),"accepted":false,
                        "reason":refusal.reason(),"preparation_error":preparation_error,"native_pulse":refusal.native_pulse()}),
                });
            }
            if request.source_bootstrap.is_some() || request.procedural_request.is_some() {
                return Err("ordinary Field read carries unowned source issuance".into());
            }
            if request.mode == "field-source" {
                return Ok(
                    json!({"selection":lease.evidence(),"native_field_source":artifact,
                    "timing":null,"standing":"complete actual held Field source observation; no material/body/audio acknowledgement"}),
                );
            }
            Ok(match host.native_field_timing_descriptor(&lease) {
                Ok(binding) => {
                    json!({"selection":lease.evidence(),"binding":binding,"native_field_source":artifact,"available":true})
                }
                Err(refusal) => {
                    json!({"selection":lease.evidence(),"binding":null,"native_field_source":artifact,
                    "native_field_receipt":refusal.native_receipt(),"available":false,"error":refusal.reason()})
                }
            })
        })();
        let mut reply = host.native_act_result(&request.request_id, &outcome);
        if request.mode == "field-source" {
            // Genuine SAME-operation host observation for the existing Session.
            // Failed pre-admission retains the actual old last_request_id;
            // no caller may promote it into an acknowledged next ordinal.
            reply["host_receipt"] = match &outcome {
                Ok(_) => host.recording_controller_receipt(&request.request_id, None),
                Err(reason) => host.recording_controller_refusal(&request.request_id, reason),
            };
        }
        if request.mode == "recording-save-cut" {
            reply["diagnostics"] = recording_diagnostics.manifest();
            if let Some(error) = recording_delivery_error {
                reply["diagnostic_delivery_error"] = json!(error);
            }
        }
        bounded(&reply, MAX_HOST_OUTPUT as u64)?;
        return pipe.borrow_mut().send(&reply);
    }
    if request.field_registration.is_some()
        || request.source_bootstrap.is_some()
        || request.procedural_request.is_some()
    {
        return Err("performance operation carries an unowned native Field registration".into());
    }
    if request.manifest["schema"] != "oi.expression-performance-delivery/v1"
        || ["act_ref", "scene_ref", "expression_ref"]
            .iter()
            .any(|key| {
                request.manifest[key].as_str().is_none_or(|text| {
                    text.is_empty() || text.len() > 4096 || text.chars().any(char::is_control)
                })
            })
        || [
            "act_digest",
            "expanded_document_sha256",
            "performance_digest",
            "selected_performance_sha256",
        ]
        .iter()
        .any(|key| {
            request.manifest[key]
                .as_str()
                .and_then(|text| text.strip_prefix("sha256:"))
                .is_none_or(|hex| {
                    hex.len() != 64
                        || !hex
                            .bytes()
                            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                })
        })
        || [
            "act_revision",
            "edition_position",
            "expression_revision",
            "scene_revision",
        ]
        .iter()
        .any(|key| request.manifest[key].as_u64().is_none())
    {
        return Err("native selected Act scalar identity is incomplete or noncanonical".into());
    }
    host.admit_native_act(&request)?;
    let pipe = RefCell::new(Pipe {
        read,
        write,
        instance_ref: request.instance_ref.clone(),
        request_id: request.request_id.clone(),
        query: 0,
    });
    let mut native_render_failure = None;
    let mut diagnostic_delivery_error = None;
    let mut diagnostic_sender = super::native_act_diagnostics::NativeDiagnosticSender::new(
        &request.instance_ref,
        &request.request_id,
        |value| pipe.borrow_mut().send(value),
        || pipe.borrow_mut().receive(),
    )?;
    let result = (|| {
        let mut pages = Pages(&pipe);
        let from = count(&json!(request.from_sample))?;
        let to = count(&json!(request.to_sample))?;
        if request.mode == "readmit" {
            // Compile all original pages before any numerical restore. Neither
            // a shortened manifest nor a checkpoint alone can mint the lease.
            let score = host.compile_native_act_score(
                count(&json!(request.edition_generation))?,
                &request.manifest,
                &mut pages,
            )?;
            let index = request
                .checkpoint_index
                .ok_or("native readmission checkpoint index absent")?;
            let checkpoint = pages.read_checkpoint(index)?;
            for key in [
                "act_ref",
                "act_revision",
                "act_digest",
                "edition_position",
                "expanded_document_sha256",
                "scene_ref",
                "scene_revision",
                "performance_digest",
            ] {
                if checkpoint[key] != request.manifest[key] {
                    return Err(
                        "native readmission checkpoint differs from complete selected Act".into(),
                    );
                }
            }
            if checkpoint["schema"] != "oi.expression-native-checkpoint-delivery/v1"
                || checkpoint["checkpoint_index"].as_u64() != u64::try_from(index).ok()
                || request.manifest["performance"]["checkpoints"][index] != checkpoint["checkpoint"]
            {
                return Err(
                    "native readmission checkpoint is absent from the full native performance"
                        .into(),
                );
            }
            let wire = checkpoint["canonical_native_wire_bytes"]
                .as_str()
                .ok_or("native checkpoint original wire text absent")?;
            let decoded: Value = serde_json::from_str(wire).map_err(|e| e.to_string())?;
            if decoded != checkpoint["native_management_wire"] {
                return Err("native checkpoint exact text and typed wire differ".into());
            }
            let reference = checkpoint["checkpoint"]["checkpoint_ref"]
                .as_str()
                .ok_or("native checkpoint original reference absent")?;
            let transaction = request
                .transaction_ref
                .as_deref()
                .filter(|s| !s.is_empty() && s.len() <= 4096 && !s.chars().any(char::is_control))
                .ok_or("native receiving transaction reference absent or unbounded")?;
            let lease = NativeActSourceLease {
                manager_lease: &request.manager_lease,
                request_id: count(&json!(request.request_id))?,
                manifest: &request.manifest,
                parent_qualification,
                source: NativeActSource::Performance {
                    checkpoint: Some(&checkpoint),
                },
            };
            lease.validate_selected_checkpoint(&request.instance_ref, reference, wire)?;
            return Ok(
                match host.readmit_retained_performance_checkpoint(
                    &lease,
                    wire,
                    reference,
                    transaction,
                ) {
                    Ok(actual) => json!({"score":score,"selection":lease.evidence(),
                    "receiving_readmission":actual.readmission(),"native_pulse":actual.native_pulse(),
                    "readmitted":true,"error":null}),
                    Err(refusal) => json!({"score":score,"selection":lease.evidence(),
                    "receiving_readmission":null,"native_pulse":refusal.native_pulse(),
                    "readmitted":false,"error":refusal.reason()}),
                },
            );
        }
        if request.checkpoint_index.is_some() || request.transaction_ref.is_some() {
            return Err("non-readmission operation carries unowned checkpoint operands".into());
        }
        if request.mode == "compile" {
            let score = host.compile_native_act_score(
                count(&json!(request.edition_generation))?,
                &request.manifest,
                &mut pages,
            )?;
            let lease = NativeActSourceLease {
                manager_lease: &request.manager_lease,
                request_id: count(&json!(request.request_id))?,
                manifest: &request.manifest,
                parent_qualification,
                source: NativeActSource::Performance { checkpoint: None },
            };
            let selected = lease.evidence();
            return Ok(json!({"score":score,"selection":selected,"rendered":false}));
        }
        let plan = host.prepare_native_act_render(
            count(&json!(request.edition_generation))?,
            &request.manifest,
            &mut pages,
            from,
            to,
        )?;
        let score = plan.score().snapshot()?;
        let lease = NativeActSourceLease {
            manager_lease: &request.manager_lease,
            request_id: count(&json!(request.request_id))?,
            manifest: &request.manifest,
            parent_qualification,
            source: NativeActSource::Performance { checkpoint: None },
        };
        let selected = lease.evidence();
        let scope = plan.scope();
        let rendered = host.render_native_act(&plan,&mut pages,|renderer| {
            pipe.borrow_mut().send(&json!({"schema":"ql.native-act-owner-ready/v1","instance_ref":request.instance_ref,
                "request_id":request.request_id,"selection":selected,"scope":scope,"score":score}))?;
            let mut commands = NativeRenderCommands::new(count(&scope["frames"])?)?;
            for _ in 0..commands.messages() {
                let control=pipe.borrow_mut().control()?;
                match commands.next(&control)? {
                    NativeRenderCommand::Render(frames)=> {
                        let mut mono=[0.0_f32;512];
                        let chunk=renderer.render_into(frames,&mut mono[..frames as usize])?;
                        pipe.borrow_mut().send(&json!({"schema":"ql.native-act-owner-chunk/v1","instance_ref":request.instance_ref,
                            "request_id":request.request_id,"chunk":chunk}))?;
                    }
                    NativeRenderCommand::Finished=>return Ok(()),
                    NativeRenderCommand::Abort=>return Err("native C consumer refused output; original owner restitution required".into()),
                }
            }
            Err("native selected Act render omitted its terminal command".into())
        });
        match rendered {
            Ok(rendered) => {
                let metadata = rendered.restoration.compact_metadata();
                if let Err(reason) = rendered.restoration.visit_original_receipts(
                    "restoration",
                    &mut |kind, index, receipt| {
                        diagnostic_sender.send_receipt(kind, index, receipt)
                    },
                ) {
                    diagnostic_delivery_error = Some(reason.clone());
                    native_render_failure = Some(
                        json!({"schema":"ql.native-render-diagnostic-delivery-failure/v1",
                        "reason":reason,"restoration":metadata}),
                    );
                    return Err("actual native restoration receipt delivery failed; retained partial custody is unavailable".into());
                }
                Ok(
                    json!({"score":score,"selection":selected,"rendered":rendered.result.is_ok(),
                    "restoration":metadata,"activity_error":rendered.result.err()}),
                )
            }
            Err(failure) => {
                let reason = failure.reason.clone();
                native_render_failure = Some(failure.compact_metadata());
                if let Err(error) = failure.visit_original_receipts(&mut |kind, index, receipt| {
                    diagnostic_sender.send_receipt(kind, index, receipt)
                }) {
                    diagnostic_delivery_error = Some(error);
                }
                Err(reason)
            }
        }
    })();
    let mut reply = host.native_act_result(&request.request_id, &result);
    // Only compact metadata and original-order file descriptors cross the
    // terminal frame. Each complete original receipt was separately ACKed.
    reply["diagnostics"] = diagnostic_sender.manifest();
    if let Some(failure) = native_render_failure {
        reply["native_render_failure"] = failure;
    }
    if let Some(error) = diagnostic_delivery_error {
        reply["diagnostic_delivery_error"] = json!(error);
    }
    pipe.borrow_mut().send(&reply)
}

#[cfg(test)]
mod render_command_tests {
    use super::*;
    // These test the real production command-state validator, not an audio or
    // native-source stand-in. The same closed native producer/renderer passage
    // still needs its normal guarded full-duration gate.
    fn command(operation: &str, frames: Option<u64>) -> Value {
        let mut value = json!({"schema":CONTROL,"instance_ref":"native:command-test",
            "request_id":"1","operation":operation});
        if let Some(frames) = frames {
            value["frames"] = json!(frames);
        }
        value
    }
    #[test]
    fn production_render_commands_keep_the_full_fifteen_minute_scope_after_16384_blocks() {
        let mut commands = NativeRenderCommands::new(43_200_000).unwrap();
        assert_eq!(commands.messages(), 84_376);
        let mut delivered = 0_u64;
        for _ in 0..84_375 {
            let actual = commands.next(&command("render", Some(512))).unwrap();
            match actual {
                NativeRenderCommand::Render(frames) => delivered += u64::from(frames),
                _ => panic!("full interval must remain rendering"),
            }
        }
        assert_eq!(delivered, 43_200_000);
        assert!(matches!(
            commands.next(&command("finished", None)).unwrap(),
            NativeRenderCommand::Finished
        ));
        assert!(commands.next(&command("render", Some(512))).is_err());
    }
    #[test]
    fn production_render_commands_refuse_early_terminal_loss_and_changed_last_block() {
        let mut early = NativeRenderCommands::new(513).unwrap();
        assert!(early.next(&command("finished", None)).is_err());
        let mut changed = NativeRenderCommands::new(513).unwrap();
        assert!(matches!(
            changed.next(&command("render", Some(512))).unwrap(),
            NativeRenderCommand::Render(512)
        ));
        assert!(changed.next(&command("render", Some(512))).is_err());
        let mut exact = NativeRenderCommands::new(513).unwrap();
        assert!(matches!(
            exact.next(&command("render", Some(512))).unwrap(),
            NativeRenderCommand::Render(512)
        ));
        assert!(matches!(
            exact.next(&command("render", Some(1))).unwrap(),
            NativeRenderCommand::Render(1)
        ));
        assert!(matches!(
            exact.next(&command("finished", None)).unwrap(),
            NativeRenderCommand::Finished
        ));
        let mut aborted = NativeRenderCommands::new(43_200_000).unwrap();
        assert!(matches!(
            aborted.next(&command("abort", None)).unwrap(),
            NativeRenderCommand::Abort
        ));
        assert!(aborted.next(&command("finished", None)).is_err());
        assert!(NativeRenderCommands::new(0).is_err());
    }
}

#[cfg(test)]
mod closed_field_carrier_tests {
    use super::*;
    use sha2::{Digest, Sha256};

    // These exercise the real closed-Document parser and full CAS validator.
    // They grant no native source lease or private host/Scene constructor.
    fn manifest(mut scenes: Vec<Value>) -> Value {
        let selected = scenes.remove(0);
        scenes.insert(0, selected.clone());
        let document = json!({"expression_ref":"unqualified:parser-document",
            "revision":7,"scenes":scenes});
        let doc_text = serde_json::to_string(&document).unwrap();
        let scene_text = serde_json::to_string(&selected).unwrap();
        json!({"schema":"oi.expression-native-current-scene-delivery/v1",
            "expression_ref":document["expression_ref"],"expression_revision":document["revision"],
            "expanded_document_sha256":format!("sha256:{:x}",Sha256::digest(doc_text.as_bytes())),
            "canonical_document_bytes":doc_text,"scene_ref":selected["scene_ref"],
            "scene_revision":selected["revision"],"scene":selected,
            "canonical_scene_bytes":scene_text,
            "selected_scene_sha256":format!("sha256:{:x}",Sha256::digest(scene_text.as_bytes())),
            "field_source_manifest":null})
    }
    fn selected() -> Value {
        json!({"scene_ref":"unqualified:generated-scene","revision":2,
            "presentation":{"schema":"oi.journey-scene/v1","scene":{}},
            "native_field_source":{"instance_ref":"unqualified:wrong-face"}})
    }
    fn world() -> Value {
        json!({"scene_ref":"unqualified:world-scene","revision":4,
            "presentation":{"schema":"oi.journey-scene/v1","scene":{
                "epiWorld":{"schema":"oi.epi-world-material/v1","world":{}}}},
            "native_field_source":{"instance_ref":"unqualified:world-source"}})
    }
    #[test]
    fn selected_generated_scene_resolves_only_the_same_document_world_carrier() {
        let source = world();
        let closed = manifest(vec![selected(), source.clone()]);
        validate_closed_scene_manifest(&closed).unwrap();
        assert_eq!(closed_field_world_carrier(&closed).unwrap(), source);
        assert_eq!(
            closed["scene"]["native_field_source"]["instance_ref"],
            "unqualified:wrong-face"
        );
        let selected_world = manifest(vec![world(), selected()]);
        validate_closed_scene_manifest(&selected_world).unwrap();
        assert_eq!(
            closed_field_world_carrier(&selected_world).unwrap(),
            selected_world["scene"]
        );
    }
    #[test]
    fn lost_or_ambiguous_world_carriers_cannot_fall_back_to_selected_source() {
        let missing = manifest(vec![selected()]);
        validate_closed_scene_manifest(&missing).unwrap();
        assert!(closed_field_world_carrier(&missing).is_err());
        let mut second = world();
        second["scene_ref"] = json!("unqualified:second-world");
        let ambiguous = manifest(vec![selected(), world(), second]);
        validate_closed_scene_manifest(&ambiguous).unwrap();
        assert!(closed_field_world_carrier(&ambiguous).is_err());
    }
    #[test]
    fn carrier_resolution_does_not_excuse_changed_full_document_or_selected_cas() {
        let original = manifest(vec![selected(), world()]);
        validate_closed_scene_manifest(&original).unwrap();
        for key in ["expanded_document_sha256", "selected_scene_sha256"] {
            let mut changed = original.clone();
            changed[key] = json!("sha256:stale");
            assert!(validate_closed_scene_manifest(&changed).is_err());
        }
        let mut changed = original.clone();
        changed["scene"]["revision"] = json!(3);
        assert!(validate_closed_scene_manifest(&changed).is_err());
        let mut document: Value =
            serde_json::from_str(original["canonical_document_bytes"].as_str().unwrap()).unwrap();
        document["scenes"][1]["native_field_source"]["instance_ref"] =
            json!("unqualified:foreign-source");
        let mut changed = original;
        changed["canonical_document_bytes"] = json!(serde_json::to_string(&document).unwrap());
        assert!(validate_closed_scene_manifest(&changed).is_err());
    }
}
