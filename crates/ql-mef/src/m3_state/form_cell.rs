//! The M3 reading's living form, generation cell and law.
//!
//! Manual §22.3 (the M3 clock as M3′) and §22.8 (Binding) read the current
//! form as a body of three folds, the cell the chosen matrix makes of its
//! hexagram, and the seed and pose map the pose is read with. This module
//! only reports them. No operator is defined here: every value is read from
//! an existing QL operator, and each section names that source in its
//! `standing`.
//!
//! - folds: [`Codon64::nucleotides`], [`Nucleotide::polarity`] /
//!   [`Nucleotide::mobility`], [`ElementalQuaternionBasis`], the line law of
//!   [`Codon64::site_of_line`];
//! - generation: [`ql_core::generate`] (`ql.pole.m3-generation-law/v1`, map
//!   `M3-3-2-0..2`) for each of the three matrix families;
//! - law: [`QuatSeed`] and [`PoseMap`] (the active-state reader's declared
//!   variables), [`generate_rotational_states`] and [`RotationalPose`].
use ql_core::*;
use serde_json::{Value, json};

pub(super) fn family_of(index: u8) -> Result<MatrixFamily, String> {
    MatrixFamily::ALL
        .get(index as usize)
        .copied()
        .ok_or_else(|| "invalid matrix family".to_string())
}

const fn family_index(family: MatrixFamily) -> u8 {
    match family {
        MatrixFamily::Complementary => 0,
        MatrixFamily::MovingResting => 1,
        MatrixFamily::SameQuality => 2,
    }
}

const fn family_name(family: MatrixFamily) -> &'static str {
    match family {
        MatrixFamily::Complementary => "complementary",
        MatrixFamily::MovingResting => "moving-resting",
        MatrixFamily::SameQuality => "same-quality",
    }
}

const fn axis_name(axis: MatrixAxis) -> &'static str {
    match axis {
        MatrixAxis::I => "i",
        MatrixAxis::J => "j",
        MatrixAxis::K => "k",
    }
}

const fn class_name(class: CodonClass) -> &'static str {
    match class {
        CodonClass::PerfectPalindromic => "perfect-palindromic",
        CodonClass::ImperfectPalindromic => "imperfect-palindromic",
        CodonClass::NonPalindromicNonDual => "non-palindromic-non-dual",
        CodonClass::Dual => "dual",
    }
}

const fn polarity_name(polarity: Polarity) -> &'static str {
    match polarity {
        Polarity::Yin => "yin",
        Polarity::Yang => "yang",
    }
}

const fn mobility_name(mobility: Mobility) -> &'static str {
    match mobility {
        Mobility::Moving => "moving",
        Mobility::Resting => "resting",
    }
}

fn matrix_json(family: MatrixFamily) -> Value {
    json!({"family":family_name(family),"index":family_index(family),
        "number":family.matrix_number(),"axis":axis_name(family.axis())})
}

fn pair_json(pair: PairIndex16) -> Value {
    json!({"index":pair.index(),"sequence":format!("{}{}",pair.first(),pair.second()),
        "angle_deg10":pair.orientation_quantum_deg10()})
}

fn trigram_json(trigram: Trigram) -> Value {
    json!({"id":trigram.id,"line_pattern":trigram.binary})
}

/// A codon with what Binding reads of it: its sequence, class, four charges
/// and the two pairs it turns by.
fn codon_json(codon: Codon64) -> Value {
    let q = codon.four_charge();
    json!({"address":codon.address(),"sequence":codon.to_string(),
        "class":class_name(codon.classify()),"state_count":codon.rotational_state_count(),
        "charges_pp_mm_mp_pm":[q.pp,q.mm,q.mp,q.pm],
        "pair_xy":pair_json(codon.pair_xy()),"pair_yz":pair_json(codon.pair_yz())})
}

fn cell_json(cell: &GenerationCell) -> Value {
    json!({"matrix":matrix_json(cell.family),"hexagram_id":cell.hexagram,
        "lower_trigram":trigram_json(cell.lower_trigram()),
        "upper_trigram":trigram_json(cell.upper_trigram()),
        "lower_pair":pair_json(cell.lower_pair),"upper_pair":pair_json(cell.upper_pair),
        "positive":codon_json(cell.positive),"negative":codon_json(cell.negative),
        "non_dual":cell.is_non_dual(),"class":class_name(cell.class),
        "singlet":cell.fixed_point().map(codon_json)})
}

