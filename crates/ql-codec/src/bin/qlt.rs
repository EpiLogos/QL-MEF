//! QLT: what each thought classification saves when the thoughts are coded.
//!
//! Usage: `qlt LABELS.jsonl CORPUS.json... [--blind BLIND.jsonl] [--out OUT.tsv]`
//! (default `crates/ql-codec/results/qlt-thoughts.tsv`; the alignment table
//! goes beside it as `qlt-alignment.tsv`).
//!
//! A corpus is a JSON array of units `{id, text, now?, seq?, gold?}`; a unit
//! without `now` is its own NOW. LABELS.jsonl holds one classifier answer per
//! unit `{id, t, cells, cf}`: the T level (an L4' position and face), up to
//! three MEF cells `L<n>['].<position>` and a context frame. Units without an
//! answer are left out of every condition alike.
//!
//! Every labelling is coded as a two-part code ([`ql_codec::thought`]) and
//! decoded again; the row records the exact measured bits and the actual
//! coded bytes. Controls: the same labels shuffled across units (no relation
//! to the text), and for the label stream, each NOW's labels shuffled in
//! order (no sequence).
//!
//! Alignment: if QL is one structure, a thought's T position and the
//! position of its MEF reading should coincide beyond chance. Mutual
//! information is tested against 2000 permutations, for the joint answer and
//! for BLIND.jsonl, a separate pass that never saw the T levels and named
//! each sublens rather than numbering it (`{id, cells}`).
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use ql_codec::coder::Io;
use ql_codec::predict::Mixed;
use ql_codec::thought::{ThoughtText, label_factories, label_stream};
use ql_codec::{Codec, compress, decompress};
use serde_json::Value;

struct Unit {
    id: String,
    now: String,
    seq: i64,
    text: Vec<u8>,
    gold: Option<String>,
    answer: Value,
}

type Labeller = fn(&Unit) -> Option<String>;

fn t_level(unit: &Unit) -> Option<String> {
    unit.answer["t"].as_str().map(str::to_owned)
}
fn t_position(unit: &Unit) -> Option<String> {
    t_level(unit).map(|t| t[..2].to_owned())
}
fn t_face(unit: &Unit) -> Option<String> {
    t_level(unit).map(|t| {
        if t.ends_with('\'') {
            "conjugate"
        } else {
            "direct"
        }
        .to_owned()
    })
}
fn context_frame(unit: &Unit) -> Option<String> {
    unit.answer["cf"].as_str().map(str::to_owned)
}
fn first_cell(unit: &Unit) -> Option<String> {
    unit.answer["cells"][0].as_str().map(str::to_owned)
}
fn first_lens(unit: &Unit) -> Option<String> {
    first_cell(unit).map(|cell| cell.split('.').next().unwrap_or_default().to_owned())
}
fn cell_position(unit: &Unit) -> Option<String> {
    first_cell(unit).map(|cell| format!("P{}", cell.rsplit('.').next().unwrap_or_default()))
}
fn gold_level(unit: &Unit) -> Option<String> {
    unit.gold.clone()
}

/// The QL position of a T label (`T3`, `T3'` and the position fold `T3`).
fn position_of(label: &str) -> Option<u8> {
    label
        .strip_prefix('T')
        .and_then(|rest| rest.chars().next())
        .and_then(|c| c.to_digit(10))
        .and_then(|d| u8::try_from(d).ok())
}

const LABELLINGS: &[(&str, Labeller, bool)] = &[
    ("t-level", t_level, true),
    ("t-position", t_position, true),
    ("t-face", t_face, false),
    ("context-frame", context_frame, false),
    ("mef-lens", first_lens, false),
    ("mef-position", cell_position, false),
    ("mef-cell", first_cell, false),
    ("gold-t-level", gold_level, true),
];

/// A deterministic permutation (splitmix64-driven Fisher-Yates).
fn shuffle<T>(items: &mut [T], seed: &mut u64) {
    for i in (1..items.len()).rev() {
        *seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        items.swap(i, (z % (i as u64 + 1)) as usize);
    }
}

fn measure(codec: &dyn Codec, symbols: &[u16]) -> f64 {
    let mut io = Io::Measure(Vec::with_capacity(symbols.len()));
    codec.walk(&mut io, Some(symbols), symbols.len());
    let Io::Measure(costs) = io else {
        unreachable!("a measuring walk keeps its costs")
    };
    costs.iter().sum()
}

