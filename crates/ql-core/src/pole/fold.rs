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
//! Same-Quality   (k): resonance lookup — the ported RES matrix (verbatim
//!                     from the C reference kernel): the recorded partner
//!                     on the 56 admitted entries, and one of 8 evolutionary
//!                     gaps (typed [`ApplyOutcome::Provisional`]) elsewhere
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

/// The kernel's resonance gap sentinel (`M3_RESONANCE_GAP`,
/// `vendor/epi-kernel/reference/include/m3.h` FR 2.3.3/2.3.8): the system
/// has reached an evolutionary gap — `STATUS_PROVISIONAL`.
pub const RESONANCE_GAP: u8 = 0xFF;

/// The M3 RES matrix — the Same-Quality (k-axis) resonance table — ported
/// VERBATIM from the C reference kernel
/// (`vendor/epi-kernel/reference/src/m3.c`, `M3_RES_MATRIX`):
/// dataset-structural, 56 admitted entries + 8 evolutionary gaps (`0xFF`).
///
/// Indexed by the raw six-bit codon address; entry `a` records the
/// resonance partner of address `a`, with [`RESONANCE_GAP`] where the
/// resonance does not resolve.
///
/// Verified gap structure (value-independent). The gaps are trigram-row
/// positions of the address bits alone — the kernel comments place each gap
/// at an `upper<<3 | lower` crossing (the hexagram id read from the same
/// six bits, cf. [`super::codon::Codon64::hexagram_id`]):
///
/// ```text
/// 0x05  Kun(000)/Li(101)    0x2A  Li(101)/Kan(010)
/// 0x15  Kan(010)/Li(101)    0x35  Xun(110)/Li(101)
/// 0x1A  Dui(011)/Kan(010)   0x3A  Qian(111)/Kan(010)
/// 0x22  Gen(100)/Kan(010)   0x3D  Qian(111)/Li(101)
/// ```
///
/// Every gap has a Kan (Water, 010) or Li (Fire, 101) lower trigram; the
/// Zhen (001) row is entirely admitted. The only kernel consumer of the
/// table (`m3_resonance_lookup`, m3.h FR 2.3.8/2.3.9) is a plain array
/// index over the address — it never reads the nucleotide I-Ching value
/// table (`NUCLEOTIDE_ICHING_VALUE`, m3.h FR 2.3.12), which participates
/// only in the `m3_quat_from_codon`/charge paths. The gap set is therefore
/// a property of the six-bit address structure and cannot move under the
/// canonical nucleotide value-table correction
/// ([`super::nucleotide::Nucleotide::NUCLEOTIDE_COIN_VALUE`]); conformance
/// tests in `tests/pole_res_matrix.rs` pin the exact addresses and their
/// trigram decomposition.
pub const M3_RES_MATRIX: [u8; 64] = [
    // Row 0 (upper=Kun=000):    gap at 0x05 (Kun/Li)
    0x00, 0x01, 0x02, 0x03, 0x04, 0xFF, 0x06, 0x07,
    // Row 1 (upper=Zhen=001):   all valid
    0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    // Row 2 (upper=Kan=010):    gap at 0x15 (Kan/Li)
    0x10, 0x11, 0x12, 0x13, 0x14, 0xFF, 0x16, 0x17,
    // Row 3 (upper=Dui=011):    gap at 0x1A (Dui/Kan)
    0x18, 0x19, 0xFF, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    // Row 4 (upper=Gen=100):    gap at 0x22 (Gen/Kan)
    0x20, 0x21, 0xFF, 0x23, 0x24, 0x25, 0x26, 0x27,
    // Row 5 (upper=Li=101):     gap at 0x2A (Li/Kan)
    0x28, 0x29, 0xFF, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    // Row 6 (upper=Xun=110):    gap at 0x35 (Xun/Li)
    0x30, 0x31, 0x32, 0x33, 0x34, 0xFF, 0x36, 0x37,
    // Row 7 (upper=Qian=111):   gaps at 0x3A (Qian/Kan) and 0x3D (Qian/Li)
    0x38, 0x39, 0xFF, 0x3B, 0x3C, 0xFF, 0x3E, 0x3F,
];

