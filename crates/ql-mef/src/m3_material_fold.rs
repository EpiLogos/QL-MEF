//! Source-qualified material deformation for the retained Expressions sampler.
//! This is an explicit four-panel square construction over the native three
//! signed sites. Pair-hinge presentation and quaternion seeds are never used as
//! crease dihedrals or ranked pose. It owns no clock, sampling or body solver.
use crate::m_tree::native_m_registry;
use crate::m3_source::native_m3_source;
use crate::m3_state::M3State;
use crate::physical_body::{PreparedSourceBody, readmit_source_coordinate, source_coordinate_wire};
use crate::source_form_body::{
    SourceGeometryRecipe, admit_source_form_metric, source_form_coordinate,
};
use ql_core::{MatrixAxis, generate_rotational_states};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const MATERIAL_FOLD_SCHEMA: &str = "ql.m3-material-fold/v1";
pub const MATERIAL_TOPOLOGY: &str = "ql.square-four-strip-crease-tree/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialFoldRequest {
    pub event_ref: String,
    pub subject_ref: String,
    pub expected_generation: u64,
    pub expected_source_revision: String,
    pub expected_domain_revision: String,
    pub expected_registry_revision: String,
    /// Complete native coordinate/face/provenance, not a rewritten prime label.
    pub source_coordinate: Value,
    pub recipe_ref: String,
    pub recipe_revision: String,
    pub material_treatment: String,
    /// Half-width of the square in local stage units. Entity placement follows.
    pub stage_units_per_material_unit: f64,
    pub metres_per_material_unit: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MaterialFoldPlan {
    pub schema: &'static str,
    pub topology_ref: &'static str,
    pub recipe_ref: String,
    pub recipe_revision: String,
    pub material_treatment: String,
    pub event_ref: String,
    pub subject_ref: String,
    pub source_coordinate: Value,
    pub source_generation: u64,
    pub source_revision: String,
    pub domain_revision: String,
    pub registry_revision: String,
    pub stage_units_per_material_unit: f64,
    pub metres_per_material_unit: f64,
    pub panels: Value,
    pub creases: Value,
    pub crease_angles_rad: [f64; 3],
    pub site_velocities_deg10: [i32; 3],
    pub pose_axis: [f64; 3],
    pub pose_angle_rad: f64,
    pub pose_projection: Value,
    /// Entire real owner packet: symbolic relations retain their own standing.
    pub native_state: Value,
    pub construction_standing: &'static str,
}

fn bounded(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        Err("bounded printable material fold reference required".into())
    } else {
        Ok(())
    }
}

