//! The PS-G form instrument (QL-MEF #299 slice 3; P5 §4.1 dependency law, §4.2
//! commanded-vs-observed): the rasterisation/reseed instrumentation the
//! application's sampler half consumes, and the observed resident geometry
//! readback, natively.
//!
//! Slice 2 proved the dependency law at the cache key: the preparation's
//! content hash covers exactly the declared preparation, so fold progress,
//! cursor and camera are not preparation inputs. This owner makes that law
//! OBSERVABLE: counters and receipts for rasterisation, cache-hit, reseed and
//! progress events, each wired to the dependency-correct cache key it happened
//! under — so a consumer can prove that repeated angle/progress updates
//! re-sample nothing and reset no resident particle, by reading counters, not
//! by trusting an assertion.
//!
//! The instrument's teeth are the law it enforces, not only the numbers it
//! keeps. A reseed is admitted only through a genuine topology change, mapped
//! by the samples owner's own explicit correspondence; a preparation whose
//! coating changed but whose residents continue by identity is refused a
//! reseed by name — the cache key moved, the body's identity stood. The
//! rasterisation itself stays the application's PS-G half: the instrument
//! counts and cites it under the key it was performed under, never performs
//! it.
//!
//! Observed is not commanded. A reading returns the geometry read FROM the
//! retained body — its identity digest, occupancy and rest extent — beside
//! the commanded crease state, separately disclosed (P5 §4.2), so a lagging
//! consumer cannot be disguised as an applied pose. The reading is pure:
//! observing the resident body changes nothing about it.
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

use crate::form_samples::SampleBody;
use crate::form_samples::{SamplePreparation, prepare_samples};
use crate::form_sequence::{FORM_PROGRESS_CONTRACT, FoldProgress};

pub const FORM_INSTRUMENT_CONTRACT: &str = "ql.psg-form-instrument/v1";
pub const RESIDENT_GEOMETRY_CONTRACT: &str = "ql.psg-resident-geometry/v1";
pub const RESIDENT_OBSERVATION_CONTRACT: &str = "ql.psg-resident-observation/v1";

/// The takeover record's retained history: the last events, oldest first.
pub const RETAINED_EVENTS: usize = 64;

/// One instrumented event, wired to the dependency-correct cache key it
/// happened under. A native receipt: produced here, read by consumers, never
/// fabricated from JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "kebab-case", deny_unknown_fields)]
pub enum InstrumentEvent {
    /// The first derivation of a cache key through this instrument — the
    /// receipt of the rasterisation performed under it, with the mask
    /// determination cited when the occupancy is glyph-mask.
    Rasterised {
        preparation_sha256: String,
        mask_ref: Option<String>,
        samples: u32,
    },
    /// A preparation request for a cache key that already stood: the cache
    /// answered, the rasteriser did not run.
    CacheHit { preparation_sha256: String },
    /// An admitted reseed: the residents re-seated under a successor key
    /// through the samples owner's explicit old-to-new mapping — genuine
    /// topology change only.
    Reseeded {
        preparation_sha256: String,
        predecessor_sha256: String,
        continued_samples: u32,
        fresh_samples: u32,
    },
    /// A fold-progress/angle/cursor update read against the resident body.
    /// The counters around it prove it re-sampled nothing and reset nothing.
    ProgressRead {
        preparation_sha256: String,
        cursor_steps: u64,
    },
}

/// The per-cache-key counters: the observable proof surface. Everything here
/// moves only through named instrument events — a progress read cannot move
/// `rasterisations` or `reseeds` because no path exists that would.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreparationCounters {
    pub rasterisations: u32,
    pub cache_hits: u32,
    pub reseeds: u32,
    pub progress_reads: u32,
}

/// What a preparation request through the instrument did: the cache key's
/// first derivation (the rasterisation receipt) or a cache hit (the
/// rasteriser did not run).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PreparationReceipt {
    Rasterised,
    CacheHit,
}

