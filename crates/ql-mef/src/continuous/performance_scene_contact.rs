//! Contact on the existing native P/worker, under the actual closed C Scene.
//! No public command, imported source record or caller force can mint ingress.
use super::*;
use crate::continuous::performance_act_bridge::NativeActSourceLease;

pub(crate) const CONTACT_OWNER_REQUEST: &str = "ql.native-scene-contact-owner-request/v1";
const CONTACT_SOURCE: &str = "ql.native-scene-contact-source/v1";
const CONTACT_ADMISSION: &str = "ql.native-scene-contact-admission/v1";
const PART_LIMIT: usize = 4 * 1024 * 1024;
const SOURCE_LIMIT: usize = 8 * 1024 * 1024;

/// Private mint is reached only after the same C lease selects the full current
/// source, original sidecars, actual Scene constructor and authored definition.
/// No Deserialize/Clone/Default/public factory; serialization is a borrowed
/// transport observation, not construction authority.
pub(crate) struct QualifiedNativeSceneContactWorkerRequest {
    request: Value,
}
impl QualifiedNativeSceneContactWorkerRequest {
    pub(crate) fn request(&self) -> &Value {
        &self.request
    }
}

pub(super) struct PreparedNativeSceneContact {
    before_source_assets: Value,
    source: Value,
}
pub(crate) struct NativeSceneContactOperation<'a, 'lease> {
    pub(crate) lease: &'a NativeActSourceLease<'lease>,
    pub(crate) operation: &'a str,
    pub(crate) contact_ref: &'a str,
    pub(crate) scene_constructor: &'a Value,
    pub(crate) retained_reservation: &'a Value,
    pub(crate) origin: Option<&'a Value>,
}
pub(crate) struct NativeSceneContactRefusal {
    pub(crate) reason: String,
    pub(crate) native_pulse: Option<Value>,
    pub(crate) queue_committed: bool,
}
impl From<String> for NativeSceneContactRefusal {
    fn from(reason: String) -> Self {
        Self {
            reason,
            native_pulse: None,
            queue_committed: false,
        }
    }
}
impl From<&str> for NativeSceneContactRefusal {
    fn from(reason: &str) -> Self {
        reason.to_owned().into()
    }
}

fn exact_keys(value: &Value, names: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("native Contact object absent")?;
    if object.len() != names.len() || names.iter().any(|name| !object.contains_key(*name)) {
        return Err("native Contact fields missing or foreign".into());
    }
    Ok(())
}
fn encoded_bytes(value: &Value) -> Result<usize, String> {
    serde_json::to_vec(value)
        .map(|v| v.len())
        .map_err(|e| e.to_string())
}
fn reservation(value: &Value) -> Result<(usize, usize), String> {
    exact_keys(
        value,
        &[
            "schema",
            "source_record_bytes_limit",
            "native_pulse_bytes_limit",
        ],
    )?;
    if value["schema"] != "ql.native-contact-retention-reservation/v1" {
        return Err("native Contact retention reservation differs".into());
    }
    let source = decimal(&value["source_record_bytes_limit"])?;
    let pulse = decimal(&value["native_pulse_bytes_limit"])?;
    if source == 0 || pulse == 0 || source > PART_LIMIT as u64 || pulse > PART_LIMIT as u64 {
        return Err("native Contact retention exceeds unchanged native part bound".into());
    }
    Ok((source as usize, pulse as usize))
}

impl PerformanceOwner {
    /// Complete original native admissions, kept outside each numerical source
    /// bundle so its BEFORE epoch never recursively embeds previous pulses.
    pub fn native_contact_admission_history(&self) -> Vec<Value> {
        self.contact_admission_history.clone()
    }

