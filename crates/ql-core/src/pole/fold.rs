//! The M3 fold/rūpa state projection (T5) — the landing zone of M3
//! unresolved item 10.
//!
//! Three articulated crease sites carry the same two-bit distinction as the
//! M3 material alphabet; the codon is the fold body, not an image assigned
//! to it after the fact. The projection is bidirectional and exact:
//!
//! ```text
//! crease telemetry (signed angle ρ, angular velocity ρ̇)
//!     --coin-cast-->  (polarity, mobility) per site
//!     -->  six-bit fold motif  -->  address64
//!
//! address64  -->  pair16 hinge geometry at 22.5°·p (n2 the shared hinge)
//!     -->  site angles/velocities  -->  crease telemetry
//! ```
//!
//! Cast law (fold-grammar s12): polarity = sign(ρ) — valley (+) is yin,
//! mountain (−) is yang; mobility = [ρ̇ ≠ 0] — moving when the crease is
//! alive, resting when it holds. The projections use the canonical
//! convention: yin sites fold +22.5°, yang sites fold −22.5°; moving sites
//! carry +22.5°/tick of angular velocity, resting sites 0.
//!
//! The three M3 matrices physically transform the form on their bound
//! quaternion axes (kernel laws, encoded exactly):
//!
//! ```text
//! Complementary  (i): codon ⊕ 0x3F — every bit of every site conjugates
//! Moving/Resting (j): trigram exchange — lower and upper trigrams swap
//! Same-Quality   (k): resonance lookup — dataset-structural, 8 gaps
//! ```

use super::aperture::ApertureIndex;
use super::codon::{
    AngleDeg10, Codon64, FoldMotif, MatrixAxis, MatrixFamily, PairIndex16, SiteState,
};
use super::coin::{Mobility, Polarity};
use super::nucleotide::Nucleotide;
use super::pose::RotationalPose;
use crate::QlError;

/// The canonical sign conventions of the fold register, in tenths of a
/// degree (per tick for velocity).
pub const VALLEY_ANGLE_DEG10: i32 = 225;
pub const MOUNTAIN_ANGLE_DEG10: i32 = -225;
pub const MOVING_VELOCITY_DEG10: i32 = 225;
pub const RESTING_VELOCITY_DEG10: i32 = 0;

/// Live crease telemetry at one site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SiteReading {
    /// Signed crease angle ρ: positive valley, negative mountain.
    pub signed_angle: i32,
    /// Angular velocity ρ̇: nonzero when the crease is alive.
    pub angular_velocity: i32,
}

impl SiteReading {
    /// The coin-cast: polarity = sign(ρ) (0 rests as yin), mobility =
    /// [ρ̇ ≠ 0].
    pub const fn cast(&self) -> SiteState {
        SiteState {
            polarity: if self.signed_angle < 0 {
                Polarity::Yang
            } else {
                Polarity::Yin
            },
            mobility: if self.angular_velocity != 0 {
                Mobility::Moving
            } else {
                Mobility::Resting
            },
        }
    }
}

/// The canonical projection of one site state into crease telemetry.
pub const fn project_site(site: SiteState) -> SiteReading {
    SiteReading {
        signed_angle: match site.polarity {
            Polarity::Yin => VALLEY_ANGLE_DEG10,
            Polarity::Yang => MOUNTAIN_ANGLE_DEG10,
        },
        angular_velocity: match site.mobility {
            Mobility::Moving => MOVING_VELOCITY_DEG10,
            Mobility::Resting => RESTING_VELOCITY_DEG10,
        },
    }
}

/// The hinge geometry of a codon: the two overlapping pair relations with
/// the middle nucleotide as the shared hinge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldGeometry {
    /// Outer pair relation (X,Y).
    pub pair_xy: PairIndex16,
    /// Inner pair relation (Y,Z) — shares the hinge.
    pub pair_yz: PairIndex16,
    /// The shared hinge nucleotide (n2, the middle site).
    pub hinge: Nucleotide,
}

impl FoldGeometry {
    pub const fn from_codon(codon: Codon64) -> Self {
        Self {
            pair_xy: codon.pair_xy(),
            pair_yz: codon.pair_yz(),
            hinge: codon.middle(),
        }
    }

    /// Orientation quantum of the outer pair: 22.5°·p(X,Y).
    pub const fn pair_angle_xy(&self) -> AngleDeg10 {
        AngleDeg10(self.pair_xy.orientation_quantum_deg10())
    }

    /// Orientation quantum of the inner pair: 22.5°·p(Y,Z).
    pub const fn pair_angle_yz(&self) -> AngleDeg10 {
        AngleDeg10(self.pair_yz.orientation_quantum_deg10())
    }
}

