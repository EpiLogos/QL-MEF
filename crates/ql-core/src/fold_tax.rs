//! QLC-0 — the fold tax: how much of a source each many-to-one kernel map
//! leaves behind, and the exact side information that restores it.
//!
//! The kernel already folds. Geometry presents a 0/1 whole over a retained
//! basis; the M2→M3 epogdoon carries 72 vibration states onto the 64-form
//! board; the quaternion reading carries 64 codons onto 46 points; a
//! generation cell yields its negative codon from almost everything its
//! positive codon already says. A lossless code over any of these maps must
//! send the image and, beside it, which preimage the source was. That
//! remainder is the **tax**:
//!
//! ```text
//! H(source) = H(image) + H(source | image)
//!                           └── the tax: side information a decoder needs
//! ```
//!
//! [`FoldCode`] is the tax as an executable code: `split` gives the image and
//! the source's rank among that image's preimages; `join` inverts it exactly.
//! [`fold_tax_table`] measures every kernel fold under the uniform law and
//! under the distribution the kernel's own generation law produces, and
//! `fixtures/kernel/qlc0-fold-tax-v1.tsv` is its portable receipt.
//!
//! Nothing here is a compressor or a claim that QL compresses arbitrary
//! data. It is the floor every later QL codec (QLC-1 prediction, QLC-2
//! Context-Frame mixing, QLC-3 operator grammar) must pay: a code that sends
//! less than the tax for a map is not lossless. The maps are the kernel's own
//! homes, called, not restated.
use std::collections::BTreeMap;

use crate::{
    Codon64, M2_VIBRATION_CYCLE, MatrixFamily, QlCoordinate, QlFace, QlPosition,
    Transduction18to16, all_cells, apply_epogdoon_compression, carrier, det_shadow,
    m3_codon_amino_index, nuclear_hexagram, quat_from_codon,
};

pub const FOLD_TAX_CONTRACT: &str = "ql.qlc0.fold-tax/v1";

/// One measured fold: a kernel map under one source distribution.
#[derive(Debug, Clone, PartialEq)]
pub struct FoldTax {
    pub map_ref: &'static str,
    pub distribution_ref: &'static str,
    /// Distinct sources carrying weight.
    pub support: usize,
    /// Distinct images those sources reach.
    pub image: usize,
    /// The largest preimage: the widest residual alphabet a decoder meets.
    pub max_preimage: usize,
    /// Preimage size → how many images have it.
    pub preimage_histogram: BTreeMap<usize, usize>,
    /// `H(source)` in bits.
    pub source_bits: f64,
    /// `H(image)` in bits.
    pub image_bits: f64,
    /// `H(source | image)` in bits: the tax.
    pub tax_bits: f64,
}

impl FoldTax {
    /// Measure a map given as weighted `(source, image)` pairs. Each source
    /// appears once; a repeated source sums its weight and must keep its image.
    pub fn measure<S, I>(
        map_ref: &'static str,
        distribution_ref: &'static str,
        entries: impl IntoIterator<Item = (S, I, u64)>,
    ) -> Self
    where
        S: Ord + Copy,
        I: Ord + Copy,
    {
        let mut sources: BTreeMap<S, (I, u64)> = BTreeMap::new();
        for (source, image, weight) in entries {
            if weight == 0 {
                continue;
            }
            let slot = sources.entry(source).or_insert((image, 0));
            assert!(slot.0 == image, "{map_ref}: a source must have one image");
            slot.1 += weight;
        }
        let mut images: BTreeMap<I, Vec<u64>> = BTreeMap::new();
        for (image, weight) in sources.values() {
            images.entry(*image).or_default().push(*weight);
        }
        let total: u64 = sources.values().map(|(_, weight)| weight).sum();
        let source_bits = entropy(sources.values().map(|(_, weight)| *weight), total);
        let image_bits = entropy(images.values().map(|w| w.iter().sum()), total);
        let tax_bits = images
            .values()
            .map(|weights| {
                let mass: u64 = weights.iter().sum();
                mass as f64 / total as f64 * entropy(weights.iter().copied(), mass)
            })
            .sum();
        let mut preimage_histogram = BTreeMap::new();
        for weights in images.values() {
            *preimage_histogram.entry(weights.len()).or_insert(0) += 1;
        }
        Self {
            map_ref,
            distribution_ref,
            support: sources.len(),
            image: images.len(),
            max_preimage: images.values().map(Vec::len).max().unwrap_or(0),
            preimage_histogram,
            source_bits,
            image_bits,
            tax_bits,
        }
    }