    pub(crate) fn exchange_native_scene_contact(
        &mut self,
        current: &CoupledBasis,
        receiving: &super::super::performance_receiving::NativePerformanceReceivingSource,
        session: &mut CoupledFieldSession,
        command: NativeSceneContactOperation<'_, '_>,
    ) -> Result<Value, NativeSceneContactRefusal> {
        let NativeSceneContactOperation {
            lease,
            operation,
            contact_ref,
            scene_constructor,
            retained_reservation,
            origin,
        } = command;
        self.validate_current(current)?;
        if !matches!(
            operation,
            "contact-prepare" | "contact-apply" | "contact-trigger"
        ) {
            return Err("unknown private Scene Contact phase".into());
        }
        let instance = self.binding().determination()["identity"]["instance"]
            .as_str()
            .ok_or("native Contact instance absent")?
            .to_owned();
        let definition = lease.selected_scene_contact_definition(
            &instance,
            &self.source_assets,
            contact_ref,
            scene_constructor,
        )?;
        lease.validate_recorded_source_applications(
            &instance,
            &self.source_assets,
            &self.native_physical_source_history(),
            &self.native_acoustic_source_history(),
        )?;
        lease.validate_recorded_contact_admissions(
            &instance,
            &self.source_assets,
            &self.contact_admission_history,
        )?;
        let admitted_at = decimal(
            &self.source_assets["current_receiving"]["native_admission"]["operation"]["native_sample"],
        )?;
        let cursor = decimal(
            &self
                .reading()
                .ok_or("actual native Contact reading absent")?["samples_elapsed"],
        )?;
        let fresh = receiving.prepare_current(self, current, cursor)?;
        fresh.validate_current(receiving, self, current, cursor)?;
        if *receiving
            .prepare_retained(
                self,
                current,
                admitted_at,
                &self.source_assets["current_receiving"],
            )?
            .retained_snapshot()
            != self.source_assets["current_receiving"]
        {
            return Err("native Contact lost the complete original receiving owner".into());
        }
        let before_source_assets = self.source_assets.clone();
        let source_asset =
            lease.selected_full_scene_source_asset(&instance, &before_source_assets)?;
        let (source_limit, pulse_limit) = reservation(retained_reservation)?;
        // The C caller reserves the next complete asset/part before invoking
        // this route. Independently enforce that reservation against the actual
        // closed source and full BEFORE epoch before any native queue command.
        let prospective = encoded_bytes(source_asset)?
            .checked_add(encoded_bytes(&before_source_assets)?)
            .and_then(|n| n.checked_add(source_limit.checked_mul(2)?))
            .and_then(|n| n.checked_add(pulse_limit))
            .and_then(|n| n.checked_add(4096))
            .ok_or("native Contact full retention size exhausted")?;
        if prospective > SOURCE_LIMIT || prospective > PART_LIMIT {
            return Err(
                "next full native Contact source/sidecar cannot fit its existing asset/part".into(),
            );
        }
        let mut request = self.raw(operation)?;
        request["schema"] = json!(CONTACT_OWNER_REQUEST);
        request["scene_constructor"] = scene_constructor.clone();
        request["retention_reservation"] = retained_reservation.clone();
        let mut expected_prepared_source = None;
        if operation == "contact-apply" {
            let origin = origin.ok_or("original native Contact preparation absent")?;
            exact_keys(origin, &["original_native_request_id", "native_boundary"])?;
            let prepared = self
                .prepared_scene_contact
                .as_ref()
                .ok_or("actual same-owner native Contact candidate absent")?;
            if prepared.before_source_assets != before_source_assets
                || prepared.source["original_native_request_id"]
                    != origin["original_native_request_id"]
                || prepared.source["native_boundary"] != origin["native_boundary"]
                || prepared.source["authored_definition"] != *definition
                || decimal(&origin["original_native_request_id"])? >= lease.native_request_id()
            {
                return Err(
                    "native Contact preparation/source/ordinal changed before apply".into(),
                );
            }
            request["original_native_request_id"] = origin["original_native_request_id"].clone();
            request["native_boundary"] = origin["native_boundary"].clone();
            expected_prepared_source = Some(prepared.source.clone());
        } else {
            if origin.is_some() {
                return Err("native Contact origin belongs only to actual apply".into());
            }
            request["original_native_request_id"] = json!(lease.native_request_id().to_string());
            request["definition"] = definition.clone();
        }
        // Sole private mint. The lower FieldSession path takes THIS type, and
        // the ordinary performance_exchange_retained still refuses the schema.
        let qualified = QualifiedNativeSceneContactWorkerRequest { request };
        let pulse = match session.scene_contact_exchange_retained(&qualified) {
            Ok(pulse) => pulse,
            Err((reason, pulse)) => {
                let committed = pulse.as_ref().is_some_and(|p| {
                    p["payload"]["queue_committed"] == true || p["state_committed"] == true
                });
                let reason = if committed {
                    session.performance_invalidate(&reason)
                } else {
                    reason
                };
                if let Some(original) = &pulse {
                    self.last = Some(original.clone());
                }
                return Err(NativeSceneContactRefusal {
                    reason,
                    native_pulse: pulse,
                    queue_committed: committed,
                });
            }
        };
        let queued = pulse["payload"]["queue_committed"] == true;
        let source = if operation == "contact-prepare" {
            &pulse["payload"]["prepared_contact"]
        } else {
            &pulse["payload"]["contact_source"]
        };
        let checked = (|| -> Result<(), String> {
            self.validate_reply(&pulse)?;
            if pulse["operation"] != operation || !pulse["payload"]["queue_committed"].is_boolean()
            {
                return Err("native Contact original operation/queue receipt differs".into());
            }
            if pulse["accepted"] != true {
                return Err(pulse["reason"]
                    .as_str()
                    .unwrap_or("native Contact refused")
                    .to_owned());
            }
            if source["schema"] != CONTACT_SOURCE
                || source["authored_definition"] != *definition
                || source["original_native_request_id"]
                    != qualified.request()["original_native_request_id"]
                || source["occurrence"]["constructor_lineage"] != scene_constructor["instance_ref"]
                || source["occurrence"]["original_request_id"]
                    != source["original_native_request_id"]
                || source["force_newtons"].as_array().is_none_or(|v| {
                    v.len() != 512 || v.iter().any(|n| n.as_f64().is_none_or(|n| !n.is_finite()))
                })
                || expected_prepared_source
                    .as_ref()
                    .is_some_and(|old| old != source)
                || encoded_bytes(source)? > source_limit
                || encoded_bytes(&pulse)? > pulse_limit
            {
                return Err(
                    "native Contact lost complete original geometry/force/source/custody".into(),
                );
            }
            super::contact_history::validate_current_scene_contact_record(
                self,
                &before_source_assets,
                source,
            )?;
            if operation == "contact-prepare" {
                if queued || pulse["payload"]["prepared_only"] != true {
                    return Err("native Contact preparation changed the actual queue".into());
                }
            } else if !queued
                || pulse["payload"]["application_committed"] != false
                || pulse["payload"]["score_admission"]["schema"] != "ql.native-score-admission/v1"
                || pulse["payload"]["score_admission"]["queued"] != true
                || pulse["payload"]["score_admission"]["event"]["kind"] != 7
                || pulse["payload"]["score_admission"]["input_ref"] != contact_ref
            {
                return Err(
                    "native Contact queue admission is absent or a fabricated application".into(),
                );
            }
            Ok(())
        })();
        // Numerical qualification uses the full original BEFORE epoch. Only
        // then publish its successor. Every post-queue failure below retains
        // this original pulse and holds the actual worker; none becomes retry.
        let checked = checked.and_then(|()| {
            if queued {
                self.source_assets.as_object_mut().ok_or("native source object absent")?
                    .entry("contact_occurrence_history").or_insert_with(|| json!([]))
                    .as_array_mut().ok_or("native Contact source history malformed")?.push(source.clone());
                self.contact_admission_history.push(json!({"schema":CONTACT_ADMISSION,
                    "before_source_assets":before_source_assets,"source":source,"native_admission":pulse}));
            }
            Ok(())
        });
        self.last = Some(pulse.clone());
        if let Err(reason) = checked {
            let reason = if queued {
                session.performance_invalidate(&reason)
            } else {
                reason
            };
            return Err(NativeSceneContactRefusal {
                reason,
                native_pulse: Some(pulse),
                queue_committed: queued,
            });
        }
        if operation == "contact-prepare" {
            self.prepared_scene_contact = Some(PreparedNativeSceneContact {
                before_source_assets,
                source: source.clone(),
            });
        } else {
            self.prepared_scene_contact = None;
        }
        Ok(pulse)
    }
}
