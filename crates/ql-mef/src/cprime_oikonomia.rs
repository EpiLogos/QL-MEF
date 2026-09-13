//! QL-owned completion of the source-defined C′ execution semantics.
//!
//! `VakComposition` remains the carrier and native execution remains with the
//! existing owners. This module makes two source-defined C′ determinations that
//! were previously thinner than the other four offices explicit and testable:
//!
//! * CFP chooses one of the six thread forms and can enter the governed Z cycle;
//! * CS chooses an exact source profile, direction and extent, then validates
//!   each performed Return against the selected CP pair.
//!
//! The implementation deliberately records these choices into the existing
//! reflective receipts so they are copied into `ReflectiveDerivation` and survive
//! through Return. It is not a scheduler, parser, workflow engine or second C
//! registry.

use crate::vak_composition::{
    Basis, CPrimeContext, CompositionError, FramedReading, ReflectiveReceipt, ReturnInput,
    Returned, SelectedAddress, VakComposition,
};
use ql_core::{QlPosition, VakFamily};

pub const C_PRIME_OIKONOMIA_CONTRACT: &str = "ql.cprime-oikonomia/v1";
pub const CFP_SOURCE_REF: &str =
    "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md#13-cfp-the-musical-forms-of-working";
pub const CS_SOURCE_REF: &str =
    "docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md#14-cs-the-actual-passage-through-paired-positions";

fn error(message: impl Into<String>) -> CompositionError {
    CompositionError(message.into())
}

fn require(test: bool, message: &str) -> Result<(), CompositionError> {
    if test { Ok(()) } else { Err(error(message)) }
}

fn nonempty(value: &str, what: &str) -> Result<(), CompositionError> {
    require(!value.trim().is_empty(), what)
}

fn evidence(values: &[String]) -> Result<(), CompositionError> {
    require(!values.is_empty(), "source-qualified operation requires evidence")?;
    for value in values {
        nonempty(value, "empty evidence reference")?;
    }
    Ok(())
}

/// The six source-defined CFP thread forms. Z is a governed cycle over work,
/// not a seventh ordinary thread-form position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfpThreadForm {
    Base,
    IndependentParallel,
    Chain,
    Fusion,
    Sustained,
    Nested,
}

impl CfpThreadForm {
    pub const ALL: [Self; 6] = [
        Self::Base,
        Self::IndependentParallel,
        Self::Chain,
        Self::Fusion,
        Self::Sustained,
        Self::Nested,
    ];

    pub const fn index(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::IndependentParallel => 1,
            Self::Chain => 2,
            Self::Fusion => 3,
            Self::Sustained => 4,
            Self::Nested => 5,
        }
    }

    pub const fn marker(self) -> &'static str {
        match self {
            Self::Base => "ql.cprime-oikonomia/v1:cfp0:base-one-voice",
            Self::IndependentParallel => {
                "ql.cprime-oikonomia/v1:cfp1:independent-parallel-p-thread"
            }
            Self::Chain => "ql.cprime-oikonomia/v1:cfp2:chained-c-thread",
            Self::Fusion => "ql.cprime-oikonomia/v1:cfp3:fusion-f-thread",
            Self::Sustained => "ql.cprime-oikonomia/v1:cfp4:sustained-l-thread",
            Self::Nested => "ql.cprime-oikonomia/v1:cfp5:nested-meta-b-thread",
        }
    }
}

/// The source-defined core of Z. Goal discovery, authorisation and reevaluation
/// qualify the cycle but do not replace these five ordered performance stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfpZStage {
    Compose,
    Perform,
    Record,
    Rehear,
    Recompose,
}

impl CfpZStage {
    pub const ORDER: [Self; 5] = [
        Self::Compose,
        Self::Perform,
        Self::Record,
        Self::Rehear,
        Self::Recompose,
    ];

