//! Wave 2 and 3: the codecs as predictors, and a mixer that never loses much
//! to the best of them.
//!
//! A [`Predictor`] states a full distribution over the stream's alphabet at
//! each step, then learns the actual symbol. [`Mixed`] codes each symbol
//! under a weighted mixture of its predictors; the weights follow each
//! predictor's likelihood with a fixed share (Herbster–Warmuth), so the
//! mixture tracks whichever predictor is currently right. Its total cost
//! stays within `log2(m)` bits plus switching costs of the best predictor's,
//! which is how a QL prior can be carried without paying for it where the
//! law fails.
//!
//! Wave 3's [`MatchModel`] finds the longest earlier recurrence of the recent
//! trajectory and predicts its continuation. Under [`Invariance::Exact`] it
//! is the generic LZ-style match; under [`Invariance::Transposition`] and
//! [`Invariance::LineMask`] it matches a phrase up to the kernel's group
//! action: a melody under any of the twelve lens transpositions, a hexagram
//! walk under any XOR line mask (which includes the complement).
use std::collections::HashMap;

use ql_core::{MatrixFamily, generate};

use crate::codecs::{CellState, FrameTied, GenerationLaw, KernelOp};
use crate::coder::{Freq, Io};
use crate::{Codec, at};

pub trait Predictor {
    fn name(&self) -> String;
    /// Write the distribution of symbol `history.len()` into `dist` (zeroed,
    /// alphabet-sized); it need not be normalised.
    fn predict(&mut self, history: &[u16], dist: &mut [f64]);
    /// Learn the actual symbol that followed `history`.
    fn update(&mut self, history: &[u16], actual: u16);
}

pub type Factory = Box<dyn Fn() -> Box<dyn Predictor>>;

/// Quantisation total for mixed distributions.
const SCALE: f64 = 60_000.0;
/// Fixed-share rate: how fast the mixture may move to another predictor.
const SHARE: f64 = 1.0 / 256.0;

pub struct Mixed {
    pub name: String,
    pub alphabet: usize,
    pub factories: Vec<Factory>,
}

/// One fixed-share mixture over fresh predictors: the shared step of every
/// mixed walk. [`Mixture::counts`] states the quantised distribution of the
/// next symbol, [`Mixture::learn`] moves the weights and teaches every
/// predictor the symbol actually coded.
pub struct Mixture {
    alphabet: usize,
    predictors: Vec<Box<dyn Predictor>>,
    weights: Vec<f64>,
    dists: Vec<Vec<f64>>,
    counts: Vec<u32>,
}

impl Mixture {
    pub fn new(factories: &[Factory], alphabet: usize) -> Self {
        let predictors: Vec<Box<dyn Predictor>> = factories.iter().map(|f| f()).collect();
        let m = predictors.len();
        Self {
            alphabet,
            predictors,
            weights: vec![1.0 / m as f64; m],
            dists: vec![vec![0.0; alphabet]; m],
            counts: vec![0; alphabet],
        }
    }

    pub fn counts(&mut self, history: &[u16]) -> (&[u32], u32) {
        for (predictor, dist) in self.predictors.iter_mut().zip(&mut self.dists) {
            dist.iter_mut().for_each(|p| *p = 0.0);
            predictor.predict(history, dist);
            let sum: f64 = dist.iter().sum();
            if sum > 0.0 {
                dist.iter_mut().for_each(|p| *p /= sum);
            } else {
                dist.iter_mut()
                    .for_each(|p| *p = 1.0 / self.alphabet as f64);
            }
        }
        let mut total = 0u32;
        for (s, count) in self.counts.iter_mut().enumerate() {
            let p: f64 = self
                .weights
                .iter()
                .zip(&self.dists)
                .map(|(w, d)| w * d[s])
                .sum();
            *count = 1 + (p * SCALE) as u32;
            total += *count;
        }
        (&self.counts, total)
    }

    pub fn learn(&mut self, history: &[u16], symbol: u16) {
        let m = self.predictors.len();
        let mut norm = 0.0;
        for (w, d) in self.weights.iter_mut().zip(&self.dists) {
            *w *= d[usize::from(symbol)].max(1e-12);
            norm += *w;
        }
        for w in &mut self.weights {
            *w = (1.0 - SHARE) * *w / norm + SHARE / m as f64;
        }
        for predictor in &mut self.predictors {
            predictor.update(history, symbol);
        }
    }
}

