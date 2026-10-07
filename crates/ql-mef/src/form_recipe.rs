//! The PS-G form-recipe join (QL-MEF #299, first vertical): the M3 form law
//! meeting the landed Paśu manifestation owner.
//!
//! PS-E resolves WHO stands WHERE in WHICH roles (`ql.subject-manifestation/v1`).
//! This owner names the next join: given a resolved manifestation, the
//! entity-to-form determination for its formation occurrence, compiled to the
//! exact [`M3Operation`]s the Ta-Onta stage already routes onto the event's
//! own command batch (`StageChange::Form`), with the qualification's source
//! basis carried and the resulting form resolved exactly — before anything is
//! applied.
//!
//! The source law (`docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md`
//! §6 "Direct fold / rūpa grammar"; the August 23, 2026 human review, ruling
//! 11): M3's polarity×mobility binary is given direct spatial form as a
//! crease/fold state grammar — polarity = signed crease orientation (valley
//! yin / mountain yang), mobility = moving-resting hinge — and three
//! articulated sites realise the same six-bit/64-state address field shared by
//! codon and hexagram. The kernel encodes that cast law exactly and
//! parity-tests it (`ql-core` `pole::fold`).
//!
//! What this owner deliberately does not do: no invented lookup from
//! descriptive keys to addresses. The deep matrix names upstream primary-form
//! selection from an arbitrary situated entity as the source's unresolved seam
//! (§8); the recipe therefore carries the determination as a DECLARED input —
//! verified against the locus's actual formation qualification (refused by
//! name where the source qualifies nothing) and compiled exactly — never
//! fabricated out of text keys. The glyph-raster crease sampler stays the
//! app's PS-G half; this is the native form-law join.
use serde::{Deserialize, Serialize};

use crate::aw1_world::RootedFace;
use crate::coordinate_expression::{ExpressiveRole, SubjectManifestation};
use crate::m3_state::M3Operation;
use ql_core::{
    Codon64, FoldGeometry, FoldMotif, Mobility, Polarity, SiteReading, SiteState, project_site,
};

pub const FORM_RECIPE_CONTRACT: &str = "ql.psg-form-recipe/v1";

/// Where the form law this owner compiles actually lives. The cast law is
/// kernel-encoded and parity-tested; the fold grammar is the human-ratified
/// M3 reading; the open seam is named, not papered over.
pub const FORM_LAW_SOURCE_REFS: [(&str, &str); 4] = [
    (
        "EpiLogos/QL-MEF",
        "docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md §6 Direct fold / rūpa grammar — polarity = signed crease orientation (valley/mountain), mobility = moving-resting hinge; three articulated sites × two bits = the six-bit/64-state address field shared by codon and hexagram; one one-bit site change is one M3 line change (64 × 6 = 384)",
    ),
    (
        "EpiLogos/QL-MEF",
        "M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md §1 ruling 11 (August 23, 2026 review) — M3's polarity×mobility binary given direct spatial form as a crease/fold state grammar",
    ),
    (
        "EpiLogos/QL-MEF",
        "crates/ql-core/src/pole/fold.rs — the cast law (fold-grammar s12, kernel parity-tested): polarity = sign(ρ), valley (+) yin / mountain (−) yang; mobility = [ρ̇ ≠ 0]; canonical projection yin +22.5°, yang −22.5°, moving +22.5°/tick, resting 0",
    ),
    (
        "EpiLogos/QL-MEF",
        "M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md §8 — the upstream primary-form selection from an arbitrary situated entity is the source's unresolved seam; this owner carries determinations declared and verified, never closes the seam by invention",
    ),
];

/// The disclosed standing carried by every recipe.
pub const FORM_RECIPE_STANDING: &str = "declared-form-determination: the entity-to-form determination is declared by the \
     caller and verified against the locus's actual formation qualification (ql.subject-manifestation/v1) — a formation \
     occurrence the source does not represent refuses by name; the determination compiles to the exact M3Operations the \
     Ta-Onta stage routes onto the event's own command batch, and the resulting form resolves exactly through the kernel \
     cast law (parity-tested); no determination is looked up from descriptive keys — the upstream primary-form selection \
     from an arbitrary situated entity stays the source's named open seam (M3 matrix §8); the glyph-raster crease sampler \
     is the application's PS-G half and is not claimed here";

