//! Every codec decodes every stream kind exactly, including the edges: an
//! empty stream, a single symbol, and noise that defeats every prior.

use ql_codec::codecs::{
    CellState, FrameTied, GenerationLaw, KernelOp, OperatorGrammar, OrderK, PairContext, XorMask,
    lens_scales,
};
use ql_codec::coder::{Encoder, Freq, Io};
use ql_codec::source::{self, Rng};
use ql_codec::{Codec, compress, decompress};

fn round_trip(codec: &dyn Codec, symbols: &[u16]) -> usize {
    let packed = compress(codec, symbols);
    assert_eq!(decompress(codec, &packed), symbols, "{}", codec.name());
    packed.len()
}

#[test]
fn the_range_coder_round_trips_skewed_and_uniform_symbols() {
    let mut rng = Rng::new(7);
    for alphabet in [2usize, 7, 64, 256] {
        let symbols: Vec<u16> = (0..20_000)
            .map(|i| {
                if i % 3 == 0 {
                    rng.below(alphabet as u64) as u16
                } else {
                    0
                }
            })
            .collect();
        round_trip(&OrderK { k: 0, alphabet }, &symbols);
        round_trip(&OrderK { k: 2, alphabet }, &symbols);
    }
}

#[test]
fn empty_and_single_symbol_streams_round_trip() {
    for codec in [
        Box::new(OrderK { k: 1, alphabet: 64 }) as Box<dyn Codec>,
        Box::new(GenerationLaw::default()),
        Box::new(OperatorGrammar::default()),
        Box::new(XorMask),
        Box::new(PairContext),
        Box::new(CellState::default()),
    ] {
        round_trip(codec.as_ref(), &[]);
        round_trip(codec.as_ref(), &[17]);
    }
    round_trip(&FrameTied::new(true), &[]);
    round_trip(&FrameTied::new(true), &[5]);
}

#[test]
fn every_codec_round_trips_its_streams_at_every_noise_rate() {
    for eps in [0.0, 0.3, 1.0] {
        let pairs = source::generation_pairs(400, eps);
        for codec in [
            Box::new(PairContext) as Box<dyn Codec>,
            Box::new(GenerationLaw::default()),
            Box::new(CellState::default()),
        ] {
            round_trip(codec.as_ref(), &pairs.symbols);
        }
        let walk = source::operator_walk(800, eps);
        round_trip(&OperatorGrammar::default(), &walk.symbols);
        round_trip(&XorMask, &walk.symbols);
        let melody = source::modal_melody(800, eps);
        round_trip(&FrameTied::new(true), &melody.symbols);
        round_trip(&FrameTied::new(false), &melody.symbols);
    }
    round_trip(
        &OperatorGrammar::default(),
        &source::recorded_clock().symbols,
    );
}

#[test]
fn the_generation_law_codes_a_clean_conjugate_near_its_one_bit_tax() {
    let pairs = source::generation_pairs(20_000, 0.0);
    let law = round_trip(&GenerationLaw::default(), &pairs.symbols);
    let generic = round_trip(&PairContext, &pairs.symbols);
    assert!(law < generic, "law {law} vs pair-context {generic}");
    let cell = round_trip(&CellState::default(), &pairs.symbols);
    assert!(cell < law, "cell-state {cell} vs codon law {law}");
}

#[test]
fn the_operator_grammar_applies_the_kernel_operators() {
    let ops = KernelOp::all();
    assert_eq!(ops.len(), 12);
    for hexagram in 0u8..64 {
        assert_eq!(KernelOp::Identity.apply(hexagram), hexagram);
        assert_eq!(
            KernelOp::Complement.apply(KernelOp::Complement.apply(hexagram)),
            hexagram
        );
    }
}

#[test]
fn the_twelve_lens_cuts_are_seven_note_scales() {
    for scale in lens_scales() {
        let mut distinct = scale.to_vec();
        distinct.dedup();
        assert_eq!(distinct.len(), 7);
        assert!(scale.iter().all(|p| *p < 12));
    }
}

