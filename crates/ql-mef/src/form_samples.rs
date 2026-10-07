//! The PS-G retained sample body (QL-MEF #299 slice 2; P5 §4.1 material
//! preparation): the form-recipe body's correspondence, held natively.
//!
//! Deformation composes over correspondence, not over per-call fabrication.
//! For every sample this owner retains the stable per-point identity, the
//! source/layer identity, the square-domain rest coordinates `(u, v)` with
//! rest depth `w`, and the density/coverage — exactly the contract's
//! "for every point retain" law. Re-resolving the same preparation yields the
//! identical body: the same IDs, the same order, the same rest coordinates —
//! so a consumer that re-resolves never breaks a standing correspondence.
//!
//! What this owner deliberately does not do: rasterise a glyph. The profile's
//! glyph-mask occupancy is DECLARED here — the caller supplies the mask
//! determination (`mask_ref` + per-sample coverages, the application
//! rasteriser's own output, cited not performed) and the body retains it under
//! the same stable identities. The rasterisation itself stays the app's PS-G
//! half, as the form-recipe owner already rules.
//!
//! The units law is explicit: the normalised square domain `(u, v, w)` maps to
//! stage units by the declared extent/depth and to the physical metric scale
//! by the declared metres-per-unit — the same stage-unit convention the scene
//! geometry carries. The preparation's content hash is the dependency-correct
//! cache key: it covers exactly the declared preparation (treatment, layers,
//! resolution, units, mask determination) and nothing else — fold progress,
//! cursor, entity position and camera are not preparation inputs, so updating
//! them never invalidates or re-derives a body (P5 §4.1's dependency law).
//! When topology genuinely changes (resolution or layers), [`SampleBody::remap`]
//! returns the explicit old-to-new mapping instead of pretending identity.
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use sha2::{Digest, Sha256};

pub const FORM_SAMPLES_CONTRACT: &str = "ql.psg-form-samples/v1";

/// The square allocation grid's largest side; 64×64×16 layers is the bounded
/// preparation envelope (65,536 samples) this owner retains.
pub const MAX_RESOLUTION: u16 = 64;
pub const MAX_LAYERS: usize = 16;
/// The largest magnitude the declared units admit, matching the scene
/// geometry's declared-unit bound.
const MAX_UNITS: f64 = 1_000_000.0;

/// The material treatment the profile chooses (P5 §4.1): glyph-mask occupancy
/// over the square domain, or a full sheet carrier.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaterialTreatment {
    GlyphMask,
    SheetCarrier,
}

/// The declared unit conversions between the normalised material coordinates,
/// the existing stage units and the physical body's metric scale.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleUnits {
    /// The square domain's side, in stage units.
    pub extent_units: f64,
    /// The body's full depth (the `w` span 0..1), in stage units.
    pub depth_units: f64,
    /// Declared metres per stage unit — the scene geometry's own convention.
    pub metres_per_unit: f64,
}

/// One material layer: its source/layer identity and its rest depth `w` in the
/// normalised domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleLayer {
    pub layer_ref: String,
    pub rest_depth_w: f64,
}

/// The declared glyph-mask determination: where the coverages came from (the
/// application rasteriser's own identity) and one coverage per retained
/// sample, in the body's allocation order. Carried, never performed, here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaskDeclaration {
    pub mask_ref: String,
    pub coverages: Vec<f64>,
}

/// A declared sample preparation: the retained body's complete dependency set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SamplePreparation {
    pub schema: String,
    /// The caller's stable preparation identity; sample IDs derive from it, so
    /// a body's IDs survive re-resolution under the same reference.
    pub prep_ref: String,
    pub treatment: MaterialTreatment,
    pub layers: Vec<SampleLayer>,
    /// The square allocation grid's side: `resolution × resolution` points
    /// per layer, at the pixel centres `(i + 1/2) / resolution`.
    pub resolution: u16,
    pub units: SampleUnits,
    /// Required exactly when the treatment is glyph-mask occupancy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<MaskDeclaration>,
}

/// One retained sample: stable identity, layer, allocation indices and
/// coverage. The rest coordinates derive exactly from the indices and the
/// retained preparation — never stored per call, never re-derived differently.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestSample {
    pub sample_id: u64,
    pub layer_index: u16,
    pub u_index: u16,
    pub v_index: u16,
    /// Density/coverage 0..1: 1.0 across a sheet carrier, the declared
    /// glyph-mask value where the profile chose occupancy.
    pub coverage: f64,
}

