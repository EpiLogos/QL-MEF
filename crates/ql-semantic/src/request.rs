use ql_core::QlFormRef;
use ql_mef::{LensRef, QlTarget, SublensRef};

use crate::SemanticReading;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocateRequest {
    pub target: QlTarget,
    pub frame: Option<QlFormRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefractRequest {
    pub target: QlTarget,
    pub lens: LensRef,
    pub sublens: Option<SublensRef>,
    pub frame: Option<QlFormRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelateRequest {
    pub subjects: Vec<QlTarget>,
    pub frame: Option<QlFormRef>,
    pub lenses: Vec<LensRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesiseRequest {
    pub readings: Vec<SemanticReading>,
    pub frame: Option<QlFormRef>,
}
