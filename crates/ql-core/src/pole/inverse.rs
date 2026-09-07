//! The M3 inverse seam — `Q_entity/Q_composed → primary address64` — typed
//! as the retrieval-vs-canonical split (`M3-C31`; integrated-object s11;
//! s17.3 #7; fold-grammar s12/§14 disposition 6).
//!
//! The stable distinction (integrated-object s11):
//!
//! ```text
//! identity hash      = exact identity/provenance anchor
//! spectral embedding = modal/form similarity and neighbourhood
//! quaternion state   = orientation/composition
//! M3 address         = determinate archetypal/form grammar
//! ```
//!
//! Retrieval is alias-tolerant: modal/spectral evidence yields inspectable
//! candidates, ordered, never decisive. Canonical selection is a law, not a
//! nearest neighbour: it stays accountable to the M2/M3 elemental and
//! operator relations. No law is shipped here — only the types that keep
//! the two roles from collapsing (acceptance criterion 11).
//!
//! ## Disposition after the kernel-side audit (N4)
//!
//! The open seam was re-examined for a canonical, deterministic selection
//! law grounded in the ported kernel architecture. The candidate mechanics
//! were audited and none qualifies:
//!
//! - `m3_quat_active_state`
//!   (`vendor/epi-kernel/reference/include/m3.h` FR 2.3.3) composes
//!   `env × codon → 8-fold active state`: it already requires the codon. It
//!   selects an orientation of a known form, never the form itself.
//! - `m3_det_with_quaternion` (`vendor/epi-kernel/reference/src/m3.c`,
//!   DET overlay) composes the ring × element × matrix-axis environment
//!   quaternion and orients the codons of an M2-derived active mask
//!   (`transduce_vibration_to_symbol`): the candidate set arrives from M2
//!   evidence, the quaternion only orients it. It never produces a primary.
//! - `m3_tarot_translate` (m3.h FR 2.3.16) rotates an already-given source
//!   codon quaternion by a card rotation and truncates the composed angle
//!   into an index: a positional readout, not an accountable selection.
//! - The only quaternion→codon bridge, `m3_quat_from_codon` (m3.h), is
//!   many-to-one: its image `(sum, v1−v3, sum mod 6)` distinguishes exactly
//!   46 points over the 64 codons for ANY assignment of the coin values
//!   {6,7,8,9} (the count depends only on the value multiset — pinned
//!   kernel-side in the tests below). No canonical preimage exists; any
//!   inversion would choose among preimages by convenience — exactly what
//!   the law form forbids (fold-grammar s12; M3 matrix §3 "Missing central
//!   inverse seam": solved from elemental, L2′, M2 evidence, charge and M3
//!   operator relations, or not at all).
//!
//! What moved: [`SelectionContext`] now carries the elemental composition
//! of both quaternions through the ratified basis
//! ([`super::basis::ElementalQuaternionBasis`] — `q = w_E + x_F·i + y_W·j +
//! z_A·k`), so any future law reads elements, not raw slots. What remains
//! missing is the ratified selection law itself: an operator-accountable
//! rule that resolves the preimage ambiguity from the elemental (M2-carrier)
//! and operator (M3-matrix) relations. `M3-C31` therefore stays open as the
//! explicitly missing kernel operation.

use super::basis::{Element, ElementalQuaternionBasis, QuaternionComponents};
use super::codon::Codon64;

/// Semantic identity of the typed inverse seam.
pub const INVERSE_SEAM_CONTRACT_REF: &str = "ql.pole.inverse-seam/v1";

/// Alias-tolerant retrieval evidence: ordered candidate addresses with
/// affinity scores.
///
/// This is the retrieval role's only product. There is intentionally no
/// conversion from evidence to a canonical address — collapsing the two
/// roles is the bug the split exists to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalEvidence {
    candidates: Vec<(Codon64, u32)>,
}

impl RetrievalEvidence {
    /// Assemble evidence from a spectral/modal retriever. Scores are
    /// unit-less affinities; the retriever's own provenance is responsible
    /// for their meaning.
    pub fn from_scores(mut candidates: Vec<(Codon64, u32)>) -> Self {
        candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.address().cmp(&b.0.address())));
        candidates.dedup_by(|a, b| a.0 == b.0);
        Self { candidates }
    }

    /// The ordered candidates — inspectable evidence, highest affinity
    /// first. Ties break by address for determinism.
    pub fn candidates(&self) -> &[(Codon64, u32)] {
        &self.candidates
    }

    /// The highest-affinity candidate, if any — still only evidence.
    pub fn best(&self) -> Option<(Codon64, u32)> {
        self.candidates.first().copied()
    }
}