/// One declared site of a fold-motif determination, in body order (0 = X
/// outer, 1 = Y hinge, 2 = Z inner). The two bits are the M3 material
/// alphabet itself (A yin/moving, T yang/moving, C yin/resting, G
/// yang/resting — M3-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredSite {
    pub polarity: DeclaredPolarity,
    pub mobility: DeclaredMobility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeclaredPolarity {
    Yin,
    Yang,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeclaredMobility {
    Moving,
    Resting,
}

impl DeclaredSite {
    const fn site_state(self) -> SiteState {
        SiteState {
            polarity: match self.polarity {
                DeclaredPolarity::Yin => Polarity::Yin,
                DeclaredPolarity::Yang => Polarity::Yang,
            },
            mobility: match self.mobility {
                DeclaredMobility::Moving => Mobility::Moving,
                DeclaredMobility::Resting => Mobility::Resting,
            },
        }
    }
}

/// The declared entity-to-form determination for a formation occurrence. The
/// caller determines WHICH form the occurrence stands on — the recipe verifies
/// what the locus's qualification actually carries, compiles the determination
/// exactly, and records what stands; it never selects or invents a form on the
/// caller's behalf (the `DeterminedRelation` law of the manifestation owner).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "determination", rename_all = "kebab-case", deny_unknown_fields)]
pub enum FormDetermination {
    /// The M3-native grammar: a declared three-site fold motif
    /// (polarity×mobility per site), compiled through the kernel cast law to
    /// exact crease telemetry — one `CastCreases` operation.
    FoldMotif { sites: [DeclaredSite; 3] },
    /// A declared six-bit form address, compiled as declared — one
    /// `SelectForm` operation. Bounded by the address law; the recipe carries
    /// the declaration, it does not source it.
    Address { address: u8 },
}

/// The form the compiled operations resolve to, exactly — inspectable before
/// anything is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedForm {
    pub address: u8,
    /// The six-bit fold motif — the same bits as the address (the codon is
    /// the fold body, not an image assigned after the fact).
    pub motif_bits: u8,
    pub nucleotide_bits: [u8; 3],
    /// The form's lawful rotational states (7 non-dual, 8 dual).
    pub state_count: u8,
    pub hinge_bits: u8,
    /// The two overlapping pair relations at their 22.5° quanta, sharing the
    /// hinge (Y) — the source's own pair-field geometry.
    pub pair_angle_xy_deg10: i32,
    pub pair_angle_yz_deg10: i32,
}

impl ResolvedForm {
    pub(crate) fn of_codon(codon: Codon64) -> Self {
        let geometry = FoldGeometry::from_codon(codon);
        Self {
            address: codon.address(),
            motif_bits: codon.fold_motif().bits(),
            nucleotide_bits: codon.nucleotides().map(|n| n.bits()),
            state_count: codon.rotational_state_count(),
            hinge_bits: geometry.hinge.bits(),
            pair_angle_xy_deg10: geometry.pair_angle_xy().0,
            pair_angle_yz_deg10: geometry.pair_angle_yz().0,
        }
    }
}

/// The qualification's source basis: one entry per locus record whose keys
/// actually qualify the formation occurrence, with the payload pin and the
/// source revision it stands on — what a consumer re-verifies before trusting
/// the recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormationSourceRecord {
    pub payload_sha256: String,
    pub file_path: String,
    pub source_repository: String,
    pub source_revision: String,
    pub property_keys: Vec<String>,
}

/// One source-qualified form recipe: the declared entity-to-form determination
/// for a formation occurrence, the exact M3 operations it compiles to, the
/// form they resolve to, and the qualification's source basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormRecipe {
    pub schema: String,
    pub subject_ref: String,
    /// The formation occurrence's content-derived identity
    /// (`ql.subject-manifestation/v1`).
    pub occurrence_ref: String,
    pub locus_coordinate_ref: String,
    pub locus_face: RootedFace,
    /// The binding content revision the qualification was verified against.
    pub locus_binding_content_revision: String,
    pub determination: FormDetermination,
    /// The exact M3 operations the stage routes (`StageChange::Form`).
    pub operations: Vec<M3Operation>,
    pub form: ResolvedForm,
    pub source_basis: Vec<FormationSourceRecord>,
    pub standing: String,
}

