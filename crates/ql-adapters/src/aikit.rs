use ql_core::QlFormRef;
use ql_mef::{LensRef, SublensRef};
use ql_semantic::SemanticReading;
use ql_service::QlService;

use crate::{AdapterCore, AdapterError, AdapterResult, AiKitSubject, ClientRecord, QlMode};

pub struct AiKitAdapter<'a> {
    core: AdapterCore<'a>,
}

impl<'a> AiKitAdapter<'a> {
    pub const fn new(service: Option<&'a QlService>, mode: QlMode) -> Self {
        Self {
            core: AdapterCore::new(service, mode),
        }
    }

    pub fn refract<T>(
        &self,
        client: ClientRecord<AiKitSubject, T>,
        lens: LensRef,
        sublens: Option<SublensRef>,
        frame: Option<QlFormRef>,
    ) -> Result<AdapterResult<AiKitSubject, T, SemanticReading>, AdapterError> {
        self.core.refract(client, lens, sublens, frame)
    }
}