/// The observed resident geometry: read FROM the retained body — identity,
/// occupancy and rest extent — never from the commanded state. The identity
/// digest covers every resident row (sample ID, layer, allocation indices,
/// coverage, rest position in stage units) in allocation order: a reseed or
/// re-raster that moved a resident point moves this digest, and nothing else
/// does.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentGeometry {
    pub schema: &'static str,
    pub preparation_sha256: String,
    pub sample_count: u32,
    pub resident_sha256: String,
    pub occupied_samples: u32,
    pub coverage_sum: f64,
    /// The resident rest extent in stage units: the min corner, then the max.
    pub rest_bounds_units: [[f64; 3]; 2],
    pub standing: String,
}

/// One observed reading: the commanded fold state and the observed resident
/// geometry, disclosed separately (P5 §4.2). The commanded angles move with
/// the cursor; the observed body stands unless the preparation law moved it —
/// and the two disclosures never stand in for each other.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentObservation {
    pub schema: &'static str,
    pub preparation_sha256: String,
    pub cursor_steps: u64,
    /// The commanded crease state at this cursor.
    pub commanded_site_angles_deg10: [i32; 3],
    /// The commanded form, when the codon resolved at a quanta.
    pub commanded_form_address: Option<u8>,
    /// The geometry read from the resident body.
    pub observed: ResidentGeometry,
}

/// The receipt of an admitted reseed: the explicit correspondence the samples
/// owner derived, with the continued and fresh resident counts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReseedReceipt {
    pub schema: &'static str,
    pub predecessor_sha256: String,
    pub preparation_sha256: String,
    pub continued_samples: u32,
    pub fresh_samples: u32,
    pub standing: String,
}

/// Reads the resident geometry from a retained body: identity, occupancy and
/// rest extent over the allocation order. Pure — observing changes nothing.
pub fn observe(body: &SampleBody) -> ResidentGeometry {
    let mut digest = Sha256::new();
    let mut occupied = 0u32;
    let mut coverage_sum = 0.0f64;
    let mut min: Option<[f64; 3]> = None;
    let mut max: Option<[f64; 3]> = None;
    for sample in &body.samples {
        let rest = body.rest_units(sample);
        digest.update(sample.sample_id.to_be_bytes());
        digest.update(sample.layer_index.to_be_bytes());
        digest.update(sample.u_index.to_be_bytes());
        digest.update(sample.v_index.to_be_bytes());
        digest.update(sample.coverage.to_bits().to_be_bytes());
        for axis in rest {
            digest.update(axis.to_bits().to_be_bytes());
        }
        if sample.coverage > 0.0 {
            occupied += 1;
        }
        coverage_sum += sample.coverage;
        min = Some(match min {
            None => rest,
            Some(m) => std::array::from_fn(|i| m[i].min(rest[i])),
        });
        max = Some(match max {
            None => rest,
            Some(m) => std::array::from_fn(|i| m[i].max(rest[i])),
        });
    }
    ResidentGeometry {
        schema: RESIDENT_GEOMETRY_CONTRACT,
        preparation_sha256: body.preparation_sha256.clone(),
        sample_count: body.samples.len() as u32,
        resident_sha256: format!("{:x}", digest.finalize()),
        occupied_samples: occupied,
        coverage_sum,
        rest_bounds_units: [min.unwrap_or([0.0; 3]), max.unwrap_or([0.0; 3])],
        standing: "observed resident geometry: read from the retained body, not from the \
             commanded state; the identity digest moves only when a resident point moves"
            .into(),
    }
}

/// The instrument over the retained-body lifecycle: counters and receipts per
/// dependency-correct cache key, and the observed resident readings. The
/// application's sampler half wires its own rasterisation/reseed
/// instrumentation into this contract under the same keys.
#[derive(Debug, Clone, Default)]
pub struct FormInstrument {
    counters: BTreeMap<String, PreparationCounters>,
    /// The cache keys whose preparation this instrument issued — the
    /// cache-hit decision's own basis, kept apart from counters other events
    /// created.
    issued: BTreeSet<String>,
    events: Vec<InstrumentEvent>,
}

