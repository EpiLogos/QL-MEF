//! Production Technē source over the live Wiki refraction seam.
//!
//! Composes the registry disclosure provider (canonical addresses and
//! lens/sublens semantics) with the shape-aware Wiki refraction engine
//! (shape-binding validation and byte-exact member propagation) into a
//! [`TechneAdapter`] read model. The adapter is not a store: the Wiki owner
//! supplies each subject's own projection (`WikiRefractionTarget`) through
//! [`WikiSubjectSource`], and every reference in the resulting reading is
//! copied verbatim from that target, the provider or the engine response. A
//! bounded whole is disclosed only from a validated shape binding — layout
//! never manufactures graph relations.

use crate::techne::{
    DisclosureSelection, InstrumentDisclosure, QlResultClass, QlWarrant, ReadingSnapshot,
    SourceProvenance, SubjectReading, TECHNE_CONTRACT, TechneActionRoute, TechneActionRouteReceipt,
    TechneAdapter, TechneDisclosure, TechneInstrument, TechneReading, TechneReadingKind,
    TechneSubject, TechneWhole, WarrantedQlReading,
};

type AdapterResult<T> = Result<T, crate::AdapterError>;
use ql_wiki::{
    LensSelection, ProviderMode, RegistryDisclosureProvider, ShapeAwareWikiRefractionEngine,
    WIKI_REFRACTION_CONTRACT, WikiRefractionTarget, shape_binding_from_target,
};
use serde_json::Map;

/// The lens the source refracts through when the caller does not pin one.
/// The reference lens keeps the reading deterministic; callers needing
/// another aperture compose their own request through `reading_for_request`.
pub const SOURCE_LENS_REF: &str = "mef:lens:L1@1";

/// The Wiki owner's subject-resolution handle. The owner keeps its own
/// projection state; the adapter holds only this reference and stays
/// store-free.
pub trait WikiSubjectSource {
    fn wiki_target(&self, subject_ref: &str) -> Option<WikiRefractionTarget>;
}

impl<T> WikiSubjectSource for T
where
    T: Fn(&str) -> Option<WikiRefractionTarget>,
{
    fn wiki_target(&self, subject_ref: &str) -> Option<WikiRefractionTarget> {
        self(subject_ref)
    }
}

/// The production M0′/M1′ Technē source over live Wiki subjects.
pub struct WikiTechneAdapter<'a> {
    provider: RegistryDisclosureProvider,
    subjects: &'a dyn WikiSubjectSource,
}

impl<'a> WikiTechneAdapter<'a> {
    pub fn new(subjects: &'a dyn WikiSubjectSource) -> Self {
        Self {
            provider: RegistryDisclosureProvider::new(),
            subjects,
        }
    }

