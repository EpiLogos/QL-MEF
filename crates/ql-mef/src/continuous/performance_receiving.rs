//! The current native receiving owner behind the existing FieldHost lease.
//! No Deserialize: retained witness/definition JSON is never a constructor.
//! Outer Act/context/consent custody is supplied by the existing native host.
use super::{coupled::CoupledBasis, performance::PerformanceOwner};
use crate::musical_performance_return::{ReturnContext, ReturnReference};
use crate::nara::{current, intake::IdentityProfile, replay::NaraOccasion};
use crate::nara_performance_receiving::{
    ContextKind, ReceivingCalibration, ReceivingContext, ReceivingDefinition, ReceivingPreparation,
    Reference, prepare_native_receiving,
};
use crate::performance_receiving_admission::{
    NativeReceivingAdmission, prepare_native_receiving_admission,
};
use crate::performance_source_context::{
    NativePerformanceSourceContext, NativePublicSourceOwnership, prepare_native_source_context,
};
use crate::scene::{WorldRequest, world};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Private native producer inputs retained under one existing owner/lease.
/// Personal/Shared constructors replay actual N identity and current producers.
/// Constructor calls do not grant Act, private-read or consent authority.
#[derive(Debug, Clone)]
pub struct NativePerformanceReceivingSource {
    context: ReturnContext,
    receiving_context: ReceivingContext,
    profile: Option<IdentityProfile>,
    natal: Option<Value>,
    sky: Option<Value>,
    original_occasion: Option<NaraOccasion>,
    calibration: Option<ReceivingCalibration>,
    public: Option<NativePublicSourceOwnership>,
    world_request: Option<WorldRequest>,
    acoustic: Option<super::performance::AcousticConfiguration>,
}
fn receiving_context(context: &ReturnContext) -> Result<ReceivingContext, String> {
    let r = |v: &ReturnReference| Reference {
        reference: v.reference.clone(),
        revision: v.revision.clone(),
    };
    let kind = match context.kind.as_str() {
        "world" => ContextKind::World,
        "personal" => ContextKind::Personal,
        "shared" => ContextKind::Shared,
        _ => return Err("unknown native receiving context".into()),
    };
    Ok(ReceivingContext {
        kind,
        context: r(&context.context),
        receiver: r(&context.receiver),
        original_occasion: context.source_occasion.as_ref().map(r),
        protected_state: context.protected_state.as_ref().map(r),
        consent: context.consent.as_ref().map(r),
        private: context.private,
    })
}
impl NativePerformanceReceivingSource {
    /// Actual World factory, not a field absence/label inference. It executes
    /// the source-owned constructor again; public ownership checks its complete
    /// retained original receipt before the first performance is prepared.
    pub fn world_source(request: WorldRequest, context: ReturnContext) -> Result<Self, String> {
        if context.kind != "world"
            || context.private
            || context.source_occasion.is_some()
            || context.protected_state.is_some()
            || context.consent.is_some()
        {
            return Err("World source cannot borrow a private original occasion or grant".into());
        }
        // WorldRequest is a source request. Serializing it is unnecessary;
        // replay the same bounded constructor input twice, keeping its result.
        world(request.clone())?;
        let public = NativePublicSourceOwnership::world_source(request.clone())?;
        Ok(Self {
            receiving_context: receiving_context(&context)?,
            context,
            profile: None,
            natal: None,
            sky: None,
            original_occasion: None,
            calibration: None,
            public: Some(public),
            world_request: Some(request),
            acoustic: None,
        })
    }
    /// Explicit native Reference source branch for actual library/floor owners.
    /// It retains its standing and cannot classify an opaque receipt publicly.
    pub fn reference_world(actual: &CoupledBasis, context: ReturnContext) -> Result<Self, String> {
        if context.kind != "world"
            || context.private
            || context.source_occasion.is_some()
            || context.protected_state.is_some()
            || context.consent.is_some()
        {
            return Err("Reference World cannot borrow a protected receiving scope".into());
        }
        Ok(Self {
            receiving_context: receiving_context(&context)?,
            context,
            profile: None,
            natal: None,
            sky: None,
            original_occasion: None,
            calibration: None,
            public: Some(NativePublicSourceOwnership::reference_source(actual)?),
            world_request: None,
            acoustic: None,
        })
    }
    /// Existing native N owner calls this with its actual original profile,
    /// natal/provider sources, original occasion and metric calibration under
    /// current private-read custody. No supplied identity/current JSON reading
    /// is used as authority: both actual producers execute here.
    pub fn personal(
        profile: &IdentityProfile,
        natal: Option<&Value>,
        sky: &Value,
        original_occasion: NaraOccasion,
        calibration: ReceivingCalibration,
        context: ReturnContext,
    ) -> Result<Self, String> {
        if context.kind != "personal" || !context.private || context.consent.is_some() {
            return Err("personal source requires exact private original context".into());
        }
        Self::protected(profile, natal, sky, original_occasion, calibration, context)
    }
    /// Outer existing Shared Act additionally validates its exact consent grant
    /// and receiver; this constructor retains and numerically/source binds it.
    pub fn shared(
        profile: &IdentityProfile,
        natal: Option<&Value>,
        sky: &Value,
        original_occasion: NaraOccasion,
        calibration: ReceivingCalibration,
        context: ReturnContext,
    ) -> Result<Self, String> {
        if context.kind != "shared" || !context.private || context.consent.is_none() {
            return Err(
                "Shared source requires exact current consent and original private occasion".into(),
            );
        }
        Self::protected(profile, natal, sky, original_occasion, calibration, context)
    }
    fn protected(
        profile: &IdentityProfile,
        natal: Option<&Value>,
        sky: &Value,
        original_occasion: NaraOccasion,
        calibration: ReceivingCalibration,
        context: ReturnContext,
    ) -> Result<Self, String> {
        let c = receiving_context(&context)?;
        if profile.person_ref != original_occasion.subject_id
            || c.original_occasion
                .as_ref()
                .is_none_or(|r| r.reference != original_occasion.occasion_ref)
            || c.protected_state.as_ref().is_none_or(|r| {
                r.reference != original_occasion.protected_state_ref.ref_id
                    || r.revision != original_occasion.protected_state_ref.revision
            })
        {
            return Err(
                "native identity/context/protected source belongs to another original occasion"
                    .into(),
            );
        }
        let identity = profile.inspect(natal)?;
        let transit = current::transit(Some(sky))?;
        current::personal_current(&identity, &transit)?;
        Ok(Self {
            context,
            receiving_context: c,
            profile: Some(profile.clone()),
            natal: natal.cloned(),
            sky: Some(sky.clone()),
            original_occasion: Some(original_occasion),
            calibration: Some(calibration),
            public: None,
            world_request: None,
            acoustic: None,
        })
    }
    /// Closed authored acoustic magnitudes are retained by this original
    /// native source. No caller receiver/context/profile/consent or clock can
    /// enter here; those identities remain the actual native ReturnContext.
    pub fn with_acoustic_configuration(
        mut self,
        configuration: super::performance::AcousticConfiguration,
    ) -> Result<Self, String> {
        // Performance source currently admits48k. Actual preparation repeats
        // this policy against its exact native physical rate before install.
        configuration.validate(48000)?;
        self.acoustic = Some(configuration);
        Ok(self)
    }
    pub(crate) fn acoustic_configuration(
        &self,
    ) -> Option<&super::performance::AcousticConfiguration> {
        self.acoustic.as_ref()
    }
    fn replay(&self, actual_original: &CoupledBasis) -> Result<CurrentSourceReading, String> {
        let mut public = self.public.clone();
        if let Some(request) = &self.world_request {
            let produced = world(request.clone())?;
            if produced["event"]
                != serde_json::to_value(&actual_original.input).map_err(|e| e.to_string())?
            {
                return Err(
                    "World source constructor differs from actual retained original source".into(),
                );
            }
            public = Some(NativePublicSourceOwnership::world_source(request.clone())?);
        }
        match (&self.profile, &self.sky) {
            (Some(profile), Some(sky)) => {
                let identity = profile.inspect(self.natal.as_ref())?;
                let transit = current::transit(Some(sky))?;
                let current = current::personal_current(&identity, &transit)?;
                Ok(CurrentSourceReading {
                    identity: Some(identity),
                    current: Some(current),
                    public,
                })
            }
            (None, None) if self.receiving_context.kind == ContextKind::World => {
                Ok(CurrentSourceReading {
                    identity: None,
                    current: None,
                    public,
                })
            }
            _ => Err("native original receiving producer inputs disconnected".into()),
        }
    }
    fn input<'a>(
        &'a self,
        owner: &'a PerformanceOwner,
        reading: &'a CurrentSourceReading,
    ) -> ReceivingPreparation<'a> {
        ReceivingPreparation {
            prepared: owner.binding(),
            context: self.receiving_context.clone(),
            identity: reading.identity.as_ref(),
            current: reading.current.as_ref(),
            original_occasion: self.original_occasion.as_ref(),
            calibration: self.calibration.as_ref(),
        }
    }
    /// No worker/device effects. Every call independently regenerates the
    /// actual source producers and all context determinants at the current CAS.
    pub fn prepare_current(
        &self,
        owner: &PerformanceOwner,
        actual_original: &CoupledBasis,
        admitted_native_cursor: u64,
    ) -> Result<PreparedCurrentReceiving, String> {
        let reading = self.replay(actual_original)?;
        let origin = owner.source_context_basis(actual_original)?;
        let definition = prepare_native_receiving(self.input(owner, &reading))?;
        let context = prepare_native_source_context(
            &origin,
            &definition,
            self.input(owner, &reading),
            self.context.clone(),
            reading.public.as_ref(),
        )?;
        context.validate_current(
            &origin,
            &definition,
            self.input(owner, &reading),
            self.context.clone(),
            reading.public.as_ref(),
        )?;
        let admission = prepare_native_receiving_admission(
            &definition,
            self.input(owner, &reading),
            owner.binding().native_basis(),
            admitted_native_cursor,
        )?;
        admission.validate_current(
            &definition,
            self.input(owner, &reading),
            owner.binding().native_basis(),
            admitted_native_cursor,
        )?;
        Ok(PreparedCurrentReceiving {
            definition,
            context,
            admission,
            source_inputs: self.source_inputs()?,
        })
    }
    /// Actual host performs this at its current Act/lease, before publication.
    /// The native input replay is repeated immediately before the setter; the
    /// setter's binding-only check never stands in for current receiving custody.
    pub fn admit_current(
        &self,
        owner: &mut PerformanceOwner,
        actual_original: &CoupledBasis,
        admitted_native_cursor: u64,
    ) -> Result<PreparedCurrentReceiving, String> {
        let prepared = self.prepare_current(owner, actual_original, admitted_native_cursor)?;
        let reading = self.replay(actual_original)?;
        prepared.context.validate_current(
            &owner.source_context_basis(actual_original)?,
            &prepared.definition,
            self.input(owner, &reading),
            self.context.clone(),
            reading.public.as_ref(),
        )?;
        prepared.admission.validate_current(
            &prepared.definition,
            self.input(owner, &reading),
            owner.binding().native_basis(),
            admitted_native_cursor,
        )?;
        owner.admit_source_context(actual_original, &prepared.context)?;
        Ok(prepared)
    }
    /// Complete native source inputs for the existing C asset custody. These
    /// bytes are evidence; a restored lease must call the actual constructors
    /// and validate_current again, never deserialize a witness as authority.
    pub fn source_inputs(&self) -> Result<Value, String> {
        let request = self.world_request.as_ref().map(|r| json!({"schema":r.schema,"instance_ref":r.instance_ref,"event_ref":r.event_ref,"subject_ref":r.subject_ref,"sky":r.sky,"texture":r.texture,"units_per_metre":r.units_per_metre,"start":r.start,"geometry":r.geometry,"material":r.material}));
        let mut out = json!({"schema":"ql.native-performance-receiving-source-inputs/v1","constructor":if request.is_some(){"native-world"}else if self.profile.is_some(){"native-protected"}else{"explicit-reference-world"},"world_request":request,"identity_profile":self.profile,"natal":self.natal,"sky":self.sky,"original_occasion":self.original_occasion,"calibration":self.calibration,"return_context":self.context});
        if let Some(acoustic) = &self.acoustic {
            out["acoustic_receiving"] =
                serde_json::to_value(acoustic).map_err(|e| e.to_string())?;
        }
        if serde_json::to_vec(&out).map_err(|e| e.to_string())?.len() > 16 * 1024 * 1024 {
            return Err("native receiving source asset exceeds bound".into());
        }
        Ok(out)
    }
    pub fn return_context(&self) -> &ReturnContext {
        &self.context
    }
    pub fn original_occasion(&self) -> Option<&NaraOccasion> {
        self.original_occasion.as_ref()
    }
}
/// Immutable private native result. Serialized fields retain evidence only;
/// no standalone deserialization can manufacture the live route admission.
#[derive(Debug, Clone)]
pub struct PreparedCurrentReceiving {
    definition: ReceivingDefinition,
    context: NativePerformanceSourceContext,
    admission: NativeReceivingAdmission,
    source_inputs: Value,
}
struct CurrentSourceReading {
    identity: Option<Value>,
    current: Option<Value>,
    public: Option<NativePublicSourceOwnership>,
}
impl PreparedCurrentReceiving {
    pub fn definition(&self) -> &ReceivingDefinition {
        &self.definition
    }
    pub fn context(&self) -> &NativePerformanceSourceContext {
        &self.context
    }
    pub fn admission(&self) -> &NativeReceivingAdmission {
        &self.admission
    }
    pub fn source_inputs(&self) -> &Value {
        &self.source_inputs
    }
    /// Complete retained evidence; only native replay of the private source
    /// owner grants continuation. Full equality detects appended sidecars.
    pub fn snapshot(&self) -> Result<Value, String> {
        let context = self.context.snapshot()?;
        let admission = self.admission.snapshot()?;
        let hash = |v: &Value| -> Result<String, String> {
            Ok(format!(
                "sha256:{:x}",
                Sha256::digest(serde_json::to_vec(v).map_err(|e| e.to_string())?)
            ))
        };
        // Created only by this private native original-source factory. These
        // hashes bind the complete payload; authority still requires full
        // independently regenerated snapshot/current Act/receiver/consent.
        let payload = json!({"schema":"ql.native-receiving-source-payload-context/v1","source_inputs_sha256":hash(&self.source_inputs)?,"source_context_sha256":hash(&context)?,"native_admission_sha256":hash(&admission)?,"private":context["context"]["private"],"owner":{"ref":"crates/ql-mef/src/continuous/performance_receiving.rs","revision":format!("sha256:{:x}",Sha256::digest(include_str!("performance_receiving.rs").as_bytes())),"availability":"available"},"standing":"complete original inputs of this private native receiving constructor; current lease/replay is mandatory"});
        Ok(
            json!({"schema":"ql.current-performance-receiving/v1","source_inputs":self.source_inputs,"source_context":context,"source_payload_context":payload,"receiving_definition":self.definition.snapshot()?,"native_admission":admission}),
        )
    }
    pub fn validate_current(
        &self,
        source: &NativePerformanceReceivingSource,
        owner: &PerformanceOwner,
        actual_original: &CoupledBasis,
        cursor: u64,
    ) -> Result<(), String> {
        let actual = source.prepare_current(owner, actual_original, cursor)?;
        if self.snapshot()? != actual.snapshot()? {
            return Err("complete native receiving source/context/occasion/grant changed".into());
        }
        Ok(())
    }
}
