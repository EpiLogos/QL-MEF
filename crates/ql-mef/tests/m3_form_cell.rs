//! MM1: the M3 reading carries the living form, the generation cell and the
//! law (manual §22.3 and §22.8). Every expected value is computed here from
//! the QL operators themselves (`ql_core::generate`, `Codon64`, `Nucleotide`,
//! `QuatSeed`, `PoseMap`, `generate_rotational_states`), never from a typed
//! table.
use ql_core::*;
use ql_mef::m3_state::*;
use serde_json::{Value, json};

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/kernel/m3-parent-consumer-current-v1.json"
    ))
    .unwrap()
}

fn request(address: u8, pose: u8, axis: u8) -> M3Request {
    let mut v = fixture()["request"].clone();
    v["address"] = json!(address);
    v["pose"] = json!(pose);
    v["aperture"] = json!(0);
    v["matrix_axis"] = json!(axis);
    serde_json::from_value(v).unwrap()
}

fn state(address: u8, pose: u8, axis: u8) -> M3State {
    M3State::new(request(address, pose, axis)).unwrap()
}

fn command(state: &M3State, operations: Vec<M3Operation>) -> M3Command {
    let mut c: M3Command = serde_json::from_value(fixture()["commands"][0].clone()).unwrap();
    c.expected_generation = state.generation();
    c.operations = operations;
    c
}

fn class_str(class: CodonClass) -> &'static str {
    match class {
        CodonClass::PerfectPalindromic => "perfect-palindromic",
        CodonClass::ImperfectPalindromic => "imperfect-palindromic",
        CodonClass::NonPalindromicNonDual => "non-palindromic-non-dual",
        CodonClass::Dual => "dual",
    }
}

fn family_str(family: MatrixFamily) -> &'static str {
    match family {
        MatrixFamily::Complementary => "complementary",
        MatrixFamily::MovingResting => "moving-resting",
        MatrixFamily::SameQuality => "same-quality",
    }
}

fn axis_str(axis: MatrixAxis) -> &'static str {
    match axis {
        MatrixAxis::I => "i",
        MatrixAxis::J => "j",
        MatrixAxis::K => "k",
    }
}

fn family_at(index: usize) -> MatrixFamily {
    MatrixFamily::ALL[index]
}

fn assert_codon(got: &Value, want: Codon64) {
    let q = want.four_charge();
    assert_eq!(got["address"], want.address());
    assert_eq!(got["sequence"], want.to_string());
    assert_eq!(got["class"], class_str(want.classify()));
    assert_eq!(got["state_count"], want.rotational_state_count());
    assert_eq!(got["charges_pp_mm_mp_pm"], json!([q.pp, q.mm, q.mp, q.pm]));
    for (key, pair) in [("pair_xy", want.pair_xy()), ("pair_yz", want.pair_yz())] {
        assert_eq!(got[key]["index"], pair.index());
        assert_eq!(
            got[key]["sequence"],
            format!("{}{}", pair.first(), pair.second())
        );
        assert_eq!(got[key]["angle_deg10"], pair.orientation_quantum_deg10());
    }
}

fn assert_cell(got: &Value, want: &GenerationCell) {
    assert_eq!(got["matrix"]["family"], family_str(want.family));
    assert_eq!(got["matrix"]["number"], want.family.matrix_number());
    assert_eq!(got["matrix"]["axis"], axis_str(want.family.axis()));
    assert_eq!(got["hexagram_id"], want.hexagram);
    assert_eq!(got["lower_trigram"]["id"], want.lower_trigram().id);
    assert_eq!(got["upper_trigram"]["id"], want.upper_trigram().id);
    assert_eq!(
        got["lower_trigram"]["line_pattern"],
        want.lower_trigram().binary
    );
    for (key, pair) in [
        ("lower_pair", want.lower_pair),
        ("upper_pair", want.upper_pair),
    ] {
        assert_eq!(got[key]["index"], pair.index());
        assert_eq!(
            got[key]["sequence"],
            format!("{}{}", pair.first(), pair.second())
        );
    }
    assert_codon(&got["positive"], want.positive);
    assert_codon(&got["negative"], want.negative);
    assert_eq!(got["non_dual"], want.is_non_dual());
    assert_eq!(got["class"], class_str(want.class));
    match want.fixed_point() {
        Some(single) => assert_codon(&got["singlet"], single),
        None => assert!(got["singlet"].is_null()),
    }
}

