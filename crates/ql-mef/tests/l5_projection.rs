use ql_core::{KernelRelationId, QlFamily, VakFamily};
use ql_mef::l5_projection::{
    ARTICULATION_SQUARE, L5Office, L5_PROJECTION_VERSION, QL_KERNEL_NAME,
    lookup_context_frame_expression,
};
use ql_mef::{
    ContextFrameCut, ContextFrameId, LensId, MefSquare, MusicalBasis, SublensRef,
    all_sublens_definitions, cf_diatonic_cut, lens_definition,
};

#[test]
fn faithful_offices_project_existing_refs_without_replacing_source_labels() {
    assert_eq!(QL_KERNEL_NAME, "QL Kernel");
    assert_eq!(L5_PROJECTION_VERSION, "ql.kernel.l5-projection/v1");
    let fixture = include_str!("../../../fixtures/kernel/l5-projection-v1.tsv");
    let rows: Vec<_> = fixture.lines().skip(1).collect();
    assert_eq!(rows.len(), L5Office::ALL.len());
    for (office, row) in L5Office::ALL.into_iter().zip(rows) {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 4);
        assert_eq!(office.local_position().to_string(), fields[0]);
        assert_eq!(office.name(), fields[1]);
        assert_eq!(office.reference().to_string(), fields[2]);
        assert_eq!(format!("{:?}", office.context_frame_role()), fields[3]);
        assert_eq!(fields[2].parse::<SublensRef>().unwrap(), office.reference());
        assert_eq!(
            office.source_label(),
            lens_definition(LensId::L5).sublens_labels()[office.local_position() as usize]
        );
    }
    assert_eq!(lens_definition(LensId::L5).name(), "Para Vāk");
    assert_eq!(L5Office::Harmonics.source_label(), "Paśyantī");
    assert_ne!(L5Office::Harmonics.name(), L5Office::Harmonics.source_label());
}

#[test]
fn articulation_is_the_existing_four_lens_square() {
    let actual: Vec<_> = LensId::ALL
        .into_iter()
        .filter(|lens| lens.square() == MefSquare::Articulation)
        .collect();
    assert_eq!(actual.len(), ARTICULATION_SQUARE.len());
    for lens in ARTICULATION_SQUARE {
        assert!(actual.contains(&lens));
        assert!(ARTICULATION_SQUARE.contains(&lens.conjugate_twin()));
        assert!(ARTICULATION_SQUARE.contains(&lens.same_face_complement()));
        assert!(ARTICULATION_SQUARE.contains(&lens.mobius_partner()));
    }
    assert_eq!(ARTICULATION_SQUARE[3], LensId::L5);
}

#[test]
fn the_projection_does_not_grow_or_reindex_the_mef_manifold() {
    assert_eq!(LensId::ALL.len(), 12);
    let sublenses: Vec<_> = all_sublens_definitions().collect();
    assert_eq!(sublenses.len(), 72);
    assert!(sublenses.iter().all(Result::is_ok));
    for lens in LensId::ALL {
        for local in 0..6 {
            let reference = SublensRef::canonical(lens, local).unwrap();
            assert_eq!(
                reference.rotation().absolute_position().value(),
                (lens.index() + local) % 6
            );
        }
    }
    assert_eq!(L5Office::Harmonics.local_position(), 2);
    let harmonic_rotation = L5Office::Harmonics.reference().rotation();
    assert_eq!(harmonic_rotation.absolute_position().value(), 1);
}

#[test]
fn harmonic_cuts_and_vak_keep_the_existing_context_frame_identity() {
    assert_eq!(VakFamily::Cf.relation_id(), KernelRelationId::ContextFrame);
    for basis in MusicalBasis::ALL {
        for lens in LensId::ALL {
            let harmonic = cf_diatonic_cut(basis, lens);
            let canonical = ContextFrameCut::canonical(lens);
            assert_eq!(harmonic.frames, ContextFrameId::ALL);
            for (index, selection) in canonical.selected().iter().enumerate() {
                assert_eq!(harmonic.frames[index], selection.frame());
                assert_eq!(harmonic.forms[index], selection.coordinate().unit_face());
                assert_eq!(
                    lookup_context_frame_expression(harmonic.frames[index].expression()),
                    Some(selection.frame())
                );
            }
        }
    }
}

#[test]
fn historical_or_unparsed_expressions_do_not_invent_context_frames() {
    for expression in ["(4/5/0)", "CF8", "(0/1) + (5/0)", " (0/1)", ""] {
        assert_eq!(lookup_context_frame_expression(expression), None);
    }
    for frame in ContextFrameId::ALL {
        assert_eq!(
            lookup_context_frame_expression(frame.expression()),
            Some(frame)
        );
    }
}

#[test]
fn root_is_the_existing_six_family_heads_not_a_seventh_family() {
    let heads: Vec<_> = QlFamily::ALL
        .into_iter()
        .filter(|family| *family != QlFamily::None)
        .collect();
    assert_eq!(
        heads,
        [
            QlFamily::C,
            QlFamily::P,
            QlFamily::L,
            QlFamily::S,
            QlFamily::T,
            QlFamily::M,
        ]
    );
}