/// Preparation runs on the control thread. Canonical sites are source-produced
/// signed telemetry; mobility remains a separate native reading. Aperture and
/// clock edits leave this geometry unchanged while retaining their new basis.
pub fn prepare_material_fold(
    state: &M3State,
    request: &MaterialFoldRequest,
) -> Result<MaterialFoldPlan, String> {
    let snapshot = state.snapshot();
    let source = native_m3_source();
    let registry = native_m_registry();
    for value in [
        &request.event_ref,
        &request.subject_ref,
        &request.recipe_ref,
        &request.recipe_revision,
    ] {
        bounded(value)?;
    }
    if Some(request.event_ref.as_str()) != snapshot["identity"]["event_ref"].as_str()
        || Some(request.subject_ref.as_str()) != snapshot["subject_ref"].as_str()
        || request.expected_generation != state.generation()
        || request.expected_source_revision != source.source_revision()
        || request.expected_domain_revision != source.revision()
        || request.expected_registry_revision != registry.manifest().registry_revision
    {
        return Err("material fold event/subject/generation/source mismatch".into());
    }
    if !matches!(request.material_treatment.as_str(), "glyph-mask" | "sheet") {
        return Err("unknown material treatment".into());
    }
    for scale in [
        request.stage_units_per_material_unit,
        request.metres_per_material_unit,
    ] {
        if !scale.is_finite() || !(1e-9..=1e9).contains(&scale) {
            return Err("material fold unit conversion must be finite and positive".into());
        }
    }
    let coordinate = readmit_source_coordinate(&request.source_coordinate)?;
    let actual = source_form_coordinate(state, coordinate.face)?;
    if source_coordinate_wire(&actual) != request.source_coordinate {
        return Err("material fold requires exact current native form and face".into());
    }
    let fold = state.fold();
    let ranked = generate_rotational_states(fold.codon());
    let pose = ranked[usize::from(fold.rotational_index())];
    if pose.rotation_slot != fold.rotational_index() {
        return Err("native material pose/rank mismatch".into());
    }
    let axis = match fold.active_matrix_axis() {
        MatrixAxis::I => [1., 0., 0.],
        MatrixAxis::J => [0., 1., 0.],
        MatrixAxis::K => [0., 0., 1.],
    };
    let cuts = [-1., -0.5, 0., 0.5, 1.];
    let panels: Vec<_> = (0..4).map(|i| json!({"id":format!("panel-{i}"),
        "bounds":[cuts[i],cuts[i+1],-1.,1.],"parent":if i==0 {None} else {Some(format!("panel-{}",i-1))},
        "crease":if i==0 {None} else {Some(i-1)}})).collect();
    let creases: Vec<_> = (0..3)
        .map(|i| {
            json!({"id":format!("site-{}",["X","Y","Z"][i]),
        "site_index":i,"axis_start":[cuts[i+1],-1.,0.],"axis_end":[cuts[i+1],1.,0.]})
        })
        .collect();
    Ok(MaterialFoldPlan {
        schema: MATERIAL_FOLD_SCHEMA,
        topology_ref: MATERIAL_TOPOLOGY,
        recipe_ref: request.recipe_ref.clone(),
        recipe_revision: request.recipe_revision.clone(),
        material_treatment: request.material_treatment.clone(),
        event_ref: request.event_ref.clone(),
        subject_ref: request.subject_ref.clone(),
        source_coordinate: request.source_coordinate.clone(),
        source_generation: state.generation(),
        source_revision: source.source_revision().into(),
        domain_revision: source.revision().into(),
        registry_revision: registry.manifest().registry_revision.clone(),
        stage_units_per_material_unit: request.stage_units_per_material_unit,
        metres_per_material_unit: request.metres_per_material_unit,
        panels: json!(panels),
        creases: json!(creases),
        crease_angles_rad: fold
            .sites()
            .map(|s| (f64::from(s.signed_angle) / 10.).to_radians()),
        site_velocities_deg10: fold.sites().map(|s| s.angular_velocity),
        pose_axis: axis,
        pose_angle_rad: f64::from(pose.rotation_degrees).to_radians(),
        pose_projection: json!({"rotation_slot":pose.rotation_slot,"rotation_degrees":pose.rotation_degrees,
            "slot_degrees":45,"pose_ordinal":fold.rotational_pose().ordinal()}),
        native_state: snapshot,
        construction_standing: "declared square crease-tree material realisation; no empirical cloth/physical-body claim",
    })
}

