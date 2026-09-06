use ql_core::{
    CanonicalCrossPass, HOLOGRAPHIC_KERNEL_CONTRACT_VERSION, KernelRelationId, QlAddress, QlFace,
    QlFamily, QlPosition, RelationalSixfold, SixBySixField, WHOLE_ANCHOR_SYMBOL,
    canonical_cross_pass_d1,
};
use ql_mef::{
    CANONICAL_RATIOS, HarmonicRatio, KERNEL_FAMILY_RELATION, LensId,
    MUSICAL_DERIVATION_SOURCE_BLOB, MefRotation, SECOND_SPANDA_VERTICAL,
};

const CONTRACT: &str = include_str!("../../../fixtures/kernel/matheme-derivation-contract-v1.tsv");
const KERNEL_CONTRACT: &str =
    include_str!("../../../fixtures/kernel/holographic-kernel-contract-v1.tsv");

fn row<'a>(contract: &'a str, kind: &str, key: &str) -> Vec<&'a str> {
    contract
        .lines()
        .find_map(|line| {
            let columns: Vec<_> = line.split('\t').collect();
            (columns.first() == Some(&kind) && columns.get(1) == Some(&key)).then_some(columns)
        })
        .unwrap_or_else(|| panic!("missing contract row {kind}/{key}"))
}

fn value<'a>(contract: &'a str, kind: &str, key: &str) -> &'a str {
    row(contract, kind, key)[2]
}

fn kernel_value(kind: &str, key: &str) -> &'static str {
    value(KERNEL_CONTRACT, kind, key)
}

fn position(count: u8) -> QlPosition {
    QlPosition::new(count).expect("canonical positions are modulo six")
}

fn ratio(numerator: u32, denominator: u32) -> HarmonicRatio {
    HarmonicRatio::new(numerator, denominator).expect("canonical matheme ratios are non-zero")
}

#[test]
fn contract_meta_pins_the_zero_layer_to_the_kernel_contract_and_v3_source() {
    assert_eq!(value(CONTRACT, "meta", "contract-version"), "1.0.0");
    assert_eq!(value(CONTRACT, "meta", "layer"), "0");
    assert_eq!(
        value(CONTRACT, "meta", "kernel-contract"),
        "ql.holographic-kernel-contract/v1"
    );
    assert_eq!(
        value(CONTRACT, "meta", "kernel-contract-version"),
        HOLOGRAPHIC_KERNEL_CONTRACT_VERSION
    );
    assert_eq!(
        value(CONTRACT, "meta", "derivation-source"),
        "ql-musical-derivation-v3"
    );
    assert_eq!(
        value(CONTRACT, "meta", "derivation-source-blob"),
        MUSICAL_DERIVATION_SOURCE_BLOB
    );
}

#[test]
fn eq1_top_line_binds_hash_and_both_slash_directions_to_kernel_elements() {
    assert_eq!(value(CONTRACT, "eq1", "top-line"), "# / 0/1 <-> 1/0");

    let bedrock = row(CONTRACT, "eq1", "hash-bedrock");
    assert_eq!(bedrock[2], "#");
    assert_eq!(bedrock[3], "family");
    assert_eq!(bedrock[4], QlFamily::None.code());
    assert_eq!(kernel_value("family", "NONE"), "7");
    assert_eq!(QlFamily::None.value(), 7);

    let copula = row(CONTRACT, "eq1", "copula");
    assert_eq!(copula[2], WHOLE_ANCHOR_SYMBOL);
    assert_eq!(copula[3], "face");
    assert_eq!(copula[4], QlFace::Direct.kernel_code());
    assert_eq!(QlFace::Direct.kernel_value(), 0);

    let return_switch = row(CONTRACT, "eq1", "return-switch");
    assert_eq!(return_switch[2], "1/0");
    assert_eq!(return_switch[3], "face");
    assert_eq!(return_switch[4], QlFace::Conjugate.kernel_code());
    assert_eq!(QlFace::Conjugate.kernel_value(), 1);

    assert_eq!(QlFace::Direct.conjugate(), QlFace::Conjugate);
    assert_eq!(QlFace::Conjugate.conjugate(), QlFace::Direct);
}

