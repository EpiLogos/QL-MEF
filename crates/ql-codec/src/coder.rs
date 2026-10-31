//! The entropy coder every QLC codec shares: an LZMA-style range coder with
//! carry propagation, and adaptive frequency models over small alphabets.
//!
//! A codec is written once as a walk over its symbols through an [`Io`]: on
//! encode the walk carries the input and the coder writes; on decode the same
//! walk asks the coder for each symbol. Both directions run the same model
//! updates in the same order, so the decoder reproduces the encoder exactly.

const TOP: u32 = 1 << 24;
/// Count added per observation, and the total at which counts are halved.
const INCREMENT: u32 = 24;
const LIMIT: u32 = 1 << 16;

pub struct Encoder {
    low: u64,
    range: u32,
    cache: u8,
    cache_size: u64,
    out: Vec<u8>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self {
            low: 0,
            range: u32::MAX,
            cache: 0,
            cache_size: 1,
            out: Vec::new(),
        }
    }
}

impl Encoder {
    pub fn encode(&mut self, cum: u32, freq: u32, total: u32) {
        let r = self.range / total;
        self.low += u64::from(r) * u64::from(cum);
        self.range = r * freq;
        while self.range < TOP {
            self.range <<= 8;
            self.shift_low();
        }
    }

    fn shift_low(&mut self) {
        if self.low < 0xFF00_0000 || self.low > 0xFFFF_FFFF {
            let carry = (self.low >> 32) as u8;
            let mut temp = self.cache;
            loop {
                self.out.push(temp.wrapping_add(carry));
                temp = 0xFF;
                self.cache_size -= 1;
                if self.cache_size == 0 {
                    break;
                }
            }
            self.cache = (self.low >> 24) as u8;
        }
        self.cache_size += 1;
        self.low = (self.low & 0x00FF_FFFF) << 8;
    }

    pub fn finish(mut self) -> Vec<u8> {
        for _ in 0..5 {
            self.shift_low();
        }
        self.out
    }
}

pub struct Decoder<'a> {
    data: &'a [u8],
    pos: usize,
    range: u32,
    code: u32,
    step: u32,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut decoder = Self {
            data,
            pos: 0,
            range: u32::MAX,
            code: 0,
            step: 1,
        };
        for _ in 0..5 {
            decoder.code = (decoder.code << 8) | u32::from(decoder.next_byte());
        }
        decoder
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.data.get(self.pos).copied().unwrap_or(0);
        self.pos += 1;
        byte
    }

    pub fn target(&mut self, total: u32) -> u32 {
        self.step = self.range / total;
        (self.code / self.step).min(total - 1)
    }

    pub fn consume(&mut self, cum: u32, freq: u32) {
        self.code -= self.step * cum;
        self.range = self.step * freq;
        while self.range < TOP {
            self.code = (self.code << 8) | u32::from(self.next_byte());
            self.range <<= 8;
        }
    }
}

/// An adaptive frequency model over `0..n`.
#[derive(Debug, Clone)]
pub struct Freq {
    counts: Vec<u32>,
    total: u32,
}

impl Freq {
    pub fn new(n: usize) -> Self {
        Self::with_prior(&vec![1; n])
    }

    /// Start from given counts: a prior the decoder knows without being told.
    pub fn with_prior(prior: &[u32]) -> Self {
        let counts: Vec<u32> = prior.iter().map(|c| (*c).max(1)).collect();
        let total = counts.iter().sum();
        Self { counts, total }
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// The model's current probability of `symbol`.
    pub fn p(&self, symbol: usize) -> f64 {
        f64::from(self.counts[symbol]) / f64::from(self.total)
    }

    /// Learn one observation of `symbol` without coding it.
    pub fn observe(&mut self, symbol: usize) {
        self.update(symbol);
    }

    fn update(&mut self, symbol: usize) {
        self.counts[symbol] += INCREMENT;
        self.total += INCREMENT;
        if self.total > LIMIT {
            self.total = 0;
            for count in &mut self.counts {
                *count = (*count).div_ceil(2);
                self.total += *count;
            }
        }
    }
}

/// One direction of a codec's walk.
pub enum Io<'a> {
    Encode(Encoder),
    Decode(Decoder<'a>),
    /// Encode without output: each symbol's ideal cost, `log2(total/freq)`
    /// bits under exactly the counts the encoder would use.
    Measure(Vec<f64>),
}

impl Io<'_> {
    /// Code `value` (ignored on decode) under `model`, update it, and return
    /// the symbol actually coded.
    pub fn symbol(&mut self, model: &mut Freq, value: usize) -> usize {
        let symbol = self.code(&model.counts, model.total, value);
        model.update(symbol);
        symbol
    }

    /// Code `value` (ignored on decode) under fixed counts, every one ≥ 1.
    pub fn code(&mut self, counts: &[u32], total: u32, value: usize) -> usize {
        match self {
            Io::Encode(encoder) => {
                assert!(value < counts.len(), "symbol {value} outside its model");
                let cum = counts[..value].iter().sum();
                encoder.encode(cum, counts[value], total);
                value
            }
            Io::Decode(decoder) => {
                let target = decoder.target(total);
                let mut cum = 0;
                let mut symbol = 0;
                while cum + counts[symbol] <= target {
                    cum += counts[symbol];
                    symbol += 1;
                }
                decoder.consume(cum, counts[symbol]);
                symbol
            }
            Io::Measure(costs) => {
                assert!(value < counts.len(), "symbol {value} outside its model");
                costs.push((f64::from(total) / f64::from(counts[value])).log2());
                value
            }
        }
    }

    pub const fn encoding(&self) -> bool {
        matches!(self, Io::Encode(_) | Io::Measure(_))
    }
}
