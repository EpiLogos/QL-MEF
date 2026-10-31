//! QL codec experiments (QLC-1..4) over the QLC-0 fold tax.
//!
//! Every codec here is lossless and deterministic: it is one walk over a
//! symbol stream through the shared range coder ([`coder::Io`]), run forward
//! to encode and again to decode. The QL codecs differ from the baselines
//! only in what the decoder is allowed to know in advance — the kernel's own
//! laws, called from `ql-core` and `ql-mef`, never data about the stream:
//!
//! - QLC-1 [`codecs::GenerationLaw`]: a generation cell's negative codon is
//!   coded as its rank among the two the kernel yields for its positive codon
//!   under the tracked matrix (the 1-bit tax of QLC-0), with an escape;
//! - QLC-2 [`codecs::FrameTied`]: a pitch is coded as a scale-degree step in
//!   the lens's canonical Context-Frame cut, one model shared by all twelve
//!   lenses (MEF rotation tying), against the same model untied per lens;
//! - QLC-3 [`codecs::OperatorGrammar`]: a hexagram is coded as the kernel
//!   operator that carries its predecessor to it (line change, complement,
//!   matrix action, nuclear), with an escape to the line mask.
//!
//! Wave 2 ([`predict::Mixed`]) carries every codec as a predictor inside a
//! fixed-share mixture with the order-k models, so a QL prior costs little
//! where its law fails. Wave 3 ([`predict::MatchModel`]) matches earlier
//! trajectories exactly or up to the kernel's group action (lens
//! transposition, XOR line mask). Wave 4 ([`predict::HelixTied`]) ties pitch
//! contexts across all twelve lens rotations, across only the six whole-tone
//! rotations that keep a pitch on its P / P′ helix, or not at all.
//!
//! QLT ([`thought`]) turns the instrument on classified thought: a
//! labelling (T level, MEF cell, context frame) is coded as a two-part code,
//! labels then text with the label as context, against no labels and the
//! same labels shuffled; `qlt` also tests whether T and MEF positions align.
//!
//! Baselines: adaptive order-k context models on the same symbols, and zstd /
//! xz on the same bytes. `qlc` (src/bin) runs the sweep and writes
//! `results/qlc-sweep.tsv`.
pub mod codecs;
pub mod coder;
pub mod predict;
pub mod source;
pub mod thought;

use coder::{Decoder, Encoder, Io};

pub trait Codec {
    fn name(&self) -> String;
    /// Walk `n` symbols. On encode `input` carries them; on decode it is
    /// `None` and the walk returns what the coder yields.
    fn walk(&self, io: &mut Io, input: Option<&[u16]>, n: usize) -> Vec<u16>;
}

/// Encode: a 4-byte little-endian symbol count, then the range-coded walk.
pub fn compress(codec: &dyn Codec, symbols: &[u16]) -> Vec<u8> {
    let mut io = Io::Encode(Encoder::default());
    codec.walk(&mut io, Some(symbols), symbols.len());
    let Io::Encode(encoder) = io else {
        unreachable!("an encode walk keeps its encoder")
    };
    let mut out = u32::try_from(symbols.len())
        .expect("stream fits a u32 count")
        .to_le_bytes()
        .to_vec();
    out.extend(encoder.finish());
    out
}

pub fn decompress(codec: &dyn Codec, bytes: &[u8]) -> Vec<u16> {
    let n = u32::from_le_bytes(bytes[..4].try_into().expect("4-byte count")) as usize;
    let mut io = Io::Decode(Decoder::new(&bytes[4..]));
    codec.walk(&mut io, None, n)
}

/// The input symbol at `t` on encode; a placeholder the coder replaces on decode.
pub(crate) fn at(input: Option<&[u16]>, t: usize) -> usize {
    input.map_or(0, |symbols| usize::from(symbols[t]))
}
