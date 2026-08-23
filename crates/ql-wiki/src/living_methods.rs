//! Living-Wiki contemplation methods composed over the accepted Wiki refraction engine.
//!
//! A method pass never owns the target Wiki or redefines a QL operator. It names the
//! investigative relation requested by the caller, validates that the existing
//! `WikiRefractionRequest` actually carries the corresponding target form, delegates execution to
//! `WikiRefractionEngine`, and retains the resulting provider/operator/lens provenance.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    ContextFrameDepth, LivingWikiMode, LivingWikiRefractionPlan, ProviderMode, RefractionStatus,
    WikiRefractionEngine, WikiRefractionError, WikiRefractionRequest, WikiRefractionResponse,
    WikiTargetKind,
};

pub const LIVING_WIKI_METHOD_PROFILE: &str = "ql-mef/living-wiki-methods/v1";
pub const LIVING_WIKI_COMPARISON_RUBRIC: &str = "ql-mef/living-wiki-comparison-rubric/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LivingWikiPresentationDepth {
    Ordinary,
    Explain,
    Formal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LivingWikiMethodFamily {
    RelationFamilyExpansion,
    ConjugacyInvestigation,
    SquareFieldExpansion,
    MefLensRefraction,
    ContextFrameReading,
    HarmonicTraversal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingWikiMethodPass {
    pub method_ref: String,
    pub family: LivingWikiMethodFamily,
    pub request: WikiRefractionRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingWikiMethodProfile {
    #[serde(default = "method_profile")]
    pub profile: String,
    pub profile_ref: String,
    pub presentation: LivingWikiPresentationDepth,
    pub max_passes: usize,
    #[serde(default)]
    pub passes: Vec<LivingWikiMethodPass>,
}

fn method_profile() -> String {
    LIVING_WIKI_METHOD_PROFILE.into()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingWikiMethodPassResult {
    pub method_ref: String,
    pub family: LivingWikiMethodFamily,
    pub response: WikiRefractionResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingWikiMethodResult {
    pub profile: String,
    pub profile_ref: String,
    pub presentation: LivingWikiPresentationDepth,
    pub entry_positions: [u8; 3],
    pub recovered_positions: Vec<u8>,
    pub context_depth: ContextFrameDepth,
    pub canonical_return_through_anchor: bool,
    #[serde(default)]
    pub passes: Vec<LivingWikiMethodPassResult>,
    #[serde(default)]
    pub notices: Vec<String>,
    pub target_wiki_mutated: bool,
    pub owns_source_truth: bool,
    pub owns_agent_orchestration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivingWikiMethodError {
    WrongProfile(String),
    EmptyProfileRef,
    InvalidPassBudget(usize),
    TooManyPasses { requested: usize, budget: usize },
    EmptyMethodRef,
    MethodTargetMismatch(String),
    Refraction(String),
}

impl core::fmt::Display for LivingWikiMethodError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongProfile(profile) => write!(
                f,
                "Living Wiki method profile must be {LIVING_WIKI_METHOD_PROFILE}, got {profile}"
            ),
            Self::EmptyProfileRef => f.write_str("Living Wiki method profile_ref must be non-empty"),
            Self::InvalidPassBudget(value) => {
                write!(f, "Living Wiki method max_passes must be positive, got {value}")
            }
            Self::TooManyPasses { requested, budget } => write!(
                f,
                "Living Wiki method profile requested {requested} passes with budget {budget}"
            ),
            Self::EmptyMethodRef => f.write_str("Living Wiki method_ref must be non-empty"),
            Self::MethodTargetMismatch(detail) => f.write_str(detail),
            Self::Refraction(detail) => write!(f, "Living Wiki refraction failed: {detail}"),
        }
    }
}

impl std::error::Error for LivingWikiMethodError {}

impl From<WikiRefractionError> for LivingWikiMethodError {
    fn from(value: WikiRefractionError) -> Self {
        Self::Refraction(value.to_string())
    }
}

fn non_empty_context_ref(request: &WikiRefractionRequest, key: &str) -> bool {
    request
        .context
        .get(key)
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

impl LivingWikiMethodPass {
    pub fn validate(&self) -> Result<(), LivingWikiMethodError> {
        if self.method_ref.trim().is_empty() {
            return Err(LivingWikiMethodError::EmptyMethodRef);
        }
        self.request.validate()?;
        match self.family {
            LivingWikiMethodFamily::RelationFamilyExpansion => {
                if self.request.target.kind != WikiTargetKind::Pair {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "relation-family expansion requires the accepted Pair target form".into(),
                    ));
                }
            }
            LivingWikiMethodFamily::ConjugacyInvestigation => {
                if !matches!(self.request.target.kind, WikiTargetKind::D1 | WikiTargetKind::D2) {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "conjugacy investigation requires the accepted D1 or D2 target form".into(),
                    ));
                }
            }
            LivingWikiMethodFamily::SquareFieldExpansion => {
                if self.request.target.kind != WikiTargetKind::D3 {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "square-field expansion requires the accepted D3 target form".into(),
                    ));
                }
            }
            LivingWikiMethodFamily::MefLensRefraction => {
                if self.request.lenses.is_empty() || self.request.mode == ProviderMode::Disabled {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "MEF lens refraction requires an enabled request with a selected lens".into(),
                    ));
                }
            }
            LivingWikiMethodFamily::ContextFrameReading => {
                if !matches!(self.request.target.kind, WikiTargetKind::Frame | WikiTargetKind::Space)
                    || !non_empty_context_ref(&self.request, "context_frame_ref")
                {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "Context-Frame reading requires a bounded Frame/Space plus explicit current context_frame_ref supplied by the caller"
                            .into(),
                    ));
                }
            }
            LivingWikiMethodFamily::HarmonicTraversal => {
                if self.request.target.structural_field.is_none()
                    && !non_empty_context_ref(&self.request, "harmonic_field_ref")
                {
                    return Err(LivingWikiMethodError::MethodTargetMismatch(
                        "harmonic traversal requires an accepted structural operator field or explicit harmonic_field_ref"
                            .into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl LivingWikiMethodProfile {
    pub fn validate(&self) -> Result<(), LivingWikiMethodError> {
        if self.profile != LIVING_WIKI_METHOD_PROFILE {
            return Err(LivingWikiMethodError::WrongProfile(self.profile.clone()));
        }
        if self.profile_ref.trim().is_empty() {
            return Err(LivingWikiMethodError::EmptyProfileRef);
        }
        if self.max_passes == 0 {
            return Err(LivingWikiMethodError::InvalidPassBudget(self.max_passes));
        }
        if self.passes.len() > self.max_passes {
            return Err(LivingWikiMethodError::TooManyPasses {
                requested: self.passes.len(),
                budget: self.max_passes,
            });
        }
        for pass in &self.passes {
            pass.validate()?;
        }
        Ok(())
    }
}

fn annotate_response(
    response: &mut WikiRefractionResponse,
    plan: &LivingWikiRefractionPlan,
    profile: &LivingWikiMethodProfile,
    pass: &LivingWikiMethodPass,
) {
    for reading in &mut response.readings {
        reading.extensions.insert(
            "living_wiki_method".into(),
            json!({
                "profile": profile.profile,
                "profile_ref": profile.profile_ref,
                "method_ref": pass.method_ref,
                "family": pass.family,
                "presentation": profile.presentation,
                "entry_positions": plan.entry.positions,
                "entry_is_return": plan.entry.is_return,
                "recovered_positions": plan.positions,
                "context_depth": plan.depth,
                "canonical_return": plan.canonical_return,
            }),
        );
    }
}

pub fn execute_living_wiki_methods(
    engine: &WikiRefractionEngine<'_>,
    plan: &LivingWikiRefractionPlan,
    profile: &LivingWikiMethodProfile,
) -> Result<LivingWikiMethodResult, LivingWikiMethodError> {
    profile.validate()?;
    if plan.mode == LivingWikiMode::Ordinary {
        return Ok(LivingWikiMethodResult {
            profile: profile.profile.clone(),
            profile_ref: profile.profile_ref.clone(),
            presentation: profile.presentation,
            entry_positions: plan.entry.positions,
            recovered_positions: vec![],
            context_depth: ContextFrameDepth::None,
            canonical_return_through_anchor: plan.canonical_return.through_anchor,
            passes: vec![],
            notices: vec![
                "ordinary Wiki operation remains valid without executing a QL method".into(),
            ],
            target_wiki_mutated: false,
            owns_source_truth: false,
            owns_agent_orchestration: false,
        });
    }

    let mut passes = Vec::new();
    let mut notices = Vec::new();
    for pass in &profile.passes {
        let mut response = engine.refract(&pass.request)?;
        annotate_response(&mut response, plan, profile, pass);
        notices.extend(response.notices.clone());
        passes.push(LivingWikiMethodPassResult {
            method_ref: pass.method_ref.clone(),
            family: pass.family,
            response,
        });
    }
    Ok(LivingWikiMethodResult {
        profile: profile.profile.clone(),
        profile_ref: profile.profile_ref.clone(),
        presentation: profile.presentation,
        entry_positions: plan.entry.positions,
        recovered_positions: plan.positions.clone(),
        context_depth: plan.depth,
        canonical_return_through_anchor: plan.canonical_return.through_anchor,
        passes,
        notices,
        target_wiki_mutated: false,
        owns_source_truth: false,
        owns_agent_orchestration: false,
    })
}

/// Preserve the provider's truthfulness categories exactly; the Living Wiki layer does not
/// coerce ambiguous/partial/unavailable outcomes into a complete method result.
pub fn response_truth_state(response: &WikiRefractionResponse) -> &'static str {
    match response.status {
        RefractionStatus::Disabled => "disabled",
        RefractionStatus::Unavailable => "unavailable",
        RefractionStatus::Degraded => "degraded",
        RefractionStatus::Complete => {
            if response
                .readings
                .iter()
                .any(|reading| reading.disclosure_status == "ambiguous")
            {
                "ambiguous"
            } else if response
                .readings
                .iter()
                .any(|reading| reading.disclosure_status == "insufficient-evidence")
            {
                "insufficient-evidence"
            } else if response
                .readings
                .iter()
                .any(|reading| reading.disclosure_status == "partial")
            {
                "partial"
            } else {
                "complete"
            }
        }
        RefractionStatus::Partial => "partial",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivingWikiComparisonCase {
    pub case_ref: String,
    pub context_budget: usize,
    pub dependency_paths_recovered: usize,
    pub whole_part_links_recovered: usize,
    pub source_ground_refs_recovered: usize,
    pub context_position_four_recovered: bool,
    pub tensions_surfaced: usize,
    pub unsupported_relations: usize,
    pub context_units_used: usize,
    pub duplicated_context_units: usize,
    pub provenance_refs_retained: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LivingWikiComparisonFinding {
    SupportsAperture,
    SupportsOrdinary,
    Mixed,
    Equivalent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivingWikiComparisonEvidence {
    pub rubric: String,
    pub field_ref: String,
    pub ordinary: LivingWikiComparisonCase,
    pub aperture: LivingWikiComparisonCase,
    pub finding: LivingWikiComparisonFinding,
    #[serde(default)]
    pub improvements: Vec<String>,
    #[serde(default)]
    pub regressions: Vec<String>,
    #[serde(default)]
    pub observations: Vec<String>,
    /// Comparative evidence is research evidence only; it never promotes a preferred profile.
    pub automatic_profile_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivingWikiComparisonError {
    BudgetMismatch { ordinary: usize, aperture: usize },
    BudgetExceeded { case_ref: String, used: usize, budget: usize },
    EmptyFieldRef,
}

impl core::fmt::Display for LivingWikiComparisonError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BudgetMismatch { ordinary, aperture } => write!(
                f,
                "comparison requires equal context budgets; ordinary={ordinary}, aperture={aperture}"
            ),
            Self::BudgetExceeded {
                case_ref,
                used,
                budget,
            } => write!(
                f,
                "comparison case {case_ref} used {used} context units with budget {budget}"
            ),
            Self::EmptyFieldRef => f.write_str("comparison field_ref must be non-empty"),
        }
    }
}

impl std::error::Error for LivingWikiComparisonError {}

pub fn compare_living_wiki_entry(
    field_ref: impl Into<String>,
    ordinary: LivingWikiComparisonCase,
    aperture: LivingWikiComparisonCase,
) -> Result<LivingWikiComparisonEvidence, LivingWikiComparisonError> {
    let field_ref = field_ref.into();
    if field_ref.trim().is_empty() {
        return Err(LivingWikiComparisonError::EmptyFieldRef);
    }
    if ordinary.context_budget != aperture.context_budget {
        return Err(LivingWikiComparisonError::BudgetMismatch {
            ordinary: ordinary.context_budget,
            aperture: aperture.context_budget,
        });
    }
    for case in [&ordinary, &aperture] {
        if case.context_units_used > case.context_budget {
            return Err(LivingWikiComparisonError::BudgetExceeded {
                case_ref: case.case_ref.clone(),
                used: case.context_units_used,
                budget: case.context_budget,
            });
        }
    }

    let mut improvements = Vec::new();
    let mut regressions = Vec::new();
    let mut observations = Vec::new();
    macro_rules! compare_more {
        ($field:ident, $label:literal) => {
            if aperture.$field > ordinary.$field {
                improvements.push($label.into());
            } else if aperture.$field < ordinary.$field {
                regressions.push($label.into());
            }
        };
    }
    compare_more!(dependency_paths_recovered, "dependency-path recovery");
    compare_more!(whole_part_links_recovered, "whole-part continuity");
    compare_more!(source_ground_refs_recovered, "source/ground traceability");
    compare_more!(tensions_surfaced, "tensions surfaced");
    compare_more!(provenance_refs_retained, "reading-basis provenance");
    if aperture.context_position_four_recovered && !ordinary.context_position_four_recovered {
        improvements.push("context-position-four recovery".into());
    } else if ordinary.context_position_four_recovered && !aperture.context_position_four_recovered {
        regressions.push("context-position-four recovery".into());
    }
    if aperture.unsupported_relations < ordinary.unsupported_relations {
        improvements.push("unsupported relation rate".into());
    } else if aperture.unsupported_relations > ordinary.unsupported_relations {
        regressions.push("unsupported relation rate".into());
    }
    if aperture.context_units_used < ordinary.context_units_used {
        improvements.push("context volume".into());
    } else if aperture.context_units_used > ordinary.context_units_used {
        regressions.push("context volume".into());
    }
    if aperture.duplicated_context_units < ordinary.duplicated_context_units {
        improvements.push("duplicated material".into());
    } else if aperture.duplicated_context_units > ordinary.duplicated_context_units {
        regressions.push("duplicated material".into());
    }
    observations.push(format!(
        "controlled budget: {} context units",
        ordinary.context_budget
    ));

    let finding = match (improvements.is_empty(), regressions.is_empty()) {
        (false, true) => LivingWikiComparisonFinding::SupportsAperture,
        (true, false) => LivingWikiComparisonFinding::SupportsOrdinary,
        (false, false) => LivingWikiComparisonFinding::Mixed,
        (true, true) => LivingWikiComparisonFinding::Equivalent,
    };
    Ok(LivingWikiComparisonEvidence {
        rubric: LIVING_WIKI_COMPARISON_RUBRIC.into(),
        field_ref,
        ordinary,
        aperture,
        finding,
        improvements,
        regressions,
        observations,
        automatic_profile_promotion: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use ql_core::{QlFace, RelationFamily};

    use crate::{
        plan_living_wiki_refraction, FieldCoordinate, LensSelection, LivingWikiRelevance,
        RegistryDisclosureProvider, RevisionValue, WikiProvenanceRef, WikiRefractionTarget,
        WikiStructuralField, WikiSubjectSnapshot,
    };

    fn pair_request() -> WikiRefractionRequest {
        let pair = RelationFamily::A.pair(0).unwrap();
        WikiRefractionRequest {
            contract: crate::WIKI_REFRACTION_CONTRACT.into(),
            mode: ProviderMode::Optional,
            target: WikiRefractionTarget {
                kind: WikiTargetKind::Pair,
                target_ref: "wiki:frame:pair-a0".into(),
                target_frame_ref: Some("wiki:frame:pair-a0".into()),
                target_revision: Some(RevisionValue::Integer(3)),
                target_snapshot_hash: "sha256:pair-a0".into(),
                provenance: vec![WikiProvenanceRef {
                    source_ref: "source:wiki:pair-a0".into(),
                    source_revision: Some(RevisionValue::String("r3".into())),
                    extensions: BTreeMap::new(),
                }],
                subjects: vec![
                    WikiSubjectSnapshot {
                        subject_ref: "wiki:node:left".into(),
                        revision: Some(RevisionValue::Integer(1)),
                        position: Some(pair.left.value()),
                        face: Some(QlFace::Direct.as_str().into()),
                        extensions: BTreeMap::new(),
                    },
                    WikiSubjectSnapshot {
                        subject_ref: "wiki:node:right".into(),
                        revision: Some(RevisionValue::Integer(1)),
                        position: Some(pair.right.value()),
                        face: Some(QlFace::Direct.as_str().into()),
                        extensions: BTreeMap::new(),
                    },
                ],
                relations: vec![],
                structural_field: Some(WikiStructuralField {
                    operator_ref: pair.operator_ref(),
                    family: Some("A".into()),
                    pair_index: Some(0),
                    degree: "pair".into(),
                    expansion_side: None,
                    coordinates: vec![
                        FieldCoordinate {
                            position: pair.left.value(),
                            face: "direct".into(),
                        },
                        FieldCoordinate {
                            position: pair.right.value(),
                            face: "direct".into(),
                        },
                    ],
                    provenance: vec![],
                }),
                material: BTreeMap::new(),
                extensions: BTreeMap::new(),
            },
            lenses: vec![LensSelection {
                lens_ref: "L0".into(),
                sublens_ref: None,
            }],
            context: BTreeMap::new(),
        }
    }

    #[test]
    fn method_pass_delegates_to_existing_engine_and_keeps_formal_provenance() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                relevant_positions: vec![2, 3],
                requires_context: true,
                full_context_frame: false,
                position_budget: 6,
            },
            None,
        )
        .unwrap();
        let profile = LivingWikiMethodProfile {
            profile: LIVING_WIKI_METHOD_PROFILE.into(),
            profile_ref: "ql-mef:living-profile:test".into(),
            presentation: LivingWikiPresentationDepth::Formal,
            max_passes: 2,
            passes: vec![LivingWikiMethodPass {
                method_ref: "ql-mef:method:relation-family".into(),
                family: LivingWikiMethodFamily::RelationFamilyExpansion,
                request: pair_request(),
            }],
        };
        let provider = RegistryDisclosureProvider::new();
        let engine = WikiRefractionEngine::new(Some(&provider));
        let result = execute_living_wiki_methods(&engine, &plan, &profile).unwrap();
        assert_eq!(result.passes.len(), 1);
        let reading = &result.passes[0].response.readings[0];
        assert_eq!(reading.operator_refs.len(), 1);
        assert_eq!(reading.lens_ref, "L0");
        assert_eq!(reading.target_ref, "wiki:frame:pair-a0");
        assert!(!reading.provenance.is_empty());
        assert!(reading.extensions.contains_key("living_wiki_method"));
        assert!(!result.target_wiki_mutated);
        assert!(result.recovered_positions.contains(&4));
    }

    #[test]
    fn ordinary_mode_executes_no_ql_pass_even_when_profile_supplies_one() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Ordinary,
            LivingWikiRelevance::default(),
            None,
        )
        .unwrap();
        let profile = LivingWikiMethodProfile {
            profile: LIVING_WIKI_METHOD_PROFILE.into(),
            profile_ref: "ql-mef:living-profile:ordinary".into(),
            presentation: LivingWikiPresentationDepth::Ordinary,
            max_passes: 1,
            passes: vec![LivingWikiMethodPass {
                method_ref: "ql-mef:method:relation-family".into(),
                family: LivingWikiMethodFamily::RelationFamilyExpansion,
                request: pair_request(),
            }],
        };
        let provider = RegistryDisclosureProvider::new();
        let engine = WikiRefractionEngine::new(Some(&provider));
        let result = execute_living_wiki_methods(&engine, &plan, &profile).unwrap();
        assert!(result.passes.is_empty());
        assert!(!result.owns_source_truth);
        assert!(!result.owns_agent_orchestration);
    }

    #[test]
    fn method_family_cannot_claim_a_square_over_a_pair_target() {
        let pass = LivingWikiMethodPass {
            method_ref: "ql-mef:method:square".into(),
            family: LivingWikiMethodFamily::SquareFieldExpansion,
            request: pair_request(),
        };
        assert!(matches!(
            pass.validate().unwrap_err(),
            LivingWikiMethodError::MethodTargetMismatch(_)
        ));
    }

    #[test]
    fn optional_absent_provider_remains_truthfully_unavailable() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance::default(),
            None,
        )
        .unwrap();
        let profile = LivingWikiMethodProfile {
            profile: LIVING_WIKI_METHOD_PROFILE.into(),
            profile_ref: "ql-mef:living-profile:absent".into(),
            presentation: LivingWikiPresentationDepth::Explain,
            max_passes: 1,
            passes: vec![LivingWikiMethodPass {
                method_ref: "ql-mef:method:mef-lens".into(),
                family: LivingWikiMethodFamily::MefLensRefraction,
                request: pair_request(),
            }],
        };
        let engine = WikiRefractionEngine::new(None);
        let result = execute_living_wiki_methods(&engine, &plan, &profile).unwrap();
        assert_eq!(response_truth_state(&result.passes[0].response), "unavailable");
        assert!(result.passes[0].response.readings.is_empty());
    }

    #[test]
    fn comparison_fixture_can_return_mixed_evidence_without_profile_promotion() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../fixtures/living-wiki-comparison-v1.json"
        ))
        .unwrap();
        let ordinary: LivingWikiComparisonCase =
            serde_json::from_value(fixture["ordinary"].clone()).unwrap();
        let aperture: LivingWikiComparisonCase =
            serde_json::from_value(fixture["aperture"].clone()).unwrap();
        let evidence = compare_living_wiki_entry(
            fixture["field_ref"].as_str().unwrap(),
            ordinary,
            aperture,
        )
        .unwrap();
        assert_eq!(evidence.finding, LivingWikiComparisonFinding::Mixed);
        assert!(!evidence.improvements.is_empty());
        assert!(!evidence.regressions.is_empty());
        assert!(!evidence.automatic_profile_promotion);
    }
}
