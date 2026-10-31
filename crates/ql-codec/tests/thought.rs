//! QLT: the thought codec round-trips, and a labelling saves bits only when
//! it carries information about the text.
use ql_codec::coder::Io;
use ql_codec::predict::Mixed;
use ql_codec::source::Rng;
use ql_codec::thought::{ThoughtText, label_factories, label_stream};
use ql_codec::{Codec, compress, decompress};

fn measure(codec: &dyn Codec, symbols: &[u16]) -> f64 {
    let mut io = Io::Measure(Vec::new());
    codec.walk(&mut io, Some(symbols), symbols.len());
    let Io::Measure(costs) = io else {
        unreachable!()
    };
    costs.iter().sum()
}

/// Two classes of unit drawn from disjoint vocabularies, in random order.
fn corpus() -> (Vec<Vec<u8>>, Vec<u16>) {
    let words = [
        ["anuttara", "paramasiva", "spanda", "vimarsa"],
        ["compiler", "register", "pointer", "segment"],
    ];
    let mut rng = Rng::new(450);
    let mut units = Vec::new();
    let mut labels = Vec::new();
    for _ in 0..120 {
        let class = (rng.next_u64() % 2) as usize;
        let text: Vec<&str> = (0..12)
            .map(|_| words[class][(rng.next_u64() % 4) as usize])
            .collect();
        units.push(text.join(" ").into_bytes());
        labels.push(class as u16);
    }
    (units, labels)
}

fn text_bits(units: &[Vec<u8>], labels: &[u16], classes: usize) -> f64 {
    let text = ThoughtText {
        labels: labels.to_vec(),
        classes,
        labelled: classes > 1,
    };
    let bodies: Vec<&[u8]> = units.iter().map(Vec::as_slice).collect();
    measure(&text, &text.stream(&bodies))
}

#[test]
fn labelled_text_round_trips_with_its_labels_inserted_not_coded() {
    let (units, labels) = corpus();
    let text = ThoughtText {
        labels: labels.clone(),
        classes: 2,
        labelled: true,
    };
    let bodies: Vec<&[u8]> = units.iter().map(Vec::as_slice).collect();
    let stream = text.stream(&bodies);
    assert_eq!(decompress(&text, &compress(&text, &stream)), stream);
    let mut costs = Io::Measure(Vec::new());
    text.walk(&mut costs, Some(&stream), stream.len());
    let Io::Measure(costs) = costs else {
        unreachable!()
    };
    assert_eq!(
        costs.len(),
        stream.len() - units.len(),
        "markers are not coded"
    );
}

#[test]
fn true_labels_inform_the_text_and_shuffled_labels_mislead_it() {
    let (units, labels) = corpus();
    let plain = text_bits(&units, &vec![0; units.len()], 1);
    let labelled = text_bits(&units, &labels, 2);
    let mut shuffled = labels.clone();
    shuffled.rotate_left(37);
    let control = text_bits(&units, &shuffled, 2);
    // The text reveals its class after one word, so true labels can save at
    // most about a bit a unit; a same-shape random labelling misleads the
    // label-keyed contexts and costs more than no labels at all.
    assert!(labelled < plain, "{plain} {labelled}");
    assert!(control > plain, "{plain} {control}");
    assert!(
        control - labelled > units.len() as f64,
        "{control} {labelled}"
    );
}

#[test]
fn the_ql_cycle_codes_a_cyclic_thought_sequence_cheaper() {
    // Twelve T labels: index 2p is Tp, 2p+1 is Tp'.
    let positions: Vec<Option<u8>> = (0..12).map(|i| Some(i / 2)).collect();
    let mut rng = Rng::new(7);
    let nows: Vec<Vec<u16>> = (0..40)
        .map(|_| {
            let start = (rng.next_u64() % 6) as u16;
            (0..9)
                .map(|step| ((start + step) % 6) * 2 + (rng.next_u64() % 2) as u16)
                .collect()
        })
        .collect();
    let stream = label_stream(&nows, 12);
    let codec = |positions| Mixed {
        name: "labels".into(),
        alphabet: 13,
        factories: label_factories(12, positions),
    };
    let plain = codec(None);
    let cycle = codec(Some(positions));
    assert_eq!(decompress(&cycle, &compress(&cycle, &stream)), stream);
    assert!(measure(&cycle, &stream) < measure(&plain, &stream));
}