impl Codec for Mixed {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut mixture = Mixture::new(&self.factories, self.alphabet);
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            let (counts, total) = mixture.counts(&out);
            let symbol = io.code(counts, total, at(input, t)) as u16;
            mixture.learn(&out, symbol);
            out.push(symbol);
        }
        out
    }
}

fn spread(model: &Freq, mass: f64, dist: &mut [f64]) {
    for (s, p) in dist.iter_mut().enumerate() {
        *p += mass * model.p(s);
    }
}

pub struct OrderKP {
    k: usize,
    alphabet: usize,
    models: HashMap<u64, Freq>,
}

impl OrderKP {
    pub fn boxed(k: usize, alphabet: usize) -> Box<dyn Predictor> {
        Box::new(Self {
            k,
            alphabet,
            models: HashMap::new(),
        })
    }

    fn key(&self, history: &[u16]) -> u64 {
        history.iter().rev().take(self.k).fold(0u64, |key, s| {
            key * (self.alphabet as u64 + 1) + u64::from(*s) + 1
        })
    }
}

impl Predictor for OrderKP {
    fn name(&self) -> String {
        format!("order-{}", self.k)
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let key = self.key(history);
        match self.models.get(&key) {
            Some(model) => spread(model, 1.0, dist),
            None => dist.iter_mut().for_each(|p| *p = 1.0),
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let key = self.key(history);
        let alphabet = self.alphabet;
        self.models
            .entry(key)
            .or_insert_with(|| Freq::new(alphabet))
            .observe(usize::from(actual));
    }
}

/// QLC-1 as a predictor over the positive/negative pair stream.
pub struct GenerationLawP {
    law: GenerationLaw,
    positives: HashMap<u16, Freq>,
    ranks: HashMap<(usize, usize), Freq>,
    literal: Freq,
    matrix: usize,
    last_rank: usize,
}

impl GenerationLawP {
    pub fn boxed() -> Box<dyn Predictor> {
        Box::new(Self {
            law: GenerationLaw::default(),
            positives: HashMap::new(),
            ranks: HashMap::new(),
            literal: Freq::new(64),
            matrix: 0,
            last_rank: 0,
        })
    }

    fn rank_model(&mut self, len: usize) -> &mut Freq {
        self.ranks
            .entry((len, self.last_rank.min(2)))
            .or_insert_with(|| Freq::with_prior(&[8, 8, 1, 1, 1, 1, 2]))
    }
}

impl Predictor for GenerationLawP {
    fn name(&self) -> String {
        "qlc1-generation-law".into()
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let t = history.len();
        if t % 2 == 0 {
            let key = if t >= 2 { history[t - 2] } else { 64 };
            let model = self.positives.entry(key).or_insert_with(|| Freq::new(64));
            spread(model, 1.0, dist);
            return;
        }
        let list = self.law.ordered(self.matrix, history[t - 1]);
        let model = self.rank_model(list.len()).clone();
        for (rank, negative) in list.iter().enumerate() {
            dist[usize::from(*negative)] += model.p(rank);
        }
        spread(&self.literal, model.p(GenerationLaw::ESCAPE), dist);
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let t = history.len();
        if t % 2 == 0 {
            let key = if t >= 2 { history[t - 2] } else { 64 };
            self.positives
                .entry(key)
                .or_insert_with(|| Freq::new(64))
                .observe(usize::from(actual));
            return;
        }
        let positive = history[t - 1];
        let list = self.law.ordered(self.matrix, positive);
        let rank = list
            .iter()
            .position(|c| *c == actual)
            .unwrap_or(GenerationLaw::ESCAPE);
        self.rank_model(list.len()).observe(rank);
        if rank == GenerationLaw::ESCAPE {
            self.literal.observe(usize::from(actual));
        }
        let candidates = &self.law.candidates;
        if !candidates[self.matrix][usize::from(positive)].contains(&actual) {
            if let Some(m) =
                (0..3).find(|m| candidates[*m][usize::from(positive)].contains(&actual))
            {
                self.matrix = m;
            }
        }
        self.last_rank = rank;
    }
}

/// QLC-1b as a predictor: the joint over cells, read one codon at a time.
pub struct CellStateP {
    cells: CellState,
    modes: Freq,
    masks: Freq,
    literal: Freq,
    matrix: usize,
    hexagram: u8,
}

impl CellStateP {
    pub fn boxed() -> Box<dyn Predictor> {
        Box::new(Self {
            cells: CellState::default(),
            modes: Freq::with_prior(&[64, 1, 1, 2]),
            masks: Freq::new(64),
            literal: Freq::new(64),
            matrix: 0,
            hexagram: 0,
        })
    }

