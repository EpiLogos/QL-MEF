//! M2-1′, the lens's material (M2′ seed §2.M2-1′; DLW manual §20.14 Modes: "The lens in force gives the surface its material"; O-I #854):
//! one material for each lens and position, the same every time; a lens and its partner (Lx and Lx′, seed invariant 2) show opposite
//! surfaces; the material is display-only and never moves a note.
use ql_mef::LensId;
use ql_mef::continuous::scene_field::{
    ShapeBasis, Tuning, complete, default_event, default_material, lens_material_of,
};
use ql_mef::m2_engine::MaterialFibre;
use ql_mef::m2_material::{MATERIAL_STANDING, lens_material};

#[test]
fn each_lens_and_position_gives_one_material_and_its_partner_the_opposite_surface() {
    for lens in LensId::ALL {
        for position in 0..6 {
            let material = lens_material(lens, position).unwrap();
            assert_eq!(
                material,
                lens_material(lens, position).unwrap(),
                "the same for the same lens and position"
            );
            let partner = lens_material(lens.conjugate_twin(), position).unwrap();
            assert_eq!(material.lens, lens.code());
            assert_eq!(material.partner, lens.conjugate_twin().code());
            assert_eq!(partner.partner, lens.code());
            assert_eq!(
                material.valence, -partner.valence,
                "{lens}: opposite valence"
            );
            let surface = |m: &ql_mef::m2_material::LensMaterial| {
                vec![
                    m.colour_rgb[0],
                    m.colour_rgb[1],
                    m.colour_rgb[2],
                    m.roughness,
                    m.elasticity,
                    m.particle_affinity,
                    m.tension,
                ]
            };
            for (a, b) in surface(&material).iter().zip(surface(&partner)) {
                assert!(
                    (0.0..=1.0).contains(a),
                    "{lens}@{position}: {a} within 0..1"
                );
                assert!(
                    (a + b - 1.0).abs() < 1e-12,
                    "{lens}@{position}: the partner's surface is the mirror ({a} vs {b})"
                );
            }
            assert_eq!(material.standing, MATERIAL_STANDING);
        }
    }
    // Every lens and position is its own surface.
    let mut seen = std::collections::BTreeSet::new();
    for lens in LensId::ALL {
        for position in 0..6 {
            let m = lens_material(lens, position).unwrap();
            assert!(
                seen.insert(format!(
                    "{:.9?}{:.9}{:.9}",
                    m.colour_rgb, m.roughness, m.tension
                )),
                "{lens}@{position} repeats a surface"
            );
        }
    }
    assert!(
        lens_material(LensId::L0, 6)
            .unwrap_err()
            .contains("position")
    );
}

#[test]
fn changing_the_lens_changes_the_surface_material_and_leaves_the_notes_alone() {
    let read = |lens12: u8, tuning: Tuning| {
        let mut event = default_event().unwrap();
        event.m1.lens12 = lens12;
        let (_, basis) =
            complete(&event, &default_material(), MaterialFibre::Earth, tuning).unwrap();
        let notes: Vec<f64> = ShapeBasis::from_basis(&basis)
            .unwrap()
            .sounding(tuning)
            .map(|v| v.frequency_hz)
            .collect();
        (lens_material_of(&basis), notes)
    };
    // The default event stands under L5′ at tick 7: position 1.
    let (material, sky_notes) = read(11, Tuning::Sky);
    assert_eq!(material["lens"], "L5'");
    assert_eq!(material["position"], 1);
    assert_eq!(
        material,
        serde_json::to_value(lens_material(LensId::L5Prime, 1).unwrap()).unwrap()
    );
    // Under Sky the planets are the voices: a lens and its partner, or any other lens, leave every note where it was.
    for lens12 in [10, 4, 0] {
        let (other, notes) = read(lens12, Tuning::Sky);
        assert_ne!(other, material, "lens12 {lens12}: the surface changes");
        assert_eq!(notes, sky_notes, "lens12 {lens12}: the notes stay");
    }
    // The material is display-only: whatever the tuning, the same lens gives the same surface.
    assert_eq!(read(11, Tuning::Logic).0, material);
}