/// The eight evolutionary gap addresses of [`M3_RES_MATRIX`], ascending —
/// exactly its [`RESONANCE_GAP`] entries (test-pinned).
pub const RES_GAP_ADDRESSES: [u8; 8] = [0x05, 0x15, 0x1A, 0x22, 0x2A, 0x35, 0x3A, 0x3D];

/// Admitted (resonance-resolvable) codons: 64 − 8 gaps = 56, the kernel's
/// "56 entries + 8 gaps" admissibility split (`M3-C16`).
pub const RES_ADMITTED_COUNT: usize = 56;

/// The recorded resonance entry of a codon address — the typed form of the
/// kernel's `m3_resonance_lookup` (m3.h FR 2.3.9): `Some(partner)` on the
/// 56 admitted entries, `None` at the 8 evolutionary gaps (where the kernel
/// sets `STATUS_PROVISIONAL`).
pub const fn resonance_entry(address: u8) -> Option<u8> {
    let entry = M3_RES_MATRIX[(address & 0x3F) as usize];
    if entry == RESONANCE_GAP {
        None
    } else {
        Some(entry)
    }
}

/// Whether a codon address sits on an evolutionary resonance gap.
pub const fn is_resonance_gap(address: u8) -> bool {
    resonance_entry(address).is_none()
}

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

/// The outcome of applying one M3 matrix family through its bound
/// quaternion axis.
///
/// The i (Complementary) and j (Moving/Resting) transforms are total and
/// always [`ApplyOutcome::Applied`]. The k (Same-Quality) transform reads
/// the dataset-structural RES matrix, whose 8 evolutionary gaps yield
/// [`ApplyOutcome::Provisional`] — the kernel's `STATUS_PROVISIONAL`
/// (m3.h FR 2.3.8) carried as a typed outcome rather than an error: the
/// operation is lawful and executable, the gap means the transform itself
/// does not resolve and the fold stays at its current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyOutcome {
    /// The transform resolved to a determinate next fold state.
    Applied(FoldState),
    /// The k-axis resonance entry is an evolutionary gap
    /// ([`RESONANCE_GAP`]): typed but unresolved; the resident fold state
    /// is unchanged.
    Provisional,
}

