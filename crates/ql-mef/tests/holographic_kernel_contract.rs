use ql_core::{
    CanonicalCrossPass, D2CrossPassKind, HOLOGRAPHIC_KERNEL_CONTRACT_VERSION,
    HOLOGRAPHIC_KERNEL_POINTER_WEB_BLOB, HOLOGRAPHIC_KERNEL_REFERENCE_REVISION, KernelRelationId,
    PAIRING_GRAMMAR_VERSION, QlAddress, QlFace, QlFamily, QlPosition, RelationFamily,
    canonical_cross_pass_d1, canonical_cross_pass_d2, canonical_cross_pass_d3,
};
use ql_mef::{
    CONTEXT_FRAME_GRAMMAR_VERSION, LensFace, LensId, MEF_REGISTRY_REVISION, MEF_ROTATION_VERSION,
    MefGrain, MefRotation, MefUnitFace, SublensRef, canonical_context_frame_progression,
};

const CONTRACT: &str = include_str!("../../../fixtures/kernel/holographic-kernel-contract-v1.tsv");

fn row(kind: &str, key: &str) -> Vec<&'static str> {
    CONTRACT
        .lines()
        .find_map(|line| {
            let columns: Vec<_> = line.split('\t').collect();
            (columns.first() == Some(&kind) && columns.get(1) == Some(&key)).then_some(columns)
        })
        .unwrap_or_else(|| panic!("missing kernel contract row {kind}/{key}"))
}

#[test]
fn shared_contract_versions_and_provenance_match_native_c_contract() {
    assert_eq!(
        row("meta", "contract-version")[2],
        HOLOGRAPHIC_KERNEL_CONTRACT_VERSION
    );
    assert_eq!(
        row("meta", "historical-reference-revision")[2],
        HOLOGRAPHIC_KERNEL_REFERENCE_REVISION
    );
    assert_eq!(
        row("meta", "historical-pointer-web-blob")[2],
        HOLOGRAPHIC_KERNEL_POINTER_WEB_BLOB
    );
    assert_eq!(
        row("meta", "rust-pairing-version")[2],
        PAIRING_GRAMMAR_VERSION
    );
    assert_eq!(
        row("meta", "rust-mef-rotation-version")[2],
        MEF_ROTATION_VERSION
    );
    assert_eq!(
        row("meta", "rust-context-frame-version")[2],
        CONTEXT_FRAME_GRAMMAR_VERSION
    );
}

#[test]
fn family_position_and_face_identities_are_shared_without_a_second_rust_address_type() {
    for family in QlFamily::ALL {
        let contract = row("family", family.code());
        assert_eq!(contract[2].parse::<u8>().unwrap(), family.value());
    }

    assert_eq!(row("face", QlFace::Direct.kernel_code())[2], "0");
    assert_eq!(row("face", QlFace::Conjugate.kernel_code())[2], "1");
    assert_eq!(QlFace::Direct.kernel_value(), 0);
    assert_eq!(QlFace::Conjugate.kernel_value(), 1);

    for position in 0..6 {
        let direct = QlAddress::sixfold(position, QlFace::Direct, 0).unwrap();
        let prime = direct.with_face(QlFace::Conjugate);
        assert_eq!(direct.position(), prime.position());
        assert_eq!(prime.face().kernel_code(), "prime");
    }
}

#[test]
fn stable_relation_ids_preserve_pair_cross_mirror_and_return_distinctions() {
    let expected = [
        (KernelRelationId::PositionIdentity, "position.identity"),
        (KernelRelationId::FamilySamePosition, "family.same-position"),
        (KernelRelationId::CrossSamePosition, "cross.same-position"),
        (KernelRelationId::PairA, "pair.A"),
        (KernelRelationId::PairB, "pair.B"),
        (KernelRelationId::PairC, "pair.C"),
        (KernelRelationId::CrossTransform, "cross.transform"),
        (KernelRelationId::CrossRequire, "cross.require"),
        (KernelRelationId::CrossComplete, "cross.complete"),
        (
            KernelRelationId::ConjugateInvarianceA,
            "conjugate-invariance.A",
        ),
        (
            KernelRelationId::ConjugateInvarianceB,
            "conjugate-invariance.B",
        ),
        (
            KernelRelationId::ConjugateInvarianceC,
            "conjugate-invariance.C",
        ),
        (KernelRelationId::MirrorComplement, "mirror.complement"),
        (KernelRelationId::PositionSuccessor, "position.successor"),
        (KernelRelationId::MobiusReturn, "return.mobius"),
        (KernelRelationId::LensAnchor, "lens.anchor"),
        (KernelRelationId::ContextFrame, "context-frame"),
        (KernelRelationId::Nesting, "nesting"),
        (KernelRelationId::Branching, "branching"),
        (KernelRelationId::SourceProvenance, "source.provenance"),
    ];
    assert_eq!(expected.len(), KernelRelationId::ALL.len());
    for (relation, fixture_key) in expected {
        assert_eq!(row("relation", fixture_key)[2], relation.as_str());
    }

    assert_ne!(KernelRelationId::PairC, KernelRelationId::MirrorComplement);
    assert_ne!(
        KernelRelationId::MirrorComplement,
        KernelRelationId::CrossComplete
    );
}

