//! The M3 inverse seam — `Q_entity/Q_composed → primary address64` — typed
//! as the retrieval-vs-canonical split and **held open by design**
//! (`M3-C31`; integrated-object s11; s17.3 #7).
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
/// **Deliberately unimplemented** — this is the open seam (`M3-C31`):
/// "canonical primary codon/address64 selection" remains the missing kernel
/// operation. Any future implementation must carry the elemental
/// (M2-carrier) and operator (M3-matrix) accountability; candidate
/// retrieval alone cannot discharge it.
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
#[derive(Debug, Clone, Copy)]
pub struct SelectionContext {
    /// Persistent elemental constitution/orientation — `Q_identity`.
    pub q_identity: [i32; 4],
    /// Current lived posture/deformation — `Q_composed`.
    pub q_composed: [i32; 4],
}

#[cfg(test)]
mod tests {
    use super::*;

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
            q_identity: [1, 0, 0, 0],
            q_composed: [1, 0, 0, 0],
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
            q_identity: [0, 0, 0, 0],
            q_composed: [0, 0, 0, 0],
        };
        let empty = RetrievalEvidence::from_scores(vec![]);
        assert!(
            NoDecision.select_primary(&empty, &context).is_none(),
            "retrieval returning nothing is not a canonical verdict"
        );
    }
}
