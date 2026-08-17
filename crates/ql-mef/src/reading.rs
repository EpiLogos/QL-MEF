use ql_core::{QlAddress, QlFormRef};

use crate::{ClientRef, LensRef, QlProvenance, QlTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlReading<T> {
    pub id: ClientRef,
    pub target: QlTarget,
    pub operation: String,
    pub ql_form: Option<QlFormRef>,
    pub address: Option<QlAddress>,
    pub lens: Option<LensRef>,
    pub reading: T,
    pub evidence_refs: Vec<ClientRef>,
    pub warnings: Vec<String>,
    pub provenance: QlProvenance,
}

impl<T> QlReading<T> {
    pub fn new(
        id: ClientRef,
        target: QlTarget,
        lens: Option<LensRef>,
        reading: T,
        provenance: QlProvenance,
    ) -> Self {
        Self {
            id,
            target,
            operation: provenance.operation.clone(),
            ql_form: None,
            address: None,
            lens,
            reading,
            evidence_refs: Vec::new(),
            warnings: Vec::new(),
            provenance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlRelationReading<R> {
    pub id: ClientRef,
    pub subjects: Vec<ClientRef>,
    pub frame: Option<QlFormRef>,
    pub relation: R,
    pub addresses: Vec<QlAddress>,
    pub lenses: Vec<LensRef>,
    pub evidence_refs: Vec<ClientRef>,
    pub provenance: QlProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlSynthesis<S> {
    pub id: ClientRef,
    pub input_readings: Vec<ClientRef>,
    pub synthesis: S,
    pub retained_differences: Vec<String>,
    pub unresolved: Vec<String>,
    pub provenance: QlProvenance,
}