impl ApplyOutcome {
    /// The applied state, when the transform resolved.
    pub const fn applied(self) -> Option<FoldState> {
        match self {
            Self::Applied(state) => Some(state),
            Self::Provisional => None,
        }
    }
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
    ///
    /// The Same-Quality (k) family reads the ported RES matrix
    /// ([`M3_RES_MATRIX`]): a valid entry applies like the i/j families —
    /// the rotational index carries over clamped to the new codon's state
    /// count, aperture and Fibonacci phase are preserved, and the active
    /// axis is set to the family's own (k). An entry that is one of the 8
    /// evolutionary gaps returns [`ApplyOutcome::Provisional`] instead of an
    /// error (kernel `STATUS_PROVISIONAL`); the design keeps all three
    /// families on one return path so the provisional status is
    /// distinguishable in the type, not buried in an error channel.
    pub fn apply_matrix(&self, family: MatrixFamily) -> Result<ApplyOutcome, QlError> {
        let codon = match family {
            MatrixFamily::Complementary => Codon64::new(self.codon.address() ^ 0x3F),
            super::codon::MatrixFamily::MovingResting => {
                let a = self.codon.address();
                Codon64::new(((a & 0x07) << 3) | ((a >> 3) & 0x07))
            }
            super::codon::MatrixFamily::SameQuality => {
                match resonance_entry(self.codon.address()) {
                    Some(target) => Codon64::new(target),
                    None => return Ok(ApplyOutcome::Provisional),
                }
            }
        };
        let mut next = Self::from_codon(codon, self.aperture16, self.fibonacci_phase60);
        next.rotational_index = self
            .rotational_index
            .min(codon.rotational_state_count() - 1);
        next.active_matrix_axis = family.axis();
        Ok(ApplyOutcome::Applied(next))
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
            let complemented = state
                .apply_matrix(MatrixFamily::Complementary)
                .unwrap()
                .applied()
                .expect("the i axis is total");
            assert_eq!(complemented.codon().address(), address ^ 0x3F);
            assert_eq!(complemented.active_matrix_axis(), MatrixAxis::I);
            // Involution.
            let back = complemented
                .apply_matrix(MatrixFamily::Complementary)
                .unwrap()
                .applied()
                .expect("the i axis is total");
            assert_eq!(back.codon(), codon);
        }
    }

    #[test]
    fn moving_resting_axis_exchanges_trigrams() {
        let codon = Codon64::new(0b001010); // lower 010, upper 001
        let state = FoldState::from_codon(codon, ApertureIndex::new(0).unwrap(), 0);
        let moved = state
            .apply_matrix(MatrixFamily::MovingResting)
            .unwrap()
            .applied()
            .expect("the j axis is total");
        assert_eq!(moved.codon().address(), 0b010001);
        assert_eq!(moved.active_matrix_axis(), MatrixAxis::J);
        // Involution.
        let back = moved
            .apply_matrix(MatrixFamily::MovingResting)
            .unwrap()
            .applied()
            .expect("the j axis is total");
        assert_eq!(back.codon(), codon);
    }

    #[test]
    fn resonance_axis_applies_admitted_entries_and_marks_gaps_provisional() {
        let aperture = ApertureIndex::new(3).unwrap();
        for address in 0u8..64 {
            let state = FoldState::from_codon(Codon64::new(address), aperture, 7);
            let outcome = state.apply_matrix(MatrixFamily::SameQuality).unwrap();
            if is_resonance_gap(address) {
                // The kernel's STATUS_PROVISIONAL as a typed outcome, never
                // an error: the fold stays at its current state.
                assert_eq!(outcome, ApplyOutcome::Provisional, "gap {address:#04x}");
                assert!(outcome.applied().is_none());
            } else {
                let next = outcome.applied().expect("admitted resonance entry applies");
                assert_eq!(next.codon().address(), address, "partner of {address:#04x}");
                assert_eq!(next.active_matrix_axis(), MatrixAxis::K);
                // Registers preserved; the pose carries over the identity
                // partner within the same state count.
                assert_eq!(next.aperture16(), aperture);
                assert_eq!(next.fibonacci_phase60(), 7);
                assert_eq!(next.rotational_index(), 0);
                assert_eq!(next.rotational_pose().codon(), next.codon());
            }
        }
    }

    #[test]
    fn same_quality_carries_the_pose_and_preserves_the_registers() {
        // AAA (admitted, 7 rotational states): pose index 6 carries exactly.
        let mut state = FoldState::from_codon(Codon64::new(0), ApertureIndex::new(2).unwrap(), 11);
        state.set_rotational_index(6).unwrap();
        let next = state
            .apply_matrix(MatrixFamily::SameQuality)
            .unwrap()
            .applied()
            .expect("AAA is admitted");
        assert_eq!(next.rotational_index(), 6, "pose carries over");
        assert_eq!(next.aperture16().index(), 2, "aperture preserved");
        assert_eq!(next.fibonacci_phase60(), 11, "phase preserved");
        assert_eq!(next.active_matrix_axis(), MatrixAxis::K);
        // The clamp to the new codon's state count is the shared tail of all
        // three arms; under the identity RES data the k axis never changes
        // the codon, so the clamp itself is exercised by the i/j arms.
    }

    #[test]
    fn the_eight_res_gaps_are_exactly_the_recorded_addresses() {
        let gaps: Vec<u8> = (0u8..64).filter(|a| is_resonance_gap(*a)).collect();
        assert_eq!(gaps, RES_GAP_ADDRESSES);
        assert_eq!(gaps.len(), 64 - RES_ADMITTED_COUNT);
        for gap in gaps {
            assert_eq!(M3_RES_MATRIX[gap as usize], RESONANCE_GAP);
        }
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
