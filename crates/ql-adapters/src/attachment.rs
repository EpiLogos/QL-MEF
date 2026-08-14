use ql_semantic::ProviderHealth;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QlMode {
    Disabled,
    Optional,
    Required,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QlAttachment<R> {
    Disabled,
    Reading {
        health: ProviderHealth,
        value: R,
    },
    Unavailable {
        health: ProviderHealth,
        reason: String,
    },
    Failed {
        health: Option<ProviderHealth>,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResult<S, T, R> {
    pub client: crate::ClientRecord<S, T>,
    pub ql: QlAttachment<R>,
}
