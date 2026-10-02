//! Metric source constructor for the native M3 three-site fold grammar.
//! The native owner supplies codon, pair, site, matrix-axis and ranked pose.
//! This module owns an explicit mechanical realisation: three quadrilateral
//! elastic frames joined by crossed struts. It does not treat a glyph, the
//! normalized presentation hinge or an orientation seed as a physical mesh.
use crate::m3_engine::{M3NodeKind, native_m3_engine};
use crate::m3_state::M3State;
use crate::physical_body::*;
use crate::{MCoordinate, MFace};
use ql_core::{ElementalQuaternionBasis, MatrixAxis, Mobility, generate_rotational_states};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const SOURCE_GEOMETRY_CONTRACT: &str = "ql.m3-source-frame-geometry/v1";
pub const SOURCE_GEOMETRY_BASIS: &str =
    "docs/origami work/INTEGRATED-1-2-3-PHYSICAL-POLE-OBJECT.md#5-mahamaya-foldrupa-grammar";
/// Geometric/material magnitudes are supplied, tunable and bounded, rather
/// than inferred from coin values or assigned to an element as physical truth.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceGeometryRecipe {
    pub provenance: PhysicalProvenance,
    pub family: BodyFamily,
    pub frame_side_metres: f64,
    pub site_separation_metres: f64,
    /// Sections in the existing ratified Earth/Fire/Water/Air carrier order.
    pub section_by_element_m2: [f64; 4],
    pub intersite_section_m2: f64,
    pub prestress_newtons: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBodyControls {
    pub expected_m3_generation: u64,
    pub body_revision: u64,
    pub preparation_ref: String,
    pub state_ref: String,
    pub material: PhysicalMaterial,
    pub sample_rate: u32,
    pub exciter: SpatialProjection,
    pub pickup: SpatialProjection,
    pub pickup_linear_per_metre: f64,
    pub max_force_newtons: f64,
    pub max_impulse_newton_seconds: f64,
    pub max_displacement_metres: f64,
}
#[derive(Debug, Clone, Serialize)]
pub struct PreparedSourceFormBody {
    contract: &'static str,
    recipe: SourceGeometryRecipe,
    body: PreparedSourceBody,
    source_reading: Value,
    mechanical_law: Value,
}
impl PreparedSourceFormBody {
    pub fn body(&self) -> &PreparedSourceBody {
        &self.body
    }
    pub fn recipe(&self) -> &SourceGeometryRecipe {
        &self.recipe
    }
    pub fn source_reading(&self) -> &Value {
        &self.source_reading
    }
    /// Recompute through the actual current owner. Full metric topology,
    /// constraints and declared law must match, not merely a constituent label.
    pub fn validate_source_geometry(&self, state: &M3State) -> Result<(), String> {
        let geometry = source_form_geometry(state, &self.recipe)?;
        if geometry != self.body.request().geometry {
            return Err("metric body differs from actual native fold geometry".into());
        }
        Ok(())
    }
}
fn reference(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        Err("bounded printable source geometry reference required".into())
    } else {
        Ok(())
    }
}
fn finite(value: f64, lo: f64, hi: f64, name: &str) -> Result<(), String> {
    if !value.is_finite() || value < lo || value > hi {
        Err(format!("invalid source geometry {name}"))
    } else {
        Ok(())
    }
}
fn validate_recipe(recipe: &SourceGeometryRecipe) -> Result<(), String> {
    for value in [
        &recipe.provenance.reference,
        &recipe.provenance.revision,
        &recipe.provenance.source_ref,
    ] {
        reference(value)?;
    }
    // The source specifies the fold grammar; it explicitly leaves the metric
    // mesh/solver open. A mechanical model cannot claim SourceAuthored merely
    // because it carries that grammar's source reference.
    if recipe.provenance.standing == PhysicalStanding::SourceAuthored {
        return Err("metric frame realisation needs its own declared mechanical standing".into());
    }
    finite(recipe.frame_side_metres, 1e-4, 100.0, "frame side metres")?;
    finite(
        recipe.site_separation_metres,
        2.0 * recipe.frame_side_metres,
        100.0,
        "site separation metres",
    )?;
    for section in recipe.section_by_element_m2 {
        finite(section, 1e-12, 1.0, "elemental frame section m2")?;
    }
    finite(
        recipe.intersite_section_m2,
        1e-12,
        1.0,
        "intersite section m2",
    )?;
    finite(recipe.prestress_newtons, 0.0, 1e9, "prestress Newtons")?;
    if recipe.family == BodyFamily::AxialTruss && recipe.prestress_newtons != 0.0 {
        return Err("source axial frames cannot silently use prestressed network law".into());
    }
    if recipe.family == BodyFamily::PrestressedTensionNetwork && recipe.prestress_newtons == 0.0 {
        return Err("source tension network needs positive declared prestress".into());
    }
    Ok(())
}
fn rotate(point: [f64; 3], axis: usize, angle: f64) -> [f64; 3] {
    let mut result = point;
    let a = (axis + 1) % 3;
    let b = (axis + 2) % 3;
    let (s, c) = angle.sin_cos();
    result[a] = c * point[a] - s * point[b];
    result[b] = s * point[a] + c * point[b];
    result
}
/// Build a bounded physical graph from the existing source operations. Twelve
/// persistent node IDs identify the same material sites across form changes.
/// Polarity sets the signed site dihedral; native pair16 gives its orientation.
/// Resting sites constrain their last corner's Z displacement; moving sites
/// release it. The first two corners anchor the body. Each quadrilateral has
/// six axial members; neighbouring frames have four longitudinal and four
/// crossed members. This is an elastic frame law, not a cloth/plate claim.
pub fn source_form_geometry(
    state: &M3State,
    recipe: &SourceGeometryRecipe,
) -> Result<MetricGeometry, String> {
    validate_recipe(recipe)?;
    let fold = state.fold();
    let codon = fold.codon();
    let engine = native_m3_engine();
    let target = engine
        .node(M3NodeKind::Codon, usize::from(codon.address()))
        .ok_or("missing exact source form")?;
    let angles = [
        fold.geometry().pair_angle_xy().0,
        0,
        fold.geometry().pair_angle_yz().0,
    ];
    let ranked = generate_rotational_states(codon);
    let pose = ranked[usize::from(fold.rotational_index())];
    if pose.rotation_slot != fold.rotational_index() {
        return Err("native pose/rank disagreement".into());
    }
    let pose_angle = f64::from(pose.rotation_degrees).to_radians();
    let pose_axis = match fold.active_matrix_axis() {
        MatrixAxis::I => 0,
        MatrixAxis::J => 1,
        MatrixAxis::K => 2,
    };
    let basis = ElementalQuaternionBasis::canonical();
    let half = recipe.frame_side_metres / 2.0;
    let corners = [
        [-half, -half, 0.0],
        [half, -half, 0.0],
        [half, half, 0.0],
        [-half, half, 0.0],
    ];
    let mut nodes = Vec::with_capacity(12);
    let mut edges = Vec::with_capacity(34);
    for (site, nucleotide) in fold.nucleotides().into_iter().enumerate() {
        let native = engine
            .node(M3NodeKind::Nucleotide, usize::from(nucleotide.bits()))
            .ok_or("missing exact nucleotide constituent")?;
        let signed = f64::from(fold.sites()[site].signed_angle) / 10.0;
        let dihedral = signed.to_radians();
        let pair = (f64::from(angles[site]) / 10.0).to_radians();
        for (corner, local) in corners.into_iter().enumerate() {
            let oriented = rotate(rotate(local, 0, dihedral), 2, pair);
            let shifted = [
                oriented[0] + (site as f64 - 1.0) * recipe.site_separation_metres,
                oriented[1],
                oriented[2],
            ];
            let anchored = site == 0 && corner < 2;
            nodes.push(PhysicalNode {
                identity: (4 * site + corner + 1) as u64,
                constituent: if site == 0 && corner == 0 {
                    target.source_ref.clone()
                } else {
                    native.source_ref.clone()
                },
                rest_metres: rotate(shifted, pose_axis, pose_angle),
                additional_mass_kg: 0.0,
                fixed: if anchored {
                    [true; 3]
                } else {
                    [
                        false,
                        false,
                        corner == 3 && nucleotide.mobility() == Mobility::Resting,
                    ]
                },
            });
        }
        let section = recipe.section_by_element_m2[basis.element_of(nucleotide).component_index()];
        for first in 0..4 {
            for second in first + 1..4 {
                edges.push(PhysicalEdge {
                    first: 4 * site + first,
                    second: 4 * site + second,
                    section_m2: section,
                    prestress_newtons: recipe.prestress_newtons,
                });
            }
        }
        if site > 0 {
            for corner in 0..4 {
                for previous in [corner, (corner + 1) % 4] {
                    edges.push(PhysicalEdge {
                        first: 4 * (site - 1) + previous,
                        second: 4 * site + corner,
                        section_m2: recipe.intersite_section_m2,
                        prestress_newtons: recipe.prestress_newtons,
                    });
                }
            }
        }
    }
    Ok(MetricGeometry {
        provenance: recipe.provenance.clone(),
        family: recipe.family,
        nodes,
        edges,
    })
}
fn request(
    state: &M3State,
    recipe: &SourceGeometryRecipe,
    controls: SourceBodyControls,
) -> Result<BodyPreparationRequest, String> {
    Ok(BodyPreparationRequest {
        expected_m3_generation: controls.expected_m3_generation,
        body_revision: controls.body_revision,
        preparation_ref: controls.preparation_ref,
        state_ref: controls.state_ref,
        geometry: source_form_geometry(state, recipe)?,
        material: controls.material,
        sample_rate: controls.sample_rate,
        exciter: controls.exciter,
        pickup: controls.pickup,
        pickup_linear_per_metre: controls.pickup_linear_per_metre,
        max_force_newtons: controls.max_force_newtons,
        max_impulse_newton_seconds: controls.max_impulse_newton_seconds,
        max_displacement_metres: controls.max_displacement_metres,
    })
}
pub fn prepare_source_form_body(
    state: &M3State,
    coordinate: MCoordinate,
    recipe: SourceGeometryRecipe,
    controls: SourceBodyControls,
) -> Result<PreparedSourceFormBody, String> {
    let body = prepare_source_body(state, coordinate, request(state, &recipe, controls)?)?;
    Ok(PreparedSourceFormBody {
        contract: SOURCE_GEOMETRY_CONTRACT,
        recipe,
        body,
        source_reading: state.snapshot(),
        mechanical_law: json!({"source":SOURCE_GEOMETRY_BASIS,"nodes":12,"members":34,
            "site_dihedral":"native signed crease angle; radians converted from deg10",
            "pair_orientation":"native XY/YZ pair16; middle frame is the mediator",
            "pose":"actual native ranked candidate rotation_degrees about active matrix axis i/x,j/y,k/z",
            "mobility":"resting last corner fixed in Z; moving corner released",
            "ground":"first material edge anchored","aperture":"reading only; no metric or PCM mutation",
            "clock":"native reading only; no additional clock","units":{"position":"m","section":"m^2","prestress":"N"},
            "standing":"declared finite elastic-frame realisation of authored grammar; no empirical cloth/shell claim"}),
    })
}
/// Prove a supplied metric projection by regenerating from actual native form.
/// This is deliberately stronger than generic metric-provider admission.
pub fn admit_source_form_metric(
    state: &M3State,
    recipe: &SourceGeometryRecipe,
    geometry: &MetricGeometry,
) -> Result<(), String> {
    if source_form_geometry(state, recipe)? != *geometry {
        return Err("source form metric is stale, relabelled or disconnected".into());
    }
    Ok(())
}
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "effect", rename_all = "kebab-case")]
pub enum SourceFormUpdate {
    /// A view/readout changed while the physical determinants stayed fixed.
    /// Keep the resident body and its sample cursor; return the owner reading.
    ReadingOnly {
        preparation_ref: String,
        state_ref: String,
        body_revision: u64,
        source_reading: Value,
    },
    Physical {
        recipe: SourceGeometryRecipe,
        source_reading: Value,
        transition: PreparedFormTransition,
    },
}
pub fn prepare_source_form_update(
    current: &PreparedSourceFormBody,
    state: &M3State,
    coordinate: MCoordinate,
    recipe: SourceGeometryRecipe,
    controls: SourceBodyControls,
    expected_body_revision: u64,
    expected_sample: u64,
    policy: FormTransitionPolicy,
) -> Result<SourceFormUpdate, String> {
    if expected_body_revision != current.body.request().body_revision
        || controls.expected_m3_generation != state.generation()
        || state.generation() < current.body.source_generation()
    {
        return Err("stale source form update".into());
    }
    let next = request(state, &recipe, controls)?;
    let admitted = prepare_source_body(state, coordinate.clone(), next.clone())?;
    if admitted.event_ref() != current.body.event_ref()
        || admitted.subject_ref() != current.body.subject_ref()
        || next.state_ref != current.body.request().state_ref
    {
        return Err("source form update has disconnected event/state".into());
    }
    let old = current.body.request();
    if next.geometry == old.geometry
        && next.material == old.material
        && next.sample_rate == old.sample_rate
        && next.exciter == old.exciter
        && next.pickup == old.pickup
        && next.pickup_linear_per_metre == old.pickup_linear_per_metre
        && next.max_force_newtons == old.max_force_newtons
        && next.max_impulse_newton_seconds == old.max_impulse_newton_seconds
        && next.max_displacement_metres == old.max_displacement_metres
        && coordinate == *current.body.source_coordinate()
    {
        return Ok(SourceFormUpdate::ReadingOnly {
            preparation_ref: old.preparation_ref.clone(),
            state_ref: old.state_ref.clone(),
            body_revision: old.body_revision,
            source_reading: state.snapshot(),
        });
    }
    let transition = prepare_form_transition(
        &current.body,
        state,
        coordinate,
        next,
        expected_body_revision,
        expected_sample,
        policy,
    )?;
    Ok(SourceFormUpdate::Physical {
        recipe,
        source_reading: state.snapshot(),
        transition,
    })
}
/// Explicit source coordinate selection with separately preserved face. This
/// resolves the actual form node; it never admits a parent or phase alias.
pub fn source_form_coordinate(state: &M3State, face: MFace) -> Result<MCoordinate, String> {
    let node = native_m3_engine()
        .node(
            M3NodeKind::Codon,
            usize::from(state.fold().codon().address()),
        )
        .ok_or("missing native source form")?;
    crate::m_tree::native_current_m_registry().coordinate(&node.source_ref, face)
}