#[test]
fn eq1_same_position_conjugation_is_one_two_coordinate_circuit() {
    let circuit = row(CONTRACT, "eq1", "circuit");
    assert_eq!(circuit[2], "n<->n'");
    assert_eq!(circuit[3], "relation");
    assert_eq!(circuit[4], KernelRelationId::CrossSamePosition.as_str());
    assert_eq!(circuit[4], kernel_value("relation", "cross.same-position"));
    assert_eq!(circuit[5], "coordinates");
    assert_eq!(circuit[6], "2");
    assert_eq!(circuit[7], "degrees");
    assert_eq!(circuit[8], "360");

    for index in 0..6 {
        let cross = canonical_cross_pass_d1(position(index));
        assert_eq!(
            cross.kernel_relation_id(),
            KernelRelationId::CrossSamePosition
        );
        assert_eq!(
            cross.operator_ref(),
            KernelRelationId::CrossSamePosition.as_str()
        );
        assert!(cross.derivation_ref().contains(":cross:D1:"));
        match cross {
            CanonicalCrossPass::D1 { coordinates, .. } => {
                assert_eq!(coordinates.len(), 2);
                assert_eq!(coordinates[0].position, position(index));
                assert_eq!(coordinates[0].face, QlFace::Direct);
                assert_eq!(coordinates[1].position, position(index));
                assert_eq!(coordinates[1].face, QlFace::Conjugate);
            }
            other => panic!("expected D1 circuit, got {other:?}"),
        }
    }
}

#[test]
fn eq1_double_beat_restores_identity_only_after_both_turns() {
    let double_beat = row(CONTRACT, "eq1", "double-beat");
    assert_eq!(double_beat[2], "n->n'->n");
    assert_eq!(double_beat[3], "turns");
    assert_eq!(double_beat[4], "2");

    let recognition = row(CONTRACT, "eq1", "recognition");
    assert_eq!(recognition[2], "degrees");
    let degrees: u32 = recognition[3].parse().unwrap();
    let circuit_degrees: u32 = row(CONTRACT, "eq1", "circuit")[8].parse().unwrap();
    let turns: u32 = double_beat[4].parse().unwrap();
    assert_eq!(degrees, circuit_degrees * turns);
    assert_eq!(recognition[4], "identity");
    assert_eq!(recognition[5], "n->n'->n==n");

    for face in [QlFace::Direct, QlFace::Conjugate] {
        assert_ne!(face.conjugate(), face);
        assert_eq!(face.conjugate().conjugate(), face);
    }

    for index in 0..6 {
        let anchor = QlAddress::sixfold(index, QlFace::Direct, 0).unwrap();
        let one_turn = anchor.with_face(anchor.face().conjugate());
        assert_ne!(one_turn, anchor);
        assert_eq!(one_turn.position(), anchor.position());
        let two_turns = one_turn.with_face(one_turn.face().conjugate());
        assert_eq!(two_turns, anchor);
    }
}

#[test]
fn eq1_sum_binds_both_directions_to_the_standing_whole() {
    let sum = row(CONTRACT, "eq1", "sum");
    assert_eq!(sum[2], "(0/1)+(1/0)");
    assert_eq!(sum[3], "1/1");
    assert_eq!(CANONICAL_RATIOS[0], ratio(1, 1));

    let sixfold = RelationalSixfold::canonical();
    assert_eq!(sixfold.return_anchor_symbol, WHOLE_ANCHOR_SYMBOL);
    for site in &sixfold.sites {
        assert_eq!(site.direct.position, site.conjugate.position);
    }

    assert_eq!(KERNEL_FAMILY_RELATION, KernelRelationId::FamilySamePosition);
    assert_eq!(
        kernel_value("relation", "family.same-position"),
        KernelRelationId::FamilySamePosition.as_str()
    );
}