/// Exact bits, and the actual coded bytes after a verified round trip.
fn coded(codec: &dyn Codec, symbols: &[u16]) -> (f64, usize) {
    let packed = compress(codec, symbols);
    assert_eq!(
        decompress(codec, &packed),
        symbols,
        "{} did not round-trip",
        codec.name()
    );
    (measure(codec, symbols), packed.len())
}

fn external(tool: &str, args: &[&str], bytes: &[u8]) -> Option<usize> {
    let mut child = Command::new(tool)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdin = child.stdin.take()?;
    let owned = bytes.to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&owned));
    let output = child.wait_with_output().ok()?;
    writer.join().ok()?.ok()?;
    output.status.success().then_some(output.stdout.len())
}

struct Coding {
    label_bits: f64,
    label_bits_cycle: Option<f64>,
    label_bits_unordered: f64,
    text_bits: f64,
    coded_bytes: usize,
}

/// Code `units` under one labelling; `labels[i]` is unit i's class.
fn code(units: &[&Unit], labels: &[u16], classes: usize, names: &[String], cycle: bool) -> Coding {
    let mut nows: BTreeMap<&str, Vec<u16>> = BTreeMap::new();
    for (unit, label) in units.iter().zip(labels) {
        nows.entry(unit.now.as_str()).or_default().push(*label);
    }
    let nows: Vec<Vec<u16>> = nows.into_values().collect();
    let labelled = classes > 1;
    let label_codec = |positions| Mixed {
        name: "labels".into(),
        alphabet: classes + 1,
        factories: label_factories(classes, positions),
    };
    let stream = label_stream(&nows, classes);
    let (label_bits, label_bytes) = coded(&label_codec(None), &stream);
    let label_bits_cycle = (cycle && labelled).then(|| {
        let positions = names.iter().map(|n| position_of(n)).collect();
        measure(&label_codec(Some(positions)), &stream)
    });
    let mut seed = 0x0051_7450;
    let unordered: Vec<Vec<u16>> = nows
        .iter()
        .map(|now| {
            let mut now = now.clone();
            shuffle(&mut now, &mut seed);
            now
        })
        .collect();
    let label_bits_unordered = measure(&label_codec(None), &label_stream(&unordered, classes));
    // The text follows the label stream's NOW order, so both sides agree.
    let mut ordered: Vec<(&Unit, u16)> =
        units.iter().copied().zip(labels.iter().copied()).collect();
    ordered.sort_by(|a, b| (a.0.now.as_str(), a.0.seq).cmp(&(b.0.now.as_str(), b.0.seq)));
    let text = ThoughtText {
        labels: ordered.iter().map(|(_, l)| *l).collect(),
        classes,
        labelled,
    };
    let bodies: Vec<&[u8]> = ordered.iter().map(|(u, _)| u.text.as_slice()).collect();
    let (text_bits, text_bytes) = coded(&text, &text.stream(&bodies));
    Coding {
        label_bits,
        label_bits_cycle,
        label_bits_unordered,
        text_bits,
        coded_bytes: label_bytes + text_bytes,
    }
}

fn mutual_information(xs: &[u8], ys: &[u8]) -> f64 {
    let n = xs.len() as f64;
    let mut joint: HashMap<(u8, u8), f64> = HashMap::new();
    let mut px: HashMap<u8, f64> = HashMap::new();
    let mut py: HashMap<u8, f64> = HashMap::new();
    for (&x, &y) in xs.iter().zip(ys) {
        *joint.entry((x, y)).or_default() += 1.0;
        *px.entry(x).or_default() += 1.0;
        *py.entry(y).or_default() += 1.0;
    }
    joint
        .iter()
        .map(|(&(x, y), &c)| c / n * (c * n / (px[&x] * py[&y])).log2())
        .sum()
}

