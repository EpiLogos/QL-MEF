//! Bounded local control of the existing coupled owner. Pipe access is supplied
//! by the native host; a subject/reference is not a grant of authority.
use super::coupled::{CoupledFieldSession, CoupledInput};
use super::performance::{
    AuthoredCurrentPerformancePreparation, PerformanceCommand, PerformanceConfig, PerformanceOwner,
    prepare_current_configuration,
};
use super::performance_receiving::NativePerformanceReceivingSource;
use super::scene_field::{self, SceneConfig, SceneInstrument};
use super::{FieldInput, LiftInput};
use crate::musical_performance_return::ReturnContext;
use crate::procedural_conduct::{ConductHost, ConductRequest, NativePosition};
use crate::scene::{WorldRequest, world};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "procedural_field_timing.rs"]
mod procedural_field_timing;
#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "procedural_source_bootstrap.rs"]
mod procedural_source_bootstrap;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "receiving_readmission.rs"]
mod receiving_readmission;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use receiving_readmission::{
    NativeReceivingReadmissionRefusal, NativeReceivingReadmissionReply,
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "performance_recording.rs"]
mod performance_recording;

pub const WORLD_HOST_CONFIG: &str = "ql.field-host-world-config/v1";
pub const HOST_REQUEST: &str = "ql.field-host-request/v1";
pub const HOST_RECEIPT: &str = "ql.field-host-receipt/v1";
pub const MAX_HOST_INPUT: u64 = 32 * 1024 * 1024;
pub const MAX_HOST_OUTPUT: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostConfig {
    pub instance_ref: String,
    pub basis: CoupledInput,
    pub field: FieldInput,
}