#[test]
fn eq2_totality_decomposes_into_both_registers_of_the_sixfold() {
    let totality = row(CONTRACT, "eq2", "totality");
    assert_eq!(totality[2], "1/1");
    assert_eq!(totality[3], "100");
    assert_eq!(CANONICAL_RATIOS[0], ratio(1, 1));

    let decomposition = row(CONTRACT, "eq2", "decomposition");
    assert_eq!(decomposition[2], "100");
    assert_eq!(decomposition[3], "2^6+6^2");

    let binary = row(CONTRACT, "eq2", "binary-register");
    assert_eq!(binary[2], "2^6");
    let binary_cardinality: u32 = binary[3].parse().unwrap();
    assert_eq!(binary_cardinality, 2u32.pow(6));

    let self_register = row(CONTRACT, "eq2", "self-register");
    assert_eq!(self_register[2], "6^2");
    let self_cardinality: u32 = self_register[3].parse().unwrap();
    assert_eq!(
        self_cardinality,
        SixBySixField::canonical().addresses.len() as u32
    );

    assert_eq!(binary_cardinality + self_cardinality, 100);
}

#[test]
fn eq2_ratio_field_reduces_to_the_totality_over_the_position_hexad() {
    assert_eq!(ratio(64, 36), ratio(16, 9));
    assert_eq!(CANONICAL_RATIOS[5], ratio(16, 9));
    assert_eq!(
        CANONICAL_RATIOS[1].multiply(CANONICAL_RATIOS[1]),
        ratio(16, 9)
    );

    let ratio_field = row(CONTRACT, "eq2", "ratio-field");
    assert_eq!(ratio_field[2], "64/36");
    assert_eq!(ratio_field[3], "16/9");

    let prime_factorisation = row(CONTRACT, "eq2", "prime-factorisation");
    assert_eq!(prime_factorisation[2], "16/9");
    assert_eq!(prime_factorisation[3], "2^4/3^2");
    assert_eq!(2u32.pow(4), 16);
    assert_eq!(3u32.pow(2), 9);
    assert_eq!(2u32.pow(4) * 3u32.pow(2), 16 * 9);

    let factor_count = row(CONTRACT, "eq2", "prime-factor-count");
    assert_eq!(factor_count[2], "4+2");
    assert_eq!(factor_count[3], "6");
    assert_eq!(
        u32::from(SECOND_SPANDA_VERTICAL.0 + SECOND_SPANDA_VERTICAL.1),
        factor_count[3].parse::<u32>().unwrap()
    );

    let hexad = row(CONTRACT, "eq2", "position-hexad");
    assert_eq!(hexad[2], "6");
    assert_eq!(hexad[3], "registry");
    assert_eq!(hexad[4], "mef.local-positions-per-lens");
    assert_eq!(kernel_value("mef", "local-positions-per-lens"), "6");
    for index in 0..6 {
        assert_eq!(position(index).value(), index);
    }
}

#[test]
fn eq2_twelve_ring_is_the_doubled_hexad_at_the_octave_and_counts_the_field() {
    let ring = row(CONTRACT, "eq2", "ring");
    assert_eq!(ring[2], "6+6");
    assert_eq!(ring[3], "12");

    let sixfold = RelationalSixfold::canonical();
    assert_eq!(sixfold.sites.len(), 6);
    let doubled: usize = sixfold.sites.len() * 2;
    assert_eq!(doubled, 12);
    assert_eq!(value(CONTRACT, "eq2", "ring-faces"), "direct+prime");
    for site in &sixfold.sites {
        assert_eq!(site.direct.face, QlFace::Direct);
        assert_eq!(site.conjugate.face, QlFace::Conjugate);
    }

    assert_eq!(LensId::ALL.len(), 12);

    let octave = row(CONTRACT, "eq2", "ring-octave");
    assert_eq!(octave[2], "12:6");
    assert_eq!(octave[3], "2:1");
    assert_eq!(ratio(12, 6), CANONICAL_RATIOS[7]);

    let product = row(CONTRACT, "eq2", "ring-product");
    assert_eq!(product[2], "12x6");
    assert_eq!(product[3], "72");
    assert_eq!(product[4], "registry");
    assert_eq!(product[5], "mef.address-count");
    assert_eq!(kernel_value("mef", "address-count"), "72");

    let mut rotations = 0;
    for lens in LensId::ALL {
        for local in 0..6 {
            MefRotation::new(lens, position(local));
            rotations += 1;
        }
    }
    assert_eq!(rotations, 72);
}