    /// The production composition: one live Wiki target → one Technē
    /// reading. Refs are byte-exact: subject and revision come from the
    /// target, reading/provenance refs come from the engine response, and
    /// shape/whole/member refs come only from a validated shape binding.
    pub fn reading_for_target(
        &self,
        target: &WikiRefractionTarget,
    ) -> AdapterResult<TechneReading> {
        let request = ql_wiki::WikiRefractionRequest {
            contract: WIKI_REFRACTION_CONTRACT.into(),
            mode: ProviderMode::Required,
            target: target.clone(),
            lenses: vec![LensSelection {
                lens_ref: SOURCE_LENS_REF.into(),
                sublens_ref: None,
            }],
            context: Map::new(),
        };
        let engine = ShapeAwareWikiRefractionEngine::new(Some(&self.provider));
        let response = engine.refract(&request).map_err(|error| {
            crate::AdapterError::InvalidTechneReading(format!(
                "wiki refraction refused {}: {error}",
                target.target_ref
            ))
        })?;
        let first = response.readings.first().ok_or_else(|| {
            crate::AdapterError::InvalidTechneReading(format!(
                "wiki refraction returned no reading for {}",
                target.target_ref
            ))
        })?;
        let binding = shape_binding_from_target(target).map_err(|error| {
            crate::AdapterError::InvalidTechneReading(format!(
                "shape binding refused {}: {error}",
                target.target_ref
            ))
        })?;

        let mut derivation_refs = first.operator_refs.clone();
        let mut evidence_refs = vec![first.reading_ref.clone()];
        if let Some(binding) = &binding {
            if let Some(operator_ref) = &binding.operator_ref {
                if !derivation_refs.contains(operator_ref) {
                    derivation_refs.push(operator_ref.clone());
                }
            }
            if let Some(derivation_ref) = &binding.derivation_ref {
                if !derivation_refs.contains(derivation_ref) {
                    derivation_refs.push(derivation_ref.clone());
                }
            }
            evidence_refs.push(binding.whole_ref.clone());
        }

        let ql = WarrantedQlReading {
            address: Some(target.target_ref.clone()),
            // The refraction target carries no warranted M identity and the
            // owner supplies no return ground here: both stay absent rather
            // than manufactured.
            m_coordinate_ref: None,
            shape_ref: binding
                .as_ref()
                .map(|binding| binding.shape_ref.clone())
                .or_else(|| first.ql_form_refs.first().cloned()),
            constellation_ref: None,
            lens_ref: Some(first.lens_ref.clone()),
            sublens_ref: first.sublens_ref.clone(),
            context_frame_ref: None,
            refraction_summary: Some(format!("{}:{}", first.disclosure, first.disclosure_status)),
            harmonic_reading: first.harmonic_field_ref.clone(),
            geometric_reading: None,
            vak_source_ref: None,
            derivation_refs,
            return_ref: None,
            warrant: QlWarrant {
                result_class: QlResultClass::Deterministic,
                evidence_refs,
                provenance_ref: format!(
                    "{}:{}",
                    first.provider.provider_ref, first.provider.provider_version
                ),
            },
        };

        let whole = binding.map(|binding| TechneWhole {
            whole_ref: binding.whole_ref.clone(),
            member_refs: binding.basis_refs.clone(),
            relations: Vec::new(),
            focus_refs: Vec::new(),
        });

        let instruments = vec![
            InstrumentDisclosure {
                instrument: TechneInstrument::Project,
                available: true,
                reason: None,
                m_prime: Some(0),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Canvas,
                available: whole.is_some(),
                reason: whole.is_none().then_some(
                    "no validated shape binding discloses a bounded local whole for this subject"
                        .into(),
                ),
                m_prime: Some(1),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Timeline,
                available: false,
                reason: Some(
                    "no live temporal producer is attached to this subject (native owner: Factory/AIKit T1)"
                        .into(),
                ),
                m_prime: Some(2),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Journey,
                available: false,
                reason: Some(
                    "no source-backed Expression Scenes are composed for this subject".into(),
                ),
                m_prime: Some(3),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Place,
                available: false,
                reason: Some("no place facets are disclosed for this subject".into()),
                m_prime: Some(4),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Palace,
                available: false,
                reason: Some("Palace producer is not landed (native owner: O-I #218)".into()),
                m_prime: Some(5),
                reading: Some(TechneReadingKind::DeepFourTwo),
            },
            InstrumentDisclosure {
                instrument: TechneInstrument::Expressions,
                available: false,
                reason: Some("no Expression binding is disclosed for this subject".into()),
                m_prime: None,
                reading: Some(TechneReadingKind::ConjugateThreeThree),
            },
        ];

        let reading = TechneReading {
            contract: TECHNE_CONTRACT.into(),
            reading_ref: format!("{TECHNE_CONTRACT}:{}", first.reading_ref),
            snapshot: Some(ReadingSnapshot {
                revision: target.target_revision.as_ref().map(|r| r.to_string()),
                basis_ref: Some(target.target_snapshot_hash.clone()),
            }),
            subject: TechneSubject {
                subject_ref: target.target_ref.clone(),
                native_owner: "ql-mef/wiki".into(),
                native_revision: target.target_revision.as_ref().map(|r| r.to_string()),
                readings: response
                    .readings
                    .iter()
                    .map(|r| SubjectReading {
                        r#ref: r.reading_ref.clone(),
                        revision: r.target_revision.as_ref().map(|r| r.to_string()),
                    })
                    .collect(),
                kind: Some(first.reading_type.clone()),
                standing: None,
            },
            whole,
            ql: Some(ql),
            temporal: Vec::new(),
            spatial: Vec::new(),
            provenance: target
                .provenance
                .iter()
                .map(|p| SourceProvenance {
                    source_ref: p.source_ref.clone(),
                    source_revision: p.source_revision.as_ref().map(|r| r.to_string()),
                    native_owner: "ql-mef/wiki".into(),
                    selector: None,
                    standing: None,
                    evidence_refs: Vec::new(),
                })
                .collect(),
            expressions: Vec::new(),
            actions: Vec::new(),
            // No situated-Agency role arises from a Wiki refraction source.
            agency: Vec::new(),
            disclosure: TechneDisclosure {
                instruments,
                degraded: Vec::new(),
                // Cut-level availability is disclosed by the instrument
                // surfaces; the Wiki source contributes none of its own.
                application_cuts: Vec::new(),
                suggestions: Vec::new(),
            },
        };
        reading.validate()?;
        Ok(reading)
    }

