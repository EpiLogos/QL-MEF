use ql_core::{
    COMPRESS_TO_WHOLE_OPERATOR_REF, CallerProvenance, QlFace, QlShape, QlShapeCompression,
    ShapeBinding, StructuralConstellation, StructuralParticipation, THREE_TO_ONE_OPERATOR_REF,
    resolve_shape_ref,
};
use ql_semantic::QlProvider;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    WikiRefractionEngine, WikiRefractionError, WikiRefractionRequest, WikiRefractionResponse,
    WikiRefractionTarget,
};

/// Reserved Wiki extension carrying a QL-owned shape binding view. The value is
/// transport/projection metadata; QL shape semantics remain owned by `ql-core`.
pub const WIKI_SHAPE_BINDING_EXTENSION: &str = "ql-mef/shape-binding/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiShapeBindingProvenance {
    pub caller_ref: String,
    pub source_ref: String,
    pub standing_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiShapeMember {
    pub subject_ref: String,
    pub position: u8,
    pub face: String,
}

/// Serializable structural view of the existing `ql_core::ShapeBinding` seam.
///
/// Semantic relation bindings are not duplicated here: Wiki already carries
/// those as attributable relations. This view preserves the identity, basis,
/// derivation, operator and Return information required to reopen a QL shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WikiShapeBindingView {
    pub subject_ref: String,
    pub shape_ref: String,
    pub whole_ref: String,
    #[serde(default)]
    pub basis_refs: Vec<String>,
    #[serde(default)]
    pub members: Vec<WikiShapeMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_ref: Option<String>,
    #[serde(default)]
    pub return_refs: Vec<String>,
    pub provenance: WikiShapeBindingProvenance,
}

impl WikiShapeBindingView {
    pub fn from_core(binding: &ShapeBinding) -> Self {
        Self {
            subject_ref: binding.subject_ref.clone(),
            shape_ref: binding.shape_ref.clone(),
            whole_ref: binding.whole_ref.clone(),
            basis_refs: binding.basis_refs.clone(),
            members: binding
                .members
                .iter()
                .map(|member| WikiShapeMember {
                    subject_ref: member.subject_ref.clone(),
                    position: member.coordinate.position.value(),
                    face: member.coordinate.face.as_str().into(),
                })
                .collect(),
            derivation_ref: binding.derivation_ref.clone(),
            operator_ref: binding.operator_ref.clone(),
            return_refs: binding.return_refs.clone(),
            provenance: WikiShapeBindingProvenance {
                caller_ref: binding.provenance.caller_ref.clone(),
                source_ref: binding.provenance.source_ref.clone(),
                standing_ref: binding.provenance.standing_ref.clone(),
            },
        }
    }

    pub fn shape(&self) -> Result<QlShape, WikiRefractionError> {
        resolve_shape_ref(&self.shape_ref).ok_or_else(|| {
            invalid_shape(format!("unknown or non-canonical QL shape ref {}", self.shape_ref))
        })
    }

    pub fn to_core(&self) -> Result<ShapeBinding, WikiRefractionError> {
        let members = self.core_members()?;
        let provenance = CallerProvenance::new(
            &self.provenance.caller_ref,
            &self.provenance.source_ref,
            &self.provenance.standing_ref,
        )
        .map_err(|error| invalid_shape(error.to_string()))?;
        ShapeBinding::new(
            &self.subject_ref,
            &self.shape_ref,
            &self.whole_ref,
            self.basis_refs.clone(),
            members,
            Vec::new(),
            self.derivation_ref.clone(),
            self.operator_ref.clone(),
            self.return_refs.clone(),
            provenance,
        )
        .map_err(|error| invalid_shape(error.to_string()))
    }

    pub fn validate_for_target(
        &self,
        target: &WikiRefractionTarget,
    ) -> Result<(), WikiRefractionError> {
        if self.subject_ref != target.target_ref {
            return Err(invalid_shape(
                "shape-binding subject must equal the stable Wiki target ref",
            ));
        }
        let presented = self.shape()?;
        let members = self.core_members()?;
        for member in &members {
            if !self.basis_refs.contains(&member.subject_ref) {
                return Err(invalid_shape(format!(
                    "basis_refs does not retain member {}",
                    member.subject_ref
                )));
            }
        }
        self.to_core()?;

        let form = StructuralConstellation::new(&self.whole_ref, members, Vec::new())
            .map_err(|error| invalid_shape(error.to_string()))?;
        let compression_operator = self.operator_ref.as_deref().is_some_and(|operator| {
            operator == THREE_TO_ONE_OPERATOR_REF || operator == COMPRESS_TO_WHOLE_OPERATOR_REF
        });

        if presented == QlShape::onefold() && !form.members.is_empty() {
            if !compression_operator {
                return Err(invalid_shape(
                    "one-fold presentation with disclosed members requires the canonical compression operator",
                ));
            }
            let disclosed = QlShape::Constellation(form.grain());
            let compression = QlShapeCompression::to_onefold(disclosed).ok_or_else(|| {
                invalid_shape("one-fold compression requires an actual disclosed two- or three-fold basis")
            })?;
            if self.operator_ref.as_deref() != Some(compression.operator_ref)
                || self.derivation_ref.as_deref() != Some(compression.derivation_ref().as_str())
            {
                return Err(invalid_shape(
                    "compression operator/derivation does not match the disclosed basis",
                ));
            }
            return Ok(());
        }

        if compression_operator {
            return Err(invalid_shape(
                "compression operator is valid only for a presented one-fold with disclosed members",
            ));
        }

        if let QlShape::Constellation(expected) = presented {
            if form.grain() != expected {
                return Err(invalid_shape(format!(
                    "shape ref names {expected:?} but members disclose {:?}",
                    form.grain()
                )));
            }
        }
        Ok(())
    }

