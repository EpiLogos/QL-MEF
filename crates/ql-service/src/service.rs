use ql_semantic::QlProvider;

pub struct QlService {
    pub(crate) provider: Option<Box<dyn QlProvider>>,
}

impl Default for QlService {
    fn default() -> Self {
        Self::new()
    }
}

impl QlService {
    pub const fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider<P: QlProvider + 'static>(provider: P) -> Self {
        Self {
            provider: Some(Box::new(provider)),
        }
    }

    pub fn replace_provider<P: QlProvider + 'static>(&mut self, provider: P) {
        self.provider = Some(Box::new(provider));
    }

    pub fn clear_provider(&mut self) {
        self.provider = None;
    }
}
