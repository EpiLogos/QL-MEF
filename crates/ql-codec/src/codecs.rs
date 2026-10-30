//! The baseline and QL codecs. Each is a walk: state is rebuilt from symbols
//! already coded, so encoder and decoder hold the same state at every step.
use std::collections::HashMap;

use ql_core::{MatrixFamily, all_cells, complement, nuclear_hexagram};
use ql_mef::{LensId, MusicalBasis, cf_diatonic_cut};

use crate::coder::{Freq, Io};
use crate::{Codec, at};

/// Adaptive order-k context model: the generic baseline.
pub struct OrderK {
    pub k: usize,
    pub alphabet: usize,
}

impl Codec for OrderK {
    fn name(&self) -> String {
        format!("order-{}", self.k)
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut models: HashMap<u64, Freq> = HashMap::new();
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            let key = out.iter().rev().take(self.k).fold(0u64, |key, s| {
                key * (self.alphabet as u64 + 1) + u64::from(*s) + 1
            });
            let model = models
                .entry(key)
                .or_insert_with(|| Freq::new(self.alphabet));
            out.push(io.symbol(model, at(input, t)) as u16);
        }
        out
    }
}

/// Ablation for QLC-1: the pair structure without the law. A positive codon
/// is read against the previous positive, a negative against its positive.
pub struct PairContext;

impl Codec for PairContext {
    fn name(&self) -> String {
        "pair-context".into()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut models: HashMap<(bool, u16), Freq> = HashMap::new();
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            let key = if t % 2 == 0 {
                (false, if t >= 2 { out[t - 2] } else { 64 })
            } else {
                (true, out[t - 1])
            };
            let model = models.entry(key).or_insert_with(|| Freq::new(64));
            out.push(io.symbol(model, at(input, t)) as u16);
        }
        out
    }
}

/// QLC-1: the generation law as the decoder's prior. Stream: positive,
/// negative, positive, negative … codons of generation cells.
pub struct GenerationLaw {
    /// `[matrix][positive]` → the negatives the kernel yields.
    pub(crate) candidates: Vec<Vec<Vec<u16>>>,
}

impl Default for GenerationLaw {
    fn default() -> Self {
        let mut candidates = vec![vec![Vec::new(); 64]; 3];
        for cell in all_cells() {
            let m = usize::from(cell.family.matrix_number() - 1);
            let list: &mut Vec<u16> = &mut candidates[m][usize::from(cell.positive.address())];
            let negative = u16::from(cell.negative.address());
            if !list.contains(&negative) {
                list.push(negative);
            }
        }
        Self { candidates }
    }
}

impl GenerationLaw {
    pub(crate) const ESCAPE: usize = 6;

    pub(crate) fn ordered(&self, matrix: usize, positive: u16) -> Vec<u16> {
        let mut list = Vec::with_capacity(6);
        for m in std::iter::once(matrix).chain((0..3).filter(|m| *m != matrix)) {
            for negative in &self.candidates[m][usize::from(positive)] {
                if !list.contains(negative) {
                    list.push(*negative);
                }
            }
        }
        list
    }
}

impl Codec for GenerationLaw {
    fn name(&self) -> String {
        "qlc1-generation-law".into()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut positives: HashMap<u16, Freq> = HashMap::new();
        let mut ranks: HashMap<(usize, usize), Freq> = HashMap::new();
        let mut literal = Freq::new(64);
        let mut matrix = 0usize;
        let mut last_rank = 0usize;
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            if t % 2 == 0 {
                let key = if t >= 2 { out[t - 2] } else { 64 };
                let model = positives.entry(key).or_insert_with(|| Freq::new(64));
                out.push(io.symbol(model, at(input, t)) as u16);
                continue;
            }
            let positive = out[t - 1];
            let list = self.ordered(matrix, positive);
            let wanted = if io.encoding() {
                let actual = at(input, t) as u16;
                list.iter()
                    .position(|c| *c == actual)
                    .unwrap_or(Self::ESCAPE)
            } else {
                0
            };
            let model = ranks
                .entry((list.len(), last_rank.min(2)))
                .or_insert_with(|| Freq::with_prior(&[8, 8, 1, 1, 1, 1, 2]));
            let rank = io.symbol(model, wanted);
            let negative = if rank == Self::ESCAPE {
                io.symbol(&mut literal, at(input, t)) as u16
            } else {
                list[rank]
            };
            if !self.candidates[matrix][usize::from(positive)].contains(&negative) {
                if let Some(m) =
                    (0..3).find(|m| self.candidates[*m][usize::from(positive)].contains(&negative))
                {
                    matrix = m;
                }
            }
            last_rank = rank;
            out.push(negative);
        }
        out
    }
}

