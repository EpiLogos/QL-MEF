//! The 472 rotational poses of the resolved M3 forms.
//!
//! The Tarot rotational language is the pose language of the physical form:
//! 40 non-dual codons carry 7 distinct rotational states (the algebra collapses
//! one orientation at the non-dual eigenstate), 24 dual codons carry 8, giving
//! 40×7 + 24×8 = 472 lawful oriented poses. A pose is the orientation of the
//! same archetypal form, never a separate display property.

use super::codon::Codon64;
use crate::QlError;

/// Total lawful oriented poses over the 64 forms.
pub const ROTATIONAL_STATE_TOTAL: usize = 472;

/// One oriented pose of one codon: `(codon, rotation_slot)` with
/// `slot < codon.rotational_state_count()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationalPose {
    codon: Codon64,
    slot: u8,
}

impl RotationalPose {
    /// Trusted constructor for code paths that maintain the slot invariant
    /// themselves (the fold state keeps `slot < state_count`).
    pub const fn from_trusted(codon: Codon64, slot: u8) -> Self {
        Self { codon, slot }
    }

    pub fn new(codon: Codon64, slot: u8) -> Result<Self, QlError> {
        let state_count = codon.rotational_state_count();
        if slot < state_count {
            Ok(Self { codon, slot })
        } else {
            Err(QlError::InvalidPoleValue {
                field: "rotation-slot",
                value: slot as u32,
            })
        }
    }

    pub const fn codon(self) -> Codon64 {
        self.codon
    }

    pub const fn slot(self) -> u8 {
        self.slot
    }

    /// 7 or 8 — the dataset-backed state count of this form.
    pub const fn state_count(self) -> u8 {
        self.codon.rotational_state_count()
    }

    /// The ordinal of this pose in the 472-pose surface: codon order first,
    /// slot order second.
    pub const fn ordinal(self) -> usize {
        let codon_index = self.codon.address() as usize;
        let mut base = 0usize;
        let mut i = 0usize;
        while i < codon_index {
            base += Codon64::new(i as u8).rotational_state_count() as usize;
            i += 1;
        }
        base + self.slot as usize
    }
}

/// Iterate every lawful pose of the 472-pose surface in ordinal order.
pub fn all_poses() -> impl Iterator<Item = RotationalPose> {
    (0u8..64).flat_map(|address| {
        let codon = Codon64::new(address);
        (0..codon.rotational_state_count()).map(move |slot| RotationalPose { codon, slot })
    })
}

#[cfg(test)]
mod tests {
    use super::super::codon::CodonClass;
    use super::*;

    #[test]
    fn pose_surface_is_exactly_472() {
        assert_eq!(all_poses().count(), ROTATIONAL_STATE_TOTAL);
    }

    #[test]
    fn poses_round_trip_through_ordinal() {
        for pose in all_poses() {
            assert!(RotationalPose::new(pose.codon(), pose.slot()).is_ok());
            assert!(pose.ordinal() < ROTATIONAL_STATE_TOTAL);
        }
        // slots beyond the state count are unlawful
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            assert!(RotationalPose::new(codon, codon.rotational_state_count()).is_err());
        }
    }

    #[test]
    fn ordinals_are_injective_and_surjective() {
        let mut seen = vec![false; ROTATIONAL_STATE_TOTAL];
        for pose in all_poses() {
            let ordinal = pose.ordinal();
            assert!(!seen[ordinal], "ordinal {ordinal} reused");
            seen[ordinal] = true;
        }
        assert!(seen.iter().all(|seen| *seen));
    }

    #[test]
    fn non_dual_forms_collapse_one_orientation() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let expected = match codon.classify() {
                CodonClass::Dual => 8,
                _ => 7,
            };
            assert_eq!(codon.rotational_state_count(), expected);
        }
    }
}
