use ql_core::QlFormRef;
use ql_mef::{
    ClientRef, InputRefRevision, LensId, LensRef, QlProvenance, QlProviderRef, QlReading,
    QlRelationReading, QlSynthesis, ResultClass, lens_definition,
};
use ql_semantic::{
    InputLimits, LocateRequest, LocateResult, Operation, ProviderCapabilities, ProviderClass,
    ProviderError, ProviderHealth, QlProvider, RefractRequest, RelateRequest, SemanticDisclosure,
    SemanticReading, SemanticRelationReading, SemanticStatus, SemanticSynthesis, SynthesiseRequest,
};

use crate::WIKI_REFRACTION_CONTRACT;

/// Minimal production reference provider for the Wiki refraction wire contract.
///
/// It is intentionally deterministic and registry-backed: it proves the external
/// provider path and exposes canonical lens/sublens semantics already owned by
/// QL-MEF, but does not pretend to be a model-backed relational discovery engine.
/// More capable semantic providers can replace it through the existing
/// [`QlProvider`] boundary without changing the Wiki wire contract.
#[derive(Debug, Clone)]
pub struct RegistryDisclosureProvider {
    capabilities: ProviderCapabilities,
}

impl RegistryDisclosureProvider {
    pub fn new() -> Self {
        Self {
            capabilities: ProviderCapabilities {
                provider: QlProviderRef::new("ql-mef:provider:registry-disclosure", "1.0.0")
                    .expect("static provider identity is valid"),
                health: ProviderHealth::available(),
                classes: vec![ProviderClass::SemanticRefraction],
                supported_forms: vec![
                    QlFormRef::SIXFOLD_V1,
                    QlFormRef::FOUR_PLUS_TWO_V1,
                    QlFormRef::DIRECT_CONJUGATE_V1,
                ],
                supported_lenses: LensId::ALL.into_iter().map(LensRef::canonical).collect(),
                operations: vec![Operation::Capabilities, Operation::Refract],
                extension_namespaces: vec![WIKI_REFRACTION_CONTRACT.into()],
                deterministic_operations: vec![Operation::Capabilities, Operation::Refract],
                input_limits: InputLimits {
                    max_relation_subjects: 16,
                    max_synthesis_readings: 12,
                },
                output_schema_versions: vec![WIKI_REFRACTION_CONTRACT.into()],
            },
        }
    }
}

impl Default for RegistryDisclosureProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl QlProvider for RegistryDisclosureProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }

    fn locate(&self, _request: LocateRequest) -> Result<LocateResult, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Locate))
    }

    fn refract(&self, request: RefractRequest) -> Result<SemanticReading, ProviderError> {
        let lens = lens_definition(request.lens.lens());
        let subject = request.input.target.subject.clone();
        let input_revision = request.input.revision.clone();
        let sublens_text = request.sublens.map(|sublens| {
            let definition = lens
                .sublens(sublens.position().value())
                .expect("validated sublens coordinate belongs to canonical lens");
            format!(" / {}", definition.label())
        });
        let disclosure = format!(
            "{}{} refractive reading of {}",
            lens.name(),
            sublens_text.unwrap_or_default(),
            subject
        );
        let provenance = QlProvenance::new(
            self.capabilities.provider.clone(),
            Operation::Refract.as_str(),
            vec![InputRefRevision::new(subject.clone(), input_revision)],
            ResultClass::Deterministic,
        );
        let mut reading = QlReading::new(
            ClientRef::new(format!(
                "ql-mef:reading:registry:{}:{}",
                sanitise_ref(subject.as_str()),
                request.lens.lens().code()
            ))
            .expect("derived reading ref is non-empty"),
            request.input.target,
            Some(request.lens),
            SemanticDisclosure {
                text: disclosure,
                status: SemanticStatus::Complete,
                confidence_per_mille: Some(1000),
            },
            provenance,
        );
        reading
            .evidence_refs
            .push(ClientRef::new(format!("ql-mef:wiki:node:mef-{}", request.lens.lens().code()))
                .expect("canonical lens evidence ref is valid"));
        Ok(reading)
    }

    fn relate(&self, _request: RelateRequest) -> Result<SemanticRelationReading, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Relate))
    }

    fn synthesise(&self, _request: SynthesiseRequest) -> Result<SemanticSynthesis, ProviderError> {
        Err(ProviderError::UnsupportedOperation(Operation::Synthesise))
    }
}

fn sanitise_ref(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '-'
            }
        })
        .collect()
}
