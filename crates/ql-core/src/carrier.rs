use core::fmt;

use crate::{
    AnchorReturn, ConstellationGrain, QlCoordinate, QlFace, QlShape, QlShapeAddress,
    StructuralConstellation, StructuralParticipation, WHOLE_ANCHOR_SYMBOL,
};

/// Version of the language-neutral structural-carrier seam layered over the
/// existing QL shape and structural contracts.
pub const STRUCTURAL_CARRIER_CONTRACT_VERSION: &str = "1.0.0";

/// QL-owned operation for composing two already-disclosed whole axes into an
/// address field. The operation creates addresses only; semantic relations are
/// caller-attributable bindings and are never manufactured here.
pub const RELATION_FIELD_COMPOSITION_OPERATOR_REF: &str =
    "ql:carrier:1.0.0:relation-field:cartesian-addresses";

/// Consumer-facing name for the existing QL-owned structural definition.
///
/// This is deliberately an alias, not a second shape ontology: callers bind
/// external objects to the canonical `QlShape` grammar.
pub type ShapeDefinition = QlShape;

/// Consumer-facing name for an existing structural participation.
///
/// The alias makes the binding role explicit without copying the QL coordinate
/// or participation model into another carrier type.
pub type ShapeMemberBinding = StructuralParticipation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarrierError {
    EmptyRef(&'static str),
    EmptyAxis(&'static str),
    UndisclosedAxisShape {
        axis: &'static str,
        grain: ConstellationGrain,
    },
    MissingEvidence,
    BindingShapeMismatch {
        binding_shape_ref: String,
        field_shape_ref: String,
    },
    AddressOutsideField(QlShapeAddress),
}

impl fmt::Display for CarrierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRef(field) => write!(f, "{field} must be a non-empty stable ref"),
            Self::EmptyAxis(axis) => write!(
                f,
                "{axis} whole has no positional members and cannot form a relation axis"
            ),
            Self::UndisclosedAxisShape { axis, grain } => write!(
                f,
                "{axis} whole grain {grain:?} is not a disclosed positive carrier shape"
            ),
            Self::MissingEvidence => {
                f.write_str("an attributable semantic relation requires at least one evidence ref")
            }
            Self::BindingShapeMismatch {
                binding_shape_ref,
                field_shape_ref,
            } => write!(
                f,
                "binding shape {binding_shape_ref} does not match relation field {field_shape_ref}"
            ),
            Self::AddressOutsideField(address) => write!(
                f,
                "relation binding address {}/{} x {}/{} is outside the disclosed field",
                address.row.position.value(),
                address.row.face,
                address.column.position.value(),
                address.column.face
            ),
        }
    }
}

impl std::error::Error for CarrierError {}

fn require_ref(value: &str, field: &'static str) -> Result<(), CarrierError> {
    if value.trim().is_empty() {
        return Err(CarrierError::EmptyRef(field));
    }
    Ok(())
}

fn validate_refs(values: &[String], field: &'static str) -> Result<(), CarrierError> {
    for value in values {
        require_ref(value, field)?;
    }
    Ok(())
}

fn is_disclosed_carrier_grain(grain: ConstellationGrain) -> bool {
    !matches!(grain, ConstellationGrain::Other { .. })
}

/// One already-disclosed QL whole made usable as an axis without losing the
/// caller/source identities that disclosed it.
///
/// Member order is canonicalised by face then position so relation addresses
/// are deterministic even when external bindings arrive in a different order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QlWholeAxis {
    pub whole_ref: String,
    pub shape: ShapeDefinition,
    pub grain: ConstellationGrain,
    pub members: Vec<ShapeMemberBinding>,
    pub returns: Vec<AnchorReturn>,
}

impl QlWholeAxis {
    pub fn from_constellation(constellation: &StructuralConstellation) -> Self {
        let grain = constellation.grain();
        let mut members = constellation.members.clone();
        members.sort_by_key(|member| {
            (
                match member.coordinate.face {
                    QlFace::Direct => 0_u8,
                    QlFace::Conjugate => 1_u8,
                },
                member.coordinate.position.value(),
            )
        });
        Self {
            whole_ref: constellation.anchor_ref.clone(),
            shape: QlShape::Constellation(grain),
            grain,
            members,
            returns: constellation.returns.clone(),
        }
    }

    pub fn shape_ref(&self) -> String {
        self.shape.shape_ref()
    }

    pub fn coordinates(&self) -> Vec<QlCoordinate> {
        self.members
            .iter()
            .map(|member| member.coordinate)
            .collect()
    }