    fn order(&self) -> [usize; 3] {
        let others: Vec<usize> = (0..3).filter(|m| *m != self.matrix).collect();
        [self.matrix, others[0], others[1]]
    }

    /// Every lawful next cell with its mass: (positive, negative, mass).
    fn cell_masses(&self) -> Vec<(u16, u16, f64)> {
        let mut masses = Vec::with_capacity(192);
        for (mode, matrix) in self.order().into_iter().enumerate() {
            for hexagram in 0u8..64 {
                let mass = self.modes.p(mode) * self.masks.p(usize::from(hexagram ^ self.hexagram));
                let cell = generate(MatrixFamily::ALL[matrix], hexagram);
                masses.push((
                    u16::from(cell.positive.address()),
                    u16::from(cell.negative.address()),
                    mass,
                ));
            }
        }
        masses
    }
}

impl Predictor for CellStateP {
    fn name(&self) -> String {
        "qlc1b-cell-state".into()
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let breaking = self.modes.p(CellState::BREAK);
        let t = history.len();
        if t % 2 == 0 {
            for (positive, _, mass) in self.cell_masses() {
                dist[usize::from(positive)] += mass;
            }
            spread(&self.literal, breaking, dist);
        } else {
            let positive = history[t - 1];
            for (p, negative, mass) in self.cell_masses() {
                if p == positive {
                    dist[usize::from(negative)] += mass;
                }
            }
            spread(
                &self.literal,
                breaking * self.literal.p(usize::from(positive)),
                dist,
            );
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let t = history.len();
        if t % 2 == 0 {
            return;
        }
        let pair = (history[t - 1], actual);
        let found = self.order().into_iter().enumerate().find_map(|(mode, m)| {
            self.cells
                .hexagrams
                .get(&(m, pair.0, pair.1))
                .map(|h| (mode, m, *h))
        });
        match found {
            Some((mode, matrix, hexagram)) => {
                self.modes.observe(mode);
                self.masks.observe(usize::from(hexagram ^ self.hexagram));
                self.matrix = matrix;
                self.hexagram = hexagram;
            }
            None => {
                self.modes.observe(CellState::BREAK);
                self.literal.observe(usize::from(pair.0));
                self.literal.observe(usize::from(pair.1));
            }
        }
    }
}

/// QLC-2 as a predictor.
pub struct FrameTiedP {
    frame: FrameTied,
    models: HashMap<(usize, bool, usize), Freq>,
    literal: Freq,
    lens: usize,
    last_step: usize,
}

impl FrameTiedP {
    pub fn boxed(tied: bool) -> Box<dyn Predictor> {
        Box::new(Self {
            frame: FrameTied::new(tied),
            models: HashMap::new(),
            literal: Freq::new(12),
            lens: 0,
            last_step: 8,
        })
    }

    fn reading(&mut self, history: &[u16]) -> ([u8; 7], Option<usize>, (usize, bool, usize)) {
        let recent = &history[history.len().saturating_sub(FrameTied::WINDOW)..];
        self.lens = self.frame.best_lens(recent, self.lens);
        let scale = self.frame.scales[self.lens];
        let anchor = history
            .last()
            .and_then(|p| scale.iter().position(|s| u16::from(*s) == *p));
        let key = (
            if self.frame.tied { 0 } else { self.lens + 1 },
            anchor.is_some(),
            self.last_step,
        );
        (scale, anchor, key)
    }
}

impl Predictor for FrameTiedP {
    fn name(&self) -> String {
        self.frame.name()
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let (scale, anchor, key) = self.reading(history);
        let model = self
            .models
            .entry(key)
            .or_insert_with(|| Freq::new(8))
            .clone();
        for step in 0..7 {
            let degree = anchor.map_or(step, |a| (a + step) % 7);
            dist[usize::from(scale[degree])] += model.p(step);
        }
        spread(&self.literal, model.p(7), dist);
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let (scale, anchor, key) = self.reading(history);
        let step = scale
            .iter()
            .position(|s| u16::from(*s) == actual)
            .map_or(7, |d| anchor.map_or(d, |a| (d + 7 - a) % 7));
        self.models
            .entry(key)
            .or_insert_with(|| Freq::new(8))
            .observe(step);
        if step == 7 {
            self.literal.observe(usize::from(actual));
        }
        self.last_step = step;
    }
}

/// QLC-3 as a predictor: every operator that reaches a hexagram lends it
/// its probability; the escape spreads over the line masks.
pub struct OperatorGrammarP {
    ops: Vec<KernelOp>,
    models: HashMap<usize, Freq>,
    masks: Freq,
    first: Freq,
    last_op: usize,
}

impl OperatorGrammarP {
    pub fn boxed() -> Box<dyn Predictor> {
        let ops = KernelOp::all();
        let last_op = ops.len() + 1;
        Box::new(Self {
            ops,
            models: HashMap::new(),
            masks: Freq::new(64),
            first: Freq::new(64),
            last_op,
        })
    }

