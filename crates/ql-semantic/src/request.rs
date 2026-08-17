use ql_core::QlFormRef;
use ql_mef::{LensRef, QlTarget, SublensRef};

use crate::SemanticReading;

/// A client-owned target together with the source revision observed by the caller.
///
/// The provider may use this pair for provenance, but does not own or reinterpret
/// either value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetInput {
    pub target: QlTarget,
    pub revision: Option<String>,
}

impl TargetInput {
    pub const fn new(target: QlTarget, revision: Option<String>) -> Self {
        Self { target, revision }
    }

    pub const fn unversioned(target: QlTarget) -> Self {
        Self {
            target,
            revision: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocateRequest {
    pub input: TargetInput,
    pub frame: Option<QlFormRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefractRequest {
    pub input: TargetInput,
    pub lens: LensRef,
    pub sublens: Option<SublensRef>,
    pub frame: Option<QlFormRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelateRequest {
    pub inputs: Vec<TargetInput>,
    pub frame: Option<QlFormRef>,
    pub lenses: Vec<LensRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesiseRequest {
    pub readings: Vec<SemanticReading>,
    pub frame: Option<QlFormRef>,
}
