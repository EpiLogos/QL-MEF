//! `qlc` — run the QLC sweep: every codec and mixture on every stream, each
//! output decoded and checked byte-for-byte, results written as TSV.
//!
//! Usage: `qlc [OUT.tsv]` (default `crates/ql-codec/results/qlc-sweep.tsv`).
//! Real melodies come from Ableton Live's Core Library tonal clips when that
//! library is installed (read at run time, never stored); `QLC_CLIPS` points
//! at another clip directory.
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use ql_codec::codecs::{
    CellState, FrameTied, GenerationLaw, OperatorGrammar, OrderK, PairContext, XorMask,
};
use ql_codec::predict::{
    CellStateP, Factory, FrameTiedP, GenerationLawP, HelixTied, Invariance, MatchModel, Mixed,
    OperatorGrammarP, order_factories,
};
use ql_codec::source::{self, Kind, Stream};
use ql_codec::{Codec, compress, decompress};

const SIZES: [usize; 3] = [512, 8192, 65536];
const NOISE: [f64; 4] = [0.0, 0.1, 0.5, 1.0];

fn codecs_for(stream: &Stream) -> Vec<Box<dyn Codec>> {
    let a = stream.alphabet;
    let mut list: Vec<Box<dyn Codec>> = (0..=2)
        .map(|k| Box::new(OrderK { k, alphabet: a }) as Box<dyn Codec>)
        .collect();
    if a <= 64 {
        list.push(Box::new(OrderK { k: 3, alphabet: a }));
    }
    match stream.kind {
        Kind::GenerationPairs => {
            list.push(Box::new(PairContext));
            list.push(Box::new(GenerationLaw::default()));
            list.push(Box::new(CellState::default()));
        }
        Kind::Hexagrams => {
            list.push(Box::new(XorMask));
            list.push(Box::new(OperatorGrammar::default()));
        }
        Kind::Pitches => {
            list.push(Box::new(FrameTied::new(false)));
            list.push(Box::new(FrameTied::new(true)));
        }
        Kind::Bytes => {}
    }
    list.extend(mixtures(stream));
    list
}

fn mixed(name: &str, alphabet: usize, factories: Vec<Factory>) -> Box<dyn Codec> {
    Box::new(Mixed {
        name: name.into(),
        alphabet,
        factories,
    })
}

/// Wave 2/3 mixtures, each one adding a layer to the one before so the
/// difference between rows is what that layer contributes.
fn mixtures(stream: &Stream) -> Vec<Box<dyn Codec>> {
    let a = stream.alphabet;
    let with = |extra: Vec<Factory>| {
        let mut factories = order_factories(a);
        factories.push(Box::new(|| MatchModel::boxed(Invariance::Exact)));
        factories.extend(extra);
        factories
    };
    let invariant: Option<fn() -> Box<dyn ql_codec::predict::Predictor>> = match stream.kind {
        Kind::Hexagrams => Some(|| MatchModel::boxed(Invariance::LineMask)),
        Kind::Pitches => Some(|| MatchModel::boxed(Invariance::Transposition)),
        _ => None,
    };
    let ql: Vec<fn() -> Box<dyn ql_codec::predict::Predictor>> = match stream.kind {
        Kind::GenerationPairs => vec![GenerationLawP::boxed, CellStateP::boxed],
        Kind::Hexagrams => vec![OperatorGrammarP::boxed],
        Kind::Pitches => vec![
            || FrameTiedP::boxed(true),
            || HelixTied::boxed(2, 6),
            || HelixTied::boxed(2, 12),
        ],
        Kind::Bytes => vec![],
    };
    let mut list = Vec::new();
    if stream.kind == Kind::Pitches {
        // Wave 4: the same order-2 context, tied across no rotation, the six
        // whole-tone rotations of a helix, or all twelve lens transpositions.
        for group in [1u16, 6, 12] {
            list.push(mixed(
                &format!("helix-tied-{group}"),
                a,
                vec![Box::new(move || HelixTied::boxed(2, group)) as Factory],
            ));
        }
    }
    list.extend([
        mixed("mix-order", a, order_factories(a)),
        mixed("mix-order+match", a, with(vec![])),
    ]);
    if let Some(invariant) = invariant {
        list.push(mixed(
            "mix-order+match+invariant",
            a,
            with(vec![Box::new(invariant) as Factory]),
        ));
    }
    if !ql.is_empty() {
        let mut extra: Vec<Factory> = invariant
            .into_iter()
            .map(|f| Box::new(f) as Factory)
            .collect();
        extra.extend(ql.into_iter().map(|f| Box::new(f) as Factory));
        list.push(mixed("mix-all-ql", a, with(extra)));
    }
    list
}

