use ql_core::QlFormRef;
use ql_mef::{LensRef, RefractionContract, SublensRef};
use ql_semantic::{ProviderHealth, SemanticReading};
use ql_service::{QlService, ServiceError};

use crate::{AdapterError, AdapterResult, AdapterSubject, ClientRecord, QlAttachment, QlMode};

pub(crate) struct AdapterCore<'a> {
    service: Option<&'a QlService>,
    mode: QlMode,
}

impl<'a> AdapterCore<'a> {
    pub const fn new(service: Option<&'a QlService>, mode: QlMode) -> Self {
        Self { service, mode }
    }

    pub fn refract<S, T>(
        &self,
        client: ClientRecord<S, T>,
        lens: LensRef,
        sublens: Option<SublensRef>,
        frame: Option<QlFormRef>,
    ) -> Result<AdapterResult<S, T, SemanticReading>, AdapterError>
    where
        S: AdapterSubject,
    {
        if self.mode == QlMode::Disabled {
            return Ok(AdapterResult {
                client,
                ql: QlAttachment::Disabled,
            });
        }

        let contract =
            RefractionContract::new(client.subject.client_subject().target(), lens, sublens)
                .map_err(AdapterError::InvalidRefraction)?;

        let Some(service) = self.service else {
            return match self.mode {
                QlMode::Required => Err(AdapterError::ServiceUnavailable),
                QlMode::Optional => Ok(AdapterResult {
                    client,
                    ql: QlAttachment::Unavailable {
                        health: ProviderHealth::absent(),
                        reason: "QL service not supplied".into(),
                    },
                }),
                QlMode::Disabled => unreachable!("disabled mode returned before service lookup"),
            };
        };

        let health = service.capabilities().health;
        let request = ql_semantic::RefractRequest {
            target: contract.target,
            lens: contract.lens,
            sublens: contract.sublens,
            frame,
        };

        match service.refract(request) {
            Ok(value) => Ok(AdapterResult {
                client,
                ql: QlAttachment::Reading { health, value },
            }),
            Err(error) if self.mode == QlMode::Required => Err(AdapterError::QlRequired(error)),
            Err(error) => Ok(AdapterResult {
                client,
                ql: optional_failure(health, error),
            }),
        }
    }
}

fn optional_failure(health: ProviderHealth, error: ServiceError) -> QlAttachment<SemanticReading> {
    match error {
        ServiceError::ProviderAbsent
        | ServiceError::ProviderIncompatible(_)
        | ServiceError::UnsupportedOperation(_) => QlAttachment::Unavailable {
            health,
            reason: error.to_string(),
        },
        other => QlAttachment::Failed {
            health: Some(health),
            message: other.to_string(),
        },
    }
}
