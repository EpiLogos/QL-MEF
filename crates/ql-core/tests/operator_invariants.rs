use ql_core::{
    apply_operator, FourPlusTwoClass, OperatorValue, QlAddress, QlFace, QlOperator,
};

fn address_value(value: OperatorValue) -> QlAddress {
    match value {
        OperatorValue::Address(address) => address,
        OperatorValue::FourPlusTwo(_) => panic!("expected address result"),
    }
}

#[test]
fn conjugation_is_an_involution_and_preserves_position_and_depth() {
    for position in 0..=5 {
        for face in [QlFace::Direct, QlFace::Conjugate] {
            for depth in [0, 1, 21, u16::MAX as u32] {
                let initial = QlAddress::sixfold(position, face, depth).expect("valid address");
                let once = address_value(apply_operator(QlOperator::ConjugateAddress, initial).value);
                let twice = address_value(apply_operator(QlOperator::ConjugateAddress, once).value);
                assert_eq!(once.position(), initial.position());
                assert_eq!(once.depth(), initial.depth());
                assert_eq!(twice, initial);
            }
        }
    }
}

#[test]
fn complement_is_sum_to_five_and_an_involution() {
    for position in 0..=5 {
        let initial = QlAddress::sixfold(position, QlFace::Direct, 0).expect("valid address");
        let once = address_value(apply_operator(QlOperator::ComplementAddress, initial).value);
        let twice = address_value(apply_operator(QlOperator::ComplementAddress, once).value);
        assert_eq!(position + once.position().value(), 5);
        assert_eq!(twice, initial);
    }
}

#[test]
fn four_plus_two_partition_is_exact() {
    for position in 0..=5 {
        let address = QlAddress::sixfold(position, QlFace::Direct, 0).expect("valid address");
        let value = apply_operator(QlOperator::ClassifyFourPlusTwo, address).value;
        let expected = if matches!(position, 0 | 5) {
            FourPlusTwoClass::Implicate
        } else {
            FourPlusTwoClass::Explicate
        };
        assert_eq!(value, OperatorValue::FourPlusTwo(expected));
    }
}

#[test]
fn deterministic_replay_is_byte_for_byte_equal_at_the_value_level() {
    let address = QlAddress::sixfold(3, QlFace::Conjugate, 8).expect("valid address");
    for operator in [
        QlOperator::ConjugateAddress,
        QlOperator::ComplementAddress,
        QlOperator::ClassifyFourPlusTwo,
    ] {
        assert_eq!(apply_operator(operator, address), apply_operator(operator, address));
    }
}

#[test]
fn unsupported_research_operator_fails_instead_of_filling_symmetry() {
    let error = "harmonic-64-state"
        .parse::<QlOperator>()
        .expect_err("research operator must not be promoted in Q1");
    assert_eq!(error.code(), "UNKNOWN_OPERATOR");
}
