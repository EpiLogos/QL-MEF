use std::collections::BTreeMap;

use ql_core::{
    COMPRESS_TO_WHOLE_OPERATOR_REF, CallerProvenance, ConstellationGrain, QlFace, QlShape,
    QlShapeCompression, RELATIONAL_SIXFOLD_OPERATOR_REF, RelationalSixfold, ShapeBinding,
    StructuralConstellation, StructuralParticipation, THREE_TO_ONE_OPERATOR_REF, resolve_shape_ref,
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
            invalid_shape(format!(
                "unknown or non-canonical QL shape ref {}",
                self.shape_ref
            ))
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
        self.validate_view()
    }

    /// The binding's own structural law, whatever target carries it: its shape resolves, its basis retains every member, and the
    /// members disclose the shape it names (or the one-fold compression of their disclosed basis).
    pub fn validate_view(&self) -> Result<(), WikiRefractionError> {
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
                invalid_shape(
                    "one-fold compression requires an actual disclosed two- or three-fold basis",
                )
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
                    invalid_shape(format!(
                        "shape member position {} outside 0..5",
                        member.position
                    ))
                })?;
                let face = match member.face.as_str() {
                    "direct" => QlFace::Direct,
                    "conjugate" => QlFace::Conjugate,
                    other => {
                        return Err(invalid_shape(format!("unknown shape member face {other}")));
                    }
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
            reading
                .extensions
                .insert(WIKI_SHAPE_BINDING_EXTENSION.into(), extension_value.clone());
        }
        Ok(response)
    }
}

/// The version of [`ShapeBindReading`], the answer of `ql shape bind`.
pub const SHAPE_BIND_VERSION: &str = "ql.shape-bind/v1";

/// A constellation a caller drafts, asking QL for its shape binding: the subject the binding is of (a Wiki frame), the whole it
/// gathers round and each member on its position and face. QL reads only these: it reads no Wiki and keeps no store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShapeBindRequest {
    pub subject_ref: String,
    pub whole_ref: String,
    #[serde(default)]
    pub members: Vec<WikiShapeMember>,
    pub provenance: WikiShapeBindingProvenance,
}

/// One of the kernel's same-position generation sites the members fill on both faces: the 6 / 6′ -> 6+6′ operation at one
/// position, over the direct and conjugate member standing there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeGenerationSite {
    pub position: u8,
    pub direct_ref: String,
    pub conjugate_ref: String,
    /// The kernel's operator at this position (`QlGenerationSite::operator_ref`).
    pub operator_ref: String,
    /// The generated relation's identity in this whole: the operator `:of:` the whole. Structural identity only; what the relation
    /// says is an attributable return, never QL's to supply.
    pub generated_ref: String,
}

/// What a binding states and what it never does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeBindConstraints {
    pub creates_members: bool,
    pub asserts_relations: bool,
    pub semantic_generation_requires_attributable_return: bool,
}

/// QL's shape binding of a drafted constellation: the canonical shape its members disclose, the binding view a Wiki keeps for it
/// (under [`WIKI_SHAPE_BINDING_EXTENSION`]) and the identity of each generated relation, by position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeBindReading {
    pub schema: String,
    pub owner: String,
    pub grain: String,
    pub shape_ref: String,
    pub fold_count: Option<u8>,
    pub binding_extension: String,
    pub binding: WikiShapeBindingView,
    pub generation_operator_ref: String,
    pub generation_sites: Vec<ShapeGenerationSite>,
    /// Position (`"0"`..`"5"`) -> the generated relation's identity, for every site both faces fill.
    pub generated: BTreeMap<String, String>,
    pub constraints: ShapeBindConstraints,
}

/// Bind a drafted constellation to its canonical QL shape. The structural contract's refusals (a duplicate coordinate, a conjugate
/// with no direct, more than twelve members) come back in its words; a whole standing as its own member, and members that disclose
/// no canonical constellation, are refused too. Pure: creates no member and asserts no relation.
pub fn bind_shape(request: &ShapeBindRequest) -> Result<ShapeBindReading, WikiRefractionError> {
    if request
        .members
        .iter()
        .any(|member| member.subject_ref == request.whole_ref)
    {
        return Err(invalid_shape(
            "the whole is the 0/1 anchor its members gather round, never a member of its own constellation",
        ));
    }
    let draft = WikiShapeBindingView {
        subject_ref: request.subject_ref.clone(),
        shape_ref: String::new(),
        whole_ref: request.whole_ref.clone(),
        basis_refs: request
            .members
            .iter()
            .map(|member| member.subject_ref.clone())
            .collect(),
        members: request.members.clone(),
        derivation_ref: None,
        operator_ref: None,
        return_refs: Vec::new(),
        provenance: request.provenance.clone(),
    };
    let members = draft.core_members()?;
    let form = StructuralConstellation::new(&request.whole_ref, members, Vec::new())
        .map_err(|error| invalid_shape(error.to_string()))?;
    let grain = form.grain();
    if let ConstellationGrain::Other { .. } = grain {
        return Err(invalid_shape(format!(
            "the members disclose no canonical QL constellation (direct positions {:?}, conjugate positions {:?})",
            form.direct_positions(),
            form.conjugate_positions()
        )));
    }
    let shape = QlShape::Constellation(grain);
    let binding = WikiShapeBindingView {
        shape_ref: shape.shape_ref(),
        ..draft
    };
    binding.validate_view()?;

    let mut generation_sites = Vec::new();
    for site in RelationalSixfold::canonical().sites {
        let at = |face: QlFace| {
            form.members
                .iter()
                .find(|member| {
                    member.coordinate.position == site.position && member.coordinate.face == face
                })
                .map(|member| member.subject_ref.clone())
        };
        let (Some(direct_ref), Some(conjugate_ref)) = (at(QlFace::Direct), at(QlFace::Conjugate))
        else {
            continue;
        };
        let operator_ref = site.operator_ref();
        generation_sites.push(ShapeGenerationSite {
            position: site.position.value(),
            direct_ref,
            conjugate_ref,
            generated_ref: format!("{operator_ref}:of:{}", request.whole_ref),
            operator_ref,
        });
    }
    let generated = generation_sites
        .iter()
        .map(|site| (site.position.to_string(), site.generated_ref.clone()))
        .collect();
    Ok(ShapeBindReading {
        schema: SHAPE_BIND_VERSION.into(),
        owner: "ql-core".into(),
        grain: grain.as_str().into(),
        shape_ref: binding.shape_ref.clone(),
        fold_count: shape.fold_count(),
        binding_extension: WIKI_SHAPE_BINDING_EXTENSION.into(),
        binding,
        generation_operator_ref: RELATIONAL_SIXFOLD_OPERATOR_REF.into(),
        generation_sites,
        generated,
        constraints: ShapeBindConstraints {
            creates_members: false,
            asserts_relations: false,
            semantic_generation_requires_attributable_return: true,
        },
    })
}

fn invalid_shape(detail: impl Into<String>) -> WikiRefractionError {
    WikiRefractionError::InvalidStructuralField(format!("QL shape binding: {}", detail.into()))
}
