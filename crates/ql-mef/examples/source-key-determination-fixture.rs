// Actual retained M1/M2/M3 preparation plus B source entries and K reduction.
// A consumes the available cells, rather than filling the unavailable ones.
use ql_mef::MFace;
use ql_mef::m2_tuning_sources::retained_condition_collection;
use ql_mef::music_determination::{TuningProvenance, TuningStanding};
use ql_mef::performance_audio::prepare_native_performance;
use ql_mef::source_key_determination::*;
use serde_json::json;

#[path = "../tests/support/retained_performance.rs"]
mod retained_performance;

fn main() -> Result<(), String> {
    let input = retained_performance::preparation();
    let original = input.coupled.compose()?;
    let prepared = prepare_native_performance(input)?;
    let condition = &prepared.relation_plan().execution.condition_input;
    let collection = retained_condition_collection(
        condition.maqam_index,
        condition.role,
        &prepared.targets().fundamental,
    )?;
    let writer =
        ql_mef::m_tree::native_current_m_registry().coordinate("#2-1", MFace::Pratibimba)?;
    let provenance = TuningProvenance {
        policy_ref: "reference:explicit-seven-active-key-assignment/v1".into(),
        source_ref: "reference:source-key-performance-fixture".into(),
        revision: "1".into(),
        standing: TuningStanding::Reference,
    };
    let targets = SparseKeyTargets::prepare(SparseKeyPreparation {
        determination: prepared.targets().determination.clone(),
        collection,
        reduction: ActiveKeyReduction {
            provenance: provenance.clone(),
            assignments: [0, 2, 4, 5, 7, 9, 11]
                .into_iter()
                .enumerate()
                .map(|(degree, key)| KeyDegreeAssignment {
                    key,
                    source_degree: degree as u16,
                    octave: 0,
                })
                .collect(),
        },
        // Explicitly supplied eight-component policy, independently of keys.
        // Degree7 here is the retained octave return, not an inferred filler.
        octet: Some(SourceOctetReduction {
            provenance,
            assignments: (0..8)
                .map(|component| ComponentDegreeAssignment {
                    component,
                    source_degree: u16::from(component),
                    octave: 0,
                })
                .collect(),
        }),
        requirement: SourcePitchRequirement::DeclaredAvailable,
        consumer: SparseMusicalConsumer {
            basis: prepared.native_basis(),
            writer: &writer,
            phase: prepared.targets().determination.identity().tick12() / 6,
            condition: Some(SparseConditionConsumer {
                producer_input: &original.m2_input,
                plan: prepared.relation_plan(),
            }),
        },
    })?;
    let cells = (0..12u8)
        .map(|key| {
            targets
                .key_target(key, 0, &format!("touch:source-key/{key}"))
                .and_then(|target| target.receipt())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let octet = targets
        .audio_octet_targets(0)?
        .iter()
        .map(|target| target.receipt())
        .collect::<Vec<_>>();
    let packet = json!({"schema":"ql.source-key-native-fixture/v1",
        "standing":"actual-native-source-and-target-preparation; no-device-or-audibility-verdict",
        "existing_full12_reference":prepared,"source_key_preparation":targets.preparation_receipt()?,
        "cells":cells,"source_octet_targets":octet});
    println!(
        "{}",
        serde_json::to_string(&packet).map_err(|e| e.to_string())?
    );
    Ok(())
}