/// A canonically selected primary address.
///
/// Constructible **only** through a [`SelectionLaw`]: no public constructor,
/// no `From<RetrievalEvidence>`, no conversion from identity hashes or
/// quaternion states. The canonical selection stays accountable to the
/// elemental and operator relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalAddress {
    codon: Codon64,
}

impl CanonicalAddress {
    /// The law's verdict, visible once selected.
    pub const fn codon(&self) -> Codon64 {
        self.codon
    }
}

/// The canonical selection law of `Q_entity/Q_composed → primary address64`.
///
/// **Still unimplemented** — this is the open seam (`M3-C31`): the
/// kernel-side audit (module header) found no deterministic, grounded,
/// operator-accountable selection mechanic to implement it from; the
/// quaternion→codon bridge is provably many-to-one. Any future
/// implementation must carry the elemental (M2-carrier) and operator
/// (M3-matrix) accountability through [`SelectionContext::basis`] and
/// resolve the preimage ambiguity by law, not convenience.
pub trait SelectionLaw {
    /// Select the canonical primary address from retrieval evidence and the
    /// situated quaternion context. Returns `None` when the law cannot
    /// decide from this evidence (which is not the same as retrieval
    /// returning nothing).
    fn select_primary(
        &self,
        evidence: &RetrievalEvidence,
        context: &SelectionContext,
    ) -> Option<CanonicalAddress>;
}

/// The situated context a selection law reads: the identity quaternion and
/// the composed quaternion are typed separately (acceptance criterion 11),
/// and the evidence never substitutes for them.
///
/// Both quaternions are [`QuaternionComponents`] — composed in the one
/// ratified elemental basis ([`ElementalQuaternionBasis`]:
/// `q = w_E + x_F·i + y_W·j + z_A·k`, `[w,x,y,z] = [Earth,Fire,Water,Air]`,
/// M4 ratification 5) — so a law reads elemental composition
/// (`context.by_element(...)`) rather than raw positional slots, and a
/// permuted elemental order cannot enter the seam as a context value.
#[derive(Debug, Clone, Copy)]
pub struct SelectionContext {
    /// Persistent elemental constitution/orientation — `Q_identity`.
    pub q_identity: QuaternionComponents,
    /// Current lived posture/deformation — `Q_composed`.
    pub q_composed: QuaternionComponents,
}

impl SelectionContext {
    /// The ratified elemental basis both quaternions are composed in — the
    /// only route from a component slot to an element.
    pub const fn basis(&self) -> ElementalQuaternionBasis {
        ElementalQuaternionBasis::canonical()
    }

