//! QL-owned operative-scope binding over the existing C′ composition owner.
//!
//! AIKit owns Resolve/Context and effect-point admission. QL owns the semantic
//! binding that says which actual compiled C′ whole/profile/source basis is being
//! interpreted. This module adds no scope registry: public binding and
//! reobservation enter through [`VakComposition`], which compiles the current QL
//! whole internally. A client-supplied `CompiledProfile` is never evidence of
//! currentness.

use std::collections::{BTreeMap, BTreeSet};

use ql_core::QlFace;
use serde::{Deserialize, Serialize};

use crate::vak_composition::{CompositionError, PositionBasis, Result, VakComposition};
use crate::vak_profile::{
    CPrimeProfile, CompiledProfile, PROFILE_CONTRACT, PROFILE_SOURCE, PROFILE_SOURCE_BLOB,
    constitutional_voice,
};
use crate::{MusicalBasis, VakStanding};

pub const OPERATIVE_BINDING_CONTRACT: &str = "ql.cprime-operative-binding/v1";
pub const OPERATIVE_PROVIDER_REF: &str = "provider/ql-mef";
pub const OPERATIVE_OWNER_REF: &str = "QL-MEF";
pub const C_PRIME_INTERPRETATION_REF: &str = "ql/interpretation/c-prime";
const MAX_REF: usize = 16_384;
const MAX_SOURCES: usize = 4096;

fn error(message: impl Into<String>) -> CompositionError {
    CompositionError(message.into())
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(error(message))
    }
}

fn reference(value: &str, message: &str) -> Result<()> {
    require(
        !value.trim().is_empty() && value.len() <= MAX_REF && !value.contains('\0'),
        message,
    )
}

/// Correlations whose native identity remains outside QL. The QL binding
/// identity itself is derived from the compiled whole/subject and is therefore
/// deliberately absent here. A changed world occasion or Method selection still
/// makes reobservation stale without becoming QL-owned source material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativeScopeCorrelation {
    pub world_ref: String,
    pub world_generation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_skill_ref: Option<String>,
}

impl OperativeScopeCorrelation {
    fn validate(&self) -> Result<()> {
        reference(&self.world_ref, "missing correlated world ref")?;
        reference(
            &self.world_generation,
            "missing correlated world generation",
        )?;
        if let Some(method) = &self.method_skill_ref {
            reference(method, "invalid Method-classified Skill ref")?;
        }
        Ok(())
    }
}