    pub fn basis_refs(&self) -> Vec<String> {
        self.members
            .iter()
            .map(|member| member.subject_ref.clone())
            .collect()
    }

    pub fn return_operator_refs(&self) -> Vec<String> {
        self.returns
            .iter()
            .map(AnchorReturn::operator_ref)
            .collect()
    }
}

/// Provenance of a whole×whole address-field derivation.
///
/// The generated field keeps both source wholes, their actual QL forms, grains,
/// operator and Return basis. It does not flatten them into anonymous matrix
/// dimensions or Return operator labels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationFieldDerivation {
    pub source_whole_refs: [String; 2],
    pub source_shape_refs: [String; 2],
    pub source_grains: [ConstellationGrain; 2],
    pub operator_ref: &'static str,
    pub generated_shape_ref: String,
    pub return_basis: &'static str,
    pub source_return_refs: Vec<String>,
    pub source_returns: Vec<AnchorReturn>,
}

/// Generic composition of two legitimate disclosed QL whole axes.
///
/// Cardinality follows from the actual axes. There is intentionally no
/// constructor from `(rows, columns)`: dimensions are observations of a field,
/// never authority for declaring a QL shape. A structurally valid but presently
/// undisclosed `Other` grain is therefore not silently promoted into a carrier
/// axis merely because its member count resembles a familiar matrix dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationFieldComposition {
    pub row_axis: QlWholeAxis,
    pub column_axis: QlWholeAxis,
    pub addresses: Vec<QlShapeAddress>,
}

impl RelationFieldComposition {
    pub fn compose(
        row_whole: &StructuralConstellation,
        column_whole: &StructuralConstellation,
    ) -> Result<Self, CarrierError> {
        let row_axis = QlWholeAxis::from_constellation(row_whole);
        let column_axis = QlWholeAxis::from_constellation(column_whole);
        if row_axis.members.is_empty() {
            return Err(CarrierError::EmptyAxis("row"));
        }
        if column_axis.members.is_empty() {
            return Err(CarrierError::EmptyAxis("column"));
        }
        if !is_disclosed_carrier_grain(row_axis.grain) {
            return Err(CarrierError::UndisclosedAxisShape {
                axis: "row",
                grain: row_axis.grain,
            });
        }
        if !is_disclosed_carrier_grain(column_axis.grain) {
            return Err(CarrierError::UndisclosedAxisShape {
                axis: "column",
                grain: column_axis.grain,
            });
        }

        let rows = row_axis.coordinates();
        let columns = column_axis.coordinates();
        let mut addresses = Vec::with_capacity(rows.len() * columns.len());
        for row in &rows {
            for column in &columns {
                addresses.push(QlShapeAddress {
                    row: *row,
                    column: *column,
                });
            }
        }

        Ok(Self {
            row_axis,
            column_axis,
            addresses,
        })
    }

    pub fn shape_ref(&self) -> String {
        format!(
            "ql:carrier:{STRUCTURAL_CARRIER_CONTRACT_VERSION}:relation-field:{}:by:{}",
            self.row_axis.shape_ref(),
            self.column_axis.shape_ref()
        )
    }

    pub const fn operator_ref(&self) -> &'static str {
        RELATION_FIELD_COMPOSITION_OPERATOR_REF
    }

    pub fn cardinality(&self) -> (usize, usize, usize) {
        (
            self.row_axis.members.len(),
            self.column_axis.members.len(),
            self.addresses.len(),
        )
    }

    pub fn contains_address(&self, address: QlShapeAddress) -> bool {
        self.addresses.contains(&address)
    }

    pub fn derivation(&self) -> RelationFieldDerivation {
        let mut source_return_refs = self.row_axis.return_operator_refs();
        source_return_refs.extend(self.column_axis.return_operator_refs());
        let mut source_returns = self.row_axis.returns.clone();
        source_returns.extend(self.column_axis.returns.clone());
        RelationFieldDerivation {
            source_whole_refs: [
                self.row_axis.whole_ref.clone(),
                self.column_axis.whole_ref.clone(),
            ],
            source_shape_refs: [self.row_axis.shape_ref(), self.column_axis.shape_ref()],
            source_grains: [self.row_axis.grain, self.column_axis.grain],
            operator_ref: self.operator_ref(),
            generated_shape_ref: self.shape_ref(),
            return_basis: WHOLE_ANCHOR_SYMBOL,
            source_return_refs,
            source_returns,
        }
    }
}

/// Caller/source attribution for a structural binding. `standing_ref` is kept
/// opaque because authority and epistemic standing belong to the caller's
/// world, not to QL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallerProvenance {
    pub caller_ref: String,
    pub source_ref: String,
    pub standing_ref: String,
}