    /// One elemental component of one quaternion through the ratified
    /// basis: `quaternion.by_element(element)` with the basis's own slot
    /// law (w=Earth, x=Fire, y=Water, z=Air).
    pub const fn by_element(&self, composed: bool, element: Element) -> i32 {
        match composed {
            false => self.q_identity.by_element(element),
            true => self.q_composed.by_element(element),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pole::Nucleotide;

    #[test]
    fn evidence_orders_candidates_deterministically() {
        let evidence = RetrievalEvidence::from_scores(vec![
            (Codon64::new(9), 10),
            (Codon64::new(3), 40),
            (Codon64::new(3), 40),  // duplicate collapses
            (Codon64::new(51), 40), // tie breaks by address
        ]);
        let ordered: Vec<u8> = evidence
            .candidates()
            .iter()
            .map(|(c, _)| c.address())
            .collect();
        assert_eq!(ordered, vec![3, 51, 9]);
        assert_eq!(evidence.best().map(|(c, _)| c.address()), Some(3));
    }

    #[test]
    fn canonical_address_has_no_path_from_evidence() {
        // The type-level split: evidence never becomes the address. The only
        // constructor is a law's verdict.
        let evidence = RetrievalEvidence::from_scores(vec![(Codon64::new(21), 100)]);
        let context = SelectionContext {
            q_identity: QuaternionComponents {
                w: 1,
                x: 0,
                y: 0,
                z: 0,
            },
            q_composed: QuaternionComponents {
                w: 1,
                x: 0,
                y: 0,
                z: 0,
            },
        };
        // With no law in scope, nothing selects. A local law can, and its
        // accountability is explicit at the call site.
        struct TestOnlyLaw;
        impl SelectionLaw for TestOnlyLaw {
            fn select_primary(
                &self,
                evidence: &RetrievalEvidence,
                _context: &SelectionContext,
            ) -> Option<CanonicalAddress> {
                evidence.best().map(|(codon, _)| CanonicalAddress { codon })
            }
        }
        let law = TestOnlyLaw;
        let selected = law
            .select_primary(&evidence, &context)
            .expect("law decides");
        assert_eq!(selected.codon().address(), 21);
    }

    #[test]
    fn empty_evidence_selects_nothing() {
        struct NoDecision;
        impl SelectionLaw for NoDecision {
            fn select_primary(
                &self,
                evidence: &RetrievalEvidence,
                _context: &SelectionContext,
            ) -> Option<CanonicalAddress> {
                evidence.best().map(|(codon, _)| CanonicalAddress { codon })
            }
        }
        let context = SelectionContext {
            q_identity: QuaternionComponents::default(),
            q_composed: QuaternionComponents::default(),
        };
        let empty = RetrievalEvidence::from_scores(vec![]);
        assert!(
            NoDecision.select_primary(&empty, &context).is_none(),
            "retrieval returning nothing is not a canonical verdict"
        );
    }

    #[test]
    fn context_carries_the_ratified_elemental_composition() {
        // The context's quaternion slots ARE the ratified basis slots:
        // w=Earth, x=Fire, y=Water, z=Air (M4 ratification 5) — read through
        // the basis, never by raw position.
        let context = SelectionContext {
            q_identity: QuaternionComponents {
                w: 10,
                x: 20,
                y: 30,
                z: 40,
            },
            q_composed: QuaternionComponents {
                w: 1,
                x: 2,
                y: 3,
                z: 4,
            },
        };
        let basis = context.basis();
        assert_eq!(basis.element_of(Nucleotide::A), Element::Water);
        assert_eq!(basis.element_of(Nucleotide::T), Element::Fire);
        assert_eq!(basis.element_of(Nucleotide::C), Element::Earth);
        assert_eq!(basis.element_of(Nucleotide::G), Element::Air);
        assert_eq!(context.by_element(false, Element::Earth), 10);
        assert_eq!(context.by_element(false, Element::Fire), 20);
        assert_eq!(context.by_element(false, Element::Water), 30);
        assert_eq!(context.by_element(false, Element::Air), 40);
        assert_eq!(context.by_element(true, Element::Earth), 1);
        assert_eq!(context.by_element(true, Element::Water), 3);
        assert_eq!(
            context.q_composed.by_element(Element::Air),
            4,
            "components and basis agree on the slot law"
        );
    }

    #[test]
    fn the_kernel_quaternion_bridge_is_not_invertible() {
        // Mirror of `m3_quat_from_codon`
        // (vendor/epi-kernel/reference/include/m3.h, FR 2.3.3): the only
        // quaternion→codon bridge distinguishes codons by
        // (sum, v1−v3, sum mod 6) — and z is a function of the sum, so the
        // content is (sum, v1−v3). The reachable image depends only on the
        // value multiset {6,7,8,9} ([`Nucleotide::NUCLEOTIDE_COIN_VALUE`] is
        // a `CoinSum`, confined to 6..=9), so the count below is invariant
        // under any value-table correction.
        let mut image = std::collections::HashSet::new();
        for address in 0u8..64 {
            let codon = Codon64::new(address);
            let v = |n: Nucleotide| n.coin_value().value() as i32;
            let sum = v(codon.outer()) + v(codon.middle()) + v(codon.inner());
            let diff = v(codon.outer()) - v(codon.inner());
            image.insert((sum, diff, sum % 6));
        }
        // 46 distinguishable quaternion points over 64 codons: at least 18
        // codons share a quaternion with another — no canonical preimage
        // exists, so no selection law can be inversion of this bridge.
        assert_eq!(image.len(), 46);
        assert!(image.len() < 64, "many-to-one: no canonical preimage");
        // Exhibit a concrete collision (computed, not hardcoded, so it holds
        // under any assignment of the four coin values). The extreme codons
        // (sum 18/27) are unique, so scan for the first shared key.
        let v = |n: Nucleotide| n.coin_value().value() as i32;
        let key = |address: u8| {
            let c = Codon64::new(address);
            (
                v(c.outer()) + v(c.middle()) + v(c.inner()),
                v(c.outer()) - v(c.inner()),
            )
        };
        let collision = (0u8..64)
            .map(|probe| {
                let k = key(probe);
                (
                    probe,
                    (0u8..64).filter(|a| key(*a) == k).collect::<Vec<u8>>(),
                )
            })
            .find(|(_, partners)| partners.len() >= 2)
            .expect("46 points over 64 codons force a shared key");
        assert!(
            collision.1.len() >= 2,
            "codons {:?} share one bridge quaternion",
            collision.1
        );
    }
}