fn apply(state: &mut M3State, operations: Vec<M3Operation>) -> M3Receipt {
    let c = command(state, operations);
    state.apply(c).unwrap()
}

#[test]
fn the_prior_snapshot_fields_are_byte_identical_for_unchanged_states() {
    // `m3_state_prior_snapshots.json` was captured from the snapshot before
    // this lane added a field. Removing the three new sections from today's
    // snapshot must give back those bytes: nothing was renamed or removed.
    let golden: Vec<Value> =
        serde_json::from_str(include_str!("m3_state_prior_snapshots.json")).unwrap();
    let f = fixture();
    let mut ours = Vec::new();
    let mut s = M3State::new(serde_json::from_value(f["request"].clone()).unwrap()).unwrap();
    let commands: Vec<M3Command> = serde_json::from_value(f["commands"].clone()).unwrap();
    ours.push(("fixture-initial".to_string(), s.snapshot()));
    for (i, c) in commands.into_iter().enumerate() {
        s.apply(c).unwrap();
        ours.push((format!("fixture-after-command-{i}"), s.snapshot()));
    }
    for (label, address, pose, aperture, axis) in [
        ("address5-moving-resting", 5, 0, 0, 1),
        ("address56-same-quality", 56, 3, 9, 2),
        ("address0-perfect-palindrome", 0, 0, 0, 0),
    ] {
        let mut r = f["request"].clone();
        r["address"] = json!(address);
        r["pose"] = json!(pose);
        r["aperture"] = json!(aperture);
        r["matrix_axis"] = json!(axis);
        let s = M3State::new(serde_json::from_value(r).unwrap()).unwrap();
        ours.push((label.to_string(), s.snapshot()));
    }
    assert_eq!(golden.len(), ours.len());
    for (golden, (label, mut now)) in golden.iter().zip(ours) {
        assert_eq!(golden["label"], label);
        let object = now.as_object_mut().unwrap();
        for new in ["living_form", "generation", "law"] {
            assert!(object.remove(new).is_some(), "{label}: {new} is missing");
        }
        assert_eq!(
            serde_json::to_string(&now).unwrap(),
            serde_json::to_string(&golden["snapshot"]).unwrap(),
            "{label}: a prior snapshot field changed"
        );
    }
}

