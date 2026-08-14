use ql_mef::{LensFace, LensId};

#[test]
fn conjugate_twins_preserve_index_and_flip_face() {
    for lens in LensId::ALL {
        let twin = lens.conjugate_twin();
        assert_eq!(twin.index(), lens.index());
        assert_ne!(twin.face(), lens.face());
        assert_eq!(twin.conjugate_twin(), lens);
    }
}

#[test]
fn same_face_complements_are_sum_to_five_involutions() {
    for lens in LensId::ALL {
        let complement = lens.same_face_complement();
        assert_eq!(lens.index() + complement.index(), 5);
        assert_eq!(complement.face(), lens.face());
        assert_eq!(complement.square(), lens.square());
        assert_eq!(complement.same_face_complement(), lens);
    }
}

#[test]
fn mobius_partners_are_cross_face_sum_to_five_involutions() {
    for lens in LensId::ALL {
        let partner = lens.mobius_partner();
        assert_eq!(lens.index() + partner.index(), 5);
        assert_ne!(partner.face(), lens.face());
        assert_eq!(partner.square(), lens.square());
        assert_eq!(partner.mobius_partner(), lens);
    }
}

#[test]
fn settled_mobius_pairs_are_exact() {
    let pairs = [
        (LensId::L0, LensId::L5Prime),
        (LensId::L5, LensId::L0Prime),
        (LensId::L1, LensId::L4Prime),
        (LensId::L4, LensId::L1Prime),
        (LensId::L2, LensId::L3Prime),
        (LensId::L3, LensId::L2Prime),
    ];
    for (day, night) in pairs {
        assert_eq!(day.face(), LensFace::Day);
        assert_eq!(night.face(), LensFace::Night);
        assert_eq!(day.mobius_partner(), night);
        assert_eq!(night.mobius_partner(), day);
    }
}