    /// Co-referenced selections for the instruments this subject actually
    /// supports: the ground (M0′) and, when a shape binding is disclosed,
    /// the canvas (M1′).
    pub fn selections(
        &self,
        target: &WikiRefractionTarget,
        reading: &TechneReading,
    ) -> AdapterResult<Vec<DisclosureSelection>> {
        let mut selections = vec![DisclosureSelection {
            selection_ref: format!("{}:ground", reading.reading_ref),
            subject_ref: reading.subject.subject_ref.clone(),
            coordinate_ref: reading.ql.as_ref().and_then(|ql| ql.address.clone()),
            source_ref: target.provenance.first().map(|p| p.source_ref.clone()),
            source_revision: target.target_revision.as_ref().map(|r| r.to_string()),
            disclosure_ref: Some(reading.reading_ref.clone()),
            focus_refs: reading
                .whole
                .as_ref()
                .map(|w| w.focus_refs.clone())
                .unwrap_or_default(),
            reading_ref: reading
                .ql
                .as_ref()
                .and_then(|ql| ql.lens_ref.clone())
                .unwrap_or_default(),
            snapshot_revision: reading.snapshot.as_ref().and_then(|s| s.revision.clone()),
            instrument: TechneInstrument::Project,
            agent_session_ref: None,
            selection_standing: None,
        }];
        if let Some(whole) = &reading.whole {
            selections.push(DisclosureSelection {
                instrument: TechneInstrument::Canvas,
                selection_ref: format!("{}:canvas:{}", reading.reading_ref, whole.whole_ref),
                focus_refs: whole.member_refs.clone(),
                ..selections[0].clone()
            });
        }
        Ok(selections)
    }
}