    pub const fn marker(self) -> &'static str {
        match self {
            Self::Compose => "ql.cprime-oikonomia/v1:z:compose",
            Self::Perform => "ql.cprime-oikonomia/v1:z:perform",
            Self::Record => "ql.cprime-oikonomia/v1:z:record",
            Self::Rehear => "ql.cprime-oikonomia/v1:z:rehear",
            Self::Recompose => "ql.cprime-oikonomia/v1:z:recompose",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfpZStageReceipt {
    pub iteration: u64,
    pub stage: CfpZStage,
    pub result_ref: String,
    pub evidence: Vec<String>,
    pub basis: Basis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfpZCycle {
    pub undertaking_ref: String,
    pub goal_evidence: Vec<String>,
    pub authorisation_ref: String,
    pub authorisation_basis: Basis,
    pub iteration: u64,
    pub stages: Vec<CfpZStageReceipt>,
    pub reevaluation_evidence: Vec<String>,
}

impl CfpZCycle {
    pub fn authorised(
        undertaking_ref: impl Into<String>,
        goal_evidence: Vec<String>,
        authorisation_ref: impl Into<String>,
        authorisation_basis: Basis,
    ) -> Result<Self, CompositionError> {
        let undertaking_ref = undertaking_ref.into();
        let authorisation_ref = authorisation_ref.into();
        nonempty(&undertaking_ref, "Z requires an undertaking")?;
        nonempty(&authorisation_ref, "Z requires explicit authorisation")?;
        evidence(&goal_evidence)?;
        authorisation_basis.validate()?;
        Ok(Self {
            undertaking_ref,
            goal_evidence,
            authorisation_ref,
            authorisation_basis,
            iteration: 1,
            stages: Vec::new(),
            reevaluation_evidence: Vec::new(),
        })
    }

    pub fn expected_stage(&self) -> Option<CfpZStage> {
        CfpZStage::ORDER.get(self.stages.len()).copied()
    }

    pub fn complete(&self) -> bool {
        self.stages.len() == CfpZStage::ORDER.len()
    }

    pub fn advance(
        &mut self,
        stage: CfpZStage,
        result_ref: impl Into<String>,
        stage_evidence: Vec<String>,
        basis: Basis,
    ) -> Result<CfpZStageReceipt, CompositionError> {
        require(
            self.expected_stage() == Some(stage),
            "Z stage is out of source-defined order",
        )?;
        let result_ref = result_ref.into();
        nonempty(&result_ref, "Z stage requires an actual result reference")?;
        evidence(&stage_evidence)?;
        basis.validate()?;
        let receipt = CfpZStageReceipt {
            iteration: self.iteration,
            stage,
            result_ref,
            evidence: stage_evidence,
            basis,
        };
        self.stages.push(receipt.clone());
        Ok(receipt)
    }

    /// A completed cycle can continue only through attributable reevaluation.
    /// This is the source-defined recompose→new-ground seam, not an autonomous
    /// loop or a scheduler retry.
    pub fn reopen(
        &mut self,
        reevaluation_evidence: Vec<String>,
        basis: Basis,
    ) -> Result<(), CompositionError> {
        require(self.complete(), "Z cannot reopen before recompose")?;
        evidence(&reevaluation_evidence)?;
        basis.validate()?;
        self.reevaluation_evidence = reevaluation_evidence;
        self.iteration = self
            .iteration
            .checked_add(1)
            .ok_or_else(|| error("Z iteration overflow"))?;
        self.stages.clear();
        self.authorisation_basis = basis;
        Ok(())
    }

    pub fn authorisation_marker(&self) -> String {
        format!(
            "{C_PRIME_OIKONOMIA_CONTRACT}:z:authorised:{}",
            self.iteration
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsProfile {
    FullTraverse,
    QuickGroundingContext,
    GroundThroughOperation,
    ThroughPattern,
    ContextFocused,
    DirectSynthesis,
}

impl CsProfile {
    pub const ALL: [Self; 6] = [
        Self::FullTraverse,
        Self::QuickGroundingContext,
        Self::GroundThroughOperation,
        Self::ThroughPattern,
        Self::ContextFocused,
        Self::DirectSynthesis,
    ];

    pub const fn index(self) -> u8 {
        match self {
            Self::FullTraverse => 0,
            Self::QuickGroundingContext => 1,
            Self::GroundThroughOperation => 2,
            Self::ThroughPattern => 3,
            Self::ContextFocused => 4,
            Self::DirectSynthesis => 5,
        }
    }

    const fn pairs(self) -> &'static [(u8, u8)] {
        match self {
            Self::FullTraverse => &[(0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0)],
            Self::QuickGroundingContext => &[(0, 5), (1, 4)],
            Self::GroundThroughOperation => &[(0, 5), (1, 4), (2, 3)],
            Self::ThroughPattern => &[(0, 5), (1, 4), (2, 3), (3, 2)],
            Self::ContextFocused => &[(0, 5), (4, 1), (5, 0)],
            Self::DirectSynthesis => &[(0, 5), (5, 0)],
        }
    }

    pub fn passage(self, direction: CsDirection) -> CsPassage {
        let hops = self
            .pairs()
            .iter()
            .enumerate()
            .map(|(index, (left, right))| {
                let (from, to) = match direction {
                    CsDirection::ForwardSynthesis => (*left, *right),
                    CsDirection::ReturningInquiry => (*right, *left),
                };
                CsHop {
                    index,
                    paired_left: position(*left),
                    paired_right: position(*right),
                    from: position(from),
                    to: position(to),
                }
            })
            .collect();
        CsPassage {
            profile: self,
            direction,
            hops,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsDirection {
    ForwardSynthesis,
    ReturningInquiry,
}

impl CsDirection {
    pub const fn marker(self) -> &'static str {
        match self {
            Self::ForwardSynthesis => "forward-synthesis",
            Self::ReturningInquiry => "returning-inquiry",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsHop {
    pub index: usize,
    pub paired_left: QlPosition,
    pub paired_right: QlPosition,
    pub from: QlPosition,
    pub to: QlPosition,
}

impl CsHop {
    pub fn marker(&self, profile: CsProfile, direction: CsDirection) -> String {
        format!(
            "{C_PRIME_OIKONOMIA_CONTRACT}:cs{}:{}:{}:{}>{}",
            profile.index(),
            direction.marker(),
            self.index,
            self.from.value(),
            self.to.value()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsPassage {
    pub profile: CsProfile,
    pub direction: CsDirection,
    pub hops: Vec<CsHop>,
}

impl CsPassage {
    pub fn marker(&self) -> String {
        format!(
            "{C_PRIME_OIKONOMIA_CONTRACT}:cs{}:{}:extent:{}",
            self.profile.index(),
            self.direction.marker(),
            self.hops.len()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsExecutedHop {
    pub index: usize,
    pub return_ref: String,
    pub determination_ref: String,
    pub target_use: String,
    pub from: QlPosition,
    pub to: QlPosition,
}

/// State that qualifies the existing C′ context. Native work remains in
/// `VakComposition`/Factory/Actuation/Workcell; this owns only the QL meaning of
/// CFP form, Z and the directed CS walk.
#[derive(Debug, Clone, Default)]
pub struct CPrimeOikonomia {
    pub cfp_form: Option<CfpThreadForm>,
    pub z_cycle: Option<CfpZCycle>,
    pub cs_passage: Option<CsPassage>,
    pub cs_executed: Vec<CsExecutedHop>,
}

impl CPrimeOikonomia {
    pub fn select_cfp(
        &mut self,
        context: &mut CPrimeContext,
        form: CfpThreadForm,
        basis: Basis,
    ) -> Result<(), CompositionError> {
        basis.validate()?;
        let marker = form.marker().to_owned();
        context.operations.push(ReflectiveReceipt {
            family: VakFamily::Cfp,
            relation_id: VakFamily::Cfp.relation_id(),
            input_refs: vec![context.focus_use.clone(), CFP_SOURCE_REF.to_owned()],
            output_refs: vec![marker],
            basis,
        });
        self.cfp_form = Some(form);
        Ok(())
    }

    pub fn begin_z(
        &mut self,
        context: &mut CPrimeContext,
        cycle: CfpZCycle,
    ) -> Result<(), CompositionError> {
        require(self.z_cycle.is_none(), "Z is already active")?;
        let marker = cycle.authorisation_marker();
        context.operations.push(ReflectiveReceipt {
            family: VakFamily::Cfp,
            relation_id: VakFamily::Cfp.relation_id(),
            input_refs: std::iter::once(cycle.undertaking_ref.clone())
                .chain(std::iter::once(cycle.authorisation_ref.clone()))
                .chain(cycle.goal_evidence.iter().cloned())
                .chain(std::iter::once(CFP_SOURCE_REF.to_owned()))
                .collect(),
            output_refs: vec![marker],
            basis: cycle.authorisation_basis.clone(),
        });
        self.z_cycle = Some(cycle);
        Ok(())
    }

    pub fn advance_z(
        &mut self,
        context: &mut CPrimeContext,
        stage: CfpZStage,
        result_ref: impl Into<String>,
        stage_evidence: Vec<String>,
        basis: Basis,
    ) -> Result<(), CompositionError> {
        let cycle = self
            .z_cycle
            .as_mut()
            .ok_or_else(|| error("Z has not been authorised"))?;
        let receipt = cycle.advance(stage, result_ref, stage_evidence, basis)?;
        context.operations.push(ReflectiveReceipt {
            family: VakFamily::Cfp,
            relation_id: VakFamily::Cfp.relation_id(),
            input_refs: std::iter::once(cycle.undertaking_ref.clone())
                .chain(receipt.evidence.iter().cloned())
                .chain(std::iter::once(CFP_SOURCE_REF.to_owned()))
                .collect(),
            output_refs: vec![receipt.stage.marker().to_owned(), receipt.result_ref],
            basis: receipt.basis,
        });
        Ok(())
    }

    pub fn reopen_z(
        &mut self,
        context: &mut CPrimeContext,
        reevaluation_evidence: Vec<String>,
        basis: Basis,
    ) -> Result<(), CompositionError> {
        let cycle = self
            .z_cycle
            .as_mut()
            .ok_or_else(|| error("Z has not been authorised"))?;
        cycle.reopen(reevaluation_evidence.clone(), basis.clone())?;
        context.operations.push(ReflectiveReceipt {
            family: VakFamily::Cfp,
            relation_id: VakFamily::Cfp.relation_id(),
            input_refs: reevaluation_evidence,
            output_refs: vec![cycle.authorisation_marker()],
            basis,
        });
        Ok(())
    }

    /// Select an exact CS profile before determination. Because this emits a
    /// normal C′ receipt, `CPrimeContext::determine` retains profile, direction
    /// and extent inside the resulting `ReflectiveDerivation`.
    pub fn select_cs(
        &mut self,
        context: &mut CPrimeContext,
        passage: CsPassage,
        basis: Basis,
    ) -> Result<(), CompositionError> {
        basis.validate()?;
        require(!passage.hops.is_empty(), "CS passage cannot be empty")?;
        let mut outputs = vec![passage.marker()];
        outputs.extend(
            passage
                .hops
                .iter()
                .map(|hop| hop.marker(passage.profile, passage.direction)),
        );
        context.operations.push(ReflectiveReceipt {
            family: VakFamily::Cs,
            relation_id: VakFamily::Cs.relation_id(),
            input_refs: vec![context.focus_use.clone(), CS_SOURCE_REF.to_owned()],
            output_refs: outputs,
            basis,
        });
        self.cs_passage = Some(passage);
        self.cs_executed.clear();
        Ok(())
    }

    /// Execute the next selected CS hop through the existing Return operation.
    /// The determination and target must occupy the exact CP pair selected by
    /// profile+direction. This is what prevents CS from degrading to a label on
    /// an otherwise generic Return.
    pub fn perform_cs_hop(
        &mut self,
        context: &mut CPrimeContext,
        graph: &mut VakComposition,
        input: ReturnInput,
    ) -> Result<(), CompositionError> {
        let passage = self
            .cs_passage
            .as_ref()
            .ok_or_else(|| error("CS profile has not been selected"))?;
        let index = self.cs_executed.len();
        let expected = passage
            .hops
            .get(index)
            .ok_or_else(|| error("selected CS passage is already complete"))?;

        let determination = graph.determination(&input.determination)?;
        let source_position = cp_position(&determination.reading)?;
        let target = graph.whole(&input.target_use)?;
        let target_reading = graph.read(&input.target_use, target.frame.lens)?;
        let target_position = cp_position(&target_reading)?;
        require(
            source_position == expected.from && target_position == expected.to,
            "Return does not follow the next source-defined CS pair/direction",
        )?;

        let executed = CsExecutedHop {
            index,
            return_ref: input.reference.clone(),
            determination_ref: input.determination.clone(),
            target_use: input.target_use.clone(),
            from: source_position,
            to: target_position,
        };
        context.cs(graph, input)?;
        self.cs_executed.push(executed);
        Ok(())
    }

    pub fn cs_complete(&self) -> bool {
        self.cs_passage
            .as_ref()
            .is_some_and(|passage| self.cs_executed.len() == passage.hops.len())
    }

    /// Package an existing Return with the operative C′ determinations that
    /// qualify it. The base Return is cloned unchanged; this is an attributable
    /// semantic envelope, not a replacement Return type or persistence owner.
    pub fn returned(
        &self,
        graph: &VakComposition,
        return_ref: &str,
    ) -> Result<OperativeReturn, CompositionError> {
        let base = graph.returned(return_ref)?.clone();
        let form = self
            .cfp_form
            .ok_or_else(|| error("Return has no explicit CFP form"))?;
        let passage = self
            .cs_passage
            .clone()
            .ok_or_else(|| error("Return has no explicit CS profile/direction"))?;
        let derivation = base
            .producing
            .context
            .as_ref()
            .ok_or_else(|| error("Return was not produced through a C′ derivation"))?;
        require(
            derivation
                .operations
                .iter()
                .any(|receipt| receipt.output_refs.iter().any(|r| r == form.marker())),
            "CFP form was selected after determination and did not survive into Return",
        )?;
        let passage_marker = passage.marker();
        require(
            derivation.operations.iter().any(|receipt| {
                receipt
                    .output_refs
                    .iter()
                    .any(|reference| reference == &passage_marker)
            }),
            "CS profile was selected after determination and did not survive into Return",
        )?;
        Ok(OperativeReturn {
            base,
            cfp_form: form,
            z_cycle: self.z_cycle.clone(),
            cs_passage: passage,
            cs_executed: self.cs_executed.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct OperativeReturn {
    pub base: Returned,
    pub cfp_form: CfpThreadForm,
    pub z_cycle: Option<CfpZCycle>,
    pub cs_passage: CsPassage,
    pub cs_executed: Vec<CsExecutedHop>,
}

fn position(value: u8) -> QlPosition {
    QlPosition::new(value).expect("source-defined CS position is modulo-six")
}

fn cp_position(reading: &FramedReading) -> Result<QlPosition, CompositionError> {
    match &reading.address {
        SelectedAddress::Member { member, .. } => Ok(member.coordinate.position),
        SelectedAddress::Anchor(_) => Ok(reading.frame.coordinate().position),
        SelectedAddress::Relation { .. } => Err(error(
            "CS requires a positioned participant, not an unresolved relation whole",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ql_core::CallerProvenance;

    fn basis(source: &str) -> Basis {
        Basis {
            provenance: CallerProvenance::new("test/caller", source, "DERIVED").unwrap(),
            revision: "1".into(),
            evidence: vec!["test/evidence".into()],
        }
    }

    #[test]
    fn all_six_cfp_forms_are_distinct_and_source_positioned() {
        let indexes: Vec<_> = CfpThreadForm::ALL.iter().map(|f| f.index()).collect();
        assert_eq!(indexes, vec![0, 1, 2, 3, 4, 5]);
        let markers: std::collections::BTreeSet<_> =
            CfpThreadForm::ALL.iter().map(|f| f.marker()).collect();
        assert_eq!(markers.len(), 6);
    }

    #[test]
    fn z_requires_authorisation_and_advances_only_in_source_order() {
        assert!(
            CfpZCycle::authorised("work", vec![], "approval", basis("z/start")).is_err()
        );
        let mut cycle = CfpZCycle::authorised(
            "work",
            vec!["goal/source".into()],
            "approval",
            basis("z/start"),
        )
        .unwrap();
        assert!(
            cycle
                .advance(
                    CfpZStage::Perform,
                    "result/early",
                    vec!["e".into()],
                    basis("z/early")
                )
                .is_err()
        );
        for stage in CfpZStage::ORDER {
            cycle
                .advance(
                    stage,
                    format!("result/{stage:?}"),
                    vec![format!("evidence/{stage:?}")],
                    basis("z/stage"),
                )
                .unwrap();
        }
        assert!(cycle.complete());
        assert_eq!(cycle.expected_stage(), None);
        cycle
            .reopen(vec!["human/reevaluation".into()], basis("z/reopen"))
            .unwrap();
        assert_eq!(cycle.iteration, 2);
        assert_eq!(cycle.expected_stage(), Some(CfpZStage::Compose));
    }

    #[test]
    fn all_six_cs_profiles_preserve_exact_source_extent() {
        let lengths: Vec<_> = CsProfile::ALL
            .iter()
            .map(|profile| profile.passage(CsDirection::ForwardSynthesis).hops.len())
            .collect();
        assert_eq!(lengths, vec![6, 2, 3, 4, 3, 2]);

        let cs4 = CsProfile::ContextFocused.passage(CsDirection::ForwardSynthesis);
        let pairs: Vec<_> = cs4
            .hops
            .iter()
            .map(|hop| (hop.paired_left.value(), hop.paired_right.value()))
            .collect();
        assert_eq!(pairs, vec![(0, 5), (4, 1), (5, 0)]);
    }

    #[test]
    fn returning_inquiry_changes_pair_direction_without_reversing_profile_order() {
        let forward = CsProfile::ThroughPattern.passage(CsDirection::ForwardSynthesis);
        let returning = CsProfile::ThroughPattern.passage(CsDirection::ReturningInquiry);
        assert_eq!(forward.hops.len(), returning.hops.len());
        for (f, r) in forward.hops.iter().zip(&returning.hops) {
            assert_eq!(f.paired_left, r.paired_left);
            assert_eq!(f.paired_right, r.paired_right);
            assert_eq!(f.from, r.to);
            assert_eq!(f.to, r.from);
        }
    }
}
