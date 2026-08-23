//! Optional Living Wiki refraction depth over the existing WikiFrame refraction contract.
//!
//! The entry aperture `5 -> 0 -> 1` is a bounded orientation into a living knowledge change.
//! It is deliberately distinct from the canonical Return relation, which remains
//! `#5 -> whole-anchor -> #0` together with `#0 <-> anchor` and `#5 <-> anchor`.
//! This module owns no source watcher, freshness engine, Agent orchestration or source authority.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{ProviderMode, WIKI_REFRACTION_CONTRACT, WikiRefractionRequest};

pub const LIVING_WIKI_REFRACTION_PROFILE: &str = "ql-mef/living-wiki-refraction/v1";
pub const LIVING_WIKI_ENTRY_APERTURE: [u8; 3] = [5, 0, 1];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LivingWikiMode {
    /// Ordinary Wiki correctness remains independent of QL-MEF.
    Ordinary,
    /// Explain the bounded structural aperture without requiring deeper formal execution.
    Explain,
    /// Carry the existing Wiki refraction request as optional/required formal depth.
    Formal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContextFrameDepth {
    None,
    EntryAperture,
    Partial,
    FullSixfold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivingWikiRelevance {
    /// Explicit relevance supplied by the caller. QL-MEF does not infer this from source text.
    #[serde(default)]
    pub relevant_positions: Vec<u8>,
    /// A contextual dependence explicitly requires recovery of position #4.
    #[serde(default)]
    pub requires_context: bool,
    /// Caller explicitly requests the complete 0..5 Context Frame.
    #[serde(default)]
    pub full_context_frame: bool,
    /// Hard bound on distinct positions admitted to this pass.
    pub position_budget: usize,
}

impl Default for LivingWikiRelevance {
    fn default() -> Self {
        Self {
            relevant_positions: Vec::new(),
            requires_context: false,
            full_context_frame: false,
            position_budget: 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryAperture {
    pub positions: [u8; 3],
    pub meaning: String,
    /// Always false: the aperture is orientation, not the Return operator.
    pub is_return: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalReturnTransit {
    pub from_position: u8,
    pub through_anchor: bool,
    pub to_position: u8,
    pub anchor_relations: Vec<String>,
}

impl CanonicalReturnTransit {
    pub fn established() -> Self {
        Self {
            from_position: 5,
            through_anchor: true,
            to_position: 0,
            anchor_relations: vec!["#0 <-> anchor".into(), "#5 <-> anchor".into()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivingWikiRefractionPlan {
    pub profile: String,
    pub mode: LivingWikiMode,
    pub entry: EntryAperture,
    pub canonical_return: CanonicalReturnTransit,
    /// Ordered set of positions actually admitted after relevance + budget resolution.
    pub positions: Vec<u8>,
    pub depth: ContextFrameDepth,
    #[serde(default)]
    pub omitted_relevant_positions: Vec<u8>,
    /// Existing refraction contract, preserved rather than replaced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refraction: Option<WikiRefractionRequest>,
    pub ordinary_wiki_correctness_requires_ql: bool,
    pub owns_source_freshness: bool,
    pub owns_agent_orchestration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivingWikiRefractionError {
    InvalidPosition(u8),
    PositionBudgetTooSmall(usize),
    FormalRequestMissing,
    FormalRequestDisabled,
    InvalidRefraction(String),
}

impl core::fmt::Display for LivingWikiRefractionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPosition(value) => {
                write!(f, "Living Wiki relevance position {value} is outside 0..5")
            }
            Self::PositionBudgetTooSmall(value) => write!(
                f,
                "Living Wiki position budget {value} cannot preserve the 5->0->1 entry aperture"
            ),
            Self::FormalRequestMissing => {
                f.write_str("formal Living Wiki depth requires an existing WikiRefractionRequest")
            }
            Self::FormalRequestDisabled => f.write_str(
                "formal Living Wiki depth cannot carry a disabled WikiRefractionRequest",
            ),
            Self::InvalidRefraction(value) => {
                write!(f, "invalid existing Wiki refraction request: {value}")
            }
        }
    }
}

impl std::error::Error for LivingWikiRefractionError {}

fn bounded_positions(
    relevance: &LivingWikiRelevance,
) -> Result<(Vec<u8>, Vec<u8>), LivingWikiRefractionError> {
    if relevance.position_budget < LIVING_WIKI_ENTRY_APERTURE.len() {
        return Err(LivingWikiRefractionError::PositionBudgetTooSmall(
            relevance.position_budget,
        ));
    }
    for position in &relevance.relevant_positions {
        if *position > 5 {
            return Err(LivingWikiRefractionError::InvalidPosition(*position));
        }
    }

    let mut requested = BTreeSet::new();
    for position in LIVING_WIKI_ENTRY_APERTURE {
        requested.insert(position);
    }
    if relevance.full_context_frame {
        requested.extend(0..=5);
    } else {
        requested.extend(relevance.relevant_positions.iter().copied());
        if relevance.requires_context {
            requested.insert(4);
        }
    }

    // The aperture is preserved first; additional positions follow natural QL order.
    let mut positions = LIVING_WIKI_ENTRY_APERTURE.to_vec();
    for position in 0..=5 {
        if requested.contains(&position) && !positions.contains(&position) {
            positions.push(position);
        }
    }
    let omitted = if positions.len() > relevance.position_budget {
        positions.split_off(relevance.position_budget)
    } else {
        Vec::new()
    };
    Ok((positions, omitted))
}

pub fn plan_living_wiki_refraction(
    mode: LivingWikiMode,
    relevance: LivingWikiRelevance,
    refraction: Option<WikiRefractionRequest>,
) -> Result<LivingWikiRefractionPlan, LivingWikiRefractionError> {
    if mode == LivingWikiMode::Ordinary {
        return Ok(LivingWikiRefractionPlan {
            profile: LIVING_WIKI_REFRACTION_PROFILE.into(),
            mode,
            entry: EntryAperture {
                positions: LIVING_WIKI_ENTRY_APERTURE,
                meaning: "available orientation only; ordinary Wiki correctness does not enter QL"
                    .into(),
                is_return: false,
            },
            canonical_return: CanonicalReturnTransit::established(),
            positions: Vec::new(),
            depth: ContextFrameDepth::None,
            omitted_relevant_positions: Vec::new(),
            refraction: None,
            ordinary_wiki_correctness_requires_ql: false,
            owns_source_freshness: false,
            owns_agent_orchestration: false,
        });
    }

    let (positions, omitted_relevant_positions) = bounded_positions(&relevance)?;
    let depth = if positions.len() == 6 {
        ContextFrameDepth::FullSixfold
    } else if positions == LIVING_WIKI_ENTRY_APERTURE {
        ContextFrameDepth::EntryAperture
    } else {
        ContextFrameDepth::Partial
    };

    let refraction = match mode {
        LivingWikiMode::Ordinary => None,
        LivingWikiMode::Explain => {
            refraction.filter(|request| request.mode != ProviderMode::Disabled)
        }
        LivingWikiMode::Formal => {
            let request = refraction.ok_or(LivingWikiRefractionError::FormalRequestMissing)?;
            if request.mode == ProviderMode::Disabled {
                return Err(LivingWikiRefractionError::FormalRequestDisabled);
            }
            request
                .validate()
                .map_err(|error| LivingWikiRefractionError::InvalidRefraction(error.to_string()))?;
            if request.contract != WIKI_REFRACTION_CONTRACT {
                return Err(LivingWikiRefractionError::InvalidRefraction(
                    "formal request does not use the accepted Wiki refraction contract".into(),
                ));
            }
            Some(request)
        }
    };

    Ok(LivingWikiRefractionPlan {
        profile: LIVING_WIKI_REFRACTION_PROFILE.into(),
        mode,
        entry: EntryAperture {
            positions: LIVING_WIKI_ENTRY_APERTURE,
            meaning: "5->0->1 entry aperture into the changed knowledge field".into(),
            is_return: false,
        },
        canonical_return: CanonicalReturnTransit::established(),
        positions,
        depth,
        omitted_relevant_positions,
        refraction,
        ordinary_wiki_correctness_requires_ql: false,
        owns_source_freshness: false,
        owns_agent_orchestration: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        FieldCoordinate, LensSelection, WikiRefractionTarget, WikiStructuralField, WikiTargetKind,
    };
    use serde_json::Map;

    fn valid_formal_request() -> WikiRefractionRequest {
        WikiRefractionRequest {
            contract: WIKI_REFRACTION_CONTRACT.into(),
            mode: ProviderMode::Optional,
            target: WikiRefractionTarget {
                kind: WikiTargetKind::Frame,
                target_ref: "wiki:frame:living".into(),
                target_frame_ref: Some("wiki:frame:living".into()),
                target_revision: None,
                target_snapshot_hash: "sha256:test".into(),
                provenance: vec![],
                subjects: vec![],
                relations: vec![],
                structural_field: None::<WikiStructuralField>,
                material: Map::new(),
                extensions: Map::new(),
            },
            lenses: vec![LensSelection {
                lens_ref: "L0".into(),
                sublens_ref: None,
            }],
            context: Map::new(),
        }
    }

    #[test]
    fn entry_aperture_is_explicitly_not_return() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance::default(),
            None,
        )
        .unwrap();
        assert_eq!(plan.entry.positions, [5, 0, 1]);
        assert!(!plan.entry.is_return);
        assert_eq!(plan.canonical_return.from_position, 5);
        assert!(plan.canonical_return.through_anchor);
        assert_eq!(plan.canonical_return.to_position, 0);
        assert_eq!(
            plan.canonical_return.anchor_relations,
            vec!["#0 <-> anchor", "#5 <-> anchor"]
        );
    }

    #[test]
    fn relevant_two_three_and_context_four_are_recovered_after_entry() {
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
        assert_eq!(plan.positions, vec![5, 0, 1, 2, 3, 4]);
        assert_eq!(plan.depth, ContextFrameDepth::FullSixfold);
    }

    #[test]
    fn full_context_frame_recovers_all_six_positions() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                full_context_frame: true,
                ..LivingWikiRelevance::default()
            },
            None,
        )
        .unwrap();
        assert_eq!(plan.positions.len(), 6);
        for position in 0..=5 {
            assert!(plan.positions.contains(&position));
        }
    }

    #[test]
    fn budget_preserves_aperture_and_truthfully_reports_omission() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                relevant_positions: vec![2, 3, 4],
                requires_context: false,
                full_context_frame: false,
                position_budget: 4,
            },
            None,
        )
        .unwrap();
        assert_eq!(&plan.positions[..3], &[5, 0, 1]);
        assert_eq!(plan.positions, vec![5, 0, 1, 2]);
        assert_eq!(plan.omitted_relevant_positions, vec![3, 4]);
    }

    #[test]
    fn ordinary_mode_has_a_removal_law_and_no_ql_dependency() {
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Ordinary,
            LivingWikiRelevance {
                full_context_frame: true,
                ..LivingWikiRelevance::default()
            },
            Some(valid_formal_request()),
        )
        .unwrap();
        assert_eq!(plan.depth, ContextFrameDepth::None);
        assert!(plan.positions.is_empty());
        assert!(plan.refraction.is_none());
        assert!(!plan.ordinary_wiki_correctness_requires_ql);
        assert!(!plan.owns_source_freshness);
        assert!(!plan.owns_agent_orchestration);
    }

    #[test]
    fn formal_mode_reuses_existing_wiki_refraction_contract() {
        let request = valid_formal_request();
        let plan = plan_living_wiki_refraction(
            LivingWikiMode::Formal,
            LivingWikiRelevance::default(),
            Some(request.clone()),
        )
        .unwrap();
        assert_eq!(plan.refraction.as_ref().unwrap().contract, request.contract);
        assert_eq!(plan.depth, ContextFrameDepth::EntryAperture);
    }

    #[test]
    fn malformed_positions_and_under_budget_fail_instead_of_coercing() {
        let bad_position = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                relevant_positions: vec![6],
                ..LivingWikiRelevance::default()
            },
            None,
        );
        assert_eq!(
            bad_position.unwrap_err(),
            LivingWikiRefractionError::InvalidPosition(6)
        );

        let bad_budget = plan_living_wiki_refraction(
            LivingWikiMode::Explain,
            LivingWikiRelevance {
                position_budget: 2,
                ..LivingWikiRelevance::default()
            },
            None,
        );
        assert_eq!(
            bad_budget.unwrap_err(),
            LivingWikiRefractionError::PositionBudgetTooSmall(2)
        );
    }
}