impl TechneAdapter for WikiTechneAdapter<'_> {
    fn reading(&self, subject_ref: &str) -> AdapterResult<TechneReading> {
        let target = self.subjects.wiki_target(subject_ref).ok_or_else(|| {
            crate::AdapterError::InvalidTechneReading(format!(
                "wiki holds no projection for subject {subject_ref}"
            ))
        })?;
        self.reading_for_target(&target)
    }

    fn route_action(
        &self,
        route: &TechneActionRoute,
        reading: &TechneReading,
    ) -> AdapterResult<TechneActionRouteReceipt> {
        Ok(
            match reading
                .actions
                .iter()
                .find(|a| a.action_ref == route.action_ref)
            {
                Some(action) => TechneActionRouteReceipt {
                    action_ref: action.action_ref.clone(),
                    native_owner: action.native_owner.clone(),
                    routed: true,
                    reason: None,
                    authority: Some(action.authority.clone()),
                    expected_effects: action.expected_effects.clone(),
                },
                None => TechneActionRouteReceipt {
                    action_ref: route.action_ref.clone(),
                    native_owner: reading.subject.native_owner.clone(),
                    routed: false,
                    reason: Some(format!(
                        "subject {} discloses no native Actions; routing requires a disclosed owner Action",
                        reading.subject.subject_ref
                    )),
                    authority: None,
                    expected_effects: Vec::new(),
                },
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ql_core::{ConstellationGrain, QlShape, QlShapeCompression};
    use ql_wiki::{
        WikiShapeBindingProvenance, WikiShapeBindingView, WikiShapeMember, WikiTargetKind,
        attach_shape_binding,
    };

    fn target(reference: &str) -> WikiRefractionTarget {
        WikiRefractionTarget {
            kind: WikiTargetKind::Frame,
            target_ref: reference.into(),
            target_frame_ref: None,
            target_revision: None,
            target_snapshot_hash: "sha256:techne-wiki-test".into(),
            provenance: vec![ql_wiki::WikiProvenanceRef {
                source_ref: "example:source".into(),
                source_revision: None,
                extensions: std::collections::BTreeMap::new(),
            }],
            subjects: vec![],
            relations: vec![],
            structural_field: None,
            material: Map::new(),
            extensions: Map::new(),
        }
    }

    fn compressed_binding(subject_ref: &str) -> WikiShapeBindingView {
        let members = [
            ("example:a", 1_u8),
            ("example:b", 2_u8),
            ("example:c", 3_u8),
        ];
        let disclosed = QlShape::Constellation(ConstellationGrain::ThreeFold123);
        let compression = QlShapeCompression::to_onefold(disclosed).unwrap();
        WikiShapeBindingView {
            subject_ref: subject_ref.into(),
            shape_ref: compression.presented.shape_ref(),
            whole_ref: "example:whole-anchor".into(),
            basis_refs: members
                .iter()
                .map(|(reference, _)| (*reference).into())
                .collect(),
            members: members
                .iter()
                .map(|(reference, position)| WikiShapeMember {
                    subject_ref: (*reference).into(),
                    position: *position,
                    face: "direct".into(),
                })
                .collect(),
            derivation_ref: Some(compression.derivation_ref()),
            operator_ref: Some(compression.operator_ref.into()),
            return_refs: vec!["example:return".into()],
            provenance: WikiShapeBindingProvenance {
                caller_ref: "ql-adapters:test".into(),
                source_ref: "example:source".into(),
                standing_ref: "AUTHORED-ARCHITECTURE".into(),
            },
        }
    }

    fn bound_target() -> WikiRefractionTarget {
        let mut target = target("example:frame");
        attach_shape_binding(&mut target, &compressed_binding("example:frame")).unwrap();
        target
    }

    fn shape_bound_subjects() -> impl WikiSubjectSource {
        move |subject_ref: &str| (subject_ref == "example:frame").then(bound_target)
    }

    #[test]
    fn production_source_preserves_byte_exact_refs_through_a_shape_bound_whole() {
        let subjects = shape_bound_subjects();
        let adapter = WikiTechneAdapter::new(&subjects);
        let reading = adapter.reading("example:frame").unwrap();
        assert_eq!(reading.subject.subject_ref, "example:frame");
        assert_eq!(reading.subject.native_owner, "ql-mef/wiki");
        let whole = reading.whole.as_ref().unwrap();
        assert_eq!(whole.whole_ref, "example:whole-anchor");
        assert_eq!(
            whole.member_refs,
            vec!["example:a", "example:b", "example:c"]
        );
        let ql = reading.ql.as_ref().unwrap();
        assert_eq!(
            ql.shape_ref,
            Some(compressed_binding("example:frame").shape_ref)
        );
        assert_eq!(ql.warrant.result_class, QlResultClass::Deterministic);
        assert!(
            ql.warrant
                .evidence_refs
                .contains(&"example:whole-anchor".to_string())
        );
        assert_eq!(
            reading.snapshot.as_ref().unwrap().basis_ref,
            Some("sha256:techne-wiki-test".into())
        );
        assert!(reading.actions.is_empty());
    }

    #[test]
    fn disclosure_is_honest_per_instrument_and_validates() {
        let subjects = shape_bound_subjects();
        let adapter = WikiTechneAdapter::new(&subjects);
        let reading = adapter.reading("example:frame").unwrap();
        let available: Vec<_> = reading
            .disclosure
            .instruments
            .iter()
            .filter(|i| i.available)
            .map(|i| i.instrument)
            .collect();
        assert_eq!(
            available,
            vec![TechneInstrument::Project, TechneInstrument::Canvas]
        );
        for instrument in &reading.disclosure.instruments {
            if !instrument.available {
                assert!(instrument.reason.is_some());
            }
            if instrument.instrument == TechneInstrument::Expressions {
                assert_eq!(
                    instrument.reading,
                    Some(TechneReadingKind::ConjugateThreeThree)
                );
                assert!(instrument.m_prime.is_none());
            }
        }
        reading.validate().unwrap();
    }

    #[test]
    fn unbound_subject_keeps_the_ground_and_refuses_canvas_honestly() {
        let subjects =
            |subject_ref: &str| (subject_ref == "example:bare").then(|| target("example:bare"));
        let adapter = WikiTechneAdapter::new(&subjects);
        let reading = adapter.reading("example:bare").unwrap();
        assert!(reading.whole.is_none());
        let canvas = reading
            .disclosure
            .instruments
            .iter()
            .find(|i| i.instrument == TechneInstrument::Canvas)
            .unwrap();
        assert!(!canvas.available);
        assert!(canvas.reason.is_some());
        assert_eq!(
            reading.ground_ref(),
            "example:bare",
            "the ground entry falls back to the subject itself"
        );
    }

    #[test]
    fn selections_are_co_referenced_across_the_disclosed_instruments() {
        let subjects = shape_bound_subjects();
        let adapter = WikiTechneAdapter::new(&subjects);
        let target = bound_target();
        let reading = adapter.reading("example:frame").unwrap();
        let selections = adapter.selections(&target, &reading).unwrap();
        assert_eq!(selections.len(), 2);
        assert!(selections[0].co_referenced(&selections[1]));
        assert_eq!(selections[1].instrument, TechneInstrument::Canvas);
        assert_eq!(
            selections[1].focus_refs,
            vec!["example:a", "example:b", "example:c"]
        );
    }

    #[test]
    fn unknown_subjects_and_undisclosed_actions_refuse_without_inventing_ownership() {
        let subjects = shape_bound_subjects();
        let adapter = WikiTechneAdapter::new(&subjects);
        assert!(adapter.reading("example:absent").is_err());
        let reading = adapter.reading("example:frame").unwrap();
        let receipt = adapter
            .route_action(
                &TechneActionRoute {
                    action_ref: "wiki:rewrite".into(),
                    subject_ref: "example:frame".into(),
                    selection_ref: None,
                    input: None,
                },
                &reading,
            )
            .unwrap();
        assert!(!receipt.routed);
        assert_eq!(receipt.native_owner, "ql-mef/wiki");
        assert!(receipt.authority.is_none());
    }

    #[test]
    fn repeated_readings_are_deterministic_and_ref_bytes_are_untouched() {
        let subjects = shape_bound_subjects();
        let adapter = WikiTechneAdapter::new(&subjects);
        let first = adapter.reading("example:frame").unwrap();
        let second = adapter.reading("example:frame").unwrap();
        assert_eq!(first, second);
    }
}
