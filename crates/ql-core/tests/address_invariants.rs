use ql_core::{QlAddress, QlFace, QlFormRef, QlPosition};

#[test]
fn form_refs_are_versioned_and_stable() {
    assert_eq!(QlFormRef::SIXFOLD_V1.to_string(), "qlform:sixfold@1");
    assert_eq!(
        QlFormRef::FOUR_PLUS_TWO_V1.to_string(),
        "qlform:four-plus-two@1"
    );
    assert_eq!(
        QlFormRef::DIRECT_CONJUGATE_V1.to_string(),
        "qlform:direct-conjugate@1"
    );
}

#[test]
fn canonical_addresses_round_trip_exhaustively_over_finite_faces_and_positions() {
    for position in 0..=5 {
        for face in [QlFace::Direct, QlFace::Conjugate] {
            for depth in [0, 1, 3, 255] {
                let address = QlAddress::sixfold(position, face, depth).expect("valid address");
                let encoded = address.to_string();
                let decoded = encoded
                    .parse::<QlAddress>()
                    .expect("canonical address parses");
                assert_eq!(decoded, address);
            }
        }
    }
}

#[test]
fn negative_fixture_corpus_fails_visibly() {
    let fixtures = include_str!("../../../fixtures/q1/invalid-addresses.txt");
    for address in fixtures.lines().filter(|line| !line.trim().is_empty()) {
        assert!(
            address.parse::<QlAddress>().is_err(),
            "fixture unexpectedly parsed: {address}"
        );
    }
}

#[test]
fn non_sixfold_forms_are_not_silently_accepted_as_address_frames() {
    let error = QlAddress::new(
        QlFormRef::FOUR_PLUS_TWO_V1,
        QlPosition::new(2).expect("P2"),
        QlFace::Direct,
        0,
    )
    .expect_err("four-plus-two is not the Q1 address frame");
    assert_eq!(error.code(), "UNSUPPORTED_ADDRESS_FRAME");
}
