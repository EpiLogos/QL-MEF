//! Correlate a supplied native execution observation; QL does not execute or
//! authenticate an external actor merely because it accepts a typed receipt.
use super::*;
use crate::{VakExecutionObservationV1, VakPathV1, reconstruct_observed_vak_path_for_revision};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativePathCorrelation {
    pub reference: String,
    pub determination: String,
    pub native_node_ref: String,
    pub native_step_ref: String,
    pub path: VakPathV1,
    pub basis: Basis,
    pub standing: VakStanding,
}
#[derive(Debug, Clone)]
pub struct NativePathInput {
    pub reference: String,
    pub determination: String,
    pub native_step_ref: String,
    pub observation: VakExecutionObservationV1,
    pub basis: Basis,
}
impl VakComposition {
    pub fn record_native_observation(&mut self, registry: &VakRegistry, input: NativePathInput) -> Result<NativePathCorrelation> {
        reference(&input.reference)?;
        reference(&input.native_step_ref)?;
        input.basis.validate()?;
        let d = self.determination(&input.determination)?;
        require(d.native_paths.len() < MAX_DEPTH, "native observation correlation bound exceeded")?;
        require(!d.native_paths.iter().any(|p| p.reference == input.reference), "native correlation already exists")?;
        let language = d.language.as_ref().ok_or_else(|| err("native correlation requires the producing full-profile expression"))?;
        let contribution = d.contribution.as_ref().ok_or_else(|| err("native generation requires attributable Agent input"))?;
        language.validate(registry)?;
        require(input.observation.actor_ref.as_deref() == Some(&contribution.actor_ref), "native observation actor differs from producing Agent")?;
        let general = &input.observation.general_expression;
        require(general.syntax_version == language.general.syntax_version
            && general.owner_revision == language.general.owner_revision
            && general.resolve_path_identity == language.general.resolve_path_identity
            && general.rendered == language.general.rendered
            && general.full_vak_rendering == language.general.full_vak_rendering,
            "native expression/path does not co-refer with the determination")?;
        require(input.observation.world_ref == language.reading.world_ref
            && input.observation.project_ref == language.reading.project_ref
            && input.observation.focus_ref == language.reading.focus_ref,
            "native observation changes the interpreted World/Project/Focus")?;
        references(&input.observation.evidence_refs)?;
        let mut ids = BTreeSet::new();
        for step in &input.observation.steps {
            require(ids.insert(step.step_id.clone()), "native path has duplicate step identities")?;
            references(&step.evidence_refs)?;
            for r in step.invocation_ref.iter().chain(step.activity_ref.iter()).chain(step.return_ref.iter()).chain(step.result_refs.iter()) {
                reference(r)?;
            }
        }
        let path = reconstruct_observed_vak_path_for_revision(registry, input.observation, &language.accepted_syntax_revision).map_err(err)?;
        let step = path.steps.iter().find(|s| s.step_id == input.native_step_ref)
            .ok_or_else(|| err("native step is not in the supplied path"))?;
        require(step.expression.operator == language.reading.operator
            && step.expression.horizon == language.reading.horizon
            && step.expression.subjects == language.reading.subjects
            && step.expression.relation_refs == language.reading.relation_refs
            && step.expression.complement_refs == language.reading.complement_refs,
            "native step changes the full-profile subjects or source relations")?;
        require(step.result_refs.contains(&contribution.result_ref)
            || step.return_ref.as_deref() == Some(&contribution.result_ref),
            "native returned result is not the declared Agent contribution")?;
        let value = NativePathCorrelation { reference: input.reference, determination: input.determination.clone(),
            native_node_ref: language.native_node_ref.clone(), native_step_ref: input.native_step_ref,
            path, basis: input.basis, standing: VakStanding::Derived };
        self.determinations.get_mut(&input.determination).expect("validated determination").native_paths.push(value.clone());
        Ok(value)
    }
}