#[test]
fn the_living_form_has_three_folds_in_site_order_read_from_ql() {
    let basis = ElementalQuaternionBasis::canonical();
    for address in 0..64u8 {
        let s = state(address, 0, 0);
        let v = s.snapshot();
        let form = &v["living_form"];
        let codon = Codon64::new(address);
        assert_eq!(form["codon"]["address"], address);
        assert_eq!(form["codon"]["sequence"], codon.to_string());
        assert_eq!(form["codon"]["class"], class_str(codon.classify()));
        assert_eq!(form["hexagram_id"], codon.hexagram_id());
        assert_eq!(form["hexagram"], v["form"]["hexagram"]);
        let folds = form["folds"].as_array().unwrap();
        assert_eq!(folds.len(), 3);
        for (i, n) in codon.nucleotides().iter().enumerate() {
            let fold = &folds[i];
            assert_eq!(fold["site_index"], i);
            assert_eq!(fold["base"], n.symbol().to_string());
            assert_eq!(fold["base_bits"], n.bits());
            let polarity = if n.polarity().valley() { "yin" } else { "yang" };
            assert_eq!(fold["polarity"], polarity);
            let mobility = if n.mobility() == Mobility::Moving {
                "moving"
            } else {
                "resting"
            };
            assert_eq!(fold["mobility"], mobility);
            assert_eq!(fold["element"], basis.element_of(*n).name());
            assert_eq!(fold["coin_value"], n.coin_value().value());
            // The fold's two lines are the two bits of its site.
            let polarity_line = fold["polarity_line"].as_u64().unwrap() as u8;
            assert_eq!(fold["mobility_line"], polarity_line + 1);
            assert_eq!(Codon64::site_of_line(polarity_line), i);
            assert_eq!(Codon64::site_of_line(polarity_line + 1), i);
            assert_eq!(fold["signed_angle_deg10"], v["form"]["angles_deg10"][i]);
            assert_eq!(
                fold["angular_velocity_deg10"],
                v["form"]["velocities_deg10"][i]
            );
        }
        assert_eq!(form["pose"]["index"], 0);
        assert_eq!(
            form["pose"]["ordinal"],
            RotationalPose::new(codon, 0).unwrap().ordinal()
        );
        assert_eq!(form["pose"]["state_count"], codon.rotational_state_count());
        assert_eq!(form["pose"]["total_poses"], 472);
        assert_eq!(form["matrix"]["family"], "complementary");
        assert_eq!(form["matrix"]["axis"], "i");
    }
    // Manual §22.3: A is Water, T Fire, C Earth, G Air.
    let s = state(
        Codon64::from_nucleotides(Nucleotide::A, Nucleotide::T, Nucleotide::C).address(),
        0,
        0,
    );
    let folds = s.snapshot()["living_form"]["folds"].clone();
    assert_eq!(
        [
            &folds[0]["element"],
            &folds[1]["element"],
            &folds[2]["element"]
        ],
        [&json!("Water"), &json!("Fire"), &json!("Earth")]
    );
    let s = state(
        Codon64::from_nucleotides(Nucleotide::G, Nucleotide::G, Nucleotide::G).address(),
        0,
        0,
    );
    assert_eq!(s.snapshot()["living_form"]["folds"][0]["element"], "Air");
}

#[test]
fn a_line_change_turns_exactly_one_fold_and_moves_the_cell() {
    for address in 0..64u8 {
        for line in 0..6u8 {
            let mut s = state(address, 0, 1);
            let before = s.snapshot();
            apply(&mut s, vec![M3Operation::ChangeLine { line }]);
            let after = s.snapshot();
            let moved = Codon64::new(address).line_change(line).unwrap();
            assert_eq!(after["living_form"]["codon"]["address"], moved.address());
            assert_eq!(after["living_form"]["hexagram_id"], moved.hexagram_id());
            let site = Codon64::site_of_line(line);
            for i in 0..3 {
                let (b, a) = (
                    &before["living_form"]["folds"][i],
                    &after["living_form"]["folds"][i],
                );
                if i != site {
                    assert_eq!(a, b, "line {line} of {address} must leave fold {i} alone");
                    continue;
                }
                // The even line of a site is its polarity, the odd its mobility.
                let (kept, turned) = if line % 2 == 0 {
                    ("mobility", "polarity")
                } else {
                    ("polarity", "mobility")
                };
                assert_eq!(a[kept], b[kept]);
                assert_ne!(a[turned], b[turned]);
                assert_ne!(a["base"], b["base"]);
            }
            // The cell follows the hexagram, under the matrix that stayed.
            assert_cell(
                &after["generation"]["cell"],
                &generate(MatrixFamily::MovingResting, moved.hexagram_id()),
            );
            assert_eq!(after["generation"]["matrix"]["family"], "moving-resting");
            assert_eq!(after["generation"]["hexagram_id"], moved.hexagram_id());
        }
    }
}

