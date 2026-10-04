//! Source-owned requirements over privately registered SAME native consumers.
//! Serialized contracts are configuration, never constructor/lease/ACK grants.
use crate::procedural_composition::{Result, TimingBinding};
use crate::procedural_conduct::{ConductInstall, NativePosition};
use crate::procedural_manifestation::{fingerprint, nonempty, validate_material};
use crate::procedural_source::NativeBootstrapSceneRead;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const CONSUMER_CONTRACT: &str = "ql.native-procedural-consumer-contract/v1";
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeConsumerRequirement {
    pub owner: String,
    pub instance_ref: String,
    pub required_generation: u64,
    pub generation_domain: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeConsumerContract {
    pub schema: String,
    pub expression_ref: String,
    pub scene_ref: String,
    pub document_revision: u64,
    pub material_fingerprint: String,
    pub source_read_receipt_ref: String,
    pub original_native_position: NativePosition,
    pub original_timing: TimingBinding,
    pub requirements: Vec<NativeConsumerRequirement>,
    pub required_consumers: BTreeSet<String>,
    pub constructor_basis_fingerprint: String,
}
/// C's actual selected Scene/config adoption owner supplies this privately.
/// No Deserialize/Default/public wire constructor. A Window lifetime alone is
/// insufficient: this fact must be minted by the actual registered Scene owner.
pub(crate) struct NativeSceneConsumerFact {
    reading_identity: Value,
    requirement: NativeConsumerRequirement,
    native_constructor_fact: Value,
}
impl NativeSceneConsumerFact {
    pub(crate) fn constructor_fact(&self) -> &Value {
        &self.native_constructor_fact
    }
    /// Called ONLY by C after actual Scene consumer registration and closed
    /// source/read comparison; public/captured JSON never calls this ingress.
    pub(crate) fn from_registered_scene(
        read: &NativeBootstrapSceneRead,
        instance_ref: String,
        construction_generation: u64,
        generation_domain: String,
        native_constructor_fact: Value,
    ) -> Result<Self> {
        read.validate()?;
        let requirement = NativeConsumerRequirement {
            owner: "scene".into(),
            instance_ref,
            required_generation: construction_generation,
            generation_domain,
        };
        validate_requirement(&requirement)?;
        validate_scene_constructor_configuration(read, &requirement, &native_constructor_fact)?;
        validate_material(&native_constructor_fact, 0)?;
        Ok(Self {
            reading_identity: scene_identity(read),
            requirement,
            native_constructor_fact,
        })
    }
}
/// Actual R timing constructor, separate from source FIELD/M2/M3 and epoch.
/// Performance consumes Source70's SAME v1 pulse and private Management
/// construction. FIELD borrows its original non-Serde native clock getter.
pub(crate) struct NativeTimingConsumerFact {
    binding: TimingBinding,
    position: NativePosition,
    requirement: NativeConsumerRequirement,
    native_constructor_fact: Value,
}
impl NativeTimingConsumerFact {
    pub(crate) fn constructor_fact(&self) -> &Value {
        &self.native_constructor_fact
    }
    /// R's genuine PreparedProceduralTimingDescriptor is the only caller.
    /// This comparison has no native exchange, clock or public JSON ingress.
    pub(crate) fn from_registered_performance_descriptor(
        binding: &TimingBinding,
        position: &NativePosition,
        pulse: &Value,
    ) -> Result<Self> {
        let row = &pulse["native_timing_owner"];
        let requirement = NativeConsumerRequirement {
            owner: binding.owner_ref.clone(),
            instance_ref: row["instance_ref"]
                .as_str()
                .ok_or("actual Management constructor absent")?
                .into(),
            required_generation: cursor(&row["construction_ordinal"])?,
            generation_domain: "native-management-construction".into(),
        };
        validate_performance_management_configuration(binding, position, &requirement, pulse)?;
        Ok(Self {
            binding: binding.clone(),
            position: position.clone(),
            requirement,
            native_constructor_fact: row.clone(),
        })
    }
}
impl NativeTimingConsumerFact {
    /// R-only same-receipt ingress. Every argument is borrowed from the actual
    /// PreparedFieldProceduralTiming and its non-Serde clock consumer fact.
    /// This performs no exchange/read or constructor; caller JSON never calls it.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_registered_field_clock(
        binding: &TimingBinding,
        position: &NativePosition,
        clock_instance_ref: &str,
        construction_generation: u64,
        generation_domain: &str,
        native_constructor_fact: &Value,
        native_receipt: &Value,
    ) -> Result<Self> {
        let requirement = NativeConsumerRequirement {
            owner: binding.owner_ref.clone(),
            instance_ref: clock_instance_ref.into(),
            required_generation: construction_generation,
            generation_domain: generation_domain.into(),
        };
        validate_field_clock_constructor_configuration(
            binding,
            position,
            &requirement,
            native_constructor_fact,
            native_receipt,
        )?;
        Ok(Self {
            binding: binding.clone(),
            position: position.clone(),
            requirement,
            native_constructor_fact: native_constructor_fact.clone(),
        })
    }
}
/// Transported correspondence only, not a FIELD/source/Scene lease. Positive
/// custody comes exclusively from R's non-Serde same-owner prepared getter.
fn validate_field_clock_constructor_configuration(
    binding: &TimingBinding,
    position: &NativePosition,
    requirement: &NativeConsumerRequirement,
    fact: &Value,
    receipt: &Value,
) -> Result<()> {
    validate_position(position)?;
    validate_requirement(requirement)?;
    nonempty(&binding.owner_ref, "native FIELD timing owner")?;
    nonempty(&binding.epoch_ref, "original native FIELD timing epoch")?;
    let keys = [
        "role",
        "owner_ref",
        "instance_ref",
        "construction_ordinal",
        "generation",
        "generation_domain",
        "sample",
        "source_instance_ref",
        "native_clock_constructor",
    ];
    let object = fact
        .as_object()
        .ok_or("native FIELD constructor fact absent")?;
    if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("complete SAME R FIELD constructor fact required".into());
    }
    let row = &fact["native_clock_constructor"];
    let row_keys = [
        "schema",
        "instance_ref",
        "construction_ordinal",
        "generation",
        "generation_domain",
        "clock_generation",
        "initial_clock_generation",
        "samples_elapsed",
        "event_ref",
        "subject_ref",
        "sample_rate",
    ];
    let object = row
        .as_object()
        .ok_or("native FIELD clock constructor row absent")?;
    if object.len() != row_keys.len() || row_keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("complete actual FIELD clock constructor row required".into());
    }
    let (nonce, ordinal) = requirement
        .instance_ref
        .strip_prefix("native-resident:v1:")
        .and_then(|value| value.split_once(':'))
        .ok_or("native FIELD clock constructor token malformed")?;
    let valid_nonce = nonce.len() == 32
        && nonce.bytes().any(|byte| byte != b'0')
        && nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    let clock_generation = cursor(&row["clock_generation"])?;
    cursor(&row["initial_clock_generation"])?;
    if binding.domain != "native_field_samples"
        || requirement.owner != binding.owner_ref
        || requirement.generation_domain != "native-field-clock-construction"
        || requirement.instance_ref == position.instance_ref
        || requirement.instance_ref == binding.owner_ref
        || !valid_nonce
        || ordinal != requirement.required_generation.to_string()
        || fact["role"] != "field_clock"
        || fact["owner_ref"] != requirement.owner
        || fact["instance_ref"] != requirement.instance_ref
        || cursor(&fact["construction_ordinal"])? != requirement.required_generation
        || cursor(&fact["generation"])? != requirement.required_generation
        || fact["generation_domain"] != requirement.generation_domain
        || cursor(&fact["sample"])? != position.cursor()?
        || fact["source_instance_ref"] != position.instance_ref
        || receipt["schema"] != "ql.continuous-field/v1"
        || receipt["event_ref"] != position.event_ref
        || receipt["subject_ref"] != position.subject_ref
        || receipt["generation"] != position.generation
        || receipt["samples_elapsed"] != position.samples_elapsed
        || &receipt["timing_owner"] != row
        || row["schema"] != "ql.native-field-clock-constructor/v1"
        || row["instance_ref"] != requirement.instance_ref
        || cursor(&row["construction_ordinal"])? != requirement.required_generation
        || cursor(&row["generation"])? != requirement.required_generation
        || row["generation_domain"] != requirement.generation_domain
        || cursor(&row["samples_elapsed"])? != position.cursor()?
        || row["event_ref"] != position.event_ref
        || row["subject_ref"] != position.subject_ref
        || row["sample_rate"] != receipt["sample_rate"]
        || row["sample_rate"].as_u64().is_none_or(|rate| rate == 0)
        || clock_generation != cursor(&receipt["clock"]["generation"])?
    {
        return Err(
            "FIELD constructor differs from SAME private native timing/source receipt".into(),
        );
    }
    // FIELD position.generation, clock_generation, constructor generation,
    // original epoch/mapping and native cursor have deliberately distinct roles.
    // R's private receipt/lease guard owns original constructor continuation.
    Ok(())
}
/// Configuration correspondence only. The actual C31 lease supplies this
/// observation from Application's current private SceneOwner. A matching Value
/// cannot construct that owner or qualify a closed reader.
fn validate_scene_constructor_configuration(
    read: &NativeBootstrapSceneRead,
    requirement: &NativeConsumerRequirement,
    fact: &Value,
) -> Result<()> {
    let keys = [
        "schema",
        "expression_ref",
        "scene_ref",
        "instance_ref",
        "construction_generation",
        "generation_domain",
        "initial_document_revision",
        "initial_document_sha256",
        "document_revision",
        "document_sha256",
    ];
    let object = fact
        .as_object()
        .ok_or("actual native Scene constructor fact absent")?;
    if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("complete actual native Document/Scene constructor fact required".into());
    }
    let generation = fact["construction_generation"]
        .as_u64()
        .filter(|v| *v > 0)
        .ok_or("actual Scene construction generation absent")?;
    let initial_revision = fact["initial_document_revision"]
        .as_u64()
        .filter(|v| *v > 0)
        .ok_or("actual initial Document revision absent")?;
    let instance = fact["instance_ref"]
        .as_str()
        .ok_or("actual Scene constructor instance absent")?;
    let (nonce, ordinal) = instance
        .strip_prefix("oi:document-scene:")
        .and_then(|rest| rest.split_once(':'))
        .ok_or("Scene consumer is not the actual native Document constructor")?;
    let nonce_valid = nonce.len() == 32
        && nonce
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    let digest_valid = |key: &str| {
        fact[key].as_str().is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    };
    if fact["schema"] != "oi.native-document-scene-constructor/v1"
        || fact["expression_ref"] != read.expression_ref
        || fact["scene_ref"] != read.scene_ref
        || fact["document_revision"] != read.document_revision
        || initial_revision > read.document_revision
        || fact["generation_domain"] != "native-document-scene-construction"
        || requirement.owner != "scene"
        || requirement.instance_ref != instance
        || requirement.required_generation != generation
        || requirement.generation_domain != "native-document-scene-construction"
        || !nonce_valid
        || ordinal != generation.to_string()
        || !digest_valid("initial_document_sha256")
        || !digest_valid("document_sha256")
    {
        return Err("actual Scene constructor/current Document correspondence differs".into());
    }
    // C31 compares document_sha256 to the SAME closed manifest's full current
    // Document. This bounded transported check cannot replace that owner read.
    Ok(())
}
fn scene_identity(read: &NativeBootstrapSceneRead) -> Value {
    json!({"expression_ref":read.expression_ref,"scene_ref":read.scene_ref,
        "document_revision":read.document_revision,"material_fingerprint":read.material_fingerprint,
        "source_read_receipt_ref":read.source_read_receipt_ref,"source_basis":read.source_basis,"locus":read.locus})
}
fn validate_requirement(r: &NativeConsumerRequirement) -> Result<()> {
    nonempty(&r.owner, "native consumer owner")?;
    nonempty(&r.instance_ref, "native consumer constructor instance")?;
    nonempty(&r.generation_domain, "native consumer constructor domain")?;
    if r.required_generation == 0
        || [
            "m2_generation",
            "m3_source_generation",
            "body_revision",
            "transport_epoch",
            "native_samples",
            "native_field_samples",
        ]
        .contains(&r.generation_domain.as_str())
    {
        return Err("consumer generation is not an actual native constructor lifetime".into());
    }
    Ok(())
}
fn validate_position(position: &NativePosition) -> Result<()> {
    nonempty(&position.instance_ref, "native position instance")?;
    nonempty(&position.event_ref, "native position event")?;
    nonempty(&position.subject_ref, "native position subject")?;
    cursor(&json!(position.generation))?;
    position.cursor()?;
    Ok(())
}
fn cursor(v: &Value) -> Result<u64> {
    let s = v.as_str().ok_or("native cursor must retain decimal text")?;
    let n = s
        .parse::<u64>()
        .map_err(|_| "invalid native cursor".to_owned())?;
    if n.to_string() != s {
        return Err("noncanonical native cursor".into());
    }
    Ok(n)
}
/// Compare copied SAME-pulse configuration without constructing any native
/// Source/lease/consumer authority. M2 remains part of the exact native source
/// identity; the position dates the independent committed M3 snapshot.
fn validate_generation_correspondence(pulse: &Value, position: &NativePosition) -> Result<()> {
    let registry = &pulse["resident_consumers"];
    let fact = &pulse["payload"]["timing_fact"];
    let source = &registry["source"];
    let physical = &registry["physical_observation"]["snapshot"];
    cursor(&source["m2_generation"])?;
    if source != &fact["source"]["identity"]
        || fact["native_position"] != json!(position)
        || physical["schema"] != "ql.native-physical-snapshot/v1"
        || cursor(&registry["m3_source_generation"])? != cursor(&json!(position.generation))?
        || physical["source_generation"] != registry["m3_source_generation"]
        || physical["source_generation"] != pulse["reading"]["physical"]["source_generation"]
        || physical["samples_elapsed"] != registry["sample"]
        || physical["event_ref"] != position.event_ref
        || physical["subject_ref"] != position.subject_ref
    {
        return Err(
            "native M2 identity or committed M3 snapshot differs from SAME timing pulse".into(),
        );
    }
    Ok(())
}
/// Current Source70 transported correspondence only. This does not construct
/// private Management/Scene ownership; only the genuine descriptor calls the
/// adapter above. Source position remains FIELD/M3, not this clock lifetime.
pub(crate) fn validate_performance_management_configuration(
    binding: &TimingBinding,
    position: &NativePosition,
    requirement: &NativeConsumerRequirement,
    pulse: &Value,
) -> Result<()> {
    validate_position(position)?;
    validate_requirement(requirement)?;
    validate_generation_correspondence(pulse, position)?;
    let row = &pulse["native_timing_owner"];
    let keys = [
        "schema",
        "role",
        "owner_ref",
        "instance_ref",
        "construction_ordinal",
        "generation",
        "generation_domain",
        "transport_epoch",
        "sample",
        "source",
        "m3_source_generation",
        "body_revision",
        "callback_output_committed",
        "time_mapping_ref",
    ];
    let object = row
        .as_object()
        .ok_or("actual Management constructor absent")?;
    if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("complete actual Source70 Management constructor required".into());
    }
    let fact = &pulse["payload"]["timing_fact"];
    let registry = &pulse["resident_consumers"];
    let rows = registry["required_consumers"]
        .as_array()
        .filter(|rows| (2..=3).contains(&rows.len()))
        .ok_or("actual A/P/(M4) resident roster absent")?;
    let (nonce, ordinal) = requirement
        .instance_ref
        .strip_prefix("native-resident:v1:")
        .and_then(|value| value.split_once(':'))
        .ok_or("actual Management constructor token malformed")?;
    let valid_nonce = nonce.len() == 32
        && nonce.bytes().any(|b| b != b'0')
        && nonce
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if pulse["schema"] != "ql.performance-worker-reply/v1"
        || pulse["accepted"] != true
        || fact["schema"] != "ql.native-performance-timing-fact/v1"
        || registry["schema"] != "ql.native-resident-consumer-registry/v1"
        || registry["origin"] != "actual-native-constructors-and-same-pulse"
        || row["schema"] != "ql.native-management-timing-owner/v1"
        || row["role"] != "timing_owner"
        || requirement.owner != binding.owner_ref
        || row["owner_ref"] != binding.owner_ref
        || row["owner_ref"] != pulse["reading"]["session_ref"]
        || row["instance_ref"] != requirement.instance_ref
        || requirement.instance_ref == position.instance_ref
        || requirement.instance_ref == binding.owner_ref
        || !valid_nonce
        || ordinal != requirement.required_generation.to_string()
        || cursor(&row["construction_ordinal"])? != requirement.required_generation
        || cursor(&row["generation"])? != requirement.required_generation
        || requirement.generation_domain != "native-management-construction"
        || row["generation_domain"] != requirement.generation_domain
        || binding.domain != "native_samples"
        || fact["binding"]["owner_ref"] != binding.owner_ref
        || fact["binding"]["domain"] != binding.domain
        || fact["binding"]["epoch_ref"] != binding.epoch_ref
        || fact["binding"]["time_mapping_ref"] != json!(binding.time_mapping_ref)
        || !row["time_mapping_ref"].is_null()
        || binding.time_mapping_ref.is_some()
        || row["sample"] != registry["sample"]
        || row["sample"] != position.samples_elapsed
        || row["sample"] != fact["committed_cursor"]
        || row["sample"] != pulse["reading"]["samples_elapsed"]
        || row["transport_epoch"] != registry["transport_epoch"]
        || row["transport_epoch"] != fact["transport_epoch"]
        || row["transport_epoch"] != pulse["reading"]["transport_epoch"]
        || binding.epoch_ref
            != format!(
                "ql:performance/transport-epoch/{}",
                cursor(&row["transport_epoch"])?
            )
        || row["source"] != registry["source"]
        || row["source"] != fact["source"]["identity"]
        || row["source"]["instance"] != position.instance_ref
        || row["source"]["event"] != position.event_ref
        || row["source"]["subject"] != position.subject_ref
        || row["m3_source_generation"] != registry["m3_source_generation"]
        || row["m3_source_generation"] != position.generation
        || row["body_revision"] != registry["body_revision"]
        || row["body_revision"] != pulse["reading"]["physical"]["body_revision"]
        || row["callback_output_committed"]
            != registry["audio_observation"]["callback_output_committed"]
        || row["callback_output_committed"].as_bool().is_none()
        || rows
            .iter()
            .any(|r| r["role"] == "timing_owner" || r["instance_ref"] == requirement.instance_ref)
        || cursor(&fact["binding"]["requested_cursor"])? != binding.requested_cursor
        || cursor(&fact["admission_horizon"])? != binding.requested_cursor
    {
        return Err(
            "Source70 Management lifetime/source/transport/same-pulse correspondence differs"
                .into(),
        );
    }
    Ok(())
}
/// Executed only INSIDE the same private native bootstrap/definition owner,
/// after the actual C Scene factory and R timing constructor facts exist.
/// Optional pulse is the actual already-returned SAME R descriptor pulse, never
/// an additional Inspect/clock exchange. Missing facts cannot be substituted.
pub(crate) fn from_registered_native_consumers(
    read: &NativeBootstrapSceneRead,
    position: &NativePosition,
    timing: &TimingBinding,
    scene: &NativeSceneConsumerFact,
    management: &NativeTimingConsumerFact,
    performance_pulse: Option<&Value>,
) -> Result<NativeConsumerContract> {
    read.validate()?;
    validate_position(position)?;
    if scene.reading_identity != scene_identity(read)
        || management.binding != *timing
        || management.position != *position
    {
        return Err("registered Scene/management consumer differs from exact current native Source boundary".into());
    }
    let mut requirements = vec![scene.requirement.clone(), management.requirement.clone()];
    let resident_basis = if timing.domain == "native_samples" {
        let pulse = performance_pulse.ok_or("SAME native performance resident registry absent")?;
        let registry = &pulse["resident_consumers"];
        let fact = &pulse["payload"]["timing_fact"];
        let source = &registry["source"];
        let physical = &registry["physical_observation"]["snapshot"];
        validate_generation_correspondence(pulse, position)?;
        validate_performance_management_configuration(
            timing,
            position,
            &management.requirement,
            pulse,
        )?;
        if management.native_constructor_fact != pulse["native_timing_owner"] {
            return Err("Management consumer fact differs from SAME descriptor pulse".into());
        }
        if fact["schema"] != "ql.native-performance-timing-fact/v1"
            || pulse["schema"] != "ql.performance-worker-reply/v1"
            || pulse["accepted"] != true
            || registry["schema"] != "ql.native-resident-consumer-registry/v1"
            || registry["origin"] != "actual-native-constructors-and-same-pulse"
            || pulse["reading"]["session_ref"] != timing.owner_ref
            || fact["binding"]["owner_ref"] != timing.owner_ref
            || fact["binding"]["domain"] != timing.domain
            || fact["binding"]["epoch_ref"] != timing.epoch_ref
            || fact["binding"]["time_mapping_ref"] != json!(timing.time_mapping_ref)
            || source["instance"] != json!(position.instance_ref)
            || source["event"] != position.event_ref
            || source["subject"] != position.subject_ref
            || cursor(&registry["sample"])? != position.cursor()?
            || registry["sample"] != pulse["reading"]["samples_elapsed"]
            || registry["transport_epoch"] != pulse["reading"]["transport_epoch"]
        {
            return Err("registered numeric residents are detached from the SAME native timing/source pulse".into());
        }
        let rows = registry["required_consumers"]
            .as_array()
            .filter(|r| (2..=3).contains(&r.len()))
            .ok_or("complete actual numeric consumer roster absent")?;
        let mut roles = BTreeSet::new();
        let mut instances = BTreeSet::new();
        for row in rows {
            let role = row["role"].as_str().ok_or("native resident role absent")?;
            let observed = &registry[match role {
                "audio_engine" => "audio_observation",
                "physical_body" => "physical_observation",
                "acoustic_receiving" => "receiving_observation",
                _ => return Err("unknown actual native resident role".into()),
            }];
            let instance = row["instance_ref"]
                .as_str()
                .ok_or("native resident constructor instance absent")?;
            if !roles.insert(role)
                || !instances.insert(instance)
                || row["generation_domain"] != "native-resident-construction"
                || row["sample"] != registry["sample"]
                || observed["sample"] != registry["sample"]
                || observed["instance_ref"] != instance
                || (role == "physical_body" && physical["resident_instance_ref"] != instance)
            {
                return Err(
                    "native resident constructor/role/cursor correspondence changed".into(),
                );
            }
            let requirement = NativeConsumerRequirement {
                owner: role.into(),
                instance_ref: instance.into(),
                required_generation: cursor(&row["generation"])?,
                generation_domain: "native-resident-construction".into(),
            };
            validate_requirement(&requirement)?;
            requirements.push(requirement);
        }
        if !roles.contains("audio_engine")
            || !roles.contains("physical_body")
            || roles.contains("acoustic_receiving") != !registry["receiving_observation"].is_null()
            || roles.contains("timing_owner")
        {
            return Err("actual native resident role omitted or fabricated".into());
        }
        registry.clone()
    } else if timing.domain == "native_field_samples" && performance_pulse.is_none() {
        if management.requirement.generation_domain != "native-field-clock-construction"
            || management.native_constructor_fact["role"] != "field_clock"
        {
            return Err("FIELD roster lacks its actual same-receipt clock constructor".into());
        }
        Value::Null
    } else {
        return Err("native consumer roster has wrong original owner domain".into());
    };
    let required_consumers = owners(&requirements)?;
    let basis = json!({"scene":scene.native_constructor_fact,"management":management.native_constructor_fact,"residents":resident_basis});
    Ok(NativeConsumerContract {
        schema: CONSUMER_CONTRACT.into(),
        expression_ref: read.expression_ref.clone(),
        scene_ref: read.scene_ref.clone(),
        document_revision: read.document_revision,
        material_fingerprint: read.material_fingerprint.clone(),
        source_read_receipt_ref: read.source_read_receipt_ref.clone(),
        original_native_position: position.clone(),
        original_timing: timing.clone(),
        requirements,
        required_consumers,
        constructor_basis_fingerprint: fingerprint(&basis)?,
    })
}
fn owners(requirements: &[NativeConsumerRequirement]) -> Result<BTreeSet<String>> {
    if !(2..=16).contains(&requirements.len()) {
        return Err("registered native consumer count absent or exceeded".into());
    }
    let mut owners = BTreeSet::new();
    let mut instances = BTreeSet::new();
    for r in requirements {
        validate_requirement(r)?;
        if !owners.insert(r.owner.clone()) || !instances.insert(r.instance_ref.as_str()) {
            return Err("duplicate or aliased native consumer owner/lifetime".into());
        }
    }
    if !owners.contains("scene") {
        return Err("actual registered scene role absent".into());
    }
    Ok(owners)
}
impl NativeConsumerContract {
    /// Pure current configuration correspondence; live origin is owned by
    /// the private C/R caller which supplies the constructor facts.
    pub(crate) fn validate_source_read(
        &self,
        read: &NativeBootstrapSceneRead,
        position: &NativePosition,
        timing: &TimingBinding,
    ) -> Result<()> {
        read.validate()?;
        validate_position(position)?;
        if self.schema != CONSUMER_CONTRACT
            || self.expression_ref != read.expression_ref
            || self.scene_ref != read.scene_ref
            || self.document_revision != read.document_revision
            || self.material_fingerprint != read.material_fingerprint
            || self.source_read_receipt_ref != read.source_read_receipt_ref
            || self.original_native_position != *position
            || self.original_timing != *timing
            || self.required_consumers != owners(&self.requirements)?
            || !self
                .requirements
                .iter()
                .any(|r| r.owner == timing.owner_ref)
            || self.constructor_basis_fingerprint.is_empty()
        {
            return Err(
                "original native consumer contract differs from current Source read/boundary"
                    .into(),
            );
        }
        Ok(())
    }
    /// Pure transported configuration check only. Genuine Source origin and
    /// current native owner facts remain private to the producing host.
    pub fn validate_definition(&self, definition: &ConductInstall) -> Result<()> {
        if self.schema != CONSUMER_CONTRACT
            || self.expression_ref != definition.expression_ref
            || self.document_revision != definition.document_revision
            || self.original_timing != definition.procedure.timing
            || self.required_consumers != owners(&self.requirements)?
            || definition.required_consumers != self.required_consumers
            || !self
                .requirements
                .iter()
                .any(|r| r.owner == self.original_timing.owner_ref)
            || self.constructor_basis_fingerprint.is_empty()
        {
            return Err(
                "definition changed original Source-owned registered consumer contract".into(),
            );
        }
        Ok(())
    }
}
/// Assemble BEFORE install/compilation/fingerprint, never append roles after a
/// PreparedProcedure is produced. Root subsequently re-attests full original
/// Source receipt and private receiving constructor facts; this is no ACK.
pub fn bind_definition_consumers(
    mut definition: ConductInstall,
    contract: &NativeConsumerContract,
) -> Result<ConductInstall> {
    if !definition.required_consumers.is_empty()
        && definition.required_consumers != contract.required_consumers
    {
        return Err("caller consumer roles differ from actual Source-owned native roster".into());
    }
    definition.required_consumers = contract.required_consumers.clone();
    contract.validate_definition(&definition)?;
    Ok(definition)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires the genuine private C31 SceneOwner/Source bootstrap capture"]
    fn actual_scene_constructor_configuration_preserves_original_lifetime_and_current_cas() {
        let path = std::env::var("QL_PROCEDURAL_CONSUMER_CONTRACT_ARTIFACT")
            .expect("actual private native consumer capture required");
        let captured: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-procedural-consumer-contract-capture/v1"
        );
        let input: crate::procedural_source::NativeSourceBootstrap =
            serde_json::from_value(captured["source_input"].clone()).unwrap();
        let contract: NativeConsumerContract =
            serde_json::from_value(captured["source_result"]["consumer_contract"].clone()).unwrap();
        let requirement = contract
            .requirements
            .iter()
            .find(|r| r.owner == "scene")
            .unwrap();
        // Root's actual registered Application SceneOwner supplies this capture
        // row; a retained JSON row is tested as configuration, never authority.
        let fact = &captured["native_scene_constructor_fact"];
        let before = fact.clone();
        validate_scene_constructor_configuration(&input.scene, requirement, fact).unwrap();
        for (key, value) in [
            ("schema", json!("oi.native-window-constructor/v1")),
            ("expression_ref", json!("foreign:expression")),
            ("scene_ref", json!("foreign:scene")),
            (
                "document_revision",
                json!(input.scene.document_revision.checked_add(1).unwrap()),
            ),
            (
                "construction_generation",
                json!(requirement.required_generation.checked_add(1).unwrap()),
            ),
            ("generation_domain", json!("native-window-construction")),
            ("instance_ref", json!("window:caller-name")),
            (
                "initial_document_revision",
                json!(input.scene.document_revision.checked_add(1).unwrap()),
            ),
            ("document_sha256", json!("caller-hash")),
        ] {
            let mut wrong = fact.clone();
            wrong[key] = value;
            assert!(
                validate_scene_constructor_configuration(&input.scene, requirement, &wrong)
                    .is_err(),
                "{key}"
            );
        }
        let mut missing = fact.clone();
        missing
            .as_object_mut()
            .unwrap()
            .remove("initial_document_sha256");
        assert!(
            validate_scene_constructor_configuration(&input.scene, requirement, &missing).is_err()
        );
        assert_eq!(fact, &before);
        // No private Scene/Management fact, lease, witness or ACK is minted.
    }
    #[test]
    fn typed_configuration_does_not_alias_source_or_clock_generations_to_constructor_lifetimes() {
        for domain in [
            "m2_generation",
            "m3_source_generation",
            "body_revision",
            "transport_epoch",
            "native_samples",
            "native_field_samples",
        ] {
            let row = NativeConsumerRequirement {
                owner: "physical_body".into(),
                instance_ref: "native-resident:recorded-instance".into(),
                required_generation: 1,
                generation_domain: domain.into(),
            };
            assert!(validate_requirement(&row).is_err(), "accepted {domain}");
        }
        let malformed = NativeConsumerRequirement {
            owner: String::new(),
            instance_ref: String::new(),
            required_generation: 0,
            generation_domain: String::new(),
        };
        assert!(validate_requirement(&malformed).is_err());
    }
    #[test]
    #[ignore = "requires actual closed C Scene adoption + R management/resident same-pulse bootstrap capture; no native fact mocks"]
    fn actual_registered_bootstrap_contract_contains_timing_management_and_every_real_numeric_role()
    {
        let path = std::env::var("QL_PROCEDURAL_CONSUMER_CONTRACT_ARTIFACT")
            .expect("actual private owner capture required");
        let captured: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-procedural-consumer-contract-capture/v1"
        );
        let contract: NativeConsumerContract =
            serde_json::from_value(captured["source_result"]["consumer_contract"].clone()).unwrap();
        let definition: ConductInstall =
            serde_json::from_value(captured["original_definition"].clone()).unwrap();
        contract.validate_definition(&definition).unwrap();
        assert!(
            contract
                .required_consumers
                .contains(&contract.original_timing.owner_ref)
        );
        let timing = contract
            .requirements
            .iter()
            .find(|r| r.owner == contract.original_timing.owner_ref)
            .unwrap();
        if contract.original_timing.domain == "native_samples" {
            assert_eq!(
                captured["source_result"]["native_timing_pulse"]["payload"]["timing_fact"]["schema"],
                "ql.native-performance-timing-fact/v1"
            );
            assert_eq!(timing.generation_domain, "native-management-construction");
            assert_ne!(
                timing.instance_ref, contract.original_native_position.instance_ref,
                "Source FIELD identity is distinct from actual Management construction"
            );
            assert_ne!(timing.instance_ref, contract.original_timing.owner_ref);
        }
        assert!(contract.requirements.iter().any(|r| r.owner == "scene"));
        if contract.original_timing.domain == "native_samples" {
            let registry = &captured["source_result"]["native_timing_pulse"]["resident_consumers"];
            for actual in registry["required_consumers"].as_array().unwrap() {
                let matching = contract
                    .requirements
                    .iter()
                    .find(|r| json!(r.owner) == actual["role"])
                    .unwrap();
                assert_eq!(json!(matching.instance_ref), actual["instance_ref"]);
                assert_eq!(
                    matching.required_generation,
                    cursor(&actual["generation"]).unwrap()
                );
                assert_eq!(
                    json!(matching.generation_domain),
                    actual["generation_domain"]
                );
            }
        }
        for removed in contract.required_consumers.iter() {
            let mut wrong = definition.clone();
            wrong.required_consumers.remove(removed);
            assert!(
                contract.validate_definition(&wrong).is_err(),
                "omitted actual native owner {removed}"
            );
        }
        let mut wrong = definition.clone();
        wrong.required_consumers.insert("nativeBody".into());
        assert!(
            contract.validate_definition(&wrong).is_err(),
            "fixture alias cannot replace actual registered role"
        );
        let mut wrong = definition.clone();
        wrong.procedure.timing.epoch_ref = "foreign:epoch".into();
        assert!(contract.validate_definition(&wrong).is_err());
        // Retained capture validation creates no NativeSceneConsumerFact,
        // NativeTimingConsumerFact, private lease or native live authority.
    }
    #[test]
    #[ignore = "requires genuine SAME private native timing/resident bootstrap capture; no M2/M3 mock"]
    fn actual_same_pulse_keeps_m2_identity_and_m3_position_generation_separate() {
        let path = std::env::var("QL_PROCEDURAL_CONSUMER_CONTRACT_ARTIFACT")
            .expect("actual private owner capture required");
        let captured: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-procedural-consumer-contract-capture/v1"
        );
        let contract: NativeConsumerContract =
            serde_json::from_value(captured["source_result"]["consumer_contract"].clone()).unwrap();
        assert_eq!(contract.original_timing.domain, "native_samples");
        let pulse = &captured["source_result"]["native_timing_pulse"];
        let before = pulse.clone();
        validate_generation_correspondence(pulse, &contract.original_native_position).unwrap();
        for path in [
            vec!["resident_consumers", "m3_source_generation"],
            vec![
                "resident_consumers",
                "physical_observation",
                "snapshot",
                "source_generation",
            ],
            vec!["reading", "physical", "source_generation"],
            vec!["payload", "timing_fact", "native_position", "generation"],
        ] {
            let mut wrong = pulse.clone();
            let mut value = &mut wrong;
            for key in path {
                value = &mut value[key];
            }
            let original = cursor(value).unwrap();
            *value = json!((original.checked_add(1).unwrap()).to_string());
            assert!(
                validate_generation_correspondence(&wrong, &contract.original_native_position)
                    .is_err()
            );
        }
        let mut wrong = pulse.clone();
        let m2 = cursor(&wrong["resident_consumers"]["source"]["m2_generation"]).unwrap();
        wrong["resident_consumers"]["source"]["m2_generation"] =
            json!((m2.checked_add(1).unwrap()).to_string());
        assert!(
            validate_generation_correspondence(&wrong, &contract.original_native_position).is_err()
        );
        assert_eq!(pulse, &before);
        // No copied observation can construct private native consumer facts.
    }
    #[test]
    #[ignore = "requires actual private C31 Scene + R FIELD clock consumer bootstrap capture"]
    fn actual_field_clock_roster_preserves_independent_native_constructor_and_source_domains() {
        let path = std::env::var("QL_PROCEDURAL_FIELD_CLOCK_CONSUMER_ARTIFACT")
            .expect("actual SAME owner FIELD capture required");
        let captured: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-procedural-consumer-contract-capture/v1"
        );
        let contract: NativeConsumerContract =
            serde_json::from_value(captured["source_result"]["consumer_contract"].clone()).unwrap();
        let definition: ConductInstall =
            serde_json::from_value(captured["original_definition"].clone()).unwrap();
        let requirement = contract
            .requirements
            .iter()
            .find(|row| row.owner == contract.original_timing.owner_ref)
            .unwrap();
        let fact = &captured["native_field_clock_consumer_fact"];
        let receipt = &captured["source_result"]["native_field_receipt"];
        let before = captured.clone();
        assert_eq!(contract.original_timing.domain, "native_field_samples");
        assert_eq!(
            requirement.generation_domain,
            "native-field-clock-construction"
        );
        assert_eq!(contract.requirements.len(), 2);
        assert!(contract.required_consumers.contains("scene"));
        assert!(
            contract
                .required_consumers
                .contains(&contract.original_timing.owner_ref)
        );
        assert_eq!(
            contract.required_consumers,
            BTreeSet::from([
                "scene".to_owned(),
                contract.original_timing.owner_ref.clone()
            ])
        );
        assert!(
            captured["source_result"]
                .get("native_timing_pulse")
                .is_none()
        );
        contract.validate_definition(&definition).unwrap();
        validate_field_clock_constructor_configuration(
            &contract.original_timing,
            &contract.original_native_position,
            requirement,
            fact,
            receipt,
        )
        .unwrap();
        for (path, value) in [
            (vec!["role"], json!("timing_owner")),
            (vec!["owner_ref"], json!("foreign:owner")),
            (vec!["source_instance_ref"], json!(requirement.instance_ref)),
            (vec!["generation_domain"], json!("native_field_samples")),
            (
                vec!["native_clock_constructor", "schema"],
                json!("ql.native-window-constructor/v1"),
            ),
            (
                vec!["native_clock_constructor", "event_ref"],
                json!("foreign:event"),
            ),
            (
                vec!["native_clock_constructor", "subject_ref"],
                json!("foreign:subject"),
            ),
        ] {
            let mut wrong = fact.clone();
            let mut leaf = &mut wrong;
            for key in path {
                leaf = &mut leaf[key];
            }
            *leaf = value;
            assert!(
                validate_field_clock_constructor_configuration(
                    &contract.original_timing,
                    &contract.original_native_position,
                    requirement,
                    &wrong,
                    receipt
                )
                .is_err()
            );
        }
        for path in [
            vec!["construction_ordinal"],
            vec!["generation"],
            vec!["sample"],
            vec!["native_clock_constructor", "construction_ordinal"],
            vec!["native_clock_constructor", "generation"],
            vec!["native_clock_constructor", "clock_generation"],
            vec!["native_clock_constructor", "samples_elapsed"],
        ] {
            let mut wrong = fact.clone();
            let mut leaf = &mut wrong;
            for key in path {
                leaf = &mut leaf[key];
            }
            let original = cursor(leaf).unwrap();
            *leaf = json!(original.checked_add(1).unwrap().to_string());
            assert!(
                validate_field_clock_constructor_configuration(
                    &contract.original_timing,
                    &contract.original_native_position,
                    requirement,
                    &wrong,
                    receipt
                )
                .is_err()
            );
        }
        let mut missing = receipt.clone();
        missing.as_object_mut().unwrap().remove("timing_owner");
        assert!(
            validate_field_clock_constructor_configuration(
                &contract.original_timing,
                &contract.original_native_position,
                requirement,
                fact,
                &missing
            )
            .is_err()
        );
        let mut wrong = requirement.clone();
        wrong.instance_ref = contract.original_native_position.instance_ref.clone();
        assert!(
            validate_field_clock_constructor_configuration(
                &contract.original_timing,
                &contract.original_native_position,
                &wrong,
                fact,
                receipt
            )
            .is_err()
        );
        assert_eq!(captured, before);
        // This copied configuration check constructs no private clock/Scene
        // fact, lease, witness, Source authority or output ACK.
    }
    /// Actual C31 capture only. This replays configured original compiler output
    /// and inspects recorded real owner ingress; it cannot mint a native fact.
    #[test]
    #[ignore = "requires genuine private current-Scene bootstrap caller capture in all four branches"]
    fn actual_factory_caller_retains_one_original_read_and_native_roster_before_definition() {
        use sha2::{Digest, Sha256};
        let path = std::env::var("QL_PROCEDURAL_SOURCE_FACTORY_CALLER_ARTIFACT")
            .expect("genuine native caller capture required");
        let captured: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-source-factory-caller-capture/v1"
        );
        let cases = captured["cases"].as_array().unwrap();
        assert_eq!(
            cases.len(),
            4,
            "every genuine field and performance branch is required"
        );
        let mut branches = BTreeSet::new();
        for case in cases {
            let branch = case["branch"].as_str().unwrap();
            assert!(branches.insert(branch));
            let input: crate::procedural_source::NativeSourceBootstrap =
                serde_json::from_value(case["source_input"].clone()).unwrap();
            let result = &case["source_result"];
            let contract: NativeConsumerContract =
                serde_json::from_value(result["consumer_contract"].clone()).unwrap();
            let definition: ConductInstall =
                serde_json::from_value(case["definition"].clone()).unwrap();
            contract
                .validate_source_read(
                    &input.scene,
                    &contract.original_native_position,
                    &contract.original_timing,
                )
                .unwrap();
            contract.validate_definition(&definition).unwrap();
            let empty_definition = ConductInstall {
                required_consumers: BTreeSet::new(),
                ..definition.clone()
            };
            let bound = bind_definition_consumers(empty_definition, &contract).unwrap();
            assert_eq!(bound.required_consumers, definition.required_consumers);
            let request = &case["native_request"];
            assert_eq!(request["mode"], "source-bootstrap");
            assert_eq!(
                request["scene_consumer"],
                result["native_scene_constructor_fact"]
            );
            assert_eq!(request["source_bootstrap"].as_object().unwrap().len(), 2);
            assert_eq!(request["source_bootstrap"]["reading"], json!(input.scene));
            assert_eq!(
                request["source_bootstrap"]["issuer_receipt"]["original_intent"]["authorship"],
                json!(input.authorship)
            );
            let digest = request["manifest"]["expanded_document_sha256"]
                .as_str()
                .unwrap();
            assert_eq!(
                result["native_scene_constructor_fact"]["document_sha256"],
                digest.strip_prefix("sha256:").unwrap()
            );
            // Retain original real request/result bytes. Bare success labels,
            // generated facts and empty case arrays do not satisfy this gate.
            for (name, parsed) in [("request", request), ("reply", &case["native_reply"])] {
                let raw = &case[format!("raw_{name}")];
                let bytes = std::fs::read(raw["path"].as_str().unwrap()).unwrap();
                assert_eq!(raw["sha256"], format!("{:x}", Sha256::digest(&bytes)));
                let original: Value = serde_json::from_slice(&bytes).unwrap();
                assert_eq!(&original, parsed);
            }
            let reply = &case["native_reply"];
            let receipt = &reply["result"]["native_receipt"];
            assert_eq!(reply["schema"], "ql.native-act-owner-result/v1");
            assert_eq!(reply["status"], "ok");
            assert_eq!(receipt["status"], "ok");
            assert_eq!(&receipt["procedural"], result);
            assert_eq!(reply["request_id"], request["request_id"]);
            assert_eq!(reply["last_request_id"], request["request_id"]);
            assert_eq!(receipt["request_id"], request["request_id"]);
            assert_eq!(receipt["last_request_id"], request["request_id"]);
            assert_eq!(
                cursor(&request["request_id"]).unwrap(),
                cursor(&case["before_host_receipt"]["last_request_id"])
                    .unwrap()
                    .checked_add(1)
                    .unwrap()
            );
            let ingress = case["native_ingress"].as_array().unwrap();
            assert!(
                !ingress.is_empty(),
                "actual qualified owner ingress required"
            );
            let field = branch.starts_with("field-");
            let descriptor_rows: Vec<_> = ingress
                .iter()
                .filter(|row| row["owner"] == if field { "field" } else { "performance" })
                .collect();
            assert_eq!(
                descriptor_rows.len(),
                1,
                "descriptor must retain one original native observation"
            );
            let original_position = &contract.original_native_position;
            let constructor = &result["native_scene_constructor_fact"];
            let scene_requirement = contract
                .requirements
                .iter()
                .find(|r| r.owner == "scene")
                .unwrap();
            validate_scene_constructor_configuration(&input.scene, scene_requirement, constructor)
                .unwrap();
            let timing_requirement = contract
                .requirements
                .iter()
                .find(|r| r.owner == contract.original_timing.owner_ref)
                .unwrap();
            if field {
                assert_eq!(contract.original_timing.domain, "native_field_samples");
                assert_eq!(contract.requirements.len(), 2);
                assert!(result.get("native_timing_pulse").is_none());
                let field_receipt = &result["native_field_receipt"];
                assert_eq!(descriptor_rows[0]["request"]["operation"], "read");
                assert_eq!(&descriptor_rows[0]["reply"], field_receipt);
                assert_eq!(field_receipt["audio"].as_array().unwrap().len(), 0);
                let mut before = case["before_host_receipt"]["field"].clone();
                before["audio"] = json!([]);
                assert_eq!(
                    &before, field_receipt,
                    "same FIELD boundary; no extra advancement"
                );
                validate_field_clock_constructor_configuration(
                    &contract.original_timing,
                    original_position,
                    timing_requirement,
                    &result["native_timing_consumer_fact"],
                    field_receipt,
                )
                .unwrap();
            } else {
                assert_eq!(contract.original_timing.domain, "native_samples");
                assert!(result["native_field_receipt"].is_null());
                let pulse = &result["native_timing_pulse"];
                assert_eq!(descriptor_rows[0]["request"]["operation"], "timing");
                assert_eq!(descriptor_rows[0]["request"]["moment"], "boundary");
                assert_eq!(&descriptor_rows[0]["reply"], pulse);
                validate_performance_management_configuration(
                    &contract.original_timing,
                    original_position,
                    timing_requirement,
                    pulse,
                )
                .unwrap();
                assert_eq!(
                    pulse["payload"]["timing_fact"]["device_callbacks_running"],
                    branch == "performance-sounding"
                );
                assert!(contract.required_consumers.contains("audio_engine"));
                assert!(contract.required_consumers.contains("physical_body"));
            }
            let observed = crate::procedural_source::NativeBootstrapObservation {
                position: original_position.clone(),
                timing: contract.original_timing.clone(),
                field_source: result["native_field_source"].clone(),
                field_receipt: result["native_field_receipt"].clone(),
                native_timing_pulse: result.get("native_timing_pulse").cloned(),
                native_act_source: result["native_act_source"].clone(),
            };
            let resident_basis = result
                .get("native_timing_pulse")
                .map(|pulse| pulse["resident_consumers"].clone())
                .unwrap_or(Value::Null);
            assert_eq!(
                contract.constructor_basis_fingerprint,
                fingerprint(&json!({"scene":result["native_scene_constructor_fact"],
                    "management":result["native_timing_consumer_fact"],
                    "residents":resident_basis}))
                .unwrap(),
                "full original private getter diagnostic bytes must agree"
            );
            let compiled = crate::procedural_source::compile_native_source_bootstrap_registered(
                &input, &observed, &contract,
            )
            .unwrap();
            assert_eq!(compiled["consumer_contract"], result["consumer_contract"]);
            assert_eq!(compiled["source_composition"], result["source_composition"]);
            assert_eq!(compiled["binding"], result["binding"]);
            for mutation in [
                "document", "material", "read", "source", "subject", "prime", "owner", "roles",
            ] {
                let mut wrong = contract.clone();
                match mutation {
                    "document" => wrong.document_revision += 1,
                    "material" => wrong.material_fingerprint = "foreign:material".into(),
                    "read" => wrong.source_read_receipt_ref = "foreign:read".into(),
                    "source" => {
                        wrong.original_native_position.instance_ref = "foreign:source".into()
                    }
                    "subject" => {
                        wrong.original_native_position.subject_ref = "foreign:subject".into()
                    }
                    "prime" => wrong
                        .original_native_position
                        .subject_ref
                        .push_str("-foreign-prime"),
                    "owner" => wrong.original_timing.owner_ref = "foreign:owner".into(),
                    "roles" => {
                        wrong.required_consumers.remove("scene");
                    }
                    _ => unreachable!(),
                }
                assert!(
                    crate::procedural_source::compile_native_source_bootstrap_registered(
                        &input, &observed, &wrong
                    )
                    .is_err(),
                    "accepted {mutation}"
                );
            }
            // Never relax post-adoption CAS or add participants to a completed
            // prepared result; a genuinely fresh no-write Source read is required.
            let mut stale = definition.clone();
            stale.document_revision += 1;
            assert!(contract.validate_definition(&stale).is_err());
        }
        assert_eq!(
            branches,
            BTreeSet::from([
                "field-paused",
                "field-running",
                "performance-prepared-held",
                "performance-sounding"
            ])
        );
        let failures = captured["post_source_refusals"].as_array().unwrap();
        assert!(
            !failures.is_empty(),
            "genuine post-reader refusal capture required"
        );
        let mut boundaries = BTreeSet::new();
        for failure in failures {
            let boundary = failure["boundary"].as_str().unwrap();
            boundaries.insert(boundary);
            let request = &failure["native_request"];
            let reply = &failure["native_reply"];
            for (name, parsed) in [("request", request), ("reply", reply)] {
                let raw = &failure[format!("raw_{name}")];
                let bytes = std::fs::read(raw["path"].as_str().unwrap()).unwrap();
                assert_eq!(raw["sha256"], format!("{:x}", Sha256::digest(&bytes)));
                assert_eq!(&serde_json::from_slice::<Value>(&bytes).unwrap(), parsed);
            }
            assert_eq!(reply["schema"], "ql.native-act-owner-result/v1");
            assert_eq!(reply["request_id"], request["request_id"]);
            assert_eq!(reply["last_request_id"], request["request_id"]);
            let actual = &reply["result"]["native_receipt"];
            assert_eq!(actual["last_request_id"], request["request_id"]);
            let retained = match boundary {
                "host-source-postcondition" => {
                    assert_eq!(actual["status"], "refused");
                    assert!(actual.get("procedural").is_none());
                    actual
                        .get("native_field_timing_receipt")
                        .or_else(|| actual.get("native_timing_pulse"))
                        .unwrap()
                }
                "c-scene-postcondition" => {
                    let refusal = &failure["channel_refusal"];
                    assert_eq!(
                        refusal["schema"],
                        "oi.native-scene-source-channel-refusal/v1"
                    );
                    assert_eq!(refusal["accepted"], false);
                    assert!(!refusal["reason"].as_str().unwrap().is_empty());
                    assert_eq!(&refusal["native_reply"], reply);
                    // C postvalidation must retain this actual successful
                    // native reply rather than fabricate a refused Host ACK.
                    assert_eq!(actual["status"], "ok");
                    let result = &actual["procedural"];
                    assert_eq!(result["schema"], crate::procedural_source::SOURCE_BOOTSTRAP);
                    result
                        .get("native_field_receipt")
                        .filter(|value| !value.is_null())
                        .or_else(|| result.get("native_timing_pulse"))
                        .unwrap()
                }
                _ => panic!("foreign post-reader failure boundary"),
            };
            assert_eq!(retained, &failure["original_timing_observation"]);
        }
        assert_eq!(
            boundaries,
            BTreeSet::from(["host-source-postcondition", "c-scene-postcondition"])
        );
        // Original real capture only; no constructor/lease/mock ACK injection.
    }
}

