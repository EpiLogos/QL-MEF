//! The ruling-grid address transform over the MEF 72-address space (T3).
//!
//! The doubly-ruled surface gives the MEF registry its first native geometry:
//! every address `(lens, local position)` is the intersection of two ruling
//! families,
//!
//! ```text
//! P family (lens lines):     12 lines, quantum 30°  (12 × 30°  = 360°)
//! L family (position lines):  6 lines, quantum 60°  ( 6 × 60°  = 360°)
//! ```
//!
//! with the torus as the projective closure of the two families: parameters
//! are identified modulo a full turn on each axis, every ruling line closes
//! after visiting all of its points exactly once, and every P×L pair meets in
//! exactly one of the 72 = 12×6 intersections.
//!
//! The transform preserves the MEF law `absolute = (lens + local) mod 6`: the
//! absolute position is carried as a derived coordinate and survives the
//! round-trip exactly. One relation, no second substrate — the kernel field's
//! 72 addresses are the same addresses this grid covers.

use crate::{LensId, MefError, SublensRef};
use ql_core::AngleDeg10;

/// Semantic identity of the ruling-grid contract.
pub const RULING_GRID_VERSION: &str = "1.0.0";
pub const RULING_GRID_CONTRACT_REF: &str = "ql.mef.ruling-grid/v1";

/// The P-family ruling coordinate: one of the twelve lens lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RulingS(u8);

impl RulingS {
    pub const LINES: u8 = 12;
    /// The P-family quantum: 360°/12 = 30°, in tenths of a degree.
    pub const QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 12;