    pub fn tsv_row(&self) -> String {
        let histogram = self
            .preimage_histogram
            .iter()
            .map(|(size, count)| format!("{size}:{count}"))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{:.6}\t{:.6}\t{:.6}",
            self.map_ref,
            self.distribution_ref,
            self.support,
            self.image,
            self.max_preimage,
            histogram,
            self.source_bits,
            self.image_bits,
            self.tax_bits
        )
    }
}

fn entropy(weights: impl IntoIterator<Item = u64>, total: u64) -> f64 {
    weights
        .into_iter()
        .filter(|weight| *weight > 0)
        .map(|weight| {
            let p = weight as f64 / total as f64;
            -p * p.log2()
        })
        .sum::<f64>()
        .max(0.0)
}

/// The tax as a lossless code over a finite map: image plus preimage rank.
#[derive(Debug, Clone)]
pub struct FoldCode<S, I> {
    preimages: BTreeMap<I, Vec<S>>,
    images: BTreeMap<S, I>,
}

impl<S: Ord + Copy, I: Ord + Copy> FoldCode<S, I> {
    pub fn new(map: impl IntoIterator<Item = (S, I)>) -> Self {
        let mut preimages: BTreeMap<I, Vec<S>> = BTreeMap::new();
        let mut images = BTreeMap::new();
        for (source, image) in map {
            if images.insert(source, image).is_none() {
                preimages.entry(image).or_default().push(source);
            }
        }
        for sources in preimages.values_mut() {
            sources.sort();
        }
        Self { preimages, images }
    }

    /// The image and the source's rank among the image's preimages. A
    /// one-to-one image has rank 0 and costs nothing beside the image.
    pub fn split(&self, source: S) -> Option<(I, usize)> {
        let image = *self.images.get(&source)?;
        let rank = self.preimages[&image].binary_search(&source).ok()?;
        Some((image, rank))
    }

    pub fn join(&self, image: I, rank: usize) -> Option<S> {
        self.preimages.get(&image)?.get(rank).copied()
    }

    /// How many ranks the decoder must distinguish at this image.
    pub fn residual_alphabet(&self, image: I) -> usize {
        self.preimages.get(&image).map_or(0, Vec::len)
    }

    pub fn sources(&self) -> impl Iterator<Item = S> + '_ {
        self.images.keys().copied()
    }
}

type QuatKey = (u32, u32, u32, u32);

fn quat_key(codon: Codon64) -> QuatKey {
    let q = quat_from_codon(codon);
    (q.w.to_bits(), q.x.to_bits(), q.y.to_bits(), q.z.to_bits())
}

fn codons() -> impl Iterator<Item = Codon64> {
    (0u8..64).map(Codon64::new)
}

/// Codon weights as the generation law yields them: each of the 192 cells
/// contributes its positive and its negative slot, 384 in all.
pub fn generation_yield_weights() -> BTreeMap<u8, u64> {
    let mut weights = BTreeMap::new();
    for cell in all_cells() {
        for codon in cell.yields() {
            *weights.entry(codon.address()).or_insert(0) += 1;
        }
    }
    weights
}

fn matrix_index(family: MatrixFamily) -> u8 {
    family.matrix_number()
}

/// The same-position conjugacy n ↔ n′ on the twelve coordinates.
pub fn d1_conjugacy_pairs() -> Vec<((u8, bool), (u8, bool))> {
    let mut pairs = Vec::new();
    for value in 0u8..6 {
        let position = QlPosition::new(value).expect("canonical position");
        for face in [QlFace::Direct, QlFace::Conjugate] {
            let source = QlCoordinate::new(position, face);
            let flipped = match face {
                QlFace::Direct => QlFace::Conjugate,
                QlFace::Conjugate => QlFace::Direct,
            };
            let image = QlCoordinate::new(position, flipped);
            pairs.push((
                (source.position.value(), source.face == QlFace::Conjugate),
                (image.position.value(), image.face == QlFace::Conjugate),
            ));
        }
    }
    pairs
}

/// The three recorded descriptions of the 72 → 64 epogdoon, each over its
/// own source keying: flat DET, fibrewise `I4 ⊗ T18→16`, arithmetic `i×8/9`.
pub fn epogdoon_det_pairs() -> Vec<(u8, u8)> {
    (0..M2_VIBRATION_CYCLE as u8)
        .map(|source| (source, det_shadow(source).expect("in-cycle source")))
        .collect()
}

