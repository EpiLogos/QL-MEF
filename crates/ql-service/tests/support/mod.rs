use ql_core::{QlAddress, QlFace, QlFormRef};
use ql_mef::{
    ClientRef, InputRefRevision, LensId, LensRef, QlProvenance, QlProviderRef, QlReading,
    QlRelationReading, QlSynthesis, QlTarget, ResultClass,
};
use ql_semantic::{
    InputLimits, LocateRequest, LocateResult, LocateStatus, Operation, ProviderCapabilities,
    ProviderClass, ProviderError, ProviderHealth, QlProvider, RefractRequest, RelateRequest,
    SemanticDisclosure, SemanticReading, SemanticRelationReading, SemanticStatus,
    SemanticSynthesis, SynthesisDisclosure, SynthesiseRequest, TargetInput,
};

#[derive(Debug, Clone)]
pub struct FixtureProvider {
    capabilities: ProviderCapabilities,
}

impl FixtureProvider {
    pub fn full(id: &str, health: ProviderHealth) -> Self {
        let provider = QlProviderRef::new(id, "0.1.0").expect("fixture provider ref");
        let supported_lenses = LensId::ALL.into_iter().map(LensRef::canonical).collect();
        Self {
            capabilities: ProviderCapabilities {
                provider,
                health,
                classes: vec![
                    ProviderClass::FormalKernel,
                    ProviderClass::SemanticRefraction,
                ],
                supported_forms: vec![
                    QlFormRef::SIXFOLD_V1,
                    QlFormRef::FOUR_PLUS_TWO_V1,
                    QlFormRef::DIRECT_CONJUGATE_V1,
                ],
                supported_lenses,
                operations: vec![
                    Operation::Capabilities,
                    Operation::Locate,
                    Operation::Refract,
                    Operation::Relate,
                    Operation::Synthesise,
                ],
                extension_namespaces: vec![],
                deterministic_operations: vec![Operation::Capabilities, Operation::Locate],
                input_limits: InputLimits {
                    max_relation_subjects: 4,
                    max_synthesis_readings: 8,
                },
                output_schema_versions: vec!["ql-contract/1.1.0".into()],
            },
        }
    }

    pub fn locate_only(id: &str) -> Self {
        let mut provider = Self::full(id, ProviderHealth::available());
        provider.capabilities.classes = vec![ProviderClass::FormalKernel];
        provider.capabilities.supported_lenses.clear();
        provider.capabilities.operations = vec![Operation::Capabilities, Operation::Locate];
        provider
    }

    fn require(&self, operation: Operation) -> Result<(), ProviderError> {
        if self.capabilities.supports(operation) {
            Ok(())
        } else {
            Err(ProviderError::UnsupportedOperation(operation))
        }
    }

    fn provenance(
        &self,
        operation: Operation,
        refs: Vec<InputRefRevision>,
        class: ResultClass,
    ) -> QlProvenance {
        let mut provenance = QlProvenance::new(
            self.capabilities.provider.clone(),
            operation.as_str(),
            refs,
            class,
        );
        if class == ResultClass::SemanticStochastic {
            provenance.model = Some("fixture-semantic-model".into());
            provenance.config_ref = Some(ClientRef::new("fixture:config/q3").expect("config ref"));
        }
        provenance
    }
}

impl QlProvider for FixtureProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn locate(&self, request: LocateRequest) -> Result<LocateResult, ProviderError> {
        self.require(Operation::Locate)?;
        let input = request.input;
        let provenance = self.provenance(
            Operation::Locate,
            vec![InputRefRevision::new(
                input.target.subject.clone(),
                input.revision,
            )],
            ResultClass::Deterministic,
        );
        Ok(LocateResult {
            target: input.target,
            candidates: vec![QlAddress::sixfold(2, QlFace::Direct, 0).expect("fixture address")],
            status: LocateStatus::Unique,
            provenance,
        })
    }

    fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ProviderError> {
        self.require(Operation::Refract)?;
        if request
            .sublens
            .is_some_and(|sublens| sublens.lens() != request.lens)
        {
            return Err(ProviderError::InvalidRequest("sublens lens mismatch"));
        }
        let input = request.input;
        let subject = input.target.subject.clone();
        let mut reading = QlReading::new(
            ClientRef::new(format!("fixture:reading/{}", request.lens.lens().code()))
                .expect("reading id"),
            input.target,
            Some(request.lens),
            SemanticDisclosure {
                text: "fixture semantic disclosure".into(),
                status: SemanticStatus::Partial,
                confidence_per_mille: Some(750),
            },
            self.provenance(
                Operation::Refract,
                vec![InputRefRevision::new(subject, input.revision)],
                ResultClass::SemanticStochastic,
            ),
        );
        reading.ql_form = request.frame;
        reading
            .evidence_refs
            .push(ClientRef::new("fixture:source/corpus-1").expect("source ref"));
        Ok(reading)
    }

    fn relate(&self, request: RelateRequest) -> Result<SemanticRelationReading, ProviderError> {
        self.require(Operation::Relate)?;
        if request.inputs.len() < 2 {
            return Err(ProviderError::InvalidRequest(
                "relate requires at least two subjects",
            ));
        }
        let pairs = request
            .inputs
            .into_iter()
            .map(|input| (input.target.subject, input.revision))
            .collect::<Vec<_>>();
        let subjects = pairs
            .iter()
            .map(|(reference, _)| reference.clone())
            .collect::<Vec<_>>();
        let refs = pairs
            .into_iter()
            .map(|(reference, revision)| InputRefRevision::new(reference, revision))
            .collect();
        Ok(QlRelationReading {
            id: ClientRef::new("fixture:relation/1").expect("relation id"),
            subjects,
            frame: request.frame,
            relation: SemanticDisclosure {
                text: "fixture relation".into(),
                status: SemanticStatus::Partial,
                confidence_per_mille: Some(700),
            },
            addresses: vec![],
            lenses: request.lenses,
            evidence_refs: vec![ClientRef::new("fixture:source/corpus-1").expect("source ref")],
            provenance: self.provenance(Operation::Relate, refs, ResultClass::SemanticStochastic),
        })
    }

    fn synthesise(&self, request: SynthesiseRequest) -> Result<SemanticSynthesis, ProviderError> {
        self.require(Operation::Synthesise)?;
        if request.readings.is_empty() {
            return Err(ProviderError::InvalidRequest(
                "synthesise requires readings",
            ));
        }
        let input_readings = request
            .readings
            .iter()
            .map(|reading| reading.id.clone())
            .collect::<Vec<_>>();
        let refs = input_readings
            .iter()
            .cloned()
            .map(|reference| InputRefRevision::new(reference, None))
            .collect();
        Ok(QlSynthesis {
            id: ClientRef::new("fixture:synthesis/1").expect("synthesis id"),
            input_readings,
            synthesis: SynthesisDisclosure {
                common_structure: vec!["shared structure".into()],
                complementary_disclosures: vec!["complement retained".into()],
                tensions: vec!["tension retained".into()],
                possible_next_inquiry: Some("inspect unresolved evidence".into()),
            },
            retained_differences: vec!["difference retained".into()],
            unresolved: vec!["unresolved question".into()],
            provenance: self.provenance(
                Operation::Synthesise,
                refs,
                ResultClass::SemanticStochastic,
            ),
        })
    }
}

pub fn target(value: &str) -> QlTarget {
    QlTarget::new(ClientRef::new(value).expect("target ref"))
}

pub fn input(value: &str, revision: Option<&str>) -> TargetInput {
    TargetInput::new(target(value), revision.map(str::to_owned))
}