/// The retained sample body: the preparation it stands on, its content-derived
/// cache key, and the samples in the deterministic allocation order
/// (layer-major, then row, then column).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleBody {
    pub preparation: SamplePreparation,
    /// The dependency-correct cache key: the SHA-256 over exactly the declared
    /// preparation. Preparation changes move it; progress and camera never do.
    pub preparation_sha256: String,
    pub samples: Vec<RestSample>,
}

/// The explicit old-to-new identity mapping a genuine topology change
/// produces: one entry per successor sample, carrying the predecessor's sample
/// ID where correspondence continues and `None` where the point is new.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleMapping {
    pub from_prep_ref: String,
    pub to_prep_ref: String,
    pub pairs: Vec<Option<u64>>,
}

fn bounded_text(name: &str, value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 2048 || value.chars().any(|c| c.is_control()) {
        return Err(format!("invalid {name} reference"));
    }
    Ok(())
}

/// FNV-1a 64: the stable per-point identity hash. Sample IDs derive from the
/// preparation reference and the allocation indices alone — never from call
/// order, coverage values or preparation content that can change compatibly.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn sample_id(prep_ref: &str, layer_index: u16, u_index: u16, v_index: u16) -> u64 {
    let mut bytes = Vec::with_capacity(prep_ref.len() + 8);
    bytes.extend_from_slice(FORM_SAMPLES_CONTRACT.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(prep_ref.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(&layer_index.to_be_bytes());
    bytes.extend_from_slice(&u_index.to_be_bytes());
    bytes.extend_from_slice(&v_index.to_be_bytes());
    fnv1a64(&bytes)
}

impl SamplePreparation {
    /// The allocation order's length: layer-major, then row, then column.
    pub fn allocation_count(&self) -> usize {
        self.layers.len() * usize::from(self.resolution) * usize::from(self.resolution)
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema != FORM_SAMPLES_CONTRACT {
            return Err(format!(
                "unsupported sample preparation contract {}; expected {FORM_SAMPLES_CONTRACT}",
                self.schema
            ));
        }
        bounded_text("preparation", &self.prep_ref)?;
        if self.layers.is_empty() || self.layers.len() > MAX_LAYERS {
            return Err(format!(
                "a sample preparation carries 1..={MAX_LAYERS} layers"
            ));
        }
        for layer in &self.layers {
            bounded_text("layer", &layer.layer_ref)?;
            if !layer.rest_depth_w.is_finite() || !(0.0..=1.0).contains(&layer.rest_depth_w) {
                return Err(format!(
                    "layer {} rest depth must be finite within the normalised domain 0..1",
                    layer.layer_ref
                ));
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        for layer in &self.layers {
            if !seen.insert(&layer.layer_ref) {
                return Err(format!(
                    "two layers carry the layer_ref {:?}; layer identity must be unique",
                    layer.layer_ref
                ));
            }
        }
        if self.resolution == 0 || self.resolution > MAX_RESOLUTION {
            return Err(format!(
                "sample resolution must be within 1..={MAX_RESOLUTION} per side"
            ));
        }
        let units = &self.units;
        for (name, value) in [
            ("extent_units", units.extent_units),
            ("depth_units", units.depth_units),
            ("metres_per_unit", units.metres_per_unit),
        ] {
            if !value.is_finite() || value <= 0.0 || value > MAX_UNITS {
                return Err(format!(
                    "declared unit {name} must be finite, positive and within {MAX_UNITS}"
                ));
            }
        }
        match (&self.treatment, &self.mask) {
            (MaterialTreatment::GlyphMask, Some(mask)) => {
                bounded_text("mask", &mask.mask_ref)?;
                let count = self.allocation_count();
                if mask.coverages.len() != count {
                    return Err(format!(
                        "the declared glyph mask carries {} coverages for {count} allocation points; one coverage per retained sample, in allocation order",
                        mask.coverages.len()
                    ));
                }
                if mask.coverages.iter().any(|c| !c.is_finite() || !(0.0..=1.0).contains(c)) {
                    return Err("glyph-mask coverage must be finite within 0..1".into());
                }
            }
            (MaterialTreatment::GlyphMask, None) => {
                return Err(
                    "glyph-mask occupancy is a declared determination: the preparation must carry the mask (mask_ref and one coverage per sample) the application rasteriser produced — the mask is cited here, never invented".into(),
                )
            }
            (MaterialTreatment::SheetCarrier, Some(_)) => {
                return Err(
                    "a sheet carrier covers every allocation point by law; a declared mask belongs to glyph-mask occupancy"
                        .into(),
                )
            }
            (MaterialTreatment::SheetCarrier, None) => {}
        }
        Ok(())
    }
}

/// Prepares the retained body over the declared preparation: validation
/// refusals are by name; the allocation, its identities and its order are
/// deterministic in the preparation alone.
pub fn prepare_samples(preparation: SamplePreparation) -> Result<SampleBody, String> {
    preparation.validate()?;
    let resolution = preparation.resolution;
    let mut samples = Vec::with_capacity(preparation.allocation_count());
    let mut coverages = preparation.mask.as_ref().map(|m| m.coverages.iter());
    for layer_index in 0..preparation.layers.len() as u16 {
        for v_index in 0..resolution {
            for u_index in 0..resolution {
                let coverage = match (&preparation.treatment, &mut coverages) {
                    (MaterialTreatment::SheetCarrier, _) => 1.0,
                    (MaterialTreatment::GlyphMask, Some(iter)) => {
                        *iter.next().expect("validated allocation length")
                    }
                    (MaterialTreatment::GlyphMask, None) => {
                        unreachable!("glyph-mask validates its mask")
                    }
                };
                samples.push(RestSample {
                    sample_id: sample_id(&preparation.prep_ref, layer_index, u_index, v_index),
                    layer_index,
                    u_index,
                    v_index,
                    coverage,
                });
            }
        }
    }
    // The cache key covers exactly the declared preparation — the complete
    // dependency set, canonically serialised. Nothing else may enter it.
    let preparation_sha256 = format!(
        "{:x}",
        Sha256::digest(to_vec(&preparation).expect("serialisable preparation"))
    );
    Ok(SampleBody {
        preparation,
        preparation_sha256,
        samples,
    })
}

impl SampleBody {
    /// The sample's normalised square-domain rest coordinate `(u, v)`, at the
    /// allocation point's centre.
    pub fn rest_uv(&self, sample: &RestSample) -> (f64, f64) {
        let n = f64::from(self.preparation.resolution);
        (
            (f64::from(sample.u_index) + 0.5) / n,
            (f64::from(sample.v_index) + 0.5) / n,
        )
    }

    /// The sample's normalised rest depth `w` — its layer's declared depth.
    pub fn rest_w(&self, sample: &RestSample) -> f64 {
        self.preparation.layers[sample.layer_index as usize].rest_depth_w
    }

    /// The sample's rest position in stage units: the declared conversion,
    /// centred on the square domain.
    pub fn rest_units(&self, sample: &RestSample) -> [f64; 3] {
        let (u, v) = self.rest_uv(sample);
        let w = self.rest_w(sample);
        let units = &self.preparation.units;
        [
            (u - 0.5) * units.extent_units,
            (v - 0.5) * units.extent_units,
            (w - 0.5) * units.depth_units,
        ]
    }

    /// The sample's rest position in metres: the physical metric scale the
    /// declared metres-per-unit names — the shape the existing field samples
    /// already carry.
    pub fn rest_metres(&self, sample: &RestSample) -> [f64; 3] {
        let scale = self.preparation.units.metres_per_unit;
        self.rest_units(sample).map(|axis| axis * scale)
    }

    /// The explicit correspondence mapping into a successor body. A genuine
    /// topology change (resolution or layers) is mapped, not papered over:
    /// each successor sample carries its nearest predecessor's ID within the
    /// same layer, ties resolved by the predecessor's allocation order —
    /// deterministic, inspectable, and `None` where the point is new.
    pub fn remap(&self, successor: &SampleBody) -> Result<SampleMapping, String> {
        if self.preparation.prep_ref != successor.preparation.prep_ref {
            return Err(format!(
                "the explicit remap is a body's own continuation ({}); a different preparation reference ({}) is a new correspondence, not a remap",
                self.preparation.prep_ref, successor.preparation.prep_ref
            ));
        }
        if successor.preparation_sha256 == self.preparation_sha256 {
            return Ok(SampleMapping {
                from_prep_ref: self.preparation.prep_ref.clone(),
                to_prep_ref: successor.preparation.prep_ref.clone(),
                pairs: successor
                    .samples
                    .iter()
                    .map(|s| Some(s.sample_id))
                    .collect(),
            });
        }
        let mut pairs = Vec::with_capacity(successor.samples.len());
        for sample in &successor.samples {
            let (u, v) = successor.rest_uv(sample);
            let mut best: Option<(f64, u64)> = None;
            for candidate in &self.samples {
                if candidate.layer_index != sample.layer_index {
                    continue;
                }
                let (cu, cv) = self.rest_uv(candidate);
                let distance = (cu - u) * (cu - u) + (cv - v) * (cv - v);
                // Strictly-better keeps the earliest predecessor in allocation
                // order on ties: the mapping is deterministic.
                if best.is_none_or(|(best_distance, _)| distance < best_distance) {
                    best = Some((distance, candidate.sample_id));
                }
            }
            pairs.push(best.map(|(_, sample_id)| sample_id));
        }
        Ok(SampleMapping {
            from_prep_ref: self.preparation.prep_ref.clone(),
            to_prep_ref: successor.preparation.prep_ref.clone(),
            pairs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn units() -> SampleUnits {
        SampleUnits {
            extent_units: 2.0,
            depth_units: 0.5,
            metres_per_unit: 0.5,
        }
    }

    fn preparation() -> SamplePreparation {
        SamplePreparation {
            schema: FORM_SAMPLES_CONTRACT.into(),
            prep_ref: "ta-onta:psg:moon-square".into(),
            treatment: MaterialTreatment::SheetCarrier,
            layers: vec![
                SampleLayer {
                    layer_ref: "glyph:front".into(),
                    rest_depth_w: 0.25,
                },
                SampleLayer {
                    layer_ref: "glyph:back".into(),
                    rest_depth_w: 0.75,
                },
            ],
            resolution: 4,
            units: units(),
            mask: None,
        }
    }

    fn mask(count: usize) -> MaskDeclaration {
        MaskDeclaration {
            mask_ref: "app:rasteriser:moon-glyph@1".into(),
            coverages: vec![0.5; count],
        }
    }

    #[test]
    fn the_body_is_deterministic_and_ids_survive_re_resolution() {
        let first = prepare_samples(preparation()).unwrap();
        let second = prepare_samples(preparation()).unwrap();
        assert_eq!(first, second, "re-resolution fabricates nothing new");
        assert_eq!(first.samples.len(), 2 * 4 * 4);
        // Deterministic allocation order: layer-major, then row, then column.
        assert_eq!(first.samples[0].layer_index, 0);
        assert_eq!(first.samples[0].u_index, 0);
        assert_eq!(first.samples[0].v_index, 0);
        assert_eq!(first.samples[1].u_index, 1);
        assert_eq!(first.samples[16].layer_index, 1);
        // The stable identity is a per-point law, not an index into this call.
        assert_ne!(first.samples[0].sample_id, first.samples[1].sample_id);
        assert_ne!(first.samples[0].sample_id, first.samples[16].sample_id);
        // The cache key covers exactly the declared preparation.
        assert_eq!(first.preparation_sha256.len(), 64);
    }

    #[test]
    fn the_units_conversion_is_explicit_and_exact() {
        let body = prepare_samples(preparation()).unwrap();
        // Resolution 4: first point's centre sits at u = v = 0.125; the front
        // layer at w = 0.25.
        let sample = &body.samples[0];
        assert_eq!(body.rest_uv(sample), (0.125, 0.125));
        assert_eq!(body.rest_w(sample), 0.25);
        let units = body.rest_units(sample);
        assert_eq!(units[0], (0.125 - 0.5) * 2.0);
        assert_eq!(units[2], (0.25 - 0.5) * 0.5);
        let metres = body.rest_metres(sample);
        assert_eq!(metres[0], units[0] * 0.5);
        assert_eq!(metres[1], units[1] * 0.5);
        assert_eq!(metres[2], units[2] * 0.5);
    }

    #[test]
    fn the_treatments_carry_their_own_coverage_law() {
        let sheet = prepare_samples(preparation()).unwrap();
        assert!(sheet.samples.iter().all(|s| s.coverage == 1.0));
        let mut glyph = preparation();
        glyph.treatment = MaterialTreatment::GlyphMask;
        glyph.mask = Some(mask(glyph.allocation_count()));
        let body = prepare_samples(glyph).unwrap();
        assert!(body.samples.iter().all(|s| s.coverage == 0.5));
    }

    #[test]
    fn refusals_name_the_preparation_law() {
        let mut wrong_schema = preparation();
        wrong_schema.schema = "ql.psg-form-samples/v0".into();
        assert!(
            prepare_samples(wrong_schema)
                .unwrap_err()
                .contains("unsupported sample preparation contract")
        );

        let mut bare_mask = preparation();
        bare_mask.treatment = MaterialTreatment::GlyphMask;
        let error = prepare_samples(bare_mask).unwrap_err();
        assert!(error.contains("declared determination"), "{error}");
        assert!(error.contains("cited"), "{error}");

        let mut short_mask = preparation();
        short_mask.treatment = MaterialTreatment::GlyphMask;
        short_mask.mask = Some(mask(short_mask.allocation_count() - 1));
        assert!(
            prepare_samples(short_mask)
                .unwrap_err()
                .contains("one coverage per retained sample")
        );

        let mut masked_sheet = preparation();
        masked_sheet.mask = Some(mask(0));
        assert!(
            prepare_samples(masked_sheet)
                .unwrap_err()
                .contains("sheet carrier covers every allocation point")
        );

        let mut wide = preparation();
        wide.resolution = MAX_RESOLUTION + 1;
        assert!(prepare_samples(wide).unwrap_err().contains("per side"));

        let mut deep = preparation();
        deep.layers = vec![
            SampleLayer {
                layer_ref: "glyph".into(),
                rest_depth_w: 0.5,
            };
            MAX_LAYERS + 1
        ];
        assert!(prepare_samples(deep).unwrap_err().contains("1..=16 layers"));

        let mut duplicate = preparation();
        duplicate.layers[1].layer_ref = duplicate.layers[0].layer_ref.clone();
        assert!(
            prepare_samples(duplicate)
                .unwrap_err()
                .contains("layer identity must be unique")
        );

        let mut drowned = preparation();
        drowned.layers[0].rest_depth_w = 1.5;
        assert!(
            prepare_samples(drowned)
                .unwrap_err()
                .contains("normalised domain 0..1")
        );

        let mut hot = preparation();
        hot.units.metres_per_unit = MAX_UNITS * 2.0;
        assert!(
            prepare_samples(hot)
                .unwrap_err()
                .contains("metres_per_unit")
        );

        let mut anonymous = preparation();
        anonymous.prep_ref = String::new();
        assert!(
            prepare_samples(anonymous)
                .unwrap_err()
                .contains("preparation")
        );
    }

    #[test]
    fn compatible_updates_map_by_identity_and_topology_changes_map_explicitly() {
        let body = prepare_samples(preparation()).unwrap();
        // A compatible update — the mask determination moves, the topology
        // does not: identity mapping, the IDs standing unchanged.
        let mut recoated = preparation();
        recoated.treatment = MaterialTreatment::GlyphMask;
        recoated.mask = Some(MaskDeclaration {
            mask_ref: "app:rasteriser:moon-glyph@2".into(),
            coverages: vec![0.25; recoated.allocation_count()],
        });
        let recoated = prepare_samples(recoated).unwrap();
        assert_ne!(recoated.preparation_sha256, body.preparation_sha256);
        let mapping = body.remap(&recoated).unwrap();
        assert!(
            mapping
                .pairs
                .iter()
                .copied()
                .eq(recoated.samples.iter().map(|s| Some(s.sample_id))),
            "same topology maps every point to its own standing identity"
        );

        // A genuine topology change — resolution halves: every successor point
        // maps to its nearest predecessor deterministically.
        let mut coarser = preparation();
        coarser.resolution = 2;
        let coarser = prepare_samples(coarser).unwrap();
        let mapping = body.remap(&coarser).unwrap();
        assert_eq!(mapping.pairs.len(), coarser.samples.len());
        // The coarse (0,0) centre (0.25, 0.25) is nearest to the fine point at
        // indices (0,0) — centre (0.125, 0.125) — in the same layer.
        let expected = body.samples[0].sample_id;
        assert_eq!(mapping.pairs[0], Some(expected));
        // And a different preparation reference is refused as a new body.
        let mut elsewhere = preparation();
        elsewhere.prep_ref = "ta-onta:psg:elsewhere".into();
        let elsewhere = prepare_samples(elsewhere).unwrap();
        let error = body.remap(&elsewhere).unwrap_err();
        assert!(error.contains("own continuation"), "{error}");
        assert!(error.contains("new correspondence"), "{error}");
    }
}