/// One QL source retained by the compiled whole. Repeated use of the same source
/// revision is collapsed only after its callers, standings and evidence are
/// unioned; conflicting revisions for one source are refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativeBindingSource {
    pub source_ref: String,
    pub revision: String,
    pub caller_refs: BTreeSet<String>,
    pub standing_refs: BTreeSet<String>,
    pub evidence_refs: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperativeFrameReading {
    pub context_frame: String,
    pub constitutional_voice: String,
    pub lens: String,
    pub musical_basis: String,
    pub face: String,
    pub position_basis: String,
    pub frame_pitch: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CPrimeOperativeBinding {
    pub contract: String,
    pub provider_ref: String,
    pub binding_ref: String,
    pub binding_revision: String,
    pub owner_ref: String,
    pub interpretation_ref: String,
    pub interpretation_revision: String,
    pub profile_contract: String,
    pub profile_source: String,
    pub profile_source_blob: String,
    pub world_ref: String,
    pub world_generation: String,
    pub whole_ref: String,
    pub subject_ref: String,
    pub frame: OperativeFrameReading,
    pub profile: CPrimeProfile,
    pub sources: Vec<OperativeBindingSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_skill_ref: Option<String>,
    pub evidence_refs: BTreeSet<String>,
    pub standing: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case", deny_unknown_fields)]
pub enum OperativeScopeObservation {
    Current {
        binding: CPrimeOperativeBinding,
    },
    Stale {
        observed: CPrimeOperativeBinding,
        differences: Vec<String>,
    },
    Missing {
        binding_ref: String,
        current_whole_ref: String,
        reason: String,
    },
}

impl VakComposition {
    /// Bind from the current QL composition owner. The caller identifies an
    /// existing whole and the source-defined profile to compile; it cannot hand
    /// QL a prebuilt reading or binding identity and have either certify itself.
    pub fn bind_operative_scope(
        &self,
        whole_use: &str,
        profile: CPrimeProfile,
        correlation: OperativeScopeCorrelation,
    ) -> Result<CPrimeOperativeBinding> {
        let compiled = self.compile_profile(whole_use, profile)?;
        binding_from_compiled(&compiled, correlation)
    }

    /// Reobserve the original profile against the provider-selected current QL
    /// whole. QL wholes are immutable uses, so the currently selected use may be
    /// a different whole identity; that difference is retained as stale instead
    /// of silently continuing to read the historical whole.
    pub fn reobserve_operative_scope(
        &self,
        expected: &CPrimeOperativeBinding,
        current_whole_use: &str,
        correlation: OperativeScopeCorrelation,
    ) -> Result<OperativeScopeObservation> {
        require(
            expected.contract == OPERATIVE_BINDING_CONTRACT,
            "unsupported QL operative binding contract",
        )?;
        correlation.validate()?;
        reference(current_whole_use, "missing current QL whole selection")?;

        let current = match self.compile_profile(current_whole_use, expected.profile.clone()) {
            Ok(compiled) => compiled,
            Err(failure) => {
                return Ok(OperativeScopeObservation::Missing {
                    binding_ref: expected.binding_ref.clone(),
                    current_whole_ref: current_whole_use.into(),
                    reason: failure.to_string(),
                });
            }
        };
        observe_compiled(expected, &current, correlation)
    }
}

/// Private projection from a profile already compiled by the QL owner. Keeping
/// this helper private prevents external code from constructing a public
/// `CompiledProfile` and using it as proof of currentness.
fn binding_from_compiled(
    compiled: &CompiledProfile,
    correlation: OperativeScopeCorrelation,
) -> Result<CPrimeOperativeBinding> {
    correlation.validate()?;
    require(
        compiled.contract == PROFILE_CONTRACT,
        "operative binding requires the current C-prime profile contract",
    )?;
    reference(&compiled.whole_use, "compiled profile has no whole")?;
    reference(&compiled.subject_ref, "compiled profile has no subject")?;
    require(
        !compiled.basis.is_empty() && compiled.basis.len() <= MAX_SOURCES,
        "compiled profile requires a bounded source basis",
    )?;
    for basis in &compiled.basis {
        basis.validate()?;
    }

    let sources = source_projection(compiled)?;
    let frame = frame_reading(compiled);
    let binding_ref = binding_identity(compiled)?;
    let binding_revision = binding_revision(compiled, &frame, &sources)?;
    let mut evidence_refs = BTreeSet::from([
        PROFILE_SOURCE.to_owned(),
        format!("git-blob:{PROFILE_SOURCE_BLOB}"),
    ]);
    for source in &sources {
        evidence_refs.extend(source.evidence_refs.iter().cloned());
    }

    Ok(CPrimeOperativeBinding {
        contract: OPERATIVE_BINDING_CONTRACT.into(),
        provider_ref: OPERATIVE_PROVIDER_REF.into(),
        binding_ref,
        binding_revision,
        owner_ref: OPERATIVE_OWNER_REF.into(),
        interpretation_ref: C_PRIME_INTERPRETATION_REF.into(),
        interpretation_revision: PROFILE_SOURCE_BLOB.into(),
        profile_contract: PROFILE_CONTRACT.into(),
        profile_source: PROFILE_SOURCE.into(),
        profile_source_blob: PROFILE_SOURCE_BLOB.into(),
        world_ref: correlation.world_ref,
        world_generation: correlation.world_generation,
        whole_ref: compiled.whole_use.clone(),
        subject_ref: compiled.subject_ref.clone(),
        frame,
        profile: compiled.profile.clone(),
        sources,
        method_skill_ref: correlation.method_skill_ref,
        evidence_refs,
        standing: VakStanding::Derived.as_schema_str().into(),
    })
}

fn observe_compiled(
    expected: &CPrimeOperativeBinding,
    current: &CompiledProfile,
    correlation: OperativeScopeCorrelation,
) -> Result<OperativeScopeObservation> {
    let observed = binding_from_compiled(current, correlation)?;
    let differences = binding_differences(expected, &observed);
    if differences.is_empty() {
        Ok(OperativeScopeObservation::Current { binding: observed })
    } else {
        Ok(OperativeScopeObservation::Stale {
            observed,
            differences,
        })
    }
}

fn source_projection(compiled: &CompiledProfile) -> Result<Vec<OperativeBindingSource>> {
    let mut by_source = BTreeMap::<String, OperativeBindingSource>::new();
    for basis in &compiled.basis {
        let source_ref = basis.provenance.source_ref.clone();
        let caller_ref = basis.provenance.caller_ref.clone();
        let standing_ref = basis.provenance.standing_ref.clone();
        let revision = basis.revision.clone();
        reference(&source_ref, "missing operative source ref")?;
        reference(&caller_ref, "missing operative source caller")?;
        reference(&standing_ref, "missing operative source standing")?;
        reference(&revision, "missing operative source revision")?;
        match by_source.get_mut(&source_ref) {
            Some(existing) => {
                require(
                    existing.revision == revision,
                    "one operative source has conflicting revisions",
                )?;
                existing.caller_refs.insert(caller_ref);
                existing.standing_refs.insert(standing_ref);
                existing
                    .evidence_refs
                    .extend(basis.evidence.iter().cloned());
            }
            None => {
                by_source.insert(
                    source_ref.clone(),
                    OperativeBindingSource {
                        source_ref,
                        revision,
                        caller_refs: BTreeSet::from([caller_ref]),
                        standing_refs: BTreeSet::from([standing_ref]),
                        evidence_refs: basis.evidence.iter().cloned().collect(),
                    },
                );
            }
        }
    }
    require(
        !by_source.is_empty() && by_source.len() <= MAX_SOURCES,
        "operative binding has no bounded source projection",
    )?;
    Ok(by_source.into_values().collect())
}

fn frame_reading(compiled: &CompiledProfile) -> OperativeFrameReading {
    OperativeFrameReading {
        context_frame: compiled.frame.id.code().into(),
        constitutional_voice: constitutional_voice(compiled.frame.id).into(),
        lens: compiled.frame.lens.code().into(),
        musical_basis: match compiled.frame.basis {
            MusicalBasis::Chromatic => "chromatic",
            MusicalBasis::Fifths => "fifths",
        }
        .into(),
        face: match compiled.frame.face {
            QlFace::Direct => "direct",
            QlFace::Conjugate => "conjugate",
        }
        .into(),
        position_basis: match compiled.frame.positions {
            PositionBasis::Local => "local",
            PositionBasis::Absolute => "absolute",
        }
        .into(),
        frame_pitch: compiled.frame_pitch(),
    }
}

/// QL owns the binding identity. It is the stable relation between one compiled
/// whole and its subject; changing profile/frame/source changes the revision,
/// while changing whole/subject changes the binding identity itself.
fn binding_identity(compiled: &CompiledProfile) -> Result<String> {
    let identity = format!(
        "{OPERATIVE_BINDING_CONTRACT}|{C_PRIME_INTERPRETATION_REF}|whole={}|subject={}",
        compiled.whole_use, compiled.subject_ref
    );
    reference(&identity, "operative binding identity is invalid")?;
    Ok(identity)
}

/// Transparent QL owner revision over the complete compiled semantic basis. The
/// exact source projection includes every source revision plus caller/standing/
/// evidence refs, so a non-latest source change cannot retain the old revision.
fn binding_revision(
    compiled: &CompiledProfile,
    frame: &OperativeFrameReading,
    sources: &[OperativeBindingSource],
) -> Result<String> {
    let profile = serde_json::to_string(&compiled.profile)
        .map_err(|failure| CompositionError(format!("cannot encode C-prime profile: {failure}")))?;
    let source_basis = serde_json::to_string(sources)
        .map_err(|failure| CompositionError(format!("cannot encode operative source basis: {failure}")))?;
    let revision = format!(
        "{OPERATIVE_BINDING_CONTRACT}|{PROFILE_SOURCE_BLOB}|whole={}|subject={}|{}|{}|{}|{}|{}|{}|sources={}",
        compiled.whole_use,
        compiled.subject_ref,
        frame.context_frame,
        frame.lens,
        frame.musical_basis,
        frame.face,
        frame.position_basis,
        profile,
        source_basis
    );
    reference(&revision, "operative binding revision is invalid")?;
    Ok(revision)
}

fn binding_differences(
    expected: &CPrimeOperativeBinding,
    observed: &CPrimeOperativeBinding,
) -> Vec<String> {
    let mut differences = Vec::new();
    macro_rules! changed {
        ($field:ident, $name:literal) => {
            if expected.$field != observed.$field {
                differences.push($name.into());
            }
        };
    }
    changed!(provider_ref, "provider");
    changed!(binding_ref, "binding");
    changed!(binding_revision, "binding-revision");
    changed!(owner_ref, "owner");
    changed!(interpretation_ref, "interpretation");
    changed!(interpretation_revision, "interpretation-revision");
    changed!(world_ref, "world");
    changed!(world_generation, "world-generation");
    changed!(whole_ref, "whole");
    changed!(subject_ref, "subject");
    changed!(frame, "frame");
    changed!(profile, "c-prime-profile");
    changed!(sources, "source-basis");
    changed!(method_skill_ref, "method-skill");
    changed!(evidence_refs, "evidence");
    differences
}

#[cfg(test)]
mod tests {
    use ql_core::{CallerProvenance, QlFace};

    use crate::vak_composition::{ActiveFrame, Basis, PositionBasis};
    use crate::vak_profile::{
        ContentPosition, ContentType, ContextSequence, InquiryDirection, Participation, ThreadForm,
    };
    use crate::{ContextFrameId, LensId, MusicalBasis, VakStanding};

    use super::*;

    fn basis(source: &str, revision: &str, evidence: &str) -> Basis {
        Basis {
            provenance: CallerProvenance::new(
                "agent:anima",
                source,
                VakStanding::AuthoredArchitecture.as_schema_str(),
            )
            .unwrap(),
            revision: revision.into(),
            evidence: vec![evidence.into()],
        }
    }

    fn profile() -> CPrimeProfile {
        CPrimeProfile {
            participation: Participation::AuthorisedUndertaking,
            content: ContentType::Operations,
            position: ContentPosition::Operation,
            thread: ThreadForm::Chain,
            sequence: ContextSequence::ThroughOperation,
            direction: InquiryDirection::Forward,
        }
    }

    fn compiled() -> CompiledProfile {
        let profile = profile();
        CompiledProfile {
            contract: PROFILE_CONTRACT,
            whole_use: "whole:undertaking".into(),
            subject_ref: "subject:nara".into(),
            frame: ActiveFrame {
                id: ContextFrameId::Cf5,
                lens: LensId::L0,
                basis: MusicalBasis::Chromatic,
                face: QlFace::Direct,
                positions: PositionBasis::Local,
            },
            pairs: profile.sequence.pairs(),
            walk: profile.sequence.walk(profile.direction),
            profile,
            basis: vec![basis("source:vak", "source-r1", "evidence:vak")],
        }
    }

    fn correlation() -> OperativeScopeCorrelation {
        OperativeScopeCorrelation {
            world_ref: "world/one".into(),
            world_generation: "generation-1".into(),
            method_skill_ref: Some("skill/recognised/revisit".into()),
        }
    }

    #[test]
    fn owner_projection_retains_source_qualified_binding() {
        let binding = binding_from_compiled(&compiled(), correlation()).unwrap();
        assert_eq!(binding.contract, OPERATIVE_BINDING_CONTRACT);
        assert_eq!(binding.provider_ref, OPERATIVE_PROVIDER_REF);
        assert_eq!(binding.owner_ref, OPERATIVE_OWNER_REF);
        assert_eq!(binding.interpretation_ref, C_PRIME_INTERPRETATION_REF);
        assert_eq!(binding.interpretation_revision, PROFILE_SOURCE_BLOB);
        assert_eq!(
            binding.binding_ref,
            "ql.cprime-operative-binding/v1|ql/interpretation/c-prime|whole=whole:undertaking|subject=subject:nara"
        );
        assert_eq!(binding.whole_ref, "whole:undertaking");
        assert_eq!(binding.subject_ref, "subject:nara");
        assert_eq!(binding.frame.context_frame, "CF5");
        assert_eq!(binding.frame.constitutional_voice, "Anima");
        assert_eq!(binding.sources[0].source_ref, "source:vak");
        assert!(binding.evidence_refs.contains("evidence:vak"));
    }

    #[test]
    fn freshly_compiled_same_profile_reobserves_current() {
        let current = compiled();
        let binding = binding_from_compiled(&current, correlation()).unwrap();
        let observation = observe_compiled(&binding, &current, correlation()).unwrap();
        assert!(matches!(
            observation,
            OperativeScopeObservation::Current { .. }
        ));
    }

    #[test]
    fn ql_binding_identity_changes_with_whole_or_subject_not_client_input() {
        let baseline = binding_from_compiled(&compiled(), correlation()).unwrap();
        let mut changed_whole = compiled();
        changed_whole.whole_use = "whole:other".into();
        let other_whole = binding_from_compiled(&changed_whole, correlation()).unwrap();
        assert_ne!(baseline.binding_ref, other_whole.binding_ref);
        assert!(binding_differences(&baseline, &other_whole).contains(&"binding".to_string()));

        let mut changed_subject = compiled();
        changed_subject.subject_ref = "subject:other".into();
        let other_subject = binding_from_compiled(&changed_subject, correlation()).unwrap();
        assert_ne!(baseline.binding_ref, other_subject.binding_ref);
        assert!(binding_differences(&baseline, &other_subject).contains(&"binding".to_string()));
    }

    #[test]
    fn binding_revision_covers_every_source_revision_not_only_the_last() {
        let mut multiple = compiled();
        multiple
            .basis
            .push(basis("source:z-last", "source-z1", "evidence:z"));
        let baseline = binding_from_compiled(&multiple, correlation()).unwrap();
        multiple.basis[0].revision = "source-r2".into();
        let changed = binding_from_compiled(&multiple, correlation()).unwrap();
        assert_ne!(baseline.binding_revision, changed.binding_revision);
        assert!(binding_differences(&baseline, &changed).contains(&"source-basis".to_string()));
    }

    #[test]
    fn changed_profile_frame_source_or_world_is_stale() {
        let baseline = compiled();
        let binding = binding_from_compiled(&baseline, correlation()).unwrap();

        let mut changed_profile = compiled();
        changed_profile.profile.thread = ThreadForm::Parallel;
        let observation = observe_compiled(&binding, &changed_profile, correlation()).unwrap();
        assert!(matches!(
            observation,
            OperativeScopeObservation::Stale { ref differences, .. }
                if differences.contains(&"c-prime-profile".to_string())
        ));

        let mut changed_frame = compiled();
        changed_frame.frame.lens = LensId::L1;
        assert!(matches!(
            observe_compiled(&binding, &changed_frame, correlation()).unwrap(),
            OperativeScopeObservation::Stale { .. }
        ));

        let mut changed_source = compiled();
        changed_source.basis[0].revision = "source-r2".into();
        assert!(matches!(
            observe_compiled(&binding, &changed_source, correlation()).unwrap(),
            OperativeScopeObservation::Stale { .. }
        ));

        let mut changed_world = correlation();
        changed_world.world_generation = "generation-2".into();
        assert!(matches!(
            observe_compiled(&binding, &baseline, changed_world).unwrap(),
            OperativeScopeObservation::Stale { .. }
        ));
    }

    #[test]
    fn source_widening_and_method_change_are_visible_not_silent() {
        let baseline = compiled();
        let binding = binding_from_compiled(&baseline, correlation()).unwrap();
        let mut widened = compiled();
        widened
            .basis
            .push(basis("source:extra", "extra-r1", "evidence:extra"));
        let observation = observe_compiled(&binding, &widened, correlation()).unwrap();
        assert!(matches!(
            observation,
            OperativeScopeObservation::Stale { ref differences, .. }
                if differences.contains(&"source-basis".to_string())
        ));

        let mut method = correlation();
        method.method_skill_ref = Some("skill/recognised/other".into());
        assert!(matches!(
            observe_compiled(&binding, &baseline, method).unwrap(),
            OperativeScopeObservation::Stale { .. }
        ));
    }

    #[test]
    fn conflicting_revisions_for_one_source_are_refused() {
        let mut conflicting = compiled();
        conflicting
            .basis
            .push(basis("source:vak", "source-r2", "evidence:changed"));
        assert!(binding_from_compiled(&conflicting, correlation()).is_err());
    }

    #[test]
    fn public_owner_api_refuses_missing_bind_and_reports_missing_reobservation() {
        let graph = VakComposition::default();
        assert!(
            graph
                .bind_operative_scope("whole:missing", profile(), correlation())
                .is_err()
        );

        let binding = binding_from_compiled(&compiled(), correlation()).unwrap();
        let expected_ref = binding.binding_ref.clone();
        let observation = graph
            .reobserve_operative_scope(&binding, "whole:missing", correlation())
            .unwrap();
        assert!(matches!(
            observation,
            OperativeScopeObservation::Missing { ref binding_ref, .. }
                if binding_ref == &expected_ref
        ));
    }
}
