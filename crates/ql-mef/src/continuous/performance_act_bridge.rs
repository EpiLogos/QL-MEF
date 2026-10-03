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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActRequest {
    schema: String,
    pub(crate) instance_ref: String,
    pub(crate) event_ref: String,
    pub(crate) subject_ref: String,
    pub(crate) request_id: String,
    manager_lease: String,
    edition_generation: String,
    mode: String,
    from_sample: String,
    to_sample: String,
    manifest: Value,
    #[serde(default)]
    checkpoint_index: Option<usize>,
    #[serde(default)]
    transaction_ref: Option<String>,
}

/// Minted only inside this dedicated operation after the native full-source
/// score compiler accepts the complete C-owned selection. No Deserialize,
/// Clone or public constructor can turn retained JSON into a live Act lease.
pub struct NativeActSourceLease<'a> {
    manager_lease: &'a str,
    request_id: u64,
    manifest: &'a Value,
    parent_qualification: &'a Value,
    checkpoint: Option<&'a Value>,
}
impl NativeActSourceLease<'_> {
    pub fn validate_source_assets(
        &self,
        instance_ref: &str,
        actual_source_assets: &Value,
    ) -> Result<(), String> {
        let sources = self.manifest["performance"]["native_sources"]
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
    /// Exact checkpoint selected by the same native closed reader. This is
    /// required by the private receiving restore owner before issuing Control.
    pub(crate) fn validate_selected_checkpoint(
        &self,
        instance_ref: &str,
        checkpoint_ref: &str,
        wire: &str,
    ) -> Result<(), String> {
        let selected = self
            .checkpoint
            .ok_or("native Act lease has no selected checkpoint custody")?;
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
        json!({"schema":"ql.native-act-source-lease-evidence/v1","manager_lease":self.manager_lease,
            "native_parent_qualification":self.parent_qualification,"native_request_id":self.request_id.to_string(),"act_ref":self.manifest["act_ref"],
            "act_revision":self.manifest["act_revision"],"act_digest":self.manifest["act_digest"],
            "edition_position":self.manifest["edition_position"],"expanded_document_sha256":self.manifest["expanded_document_sha256"],
            "scene_ref":self.manifest["scene_ref"],"scene_revision":self.manifest["scene_revision"],
            "performance_digest":self.manifest["performance_digest"],"selected_performance_sha256":self.manifest["selected_performance_sha256"]})
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
        || !["compile", "render", "readmit"].contains(&request.mode.as_str())
    {
        return Err("unsupported dedicated native selected Act operation".into());
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
                checkpoint: Some(&checkpoint),
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
                checkpoint: None,
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
            checkpoint: None,
        };
        let selected = lease.evidence();
        let scope = plan.scope();
        host.render_native_act(&plan,&mut pages,|renderer| {
            pipe.borrow_mut().send(&json!({"schema":"ql.native-act-owner-ready/v1","instance_ref":request.instance_ref,
                "request_id":request.request_id,"selection":selected,"scope":scope,"score":score}))?;
            loop {
                let control=pipe.borrow_mut().control()?;
                match control["operation"].as_str() {
                    Some("render")=> {
                        if control.as_object().map(|v|v.len())!=Some(5) { return Err("native render control contains unowned fields".into()); }
                        let frames=u32::try_from(control["frames"].as_u64().ok_or("native render frames absent")?).map_err(|e|e.to_string())?;
                        if !(1..=512).contains(&frames) { return Err("native Act render exceeds actual block bound".into()); }
                        let mut mono=[0.0_f32;512];
                        let chunk=renderer.render_into(frames,&mut mono[..frames as usize])?;
                        pipe.borrow_mut().send(&json!({"schema":"ql.native-act-owner-chunk/v1","instance_ref":request.instance_ref,
                            "request_id":request.request_id,"chunk":chunk}))?;
                    }
                    Some("finished")=>return Ok(()),
                    Some("abort")=>return Err("native C consumer refused output; original owner restitution required".into()),
                    _=>return Err("unknown native selected Act render consumer operation".into()),
                }
            }
        }).map(|rendered| {
            json!({"score":score,"selection":selected,"rendered":rendered.result.is_ok(),"restoration":rendered.restoration,
                "activity_error":rendered.result.err()})
        })
    })();
    let reply = host.native_act_result(&request.request_id, &result);
    pipe.borrow_mut().send(&reply)
}
