use ql_core::QlFormRef;
use ql_mef::{LensRef, SublensRef};
use ql_semantic::SemanticReading;
use ql_service::QlService;

use crate::{AdapterCore, AdapterError, AdapterResult, ClientRecord, FactorySubject, QlMode};

pub struct FactoryAdapter<'a> {
    core: AdapterCore<'a>,
}

impl<'a> FactoryAdapter<'a> {
    pub const fn new(service: Option<&'a QlService>, mode: QlMode) -> Self {
        Self {
            core: AdapterCore::new(service, mode),
        }
    }

    pub fn refract<T>(
        &self,
        client: ClientRecord<FactorySubject, T>,
        lens: LensRef,
        sublens: Option<SublensRef>,
        frame: Option<QlFormRef>,
    ) -> Result<AdapterResult<FactorySubject, T, SemanticReading>, AdapterError> {
        self.core.refract(client, lens, sublens, frame)
    }
}
