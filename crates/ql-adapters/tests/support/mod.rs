use ql_mef::{ClientRef, InputRefRevision, QlProvenance, QlProviderRef, QlReading, ResultClass};
use ql_semantic::{
    InputLimits, LocateRequest, LocateResult, Operation, ProviderCapabilities, ProviderClass,
    ProviderError, ProviderHealth, QlProvider, RefractRequest, RelateRequest, SemanticDisclosure,
    SemanticReading, SemanticRelationReading, SemanticStatus, SemanticSynthesis, SynthesiseRequest,
};

#[derive(Debug, Clone)]
pub struct AdapterFixtureProvider {
    capabilities: ProviderCapabilities,
    fail_refract: bool,
}

impl AdapterFixtureProvider {
    pub fn semantic(id: &str, health: ProviderHealth) -> Self {
        Self {
            capabilities: ProviderCapabilities {
                provider: QlProviderRef::new(id, "0.1.0").expect("provider ref"),
                health,
                classes: vec![ProviderClass::SemanticRefraction],
                supported_forms: vec![],
                supported_lenses: ql_mef::LensId::ALL
                    .into_iter()
                    .map(ql_mef::LensRef::canonical)
                    .collect(),
                operations: vec![Operation::Capabilities, Operation::Refract],
                extension_namespaces: vec![],
                deterministic_operations: vec![Operation::Capabilities],
                input_limits: InputLimits {
                    max_relation_subjects: 2,
                    max_synthesis_readings: 1,
                },
                output_schema_versions: vec!["ql-contract/1.1.0".into()],
            },
            fail_refract: false,
        }
    }

    pub fn formal_only(id: &str) -> Self {
        let mut provider = Self::semantic(id, ProviderHealth::available());
        provider.capabilities.classes = vec![ProviderClass::FormalKernel];
        provider.capabilities.supported_lenses.clear();
        provider.capabilities.operations = vec![Operation::Capabilities];
        provider
    }

    pub fn failing(id: &str) -> Self {
        let mut provider = Self::semantic(id, ProviderHealth::available());
        provider.fail_refract = true;
        provider
    }
}

impl QlProvider for AdapterFixtureProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn locate(&self, _request: LocateRequest) -> Result<LocateResult, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Locate))
    }

    fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ProviderError> {
        if !self.capabilities.supports(Operation::Refract) {
            return Err(ProviderError::UnsupportedOperation(Operation::Refract));
        }
        if self.fail_refract {
            return Err(ProviderError::Failed("fixture semantic failure".into()));
        }
        if request
            .sublens
            .is_some_and(|sublens| sublens.lens() != request.lens)
        {
            return Err(ProviderError::InvalidRequest("sublens lens mismatch"));
        }

        let input = request.input;
        let subject = input.target.subject.clone();
        let mut provenance = QlProvenance::new(
            self.capabilities.provider.clone(),
            Operation::Refract.as_str(),
            vec![InputRefRevision::new(subject.clone(), input.revision)],
            ResultClass::SemanticStochastic,
        );
        provenance.model = Some("adapter-fixture-model".into());
        provenance.config_ref = Some(ClientRef::new("fixture:config/q4").expect("config ref"));

        let mut reading = QlReading::new(
            ClientRef::new(format!("fixture:reading/{}", request.lens.lens().code()))
                .expect("reading ref"),
            input.target,
            Some(request.lens),
            SemanticDisclosure {
                text: "adapter fixture disclosure".into(),
                status: SemanticStatus::Partial,
                confidence_per_mille: Some(730),
            },
            provenance,
        );
        reading.ql_form = request.frame;
        reading
            .evidence_refs
            .push(ClientRef::new("fixture:source/q4").expect("source ref"));
        Ok(reading)
    }

    fn relate(&self, _request: RelateRequest) -> Result<SemanticRelationReading, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Relate))
    }

    fn synthesise(&self, _request: SynthesiseRequest) -> Result<SemanticSynthesis, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Synthesise))
    }
}