/// QLC-1b: code the cell, not its codons. A generation cell's two codons
/// determine its hexagram under its matrix (`a·b` and `c·d` are the trigram
/// pairs), so the codec carries the hexagram's line mask and the matrix, and
/// both codons follow from `ql_core::generate`. A pair that is no cell is
/// coded as two literal codons.
pub struct CellState {
    pub(crate) hexagrams: HashMap<(usize, u16, u16), u8>,
}

impl Default for CellState {
    fn default() -> Self {
        let mut hexagrams = HashMap::new();
        for cell in all_cells() {
            let key = (
                usize::from(cell.family.matrix_number() - 1),
                u16::from(cell.positive.address()),
                u16::from(cell.negative.address()),
            );
            let previous = hexagrams.insert(key, cell.hexagram);
            assert!(
                previous.is_none(),
                "a matrix's cells have distinct codon pairs"
            );
        }
        Self { hexagrams }
    }
}

impl CellState {
    pub(crate) const BREAK: usize = 3;
}

impl Codec for CellState {
    fn name(&self) -> String {
        "qlc1b-cell-state".into()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut modes = Freq::with_prior(&[64, 1, 1, 2]);
        let mut masks = Freq::new(64);
        let mut literal = Freq::new(64);
        let mut matrix = 0usize;
        let mut hexagram = 0u8;
        let mut out: Vec<u16> = Vec::with_capacity(n);
        let mut t = 0;
        while t < n {
            if t + 1 == n {
                out.push(io.symbol(&mut literal, at(input, t)) as u16);
                break;
            }
            let others: Vec<usize> = (0..3).filter(|m| *m != matrix).collect();
            let order = [matrix, others[0], others[1]];
            let (mode, target) = if io.encoding() {
                let pair = (at(input, t) as u16, at(input, t + 1) as u16);
                order
                    .iter()
                    .enumerate()
                    .find_map(|(i, m)| self.hexagrams.get(&(*m, pair.0, pair.1)).map(|h| (i, *h)))
                    .unwrap_or((Self::BREAK, 0))
            } else {
                (0, 0)
            };
            let mode = io.symbol(&mut modes, mode);
            if mode == Self::BREAK {
                out.push(io.symbol(&mut literal, at(input, t)) as u16);
                out.push(io.symbol(&mut literal, at(input, t + 1)) as u16);
            } else {
                matrix = order[mode];
                let mask = io.symbol(&mut masks, usize::from(target ^ hexagram));
                hexagram ^= mask as u8;
                let cell = ql_core::generate(MatrixFamily::ALL[matrix], hexagram);
                out.push(u16::from(cell.positive.address()));
                out.push(u16::from(cell.negative.address()));
            }
            t += 2;
        }
        out
    }
}

/// The kernel operators QLC-3 composes, in the order the coder tries them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelOp {
    Identity,
    LineChange(u8),
    Complement,
    Matrix(MatrixFamily),
    Nuclear,
}

impl KernelOp {
    pub fn all() -> Vec<KernelOp> {
        let mut ops = vec![KernelOp::Identity];
        ops.extend((0..6).map(KernelOp::LineChange));
        ops.push(KernelOp::Complement);
        ops.extend(MatrixFamily::ALL.map(KernelOp::Matrix));
        ops.push(KernelOp::Nuclear);
        ops
    }

    pub fn apply(self, hexagram: u8) -> u8 {
        match self {
            KernelOp::Identity => hexagram,
            KernelOp::LineChange(line) => hexagram ^ (1 << line),
            KernelOp::Complement => complement(hexagram),
            KernelOp::Matrix(family) => family.apply_to_hexagram(hexagram),
            KernelOp::Nuclear => nuclear_hexagram(hexagram),
        }
    }
}

/// QLC-3: a hexagram stream as a sequence of kernel-operator applications,
/// each read against the previous operator, with an escape to the line mask.
pub struct OperatorGrammar {
    ops: Vec<KernelOp>,
}

impl Default for OperatorGrammar {
    fn default() -> Self {
        Self {
            ops: KernelOp::all(),
        }
    }
}