#[test]
fn the_generation_cell_is_ql_generate_for_every_hexagram_and_matrix() {
    for address in 0..64u8 {
        for axis in 0..3usize {
            let s = state(address, 0, axis as u8);
            let g = s.snapshot()["generation"].clone();
            assert_eq!(g["law_ref"], POLE_GENERATION_LAW_REF);
            assert_eq!(g["hexagram_id"], address);
            assert_eq!(g["matrix"]["family"], family_str(family_at(axis)));
            assert_cell(&g["cell"], &generate(family_at(axis), address));
            // The other two families are in the reading, in family order, so
            // a Pairing change is a reading and not a recompute in the shell.
            let cells = g["cells"].as_array().unwrap();
            assert_eq!(cells.len(), 3);
            for (i, cell) in cells.iter().enumerate() {
                assert_cell(cell, &generate(family_at(i), address));
            }
            assert_eq!(g["cell"], cells[axis]);
        }
    }
}

#[test]
fn a_non_dual_cell_names_one_codon_and_each_matrix_has_sixteen() {
    for family in MatrixFamily::ALL {
        let mut non_dual = 0;
        for hexagram in 0..64u8 {
            let s = state(hexagram, 0, family_index(family));
            let cell = s.snapshot()["generation"]["cell"].clone();
            let lower = cell["lower_pair"]["sequence"].as_str().unwrap();
            let upper = cell["upper_pair"]["sequence"].as_str().unwrap();
            // Manual §22.3: non-dual when the lower pair ends with the base
            // that starts the upper pair.
            let rule = lower.as_bytes()[1] == upper.as_bytes()[0];
            assert_eq!(cell["non_dual"], rule, "{family:?} {hexagram}");
            if rule {
                non_dual += 1;
                assert_eq!(cell["positive"], cell["negative"]);
                assert_eq!(cell["singlet"], cell["positive"]);
                assert_ne!(cell["class"], "dual");
            } else {
                assert!(cell["singlet"].is_null());
                assert_ne!(cell["positive"]["address"], cell["negative"]["address"]);
                assert_eq!(cell["class"], "dual");
            }
            // Positive: first base of the lower pair and the whole upper
            // pair. Negative: the whole lower pair and the last base of the
            // upper pair.
            assert_eq!(
                cell["positive"]["sequence"].as_str().unwrap(),
                format!("{}{}", &lower[..1], upper)
            );
            assert_eq!(
                cell["negative"]["sequence"].as_str().unwrap(),
                format!("{}{}", lower, &upper[1..])
            );
        }
        assert_eq!(non_dual, 16, "{family:?}");
    }
    // The four mirror trigrams take TT, AA, CC, GG in every matrix: Qian,
    // Kun, Kan and Li are the hexagrams 63, 0, 18 (Kan over Kan) and 45.
    for family in MatrixFamily::ALL {
        for (hexagram, base) in [
            (0b111_111u8, "T"),
            (0, "A"),
            (0b010_010, "C"),
            (0b101_101, "G"),
        ] {
            let s = state(hexagram, 0, family_index(family));
            let cell = s.snapshot()["generation"]["cell"].clone();
            assert_eq!(cell["lower_pair"]["sequence"], format!("{base}{base}"));
            assert_eq!(cell["upper_pair"]["sequence"], format!("{base}{base}"));
            assert_eq!(cell["singlet"]["sequence"], format!("{base}{base}{base}"));
        }
    }
}

fn family_index(family: MatrixFamily) -> u8 {
    MatrixFamily::ALL.iter().position(|f| *f == family).unwrap() as u8
}

#[test]
fn apply_matrix_turns_the_form_and_the_cell_follows_the_new_form() {
    for (address, family_number) in (0..64u8).flat_map(|a| (0..3u8).map(move |f| (a, f))) {
        let family = family_at(family_number as usize);
        let mut s = state(address, 0, 0);
        apply(
            &mut s,
            vec![M3Operation::ApplyMatrix {
                family: family_number,
            }],
        );
        let v = s.snapshot();
        let turned = family.apply_to_codon(Codon64::new(address));
        assert_eq!(v["form"]["address"], turned.address());
        assert_eq!(v["living_form"]["codon"]["address"], turned.address());
        assert_eq!(v["living_form"]["matrix"]["family"], family_str(family));
        assert_eq!(v["living_form"]["matrix"]["axis"], axis_str(family.axis()));
        assert_eq!(v["generation"]["matrix"]["family"], family_str(family));
        assert_eq!(v["law"]["matrix"]["family"], family_str(family));
        assert_cell(
            &v["generation"]["cell"],
            &generate(family, turned.hexagram_id()),
        );
    }
}

