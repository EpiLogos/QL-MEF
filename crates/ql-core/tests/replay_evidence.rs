use ql_core::{KERNEL_VERSION, QlAddress, QlFace, QlOperator, SCHEMA_VERSION, apply_operator};

#[test]
fn deterministic_results_carry_versioned_replay_provenance() {
    let input = QlAddress::sixfold(1, QlFace::Direct, 0).expect("valid address");
    let result = apply_operator(QlOperator::ComplementAddress, input);
    assert_eq!(result.provenance.schema_version, SCHEMA_VERSION);
    assert_eq!(result.provenance.kernel_version, KERNEL_VERSION);
    assert_eq!(result.provenance.operation, "complement-address");
    assert_eq!(result.provenance.input, "qladdr:sixfold@1/direct/P1/d0");
    assert_eq!(result.provenance.output, "qladdr:sixfold@1/direct/P4/d0");
}

#[test]
fn client_product_nouns_are_not_ql_positions() {
    assert!(
        "qladdr:sixfold@1/direct/Project/d0"
            .parse::<QlAddress>()
            .is_err()
    );
}