    fn core_members(&self) -> Result<Vec<StructuralParticipation>, WikiRefractionError> {
        self.members
            .iter()
            .map(|member| {
                let position = ql_core::QlPosition::new(member.position).map_err(|_| {
                    invalid_shape(format!("shape member position {} outside 0..5", member.position))
                })?;
                let face = match member.face.as_str() {
                    "direct" => QlFace::Direct,
                    "conjugate" => QlFace::Conjugate,
                    other => return Err(invalid_shape(format!("unknown shape member face {other}"))),
                };
                StructuralParticipation::new(&member.subject_ref, position, face)
                    .map_err(|error| invalid_shape(error.to_string()))
            })
            .collect()
    }
}

/// Attach one already validated QL shape view to the target without changing
/// the public Wiki target schema. Existing clients that do not opt into
/// Geometry continue to serialize exactly as before.
pub fn attach_shape_binding(
    target: &mut WikiRefractionTarget,
    binding: &WikiShapeBindingView,
) -> Result<(), WikiRefractionError> {
    binding.validate_for_target(target)?;
    let value = serde_json::to_value(binding)
        .map_err(|error| invalid_shape(format!("cannot serialize shape binding: {error}")))?;
    target
        .extensions
        .insert(WIKI_SHAPE_BINDING_EXTENSION.into(), value);
    Ok(())
}

pub fn shape_binding_from_target(
    target: &WikiRefractionTarget,
) -> Result<Option<WikiShapeBindingView>, WikiRefractionError> {
    let Some(value) = target.extensions.get(WIKI_SHAPE_BINDING_EXTENSION) else {
        return Ok(None);
    };
    let binding: WikiShapeBindingView = serde_json::from_value(value.clone())
        .map_err(|error| invalid_shape(format!("invalid shape-binding extension: {error}")))?;
    binding.validate_for_target(target)?;
    Ok(Some(binding))
}

/// Geometry-aware façade over the existing Wiki refraction engine.
///
/// The underlying refraction semantics are unchanged. This façade validates the
/// optional QL-owned shape extension before execution, then copies that exact
/// structural view into every returned reading and surfaces its canonical shape
/// and operator refs through the reading's existing QL ref arrays.
pub struct ShapeAwareWikiRefractionEngine<'a> {
    inner: WikiRefractionEngine<'a>,
}

impl<'a> ShapeAwareWikiRefractionEngine<'a> {
    pub const fn new(provider: Option<&'a dyn QlProvider>) -> Self {
        Self {
            inner: WikiRefractionEngine::new(provider),
        }
    }

    pub fn refract(
        &self,
        request: &WikiRefractionRequest,
    ) -> Result<WikiRefractionResponse, WikiRefractionError> {
        let shape_binding = shape_binding_from_target(&request.target)?;
        let mut response = self.inner.refract(request)?;
        let Some(binding) = shape_binding else {
            return Ok(response);
        };
        let extension_value: Value = serde_json::to_value(&binding)
            .map_err(|error| invalid_shape(format!("cannot serialize shape binding: {error}")))?;
        for reading in &mut response.readings {
            if !reading.ql_form_refs.contains(&binding.shape_ref) {
                reading.ql_form_refs.push(binding.shape_ref.clone());
            }
            if let Some(operator_ref) = &binding.operator_ref {
                if !reading.operator_refs.contains(operator_ref) {
                    reading.operator_refs.push(operator_ref.clone());
                }
            }
            reading.extensions.insert(
                WIKI_SHAPE_BINDING_EXTENSION.into(),
                extension_value.clone(),
            );
        }
        Ok(response)
    }
}

fn invalid_shape(detail: impl Into<String>) -> WikiRefractionError {
    WikiRefractionError::InvalidStructuralField(format!("QL shape binding: {}", detail.into()))
}