/// One alignment row: MI, its permutation null and p-value, and how often
/// the two positions coincide against the chance rate of their marginals.
fn alignment(table: &mut String, corpus: &str, pair: &str, xs: &[u8], ys: &[u8]) {
    let mi = mutual_information(xs, ys);
    let mut seed = 0x0051_7452;
    let mut shuffled = ys.to_vec();
    let (mut null, mut above) = (0.0, 0usize);
    for _ in 0..2000 {
        shuffle(&mut shuffled, &mut seed);
        let m = mutual_information(xs, &shuffled);
        null += m;
        above += usize::from(m >= mi);
    }
    let n = xs.len() as f64;
    let same = xs.iter().zip(ys).filter(|(x, y)| x == y).count() as f64 / n;
    let chance: f64 = (0..=255u8)
        .map(|v| {
            let a = xs.iter().filter(|x| **x == v).count() as f64;
            let b = ys.iter().filter(|y| **y == v).count() as f64;
            a * b / (n * n)
        })
        .sum();
    let p = (1 + above) as f64 / 2001.0;
    writeln!(
        table,
        "{corpus}\t{pair}\t{}\t{mi:.4}\t{:.4}\t{p:.4}\t{same:.4}\t{chance:.4}",
        xs.len(),
        null / 2000.0
    )
    .unwrap();
    eprintln!(
        "  {pair:<44} MI {mi:.3} (null {:.3}) p={p:.4}  same {:.1}% vs chance {:.1}%",
        null / 2000.0,
        100.0 * same,
        100.0 * chance
    );
}

/// `L2'.4` -> (lens, position).
fn cell(value: &Value) -> Option<(String, u8)> {
    let (lens, position) = value.as_str()?.rsplit_once('.')?;
    Some((lens.to_owned(), position.parse().ok()?))
}