pub fn epogdoon_fibre_pairs() -> Vec<((u8, u8), (u8, u8))> {
    let mut pairs = Vec::new();
    for element in 0u8..4 {
        for state in 0..carrier::FIBRE_STATES as u8 {
            let target = Transduction18to16::new(state)
                .expect("fibre state")
                .target();
            pairs.push(((element, state), (element, target)));
        }
    }
    pairs
}

pub fn epogdoon_arithmetic_pairs() -> Vec<(u8, u8)> {
    (0..M2_VIBRATION_CYCLE as u8)
        .map(|source| {
            (
                source,
                apply_epogdoon_compression(source).expect("in-cycle source"),
            )
        })
        .collect()
}

/// Every QLC-0 measurement, in receipt order.
pub fn fold_tax_table() -> Vec<FoldTax> {
    let yields = generation_yield_weights();
    let uniform = |_: u8| 1u64;
    let by_yield = |address: u8| yields.get(&address).copied().unwrap_or(0);
    let mut table = vec![
        FoldTax::measure(
            "d1-same-position-conjugacy",
            "uniform-12",
            d1_conjugacy_pairs().into_iter().map(|(s, i)| (s, i, 1)),
        ),
        FoldTax::measure(
            "epogdoon-det-flat-72-64",
            "uniform-72",
            epogdoon_det_pairs().into_iter().map(|(s, i)| (s, i, 1)),
        ),
        FoldTax::measure(
            "epogdoon-fibre-t18-16",
            "uniform-72",
            epogdoon_fibre_pairs().into_iter().map(|(s, i)| (s, i, 1)),
        ),
        FoldTax::measure(
            "epogdoon-arithmetic-8-9",
            "uniform-72",
            epogdoon_arithmetic_pairs()
                .into_iter()
                .map(|(s, i)| (s, i, 1)),
        ),
    ];
    for (distribution_ref, weight) in [
        ("uniform-64", &uniform as &dyn Fn(u8) -> u64),
        ("generation-yields-384", &by_yield),
    ] {
        table.push(FoldTax::measure(
            "quat-from-codon",
            distribution_ref,
            codons().map(|c| (c.address(), quat_key(c), weight(c.address()))),
        ));
        // The kernel's recorded, simplified amino table (`M3-C18`: mapping
        // rigor needs audit) — its tax is the record's, not the biology's.
        table.push(FoldTax::measure(
            "codon-to-amino-recorded",
            distribution_ref,
            codons().map(|c| (c.address(), m3_codon_amino_index(c), weight(c.address()))),
        ));
    }
    table.push(FoldTax::measure(
        "nuclear-hexagram",
        "uniform-64",
        (0u8..64).map(|hex| (hex, nuclear_hexagram(hex), 1)),
    ));
    // The conjugate codon of a generation cell, read against its positive
    // codon: with the matrix known, then unknown, then with nothing (the
    // negative folded to the 0/1 anchor, so the tax is its whole cost).
    // A positive codon never says whether its cell is non-dual: each
    // (matrix, positive) is shared by two cells with different negatives.
    table.push(FoldTax::measure(
        "generation-conjugate-given-matrix",
        "generation-cells-192",
        all_cells().map(|cell| {
            let m = matrix_index(cell.family);
            (
                (m, cell.positive.address(), cell.negative.address()),
                (m, cell.positive.address()),
                1,
            )
        }),
    ));
    table.push(FoldTax::measure(
        "generation-conjugate-matrix-unknown",
        "generation-cells-192",
        all_cells().map(|cell| {
            (
                (cell.positive.address(), cell.negative.address()),
                cell.positive.address(),
                1,
            )
        }),
    ));
    table.push(FoldTax::measure(
        "generation-negative-to-anchor",
        "generation-cells-192",
        all_cells().map(|cell| (cell.negative.address(), (), 1)),
    ));
    table
}

pub fn fold_tax_tsv() -> String {
    let mut out = format!(
        "# {FOLD_TAX_CONTRACT} — generated by ql_core::fold_tax_tsv; do not edit by hand\n\
         map_ref\tdistribution_ref\tsupport\timage\tmax_preimage\tpreimage_histogram\tsource_bits\timage_bits\ttax_bits\n"
    );
    for row in fold_tax_table() {
        out.push_str(&row.tsv_row());
        out.push('\n');
    }
    out
}