/// The typed M3 rūpa fold-state — the s8 `m3_rupa` projection of one
/// current form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldState {
    codon: Codon64,
    geometry: FoldGeometry,
    sites: [SiteReading; 3],
    active_matrix_axis: MatrixAxis,
    rotational_index: u8,
    aperture16: ApertureIndex,
    fibonacci_phase60: u16,
}

impl FoldState {
    /// Project the fold state of a codon, canonically oriented.
    pub fn from_codon(codon: Codon64, aperture16: ApertureIndex, fibonacci_phase60: u16) -> Self {
        let sites = codon.fold_motif().sites().map(project_site);
        Self {
            codon,
            geometry: FoldGeometry::from_codon(codon),
            sites,
            active_matrix_axis: MatrixAxis::I,
            rotational_index: 0,
            aperture16,
            fibonacci_phase60,
        }
    }

    /// The coin-cast: derive the state from live crease telemetry.
    pub fn from_cast(
        readings: [SiteReading; 3],
        aperture16: ApertureIndex,
        fibonacci_phase60: u16,
    ) -> Self {
        let motif = FoldMotif::from_sites(readings.map(|r| r.cast()));
        Self::from_codon(motif.to_codon(), aperture16, fibonacci_phase60)
    }

    pub const fn codon(&self) -> Codon64 {
        self.codon
    }

    pub const fn motif(&self) -> FoldMotif {
        self.codon.fold_motif()
    }

    pub const fn nucleotides(&self) -> [Nucleotide; 3] {
        self.codon.nucleotides()
    }

    pub const fn geometry(&self) -> &FoldGeometry {
        &self.geometry
    }

    /// Live crease telemetry of the three sites (X, Y, Z).
    pub const fn sites(&self) -> &[SiteReading; 3] {
        &self.sites
    }

    pub const fn active_matrix_axis(&self) -> MatrixAxis {
        self.active_matrix_axis
    }

    pub const fn rotational_index(&self) -> u8 {
        self.rotational_index
    }

    /// The active rotational pose — the orientation of this same form.
    ///
    /// The state invariant keeps `rotational_index < state_count`, so the
    /// trusted constructor is sound here.
    pub fn rotational_pose(&self) -> RotationalPose {
        RotationalPose::from_trusted(self.codon, self.rotational_index)
    }

    pub const fn state_count(&self) -> u8 {
        self.codon.rotational_state_count()
    }

    pub const fn aperture16(&self) -> ApertureIndex {
        self.aperture16
    }

    pub const fn fibonacci_phase60(&self) -> u16 {
        self.fibonacci_phase60
    }

    /// Set the pose within the form's lawful state count.
    pub fn set_rotational_index(&mut self, index: u8) -> Result<(), QlError> {
        if index < self.state_count() {
            self.rotational_index = index;
            Ok(())
        } else {
            Err(QlError::InvalidPoleValue {
                field: "rotation-slot",
                value: index as u32,
            })
        }
    }

    /// Set the active matrix axis (which quaternion axis drives the fold).
    pub fn set_matrix_axis(&mut self, axis: MatrixAxis) {
        self.active_matrix_axis = axis;
    }

    /// The reciprocal aperture of the current one — the antipodal fold
    /// enacts reciprocity across the same body.
    pub fn fold_to_reciprocal_aperture(&mut self) {
        self.aperture16 = self.aperture16.reciprocal();
    }