fn align_corpus(table: &mut String, corpus: &str, units: &[&Unit], blind: &HashMap<String, Value>) {
    let t = |u: &Unit| t_position(u).and_then(|p| position_of(&p));
    let mut pairs: Vec<(&str, Vec<(u8, u8)>)> = Vec::new();
    let first = |cells: &Value| cells[0].clone();
    let every = |cells: &Value| cells.as_array().cloned().unwrap_or_default();
    for (name, source, all, skip) in [
        (
            "t-position x mef-position (joint, first)",
            false,
            false,
            None,
        ),
        (
            "t-position x mef-position (joint, every)",
            false,
            true,
            None,
        ),
        (
            "t-position x mef-position (blind, first)",
            true,
            false,
            None,
        ),
        ("t-position x mef-position (blind, every)", true, true, None),
        (
            "t-position x mef-position (blind, every, no L0')",
            true,
            true,
            Some("L0'"),
        ),
    ] {
        let mut rows = Vec::new();
        for unit in units {
            let cells = if source {
                blind.get(&unit.id).map(|b| b["cells"].clone())
            } else {
                Some(unit.answer["cells"].clone())
            };
            let (Some(tp), Some(cells)) = (t(unit), cells) else {
                continue;
            };
            let chosen = if all {
                every(&cells)
            } else {
                vec![first(&cells)]
            };
            for c in chosen.iter().filter_map(cell) {
                if skip != Some(c.0.as_str()) {
                    rows.push((tp, c.1));
                }
            }
        }
        pairs.push((name, rows));
    }
    let frames: Vec<(u8, u8)> = units
        .iter()
        .filter_map(|u| {
            let cf = context_frame(u)?;
            Some((t(u)?, cf.strip_prefix("CF")?.parse().ok()?))
        })
        .collect();
    pairs.push(("t-position x context-frame", frames));
    let gold: Vec<(u8, u8)> = units
        .iter()
        .filter_map(|u| Some((position_of(u.gold.as_deref()?)?, t(u)?)))
        .collect();
    if !gold.is_empty() {
        pairs.push(("gold t-position x classified t-position", gold));
    }
    for (name, rows) in pairs {
        if rows.len() < 20 {
            continue;
        }
        let (xs, ys): (Vec<u8>, Vec<u8>) = rows.into_iter().unzip();
        alignment(table, corpus, name, &xs, &ys);
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let out = match args.iter().position(|a| a == "--out") {
        Some(i) => {
            let path = PathBuf::from(args.remove(i + 1));
            args.remove(i);
            path
        }
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("results/qlt-thoughts.tsv"),
    };
    let blind: HashMap<String, Value> = match args.iter().position(|a| a == "--blind") {
        Some(i) => {
            let path = args.remove(i + 1);
            args.remove(i);
            std::fs::read_to_string(path)
                .expect("blind labels")
                .lines()
                .filter_map(|line| serde_json::from_str::<Value>(line).ok())
                .filter_map(|v| Some((v["id"].as_str()?.to_owned(), v)))
                .collect()
        }
        None => HashMap::new(),
    };
    let mut aligned =
        String::from("corpus\tpair\tn\tmi_bits\tnull_mi_bits\tp\tsame_rate\tchance_rate\n");
    let (labels_path, corpora) = args
        .split_first()
        .expect("usage: qlt LABELS.jsonl CORPUS.json...");
    let answers: HashMap<String, Value> = std::fs::read_to_string(labels_path)
        .expect("labels")
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter_map(|v| Some((v["id"].as_str()?.to_owned(), v)))
        .collect();

    let mut table = String::from(
        "corpus\tlabelling\tclasses\tunits\ttext_bytes\tlabel_bits\tlabel_bits_ql_cycle\t\
         label_bits_unordered\ttext_bits\ttotal_bits\tsaving_bits\tsaving_pct\tcoded_bytes\n",
    );
    for corpus in corpora {
        let name = Path::new(corpus)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let raw: Value = serde_json::from_str(&std::fs::read_to_string(corpus).expect("corpus"))
            .expect("corpus JSON");
        let units: Vec<Unit> = raw
            .as_array()
            .expect("a unit array")
            .iter()
            .filter_map(|u| {
                let id = u["id"].as_str()?;
                Some(Unit {
                    id: id.to_owned(),
                    now: u["now"].as_str().unwrap_or(id).to_owned(),
                    seq: u["seq"].as_i64().unwrap_or(0),
                    text: u["text"].as_str()?.as_bytes().to_vec(),
                    gold: u["gold"].as_str().map(str::to_owned),
                    answer: answers.get(id)?.clone(),
                })
            })
            .collect();
        let units: Vec<&Unit> = units.iter().collect();
        let text_bytes: usize = units.iter().map(|u| u.text.len()).sum();
        let all: Vec<u8> = units.iter().flat_map(|u| u.text.iter().copied()).collect();
        eprintln!(
            "{name}: {} units, {text_bytes} bytes; zstd-19 {:?} B, xz-9e {:?} B",
            units.len(),
            external("zstd", &["-19", "-c"], &all),
            external("xz", &["-9e", "-c"], &all)
        );

        align_corpus(&mut aligned, &name, &units, &blind);
        let none = code(&units, &vec![0; units.len()], 1, &["none".into()], false);
        let none_total = none.label_bits + none.text_bits;
        let mut row = |labelling: &str, classes: usize, coding: &Coding| {
            let total = coding.label_bits + coding.text_bits;
            let opt = |v: Option<f64>| v.map_or_else(String::new, |v| format!("{v:.1}"));
            writeln!(
                table,
                "{name}\t{labelling}\t{classes}\t{}\t{text_bytes}\t{:.1}\t{}\t{:.1}\t{:.1}\t{total:.1}\t{:.1}\t{:.3}\t{}",
                units.len(),
                coding.label_bits,
                opt(coding.label_bits_cycle),
                coding.label_bits_unordered,
                coding.text_bits,
                none_total - total,
                100.0 * (none_total - total) / none_total,
                coding.coded_bytes,
            )
            .unwrap();
            eprintln!(
                "  {labelling:<22} {classes:>3} classes  saving {:>9.1} bits ({:+.3}%)",
                none_total - total,
                100.0 * (none_total - total) / none_total
            );
        };
        row("none", 1, &none);
        for &(labelling, labeller, cycle) in LABELLINGS {
            let classes: Vec<Option<String>> = units.iter().map(|u| labeller(u)).collect();
            if classes.iter().any(Option::is_none) {
                continue;
            }
            let names: Vec<String> = classes
                .iter()
                .flatten()
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let index: HashMap<&str, u16> = names
                .iter()
                .enumerate()
                .map(|(i, n)| (n.as_str(), u16::try_from(i).unwrap()))
                .collect();
            let labels: Vec<u16> = classes
                .iter()
                .flatten()
                .map(|c| index[c.as_str()])
                .collect();
            row(
                labelling,
                names.len(),
                &code(&units, &labels, names.len(), &names, cycle),
            );
            let mut shuffled = labels.clone();
            shuffle(&mut shuffled, &mut 0x0051_7451);
            row(
                &format!("{labelling}/shuffled"),
                names.len(),
                &code(&units, &shuffled, names.len(), &names, cycle),
            );
        }
    }
    std::fs::create_dir_all(out.parent().expect("output has a parent")).expect("results dir");
    std::fs::write(&out, table).expect("write results");
    std::fs::write(out.with_file_name("qlt-alignment.tsv"), aligned).expect("write alignment");
    eprintln!("wrote {}", out.display());
}