#[test]
fn set_matrix_chooses_the_reading_matrix_without_turning_the_form() {
    for address in [0u8, 5, 7, 18, 45, 56, 63] {
        for family_number in 0..3u8 {
            let family = family_at(family_number as usize);
            let mut s = state(address, 0, 0);
            let before = s.snapshot();
            let receipt = apply(
                &mut s,
                vec![M3Operation::SetMatrix {
                    family: family_number,
                }],
            );
            let after = s.snapshot();
            assert_eq!(receipt.after_generation, receipt.before_generation + 1);
            // The form does not move.
            assert_eq!(after["form"]["address"], before["form"]["address"]);
            assert_eq!(
                after["living_form"]["folds"],
                before["living_form"]["folds"]
            );
            assert_eq!(after["living_form"]["pose"], before["living_form"]["pose"]);
            assert_eq!(after["generation"]["cells"], before["generation"]["cells"]);
            // The chosen matrix does.
            assert_eq!(after["generation"]["matrix"]["family"], family_str(family));
            assert_eq!(after["form"]["matrix_axis"], family_number);
            assert_cell(&after["generation"]["cell"], &generate(family, address));
            assert_eq!(after["law"]["matrix"]["axis"], axis_str(family.axis()));
        }
    }
    let mut s = state(7, 0, 0);
    let before = s.snapshot();
    let bad = command(&s, vec![M3Operation::SetMatrix { family: 3 }]);
    assert!(s.apply(bad).is_err());
    assert_eq!(s.snapshot(), before);
    let wire: M3Operation =
        serde_json::from_value(json!({"operation":"set-matrix","family":2})).unwrap();
    assert_eq!(wire, M3Operation::SetMatrix { family: 2 });
}

#[test]
fn set_pose_moves_the_pose_and_the_law_but_not_the_cell() {
    for address in 0..64u8 {
        let codon = Codon64::new(address);
        for pose in 0..codon.rotational_state_count() {
            let mut s = state(address, 0, 2);
            let before = s.snapshot();
            apply(&mut s, vec![M3Operation::SetPose { pose }]);
            let after = s.snapshot();
            assert_eq!(after["living_form"]["pose"]["index"], pose);
            assert_eq!(
                after["living_form"]["pose"]["ordinal"],
                RotationalPose::new(codon, pose).unwrap().ordinal()
            );
            assert_eq!(
                after["living_form"]["pose"]["state_count"],
                codon.rotational_state_count()
            );
            for slot in after["law"]["pose_map"]["slots"].as_array().unwrap() {
                assert_eq!(slot["current"], slot["slot"] == pose);
            }
            assert_eq!(
                after["living_form"]["folds"],
                before["living_form"]["folds"]
            );
            assert_eq!(after["generation"], before["generation"]);
        }
    }
}

