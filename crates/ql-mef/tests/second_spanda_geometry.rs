use ql_mef::{
    MATHEME_DERIVATION_CONTRACT_VERSION, binary_register, decadic_projection,
    decomposed_totality, quaternary_cubic_register, self_register,
};

#[test]
fn second_spanda_exposes_the_complete_canonical_equation() {
    assert_eq!(MATHEME_DERIVATION_CONTRACT_VERSION, "1.2.0");
    assert_eq!(binary_register(), 64);
    assert_eq!(quaternary_cubic_register(), 64);
    assert_eq!(self_register(), 36);
    assert_eq!(decomposed_totality(), 100);
    assert_eq!(decadic_projection(), 100);
}

#[test]
fn binary_and_quaternary_reads_are_one_m3_field() {
    assert_eq!(binary_register(), quaternary_cubic_register());
    assert_eq!(binary_register() + self_register(), decomposed_totality());
}