#[test]
fn q6_pairing_grammar_resolves_through_the_shared_kernel_operator_ids() {
    let families = [
        (
            RelationFamily::A,
            [(0, 1), (2, 3), (4, 5)],
            KernelRelationId::ConjugateInvarianceA,
        ),
        (
            RelationFamily::B,
            [(1, 2), (3, 4), (5, 0)],
            KernelRelationId::ConjugateInvarianceB,
        ),
        (
            RelationFamily::C,
            [(0, 5), (1, 4), (2, 3)],
            KernelRelationId::ConjugateInvarianceC,
        ),
    ];

    for (family, pairs, invariant_id) in families {
        assert_eq!(family.pairs(), pairs);
        let expected_pairs = pairs
            .map(|(left, right)| format!("{left}-{right}"))
            .join(",");
        assert_eq!(row("pair", family.as_str())[2], expected_pairs);

        let d3 = canonical_cross_pass_d3(family);
        assert_eq!(d3.kernel_relation_id(), invariant_id);
        match d3 {
            CanonicalCrossPass::D3 { pairs, .. } => {
                for pair in pairs {
                    assert!(
                        pair.iter()
                            .all(|coordinate| coordinate.face == QlFace::Conjugate)
                    );
                }
            }
            _ => unreachable!(),
        }
    }

    for value in 0..6 {
        let position = QlPosition::new(value).unwrap();
        let d1 = canonical_cross_pass_d1(position);
        assert_eq!(d1.kernel_relation_id(), KernelRelationId::CrossSamePosition);
        match d1 {
            CanonicalCrossPass::D1 { coordinates, .. } => {
                assert_eq!(coordinates[0].position, coordinates[1].position);
                assert_eq!(coordinates[0].face, QlFace::Direct);
                assert_eq!(coordinates[1].face, QlFace::Conjugate);
            }
            _ => unreachable!(),
        }

        let cases = [
            (
                D2CrossPassKind::Transform,
                KernelRelationId::CrossTransform,
                (value + 1) % 6,
            ),
            (
                D2CrossPassKind::Require,
                KernelRelationId::CrossRequire,
                (value + 5) % 6,
            ),
            (
                D2CrossPassKind::Complete,
                KernelRelationId::CrossComplete,
                5 - value,
            ),
        ];
        for (kind, relation, target) in cases {
            let d2 = canonical_cross_pass_d2(kind, position);
            assert_eq!(d2.kernel_relation_id(), relation);
            match d2 {
                CanonicalCrossPass::D2 { coordinates, .. } => {
                    assert_eq!(coordinates[0].position.value(), value);
                    assert_eq!(coordinates[0].face, QlFace::Direct);
                    assert_eq!(coordinates[1].position.value(), target);
                    assert_eq!(coordinates[1].face, QlFace::Conjugate);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn twelve_lenses_and_seventy_two_mef_addresses_share_the_kernel_face_and_position_field() {
    assert_eq!(row("mef", "lens-count")[2], "12");
    assert_eq!(row("mef", "local-positions-per-lens")[2], "6");
    assert_eq!(row("mef", "address-count")[2], "72");
    assert_eq!(
        row("mef", "registry-revision")[2],
        MEF_REGISTRY_REVISION.to_string()
    );

    let mut coordinates = 0;
    for lens in LensId::ALL {
        let kernel_face = lens.face().kernel_face();
        match lens.face() {
            LensFace::Day => assert_eq!(kernel_face, QlFace::Direct),
            LensFace::Night => assert_eq!(kernel_face, QlFace::Conjugate),
        }
        assert_eq!(lens.conjugate_twin().index(), lens.index());
        assert_eq!(
            lens.conjugate_twin().face().kernel_face(),
            kernel_face.conjugate()
        );

        for local in 0..6 {
            let position = QlPosition::new(local).unwrap();
            let rotation = MefRotation::new(lens, position);
            assert_eq!(
                rotation.absolute_position().value(),
                (lens.index() + local) % 6
            );

            let sublens = SublensRef::canonical(lens, local).unwrap();
            assert_eq!(
                sublens.to_string(),
                format!("mef:sublens:{}.{}@{}", lens, local, MEF_REGISTRY_REVISION)
            );
            coordinates += 1;
        }
    }
    assert_eq!(coordinates, 72);
}

#[test]
fn canonical_context_frames_resolve_on_the_same_mef_rotation_field() {
    let progression = canonical_context_frame_progression();
    assert_eq!(progression.len(), 7);

    for selection in progression {
        let contract = row("cf", selection.frame().code());
        assert_eq!(
            contract[2].parse::<u8>().unwrap(),
            selection.local_position().value()
        );
        let unit = match selection.unit_face() {
            MefUnitFace::Name => "name",
            MefUnitFace::Power => "power",
        };
        let grain = match selection.grain() {
            MefGrain::InnerFour => "inner-four",
            MefGrain::OuterTwo => "outer-two",
        };
        assert_eq!(contract[3], unit);
        assert_eq!(contract[4], grain);
        assert_eq!(contract[5], selection.frame().expression());

        for lens in LensId::ALL {
            let coordinate = selection.at_lens(lens).coordinate();
            assert_eq!(coordinate.lens(), lens);
            assert_eq!(coordinate.local_position(), selection.local_position());
            assert_eq!(
                coordinate.absolute_position().value(),
                (lens.index() + selection.local_position().value()) % 6
            );
            assert_eq!(coordinate.unit_face(), selection.unit_face());
            assert_eq!(coordinate.grain(), selection.grain());
        }
    }
}