/// The determination's compile core: verified form law, exactly — the same
/// path [`resolve_form_recipe`] runs after the locus qualification holds. The
/// fold-sequence owner compiles its phase endpoints through this same core.
pub(crate) fn compile_determination(
    determination: &FormDetermination,
) -> Result<(Vec<M3Operation>, Codon64), String> {
    match determination {
        FormDetermination::FoldMotif { sites } => {
            // The kernel cast law, exactly: each declared site projects to
            // its canonical crease telemetry, and the cast of that telemetry
            // re-derives the declared motif. A projection that failed the
            // round trip would be a broken form law, not a recipe.
            let declared: [SiteState; 3] = sites.map(DeclaredSite::site_state);
            let readings: [SiteReading; 3] = declared.map(project_site);
            let recast: [SiteState; 3] = readings.map(|r| r.cast());
            if recast != declared {
                return Err(
                    "the declared fold motif does not survive the kernel cast round trip; refusing a form the cast law does not determine"
                        .into(),
                );
            }
            Ok((
                vec![M3Operation::CastCreases {
                    angles_deg10: readings.map(|r| r.signed_angle),
                    velocities_deg10: readings.map(|r| r.angular_velocity),
                }],
                FoldMotif::from_sites(declared).to_codon(),
            ))
        }
        FormDetermination::Address { address } => {
            if *address >= 64 {
                return Err(format!(
                    "declared form address {address} outside the six-bit field 0..64; the M3 address law refuses it"
                ));
            }
            Ok((
                vec![M3Operation::SelectForm { address: *address }],
                Codon64::new(*address),
            ))
        }
    }
}

/// The canonical cast telemetry of a resolved form: the codon's own site
/// readings under the kernel cast law — the standing form the fold-sequence
/// owner holds and interpolates from. Exactly the telemetry a fold-motif
/// determination compiles to.
pub(crate) fn codon_telemetry(codon: Codon64) -> [SiteReading; 3] {
    codon.fold_motif().sites().map(project_site)
}