    fn model(&mut self) -> &mut Freq {
        let size = self.ops.len() + 1;
        self.models
            .entry(self.last_op)
            .or_insert_with(|| Freq::new(size))
    }
}

impl Predictor for OperatorGrammarP {
    fn name(&self) -> String {
        "qlc3-operator-grammar".into()
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let Some(previous) = history.last().map(|p| *p as u8) else {
            spread(&self.first, 1.0, dist);
            return;
        };
        let model = self.model().clone();
        for (i, op) in self.ops.iter().enumerate() {
            dist[usize::from(op.apply(previous))] += model.p(i);
        }
        let escape = model.p(self.ops.len());
        for mask in 0..64 {
            dist[usize::from(previous) ^ mask] += escape * self.masks.p(mask);
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let Some(previous) = history.last().map(|p| *p as u8) else {
            self.first.observe(usize::from(actual));
            return;
        };
        let escape = self.ops.len();
        let op = self
            .ops
            .iter()
            .position(|op| op.apply(previous) == actual as u8)
            .unwrap_or(escape);
        self.model().observe(op);
        if op == escape {
            self.masks.observe(usize::from(actual as u8 ^ previous));
        }
        self.last_op = op;
    }
}

/// The group a match is taken up to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Invariance {
    /// Symbols as they are: the generic LZ-style match.
    Exact,
    /// Pitch classes up to the twelve lens transpositions: match intervals.
    Transposition,
    /// Hexagrams up to XOR line masks (complement included): match masks.
    LineMask,
}

impl Invariance {
    fn reduce(self, previous: Option<u16>, symbol: u16) -> u16 {
        match (self, previous) {
            (Invariance::Exact, _) | (_, None) => symbol,
            (Invariance::Transposition, Some(p)) => (symbol + 12 - p) % 12,
            (Invariance::LineMask, Some(p)) => symbol ^ p,
        }
    }

    fn restore(self, previous: Option<u16>, reduced: u16) -> u16 {
        match (self, previous) {
            (Invariance::Exact, _) | (_, None) => reduced,
            (Invariance::Transposition, Some(p)) => (reduced + p) % 12,
            (Invariance::LineMask, Some(p)) => reduced ^ p,
        }
    }
}

/// Wave 3: the longest earlier recurrence of the recent (reduced) trajectory
/// predicts its own continuation, with a confidence learned per match length.
pub struct MatchModel {
    invariance: Invariance,
    reduced: Vec<u16>,
    index: HashMap<u64, usize>,
    pointer: Option<usize>,
    length: usize,
    hits: Vec<Freq>,
}

impl MatchModel {
    const MIN: usize = 4;

    pub fn boxed(invariance: Invariance) -> Box<dyn Predictor> {
        Box::new(Self {
            invariance,
            reduced: Vec::new(),
            index: HashMap::new(),
            pointer: None,
            length: 0,
            hits: (0..=16).map(|_| Freq::with_prior(&[1, 1])).collect(),
        })
    }

