//! Source-qualified anatomy over the current public M registry.
//!
//! The anatomical text below is a bounded projection of pinned Bimba properties,
//! not a new body geometry. Property references use the immutable source file and
//! an RFC 6901 JSON pointer. No centre-to-organ action or sensory edge is inferred
//! from ordinal, purpose text, elemental similarity or numerical resonance.

use serde::Serialize;

use crate::m_tree::{MRegistry, native_current_m_registry};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BodyZoneReading {
    pub source_ref: String,
    pub registry_revision: String,
    pub property_ref: String,
    pub repository: String,
    pub revision: String,
    pub path: String,
    pub git_blob: String,
    pub payload_sha256: String,
    pub record_index: usize,
    pub anatomical_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CentreBodyReading {
    pub ordinal: u8,
    pub body_zone: BodyZoneReading,
    pub sense_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub sense_standing: String,
    pub action_standing: String,
}

/// Source anatomy for all seven canonical centres, in root-to-crown order.
/// The source's anatomical associations are symbolic body correspondences, not
/// clinical assertions or metre-valued geometry.
pub fn centre_body(ordinal: u8) -> Result<CentreBodyReading, String> {
    centre_body_from_registry(native_current_m_registry(), ordinal)
}

fn centre_body_from_registry(
    registry: &MRegistry,
    ordinal: u8,
) -> Result<CentreBodyReading, String> {
    // Epi-Logos-C-Experiments @ daa660cbc1b8c5da83828698665a753852cb0287,
    // Idea/Bimba/Map/datasets/parashakti-deep/nodes-full-detail.json,
    // records 582..588, filteredProps.anatomicalLocation. Exact payload digests
    // prevent a newer registry silently attributing this text to changed source.
    let zones = [
        (
            "#2-5-0/1-1",
            "Base of spine, perineum, pelvic floor",
            "6e31a60f44128c8dc68c808e7405958c910edf4205911d8752790d35ebfb0cf9",
        ),
        (
            "#2-5-0/1-2",
            "Lower abdomen, sacral region, reproductive organs",
            "36e6a50ff940c32a64b6dcfff5d2397aebba057eca5022b0779eac848bfac709",
        ),
        (
            "#2-5-0/1-3",
            "Solar plexus, upper abdomen, digestive system",
            "5824a1ad80616f35c807d23f48774cebd69004d88202b0f8d41b3ec0d5ac5e59",
        ),
        (
            "#2-5-0/1-4",
            "Heart region, chest center, cardiac plexus",
            "7f6109577f1334a4efb2139479e64ad3c62a554f31476b6752f08ac8e63eb2d5",
        ),
        (
            "#2-5-0/1-5",
            "Throat region, thyroid, vocal apparatus",
            "27a873292d426143dee13767564087f55c57612ba0fcfc48cc0443d39866f0cc",
        ),
        (
            "#2-5-0/1-6",
            "Between eyebrows, pineal gland, third eye region",
            "77d3b8375dbba56854d6cdda8060f8a24cc67a562baae423c2b4fb787ce1bcb6",
        ),
        (
            "#2-5-0/1-7",
            "Crown of head, fontanelle, cerebral cortex",
            "9a5c7421100bdff8ab8b53616f17d5e9ab0b3d866c185232d4771c98e85b8bd5",
        ),
    ];
    let (coordinate, anatomy, payload) = zones
        .get(usize::from(ordinal))
        .ok_or("body centre ordinal outside the canonical seven")?;
    let node = registry
        .resolve(coordinate)
        .ok_or("canonical body centre unavailable")?;
    let manifest = registry.manifest();
    let record = node
        .records
        .iter()
        .filter_map(|index| manifest.records.get(*index))
        .find(|record| {
            record.payload_sha256 == *payload
                && record
                    .property_keys
                    .iter()
                    .any(|key| key == "c_2_anatomical_location")
        })
        .ok_or("canonical anatomical source property changed or unavailable")?;
    let file = manifest
        .files
        .get(record.file)
        .ok_or("body source file unavailable")?;
    // The registry is built from the live Bimba map read (#260): the file is that
    // read, pinned by its content digest, which is the manifest's source revision.
    if file.record_class != "bimba-map-read" || file.sha256 != manifest.source_revision {
        return Err("canonical anatomical source is not the pinned Bimba map read".into());
    }
    let repository = file
        .repository
        .as_ref()
        .unwrap_or(&manifest.source_repository);
    let revision = file.revision.as_ref().unwrap_or(&manifest.source_revision);
    let property_ref = format!(
        "{}@{revision}#/{}/c_2_anatomical_location",
        file.path, record.record_index
    );
    Ok(CentreBodyReading {
        ordinal,
        body_zone: BodyZoneReading {
            source_ref: node.source_ref.clone(),
            registry_revision: manifest.registry_revision.clone(),
            property_ref,
            repository: repository.clone(),
            revision: revision.clone(),
            path: file.path.clone(),
            git_blob: file.git_blob.clone(),
            payload_sha256: record.payload_sha256.clone(),
            record_index: record.record_index,
            anatomical_location: (*anatomy).into(),
        },
        sense_refs: Vec::new(),
        action_refs: Vec::new(),
        sense_standing: "unavailable: source records disclose elemental and sensory correspondences but no direct centre-to-sense relation is projected".into(),
        action_standing: "unavailable: no direct centre-to-action relation is projected; operationalEssence remains descriptive source, not an action edge".into(),
    })
}

/// Existing Earth-to-root grounding relations. Earth remains a separate anchor;
/// duplicated authored records retain their distinct registered relation refs.
pub fn earth_grounding_refs() -> Result<Vec<String>, String> {
    let registry = native_current_m_registry();
    let earth = registry
        .resolve("#2-5-0/1-0")
        .ok_or("canonical EarthBody source unavailable")?;
    let mut refs = registry
        .relations_for(earth.id)
        .filter(|relation| {
            relation.from_ref.as_deref() == Some("#2-5-0/1-0")
                && relation.to_ref.as_deref() == Some("#2-5-0/1-1")
                && matches!(
                    relation.source_kind.as_str(),
                    "GROUNDS_CHAKRAL_PATHWAY" | "FEEDS_EARTH_ELEMENT"
                )
        })
        .map(|relation| relation.relation_ref.clone())
        .collect::<Vec<_>>();
    refs.sort();
    refs.dedup();
    if refs.is_empty() {
        return Err("canonical EarthBody grounding relations unavailable".into());
    }
    Ok(refs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_anatomical_properties_resolve_against_compiled_source_records() {
        let registry = native_current_m_registry();
        let mut refs = std::collections::BTreeSet::new();
        for ordinal in 0..7 {
            let reading = centre_body(ordinal).unwrap();
            let zone = reading.body_zone;
            // Records 870..876 of the live Bimba map read (#260).
            assert_eq!(zone.record_index, 870 + usize::from(ordinal));
            assert_eq!(zone.source_ref, format!("#2-5-0/1-{}", ordinal + 1));
            let node = registry.resolve(&zone.source_ref).unwrap();
            assert!(node.records.iter().any(|index| {
                let record = &registry.manifest().records[*index];
                record.payload_sha256 == zone.payload_sha256
                    && record.record_index == zone.record_index
                    && registry.manifest().files[record.file].git_blob == zone.git_blob
            }));
            assert!(
                zone.property_ref
                    .ends_with(&format!("#/{}/c_2_anatomical_location", zone.record_index))
            );
            assert!(refs.insert(zone.property_ref));
            assert!(reading.sense_refs.is_empty());
            assert!(reading.action_refs.is_empty());
            assert!(reading.action_standing.contains("unavailable"));
        }
        assert!(centre_body(7).is_err());
    }

    #[test]
    fn anatomical_projection_refuses_changed_source_instead_of_relabelling_text() {
        let mut manifest = native_current_m_registry().manifest().clone();
        let node = manifest
            .nodes
            .iter()
            .find(|node| node.source_ref == "#2-5-0/1-1")
            .unwrap();
        let indices = node.records.clone();
        for index in indices {
            manifest.records[index].payload_sha256 = "0".repeat(64);
        }
        let registry = MRegistry::from_json(&serde_json::to_string(&manifest).unwrap()).unwrap();
        assert!(
            centre_body_from_registry(&registry, 0)
                .unwrap_err()
                .contains("changed or unavailable")
        );
    }

    #[test]
    fn earth_relations_are_actual_grounding_edges_not_an_eighth_centre() {
        let registry = native_current_m_registry();
        let refs = earth_grounding_refs().unwrap();
        let mut kinds = std::collections::BTreeSet::new();
        for reference in refs {
            let relation = registry
                .manifest()
                .relations
                .iter()
                .find(|relation| relation.relation_ref == reference)
                .unwrap();
            assert_eq!(relation.from_ref.as_deref(), Some("#2-5-0/1-0"));
            assert_eq!(relation.to_ref.as_deref(), Some("#2-5-0/1-1"));
            kinds.insert(relation.source_kind.as_str());
        }
        assert_eq!(
            kinds,
            std::collections::BTreeSet::from(["FEEDS_EARTH_ELEMENT", "GROUNDS_CHAKRAL_PATHWAY"])
        );
    }
}