/// Native World factory input retained under the existing host lease. The
/// source constructor independently regenerates its complete scene config;
/// a supplied scene/body/witness is deliberately absent from this contract.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldHostConfig {
    pub schema: String,
    pub instance_ref: String,
    pub world_request: WorldRequest,
    pub receiving_context: ReturnContext,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HostOperation {
    Procedure {
        request: Box<ConductRequest>,
    },
    PerformancePrepare {
        config: Box<PerformanceConfig>,
    },
    PerformanceExchange {
        command: Box<PerformanceCommand>,
    },
    /// Authored first-play policy; current source context and reduction are
    /// compiled by native owners, never supplied as observed UI receipts.
    PerformancePrepareCurrent {
        preparation: Box<AuthoredCurrentPerformancePreparation>,
    },
    Read {},
    Inspect {},
    Advance {
        frames: u32,
        muted: bool,
    },
    SetAxis {
        axis: u8,
        phase: LiftInput,
    },
    Replace {
        basis: Box<CoupledInput>,
    },
    /// scene only: D30 material policy; resident amplitudes and clocks continue.
    SetDamping {
        per_second: f64,
    },
    /// scene only: M1's own advance action, then the whole event is re-read.
    M1Advance {
        ticks: u64,
    },
    /// scene only: a changed caller event; `strike` re-excites the voices explicitly.
    ReplaceEvent {
        event: Box<CoupledInput>,
        strike: bool,
    },
    /// scene only: the acting influence reading with its basis and warrant.
    Influence {},
    /// scene with a Nara constitution: receive supplied seven-centre inputs
    /// against the current event. The material field does not change.
    ReceivePersonal {
        input: Box<crate::nara::PersonalEventInput>,
    },
    /// scene with a Nara constitution: the last reception and its currentness.
    Personal {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostRequest {
    pub schema: String,
    pub instance_ref: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub request_id: String,
    pub expected_generation: String,
    pub expected_samples_elapsed: String,
    pub command: HostOperation,
}

fn exact_cursor(text: &str) -> Result<u64, String> {
    let value: u64 = text.parse().map_err(|_| "invalid host cursor")?;
    if value.to_string() != text {
        return Err("noncanonical host cursor".into());
    }
    Ok(value)
}

/// The one owner a host holds: a fully supplied coupled field, or the scene
/// instrument whose geometry and voices the native provider composes.
enum Owner {
    Supplied(Box<CoupledFieldSession>),
    Scene(Box<SceneInstrument>),
}
impl Owner {
    fn session(&self) -> &CoupledFieldSession {
        match self {
            Self::Supplied(session) => session,
            Self::Scene(instrument) => instrument.session(),
        }
    }
    fn session_mut(&mut self) -> &mut CoupledFieldSession {
        match self {
            Self::Supplied(session) => session,
            Self::Scene(instrument) => instrument.session_mut(),
        }
    }
}

/// A single supplied instance, containing the full original/current engines.
/// Reads and inspection neither advance the field nor create another worker.
pub struct FieldHost {
    instance_ref: String,
    session: Owner,
    last_request: u64,
    performance: Option<PerformanceOwner>,
    procedural: ConductHost,
    receiving_source: Option<NativePerformanceReceivingSource>,
}
impl FieldHost {
    pub fn open(worker: &Path, config: HostConfig, timeout: Duration) -> Result<Self, String> {
        if config.instance_ref.is_empty()
            || config.instance_ref.len() > 2048
            || config.instance_ref.chars().any(char::is_control)
        {
            return Err("invalid native host instance reference".into());
        }
        Ok(Self {
            instance_ref: config.instance_ref,
            session: Owner::Supplied(Box::new(CoupledFieldSession::open(
                worker,
                config.basis,
                config.field,
                timeout,
            )?)),
            last_request: 0,
            performance: None,
            procedural: ConductHost::default(),
            receiving_source: None,
        })
    }

    pub fn open_scene(
        worker: &Path,
        config: SceneConfig,
        timeout: Duration,
    ) -> Result<Self, String> {
        let instrument = SceneInstrument::open(worker, config, timeout)?;
        Ok(Self {
            instance_ref: instrument.instance_ref().to_owned(),
            session: Owner::Scene(Box::new(instrument)),
            last_request: 0,
            performance: None,
            procedural: ConductHost::default(),
            receiving_source: None,
        })
    }

    /// The existing native Factory opens this under its actual World lease.
    /// Reconstruct before starting the existing worker; original sky/private
    /// source classification and native World completion are both checked.
    pub fn open_world(
        worker: &Path,
        config: WorldHostConfig,
        timeout: Duration,
    ) -> Result<Self, String> {
        if config.schema != WORLD_HOST_CONFIG
            || config.instance_ref != config.world_request.instance_ref
        {
            return Err("native World host source/instance differs".into());
        }
        let source = NativePerformanceReceivingSource::world_source(
            config.world_request.clone(),
            config.receiving_context,
        )?;
        let produced = world(config.world_request)?;
        let scene: SceneConfig = serde_json::from_value(produced["binding"]["host"].clone())
            .map_err(|e| e.to_string())?;
        let mut host = Self::open_scene(worker, scene, timeout)?;
        host.bind_performance_receiving_source(source)?;
        Ok(host)
    }
    /// Protected N owner calls this with its privately constructed actual
    /// profile/occasion/consent source while holding the current Act/lease.
    /// There is no HostOperation accepting a browser witness or definition.
    pub fn bind_performance_receiving_source(
        &mut self,
        source: NativePerformanceReceivingSource,
    ) -> Result<(), String> {
        if self.performance.is_some() {
            return Err(
                "receiving replacement requires an atomic retained source/body transaction".into(),
            );
        }
        if !self.available() {
            return Err("native current owner/lease unavailable".into());
        }
        self.receiving_source = Some(source);
        Ok(())
    }

    /// Chooses the owner from the configuration's own contract.
    pub fn open_config(worker: &Path, bytes: &[u8], timeout: Duration) -> Result<Self, String> {
        let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if value.get("schema").and_then(Value::as_str) == Some(WORLD_HOST_CONFIG) {
            let config: WorldHostConfig =
                serde_json::from_value(value).map_err(|e| e.to_string())?;
            return Self::open_world(worker, config, timeout);
        }
        if value.get("schema").and_then(Value::as_str) == Some(scene_field::CONFIG) {
            let config: SceneConfig = serde_json::from_value(value).map_err(|e| e.to_string())?;
            return Self::open_scene(worker, config, timeout);
        }
        let config: HostConfig = serde_json::from_value(value).map_err(|e| e.to_string())?;
        Self::open(worker, config, timeout)
    }

    pub fn available(&self) -> bool {
        self.session.session().available()
    }

    /// Stages closed authored receiver magnitudes through the SAME existing
    /// native source and stopped output owner. This publishes complete source
    /// assets for C retention; it does not grant an Act or install sound.
    pub(crate) fn stage_performance_acoustic(
        &mut self,
        configuration: super::performance::AcousticConfiguration,
    ) -> Result<Value, super::performance::AcousticRefusal> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?
            .clone()
            .with_acoustic_configuration(configuration)?;
        let pulse = self
            .performance
            .as_mut()
            .ok_or("native acoustic performance not active")?
            .stage_acoustic_receiving(&current, &source, self.session.session_mut())?;
        self.receiving_source = Some(source);
        Ok(pulse)
    }

    /// Explicit legacy staging is lawful only AFTER the actual C Scene CAS
    /// selects the complete independently prepared after-assets. The initial
    /// default uses install_performance_acoustic_candidate without staging.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn stage_performance_acoustic_candidate(
        &mut self,
        candidate: &super::performance::PreparedAcousticInstallation,
        source_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, super::performance::AcousticRefusal> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let original = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic source absent")?
            .clone();
        let after = original
            .clone()
            .with_acoustic_configuration(candidate.configuration().clone())?;
        source_lease.validate_source_assets(&self.instance_ref, candidate.source_assets())?;
        self.performance
            .as_ref()
            .ok_or("native acoustic performance absent")?
            .validate_stopped_acoustic_installation_candidate(
                &current, &original, &after, candidate,
            )?;
        let pulse = self.stage_performance_acoustic(candidate.configuration().clone())?;
        let checked = (|| -> Result<(), String> {
            let owner = self
                .performance
                .as_ref()
                .ok_or("native acoustic performance disappeared")?;
            owner.validate_current(&current)?;
            if owner.source_assets() != candidate.source_assets() {
                return Err(
                    "actual staged native source differs from full selected candidate".into(),
                );
            }
            source_lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
            Ok(())
        })();
        match checked {
            Ok(()) => Ok(pulse),
            Err(reason) => {
                let reason = self.session.session_mut().performance_invalidate(&reason);
                Err(super::performance::AcousticRefusal::retaining_native_pulse(
                    reason, pulse,
                ))
            }
        }
    }

    /// Only the closed selected-Act channel supplies this native lease.
    /// Public Exchange/HostOperation never accepts an acoustic witness Value.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn install_performance_acoustic(
        &mut self,
        act_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, super::performance::AcousticRefusal> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?;
        self.performance
            .as_mut()
            .ok_or("native acoustic performance not active")?
            .install_prepared_acoustic_receiving(
                &current,
                source,
                self.session.session_mut(),
                act_lease,
            )
    }

    /// Pure initial candidate over the SAME original native source and copied
    /// stopped boundary. No inspect, drain, source change or receiver install.
    pub(crate) fn prepare_performance_acoustic_installation(
        &self,
        configuration: super::performance::AcousticConfiguration,
    ) -> Result<super::performance::PreparedAcousticInstallation, String> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis();
        let original = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?;
        let after = original
            .clone()
            .with_acoustic_configuration(configuration)?;
        self.performance
            .as_ref()
            .ok_or("native acoustic performance not active")?
            .prepare_stopped_acoustic_installation_assets(current, original, &after)
    }
    /// Fresh closed native Scene/Act selection must match the full candidate
    /// after-assets. Source publication follows the accepted actual install.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn install_performance_acoustic_candidate(
        &mut self,
        candidate: &super::performance::PreparedAcousticInstallation,
        source_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, super::performance::AcousticRefusal> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let original = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?
            .clone();
        let after = original
            .clone()
            .with_acoustic_configuration(candidate.configuration().clone())?;
        let pulse = self
            .performance
            .as_mut()
            .ok_or("native acoustic performance not active")?
            .install_acoustic_candidate(
                &current,
                &original,
                &after,
                self.session.session_mut(),
                candidate,
                source_lease,
            )?;
        self.receiving_source = Some(after);
        Ok(pulse)
    }

    /// Numerical/source preparation for the existing current Scene CAS. This
    /// returns complete candidate assets and never applies a receiver edit.
    pub(crate) fn prepare_performance_acoustic_update(
        &self,
        configuration: super::performance::AcousticConfiguration,
    ) -> Result<super::performance::PreparedAcousticReceiverUpdate, String> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?
            .clone()
            .with_acoustic_configuration(configuration)?;
        let owner = self
            .performance
            .as_ref()
            .ok_or("native acoustic performance not active")?;
        owner.prepare_stopped_acoustic_receiver_assets(current, &source)
    }
    /// Only a genuinely closed selected current Scene/recorded source lease
    /// matching the complete authored after-assets can apply this candidate.
    /// The owner independently checks its original operative state first.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn replace_performance_acoustic(
        &mut self,
        candidate: &super::performance::PreparedAcousticReceiverUpdate,
        source_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<Value, super::performance::AcousticRefusal> {
        if !self.available() {
            return Err("held native acoustic source owner unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original acoustic receiving source not bound")?
            .clone()
            .with_acoustic_configuration(candidate.configuration().clone())?;
        let pulse = self
            .performance
            .as_mut()
            .ok_or("native acoustic performance not active")?
            .replace_prepared_acoustic_receiving(
                &current,
                &source,
                self.session.session_mut(),
                candidate,
                source_lease,
            )?;
        self.receiving_source = Some(source);
        Ok(pulse)
    }

    /// Descriptor from the current #281 output owner under the SAME privately
    /// selected C/Act lease. This does not call FIELD's no-performance read or
    /// replace its fence, last_field or sample/generation domain.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn procedural_timing_descriptor(
        &mut self,
        act_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
    ) -> Result<
        super::performance::PreparedProceduralTimingDescriptor,
        super::performance::NativeTimingRefusal,
    > {
        if !self.available() {
            return Err("held native timing owner/lease unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual native timing receiving source not bound")?;
        self.performance
            .as_mut()
            .ok_or("native timing performance not active")?
            .procedural_timing_descriptor(&current, source, self.session.session_mut(), act_lease)
    }

    /// Sole private selected-Act owner operation supplies the typed C lease.
    /// No public HostOperation or Request::Exchange accepts witness JSON.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn procedural_timing_witness(
        &mut self,
        original_binding: crate::procedural_composition::TimingBinding,
        act_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
        moment: super::performance::NativeTimingMoment,
    ) -> Result<super::performance::PreparedProceduralTiming, super::performance::NativeTimingRefusal>
    {
        if !self.available() {
            return Err("held native timing owner/lease unavailable".into());
        }
        let current = self.session.session().current_basis().clone();
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual native timing receiving source not bound")?;
        self.performance
            .as_mut()
            .ok_or("native timing performance not active")?
            .procedural_timing_witness(
                &current,
                source,
                self.session.session_mut(),
                original_binding,
                act_lease,
                moment,
            )
    }

    fn response(&self, request_id: Option<&str>, status: &str, error: Option<&str>) -> Value {
        // This is the LAST ACKNOWLEDGED field, not a claim of live state after
        // transport loss. Full original/current sources are only sent on Inspect.
        let mut field = self.session.session().last_field().clone();
        field["audio"] = json!([]);
        json!({"schema":HOST_RECEIPT, "instance_ref":self.instance_ref,
            "request_id":request_id, "last_request_id":self.last_request.to_string(),
            "status":status, "available":self.available(), "error":error,
            "field":field,
            "standing":"local single-owner control; caller-supplied pipe authority; last acknowledged native state"})
    }

    pub fn ready(&self) -> Value {
        self.response(None, "ready", None)
    }

    /// Native C/Act retention consumes the actual activated owner, not a
    /// preparation-only source bundle assembled beside it. This is an immutable
    /// source artifact, not a device/output or checkpoint admission.
    pub fn retained_performance_source_artifact(
        &self,
        declared_seed: u64,
    ) -> Result<Value, String> {
        let owner = self
            .performance
            .as_ref()
            .ok_or("actual retained performance is not active")?;
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving source is not bound")?;
        let current = self.session.session().current_basis();
        if !self.available() || owner.reading().is_none() {
            return Err("native activated owner/current readback unavailable".into());
        }
        // Full original/current producer, receiver, occasion and consent replay
        // precedes retention. A stored snapshot or binding-only setter is not it.
        let prepared_receiving = source.prepare_current(owner, current, 0)?;
        if prepared_receiving.snapshot()? != owner.source_assets()["current_receiving"] {
            return Err(
                "activated native source artifact lost exact current receiving owner".into(),
            );
        }
        let returned = crate::musical_performance_return::bind_performance_return(
            owner.binding(),
            source.original_occasion().cloned(),
            source.return_context().clone(),
            declared_seed,
        )?;
        Ok(json!({"schema":"ql.retained-source-performance-fixture/v1",
            "basis":returned.expression_basis()?,"pitches":returned.expression_pitches(0)?,
            "source_assets":owner.source_assets(),"native_preparation":owner.native_packet()?,
            "native_basis":owner.binding().native_basis(),"native_reading":owner.reading(),
            "standing":"actual activated existing FieldHost/PerformanceOwner/worker; source artifact only, no hardware output or file/Act acceptance"}))
    }

    /// Reached by the existing guarded native Kernel/Act reader callback, not
    /// by a public HostOperation carrying a user Value or imported scope.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn admit_native_act(
        &mut self,
        request: &super::performance_act_bridge::ActRequest,
    ) -> Result<(), String> {
        let field = self.session.session().last_field();
        if request.instance_ref != self.instance_ref
            || field["event_ref"] != request.event_ref
            || field["subject_ref"] != request.subject_ref
            || !self.available()
            || (!request.is_field() && self.performance.is_none())
            || (self.performance.is_some() && self.receiving_source.is_none())
        {
            return Err("native selected Act has no exact available original field/performance/receiving owner".into());
        }
        let id = exact_cursor(&request.request_id)?;
        if self.last_request.checked_add(1) != Some(id) {
            return Err("native selected Act request is stale/repeated/skipped".into());
        }
        self.last_request = id;
        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn native_act_result(&self, id: &str, result: &Result<Value, String>) -> Value {
        json!({"schema":"ql.native-act-owner-result/v1","instance_ref":self.instance_ref,
            "request_id":id,"last_request_id":self.last_request.to_string(),"available":self.available(),
            "status":if result.is_ok(){"ok"}else{"refused"},"result":result.as_ref().ok(),"error":result.as_ref().err()})
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn with_native_act_score_source<T>(
        &self,
        manifest: &Value,
        consume: impl FnOnce(
            &[crate::musical_performance_source_score::RetainedScoreSource<'_>],
        ) -> Result<T, String>,
    ) -> Result<T, String> {
        let owner = self
            .performance
            .as_ref()
            .ok_or("actual performance owner absent")?;
        let receiving = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving owner absent")?;
        let current = self.session.session().current_basis();
        let seed = manifest["performance"]["bases"][0]["seed"]
            .as_str()
            .ok_or("selected native seed absent")?;
        let declared_seed = seed.parse::<u64>().map_err(|e| e.to_string())?;
        if seed != declared_seed.to_string() {
            return Err("selected native seed is not canonical".into());
        }
        let returned = crate::musical_performance_return::bind_performance_return(
            owner.binding(),
            receiving.original_occasion().cloned(),
            receiving.return_context().clone(),
            declared_seed,
        )?;
        let sample=owner.source_assets()["current_receiving"]["native_admission"]["operation"]["native_sample"].as_str()
            .ok_or("actual original receiving admitted cursor absent")?.parse::<u64>().map_err(|e|e.to_string())?;
        let source = crate::musical_performance_source_score::RetainedScoreSource {
            owner,
            current,
            receiving,
            original_return: &returned,
            source_sample: sample,
        };
        consume(&[source])
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn compile_native_act_score(
        &self,
        generation: u64,
        manifest: &Value,
        pages: &mut impl crate::musical_performance_source_score::NativeScorePages,
    ) -> Result<Value, String> {
        self.with_native_act_score_source(manifest, |sources| {
            crate::musical_performance_source_score::compile_retained_score(
                generation, sources, manifest, pages,
            )?
            .snapshot()
        })
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn prepare_native_act_render(
        &self,
        generation: u64,
        manifest: &Value,
        pages: &mut impl crate::musical_performance_source_score::NativeScorePages,
        from: u64,
        to: u64,
    ) -> Result<super::performance_export::NativeActRenderPlan, String> {
        self.with_native_act_score_source(manifest, |sources| {
            super::performance_export::NativeActRenderPlan::prepare(
                generation, sources, manifest, pages, from, to,
            )
        })
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn render_native_act<T>(
        &mut self,
        plan: &super::performance_export::NativeActRenderPlan,
        checkpoints: &mut impl super::performance_export::NativeActCheckpoints,
        consume: impl FnOnce(&mut super::performance_export::NativeActRenderer<'_>) -> Result<T, String>,
    ) -> Result<
        super::performance_export::NativeRenderResult<Result<T, String>>,
        super::performance_export::NativeRenderFailure,
    > {
        let current = self.session.session().current_basis().clone();
        let receiving = self
            .receiving_source
            .as_ref()
            .ok_or("actual original receiving owner absent")?;
        let owner = self
            .performance
            .as_mut()
            .ok_or("actual performance owner absent")?;
        super::performance_export::with_stopped_render(
            plan,
            owner,
            &current,
            receiving,
            self.session.session_mut(),
            checkpoints,
            consume,
        )
    }

    /// Bad JSON/unknown fields have no admitted sequence and never reach C++.
    pub fn reject_input(&self, error: &str) -> Value {
        self.response(None, "refused", Some(error))
    }

    fn admit(&mut self, request: &HostRequest) -> Result<(), String> {
        let field = self.session.session().last_field();
        if request.schema != HOST_REQUEST
            || request.instance_ref != self.instance_ref
            || field["event_ref"] != request.event_ref
            || field["subject_ref"] != request.subject_ref
        {
            return Err("host request has a foreign schema/instance/event/subject".into());
        }
        let sequence = exact_cursor(&request.request_id)?;
        if self.last_request.checked_add(1) != Some(sequence) {
            return Err("stale, repeated or skipped host request sequence".into());
        }
        // An admitted scoped envelope consumes its sequence even if its command
        // is refused. A lost acknowledgement must never be retried implicitly.
        self.last_request = sequence;
        exact_cursor(&request.expected_generation)?;
        exact_cursor(&request.expected_samples_elapsed)?;
        let performance_exchange =
            matches!(request.command, HostOperation::PerformanceExchange { .. });
        if !performance_exchange
            && (field["generation"] != request.expected_generation
                || field["samples_elapsed"] != request.expected_samples_elapsed)
        {
            return Err("host request is based on a stale native cursor".into());
        }
        if !self.available() {
            return Err("native transport standing unknown; explicit new owner required".into());
        }
        Ok(())
    }

    /// C's closed selected-Act operation is the only caller. The typed lease
    /// and native timing selector never come from HostOperation/browser JSON.
    /// R supplies the SAME held-owner field/audio factory; the full native pulse
    /// remains available for original C/S recording, separate from material ACK.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub(crate) fn execute_native_procedure(
        &mut self,
        request: HostRequest,
        act_lease: &super::performance_act_bridge::NativeActSourceLease<'_>,
        moment: super::performance::NativeTimingMoment,
    ) -> Value {
        if let Err(error) = self.admit(&request) {
            let status = if self.available() {
                "refused"
            } else {
                "unavailable"
            };
            return self.response(Some(&request.request_id), status, Some(&error));
        }
        let HostOperation::Procedure {
            request: procedure_request,
        } = request.command
        else {
            return self.response(
                Some(&request.request_id),
                "refused",
                Some("native selected-Act conduct route requires a procedure command"),
            );
        };
        // A valid same-worker timing exchange can drain real applications
        // even when its selector or subsequent Source conduct refuses. Keep
        // that full pulse for the private C/S receiving owner on BOTH paths.
        let mut timing_pulse = None;
        let mut field_timing_receipt = None;
        let result: Result<Value, String> = (|| {
            if let ConductRequest::SourceBootstrap { input } = &*procedure_request {
                if !matches!(moment, super::performance::NativeTimingMoment::Boundary) {
                    return Err(
                        "native source bootstrap requires SAME native owner Boundary descriptor"
                            .into(),
                    );
                }
                return match self.bootstrap_procedural_source(input, act_lease) {
                    Ok(result) => {
                        field_timing_receipt = result
                            .get("native_field_receipt")
                            .filter(|v| !v.is_null())
                            .cloned();
                        timing_pulse = result
                            .get("native_timing_pulse")
                            .filter(|v| !v.is_null())
                            .cloned();
                        Ok(result)
                    }
                    Err(refusal) => {
                        field_timing_receipt = refusal.native_receipt().cloned();
                        timing_pulse = refusal.native_timing_pulse().cloned();
                        Err(refusal.reason().to_owned())
                    }
                };
            }
            if matches!(&*procedure_request, ConductRequest::LifecycleCancel { .. }) {
                let ConductRequest::LifecycleCancel { input } = *procedure_request else {
                    unreachable!()
                };
                input
                    .validate_reading(self.procedural.lifecycle_original(&input.procedure_ref)?)?;
                act_lease.validate_procedural_scene_read(
                    &self.instance_ref,
                    &input.scene_read,
                    &input.contributors,
                )?;
                let source_read = input.scene_read.clone();
                let contributors = input.contributors.clone();
                let staged = self.procedural.stage_lifecycle_cancel(*input)?;
                act_lease.validate_procedural_scene_read(
                    &self.instance_ref,
                    &source_read,
                    &contributors,
                )?;
                act_lease.validate_field_sources(
                    &self.instance_ref,
                    self.session.session().original_basis(),
                    self.session.session().current_basis(),
                )?;
                return self.procedural.commit_lifecycle(staged);
            }
            if matches!(&*procedure_request, ConductRequest::Lifecycle { .. }) {
                let ConductRequest::Lifecycle { input } = *procedure_request else {
                    unreachable!()
                };
                input
                    .validate_reading(self.procedural.lifecycle_original(&input.procedure_ref)?)?;
                let subjects = input.selected_contributors()?;
                act_lease.validate_procedural_scene_read(
                    &self.instance_ref,
                    &input.reading.scene_read,
                    &subjects,
                )?;
                let lifecycle_read = input.reading.scene_read.clone();
                let binding = self
                    .procedural
                    .lifecycle_original(&input.procedure_ref)?
                    .procedure
                    .timing
                    .clone();
                let staged = if binding.domain
                    == procedural_field_timing::NATIVE_FIELD_TIMING_DOMAIN
                {
                    if !matches!(moment, super::performance::NativeTimingMoment::Boundary) {
                        return Err(
                            "native field lifecycle has no audio queue/application selector".into(),
                        );
                    }
                    let actual = match self.field_procedural_timing_witness(binding, act_lease) {
                        Ok(actual) => actual,
                        Err(refusal) => {
                            field_timing_receipt = refusal.native_receipt().cloned();
                            return Err(refusal.reason().to_owned());
                        }
                    };
                    field_timing_receipt = Some(actual.native_receipt().clone());
                    self.procedural.stage_lifecycle(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                    )?
                } else {
                    let actual = match self.procedural_timing_witness(binding, act_lease, moment) {
                        Ok(actual) => actual,
                        Err(refusal) => {
                            timing_pulse = refusal.native_pulse().cloned();
                            return Err(refusal.reason().to_owned());
                        }
                    };
                    timing_pulse = Some(actual.native_pulse().clone());
                    self.procedural.stage_lifecycle(
                        *input,
                        actual.position().clone(),
                        actual.witness(),
                    )?
                };
                act_lease.validate_procedural_scene_read(
                    &self.instance_ref,
                    &lifecycle_read,
                    &subjects,
                )?;
                act_lease.validate_field_sources(
                    &self.instance_ref,
                    self.session.session().original_basis(),
                    self.session.session().current_basis(),
                )?;
                return self.procedural.commit_lifecycle(staged);
            }
            let binding = self.procedural.request_timing(&procedure_request)?;
            if let Some(binding) = binding {
                if binding.domain == procedural_field_timing::NATIVE_FIELD_TIMING_DOMAIN {
                    if !matches!(moment, super::performance::NativeTimingMoment::Boundary) {
                        return Err(
                            "native field timing has no audio queue/clock/application selector"
                                .into(),
                        );
                    }
                    let actual = match self.field_procedural_timing_witness(binding, act_lease) {
                        Ok(actual) => actual,
                        Err(refusal) => {
                            field_timing_receipt = refusal.native_receipt().cloned();
                            return Err(refusal.reason().to_owned());
                        }
                    };
                    field_timing_receipt = Some(actual.native_receipt().clone());
                    return self.procedural.execute(
                        *procedure_request,
                        actual.position().clone(),
                        Some(actual.witness()),
                    );
                }
                let actual = match self.procedural_timing_witness(binding, act_lease, moment) {
                    Ok(actual) => actual,
                    Err(refusal) => {
                        timing_pulse = refusal.native_pulse().cloned();
                        return Err(refusal.reason().to_owned());
                    }
                };
                timing_pulse = Some(actual.native_pulse().clone());
                self.procedural.execute(
                    *procedure_request,
                    actual.position().clone(),
                    Some(actual.witness()),
                )
            } else {
                let position = NativePosition::from_field(
                    &self.instance_ref,
                    self.session.session().last_field(),
                )?;
                self.procedural.execute(*procedure_request, position, None)
            }
        })();
        let mut response = match result {
            Ok(procedural) => {
                let mut response = self.response(Some(&request.request_id), "ok", None);
                response["procedural"] = procedural;
                response
            }
            Err(error) => self.response(
                Some(&request.request_id),
                if self.available() {
                    "refused"
                } else {
                    "unavailable"
                },
                Some(&error),
            ),
        };
        if let Some(pulse) = timing_pulse {
            response["native_timing_pulse"] = pulse;
        }
        if let Some(receipt) = field_timing_receipt {
            response["native_field_timing_receipt"] = receipt;
        }
        response
    }

    pub fn execute(&mut self, request: HostRequest) -> Value {
        if let Err(error) = self.admit(&request) {
            let status = if self.available() {
                "refused"
            } else {
                "unavailable"
            };
            return self.response(Some(&request.request_id), status, Some(&error));
        }
        if matches!(&request.command, HostOperation::Procedure { .. }) {
            let position =
                NativePosition::from_field(&self.instance_ref, self.session.session().last_field());
            let HostOperation::Procedure {
                request: procedure_request,
            } = request.command
            else {
                unreachable!()
            };
            let result = position.and_then(|position| {
                let binding = self.procedural.request_timing(&procedure_request)?;
                if binding.is_some() {
                    return Err("performative conduct requires the private native selected-Act/source lease route".into());
                }
                self.procedural.execute(*procedure_request, position, None)
            });
            return match result {
                Ok(procedural) => {
                    let mut response = self.response(Some(&request.request_id), "ok", None);
                    response["procedural"] = procedural;
                    response
                }
                Err(error) => self.response(Some(&request.request_id), "refused", Some(&error)),
            };
        }
        if matches!(
            &request.command,
            HostOperation::PerformancePrepare { .. }
                | HostOperation::PerformancePrepareCurrent { .. }
                | HostOperation::PerformanceExchange { .. }
        ) {
            let current = self.session.session().current_basis().clone();
            let result = match request.command {
                HostOperation::PerformancePrepare { config } => {
                    if self.performance.is_some() {
                        Err("retained native performance already owns this work".into())
                    } else {
                        PerformanceOwner::prepare(&current, &self.instance_ref, *config).and_then(
                            |mut performance| {
                                let reply = match &self.receiving_source {
                                    Some(source) => performance.activate_with_current_receiving(
                                        &current,
                                        self.session.session_mut(),
                                        source,
                                    )?,
                                    None => performance
                                        .activate(&current, self.session.session_mut())?,
                                };
                                self.performance = Some(performance);
                                Ok(reply)
                            },
                        )
                    }
                }
                HostOperation::PerformancePrepareCurrent { preparation } => {
                    if self.performance.is_some() {
                        Err("retained native performance already owns this work".into())
                    } else {
                        prepare_current_configuration(&current, &self.instance_ref, *preparation)
                            .and_then(|config| {
                                PerformanceOwner::prepare(&current, &self.instance_ref, config)
                            })
                            .and_then(|mut performance| {
                                let reply = match &self.receiving_source {
                                    Some(source) => performance.activate_with_current_receiving(
                                        &current,
                                        self.session.session_mut(),
                                        source,
                                    )?,
                                    None => performance
                                        .activate(&current, self.session.session_mut())?,
                                };
                                self.performance = Some(performance);
                                Ok(reply)
                            })
                    }
                }
                HostOperation::PerformanceExchange { command } => match self.performance.as_mut() {
                    Some(performance) => {
                        performance.execute(&current, self.session.session_mut(), *command)
                    }
                    None => Err("retained native performance has not been prepared".into()),
                },
                _ => unreachable!(),
            };
            return match result {
                Ok(performance) => {
                    let mut response = self.response(Some(&request.request_id), "ok", None);
                    response["performance"] = performance;
                    response
                }
                Err(error) => self.response(
                    Some(&request.request_id),
                    if self.available() {
                        "refused"
                    } else {
                        "unavailable"
                    },
                    Some(&error),
                ),
            };
        }
        if self.performance.is_some()
            && !matches!(
                request.command,
                HostOperation::Inspect {}
                    | HostOperation::Influence {}
                    | HostOperation::Personal {}
            )
        {
            return self.response(Some(&request.request_id),"refused",Some("retained A/P owns native time/body; prepared source/material transaction required"));
        }
        if matches!(&request.command, HostOperation::Inspect { .. }) {
            let mut response = self.response(Some(&request.request_id), "ok", None);
            let session = self.session.session();
            if let Some(performance) = &self.performance {
                response["performance_reading"] =
                    performance.reading().cloned().unwrap_or(Value::Null);
                response["performance_sources"] = performance.source_assets().clone();
            }
            response["sources"] = json!({"original":session.original_basis(),
                "current":session.current_basis(), "original_field":session.original_field()});
            if let Owner::Scene(instrument) = &self.session {
                response["influence"] = instrument.influence();
                response["event"] = json!(instrument.event());
            }
            return response;
        }
        if matches!(
            &request.command,
            HostOperation::ReceivePersonal { .. } | HostOperation::Personal {}
        ) {
            let Owner::Scene(instrument) = &mut self.session else {
                return self.response(
                    Some(&request.request_id),
                    "refused",
                    Some("personal reception belongs to a scene owner with a Nara constitution"),
                );
            };
            let result = match request.command {
                HostOperation::ReceivePersonal { input } => instrument
                    .receive_personal(*input)
                    .and_then(|_| instrument.personal_reading()),
                _ => instrument.personal_reading(),
            };
            return match result {
                Ok(personal) => {
                    let mut response = self.response(Some(&request.request_id), "ok", None);
                    response["personal"] = personal;
                    response
                }
                Err(error) => self.response(Some(&request.request_id), "refused", Some(&error)),
            };
        }
        if matches!(&request.command, HostOperation::Influence {}) {
            let mut response = self.response(Some(&request.request_id), "ok", None);
            match &self.session {
                Owner::Scene(instrument) => response["influence"] = instrument.influence(),
                Owner::Supplied(_) => {
                    return self.response(
                        Some(&request.request_id),
                        "refused",
                        Some("influence reading belongs to a provider-composed scene owner"),
                    );
                }
            }
            return response;
        }
        // A scene determinant event answers with its new influence reading, so a
        // consumer needs no second exchange while its audio waits.
        let determinant = matches!(
            &request.command,
            HostOperation::M1Advance { .. }
                | HostOperation::ReplaceEvent { .. }
                | HostOperation::SetDamping { .. }
        );
        let result = match (request.command, &mut self.session) {
            (HostOperation::Read {}, owner) => owner.session_mut().read_field(),
            (HostOperation::Advance { frames, muted }, owner) => {
                if frames > 8192 {
                    Err("native block ceiling exceeded".into())
                } else {
                    owner.session_mut().advance_field(frames, muted)
                }
            }
            (HostOperation::SetAxis { axis, phase }, owner) => {
                if axis > 1 {
                    Err("unknown independent clock axis".into())
                } else {
                    owner.session_mut().set_axis_field(axis, phase)
                }
            }
            (HostOperation::Replace { basis }, Owner::Supplied(session)) => {
                session.replace_field(*basis)
            }
            (HostOperation::SetDamping { per_second }, Owner::Scene(instrument)) => {
                instrument.set_damping(per_second)
            }
            (HostOperation::M1Advance { ticks }, Owner::Scene(instrument)) => {
                instrument.m1_advance(ticks)
            }
            (HostOperation::ReplaceEvent { event, strike }, Owner::Scene(instrument)) => {
                instrument.replace(&event, strike)
            }
            (HostOperation::Replace { .. }, Owner::Scene(_)) => {
                Err("a scene owner composes its own voices; use replace-event".into())
            }
            (
                HostOperation::M1Advance { .. }
                | HostOperation::ReplaceEvent { .. }
                | HostOperation::SetDamping { .. },
                _,
            ) => Err("determinant operations belong to a provider-composed scene owner".into()),
            (
                HostOperation::Procedure { .. }
                | HostOperation::PerformancePrepare { .. }
                | HostOperation::PerformancePrepareCurrent { .. }
                | HostOperation::PerformanceExchange { .. }
                | HostOperation::Inspect {}
                | HostOperation::Influence {}
                | HostOperation::ReceivePersonal { .. }
                | HostOperation::Personal {},
                _,
            ) => unreachable!("reads and reception returned before dispatch"),
        };
        match result {
            Ok(field) => {
                let mut response = self.response(Some(&request.request_id), "ok", None);
                response["field"] = field;
                if let (true, Owner::Scene(instrument)) = (determinant, &self.session) {
                    response["influence"] = instrument.influence();
                }
                response
            }
            Err(error) => {
                let status = if self.available() {
                    "refused"
                } else {
                    "unavailable"
                };
                self.response(Some(&request.request_id), status, Some(&error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_cursors_keep_the_full_u64_range_without_aliases() {
        for text in ["", "01", "+1", "-0", " 1", "1e3", "18446744073709551616"] {
            assert!(exact_cursor(text).is_err());
        }
        assert_eq!(exact_cursor("18446744073709551615").unwrap(), u64::MAX);
    }

    #[test]
    fn command_contract_rejects_unknown_fields_and_implicit_mute() {
        for value in [
            json!({"operation":"read", "advance":true}),
            json!({"operation":"advance", "frames":128}),
            json!({"operation":"advance", "frames":1.5,"muted":false}),
            json!({"operation":"shutdown"}),
        ] {
            assert!(
                serde_json::from_value::<HostOperation>(value.clone()).is_err(),
                "accepted {value}"
            );
        }
        let command: HostOperation = serde_json::from_value(json!({"operation":"read"})).unwrap();
        assert!(matches!(command, HostOperation::Read {}));
    }
}

#[cfg(test)]
#[path = "performance_acoustic_installation_tests.rs"]
mod acoustic_initial_tests;

#[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
#[path = "dense_field_source_tests.rs"]
mod dense_field_source_tests;