impl CallerProvenance {
    pub fn new(
        caller_ref: impl Into<String>,
        source_ref: impl Into<String>,
        standing_ref: impl Into<String>,
    ) -> Result<Self, CarrierError> {
        let value = Self {
            caller_ref: caller_ref.into(),
            source_ref: source_ref.into(),
            standing_ref: standing_ref.into(),
        };
        require_ref(&value.caller_ref, "caller_ref")?;
        require_ref(&value.source_ref, "source_ref")?;
        require_ref(&value.standing_ref, "standing_ref")?;
        Ok(value)
    }
}

/// One externally supplied semantic determination at a QL address.
///
/// Multiple bindings may occupy the same address and reverse/asymmetric
/// addresses remain distinct. QL requires attribution and evidence refs but
/// does not decide whether the caller's semantic claim is true.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeRelationBinding {
    pub address: QlShapeAddress,
    pub relation_ref: String,
    pub evidence_refs: Vec<String>,
}

impl ShapeRelationBinding {
    pub fn new(
        address: QlShapeAddress,
        relation_ref: impl Into<String>,
        evidence_refs: Vec<String>,
    ) -> Result<Self, CarrierError> {
        let relation_ref = relation_ref.into();
        require_ref(&relation_ref, "relation_ref")?;
        if evidence_refs.is_empty() {
            return Err(CarrierError::MissingEvidence);
        }
        validate_refs(&evidence_refs, "evidence_ref")?;
        Ok(Self {
            address,
            relation_ref,
            evidence_refs,
        })
    }
}

/// External, attributable mapping of a real caller-owned subject into a QL
/// ShapeDefinition or relation field.
///
/// The binding carries identities and provenance; it does not promote caller
/// content into QL-owned semantics or runtime authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeBinding {
    pub subject_ref: String,
    pub shape_ref: String,
    pub whole_ref: String,
    pub basis_refs: Vec<String>,
    pub members: Vec<ShapeMemberBinding>,
    pub relation_bindings: Vec<ShapeRelationBinding>,
    pub derivation_ref: Option<String>,
    pub operator_ref: Option<String>,
    pub return_refs: Vec<String>,
    pub provenance: CallerProvenance,
}

impl ShapeBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject_ref: impl Into<String>,
        shape_ref: impl Into<String>,
        whole_ref: impl Into<String>,
        basis_refs: Vec<String>,
        members: Vec<ShapeMemberBinding>,
        relation_bindings: Vec<ShapeRelationBinding>,
        derivation_ref: Option<String>,
        operator_ref: Option<String>,
        return_refs: Vec<String>,
        provenance: CallerProvenance,
    ) -> Result<Self, CarrierError> {
        let value = Self {
            subject_ref: subject_ref.into(),
            shape_ref: shape_ref.into(),
            whole_ref: whole_ref.into(),
            basis_refs,
            members,
            relation_bindings,
            derivation_ref,
            operator_ref,
            return_refs,
            provenance,
        };
        require_ref(&value.subject_ref, "subject_ref")?;
        require_ref(&value.shape_ref, "shape_ref")?;
        require_ref(&value.whole_ref, "whole_ref")?;
        validate_refs(&value.basis_refs, "basis_ref")?;
        validate_refs(&value.return_refs, "return_ref")?;
        if let Some(reference) = &value.derivation_ref {
            require_ref(reference, "derivation_ref")?;
        }
        if let Some(reference) = &value.operator_ref {
            require_ref(reference, "operator_ref")?;
        }
        Ok(value)
    }

    pub fn member_bindings(&self) -> &[ShapeMemberBinding] {
        &self.members
    }

    pub const fn caller_provenance(&self) -> &CallerProvenance {
        &self.provenance
    }

    /// Validate relation bindings against a disclosed field without requiring
    /// every address to be filled. Missing, plural and asymmetric determinations
    /// therefore remain representable.
    pub fn validate_relation_field(
        &self,
        field: &RelationFieldComposition,
    ) -> Result<(), CarrierError> {
        let field_shape_ref = field.shape_ref();
        if self.shape_ref != field_shape_ref {
            return Err(CarrierError::BindingShapeMismatch {
                binding_shape_ref: self.shape_ref.clone(),
                field_shape_ref,
            });
        }
        for relation in &self.relation_bindings {
            if !field.contains_address(relation.address) {
                return Err(CarrierError::AddressOutsideField(relation.address));
            }
        }
        Ok(())
    }
}
