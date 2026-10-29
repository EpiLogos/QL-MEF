//! M2's body constants by element fibre: what a tattva gives the body it sets.
//!
//! Under Body source Tattva the tattva of the M2 condition sets the body through
//! its element (DLW manual §21.2 the Torus, §20.14 Resonance). Four tattvas carry
//! a fibre and Ākāśa carries none (`m2-correspondences` `material_fibre`, read
//! from the map). Each fibre gives the body its constants: Strike, Damping and Q
//! factor, Agitation and Boundary strength.
//!
//! No source table fixes these magnitudes (QL-MEF #135; the scene's material is a
//! declared policy, `continuous/scene_field.rs` MATERIAL_STANDING). They are the
//! manual's starting point, held here as M2's proposed table, and every one is a
//! live, tunable parameter (#254 D30): "The values are a starting point. Tune them
//! to what you hear."
use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::{Value, json};

use crate::m2_condition::correspondence_field;
use crate::m2_engine::MaterialFibre;

pub const BODY_CONTRACT: &str = "ql.m2-body-constants/v1";
/// The table's standing: proposed starting values, tunable, no source table.
pub const BODY_STANDING: &str = "proposed";

/// One fibre's constants.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct BodyConstants {
    pub fibre: MaterialFibre,
    /// How strongly velocity excites the body, a share of 0..1 (the Torus's Strike).
    pub strike_share: f64,
    /// How sharply the body rings (the Torus's Damping and Resonance's Q factor).
    pub q_factor: f64,
    /// How much the body is kicked about.
    pub agitation: f64,
    /// How firm the plate's edge is (Resonance's Boundary strength).
    pub boundary_strength: f64,
}

/// The manual's table, in its order: Earth, Water, Fire, Air.
pub const TABLE: [BodyConstants; 4] = [
    BodyConstants {
        fibre: MaterialFibre::Earth,
        strike_share: 0.45,
        q_factor: 12.0,
        agitation: 0.1,
        boundary_strength: 20.0,
    },
    BodyConstants {
        fibre: MaterialFibre::Water,
        strike_share: 0.55,
        q_factor: 6.0,
        agitation: 0.6,
        boundary_strength: 10.0,
    },
    BodyConstants {
        fibre: MaterialFibre::Fire,
        strike_share: 0.85,
        q_factor: 3.0,
        agitation: 2.0,
        boundary_strength: 4.0,
    },
    BodyConstants {
        fibre: MaterialFibre::Air,
        strike_share: 0.65,
        q_factor: 1.5,
        agitation: 3.5,
        boundary_strength: 2.0,
    },
];

/// The constants a fibre gives the body.
pub fn constants(fibre: MaterialFibre) -> &'static BodyConstants {
    TABLE
        .iter()
        .find(|row| row.fibre == fibre)
        .expect("every fibre has a row")
}

/// The tattvas the map gives this fibre (the coordinates of the correspondence rules that carry it).
fn tattvas_of(fibre: Option<MaterialFibre>) -> Vec<String> {
    correspondence_field()
        .rules
        .iter()
        .filter(|rule| rule.material_fibre == fibre)
        .filter_map(|rule| rule.tattva_coordinate.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// The table as QL reads it: each fibre's constants with the tattvas the map gives it, and the tattvas that carry none.
pub fn reading() -> Result<Value, String> {
    let mut rows = Vec::new();
    for row in TABLE {
        let tattvas = tattvas_of(Some(row.fibre));
        if tattvas.is_empty() {
            return Err(format!("the map gives no tattva the {:?} fibre", row.fibre));
        }
        rows.push(json!({
            "fibre": row.fibre,
            "tattvas": tattvas,
            "strike_share": row.strike_share,
            "q_factor": row.q_factor,
            "agitation": row.agitation,
            "boundary_strength": row.boundary_strength,
        }));
    }
    Ok(json!({
        "schema": BODY_CONTRACT,
        "standing": BODY_STANDING,
        "rows": rows,
        "unbodied": tattvas_of(None),
        "units": {"strike_share": "share of velocity, 0..1", "q_factor": "quality factor, 0.1..40",
            "agitation": "0..5", "boundary_strength": "0..30"},
        "source": {
            "table": "DLW manual §21.2 (Torus) and §20.14 (Resonance): a starting point, tuned by ear",
            "tunable": "#254 D30: every magnitude is a live, tunable parameter",
            "no_source_table": "QL-MEF #135: no source table fixes presentation damping or strike",
            "fibres": {"schema": crate::m2_condition::CORRESPONDENCE_CONTRACT,
                "registry_revision": correspondence_field().registry_revision},
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_fibre_has_one_row_and_the_maps_tattvas() {
        let reading = reading().unwrap();
        let rows = reading["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0]["fibre"], "earth");
        assert_eq!(rows[0]["tattvas"], json!(["#2-2-2-5-5"]));
        assert_eq!(constants(MaterialFibre::Air).q_factor, 1.5);
        assert_eq!(reading["unbodied"], json!(["#2-2-2-5-0/1"]));
    }
}