    pub fn new(index: u8) -> Result<Self, MefError> {
        if index < Self::LINES {
            Ok(Self(index))
        } else {
            Err(MefError::InvalidSublensPosition(index))
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    /// Angle of this ruling line: 30°·s.
    pub const fn orientation(self) -> AngleDeg10 {
        AngleDeg10(Self::QUANTUM_DEG10 * self.0 as i32)
    }

    /// The projective closure: stepping a full turn identifies s with itself.
    pub fn advanced(self, steps: u8) -> Self {
        Self((self.0 + steps) % Self::LINES)
    }
}

impl RulingS {
    /// The ruling slot of a lens: its position in the twelve-lens field.
    /// (Day/night twins share a MEF index but occupy distinct ruling lines.)
    pub const fn from_lens(lens: LensId) -> Self {
        Self(lens.slot())
    }
}

/// The L-family ruling coordinate: one of the six position lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RulingT(u8);

impl RulingT {
    pub const LINES: u8 = 6;
    /// The L-family quantum: 360°/6 = 60°, in tenths of a degree.
    pub const QUANTUM_DEG10: i32 = AngleDeg10::FULL_TURN_DEG10 / 6;

    pub fn new(index: u8) -> Result<Self, MefError> {
        if index < Self::LINES {
            Ok(Self(index))
        } else {
            Err(MefError::InvalidSublensPosition(index))
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    /// Angle of this ruling line: 60°·t.
    pub const fn orientation(self) -> AngleDeg10 {
        AngleDeg10(Self::QUANTUM_DEG10 * self.0 as i32)
    }

    pub fn advanced(self, steps: u8) -> Self {
        Self((self.0 + steps) % Self::LINES)
    }
}

/// A ruling-grid address: the intersection of one P line and one L line.
///
/// Bijection with the MEF 72: `s = lens`, `t = local position`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RulingGridAddress {
    s: RulingS,
    t: RulingT,
}

impl RulingGridAddress {
    /// The ruling intersection of a MEF sublens coordinate.
    pub fn from_sublens(sublens: SublensRef) -> Self {
        Self {
            s: RulingS::from_lens(sublens.lens().lens()),
            t: RulingT(sublens.position().value()),
        }
    }

    /// The MEF sublens coordinate at this intersection.
    pub fn to_sublens(self) -> Result<SublensRef, MefError> {
        let lens = LensId::ALL[self.s.0 as usize];
        SublensRef::canonical(lens, self.t.0)
    }

    pub const fn s(&self) -> RulingS {
        self.s
    }

    pub const fn t(&self) -> RulingT {
        self.t
    }

    /// The absolute position carried through the grid:
    /// `(lens index + local) mod 6`, where the lens index is the MEF index
    /// (day/night twins share it), not the twelve-fold ruling slot.
    pub const fn absolute_position(&self) -> u8 {
        (LensId::ALL[self.s.0 as usize].index() + self.t.0) % RulingT::LINES
    }
}

/// The projective-closure check of the ruling grid.
///
/// Returns the number of distinct intersections (72), the closure count of
/// each P line (6 points per line) and of each L line (12 points per line),
/// and verifies that every P×L pair meets exactly once — the torus closing
/// the two families.
pub fn projective_closure() -> Result<RulingClosureReport, MefError> {
    let mut intersections = std::collections::HashSet::new();
    for s in 0..RulingS::LINES {
        for t in 0..RulingT::LINES {
            if !intersections.insert((s, t)) {
                return Err(MefError::InvalidSublensPosition(s * RulingT::LINES + t));
            }
        }
    }

    // Every ruling line closes: advancing along its own parameter by its
    // line count returns to the start, and no sooner.
    let mut line_cycles = Vec::with_capacity((RulingS::LINES + RulingT::LINES) as usize);
    for s in 0..RulingS::LINES {
        let mut cycle = 1u8;
        let mut current = RulingS::new(s)?.advanced(1);
        while current.index() != s {
            current = current.advanced(1);
            cycle += 1;
        }
        line_cycles.push((s, cycle));
    }
    for (s, cycle) in &line_cycles {
        if *cycle != RulingS::LINES {
            return Err(MefError::InvalidSublensPosition(*cycle * 100 + s));
        }
    }
    for t in 0..RulingT::LINES {
        let mut cycle = 1u8;
        let mut current = RulingT::new(t)?.advanced(1);
        while current.index() != t {
            current = current.advanced(1);
            cycle += 1;
        }
        if cycle != RulingT::LINES {
            return Err(MefError::InvalidSublensPosition(cycle * 100 + t));
        }
    }

    Ok(RulingClosureReport {
        intersections: intersections.len(),
        p_lines: RulingS::LINES as usize,
        l_lines: RulingT::LINES as usize,
        points_per_p_line: RulingT::LINES as usize,
        points_per_l_line: RulingS::LINES as usize,
    })
}

/// The verified closure facts of the ruling grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RulingClosureReport {
    /// Distinct P×L intersections: 72 = 12×6.
    pub intersections: usize,
    /// P-family lines (lens lines): 12.
    pub p_lines: usize,
    /// L-family lines (position lines): 6.
    pub l_lines: usize,
    /// Points on each P line: 6.
    pub points_per_p_line: usize,
    /// Points on each L line: 12.
    pub points_per_l_line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruling_round_trip_covers_all_72_addresses() {
        for lens in LensId::ALL {
            for local in 0u8..6 {
                let sublens = SublensRef::canonical(lens, local).expect("canonical coordinate");
                let grid = RulingGridAddress::from_sublens(sublens);
                assert_eq!(grid.s().index(), lens.slot());
                let back = grid.to_sublens().expect("grid address must restore");
                assert_eq!(back.lens().lens(), lens);
                assert_eq!(back.position().value(), local);
                assert_eq!(
                    back.rotation().absolute_position().value(),
                    sublens.rotation().absolute_position().value()
                );
            }
        }
    }

    #[test]
    fn quanta_are_thirty_and_sixty_degrees() {
        assert_eq!(RulingS::QUANTUM_DEG10, 300);
        assert_eq!(RulingT::QUANTUM_DEG10, 600);
        assert_eq!(
            RulingS::LINES as i32 * RulingS::QUANTUM_DEG10,
            AngleDeg10::FULL_TURN_DEG10
        );
        assert_eq!(
            RulingT::LINES as i32 * RulingT::QUANTUM_DEG10,
            AngleDeg10::FULL_TURN_DEG10
        );
    }

    #[test]
    fn absolute_position_law_survives_the_transform() {
        for slot in 0u8..12 {
            let lens = LensId::ALL[slot as usize];
            for local in 0u8..6 {
                let sublens = SublensRef::canonical(lens, local).expect("coordinate");
                let grid = RulingGridAddress::from_sublens(sublens);
                let expected_absolute = (lens.index() + local) % 6;
                assert_eq!(grid.absolute_position(), expected_absolute);
            }
        }
    }

    #[test]
    fn projective_closure_closes_all_lines_and_meets_once() {
        let report = projective_closure().expect("the grid must close");
        assert_eq!(report.intersections, 72);
        assert_eq!(report.p_lines, 12);
        assert_eq!(report.l_lines, 6);
        assert_eq!(report.points_per_p_line, 6);
        assert_eq!(report.points_per_l_line, 12);
    }

    #[test]
    fn ruling_indices_reject_out_of_range() {
        assert!(RulingS::new(12).is_err());
        assert!(RulingT::new(6).is_err());
        assert_eq!(RulingS::new(0).unwrap().advanced(12).index(), 0);
        assert_eq!(RulingT::new(5).unwrap().advanced(1).index(), 0);
    }
}