    fn key(&self) -> Option<u64> {
        let n = self.reduced.len();
        (n >= Self::MIN).then(|| {
            self.reduced[n - Self::MIN..]
                .iter()
                .fold(0xcbf2_9ce4_8422_2325u64, |h, s| {
                    (h ^ u64::from(*s)).wrapping_mul(0x100_0000_01b3)
                })
        })
    }
}

impl Predictor for MatchModel {
    fn name(&self) -> String {
        match self.invariance {
            Invariance::Exact => "match-exact".into(),
            Invariance::Transposition => "match-transposition".into(),
            Invariance::LineMask => "match-line-mask".into(),
        }
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let uniform = 1.0 / dist.len() as f64;
        let Some(pointer) = self.pointer else {
            dist.iter_mut().for_each(|p| *p = uniform);
            return;
        };
        let confidence = self.hits[self.length.min(16)].p(1);
        let predicted = self
            .invariance
            .restore(history.last().copied(), self.reduced[pointer]);
        dist.iter_mut()
            .for_each(|p| *p = (1.0 - confidence) * uniform);
        if let Some(p) = dist.get_mut(usize::from(predicted)) {
            *p += confidence;
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let reduced = self.invariance.reduce(history.last().copied(), actual);
        if let Some(pointer) = self.pointer {
            let hit = self.reduced[pointer] == reduced;
            self.hits[self.length.min(16)].observe(usize::from(hit));
            if hit {
                self.length += 1;
                self.pointer = Some(pointer + 1);
            } else {
                self.length = 0;
                self.pointer = None;
            }
        }
        self.reduced.push(reduced);
        if let Some(key) = self.key() {
            if self.pointer.is_none() {
                if let Some(&next) = self.index.get(&key) {
                    self.pointer = Some(next);
                    self.length = 0;
                }
            }
            self.index.insert(key, self.reduced.len());
        }
        if self.pointer.is_some_and(|p| p >= self.reduced.len()) {
            self.pointer = None;
            self.length = 0;
        }
    }
}

/// The wave-2/3 line-up for one stream kind: the order-k baseline mixture,
/// the same mixture with the generic match, and with the QL predictors.
pub fn order_factories(alphabet: usize) -> Vec<Factory> {
    (0..=2)
        .map(|k| Box::new(move || OrderKP::boxed(k, alphabet)) as Factory)
        .collect()
}

/// Wave 4: order-k over pitch classes with contexts tied across a rotation
/// group. `group = 12` ties all twelve lens transpositions (interval
/// contexts); `group = 6` ties only the six whole-tone rotations that keep a
/// pitch on its helix (direct = even, conjugate = odd pitch classes in the
/// chromatic basis), so the helix face stays in the context; `group = 1` is
/// plain order-k.
pub struct HelixTied {
    k: usize,
    group: u16,
    models: HashMap<(u16, Vec<u16>), Freq>,
}

impl HelixTied {
    pub fn boxed(k: usize, group: u16) -> Box<dyn Predictor> {
        assert!(
            matches!(group, 1 | 6 | 12),
            "a rotation group of the twelve"
        );
        Box::new(Self {
            k,
            group,
            models: HashMap::new(),
        })
    }

    /// The shift that carries the previous pitch to its group representative.
    fn shift(&self, previous: u16) -> u16 {
        match self.group {
            12 => previous,
            6 => previous - previous % 2,
            _ => 0,
        }
    }

    fn context(&self, history: &[u16]) -> (u16, (u16, Vec<u16>)) {
        let shift = history.last().map_or(0, |p| self.shift(*p));
        let face = history.last().map_or(12, |p| {
            if self.group == 12 {
                0
            } else {
                p - self.shift(*p)
            }
        });
        let reduced = history
            .iter()
            .rev()
            .take(self.k)
            .map(|p| (p + 12 - shift) % 12)
            .collect();
        (shift, (face, reduced))
    }
}

impl Predictor for HelixTied {
    fn name(&self) -> String {
        format!("helix-tied-{}-order-{}", self.group, self.k)
    }

    fn predict(&mut self, history: &[u16], dist: &mut [f64]) {
        let (shift, key) = self.context(history);
        match self.models.get(&key) {
            Some(model) => {
                for reduced in 0..12u16 {
                    dist[usize::from((reduced + shift) % 12)] += model.p(usize::from(reduced));
                }
            }
            None => dist.iter_mut().for_each(|p| *p = 1.0),
        }
    }

    fn update(&mut self, history: &[u16], actual: u16) {
        let (shift, key) = self.context(history);
        self.models
            .entry(key)
            .or_insert_with(|| Freq::new(12))
            .observe(usize::from((actual + 12 - shift) % 12));
    }
}