#[cfg(test)]
mod source70_actual_capture_tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::path::Path;

    fn raw(base: &Path, reference: &Value) -> Value {
        let bytes = std::fs::read(base.join(reference["path"].as_str().unwrap())).unwrap();
        assert!(!bytes.is_empty() && bytes.len() <= 64 * 1024 * 1024);
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), reference["sha256"]);
        serde_json::from_slice(&bytes).unwrap()
    }

    #[test]
    #[ignore = "requires real adopted Source70 same-private-owner capture and independently trusted joined inventory"]
    fn actual_source70_keeps_management_construction_source_m3_and_restore_epoch_distinct() {
        let file = std::path::PathBuf::from(
            std::env::var("QL_PROCEDURAL_SOURCE70_ACTUAL_CAPTURE").unwrap(),
        );
        let captured: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        let base = file.parent().unwrap();
        assert_eq!(
            captured["schema"],
            "ql.native-procedural-source70-capture/v1"
        );
        let trusted =
            std::fs::read(std::env::var("OI_STAGE_PASSAGE_SOURCE_MANIFEST").unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&trusted)),
            std::env::var("OI_STAGE_PASSAGE_SOURCE_MANIFEST_SHA256").unwrap()
        );
        let manifest: Value = serde_json::from_slice(&trusted).unwrap();
        assert_eq!(manifest["running_revisions"], captured["running_revisions"]);
        assert_eq!(
            manifest["ql"]["inventory"]["schema"],
            "ql.m-current-inventory/v1"
        );
        let actual_compiled_module = format!(
            "{:x}",
            Sha256::digest(include_bytes!("procedural_consumers.rs"))
        );
        assert_eq!(
            manifest["ql"]["inventory"]["sources"]["crates/ql-mef/src/procedural_consumers.rs"],
            actual_compiled_module,
            "earlier self-consistent code cannot certify this actual adapter"
        );
        for pin in captured["source_pins"].as_array().unwrap() {
            let bytes = std::fs::read(base.join(pin["path"].as_str().unwrap())).unwrap();
            assert_eq!(format!("{:x}", Sha256::digest(&bytes)), pin["sha256"]);
            let inventory = if pin["owner"] == "ql" {
                &manifest["ql"]["inventory"]["sources"]
            } else {
                &manifest["oi"]["sources"]
            };
            assert_eq!(inventory[pin["repo_path"].as_str().unwrap()], pin["sha256"]);
        }
        let execution_bytes =
            std::fs::read(std::env::var("QL_PROCEDURAL_SOURCE70_ACTUAL_EXECUTION").unwrap())
                .unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&execution_bytes)),
            std::env::var("QL_PROCEDURAL_SOURCE70_ACTUAL_EXECUTION_SHA256").unwrap()
        );
        assert_eq!(
            captured["private_owner_execution"]["sha256"],
            format!("{:x}", Sha256::digest(&execution_bytes))
        );
        let execution = raw(base, &captured["private_owner_execution"]);
        assert_eq!(
            execution,
            serde_json::from_slice::<Value>(&execution_bytes).unwrap()
        );
        assert_eq!(
            execution["schema"],
            "oi.actual-native-source70-execution/v1"
        );
        assert_eq!(
            execution["running_revisions"],
            captured["running_revisions"]
        );
        assert_eq!(execution["entries"], captured["entries"]);
        let entries = captured["entries"].as_array().unwrap();
        assert_eq!(entries.len(), 3);
        let mut phases = BTreeSet::new();
        let mut original_management = None;
        let mut original_epoch = None;
        for entry in entries {
            let phase = entry["phase"].as_str().unwrap();
            assert!(phases.insert(phase));
            let request = raw(base, &entry["raw_native_owner_request"]);
            let reply = raw(base, &entry["raw_native_owner_reply"]);
            assert_eq!(request["mode"], "source-bootstrap");
            assert_eq!(reply["schema"], "ql.native-act-owner-result/v1");
            assert_eq!(reply["status"], "ok");
            assert_eq!(reply["request_id"], request["request_id"]);
            assert_eq!(reply["last_request_id"], request["request_id"]);
            let host = &reply["result"]["native_receipt"];
            assert_eq!(host["schema"], "ql.field-host-receipt/v1");
            assert_eq!(host["status"], "ok");
            assert_eq!(host["last_request_id"], request["request_id"]);
            let result = &host["procedural"];
            assert_eq!(result, &entry["source_result"]);
            assert_eq!(result["schema"], crate::procedural_source::SOURCE_BOOTSTRAP);
            let contract: NativeConsumerContract =
                serde_json::from_value(result["consumer_contract"].clone()).unwrap();
            let input: crate::procedural_source::NativeSourceBootstrap =
                serde_json::from_value(entry["source_input"].clone()).unwrap();
            assert_eq!(request["source_bootstrap"]["reading"], json!(input.scene));
            assert_eq!(request["source_bootstrap"].as_object().unwrap().len(), 2);
            assert_eq!(
                request["source_bootstrap"]["issuer_receipt"],
                raw(base, &entry["issuer_receipt"])
            );
            assert_eq!(
                request["source_bootstrap"]["issuer_receipt"]["original_intent"]["authorship"],
                json!(input.authorship)
            );
            assert_eq!(
                raw(base, &entry["original_native_timing_pulse"]),
                result["native_timing_pulse"]
            );
            assert_eq!(
                entry["native_descriptor_calls"].as_array().unwrap().len(),
                1
            );
            assert_eq!(entry["native_descriptor_calls"][0]["operation"], "timing");
            assert_eq!(entry["native_descriptor_calls"][0]["moment"], "boundary");
            let pulse = &result["native_timing_pulse"];
            let management = &pulse["native_timing_owner"];
            let requirement = contract
                .requirements
                .iter()
                .find(|r| r.owner == contract.original_timing.owner_ref)
                .unwrap();
            validate_performance_management_configuration(
                &contract.original_timing,
                &contract.original_native_position,
                requirement,
                pulse,
            )
            .unwrap();
            contract
                .validate_source_read(
                    &input.scene,
                    &contract.original_native_position,
                    &contract.original_timing,
                )
                .unwrap();
            let definition: ConductInstall =
                serde_json::from_value(entry["definition"].clone()).unwrap();
            contract.validate_definition(&definition).unwrap();
            assert_ne!(
                json!(requirement.instance_ref),
                json!(contract.original_native_position.instance_ref)
            );
            assert_eq!(result["native_timing_consumer_fact"], *management);
            assert_eq!(result["native_timing_owner"], *management);
            assert_eq!(
                result["native_resident_source_correspondence"]["native_timing_owner"],
                *management
            );
            assert_eq!(
                contract.requirements.len(),
                pulse["resident_consumers"]["required_consumers"]
                    .as_array()
                    .unwrap()
                    .len()
                    + 2
            );
            assert!(
                !pulse["resident_consumers"]["required_consumers"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|r| r["role"] == "timing_owner")
            );
            if phase == "prepared-held" {
                assert_eq!(
                    pulse["payload"]["timing_fact"]["device_callbacks_running"],
                    false
                );
                original_management = Some(management["instance_ref"].clone());
                original_epoch = Some(cursor(&management["transport_epoch"]).unwrap());
            } else if phase == "sounding" {
                assert_eq!(
                    pulse["payload"]["timing_fact"]["device_callbacks_running"],
                    true
                );
                assert_eq!(
                    original_management.as_ref().unwrap(),
                    &management["instance_ref"]
                );
            } else {
                assert_eq!(phase, "restored");
                assert_eq!(
                    original_management.as_ref().unwrap(),
                    &management["instance_ref"]
                );
                assert!(cursor(&management["transport_epoch"]).unwrap() > original_epoch.unwrap());
            }
            let before = pulse.clone();
            for path in [
                "/native_timing_owner/generation",
                "/native_timing_owner/construction_ordinal",
                "/native_timing_owner/transport_epoch",
                "/native_timing_owner/sample",
                "/native_timing_owner/m3_source_generation",
                "/native_timing_owner/body_revision",
                "/resident_consumers/source/m2_generation",
                "/payload/timing_fact/native_position/generation",
            ] {
                let mut wrong = pulse.clone();
                let value = wrong.pointer_mut(path).unwrap();
                let original = cursor(value).unwrap();
                *value = json!(original.checked_add(1).unwrap().to_string());
                assert!(
                    validate_performance_management_configuration(
                        &contract.original_timing,
                        &contract.original_native_position,
                        requirement,
                        &wrong
                    )
                    .is_err(),
                    "{path}"
                );
            }
            for (key, value) in [
                (
                    "instance_ref",
                    json!(contract.original_native_position.instance_ref),
                ),
                ("owner_ref", json!("foreign:owner")),
                (
                    "generation_domain",
                    json!("native-management-transport-epoch"),
                ),
                ("time_mapping_ref", json!("foreign:mapping")),
                ("source", json!({"instance":"foreign:source"})),
            ] {
                let mut wrong = pulse.clone();
                wrong["native_timing_owner"][key] = value;
                assert!(
                    validate_performance_management_configuration(
                        &contract.original_timing,
                        &contract.original_native_position,
                        requirement,
                        &wrong
                    )
                    .is_err(),
                    "{key}"
                );
            }
            assert_eq!(*pulse, before);
            assert_eq!(
                entry["before_document"], entry["after_document"],
                "private Source read never edits or restamps CAS"
            );
        }
        assert_eq!(
            phases,
            BTreeSet::from(["prepared-held", "sounding", "restored"])
        );
    }
}
