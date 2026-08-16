use ql_core::QlFormRef;
use ql_mef::{LensRef, MefError, SublensRef};
use ql_semantic::SemanticReading;
use ql_service::QlService;

use crate::{
    AdapterCore, AdapterError, AdapterResult, AdapterSubject, ClientRecord, ClientSubject, QlMode,
};

/// Client-owned identity for a Run, runtime event, or semantic closure.
///
/// The standalone QL/MEF module never assigns the underlying runtime identity.
/// It only gives that existing subject to the ordinary refraction adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSubject(ClientSubject);

impl RuntimeSubject {
    pub fn run(reference: impl Into<String>, revision: Option<String>) -> Result<Self, MefError> {
        ClientSubject::new(reference, revision)
            .map(|subject| Self(subject.with_subject_type("runtime-run")))
    }

    pub fn event(reference: impl Into<String>, revision: Option<String>) -> Result<Self, MefError> {
        ClientSubject::new(reference, revision)
            .map(|subject| Self(subject.with_subject_type("runtime-event")))
    }

    pub fn closure(
        reference: impl Into<String>,
        revision: Option<String>,
    ) -> Result<Self, MefError> {
        ClientSubject::new(reference, revision)
            .map(|subject| Self(subject.with_subject_type("runtime-closure")))
    }

    pub fn inner(&self) -> &ClientSubject {
        &self.0
    }
}

impl AdapterSubject for RuntimeSubject {
    fn client_subject(&self) -> &ClientSubject {
        &self.0
    }
}

/// Runtime selection remains a runtime concern and is retained unchanged beside
/// any optional semantic-provider result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSelection {
    pub id: String,
    pub revision: String,
}

impl RuntimeSelection {
    pub fn new(id: impl Into<String>, revision: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            revision: revision.into(),
        }
    }
}

/// Mirrors the frozen Factory optics distinction without turning semantic
/// closure into part of the generic runtime contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStatus {
    pub execution: String,
    pub semantic: String,
}

impl RuntimeStatus {
    pub fn new(execution: impl Into<String>, semantic: impl Into<String>) -> Self {
        Self {
            execution: execution.into(),
            semantic: semantic.into(),
        }
    }
}

/// An opaque runtime datum as observed through the frozen RuntimeObserver/run
/// record seam. `payload` remains owned by the runtime/client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvelope<T> {
    pub runtime: RuntimeSelection,
    pub status: RuntimeStatus,
    pub channel: Option<String>,
    pub event_type: Option<String>,
    pub payload: T,
}

impl<T> RuntimeEnvelope<T> {
    pub fn new(runtime: RuntimeSelection, status: RuntimeStatus, payload: T) -> Self {
        Self {
            runtime,
            status,
            channel: None,
            event_type: None,
            payload,
        }
    }

    pub fn with_event(mut self, channel: impl Into<String>, event_type: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self.event_type = Some(event_type.into());
        self
    }
}

/// Optional semantic projection over the frozen runtime seam.
///
/// Runtime selection is carried inside the client-owned envelope. Provider
/// selection is supplied independently when this adapter is constructed. The
/// bridge therefore cannot change which LoopRuntime runs, how RuntimeHost
/// dispatches carriers, or what RuntimeObserver records.
pub struct RuntimeRefractionAdapter<'a> {
    core: AdapterCore<'a>,
}

impl<'a> RuntimeRefractionAdapter<'a> {
    pub const fn new(service: Option<&'a QlService>, mode: QlMode) -> Self {
        Self {
            core: AdapterCore::new(service, mode),
        }
    }

    pub fn refract<T>(
        &self,
        client: ClientRecord<RuntimeSubject, RuntimeEnvelope<T>>,
        lens: LensRef,
        sublens: Option<SublensRef>,
        frame: Option<QlFormRef>,
    ) -> Result<AdapterResult<RuntimeSubject, RuntimeEnvelope<T>, SemanticReading>, AdapterError>
    {
        self.core.refract(client, lens, sublens, frame)
    }
}