/// Ableton clips under `dir`, sorted by path, as one run of MIDI keys.
fn clip_corpus(dir: &Path) -> Option<(usize, Vec<u8>)> {
    let mut paths = Vec::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(next) = pending.pop() {
        for entry in std::fs::read_dir(&next).ok()?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "alc") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    let mut keys = Vec::new();
    for path in &paths {
        let xml = Command::new("gzip").arg("-dc").arg(path).output().ok()?;
        keys.extend(source::clip_keys(&String::from_utf8_lossy(&xml.stdout)));
    }
    (!keys.is_empty()).then_some((paths.len(), keys))
}

/// A general-purpose compressor over the stream's bytes; `None` when the tool
/// is absent. The output is decoded again and must reproduce the input.
fn external(tool: &str, args: &[&str], decode: &[&str], bytes: &[u8]) -> Option<usize> {
    let run = |argv: &[&str], input: &[u8]| -> Option<Vec<u8>> {
        let mut child = Command::new(tool)
            .args(argv)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let mut stdin = child.stdin.take()?;
        let owned = input.to_vec();
        let writer = std::thread::spawn(move || stdin.write_all(&owned));
        let output = child.wait_with_output().ok()?;
        writer.join().ok()?.ok()?;
        output.status.success().then_some(output.stdout)
    };
    let packed = run(args, bytes)?;
    let unpacked = run(decode, &packed)?;
    assert_eq!(unpacked, bytes, "{tool} did not round-trip");
    Some(packed.len())
}

struct Row {
    stream: String,
    n: usize,
    eps: f64,
    codec: String,
    bytes: usize,
    bits_per_unit: f64,
}

fn measure(stream: &Stream, rows: &mut Vec<Row>) {
    let mut push = |codec: String, bytes: usize| {
        rows.push(Row {
            stream: stream.name.clone(),
            n: stream.symbols.len(),
            eps: stream.eps,
            codec,
            bytes,
            bits_per_unit: bytes as f64 * 8.0 / stream.units as f64,
        });
    };
    for codec in codecs_for(stream) {
        let packed = compress(codec.as_ref(), &stream.symbols);
        let unpacked = decompress(codec.as_ref(), &packed);
        assert_eq!(
            unpacked,
            stream.symbols,
            "{} did not round-trip on {}",
            codec.name(),
            stream.name
        );
        push(codec.name(), packed.len());
    }
    // Text sextets are a view of the same bytes; the external tools see the
    // original text once, under docs-text.
    if stream.kind == Kind::Hexagrams && stream.name == "docs-text-sextets" {
        return;
    }
    let bytes = stream.bytes();
    if let Some(size) = external("zstd", &["-19", "-q", "-c"], &["-d", "-q", "-c"], &bytes) {
        push("zstd-19".into(), size);
    }
    if let Some(size) = external(
        "xz",
        &["--format=raw", "-9e", "-c"],
        &["--format=raw", "-d", "-c"],
        &bytes,
    ) {
        push("xz-raw-9e".into(), size);
    }
}

fn main() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("results/qlc-sweep.tsv"));
    let mut streams = Vec::new();
    for n in SIZES {
        for eps in NOISE {
            streams.push(source::generation_pairs(n / 2, eps));
            streams.push(source::operator_walk(n, eps));
            streams.push(source::modal_melody(n, eps));
        }
        streams.push(source::casting_walk(n));
    }
    streams.push(source::recorded_clock());
    let text = source::docs_text(&repo, 96 * 1024);
    streams.push(source::text_bytes(&text));
    streams.push(source::text_sextets(&text));
    let clips = std::env::var_os("QLC_CLIPS").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(
            "/Applications/Ableton Live 12 Suite.app/Contents/App-Resources/Core Library/MIDI Clips/Tonal",
        )
    });
    if let Some((count, keys)) = clip_corpus(&clips) {
        eprintln!(
            "qlc: {count} clips, {} notes from {}",
            keys.len(),
            clips.display()
        );
        streams.push(source::clip_pitches("clips-tonal", &keys));
    }

    let mut rows = Vec::new();
    for stream in &streams {
        measure(stream, &mut rows);
    }
    let mut tsv = String::from(
        "# ql.qlc.sweep/v1 — generated by `cargo run -p ql-codec --release --bin qlc`; every row decoded and verified\n\
         stream\tn\teps\tcodec\tbytes\tbits_per_unit\n",
    );
    for row in &rows {
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{:.4}\n",
            row.stream, row.n, row.eps, row.codec, row.bytes, row.bits_per_unit
        ));
    }
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).expect("results directory");
    }
    std::fs::write(&out, &tsv).expect("write results");
    print!("{tsv}");
}