impl FormInstrument {
    /// Prepares the body through the instrument. The first request under a
    /// cache key records the rasterisation receipt (the mask determination
    /// cited when the occupancy is glyph-mask); a repeat request under the
    /// same key records a cache hit — the rasteriser did not run. The body
    /// itself derives through the samples owner, one law: the derivation is
    /// deterministic, so the cache-hit receipt is honest — the same
    /// preparation re-derives the identical body.
    pub fn prepare(
        &mut self,
        preparation: SamplePreparation,
    ) -> Result<(SampleBody, PreparationReceipt), String> {
        let body = prepare_samples(preparation)?;
        let key = body.preparation_sha256.clone();
        if self.issued.contains(&key) {
            let entry = self.counters.entry(key.clone()).or_default();
            entry.cache_hits += 1;
            self.retain(InstrumentEvent::CacheHit {
                preparation_sha256: key,
            });
            return Ok((body, PreparationReceipt::CacheHit));
        }
        self.issued.insert(key.clone());
        let entry = self.counters.entry(key.clone()).or_default();
        entry.rasterisations += 1;
        self.retain(InstrumentEvent::Rasterised {
            mask_ref: body.preparation.mask.as_ref().map(|m| m.mask_ref.clone()),
            samples: body.samples.len() as u32,
            preparation_sha256: key,
        });
        Ok((body, PreparationReceipt::Rasterised))
    }

    /// Records one fold-progress/angle/cursor update against the resident
    /// body and returns the observed reading. The update is not a preparation
    /// dependency: no path from here touches `rasterisations` or `reseeds`,
    /// so a consumer reading the counters after any number of updates holds
    /// the proof that nothing re-sampled and nothing reset. Refuses by name
    /// when the progress is not the fold-progress law's own receipt.
    pub fn progress_update(
        &mut self,
        body: &SampleBody,
        progress: &FoldProgress,
    ) -> Result<ResidentObservation, String> {
        if progress.schema != FORM_PROGRESS_CONTRACT {
            return Err(format!(
                "unsupported fold progress contract {}; expected {FORM_PROGRESS_CONTRACT}",
                progress.schema
            ));
        }
        let observed = observe(body);
        let entry = self
            .counters
            .entry(body.preparation_sha256.clone())
            .or_default();
        entry.progress_reads += 1;
        self.retain(InstrumentEvent::ProgressRead {
            preparation_sha256: body.preparation_sha256.clone(),
            cursor_steps: progress.cursor_steps,
        });
        Ok(ResidentObservation {
            schema: RESIDENT_OBSERVATION_CONTRACT,
            preparation_sha256: body.preparation_sha256.clone(),
            cursor_steps: progress.cursor_steps,
            commanded_site_angles_deg10: progress.site_angles_deg10,
            commanded_form_address: progress.resolved_form.as_ref().map(|f| f.address),
            observed,
        })
    }

    /// Admits a reseed through a genuine topology change: the residents are
    /// re-seated under the successor key through the samples owner's own
    /// explicit old-to-new mapping, derived here — never taken on faith. A
    /// successor whose residents continue by identity (the coating moved, the
    /// topology did not) is refused by name: nothing resets. A different
    /// preparation reference is the samples owner's own refusal — a new
    /// correspondence, not a remap.
    pub fn reseed(
        &mut self,
        predecessor: &SampleBody,
        successor: &SampleBody,
    ) -> Result<ReseedReceipt, String> {
        let mapping = predecessor.remap(successor)?;
        if successor.preparation_sha256 == predecessor.preparation_sha256 {
            return Err(
                "the successor body is the same cache key; a reseed without a preparation change is a law violation"
                    .into(),
            );
        }
        let identity = successor
            .samples
            .iter()
            .zip(&mapping.pairs)
            .all(|(sample, pair)| *pair == Some(sample.sample_id));
        if identity {
            return Err(format!(
                "the residents continue by identity across cache key {} (topology unchanged; the explicit mapping is the identity); a reseed without a genuine topology change is refused — the coating moved, the body did not",
                successor.preparation_sha256
            ));
        }
        let continued = mapping.pairs.iter().filter(|p| p.is_some()).count() as u32;
        let fresh = mapping.pairs.len() as u32 - continued;
        let entry = self
            .counters
            .entry(successor.preparation_sha256.clone())
            .or_default();
        entry.reseeds += 1;
        self.retain(InstrumentEvent::Reseeded {
            fresh_samples: fresh,
            continued_samples: continued,
            preparation_sha256: successor.preparation_sha256.clone(),
            predecessor_sha256: predecessor.preparation_sha256.clone(),
        });
        Ok(ReseedReceipt {
            schema: FORM_INSTRUMENT_CONTRACT,
            predecessor_sha256: predecessor.preparation_sha256.clone(),
            preparation_sha256: successor.preparation_sha256.clone(),
            continued_samples: continued,
            fresh_samples: fresh,
            standing: "admitted reseed: the residents re-seated through the samples owner's \
                 explicit old-to-new mapping; continued identities carried, fresh points named"
                .into(),
        })
    }

