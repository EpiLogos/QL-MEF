//! All historical source bodies compile before the SAME selected checkpoint
//! owner is activated. Full source lineage and original numerical wire remain
//! distinct; retained JSON cannot mint an opening, clock or receiving grant.
use super::*;
use crate::continuous::performance::{
    NativeStoppedExchangeFailure, PreparedColdPhysicalSource, QualifiedAcousticSourceHistory,
    qualify_saved_acoustic_physical_history,
};
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::musical_performance_source_score::{NativeScorePages, RetainedScoreSource};

pub(in crate::continuous) struct PreparedColdNativeAct {
    complete: PreparedColdPhysicalSource,
    selected: PreparedColdPhysicalSource,
    current_basis: Value,
    receiving_inputs: Value,
}
fn physical_applications(asset: &Value) -> Result<&[Value], String> {
    match asset.get("native_physical_source_history") {
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| "complete native physical application corpus has wrong type".into()),
        None => Ok(&[]),
    }
}
fn acoustic_applications(asset: &Value) -> Result<&[Value], String> {
    match asset.get("native_acoustic_source_history") {
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| "complete native acoustic application corpus has wrong type".into()),
        None => Ok(&[]),
    }
}
fn receiving_epoch(
    source: &NativePerformanceReceivingSource,
    asset: &Value,
) -> Result<NativePerformanceReceivingSource, String> {
    if let Some(configuration) =
        asset["native_bundle"]["receiving_source_inputs"].get("acoustic_receiving")
    {
        source.clone().with_acoustic_configuration(
            serde_json::from_value(configuration.clone()).map_err(|e| e.to_string())?,
        )
    } else {
        Ok(source.clone().without_acoustic_configuration())
    }
}
fn source_depth(asset: &Value) -> Result<usize, String> {
    let records = match asset["native_bundle"].get("physical_transition_history") {
        Some(value) => value
            .as_array()
            .ok_or("complete native physical source lineage has wrong type")?
            .len(),
        None => 0,
    };
    if records != physical_applications(asset)?.len() {
        return Err("native physical source lacks its complete actual application corpus".into());
    }
    let acoustic_records = match asset["native_bundle"].get("acoustic_transition_history") {
        Some(value) => value
            .as_array()
            .ok_or("complete native acoustic lineage has wrong type")?
            .len(),
        None => 0,
    };
    if acoustic_records != acoustic_applications(asset)?.len() {
        return Err("native acoustic source lacks complete actual applications".into());
    }
    records
        .checked_add(acoustic_records)
        .filter(|v| *v < 256)
        .ok_or_else(|| "complete native source epoch bound exceeded".into())
}
fn frame_index(replayed: &PreparedColdPhysicalSource, asset: &Value) -> Result<usize, String> {
    let mut matches = replayed
        .frames()
        .iter()
        .enumerate()
        .filter(|(_, frame)| asset["native_bundle"] == *frame.owner().source_assets());
    let (index, frame) = matches
        .next()
        .ok_or("Act source body is absent from complete original-owner replay")?;
    if matches.next().is_some()
        || physical_applications(asset)?
            != frame.owner().native_physical_source_history().as_slice()
        || acoustic_applications(asset)?
            != frame.owner().native_acoustic_source_history().as_slice()
    {
        return Err(
            "Act body has ambiguous source or detached actual physical applications".into(),
        );
    }
    Ok(index)
}
impl FieldHost {
    pub(in crate::continuous) fn prepare_cold_native_act(
        &self,
        manifest: &Value,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedColdNativeAct, String> {
        if self.performance.is_some() || !self.available() {
            return Err("cold continuation requires a new available actual source owner".into());
        }
        let receiving = self
            .receiving_source
            .as_ref()
            .ok_or("cold Act opening has no privately admitted original receiving constructor")?;
        let sources = manifest["performance"]["native_sources"]
            .as_array()
            .filter(|v| !v.is_empty() && v.len() <= 256)
            .ok_or("cold Act requires its complete original source cohort")?;
        let depths = sources
            .iter()
            .map(source_depth)
            .collect::<Result<Vec<_>, _>>()?;
        let maximum = depths
            .iter()
            .max()
            .copied()
            .ok_or("complete original source cohort absent")?;
        let mut maximal = sources
            .iter()
            .zip(&depths)
            .filter(|(_, depth)| **depth == maximum);
        let latest = maximal
            .next()
            .ok_or("complete original source cohort absent")?
            .0;
        if maximal.next().is_some() {
            return Err("cold Act physical source lineage has two current descendants".into());
        }
        let selected = lease.selected_checkpoint_source(&self.instance_ref)?;
        let source = receiving_epoch(receiving, latest)?;
        let current = self.session.session().current_basis();
        let complete = PerformanceOwner::prepare_cold_native_acoustic_source(
            current,
            &self.instance_ref,
            &source,
            &latest["native_bundle"],
            physical_applications(latest)?,
            acoustic_applications(latest)?,
            lease,
        )?;
        // ALL retained bodies must belong to this actual independently replayed
        // lineage. A later final source never stands in for a missing old body.
        for asset in sources {
            frame_index(&complete, asset)?;
            lease.validate_source_assets(&self.instance_ref, &asset["native_bundle"])?;
            lease.validate_recorded_source_applications(
                &self.instance_ref,
                &asset["native_bundle"],
                physical_applications(asset)?,
                acoustic_applications(asset)?,
            )?;
        }
        frame_index(&complete, selected)?;
        // A saved earlier body is reconstructed from its actual complete retained
        // prefix, while the complete Act replay remains alive for every page.
        let selected_source = receiving_epoch(receiving, selected)?;
        let selected = PerformanceOwner::prepare_cold_native_acoustic_source(
            current,
            &self.instance_ref,
            &selected_source,
            &selected["native_bundle"],
            physical_applications(selected)?,
            acoustic_applications(selected)?,
            lease,
        )?;
        Ok(PreparedColdNativeAct {
            complete,
            selected,
            current_basis: serde_json::to_value(current).map_err(|e| e.to_string())?,
            receiving_inputs: receiving.source_inputs()?,
        })
    }
    pub(in crate::continuous) fn compile_cold_native_act_score(
        &self,
        candidate: &PreparedColdNativeAct,
        generation: u64,
        manifest: &Value,
        pages: &mut impl NativeScorePages,
    ) -> Result<Value, String> {
        let receiving = self.cold_act_source(candidate)?;
        let bases = manifest["performance"]["bases"]
            .as_array()
            .ok_or("complete native bases absent")?;
        let assets = manifest["performance"]["native_sources"]
            .as_array()
            .ok_or("complete native assets absent")?;
        let mut indices = Vec::with_capacity(assets.len());
        let mut receiving_sources = Vec::with_capacity(assets.len());
        let mut returns = Vec::with_capacity(assets.len());
        for asset in assets {
            let mut matching = bases
                .iter()
                .filter(|basis| basis["content_digest"] == asset["basis_digest"]);
            let basis = matching
                .next()
                .ok_or("source epoch has no original musical basis")?;
            if matching.next().is_some() {
                return Err("source epoch musical basis ambiguous".into());
            }
            let index = frame_index(&candidate.complete, asset)?;
            let frame = &candidate.complete.frames()[index];
            let source = frame.receiving_source(&receiving)?;
            let seed = exact_cursor(
                basis["seed"]
                    .as_str()
                    .ok_or("original native Return seed absent")?,
            )?;
            returns.push(frame.prepare_return(&source, seed)?);
            receiving_sources.push(source);
            indices.push(index);
        }
        let sources = indices
            .iter()
            .enumerate()
            .map(|(ordinal, index)| {
                let frame = &candidate.complete.frames()[*index];
                RetainedScoreSource {
                    owner: frame.owner(),
                    current: frame.current(),
                    receiving: &receiving_sources[ordinal],
                    original_return: &returns[ordinal],
                    source_sample: frame.source_sample(),
                }
            })
            .collect::<Vec<_>>();
        let score = crate::musical_performance_source_score::compile_retained_score(
            generation, &sources, manifest, pages,
        )?
        .snapshot()?;
        self.cold_act_source(candidate)?;
        Ok(score)
    }
    pub(in crate::continuous) fn qualify_cold_native_act_checkpoint(
        &self,
        candidate: &PreparedColdNativeAct,
        lease: &NativeActSourceLease<'_>,
        checkpoint_ref: &str,
        original_wire: &str,
    ) -> Result<Option<QualifiedAcousticSourceHistory>, String> {
        let family = self.cold_act_source(candidate)?;
        let source = candidate.selected.final_frame().receiving_source(&family)?;
        lease.validate_selected_checkpoint(&self.instance_ref, checkpoint_ref, original_wire)?;
        let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
        if saved["native_pair"]["audio"]["receiving"]["schema"]
            != "ql.performance-receiving-checkpoint/v2"
        {
            return Ok(None);
        }
        let history = qualify_saved_acoustic_physical_history(
            &candidate.selected,
            &source,
            lease,
            &self.instance_ref,
            checkpoint_ref,
            original_wire,
        )?;
        self.cold_act_source(candidate)?;
        Ok(Some(history))
    }
    fn cold_act_source(
        &self,
        candidate: &PreparedColdNativeAct,
    ) -> Result<NativePerformanceReceivingSource, String> {
        if self.performance.is_some()
            || !self.available()
            || serde_json::to_value(self.session.session().current_basis())
                .map_err(|e| e.to_string())?
                != candidate.current_basis
        {
            return Err("cold actual Field source changed during complete Act compilation".into());
        }
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("cold original receiving owner disappeared")?
            .clone();
        if source.source_inputs()? != candidate.receiving_inputs {
            return Err("cold original receiving source/context/episode changed".into());
        }
        Ok(source)
    }
    pub(in crate::continuous) fn activate_cold_native_act(
        &mut self,
        candidate: PreparedColdNativeAct,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<Vec<Value>, NativeStoppedExchangeFailure> {
        let family = self.cold_act_source(&candidate)?;
        let source = candidate.selected.final_frame().receiving_source(&family)?;
        let current_basis = candidate.current_basis;
        let receiving_inputs = candidate.receiving_inputs;
        // Full replay stayed alive through all-page compilation and dated
        // checkpoint qualification. Numerical activation uses the selected body.
        let (mut owner, current) = candidate.selected.into_final();
        lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
        let receipts =
            owner.activate_cold_act_source(&current, &source, self.session.session_mut())?;
        let checked = (|| -> Result<(), String> {
            if self.performance.is_some()
                || !self.available()
                || serde_json::to_value(self.session.session().current_basis())
                    .map_err(|e| e.to_string())?
                    != current_basis
                || self
                    .receiving_source
                    .as_ref()
                    .ok_or("original cold receiving source disappeared")?
                    .source_inputs()?
                    != receiving_inputs
            {
                return Err(
                    "cold original source/constructor changed during actual native preparation"
                        .into(),
                );
            }
            lease.validate_source_assets(&self.instance_ref, owner.source_assets())
        })();
        if let Err(reason) = checked {
            return Err(NativeStoppedExchangeFailure {
                reason: self.session.session_mut().performance_invalidate(&reason),
                native_receipts: receipts,
            });
        }
        // Source-only publication follows the genuine accepted prepare/catalog;
        // the FIELD clock is not advanced or reset. Restore owns numerical q/v.
        self.session
            .session_mut()
            .adopt_physical_source_after_ack(current);
        self.performance = Some(owner);
        self.receiving_source = Some(source);
        Ok(receipts)
    }
    /// SAME completed native readmission pulse, projected as a controller
    /// receipt. Full checkpoint/source originals stay in the native channel;
    /// no second Inspect or invented performance acknowledgement is made.
    pub(in crate::continuous) fn recording_readmission_controller_receipt(
        &self,
        id: &str,
        reply: &super::receiving_readmission::NativeReceivingReadmissionReply,
    ) -> Value {
        let pulse = reply.native_pulse();
        let acknowledgement = &pulse["payload"]["transport_ack"];
        let performance = json!({
            "schema":super::super::performance::REPLY,"operation":"performance-continue-act",
            "accepted":pulse["accepted"],"refusal":null,"reading":pulse["reading"],
            "transport_transition":acknowledgement,"admission":null,
            "native_pulse":{
                "applications":pulse["applications"],"input_history":pulse["input_history"],
                "last_input_ordinal":pulse["last_input_ordinal"],"recording":pulse["recording"],
                "recording_available":pulse["recording_available"],"release_pending":pulse["release_pending"],
                "release_zero_proven":pulse["release_zero_proven"],"release_proof_cursor":pulse["release_proof_cursor"]
            },
            "native_payload":{"transport_ack":acknowledgement}
        });
        self.recording_controller_receipt(id, Some(&performance))
    }
    pub(in crate::continuous) fn has_native_performance(&self) -> bool {
        self.performance.is_some()
    }
}

#[cfg(test)]
#[path = "performance_cold_source_tests.rs"]
mod source_tests;

#[cfg(test)]
mod retained_history_tests {
    use super::*;
    #[test]
    #[ignore = "requires the genuine native Form/M4 worker corpus from the normal fixture gate"]
    fn actual_native_history_keeps_complete_prefixes_and_refuses_a_lost_application() {
        let path = std::env::var("QL_NATIVE_PHYSICAL_SOURCE_REPLAY_ARTIFACT")
            .expect("genuine original native source/Form/M4 corpus required; no fallback");
        let corpus: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            corpus["schema"],
            "ql.actual-native-physical-source-replay-component/v1"
        );
        let frames = corpus["frames"].as_array().unwrap();
        assert_eq!(frames.len(), 3);
        let mut sources = Vec::new();
        for (index, frame) in frames.iter().enumerate() {
            let mut source = json!({"native_bundle":frame["source_assets"]});
            if index != 0 {
                source["native_physical_source_history"] =
                    frame["native_physical_source_history"].clone();
            }
            assert_eq!(source_depth(&source).unwrap(), index);
            assert_eq!(
                physical_applications(&source).unwrap(),
                &corpus["native_physical_source_history"].as_array().unwrap()[..index]
            );
            sources.push(source);
        }
        let mut lost = sources[2].clone();
        lost["native_physical_source_history"]
            .as_array_mut()
            .unwrap()
            .remove(0);
        assert!(source_depth(&lost).is_err());
        let mut omitted = sources[2].clone();
        omitted
            .as_object_mut()
            .unwrap()
            .remove("native_physical_source_history");
        assert!(source_depth(&omitted).is_err());
        let mut wrong_type = sources[2].clone();
        wrong_type["native_physical_source_history"] = json!({});
        assert!(source_depth(&wrong_type).is_err());
        assert_eq!(source_depth(&sources[0]).unwrap(), 0);
        assert_eq!(source_depth(&sources[1]).unwrap(), 1);
        assert_eq!(source_depth(&sources[2]).unwrap(), 2);
    }
}