#[test]
fn eq3_cardinalities_retain_the_one_and_sum_the_two_registries() {
    let retained_one = row(CONTRACT, "eq3", "retained-one");
    assert_eq!(retained_one[2], "1/1");
    assert_eq!(retained_one[3], "cardinality");
    assert_eq!(retained_one[4], "1");
    assert_eq!(CANONICAL_RATIOS[0].numerator(), 1);
    assert_eq!(CANONICAL_RATIOS[0], ratio(1, 1));

    let cardinality_sum = row(CONTRACT, "eq3", "cardinality-sum");
    assert_eq!(cardinality_sum[2], "1+64+72");
    assert_eq!(cardinality_sum[3], "137");

    let registry_addresses: u32 = kernel_value("mef", "address-count").parse().unwrap();
    let binary_register: u32 = row(CONTRACT, "eq2", "binary-register")[3].parse().unwrap();
    let retained: u32 = retained_one[4].parse().unwrap();
    assert_eq!(retained + binary_register + registry_addresses, 137);
}

#[test]
fn eq3_two_way_door_computes_exact_descent_and_ascent_through_the_epogdoon() {
    let descent = row(CONTRACT, "eq3", "door-descent");
    assert_eq!(descent[2], "8/9");
    let descent_from: u32 = descent[3].parse().unwrap();
    let descent_to: u32 = descent[4].parse().unwrap();

    let ascent = row(CONTRACT, "eq3", "door-ascent");
    assert_eq!(ascent[2], "9/8");
    let ascent_from: u32 = ascent[3].parse().unwrap();
    let ascent_to: u32 = ascent[4].parse().unwrap();

    let epogdoon = CANONICAL_RATIOS[6];
    assert_eq!(epogdoon, ratio(9, 8));
    assert_eq!(epogdoon.reciprocal(), ratio(8, 9));

    let registry_addresses: u32 = kernel_value("mef", "address-count").parse().unwrap();
    let binary_register: u32 = row(CONTRACT, "eq2", "binary-register")[3].parse().unwrap();
    assert_eq!(descent_from, registry_addresses);
    assert_eq!(descent_to, binary_register);
    assert_eq!(ascent_from, binary_register);
    assert_eq!(ascent_to, registry_addresses);

    assert_eq!(
        ratio(registry_addresses, 1).multiply(epogdoon.reciprocal()),
        ratio(binary_register, 1)
    );
    assert_eq!(
        ratio(binary_register, 1).multiply(epogdoon),
        ratio(registry_addresses, 1)
    );

    let through_door = row(CONTRACT, "eq3", "octave-through-door");
    assert_eq!(through_door[2], "16/9x9/8");
    assert_eq!(through_door[3], "2/1");
    assert_eq!(CANONICAL_RATIOS[5].multiply(epogdoon), CANONICAL_RATIOS[7]);
}

#[test]
fn matheme_rows_compose_only_existing_kernel_contract_primitives() {
    let mut relation_ids = 0;
    for line in CONTRACT.lines().skip(1) {
        for column in line.split('\t') {
            if column.starts_with("ql.kernel.") {
                assert!(
                    KERNEL_CONTRACT
                        .lines()
                        .any(|kernel_line| kernel_line.split('\t').any(|c| c == column)),
                    "matheme contract mints a second substrate relation: {column}"
                );
                relation_ids += 1;
            }
        }
    }
    assert!(relation_ids >= 1);

    for line in CONTRACT.lines().skip(1) {
        let columns: Vec<_> = line.split('\t').collect();
        for (index, column) in columns.iter().enumerate() {
            if *column == "registry" {
                let reference = columns
                    .get(index + 1)
                    .expect("registry reference carries its kernel contract key");
                let (registry_kind, registry_key) = reference
                    .split_once('.')
                    .expect("registry reference is <kind>.<key> over the kernel contract");
                assert_eq!(
                    kernel_value(registry_kind, registry_key)
                        .parse::<u32>()
                        .unwrap(),
                    columns[index - 1]
                        .parse::<u32>()
                        .expect("registry-bound value is numeric"),
                    "matheme value for {reference} disagrees with the kernel registry"
                );
            }
        }
    }
}