impl Codec for OperatorGrammar {
    fn name(&self) -> String {
        "qlc3-operator-grammar".into()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let escape = self.ops.len();
        let mut op_models: HashMap<usize, Freq> = HashMap::new();
        let mut masks = Freq::new(64);
        let mut first = Freq::new(64);
        let mut last_op = escape + 1;
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            if t == 0 {
                out.push(io.symbol(&mut first, at(input, t)) as u16);
                continue;
            }
            let previous = out[t - 1] as u8;
            let wanted = if io.encoding() {
                let actual = at(input, t) as u8;
                self.ops
                    .iter()
                    .position(|op| op.apply(previous) == actual)
                    .unwrap_or(escape)
            } else {
                0
            };
            let model = op_models
                .entry(last_op)
                .or_insert_with(|| Freq::new(escape + 1));
            let op = io.symbol(model, wanted);
            let current = if op == escape {
                previous ^ io.symbol(&mut masks, at(input, t) ^ usize::from(previous)) as u8
            } else {
                self.ops[op].apply(previous)
            };
            last_op = op;
            out.push(u16::from(current));
        }
        out
    }
}

/// Ablation for QLC-3: the XOR line group alone, no kernel operators.
pub struct XorMask;

impl Codec for XorMask {
    fn name(&self) -> String {
        "xor-mask".into()
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut masks = Freq::new(64);
        let mut first = Freq::new(64);
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            let symbol = if t == 0 {
                io.symbol(&mut first, at(input, t)) as u16
            } else {
                let previous = usize::from(out[t - 1]);
                (previous ^ io.symbol(&mut masks, at(input, t) ^ previous)) as u16
            };
            out.push(symbol);
        }
        out
    }
}

/// The twelve lenses' canonical Context-Frame cuts, each as its seven pitch
/// classes in ascending order (the scale degrees of that lens).
pub fn lens_scales() -> [[u8; 7]; 12] {
    std::array::from_fn(|i| {
        let mut pitches = cf_diatonic_cut(MusicalBasis::Chromatic, LensId::ALL[i]).pitches;
        pitches.sort_unstable();
        pitches
    })
}

/// QLC-2: pitch classes coded as degree steps in the lens cut the recent
/// notes fit best. `tied` shares one step model across all twelve lenses (the
/// MEF rotation law); untied keeps one per lens.
pub struct FrameTied {
    pub tied: bool,
    pub(crate) scales: [[u8; 7]; 12],
}

impl FrameTied {
    pub(crate) const WINDOW: usize = 6;
    const ESCAPE: usize = 7;

    pub fn new(tied: bool) -> Self {
        Self {
            tied,
            scales: lens_scales(),
        }
    }

    pub(crate) fn best_lens(&self, recent: &[u16], current: usize) -> usize {
        let fit = |lens: usize| {
            recent
                .iter()
                .filter(|p| self.scales[lens].contains(&(**p as u8)))
                .count()
        };
        let best = (0..12).map(fit).max().unwrap_or(0);
        if fit(current) == best {
            current
        } else {
            (0..12).find(|lens| fit(*lens) == best).unwrap_or(current)
        }
    }
}

impl Codec for FrameTied {
    fn name(&self) -> String {
        if self.tied {
            "qlc2-frame-tied".into()
        } else {
            "qlc2-frame-untied".into()
        }
    }

    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16> {
        let mut models: HashMap<(usize, bool, usize), Freq> = HashMap::new();
        let mut literal = Freq::new(12);
        let mut lens = 0usize;
        let mut last_step = 8usize;
        let mut out: Vec<u16> = Vec::with_capacity(n);
        for t in 0..n {
            let recent = &out[t.saturating_sub(Self::WINDOW)..];
            lens = self.best_lens(recent, lens);
            let scale = self.scales[lens];
            let degree_of = |pitch: u16| scale.iter().position(|p| u16::from(*p) == pitch);
            let anchor = out.last().and_then(|p| degree_of(*p));
            let wanted = if io.encoding() {
                match degree_of(at(input, t) as u16) {
                    Some(d) => anchor.map_or(d, |a| (d + 7 - a) % 7),
                    None => Self::ESCAPE,
                }
            } else {
                0
            };
            let key = (
                if self.tied { 0 } else { lens + 1 },
                anchor.is_some(),
                last_step,
            );
            let model = models.entry(key).or_insert_with(|| Freq::new(8));
            let step = io.symbol(model, wanted);
            let pitch = if step == Self::ESCAPE {
                io.symbol(&mut literal, at(input, t)) as u16
            } else {
                u16::from(scale[anchor.map_or(step, |a| (a + step) % 7)])
            };
            last_step = step;
            out.push(pitch);
        }
        out
    }
}
