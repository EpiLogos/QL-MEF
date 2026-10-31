//! QLT: thought classification measured as compression.
//!
//! A labelling of thought units is a classifier; it is worth what it saves.
//! The corpus is coded as a two-part code. First the label stream, one label
//! per unit with a boundary symbol between NOWs, under its own mixture
//! ([`label_factories`], optionally with [`QlCycle`]). Then the text: each
//! unit's bytes, closed by [`END`], with the unit's label known at its start
//! and so never paid twice ([`ThoughtText`]). The text mixture holds
//! shared byte contexts and the same contexts keyed by the current label
//! ([`SparseContext`]), so a labelling that carries no information costs
//! only the mixer's switching overhead, and one that does pays for itself.
//!
//! Net saving of a labelling = bits(no labels) - (text bits | labels +
//! label bits). A shuffled copy of the same labels is the control: same
//! distribution, no relation to the text.
use std::collections::HashMap;

use crate::coder::Io;
use crate::predict::{Factory, Invariance, MatchModel, Mixture, Predictor};
use crate::{Codec, at};

/// The closing symbol of every unit's bytes.
pub const END: u16 = 256;
/// Label markers start here in the text stream.
pub const MARK: u16 = 257;

/// Additive prior of a sparse context: how much an unseen symbol keeps.
const PRIOR: f64 = 1.0 / 32.0;

/// Order-k over the text stream with sparse counts, keyed by the last k
/// symbols and, when `labelled`, by the current unit's label (the latest
/// marker seen). Markers enter the stream but are never predicted here.
pub struct SparseContext {
    k: usize,
    labelled: bool,
    label: u16,
    tables: HashMap<u64, (u32, Vec<(u16, u32)>)>,
}

impl SparseContext {
    pub fn boxed(k: usize, labelled: bool) -> Box<dyn Predictor> {
        Box::new(Self {
            k,
            labelled,
            label: 0,
            tables: HashMap::new(),
        })
    }

    fn key(&self, history: &[u16]) -> u64 {
        let context = history.iter().rev().take(self.k).fold(0u64, |key, s| {
            key.wrapping_mul(1031).wrapping_add(u64::from(*s) + 1)
        });
        let label = if self.labelled {
            u64::from(self.label) + 1
        } else {
            0
        };
        context.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ label
    }
}

impl Predictor for SparseContext {
    fn name(&self) -> String {
        format!(
            "{}order-{}",
            if self.labelled { "label-" } else { "" },
            self.k
        )
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        dist.iter_mut().for_each(|p| *p = PRIOR);
        if let Some((_, counts)) = self.tables.get(&self.key(history)) {
            for &(symbol, count) in counts {
                dist[usize::from(symbol)] += f64::from(count);
            }
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        if actual >= MARK {
            self.label = actual - MARK;
            return;
        }
        let key = self.key(history);
        let (total, counts) = self.tables.entry(key).or_default();
        *total += 1;
        match counts.iter_mut().find(|(symbol, _)| *symbol == actual) {
            Some((_, count)) => *count += 1,
            None => counts.push((actual, 1)),
        }
        if *total > 1 << 16 {
            *total = 0;
            for (_, count) in counts.iter_mut() {
                *count = (*count).div_ceil(2);
                *total += *count;
            }
        }
    }
}

/// The text mixture: shared orders 0..=4 and the exact match, plus (when
/// `labelled`) the same orders keyed by the current label. Without labels the
/// keyed copies would equal the shared ones, so they are left out.
pub fn text_factories(labelled: bool) -> Vec<Factory> {
    let mut factories: Vec<Factory> = (0..=4)
        .map(|k| Box::new(move || SparseContext::boxed(k, false)) as Factory)
        .collect();
    factories.push(Box::new(|| MatchModel::boxed(Invariance::Exact)));
    if labelled {
        factories
            .extend((1..=3).map(|k| Box::new(move || SparseContext::boxed(k, true)) as Factory));
    }
    factories
}

/// The text of a labelled corpus: unit bytes closed by [`END`], each unit
/// opened by its label's marker, which both sides already know from the
/// label stream and so is inserted, not coded.
pub struct ThoughtText {
    pub labels: Vec<u16>,
    pub classes: usize,
    pub labelled: bool,
}

impl ThoughtText {
    pub fn alphabet(&self) -> usize {
        usize::from(MARK) + self.classes
    }

    /// The full stream (markers included) for one labelled corpus.
    pub fn stream(&self, units: &[&[u8]]) -> Vec<u16> {
        let mut stream = Vec::new();
        for (unit, label) in units.iter().zip(&self.labels) {
            stream.push(MARK + label);
            stream.extend(unit.iter().map(|b| u16::from(*b)));
            stream.push(END);
        }
        stream
    }
}

impl Codec for ThoughtText {
    fn name(&self) -> String {
        format!(
            "thought-text/{}",
            if self.labelled { "labelled" } else { "plain" }
        )
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut mixture = Mixture::new(&text_factories(self.labelled), self.alphabet());
        let mut out: Vec<u16> = Vec::with_capacity(n);
        let mut labels = self.labels.iter();
        for t in 0..n {
            let symbol = if out.last().is_none_or(|s| *s == END) {
                // Known from the label stream: taught, not coded.
                MARK + labels.next().expect("one label per unit")
            } else {
                let (counts, total) = mixture.counts(&out);
                io.code(counts, total, at(input, t)) as u16
            };
            mixture.learn(&out, symbol);
            out.push(symbol);
        }
        out
    }
}

/// The QL thought cycle as a label predictor: after a unit at position p
/// (either face), the next unit is expected at position p + 1 mod 6.
/// `positions[label]` is the label's QL position; labels without one are
/// `None` and get no cycle mass. Boundaries (`label == classes`) reset it.
pub struct QlCycle {
    positions: Vec<Option<u8>>,
}

impl QlCycle {
    pub fn boxed(positions: Vec<Option<u8>>) -> Box<dyn Predictor> {
        Box::new(Self { positions })
    }
}

impl Predictor for QlCycle {
    fn name(&self) -> String {
        "ql-cycle".into()
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        dist.iter_mut().for_each(|p| *p = 1.0 / 64.0);
        let previous = history
            .last()
            .and_then(|label| self.positions.get(usize::from(*label)).copied().flatten());
        if let Some(position) = previous {
            let next = (position + 1) % 6;
            for (label, p) in self.positions.iter().zip(dist.iter_mut()) {
                if *label == Some(next) {
                    *p += 1.0;
                }
            }
        }
    }

    fn update(&mut self, _history: &[u16], _actual: u16) {}
}

/// The label mixture: adaptive orders 0..=2 over labels and boundaries, and
/// the QL cycle when the labelling has positions.
pub fn label_factories(classes: usize, positions: Option<Vec<Option<u8>>>) -> Vec<Factory> {
    let alphabet = classes + 1;
    let mut factories: Vec<Factory> = (0..=2)
        .map(|k| Box::new(move || crate::predict::OrderKP::boxed(k, alphabet)) as Factory)
        .collect();
    if let Some(positions) = positions {
        factories.push(Box::new(move || QlCycle::boxed(positions.clone())));
    }
    factories
}

/// The label stream: each NOW's labels, preceded by a boundary symbol.
pub fn label_stream(nows: &[Vec<u16>], classes: usize) -> Vec<u16> {
    let boundary = u16::try_from(classes).expect("classes fit u16");
    nows.iter()
        .flat_map(|labels| std::iter::once(boundary).chain(labels.iter().copied()))
        .collect()
}