#[test]
fn a_prior_is_known_to_the_decoder_without_being_sent() {
    let mut io = Io::Encode(Encoder::default());
    let mut model = Freq::with_prior(&[100, 1]);
    for _ in 0..1000 {
        io.symbol(&mut model, 0);
    }
    let Io::Encode(encoder) = io else {
        unreachable!()
    };
    assert!(encoder.finish().len() < 16);
}

mod mixtures {
    use ql_codec::predict::{
        CellStateP, Factory, FrameTiedP, GenerationLawP, HelixTied, Invariance, MatchModel, Mixed,
        OperatorGrammarP, order_factories,
    };
    use ql_codec::source;

    use super::round_trip;

    fn mixed(alphabet: usize, extra: Vec<Factory>) -> Mixed {
        let mut factories = order_factories(alphabet);
        factories.extend(extra);
        Mixed {
            name: "test-mix".into(),
            alphabet,
            factories,
        }
    }

    #[test]
    fn every_mixture_round_trips_every_stream_kind() {
        for eps in [0.0, 0.5] {
            let pairs = source::generation_pairs(300, eps);
            round_trip(
                &mixed(
                    64,
                    vec![
                        Box::new(GenerationLawP::boxed),
                        Box::new(CellStateP::boxed),
                        Box::new(|| MatchModel::boxed(Invariance::Exact)),
                    ],
                ),
                &pairs.symbols,
            );
            let walk = source::operator_walk(600, eps);
            round_trip(
                &mixed(
                    64,
                    vec![
                        Box::new(OperatorGrammarP::boxed),
                        Box::new(|| MatchModel::boxed(Invariance::LineMask)),
                    ],
                ),
                &walk.symbols,
            );
            let melody = source::modal_melody(600, eps);
            round_trip(
                &mixed(
                    12,
                    vec![
                        Box::new(|| FrameTiedP::boxed(true)),
                        Box::new(|| MatchModel::boxed(Invariance::Transposition)),
                        Box::new(|| HelixTied::boxed(2, 6)),
                        Box::new(|| HelixTied::boxed(1, 12)),
                    ],
                ),
                &melody.symbols,
            );
        }
        round_trip(&mixed(64, vec![]), &[]);
        round_trip(&mixed(12, vec![]), &[3]);
    }

    #[test]
    fn a_transposed_repeat_is_cheap_only_to_the_invariant_match() {
        let phrase: Vec<u16> = (0..64).map(|i| ((i * 7 + i / 5) % 12) as u16).collect();
        let mut stream = phrase.clone();
        for shift in 1..12 {
            stream.extend(phrase.iter().map(|p| (p + shift) % 12));
        }
        let exact = round_trip(
            &mixed(12, vec![Box::new(|| MatchModel::boxed(Invariance::Exact))]),
            &stream,
        );
        let invariant = round_trip(
            &mixed(
                12,
                vec![Box::new(|| MatchModel::boxed(Invariance::Transposition))],
            ),
            &stream,
        );
        assert!(
            invariant * 2 < exact,
            "invariant {invariant} vs exact {exact}"
        );
    }

    #[test]
    fn helix_tying_keeps_the_face_and_full_tying_drops_it() {
        // A whole-tone run on the direct helix, then the same run a semitone up
        // (the conjugate helix): an order-1 interval context sees one pattern.
        let run: Vec<u16> = (0..600).map(|i| ((i % 6) * 2) as u16).collect();
        let mut stream = run.clone();
        stream.extend(run.iter().map(|p| p + 1));
        let untied = round_trip(
            &mixed(12, vec![Box::new(|| HelixTied::boxed(1, 1))]),
            &stream,
        );
        let full = round_trip(
            &mixed(12, vec![Box::new(|| HelixTied::boxed(1, 12))]),
            &stream,
        );
        assert!(full < untied, "full tying {full} vs untied {untied}");
    }

    #[test]
    fn the_mixture_stays_near_the_best_component_on_noise() {
        let noise = source::operator_walk(20_000, 1.0);
        let plain = round_trip(&mixed(64, vec![]), &noise.symbols);
        let carrying = round_trip(
            &mixed(64, vec![Box::new(OperatorGrammarP::boxed)]),
            &noise.symbols,
        );
        assert!(
            carrying as f64 <= plain as f64 * 1.01,
            "{carrying} vs {plain}"
        );
    }
}