/// Resolves the form recipe over a resolved manifestation: the formation
/// occurrence must exist and be represented in source, the determination is
/// verified and compiled exactly, and the recipe carries the qualification's
/// source basis. Refusals are by name.
pub fn resolve_form_recipe(
    manifestation: &SubjectManifestation,
    determination: FormDetermination,
) -> Result<FormRecipe, String> {
    let occurrence = manifestation
        .occurrences
        .iter()
        .find(|o| o.role == ExpressiveRole::Formation)
        .ok_or_else(|| {
            format!(
                "a form recipe addresses the formation occurrence, which this manifestation of {} does not request (requested: {}); request formation at the locus",
                manifestation.subject_ref,
                manifestation
                    .occurrences
                    .iter()
                    .map(|o| o.role.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    // The source fact alone: a formation occurrence the locus's own records
    // do not carry is the obligation it is. The recipe refuses by name rather
    // than reading a form out of nothing — authored content never fabricates
    // source representation (the manifestation owner's own law).
    if !occurrence.represented || occurrence.source_records.is_empty() {
        return Err(format!(
            "the formation occurrence of {} at {} ({}) is not represented in source: the locus's own records carry no {} keys — the source determines no form there, and the recipe refuses rather than fabricating a determination",
            manifestation.subject_ref,
            manifestation.locus.coordinate_ref,
            manifestation.locus.face.as_str(),
            ExpressiveRole::Formation.qualifying_prefix(),
        ));
    }
    let (operations, codon) = compile_determination(&determination)?;
    let source_basis = occurrence
        .source_records
        .iter()
        .map(|source| FormationSourceRecord {
            payload_sha256: source.record.payload_sha256.clone(),
            file_path: source.file.path.clone(),
            source_repository: source.source_repository.clone(),
            source_revision: source.source_revision.clone(),
            property_keys: source
                .record
                .property_keys
                .iter()
                .filter(|key| key.starts_with(ExpressiveRole::Formation.qualifying_prefix()))
                .cloned()
                .collect(),
        })
        .collect();
    Ok(FormRecipe {
        schema: FORM_RECIPE_CONTRACT.into(),
        subject_ref: manifestation.subject_ref.clone(),
        occurrence_ref: occurrence.occurrence_ref.clone(),
        locus_coordinate_ref: manifestation.locus.coordinate_ref.clone(),
        locus_face: manifestation.locus.face,
        locus_binding_content_revision: manifestation.locus.binding_content_revision.clone(),
        determination,
        operations,
        form: ResolvedForm::of_codon(codon),
        source_basis,
        standing: FORM_RECIPE_STANDING.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ql_core::FoldState;

    /// The kernel's canonical sign conventions of the fold register (deg10).
    const VALLEY: i32 = 225;
    const MOUNTAIN: i32 = -225;
    const MOVING: i32 = 225;
    const RESTING: i32 = 0;

    fn declared_sites_of(address: u8) -> [DeclaredSite; 3] {
        Codon64::new(address)
            .fold_motif()
            .sites()
            .map(|s| DeclaredSite {
                polarity: match s.polarity {
                    Polarity::Yin => DeclaredPolarity::Yin,
                    Polarity::Yang => DeclaredPolarity::Yang,
                },
                mobility: match s.mobility {
                    Mobility::Moving => DeclaredMobility::Moving,
                    Mobility::Resting => DeclaredMobility::Resting,
                },
            })
    }

    #[test]
    fn every_declared_address_compiles_bounded_and_resolves_itself() {
        for address in 0u8..64 {
            let (operations, codon) =
                compile_determination(&FormDetermination::Address { address })
                    .expect("bounded address compiles");
            assert_eq!(operations.len(), 1);
            match operations[0] {
                M3Operation::SelectForm { address: compiled } => {
                    assert_eq!(compiled, address)
                }
                _ => panic!("an address determination compiles to SelectForm"),
            }
            assert_eq!(codon.address(), address);
        }
        let error = compile_determination(&FormDetermination::Address { address: 64 })
            .expect_err("64 is outside the six-bit field");
        assert!(error.contains("six-bit field 0..64"), "{error}");
    }

    #[test]
    fn every_declared_motif_compiles_to_cast_law_telemetry_exactly() {
        for address in 0u8..64 {
            let (operations, codon) = compile_determination(&FormDetermination::FoldMotif {
                sites: declared_sites_of(address),
            })
            .expect("a declared motif compiles");
            assert_eq!(codon.address(), address, "motif {address}");
            assert_eq!(operations.len(), 1);
            let M3Operation::CastCreases {
                angles_deg10,
                velocities_deg10,
            } = operations[0]
            else {
                panic!("a fold-motif determination compiles to CastCreases");
            };
            for (index, site) in declared_sites_of(address).iter().enumerate() {
                let expected_angle = match site.polarity {
                    DeclaredPolarity::Yin => VALLEY,
                    DeclaredPolarity::Yang => MOUNTAIN,
                };
                let expected_velocity = match site.mobility {
                    DeclaredMobility::Moving => MOVING,
                    DeclaredMobility::Resting => RESTING,
                };
                assert_eq!(
                    angles_deg10[index], expected_angle,
                    "site {index} of {address}"
                );
                assert_eq!(
                    velocities_deg10[index], expected_velocity,
                    "site {index} of {address}"
                );
            }
            // And the compiled telemetry casts back to the very same form on
            // the real fold-state owner; aperture and clock phase are held by
            // the state, never minted by the recipe.
            let recast = FoldState::from_cast(
                std::array::from_fn(|i| SiteReading {
                    signed_angle: angles_deg10[i],
                    angular_velocity: velocities_deg10[i],
                }),
                ql_core::ApertureIndex::new(0).unwrap(),
                0,
            );
            assert_eq!(
                recast.codon().address(),
                address,
                "cast round trip {address}"
            );
        }
    }

    #[test]
    fn the_resolved_form_names_the_source_pair_geometry() {
        // ATC: pairs (A,T) = index 1 and (T,C) = index 6 at their 22.5°
        // quanta, the hinge the middle site T.
        let (operations, codon) = compile_determination(&FormDetermination::FoldMotif {
            sites: [
                DeclaredSite {
                    polarity: DeclaredPolarity::Yin,
                    mobility: DeclaredMobility::Moving,
                },
                DeclaredSite {
                    polarity: DeclaredPolarity::Yang,
                    mobility: DeclaredMobility::Moving,
                },
                DeclaredSite {
                    polarity: DeclaredPolarity::Yin,
                    mobility: DeclaredMobility::Resting,
                },
            ],
        })
        .unwrap();
        assert_eq!(codon.address(), 0b00_01_10);
        assert_eq!(operations.len(), 1);
        let form = ResolvedForm::of_codon(codon);
        assert_eq!(form.address, 0b00_01_10);
        assert_eq!(form.motif_bits, 0b00_01_10);
        assert_eq!(form.nucleotide_bits, [0, 1, 2]);
        assert_eq!(form.hinge_bits, 1);
        assert_eq!(form.pair_angle_xy_deg10, 225);
        assert_eq!(form.pair_angle_yz_deg10, 6 * 225);
        assert_eq!(form.state_count, codon.rotational_state_count());
    }
}