    /// Apply one of the three M3 matrix operations through its bound
    /// quaternion axis. The address changes; the fold state re-projects.
    pub fn apply_matrix(&self, family: MatrixFamily) -> Result<Self, QlError> {
        let codon = match family {
            MatrixFamily::Complementary => Codon64::new(self.codon.address() ^ 0x3F),
            super::codon::MatrixFamily::MovingResting => {
                let a = self.codon.address();
                Codon64::new(((a & 0x07) << 3) | ((a >> 3) & 0x07))
            }
            super::codon::MatrixFamily::SameQuality => {
                // The resonance action is the dataset-structural RES matrix
                // (8 evolutionary gaps); without the dataset the operation
                // is typed but unresolved — STATUS_PROVISIONAL.
                return Err(QlError::InvalidPoleValue {
                    field: "resonance-unresolved",
                    value: self.codon.address() as u32,
                });
            }
        };
        let mut next = Self::from_codon(codon, self.aperture16, self.fibonacci_phase60);
        next.rotational_index = self
            .rotational_index
            .min(codon.rotational_state_count() - 1);
        next.active_matrix_axis = family.axis();
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::super::aperture::FibonacciGround;
    use super::*;

    #[test]
    fn cast_law_signs_and_velocities() {
        // valley (+) = yin; mountain (−) = yang; ρ̇ ≠ 0 = moving.
        let valley_moving = SiteReading {
            signed_angle: 400,
            angular_velocity: 12,
        };
        let cast = valley_moving.cast();
        assert_eq!(cast.polarity, Polarity::Yin);
        assert_eq!(cast.mobility, Mobility::Moving);

        let mountain_resting = SiteReading {
            signed_angle: -100,
            angular_velocity: 0,
        };
        let cast = mountain_resting.cast();
        assert_eq!(cast.polarity, Polarity::Yang);
        assert_eq!(cast.mobility, Mobility::Resting);

        let zero = SiteReading::default().cast();
        assert_eq!(zero.polarity, Polarity::Yin, "rest position casts yin");
        assert_eq!(zero.mobility, Mobility::Resting);
    }

    #[test]
    fn projection_and_cast_round_trip_over_all_64() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let state = FoldState::from_codon(codon, ApertureIndex::new(0).unwrap(), 0);
            // Project the state's sites back through the cast: same motif.
            let recast = FoldState::from_cast(*state.sites(), ApertureIndex::new(0).unwrap(), 0);
            assert_eq!(
                recast.codon(),
                codon,
                "projection/cast round trip at {address}"
            );
        }
    }

    #[test]
    fn hinge_geometry_uses_pair_quanta_at_22_5_degrees() {
        let codon = Codon64::from_nucleotides(Nucleotide::A, Nucleotide::T, Nucleotide::G);
        let geometry = FoldGeometry::from_codon(codon);
        assert_eq!(geometry.hinge, Nucleotide::T);
        assert_eq!(geometry.pair_xy.index(), 0b00_01);
        assert_eq!(geometry.pair_yz.index(), 0b01_11);
        assert_eq!(geometry.pair_angle_xy().reduced().0, 225);
        assert_eq!(geometry.pair_angle_yz().reduced().0, 225 * 7);
    }

    #[test]
    fn complementary_axis_is_an_involution_flipping_every_site_bit() {
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let state = FoldState::from_codon(codon, ApertureIndex::new(3).unwrap(), 7);
            let complemented = state.apply_matrix(MatrixFamily::Complementary).unwrap();
            assert_eq!(complemented.codon().address(), address ^ 0x3F);
            assert_eq!(complemented.active_matrix_axis(), MatrixAxis::I);
            // Involution.
            let back = complemented
                .apply_matrix(MatrixFamily::Complementary)
                .unwrap();
            assert_eq!(back.codon(), codon);
        }
    }

    #[test]
    fn moving_resting_axis_exchanges_trigrams() {
        let codon = Codon64::new(0b001010); // lower 010, upper 001
        let state = FoldState::from_codon(codon, ApertureIndex::new(0).unwrap(), 0);
        let moved = state.apply_matrix(MatrixFamily::MovingResting).unwrap();
        assert_eq!(moved.codon().address(), 0b010001);
        assert_eq!(moved.active_matrix_axis(), MatrixAxis::J);
        // Involution.
        let back = moved.apply_matrix(MatrixFamily::MovingResting).unwrap();
        assert_eq!(back.codon(), codon);
    }

    #[test]
    fn resonance_axis_stays_typed_but_unresolved_without_the_dataset() {
        let state = FoldState::from_codon(Codon64::new(0), ApertureIndex::new(0).unwrap(), 0);
        let result = state.apply_matrix(MatrixFamily::SameQuality);
        assert!(result.is_err(), "the RES action needs the dataset table");
    }

    #[test]
    fn reciprocal_aperture_is_the_antipodal_fold() {
        let mut state = FoldState::from_codon(Codon64::new(9), ApertureIndex::new(2).unwrap(), 30);
        state.fold_to_reciprocal_aperture();
        assert_eq!(state.aperture16().index(), 13);
        state.fold_to_reciprocal_aperture();
        assert_eq!(
            state.aperture16().index(),
            2,
            "reciprocity is an involution"
        );
    }

    #[test]
    fn pose_stays_within_the_form_state_count() {
        let mut state = FoldState::from_codon(Codon64::new(0), ApertureIndex::new(0).unwrap(), 0);
        // AAA is non-dual: 7 states.
        assert!(state.set_rotational_index(6).is_ok());
        assert!(state.set_rotational_index(7).is_err());
        assert_eq!(state.rotational_pose().codon(), Codon64::new(0));
    }

    #[test]
    fn fibonacci_phase_is_typed() {
        let state = FoldState::from_codon(Codon64::new(0), ApertureIndex::new(0).unwrap(), 59);
        assert_eq!(state.fibonacci_phase60(), 59);
        assert!(FibonacciGround::phase_orientation(59).is_ok());
    }
}
