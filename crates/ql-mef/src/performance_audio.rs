//! Serial control admission for the existing native performance callback.
//! Actual M1, K, M2/B and M3/P outputs are joined before a bounded immutable
//! packet crosses into C++. No graph, device, allocation or theory runs there.
use crate::continuous::coupled::{CoupledBasis, CoupledInput};
use crate::m1_engine::{M1Engine, carrier};
use crate::m2_relation_plan::{M2RelationPlan, M2RelationPlanContext, source_field};
use crate::m3_engine::{M3NodeKind, native_m3_engine};
use crate::m3_state::M3State;
use crate::music_determination::{
    Fundamental, MusicalDetermination, RelationSelection, TuningPolicy, VimarshaTargets,
    vimarsha_targets,
};
use crate::physical_body::{BodyPreparationRequest, PreparedSourceBody, prepare_source_body};
use crate::source_form_body::{SourceGeometryRecipe, admit_source_form_metric};
use crate::{MFace, MusicalBasis};
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const PREPARATION: &str = "ql.performance-preparation/v1";
pub const CALLBACK: &str = "ql.performance-audio/v1";
pub const MAX_TOUCHES: usize = 96;
#[derive(Debug, Clone, Copy)]
pub enum OctetScaling {
    /// Each voice uses M2 bus Hz times its K target / declared reference Hz.
    NoteRelativeToReference,
    /// Each voice uses the exact actual M2 bus Hz without transposition.
    AbsoluteBus,
}
#[derive(Debug, Clone)]
pub struct ExcitationPolicy {
    pub policy_ref: String,
    /// Explicit D30 implementation standing, not an authored twelve-to-eight map.
    pub standing: String,
    pub scaling: OctetScaling,
    pub reference_hertz: f64,
    pub root_linear: f64,
    pub octet_linear: f64,
    pub weights: [f64; 8],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FundamentalScaling {
    /// A declared reference root, with no implicit harmonic multiplication.
    AbsoluteReference,
    /// Apply the actual source-selected ratio already retained by compose().
    NativeM1HarmonicRatio,
}
#[derive(Debug, Clone)]
pub struct KeyTouch {
    pub key: u8,
    pub register: i8,
    pub member: u64,
    pub touch: u64,
    pub touch_ref: String,
}
pub struct PerformancePreparationInput {
    pub coupled: CoupledInput,
    pub relation_context: M2RelationPlanContext,
    pub source_face: MFace,
    pub physical_face: MFace,
    pub excitation: ExcitationPolicy,
    pub relation: RelationSelection,
    pub fundamental: Fundamental,
    pub fundamental_scaling: FundamentalScaling,
    pub tuning: TuningPolicy,
    /// Requiring authentic maqam tuning is a refusal while the real B path is
    /// unavailable. A named explicit policy never relabels that absence.
    pub require_authentic_condition_tuning: bool,
    pub physical: BodyPreparationRequest,
    pub instance_ref: String,
    pub receipt_ref: String,
    pub touches: Vec<KeyTouch>,
}
/// Private outputs cannot be patched piecemeal after native admission. The
/// serialized packet establishes derivation coherence, never host authority.
#[derive(Debug, Clone, Serialize)]
pub struct PreparedPerformanceBinding {
    schema: &'static str,
    callback_contract: &'static str,
    determination: Value,
    notes: Vec<Value>,
    physical_body: PreparedSourceBody,
    native_basis: CoupledBasis,
    relation_plan: M2RelationPlan,
    policy_receipts: Value,
    /// Present only after regenerating the complete metric geometry through
    /// the actual native M3 owner. A provider's constituent label cannot set it.
    #[serde(skip_serializing_if = "Option::is_none")]
    source_form_recipe: Option<SourceGeometryRecipe>,
    #[serde(skip)]
    targets: VimarshaTargets,
}
impl PreparedPerformanceBinding {
    /// Only the privately native-produced B/K source owner reaches this factory.
    /// Replays full current consumer/source closure; no JSON or label can select
    /// a new pitch policy. Musical bus and genuine physical preparation persist.
    pub(crate) fn admit_source_key_targets(
        &self,
        source: &crate::performance_source_keys::PreparedSourcePerformance,
        touches: &[KeyTouch],
    ) -> Result<Self, String> {
        if serde_json::to_value(self).map_err(|e| e.to_string())?
            != serde_json::to_value(source.base()).map_err(|e| e.to_string())?
        {
            return Err("source-key binding uses a different native preparation".into());
        }
        source.validate_current(self, source.targets().collection())?;
        let actual = source.packet_for_touches(touches)?;
        let mut next = self.clone();
        next.determination = actual["determination"].clone();
        next.notes = actual["notes"]
            .as_array()
            .ok_or("source notes absent")?
            .clone();
        next.policy_receipts["original_tuning"] = self.policy_receipts["tuning"].clone();
        next.policy_receipts["tuning"] =
            serde_json::to_value(&source.targets().reduction().provenance)
                .map_err(|e| e.to_string())?;
        next.policy_receipts["source_keys"] = actual["source_key_admission"].clone();
        next.validate_native_consumers(&next.native_basis, &next.physical_body)?;
        Ok(next)
    }
    pub fn determination(&self) -> &Value {
        &self.determination
    }
    pub fn notes(&self) -> &[Value] {
        &self.notes
    }
    pub fn physical_body(&self) -> &PreparedSourceBody {
        &self.physical_body
    }
    pub fn native_basis(&self) -> &CoupledBasis {
        &self.native_basis
    }
    pub fn relation_plan(&self) -> &M2RelationPlan {
        &self.relation_plan
    }
    pub fn targets(&self) -> &VimarshaTargets {
        &self.targets
    }
    pub fn source_form_recipe(&self) -> Option<&SourceGeometryRecipe> {
        self.source_form_recipe.as_ref()
    }
    /// Read-only correspondence from this actual immutable native preparation
    /// and ONE original resident pulse. Imported data can qualify a comparison,
    /// never mint a resident, document address, clock, lease or application ACK.
    /// The private selected-Act caller separately retains the original pulse.
    pub fn resident_source_correspondence(&self, pulse: &Value) -> Result<Value, String> {
        let number = |value: &Value| -> Result<u64, String> {
            let text = value.as_str().ok_or("canonical native decimal absent")?;
            let parsed = text.parse::<u64>().map_err(|e| e.to_string())?;
            if text != parsed.to_string() {
                return Err("noncanonical native decimal".into());
            }
            Ok(parsed)
        };
        let registry = &pulse["resident_consumers"];
        let reading = &pulse["reading"];
        let owner = &pulse["native_timing_owner"];
        let body = self.physical_body();
        let request = body.request();
        let snapshot = &registry["physical_observation"]["snapshot"];
        if pulse["schema"] != "ql.performance-worker-reply/v1"
            || pulse["accepted"] != true
            || registry["schema"] != "ql.native-resident-consumer-registry/v1"
            || registry["origin"] != "actual-native-constructors-and-same-pulse"
            || registry["source"] != self.determination()["identity"]
            || registry["sample"] != reading["samples_elapsed"]
            || registry["transport_epoch"] != reading["transport_epoch"]
            || snapshot["schema"] != "ql.native-physical-snapshot/v1"
            || snapshot["version"] != 1
            || snapshot["event_ref"] != body.event_ref()
            || snapshot["subject_ref"] != body.subject_ref()
            || snapshot["source_coordinate"] != body.source_coordinate().source_ref
            || snapshot["source_revision"] != body.source_revision()
            || snapshot["preparation_ref"] != request.preparation_ref
            || snapshot["state_ref"] != request.state_ref
            || number(&snapshot["body_revision"])? != request.body_revision
            || number(&snapshot["source_generation"])? != body.source_generation()
            || snapshot["body_revision"] != registry["body_revision"]
            || snapshot["source_generation"] != registry["m3_source_generation"]
            || snapshot["sample_rate"] != request.sample_rate
            || snapshot["pratibimba"] != (body.source_coordinate().face == MFace::Pratibimba)
            || snapshot["samples_elapsed"] != registry["sample"]
            || snapshot["eigenbasis_identity"] != reading["physical"]["eigenbasis_identity"]
            || registry["preparation_ref"] != snapshot["preparation_ref"]
            || registry["state_ref"] != snapshot["state_ref"]
            || registry["source_coordinate"] != snapshot["source_coordinate"]
            || registry["source_revision"] != snapshot["source_revision"]
            || registry["eigenbasis_identity"] != snapshot["eigenbasis_identity"]
            || registry["physical_pratibimba"] != snapshot["pratibimba"]
        {
            return Err(
                "resident correspondence differs from actual prepared source/body/pulse".into(),
            );
        }
        for (role, provenance) in [
            ("geometry", &request.geometry.provenance),
            ("material", &request.material.provenance),
        ] {
            if snapshot[format!("{role}_ref")] != provenance.reference
                || snapshot[format!("{role}_revision")] != provenance.revision
            {
                return Err("resident correspondence lost original metric source revision".into());
            }
        }
        let roles = registry["required_consumers"]
            .as_array()
            .ok_or("native resident roster absent")?;
        let mut tokens = BTreeSet::new();
        let mut names = BTreeSet::new();
        for role in roles {
            let token = role["instance_ref"]
                .as_str()
                .ok_or("native resident constructor absent")?;
            let name = role["role"].as_str().ok_or("native resident role absent")?;
            let generation = number(&role["generation"])?;
            let observation = match name {
                "audio_engine" => &registry["audio_observation"],
                "physical_body" => &registry["physical_observation"],
                "acoustic_receiving" => &registry["receiving_observation"],
                _ => return Err("unknown actual resident role".into()),
            };
            if !token.starts_with("native-resident:v1:")
                || !tokens.insert(token)
                || !names.insert(name)
                || !["audio_engine", "physical_body", "acoustic_receiving"].contains(&name)
                || role["generation_domain"] != "native-resident-construction"
                || generation == 0
                || token.rsplit(':').next() != Some(generation.to_string().as_str())
                || role["sample"] != registry["sample"]
                || observation["instance_ref"] != role["instance_ref"]
                || observation["sample"] != role["sample"]
                || !role["callback_output_committed"].is_boolean()
                || role["callback_output_committed"]
                    != registry["audio_observation"]["callback_output_committed"]
                || name == "physical_body"
                    && snapshot["resident_instance_ref"] != role["instance_ref"]
            {
                return Err(
                    "resident correspondence has duplicate/disconnected actual lifetimes".into(),
                );
            }
        }
        if !(roles.len() == 2 || roles.len() == 3)
            || !names.contains("audio_engine")
            || !names.contains("physical_body")
            || names.contains("acoustic_receiving") == registry["receiving_observation"].is_null()
        {
            return Err("resident correspondence lacks the complete installed roster".into());
        }
        let management = owner["instance_ref"]
            .as_str()
            .ok_or("actual Management constructor absent")?;
        let generation = number(&owner["generation"])?;
        if owner["schema"] != "ql.native-management-timing-owner/v1"
            || owner["role"] != "timing_owner"
            || owner["generation_domain"] != "native-management-construction"
            || generation == 0
            || owner["construction_ordinal"] != owner["generation"]
            || !management.starts_with("native-resident:v1:")
            || management.rsplit(':').next() != Some(generation.to_string().as_str())
            || tokens.contains(management)
            || owner["owner_ref"] != reading["session_ref"]
            || owner.get("time_mapping_ref").is_none_or(|v| !v.is_null())
            || owner["callback_output_committed"]
                != registry["audio_observation"]["callback_output_committed"]
        {
            return Err("Management construction/source/transport identities are aliased".into());
        }
        for key in [
            "sample",
            "transport_epoch",
            "source",
            "m3_source_generation",
            "body_revision",
        ] {
            if owner.get(key).is_none() || owner[key] != registry[key] {
                return Err("Management differs from same original resident pulse".into());
            }
        }
        let actual_nodes = registry["native_nodes"]
            .as_array()
            .ok_or("native physical nodes absent")?;
        let prepared_nodes = &request.geometry.nodes;
        let ids = snapshot["node_identity"]
            .as_array()
            .ok_or("original physical node IDs absent")?;
        let rest = snapshot["rest_positions_metres"]
            .as_array()
            .ok_or("original rest positions absent")?;
        let visible = snapshot["visible_positions_metres"]
            .as_array()
            .ok_or("original visible positions absent")?;
        if prepared_nodes.len() < 2
            || prepared_nodes.len() > 32
            || [actual_nodes.len(), ids.len(), rest.len(), visible.len()]
                .iter()
                .any(|n| *n != prepared_nodes.len())
            || snapshot["node_count"].as_u64() != Some(prepared_nodes.len() as u64)
        {
            return Err(
                "resident source correspondence dropped or filled a physical descendant".into(),
            );
        }
        let mut identities = BTreeSet::new();
        let mut nodes = Vec::with_capacity(prepared_nodes.len());
        for (index, (observed, prepared)) in actual_nodes.iter().zip(prepared_nodes).enumerate() {
            if number(&observed["native_node_id"])? != prepared.identity
                || number(&ids[index])? != prepared.identity
                || !identities.insert(prepared.identity)
                || prepared.constituent.trim().is_empty()
                || observed["visible_metres"] != visible[index]
            {
                return Err(
                    "resident source correspondence reordered or replaced a descendant".into(),
                );
            }
            for axis in 0..3 {
                let actual = observed["rest_metres"][axis]
                    .as_f64()
                    .ok_or("metric rest absent")?;
                let copied = rest[index][axis]
                    .as_f64()
                    .ok_or("copied metric rest absent")?;
                let position = visible[index][axis]
                    .as_f64()
                    .ok_or("visible metric position absent")?;
                if actual.to_bits() != prepared.rest_metres[axis].to_bits()
                    || copied.to_bits() != actual.to_bits()
                    || !position.is_finite()
                    || observed["rest_metres"]
                        .as_array()
                        .is_none_or(|v| v.len() != 3)
                    || visible[index].as_array().is_none_or(|v| v.len() != 3)
                {
                    return Err(
                        "resident source correspondence changed metric source/rest/visible copy"
                            .into(),
                    );
                }
            }
            nodes.push(json!({"native_node_id":prepared.identity.to_string(),
                "source_constituent_ref":prepared.constituent,
                "rest_metres":prepared.rest_metres,"visible_metres":visible[index],
                "fixed":prepared.fixed}));
        }
        Ok(
            json!({"schema":"ql.native-resident-source-correspondence/v1",
            "source":registry["source"],"sample":registry["sample"],
            "transport_epoch":registry["transport_epoch"],"body_revision":registry["body_revision"],
            "m3_source_generation":registry["m3_source_generation"],
            "physical_preparation":body,"source_form_recipe":self.source_form_recipe().ok_or("native source geometry recipe absent")?,
            "native_timing_owner":owner,"required_consumers":roles,"nodes":nodes,
            "document_address_correspondence":"requires-private-current-document-source-recipe-CAS",
            "standing":"exact immutable native source-to-resident comparison; no Document address, grant or application ACK"}),
        )
    }
    /// Control-owner source admission. The exact post-command native state
    /// and metric recipe must still describe this immutable prepared binding.
    /// Reading-only edits use the existing P update policy before re-admission;
    /// this check neither advances nor resets the resident physical body.
    pub fn validate_source_form_consumer(&self, state: &M3State) -> Result<(), String> {
        let recipe = self
            .source_form_recipe
            .as_ref()
            .ok_or("performance has provider metric standing, not native source-form admission")?;
        if state.snapshot() != self.native_basis.m3 {
            return Err("source-form performance disconnected from actual post-command M3".into());
        }
        admit_source_form_metric(state, recipe, &self.physical_body.request().geometry)?;
        self.validate_native_consumers(&self.native_basis, &self.physical_body)
    }
    /// Replays actual producers rather than trusting labels in a receipt.
    /// This is a control-owner check, never an audio callback operation.
    pub fn validate_native_consumers(
        &self,
        basis: &CoupledBasis,
        body: &PreparedSourceBody,
    ) -> Result<(), String> {
        let writer =
            crate::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
        self.targets.determination.validate_coupled_consumer(
            basis,
            &writer,
            self.targets.determination.identity().tick12() / 6,
        )?;
        if serde_json::to_value(basis).map_err(|e| e.to_string())?
            != serde_json::to_value(&self.native_basis).map_err(|e| e.to_string())?
            || serde_json::to_value(body).map_err(|e| e.to_string())?
                != serde_json::to_value(&self.physical_body).map_err(|e| e.to_string())?
        {
            return Err("disconnected performance native/body consumer".into());
        }
        self.relation_plan
            .source_receipts
            .validate_against(source_field())?;
        Ok(())
    }
}
fn bounded_ref(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.trim() != value
        || value.len() >= 256
        || value.chars().any(char::is_control)
    {
        return Err("bounded printable performance reference required".into());
    }
    Ok(())
}
fn same_value<T: Serialize, U: Serialize>(a: &T, b: &U) -> Result<bool, String> {
    Ok(serde_json::to_value(a).map_err(|e| e.to_string())?
        == serde_json::to_value(b).map_err(|e| e.to_string())?)
}
/// Prepare the playable source-form instrument through the same native audio
/// producer. This stronger admission retains P's declared mechanical recipe
/// and regenerates all positions, connectivity and constraints after every
/// actual M3 command; a supplied metric with matching labels is insufficient.
pub fn prepare_source_form_performance(
    input: PerformancePreparationInput,
    recipe: SourceGeometryRecipe,
) -> Result<PreparedPerformanceBinding, String> {
    let mut state = M3State::new(input.coupled.m3.clone())?;
    for command in &input.coupled.m3_commands {
        state.apply(command.clone())?;
    }
    admit_source_form_metric(&state, &recipe, &input.physical.geometry)?;
    let mut prepared = prepare_native_performance(input)?;
    prepared.source_form_recipe = Some(recipe);
    prepared.validate_source_form_consumer(&state)?;
    Ok(prepared)
}
pub fn prepare_native_performance(
    input: PerformancePreparationInput,
) -> Result<PreparedPerformanceBinding, String> {
    bounded_ref(&input.instance_ref)?;
    bounded_ref(&input.receipt_ref)?;
    bounded_ref(&input.excitation.policy_ref)?;
    bounded_ref(&input.excitation.standing)?;
    let p = &input.excitation;
    if !p.reference_hertz.is_finite()
        || p.reference_hertz < 0.001
        || p.reference_hertz > f64::from(input.physical.sample_rate) * 0.45
        || !p.root_linear.is_finite()
        || !p.octet_linear.is_finite()
        || p.root_linear < 0.0
        || p.octet_linear < 0.0
        || p.root_linear + p.octet_linear > 1.0
        || p.weights
            .iter()
            .any(|w| !w.is_finite() || *w < 0.0 || *w > 1.0)
        || (p.weights.iter().sum::<f64>() - 1.0).abs() > 1e-12
    {
        return Err("invalid declared D30 excitation policy".into());
    }
    if input.touches.is_empty() || input.touches.len() > MAX_TOUCHES {
        return Err("performance touch preparation budget exceeded".into());
    }
    if !input.coupled.frequency_bindings.is_empty()
        || !input.coupled.condition_frequency_bindings.is_empty()
        || !input.coupled.sky_frequency_bindings.is_empty()
    {
        return Err(
            "genuine physical performance cannot retune supplied modes to excitation notes".into(),
        );
    }
    if !input.relation_context.material_writes.is_empty() {
        return Err(
            "legacy modal damping write needs an explicit P physical material interpretation"
                .into(),
        );
    }
    let original = input.coupled.compose()?;
    let relation =
        M2RelationPlan::compile(&original.m2_input, input.relation_context, source_field())?;
    let replay = relation
        .plan
        .prepare_request(&original.m2_input, source_field())?;
    if !same_value(&replay, &relation.request)? {
        return Err("relation plan producer replay differs".into());
    }
    if input.require_authentic_condition_tuning {
        if relation.plan.execution.intended_tuning_hz.is_none() {
            return Err("authentic condition tuning is source-unavailable; explicit policy is a separate choice".into());
        }
        // A maqam's seven pitches do not establish twelve canonical key targets.
        return Err(
            "authentic condition tuning does not supply a complete twelve-address target policy"
                .into(),
        );
    }
    let mut receiving = input.coupled.clone();
    receiving.m2 = relation.request.clone();
    receiving
        .source_receipts
        .push(serde_json::to_value(&relation.plan).map_err(|e| e.to_string())?);
    let joined = receiving.compose()?;
    if !same_value(&joined.m2, &relation.frame)? {
        return Err("actual joined M2 consumer differs from prepared relation output".into());
    }
    let m1 = M1Engine::new(receiving.m1.clone())?;
    let determination = MusicalDetermination::from_engine(&m1, input.source_face, input.relation)?;
    let writer =
        crate::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
    determination.validate_coupled_consumer(
        &joined,
        &writer,
        determination.identity().tick12() / 6,
    )?;
    let multiplier = match input.fundamental_scaling {
        FundamentalScaling::AbsoluteReference => 1.0,
        FundamentalScaling::NativeM1HarmonicRatio => {
            let ratio = &joined.derivation["harmonic_ratio"]["ratio"];
            let numerator = ratio[0]
                .as_u64()
                .ok_or("native M1 harmonic numerator missing")?;
            let denominator = ratio[1]
                .as_u64()
                .ok_or("native M1 harmonic denominator missing")?;
            if numerator == 0 || denominator == 0 {
                return Err("invalid native M1 harmonic ratio".into());
            }
            numerator as f64 / denominator as f64
        }
    };
    let fundamental = Fundamental::new(
        input.fundamental.hertz() * multiplier,
        input.fundamental.provenance().clone(),
    )?;
    let targets = vimarsha_targets(determination, fundamental, input.tuning)?;
    let mut m3 = M3State::new(receiving.m3.clone())?;
    for command in &receiving.m3_commands {
        m3.apply(command.clone())?;
    }
    if m3.snapshot() != joined.m3 {
        return Err("physical preparation disconnected from actual joined M3".into());
    }
    let form = native_m3_engine()
        .node(M3NodeKind::Codon, usize::from(m3.fold().codon().address()))
        .ok_or("actual physical form lacks exact source node")?;
    let coordinate = crate::m_tree::native_current_m_registry()
        .coordinate(&form.source_ref, input.physical_face)?;
    let body = prepare_source_body(&m3, coordinate, input.physical)?;
    if body.event_ref() != targets.determination.identity().event_ref()
        || relation.frame.identity.event_ref != body.event_ref()
    {
        return Err("M1/M2/P physical event disconnected".into());
    }
    let config = m1.config();
    let source_carrier = carrier(
        config
            .cycle
            .parse()
            .map_err(|_| "invalid native M1 cycle")?,
        config.tick12,
    )?;
    let quadrature = match input.source_face {
        MFace::Bimba => source_carrier.quadrature,
        MFace::Pratibimba => source_carrier.opposite_quadrature,
    };
    let identity = json!({"instance":input.instance_ref,"event":body.event_ref(),"subject":body.subject_ref(),
        "m1_revision":config.revision,"m2_generation":relation.frame.identity.profile_generation.to_string()});
    let vimarsha = relation
        .frame
        .vimarsha
        .as_ref()
        .ok_or("missing actual joined Vimarśā bus")?;
    let tuning_ref = targets.tuning_policy.provenance().policy_ref.clone();
    bounded_ref(&tuning_ref)?;
    let packet = json!({"identity":identity,"m1_coordinate":targets.determination.coordinate().source_ref,
        "m1_face":u8::from(input.source_face==MFace::Pratibimba),"m2_writer":"#2-1","m2_face":1,
        "registry_revision":relation.frame.registry_revision,"source_revision":relation.plan.source_receipts.source_revision,
        "relation_plan_ref":input.receipt_ref,"tuning_ref":tuning_ref,"native_receipt_ref":input.receipt_ref,
        "tick12":targets.determination.identity().tick12(),"degree720":targets.determination.identity().degree720(),
        "basis":match targets.determination.basis(){MusicalBasis::Chromatic=>0,MusicalBasis::Fifths=>1},
        "lens12":targets.determination.lens().slot(),"context_frame":config.context_frame,
        "audio_octet_hz":joined.m2["vimarsha"]["reading"]["audio_octet_hz"],
        "nodal_quartet":vimarsha.reading.nodal_quartet.map(|n|json!({"position":n.ql_position,
            "face":u8::from(n.helix==crate::m2_vimarsha::VimarshaHelix::Pratibimba),"m":n.m,"n":n.n})),
        "body_preparation_ref":body.request().preparation_ref,"body_state_ref":body.request().state_ref,
        "body_revision":body.request().body_revision.to_string(),"tuning_available":true,
        "excitation":{"policy_ref":input.excitation.policy_ref,"standing":input.excitation.standing,
            "scaling":match input.excitation.scaling{OctetScaling::NoteRelativeToReference=>0,OctetScaling::AbsoluteBus=>1},
            "reference_hertz":input.excitation.reference_hertz,"root_linear":input.excitation.root_linear,
            "octet_linear":input.excitation.octet_linear,"weights":input.excitation.weights}});
    let mut notes = Vec::with_capacity(input.touches.len());
    let mut tokens = BTreeSet::new();
    let mut members = std::collections::BTreeMap::new();
    for touch in input.touches {
        bounded_ref(&touch.touch_ref)?;
        if touch.touch == 0 || touch.member == 0 || !tokens.insert(touch.touch) {
            return Err("duplicate/invalid physical touch identity".into());
        }
        if let Some(previous) = members.insert(touch.member, (touch.key, touch.register)) {
            if previous != (touch.key, touch.register) {
                return Err("canonical member bound to different key/register".into());
            }
        }
        let note = targets.key_target(touch.key, touch.register, &touch.touch_ref)?;
        if note.hertz >= f64::from(body.request().sample_rate) * 0.45 {
            return Err("note above admitted native audio band".into());
        }
        let (numerator, denominator) = note
            .exact_ratio
            .map(|r| (r.numerator().to_string(), r.denominator().to_string()))
            .unwrap_or_else(|| ("0".into(), "0".into()));
        notes.push(json!({"identity":identity,"source_coordinate":note.source_coordinate.source_ref,"source_face":packet["m1_face"],
            "tuning_ref":tuning_ref,"member":touch.member.to_string(),"touch":touch.touch.to_string(),"touch_ref":touch.touch_ref,
            "key":touch.key,"position":touch.key/2,"coordinate_face":touch.key%2,"register_octave":touch.register,
            "pitch_class":note.pitch_class,"fundamental_hz":note.fundamental.hertz(),"hertz":note.hertz,
            "ratio_numerator":numerator,"ratio_denominator":denominator,"exact_ratio":note.exact_ratio.is_some(),
            "phase_cos":quadrature[0],"phase_sin":quadrature[1]}));
    }
    let policy_receipts = json!({"standing":"native-derivation-coherence-not-host-authentication",
        "reference_fundamental_hz":input.fundamental.hertz(),"reference_fundamental_provenance":input.fundamental.provenance(),
        "fundamental_scaling":match input.fundamental_scaling{FundamentalScaling::AbsoluteReference=>"absolute-reference",FundamentalScaling::NativeM1HarmonicRatio=>"native-m1-harmonic-ratio"},
        "native_harmonic_ratio":joined.derivation["harmonic_ratio"],"tuning":targets.tuning_policy.provenance(),
        "condition_authentic_tuning":relation.plan.execution.intended_tuning_hz,
        "condition_tuning_standing":relation.plan.execution.tuning_standing,
        "excitation_phase_source":match input.source_face{MFace::Bimba=>"m1.carrier.quadrature",MFace::Pratibimba=>"m1.carrier.opposite_quadrature"},
        "audio_octet_source":"actual-m2.vimarsha.reading.audio_octet_hz","nodal_source":"actual-m2.vimarsha.reading.nodal_quartet",
        "key_targets_source":"native-K.key_target; same M1 excitation; distinct from audio_octet indexing",
        "excitation_policy":packet["excitation"],"excitation_units":"Hz; dimensionless gains; resulting scalar force Newtons",
        "octet_band_policy":"suppress components below 0.001 Hz or above 0.45 sample rate; retain bounded phase and report suppression",
        "physical_spectrum":"prepared metric mass/stiffness; never retuned to played note",
        "material_interpretation":"explicit P material provider; legacy modal writes require a separate physical interpretation"});
    let out = PreparedPerformanceBinding {
        schema: PREPARATION,
        callback_contract: CALLBACK,
        determination: packet,
        notes,
        physical_body: body,
        native_basis: joined,
        relation_plan: relation.plan,
        policy_receipts,
        source_form_recipe: None,
        targets,
    };
    out.validate_native_consumers(out.native_basis(), out.physical_body())?;
    Ok(out)
}
