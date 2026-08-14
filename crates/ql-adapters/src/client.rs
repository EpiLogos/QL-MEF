use ql_mef::{ClientRef, MefError, QlTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientSubject {
    reference: ClientRef,
    revision: Option<String>,
    subject_type: Option<String>,
    frame_ref: Option<String>,
    context_refs: Vec<ClientRef>,
}

impl ClientSubject {
    pub fn new(reference: impl Into<String>, revision: Option<String>) -> Result<Self, MefError> {
        Ok(Self {
            reference: ClientRef::new(reference)?,
            revision,
            subject_type: None,
            frame_ref: None,
            context_refs: Vec::new(),
        })
    }

    pub fn with_subject_type(mut self, subject_type: impl Into<String>) -> Self {
        self.subject_type = Some(subject_type.into());
        self
    }

    pub fn with_frame_ref(mut self, frame_ref: impl Into<String>) -> Self {
        self.frame_ref = Some(frame_ref.into());
        self
    }

    pub fn with_context_ref(mut self, reference: ClientRef) -> Self {
        self.context_refs.push(reference);
        self
    }

    pub fn reference(&self) -> &ClientRef {
        &self.reference
    }

    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    pub fn target(&self) -> QlTarget {
        QlTarget {
            subject: self.reference.clone(),
            subject_type: self.subject_type.clone(),
            frame_ref: self.frame_ref.clone(),
            context_refs: self.context_refs.clone(),
        }
    }
}

pub trait AdapterSubject {
    fn client_subject(&self) -> &ClientSubject;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactorySubject(ClientSubject);

impl FactorySubject {
    pub fn new(reference: impl Into<String>, revision: Option<String>) -> Result<Self, MefError> {
        ClientSubject::new(reference, revision).map(Self)
    }

    pub fn inner(&self) -> &ClientSubject {
        &self.0
    }
}

impl AdapterSubject for FactorySubject {
    fn client_subject(&self) -> &ClientSubject {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiKitSubject(ClientSubject);

impl AiKitSubject {
    pub fn new(reference: impl Into<String>, revision: Option<String>) -> Result<Self, MefError> {
        ClientSubject::new(reference, revision).map(Self)
    }

    pub fn inner(&self) -> &ClientSubject {
        &self.0
    }
}

impl AdapterSubject for AiKitSubject {
    fn client_subject(&self) -> &ClientSubject {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientRecord<S, T> {
    pub subject: S,
    pub payload: T,
}

impl<S, T> ClientRecord<S, T> {
    pub const fn new(subject: S, payload: T) -> Self {
        Self { subject, payload }
    }
}