#[test]
fn the_law_is_the_matrix_seed_and_pose_map_ql_reads_with() {
    for address in 0..64u8 {
        let codon = Codon64::new(address);
        let s = state(address, 0, 1);
        let law = s.snapshot()["law"].clone();
        assert_eq!(law["matrix"]["family"], "moving-resting");
        assert_eq!(law["matrix"]["axis"], "j");
        assert_eq!(law["matrix"]["number"], 2);
        // Seed: both laws, each with the codon's seed quaternion.
        assert_eq!(law["seed"]["in_force"]["law"], QuatSeed::default().name());
        assert_eq!(law["seed"]["in_force"]["fr"], QuatSeed::default().fr());
        let choices = law["seed"]["choices"].as_array().unwrap();
        assert_eq!(choices.len(), 2);
        for (choice, seed) in choices
            .iter()
            .zip([QuatSeed::SumDifferenceMod, QuatSeed::FourCharge])
        {
            let q = seed.quat(codon);
            assert_eq!(choice["law"], seed.name());
            assert_eq!(choice["fr"], seed.fr());
            assert_eq!(choice["quaternion_wxyz"], json!([q.w, q.x, q.y, q.z]));
        }
        // Pose map: the eight ranked slots of the codon.
        assert_eq!(law["pose_map"]["in_force"], PoseMap::default().name());
        assert_eq!(
            law["pose_map"]["choices"],
            json!([PoseMap::RawBin.name(), PoseMap::LawfulSlot.name()])
        );
        assert_eq!(
            law["pose_map"]["state_count"],
            codon.rotational_state_count()
        );
        let ranked = generate_rotational_states(codon);
        let slots = law["pose_map"]["slots"].as_array().unwrap();
        assert_eq!(slots.len(), 8);
        for (slot, entry) in slots.iter().enumerate() {
            let slot = slot as u8;
            let lawful = slot < codon.rotational_state_count();
            assert_eq!(entry["slot"], slot);
            assert_eq!(entry["lawful"], lawful);
            match RotationalPose::new(codon, slot) {
                Ok(pose) => assert_eq!(entry["ordinal"], pose.ordinal()),
                Err(_) => assert!(entry["ordinal"].is_null()),
            }
            assert_eq!(
                entry["rotation_degrees"],
                ranked[slot as usize].rotation_degrees
            );
            assert_eq!(entry["rotation_degrees"], 45 * u16::from(slot));
            assert_eq!(
                entry["valence"],
                match ranked[slot as usize].polarity {
                    RotationalPolarity::Negative => "negative",
                    RotationalPolarity::Positive => "positive",
                }
            );
            assert_eq!(
                entry["resulting_codon"],
                ranked[slot as usize].resulting_codon.address()
            );
            assert_eq!(
                entry["rotational_value"],
                ranked[slot as usize].rotational_value
            );
            assert_eq!(entry["bipolar"], ranked[slot as usize].is_non_dual);
            let (lawful_slot, collapsed) = PoseMap::LawfulSlot.slot(codon, slot);
            assert_eq!(entry["lawful_slot"], lawful_slot);
            assert_eq!(entry["collapsed_by_lawful_slot"], collapsed);
            assert_eq!(entry["current"], slot == 0);
        }
        // A non-dual codon has seven poses; its eighth slot is not lawful and
        // the lawful-slot map moves it to its mirror, the first.
        if codon.rotational_state_count() == 7 {
            assert_eq!(slots[7]["lawful"], false);
            assert!(slots[7]["ordinal"].is_null());
            assert_eq!(slots[7]["lawful_slot"], 0);
            assert_eq!(slots[7]["collapsed_by_lawful_slot"], true);
        } else {
            assert_eq!(slots[7]["lawful"], true);
            assert_eq!(slots[7]["collapsed_by_lawful_slot"], false);
        }
    }
}

#[test]
fn a_cell_at_another_hexagram_is_read_only_and_equals_the_form_there() {
    let s = state(7, 3, 1);
    let before = s.snapshot();
    for hexagram in 0..64u8 {
        for family_number in 0..3u8 {
            let asked = s.generation_at(Some(family_number), hexagram).unwrap();
            // The same reading a form standing on that hexagram carries.
            let there = state(hexagram, 0, family_number).snapshot();
            assert_eq!(asked, there["generation"]);
            assert_cell(
                &asked["cell"],
                &generate(family_at(family_number as usize), hexagram),
            );
        }
        // Without a family it reads under the matrix the state chose.
        let default = s.generation_at(None, hexagram).unwrap();
        assert_eq!(default["matrix"]["family"], "moving-resting");
        assert_eq!(default, s.generation_at(Some(1), hexagram).unwrap());
    }
    assert!(s.generation_at(Some(3), 0).is_err());
    assert!(s.generation_at(None, 64).is_err());
    // Asking moves nothing.
    assert_eq!(s.snapshot(), before);
    assert_eq!(s.generation(), 7);
}