/// The living form: the current codon's three folds in site order (X outer,
/// Y hinge, Z inner), the hexagram, the pose and the matrix axis.
pub(super) fn living_form(fold: &FoldState, hexagram: Value) -> Value {
    let codon = fold.codon();
    let basis = ElementalQuaternionBasis::canonical();
    let sites = fold.sites();
    let folds: Vec<Value> = fold
        .nucleotides()
        .iter()
        .enumerate()
        .map(|(i, n)| {
            // Site i owns lines 2(2-i) (its polarity bit) and 2(2-i)+1 (its
            // mobility bit): `Codon64::site_of_line`.
            let polarity_line = 2 * (2 - i as u8);
            let (site, role) = [("X", "outer"), ("Y", "hinge"), ("Z", "inner")][i];
            json!({"site":site,"site_index":i,"role":role,
                "base":n.symbol().to_string(),"base_bits":n.bits(),
                "polarity":polarity_name(n.polarity()),"mobility":mobility_name(n.mobility()),
                "element":basis.element_of(*n).name(),"coin_value":n.coin_value().value(),
                "polarity_line":polarity_line,"mobility_line":polarity_line + 1,
                "signed_angle_deg10":sites[i].signed_angle,
                "angular_velocity_deg10":sites[i].angular_velocity})
        })
        .collect();
    json!({"codon":{"address":codon.address(),"sequence":codon.to_string(),
            "class":class_name(codon.classify())},
        "folds":folds,"hexagram":hexagram,"hexagram_id":codon.hexagram_id(),
        "pose":{"index":fold.rotational_index(),"ordinal":fold.rotational_pose().ordinal(),
            "state_count":fold.state_count(),"total_poses":ROTATIONAL_STATE_TOTAL},
        "matrix":matrix_json(fold.active_matrix_axis().matrix_family()),
        "standing":"reading of the current fold state. Folds: Codon64::nucleotides in site order with Nucleotide::polarity and Nucleotide::mobility (A Water, T Fire, C Earth, G Air by ElementalQuaternionBasis::canonical); a line change turns the fold Codon64::site_of_line names, its polarity bit on the even line and its mobility bit on the odd one. Pose: FoldState::rotational_pose ordinal in the 472-pose surface and the codon's 7 or 8 states. Matrix: FoldState::active_matrix_axis."})
}

/// The generation reading of one hexagram under a chosen matrix: the chosen
/// matrix's cell, and the cells of all three families at the same hexagram.
pub(super) fn generation_reading(chosen: MatrixFamily, hexagram: u8) -> Value {
    let cells: Vec<Value> = MatrixFamily::ALL
        .iter()
        .map(|family| cell_json(&generate(*family, hexagram)))
        .collect();
    json!({"law_ref":POLE_GENERATION_LAW_REF,"hexagram_id":hexagram & 0x3F,
        "matrix":matrix_json(chosen),"cell":cell_json(&generate(chosen, hexagram)),"cells":cells,
        "standing":"ql_core::generate (ql.pole.m3-generation-law/v1, map M3-3-2-0..2): the cell of the hexagram under the chosen matrix, and the cells of all three matrices at the same hexagram so a Pairing change is a reading. Positive codon a·c·d, negative a·b·d; non-dual exactly when b = c, and then both are the one codon, the singlet."})
}

/// The law the pose is read with: the matrix in force, the seed and the pose
/// map, with each pose slot of the current codon.
pub(super) fn law(fold: &FoldState) -> Value {
    let codon = fold.codon();
    let seeds: Vec<Value> = [QuatSeed::SumDifferenceMod, QuatSeed::FourCharge]
        .iter()
        .map(|seed| {
            let q = seed.quat(codon);
            json!({"law":seed.name(),"fr":seed.fr(),"quaternion_wxyz":[q.w,q.x,q.y,q.z]})
        })
        .collect();
    let count = codon.rotational_state_count();
    let states = generate_rotational_states(codon);
    let slots: Vec<Value> = (0..ROTATIONAL_TABLE_ENTRIES as u8)
        .map(|slot| {
            let candidate = states[slot as usize];
            let (lawful_slot, collapsed) = PoseMap::LawfulSlot.slot(codon, slot);
            let ordinal = RotationalPose::new(codon, slot)
                .ok()
                .map(|pose| pose.ordinal());
            let valence = match candidate.polarity {
                RotationalPolarity::Negative => "negative",
                RotationalPolarity::Positive => "positive",
            };
            json!({"slot":slot,"lawful":slot < count,"ordinal":ordinal,
                "rotation_degrees":candidate.rotation_degrees,"valence":valence,
                "resulting_codon":candidate.resulting_codon.address(),
                "rotational_value":candidate.rotational_value,"bipolar":candidate.is_non_dual,
                "lawful_slot":lawful_slot,"collapsed_by_lawful_slot":collapsed,
                "current":slot == fold.rotational_index()})
        })
        .collect();
    json!({"matrix":matrix_json(fold.active_matrix_axis().matrix_family()),
        "seed":{"in_force":{"law":QuatSeed::default().name(),"fr":QuatSeed::default().fr()},
            "choices":seeds,
            "standing":"QuatSeed (FR 2.Q.9 Sum/Difference/Mod, FR 2.Q.17 four-charge): the seed quaternion of the current codon under each law. In force is the active-state reader's default, QuatSeed::default; this state does not apply a reader, its pose is the one the request or SetPose set."},
        "pose_map":{"in_force":PoseMap::default().name(),
            "choices":[PoseMap::RawBin.name(),PoseMap::LawfulSlot.name()],
            "state_count":count,"slots":slots,
            "standing":"the eight ranked slots of generate_rotational_states for the current codon (rotation 45 degrees per slot), the 472-surface ordinal of each lawful slot (RotationalPose::ordinal), and the slot PoseMap::LawfulSlot lands each on. In force is PoseMap::default (raw bin); a non-dual codon has no lawful eighth slot."},
        "standing":"the law the pose is read with. Matrix: FoldState::active_matrix_axis, set by the request, ApplyMatrix or SetMatrix. Seed and pose map are the declared variables of the active-state reader (ql_core quat_active_pose); applying them to an environment needs an environment quaternion this state does not hold."})
}