    fn retain(&mut self, event: InstrumentEvent) {
        self.events.push(event);
        let excess = self.events.len().saturating_sub(RETAINED_EVENTS);
        self.events.drain(..excess);
    }

    /// The counters of one cache key, when any instrument event stood under it.
    pub fn counter_of(&self, preparation_sha256: &str) -> Option<PreparationCounters> {
        self.counters.get(preparation_sha256).copied()
    }

    /// The retained event receipts, oldest first.
    pub fn events(&self) -> &[InstrumentEvent] {
        &self.events
    }

    /// The totals across every cache key: the whole instrument's proof line —
    /// rasterisations and reseeds over all bodies, however many keys stand.
    pub fn totals(&self) -> PreparationCounters {
        let mut totals = PreparationCounters::default();
        for counters in self.counters.values() {
            totals.rasterisations += counters.rasterisations;
            totals.cache_hits += counters.cache_hits;
            totals.reseeds += counters.reseeds;
            totals.progress_reads += counters.progress_reads;
        }
        totals
    }

    /// The instrument's full disclosure: counters per cache key and the
    /// retained event receipts, oldest first — the contract the application's
    /// sampler half consumes. Deterministic in the events alone: no clock, no
    /// ordering by arrival beyond the event sequence.
    pub fn state(&self) -> Value {
        json!({
            "schema": FORM_INSTRUMENT_CONTRACT,
            "counters": self.counters.iter().map(|(key, c)| json!({
                "preparation_sha256": key,
                "rasterisations": c.rasterisations,
                "cache_hits": c.cache_hits,
                "reseeds": c.reseeds,
                "progress_reads": c.progress_reads,
            })).collect::<Vec<_>>(),
            "events": serde_json::to_value(&self.events).expect("instrument events serialise"),
            "standing": "rasterisation/reseed instrumentation over dependency-correct cache keys; \
                the counters are the proof surface, the events the receipts",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::continuous::LiftInput;
    use crate::form_recipe::FormDetermination;
    use crate::form_samples::{MaskDeclaration, MaterialTreatment, SampleLayer, SampleUnits};
    use crate::form_sequence::{
        Easing, FORM_SEQUENCE_CONTRACT, FoldSequence, SequenceAxis, SequencePhase,
        evaluate_fold_sequence,
    };

    fn preparation() -> SamplePreparation {
        SamplePreparation {
            schema: crate::form_samples::FORM_SAMPLES_CONTRACT.into(),
            prep_ref: "ta-onta:psg:moon-square".into(),
            treatment: MaterialTreatment::GlyphMask,
            layers: vec![SampleLayer {
                layer_ref: "glyph:front".into(),
                rest_depth_w: 0.25,
            }],
            resolution: 4,
            units: SampleUnits {
                extent_units: 2.0,
                depth_units: 0.5,
                metres_per_unit: 0.5,
            },
            mask: Some(MaskDeclaration {
                mask_ref: "app:rasteriser:moon-glyph@1".into(),
                coverages: {
                    let count = 4 * 4;
                    (0..count).map(|i| (i % 3) as f64 / 2.0).collect()
                },
            }),
        }
    }

    /// The specimen fold sequence: hold form 7 one turn, transition to ATC
    /// over one turn, hold ATC one turn — the same walk the sequence owner's
    /// tests prove, here driving the instrument's commanded readings.
    fn sequence() -> FoldSequence {
        FoldSequence {
            schema: FORM_SEQUENCE_CONTRACT.into(),
            sequence_ref: "ta-onta:psg:instrument-walk".into(),
            revision: 1,
            subject_ref: "ql:k2/default-subject".into(),
            axis: SequenceAxis::Inscription,
            origin: LiftInput {
                turns: "0".into(),
                half_degrees: 0,
            },
            phases: vec![
                SequencePhase::Hold {
                    determination: FormDetermination::Address { address: 7 },
                    half_degrees: 720,
                },
                SequencePhase::Transition {
                    to: FormDetermination::Address { address: 6 },
                    half_degrees: 720,
                    easing: Easing::Smoothstep,
                },
                SequencePhase::Hold {
                    determination: FormDetermination::Address { address: 6 },
                    half_degrees: 720,
                },
            ],
        }
    }

    #[test]
    fn repeated_progress_reads_never_rasterise_or_reseed() {
        let mut instrument = FormInstrument::default();
        let (body, receipt) = instrument.prepare(preparation()).unwrap();
        assert_eq!(receipt, PreparationReceipt::Rasterised);
        assert_eq!(body.samples.len(), 16);

        // Angle/progress updates across the whole walk: holds, transitions,
        // quanta, seeks back. Every one of them a read.
        let mut observations = Vec::new();
        for cursor in [0u64, 719, 720, 1000, 1080, 1439, 1440, 2160, 1080] {
            let progress = evaluate_fold_sequence(&sequence(), cursor).unwrap();
            observations.push(instrument.progress_update(&body, &progress).unwrap());
        }

        // The proof surface: one rasterisation, no reseeds, nine reads.
        let counters = instrument.counter_of(&body.preparation_sha256).unwrap();
        assert_eq!(
            counters,
            PreparationCounters {
                rasterisations: 1,
                cache_hits: 0,
                reseeds: 0,
                progress_reads: 9,
            }
        );
        assert_eq!(
            instrument.totals(),
            PreparationCounters {
                rasterisations: 1,
                cache_hits: 0,
                reseeds: 0,
                progress_reads: 9,
            }
        );

        // Commanded moved; observed stood. The digests are byte-identical
        // across every reading; the commanded angles and forms differ.
        let digests: Vec<&str> = observations
            .iter()
            .map(|o| o.observed.resident_sha256.as_str())
            .collect();
        assert!(digests.windows(2).all(|w| w[0] == w[1]));
        assert!(
            observations
                .windows(2)
                .any(|w| w[0].commanded_site_angles_deg10 != w[1].commanded_site_angles_deg10)
        );
        assert_eq!(observations[0].commanded_form_address, Some(7));
        assert_eq!(observations[4].commanded_form_address, None);
        assert_eq!(observations[6].commanded_form_address, Some(6));
        assert_eq!(observations[8].cursor_steps, 1080);
        assert_eq!(observations[8].commanded_site_angles_deg10, [225, -225, 0]);
        assert_ne!(
            observations[0].commanded_site_angles_deg10,
            observations[6].commanded_site_angles_deg10
        );

        // The events name what happened, each under its cache key.
        assert_eq!(instrument.events().len(), 10);
        assert!(matches!(
            instrument.events().first(),
            Some(InstrumentEvent::Rasterised { samples: 16, .. })
        ));
        let cursors: Vec<u64> = instrument.events()[1..]
            .iter()
            .map(|e| match e {
                InstrumentEvent::ProgressRead { cursor_steps, .. } => *cursor_steps,
                other => panic!("a read walk records reads, got {other:?}"),
            })
            .collect();
        assert_eq!(
            cursors,
            vec![0, 719, 720, 1000, 1080, 1439, 1440, 2160, 1080]
        );
    }

    #[test]
    fn a_repeat_preparation_is_a_cache_hit_the_rasteriser_did_not_run() {
        let mut instrument = FormInstrument::default();
        let (first, one) = instrument.prepare(preparation()).unwrap();
        assert_eq!(one, PreparationReceipt::Rasterised);
        let (second, two) = instrument.prepare(preparation()).unwrap();
        assert_eq!(two, PreparationReceipt::CacheHit);
        assert_eq!(
            first, second,
            "the same preparation re-derives the identical body"
        );
        let counters = instrument.counter_of(&first.preparation_sha256).unwrap();
        assert_eq!(counters.rasterisations, 1);
        assert_eq!(counters.cache_hits, 1);
        assert!(matches!(
            instrument.events().last(),
            Some(InstrumentEvent::CacheHit { .. })
        ));
    }

    #[test]
    fn a_coating_change_moves_the_key_and_refuses_the_reseed() {
        let mut instrument = FormInstrument::default();
        let (coated, _) = instrument.prepare(preparation()).unwrap();
        // The mask determination moves; the topology does not.
        let mut recoated = preparation();
        recoated.mask = Some(MaskDeclaration {
            mask_ref: "app:rasteriser:moon-glyph@2".into(),
            coverages: vec![0.5; 16],
        });
        let (recoated, receipt) = instrument.prepare(recoated).unwrap();
        assert_eq!(
            receipt,
            PreparationReceipt::Rasterised,
            "a new cache key is a new rasterisation"
        );
        assert_ne!(coated.preparation_sha256, recoated.preparation_sha256);
        // The residents continue by identity — so nothing resets.
        let error = instrument.reseed(&coated, &recoated).unwrap_err();
        assert!(error.contains("continue by identity"), "{error}");
        assert!(
            error.contains("the coating moved, the body did not"),
            "{error}"
        );
        assert_eq!(instrument.totals().reseeds, 0, "no reseed was admitted");
        let counters = instrument.counter_of(&recoated.preparation_sha256).unwrap();
        assert_eq!(counters.rasterisations, 1);
        assert_eq!(counters.reseeds, 0);
    }

    #[test]
    fn a_topology_change_admits_the_reseed_through_the_explicit_mapping() {
        let mut instrument = FormInstrument::default();
        let (fine, _) = instrument.prepare(preparation()).unwrap();
        // Resolution halves: a genuine topology change (one layer, 2×2).
        let mut coarse = preparation();
        coarse.resolution = 2;
        coarse.mask.as_mut().unwrap().coverages =
            (0..2 * 2).map(|i| (i % 3) as f64 / 2.0).collect();
        let (coarse, receipt) = instrument.prepare(coarse).unwrap();
        assert_eq!(receipt, PreparationReceipt::Rasterised);
        let reseed = instrument.reseed(&fine, &coarse).unwrap();
        assert_eq!(reseed.schema, FORM_INSTRUMENT_CONTRACT);
        assert_eq!(
            reseed.continued_samples, 4,
            "every coarse point continues from its nearest predecessor"
        );
        assert_eq!(reseed.fresh_samples, 0);
        assert!(reseed.standing.contains("explicit old-to-new mapping"));
        let counters = instrument.counter_of(&coarse.preparation_sha256).unwrap();
        assert_eq!(
            counters,
            PreparationCounters {
                rasterisations: 1,
                cache_hits: 0,
                reseeds: 1,
                progress_reads: 0,
            }
        );
        assert!(matches!(
            instrument.events().last(),
            Some(InstrumentEvent::Reseeded {
                continued_samples: 4,
                fresh_samples: 0,
                ..
            })
        ));

        // A genuinely new layer reseats with fresh points named.
        let mut deeper = preparation();
        deeper.layers.push(SampleLayer {
            layer_ref: "glyph:back".into(),
            rest_depth_w: 0.75,
        });
        deeper.mask.as_mut().unwrap().coverages =
            (0..2 * 16).map(|i| (i % 3) as f64 / 2.0).collect();
        let (deeper, _) = instrument.prepare(deeper).unwrap();
        let layered = instrument.reseed(&fine, &deeper).unwrap();
        assert_eq!(layered.continued_samples, 16);
        assert_eq!(
            layered.fresh_samples, 16,
            "the new layer's points are fresh, not pretended continuations"
        );
    }

    #[test]
    fn refusals_name_the_instrument_law() {
        let mut instrument = FormInstrument::default();
        let (body, _) = instrument.prepare(preparation()).unwrap();
        // The same cache key never reseeds itself.
        let error = instrument.reseed(&body, &body).unwrap_err();
        assert!(error.contains("same cache key"), "{error}");
        // A different preparation reference is a new correspondence, not a
        // remap — the samples owner's own refusal, surfaced by name.
        let mut elsewhere = preparation();
        elsewhere.prep_ref = "ta-onta:psg:elsewhere".into();
        elsewhere.mask.as_mut().unwrap().coverages = vec![1.0; 16];
        let (elsewhere, _) = instrument.prepare(elsewhere).unwrap();
        let error = instrument.reseed(&body, &elsewhere).unwrap_err();
        assert!(error.contains("own continuation"), "{error}");
        // A progress read of a non-native receipt is refused by name.
        let progress = evaluate_fold_sequence(&sequence(), 0).unwrap();
        let mut foreign = progress.clone();
        foreign.schema = "ql.psg-fold-progress/v0";
        let error = instrument.progress_update(&body, &foreign).unwrap_err();
        assert!(
            error.contains("unsupported fold progress contract"),
            "{error}"
        );
        assert_eq!(
            instrument.totals().progress_reads,
            0,
            "a refused read counted nothing"
        );
    }

    #[test]
    fn the_observed_geometry_reads_the_retained_body() {
        let mut instrument = FormInstrument::default();
        let (body, _) = instrument.prepare(preparation()).unwrap();
        let geometry = observe(&body);
        assert_eq!(geometry.schema, RESIDENT_GEOMETRY_CONTRACT);
        assert_eq!(geometry.preparation_sha256, body.preparation_sha256);
        assert_eq!(geometry.sample_count, 16);
        // The declared coverages: (i % 3)/2 over 16 points — the six
        // i ≡ 0 (mod 3) points are unoccupied, five carry 0.5, five carry 1.0.
        assert_eq!(geometry.occupied_samples, 10);
        assert!((geometry.coverage_sum - 7.5).abs() < 1e-12);
        // The 4×4 extent, centred: half-unit sides at ±(0.5 − 1/8)·2.
        let half = (0.5 - 0.125) * 2.0;
        assert_eq!(
            geometry.rest_bounds_units,
            [
                [-half, -half, (0.25 - 0.5) * 0.5],
                [half, half, (0.25 - 0.5) * 0.5]
            ]
        );
        // Reading is pure: the same body observes identically, and a progress
        // update changes no observed row.
        assert_eq!(observe(&body), geometry);
    }

    #[test]
    fn the_disclosure_is_deterministic_and_names_the_contract() {
        let drive = || {
            let mut instrument = FormInstrument::default();
            let (body, _) = instrument.prepare(preparation()).unwrap();
            let progress = evaluate_fold_sequence(&sequence(), 1080).unwrap();
            instrument.progress_update(&body, &progress).unwrap();
            instrument.state()
        };
        let state = drive();
        assert_eq!(state["schema"], FORM_INSTRUMENT_CONTRACT);
        assert_eq!(drive(), state, "no clock, no hidden ordering");
        assert_eq!(state["counters"].as_array().unwrap().len(), 1);
        assert_eq!(
            state["counters"][0]["preparation_sha256"],
            state["events"][0]["preparation_sha256"]
        );
        assert_eq!(state["events"].as_array().unwrap().len(), 2);
    }
}
