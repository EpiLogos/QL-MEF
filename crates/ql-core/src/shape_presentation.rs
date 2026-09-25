//! Numerical presentation of existing canonical shape addresses.
//!
//! Geometry authority remains `QlShape`/`SixBySixField`. These coordinates are
//! a normalized display convention, not semantic distances or new relations.
//! Applying a presentation never creates members for unoccupied addresses.
use crate::{
    ConstellationGrain, QlCoordinate, QlFace, QlPosition, QlShape, QlShapeAddress, SixBySixField,
    resolve_shape_ref,
};

pub const SHAPE_PRESENTATION_VERSION: &str = "ql.shape-presentation/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationAddress {
    Position(QlCoordinate),
    Cell(QlShapeAddress),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresentationSite {
    pub address: PresentationAddress,
    pub xyz: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapePresentation {
    pub shape: QlShape,
    /// Names the numerical convention separately from the canonical shape.
    pub embedding: &'static str,
    pub sites: Vec<PresentationSite>,
    pub row_axis: Vec<QlCoordinate>,
    pub column_axis: Vec<QlCoordinate>,
}

impl ShapePresentation {
    /// Check explicitly supplied placements in this normalized local frame.
    /// Empty/partial occupancy is legitimate; duplicate or unknown addresses,
    /// nonfinite numbers, or a changed slot are not a conforming placement.
    /// A host's whole-object transform is outside this local-frame constraint.
    pub fn conforms(&self, placements: &[PresentationSite], tolerance: f64) -> bool {
        if !tolerance.is_finite() || !(0.0..=0.001).contains(&tolerance) {
            return false;
        }
        let mut seen = std::collections::HashSet::new();
        placements.iter().all(|site| {
            seen.insert(site.address)
                && self
                    .sites
                    .iter()
                    .find(|expected| expected.address == site.address)
                    .is_some_and(|expected| {
                        expected
                            .xyz
                            .iter()
                            .zip(site.xyz)
                            .all(|(a, b)| b.is_finite() && (a - b).abs() <= tolerance)
                    })
        })
    }
}

/// Read an exact native shape ref. Unsupported embeddings are absent, never
/// approximated by a sixfold or inferred from a shape-ref substring.
pub fn shape_presentation(reference: &str) -> Option<ShapePresentation> {
    let shape = resolve_shape_ref(reference)?;
    match shape {
        QlShape::Constellation(ConstellationGrain::SixFold) => {
            // Screen-space convention: position zero at the top, successive
            // native positions clockwise. Rotation/scale have no QL meaning.
            let h = 3.0_f64.sqrt() / 2.0;
            let points = [
                [0.0, -1.0, 0.0],
                [h, -0.5, 0.0],
                [h, 0.5, 0.0],
                [0.0, 1.0, 0.0],
                [-h, 0.5, 0.0],
                [-h, -0.5, 0.0],
            ];
            let sites = points
                .into_iter()
                .enumerate()
                .map(|(index, xyz)| PresentationSite {
                    address: PresentationAddress::Position(QlCoordinate::new(
                        QlPosition::new(index as u8).expect("six native positions"),
                        QlFace::Direct,
                    )),
                    xyz,
                })
                .collect();
            Some(ShapePresentation {
                shape,
                embedding: "normalized-sixfold-ring/v1",
                sites,
                row_axis: vec![],
                column_axis: vec![],
            })
        }
        QlShape::SixBySix => {
            let field = SixBySixField::canonical();
            let sites = field
                .addresses
                .iter()
                .map(|address| PresentationSite {
                    address: PresentationAddress::Cell(*address),
                    xyz: [
                        f64::from(address.column.position.value()) * 0.4 - 1.0,
                        f64::from(address.row.position.value()) * 0.4 - 1.0,
                        0.0,
                    ],
                })
                .collect();
            Some(ShapePresentation {
                shape,
                embedding: "normalized-direct-conjugate-address-grid/v1",
                sites,
                row_axis: field.direct_axis,
                column_axis: field.conjugate_axis,
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sixfold_preserves_native_positions_and_checks_partial_local_constraints() {
        let shape = QlShape::Constellation(ConstellationGrain::SixFold);
        let reading = shape_presentation(&shape.shape_ref()).unwrap();
        assert_eq!(reading.sites.len(), 6);
        for (index, site) in reading.sites.iter().enumerate() {
            assert_eq!(
                site.address,
                PresentationAddress::Position(QlCoordinate::new(
                    QlPosition::new(index as u8).unwrap(),
                    QlFace::Direct
                ))
            );
            assert!((site.xyz[0].powi(2) + site.xyz[1].powi(2) - 1.0).abs() < 1e-12);
        }
        assert!(reading.conforms(&reading.sites[1..3], 1e-12));
        assert!(reading.conforms(&[], 0.0));
        let mut moved = reading.sites[1..3].to_vec();
        moved[0].xyz[0] += 0.1;
        assert!(!reading.conforms(&moved, 1e-12));
        moved[0].xyz[0] = f64::NAN;
        assert!(!reading.conforms(&moved, 1e-12));
        assert!(!reading.conforms(&[reading.sites[0].clone(), reading.sites[0].clone()], 0.0));
        assert!(!reading.conforms(&reading.sites, 1.0));
    }
    #[test]
    fn matrix_preserves_both_native_axes_and_all_exact_addresses_without_edges() {
        let native = SixBySixField::canonical();
        let reading = shape_presentation(native.shape_ref()).unwrap();
        assert_eq!(reading.row_axis, native.direct_axis);
        assert_eq!(reading.column_axis, native.conjugate_axis);
        assert_eq!(
            reading
                .sites
                .iter()
                .map(|site| site.address)
                .collect::<Vec<_>>(),
            native
                .addresses
                .iter()
                .copied()
                .map(PresentationAddress::Cell)
                .collect::<Vec<_>>()
        );
        assert_eq!(reading.sites.len(), 36);
        assert_eq!(reading.sites[0].xyz, [-1.0, -1.0, 0.0]);
        assert_eq!(reading.sites[35].xyz, [1.0, 1.0, 0.0]);
        assert!(reading.conforms(&reading.sites, 0.0));
    }
    #[test]
    fn no_unowned_embedding_or_shape_alias_is_invented() {
        assert!(shape_presentation(&QlShape::FourByFourByFour.shape_ref()).is_none());
        assert!(shape_presentation("ql:shape:1.0.0:constellation:6").is_none());
        assert!(shape_presentation("sixfold").is_none());
    }
}