/// Consumer admission regenerates every determinant. A copied source label or
/// stale geometry cannot be accepted as current material construction.
pub fn admit_material_fold(
    state: &M3State,
    request: &MaterialFoldRequest,
    plan: &MaterialFoldPlan,
) -> Result<(), String> {
    if prepare_material_fold(state, request)? != *plan {
        return Err("material fold is stale, altered or disconnected".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialRestSample {
    pub id: String,
    pub source_ref: String,
    pub source_revision: String,
    pub layer_ref: String,
    pub rest_material: [f64; 3],
    pub density: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MaterialTargetSample {
    pub id: String,
    pub source_ref: String,
    pub source_revision: String,
    pub layer_ref: String,
    pub local_stage: [f64; 3],
    pub density: f64,
}
fn axis_rotation(point: [f64; 3], origin: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let p = std::array::from_fn::<_, 3, _>(|i| point[i] - origin[i]);
    let (s, c) = angle.sin_cos();
    let dot = axis.iter().zip(p).map(|(a, p)| a * p).sum::<f64>();
    let cross = [
        axis[1] * p[2] - axis[2] * p[1],
        axis[2] * p[0] - axis[0] * p[2],
        axis[0] * p[1] - axis[1] * p[0],
    ];
    std::array::from_fn(|i| origin[i] + p[i] * c + cross[i] * s + axis[i] * dot * (1. - c))
}
/// Native control-thread target production over the actual sampler's retained
/// material. A browser can consume these targets or independently verify them;
/// neither path owns another evolving physical variable or advances a clock.
pub fn material_fold_targets(
    state: &M3State,
    request: &MaterialFoldRequest,
    plan: &MaterialFoldPlan,
    samples: &[MaterialRestSample],
) -> Result<Vec<MaterialTargetSample>, String> {
    admit_material_fold(state, request, plan)?;
    if samples.is_empty() || samples.len() > 1_048_576 {
        return Err("material sample allocation outside budget".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    let mut targets = Vec::with_capacity(samples.len());
    let cuts = [-0.5, 0., 0.5];
    for sample in samples {
        for reference in [
            &sample.id,
            &sample.source_ref,
            &sample.source_revision,
            &sample.layer_ref,
        ] {
            bounded(reference)?;
        }
        if !ids.insert(sample.id.as_str())
            || sample.rest_material.iter().any(|v| !v.is_finite())
            || sample.rest_material[0].abs() > 1. + 1e-6
            || sample.rest_material[1].abs() > 1. + 1e-6
            || !sample.density.is_finite()
            || !(0.0..=1.0).contains(&sample.density)
        {
            return Err("invalid or duplicate retained material sample".into());
        }
        // Boundary samples belong to the parent strip. The common directed
        // hinge itself is invariant, so this does not open a geometric seam.
        let panel = cuts
            .iter()
            .filter(|cut| sample.rest_material[0] > **cut + 1e-6)
            .count();
        let mut point = sample.rest_material;
        for site in (0..panel).rev() {
            point = axis_rotation(
                point,
                [cuts[site], -1., 0.],
                [0., 1., 0.],
                plan.crease_angles_rad[site],
            );
        }
        point = axis_rotation(point, [0.; 3], plan.pose_axis, plan.pose_angle_rad);
        targets.push(MaterialTargetSample {
            id: sample.id.clone(),
            source_ref: sample.source_ref.clone(),
            source_revision: sample.source_revision.clone(),
            layer_ref: sample.layer_ref.clone(),
            local_stage: point.map(|v| v * plan.stage_units_per_material_unit),
            density: sample.density,
        });
    }
    Ok(targets)
}

/// Pair with the actual physical owner and its recipe. This is a preparation
/// join, not physical reception: the q-v/visible/acoustic ACK stays at #288.
pub fn pair_material_source_body(
    state: &M3State,
    request: &MaterialFoldRequest,
    plan: &MaterialFoldPlan,
    body: &PreparedSourceBody,
    recipe: &SourceGeometryRecipe,
) -> Result<Value, String> {
    admit_material_fold(state, request, plan)?;
    admit_source_form_metric(state, recipe, &body.request().geometry)?;
    if body.source_generation() != plan.source_generation
        || body.event_ref() != plan.event_ref
        || body.subject_ref() != plan.subject_ref
        || body.source_revision() != plan.source_revision
        || source_coordinate_wire(body.source_coordinate()) != plan.source_coordinate
    {
        return Err("material/body event, source, face or generation disconnected".into());
    }
    Ok(
        json!({"schema":"ql.m3-material-body-pair/v1","material":plan,"body":body,"body_recipe":recipe,
        "reception":"prepared-consumer-observation-required","numerical_owner":"ql::physical::PhysicalBody",
        "geometry_relation":"square crease material and declared source elastic frames retain distinct topology; native body observation supplies physical displacement"}),
    )
}

/// A retained square is a parameter domain over the ACTUAL three source
/// frames and their intervening ruled faces. This explicitly differs from the
/// four-strip crease demonstration. Every rendered surface point has a
/// nonnegative nodal partition over the same mechanical body; thickness is a
/// bounded retained layer offset along that face's geometric normal.
pub fn prepare_body_material(
    state: &M3State,
    request: &MaterialFoldRequest,
    plan: &MaterialFoldPlan,
    body: &PreparedSourceBody,
    recipe: &SourceGeometryRecipe,
    samples: &[MaterialRestSample],
    max_layer_depth_metres: f64,
) -> Result<Value, String> {
    pair_material_source_body(state, request, plan, body, recipe)?;
    // Reuse the exact identity/domain/finite admission, without using its
    // distinct four-strip output as this body's material geometry.
    material_fold_targets(state, request, plan, samples)?;
    if !max_layer_depth_metres.is_finite() || !(0.0..=100.0).contains(&max_layer_depth_metres) {
        return Err("bounded explicit retained layer depth required".into());
    }
    let half = recipe.frame_side_metres / 2.;
    let separation = recipe.site_separation_metres;
    let span = separation + half;
    let cuts = [
        -span,
        -separation + half,
        -half,
        half,
        separation - half,
        span,
    ];
    let nodes = &body.request().geometry.nodes;
    if nodes.len() != 12
        || nodes
            .iter()
            .enumerate()
            .any(|(i, n)| n.identity != (i + 1) as u64)
    {
        return Err("actual source-frame node topology required".into());
    }
    let mut output = Vec::with_capacity(samples.len());
    for sample in samples {
        let x = sample.rest_material[0].clamp(-1., 1.) * span;
        let y = (sample.rest_material[1].clamp(-1., 1.) + 1.) / 2.;
        let depth = sample.rest_material[2] * plan.metres_per_material_unit;
        if depth.abs() > max_layer_depth_metres {
            return Err("retained layer depth exceeds declared mechanical projection bound".into());
        }
        let patch = (0..5).find(|i| x <= cuts[i + 1]).unwrap_or(4);
        let s = (x - cuts[patch]) / (cuts[patch + 1] - cuts[patch]);
        let index = match patch {
            0 => [0, 1, 2, 3],
            1 => [1, 4, 7, 2],
            2 => [4, 5, 6, 7],
            3 => [5, 8, 11, 6],
            _ => [8, 9, 10, 11],
        };
        let weight = [(1. - s) * (1. - y), s * (1. - y), s * y, (1. - s) * y];
        let ds = [-(1. - y), 1. - y, y, -y];
        let dt = [-(1. - s), -s, s, 1. - s];
        let weighted = |values: [f64; 4]| -> [f64; 3] {
            std::array::from_fn(|axis| {
                index
                    .iter()
                    .zip(values)
                    .map(|(i, w)| nodes[*i].rest_metres[axis] * w)
                    .sum()
            })
        };
        let origin = weighted(weight);
        let a = weighted(ds);
        let b = weighted(dt);
        let normal = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];
        let length = normal.iter().map(|v| v * v).sum::<f64>().sqrt();
        if !length.is_finite() || length < 1e-12 {
            return Err("degenerate actual source-frame material face".into());
        }
        let normal = normal.map(|v| v / length);
        let metric = std::array::from_fn::<_, 3, _>(|axis| origin[axis] + depth * normal[axis]);
        output.push(json!({"sample":sample,"patch":patch,"depth_metres":depth,"normal":normal,
            "node_weights":index.iter().enumerate().map(|(j,i)|json!({"node_id":nodes[*i].identity,"weight":weight[j],"ds":ds[j],"dt":dt[j]})).collect::<Vec<_>>(),
            "rest_metres":metric,"local_stage":metric.map(|v|v*plan.stage_units_per_material_unit/plan.metres_per_material_unit)}));
    }
    Ok(
        json!({"schema":"ql.m3-body-material/v1","material":plan,"body":body,"body_recipe":recipe,
        "bounds":{"max_layer_depth_metres":max_layer_depth_metres,"max_displacement_metres":body.request().max_displacement_metres,"material_domain":[-1.,1.,-1.,1.]},
        "samples":output,"numerical_owner":"ql::physical::PhysicalBody","surface_law":"retained square parameterisation over actual source frames and connected ruled intersite faces; layer normal follows observed nodes"}),
    )
}
