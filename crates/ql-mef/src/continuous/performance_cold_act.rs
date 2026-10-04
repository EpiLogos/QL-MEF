//! All historical source bodies compile before the SAME selected checkpoint
//! owner is activated. Full source lineage and original numerical wire remain
//! distinct; retained JSON cannot mint an opening, clock or receiving grant.
use super::*;
use crate::continuous::coupled::CoupledBasis;
use crate::continuous::performance::contact_history::{
    ColdNativeContactCorpus, PreparedColdContactSource, QualifiedContactCheckpoint,
};
use crate::continuous::performance::retained_evidence::{encoded_bound, exact_value};
use crate::continuous::performance::selected_source_budget::{
    NativeSelectedSourceReturnBudget, NativeSelectedSourceReturnContext,
    normalize_derived_field_error,
};
use crate::continuous::performance::{
    NativeStoppedExchangeFailure, PreparedColdPhysicalSource, QualifiedAcousticSourceHistory,
    qualify_saved_acoustic_physical_history,
};
use crate::continuous::performance_act_bridge::NativeActSourceLease;
use crate::musical_performance_source_score::{NativeScorePages, RetainedScoreSource};

enum ColdSourceView {
    Original(PreparedColdPhysicalSource),
    Contact(PreparedColdContactSource),
}
impl ColdSourceView {
    fn physical(&self) -> &PreparedColdPhysicalSource {
        match self {
            Self::Original(v) => v,
            Self::Contact(v) => v.physical(),
        }
    }
    fn into_parts(self) -> (PreparedColdPhysicalSource, Vec<Value>) {
        match self {
            Self::Original(v) => (v, Vec::new()),
            Self::Contact(v) => v.into_parts(),
        }
    }
}
pub(in crate::continuous) struct PreparedColdNativeAct {
    source_views: Vec<ColdSourceView>,
    selected: ColdSourceView,
    contact_checkpoint: Option<QualifiedContactCheckpoint>,
    current_basis: Value,
    receiving_inputs: Value,
    // Only a genuine already attached owner can populate this before-state.
    // The same private candidate serves all-page compilation; it is never a
    // cold activation grant when this original resident is present.
    resident_source: Option<Value>,
}
impl PreparedColdNativeAct {
    /// Original numerical preflight from this SAME closed cold activity. The
    /// complete candidate remains alive; no new worker request or P advance.
    pub(in crate::continuous) fn native_contact_checkpoint_evidence(
        &self,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<Option<Value>, String> {
        match (&self.selected, &self.contact_checkpoint) {
            (ColdSourceView::Original(_), None) => Ok(None),
            (ColdSourceView::Contact(prepared), Some(qualified)) => {
                qualified.validate_prepared(prepared, lease)?;
                Ok(Some(
                    crate::continuous::performance::retained_evidence::contact_checkpoint_evidence(
                        qualified.original_request(),
                        qualified.original_reply(),
                    )?,
                ))
            }
            _ => Err("cold Contact evidence lacks its actual qualified selected source".into()),
        }
    }
}
#[derive(serde::Serialize)]
struct BorrowedResidentOriginals<'a> {
    native_bundle: &'a Value,
    native_preparation: &'a Value,
    native_basis: &'a CoupledBasis,
    reading: &'a Value,
}
fn resident_originals(
    owner: &PerformanceOwner,
    current: &CoupledBasis,
    source: &NativePerformanceReceivingSource,
) -> Result<Value, String> {
    owner.precharge_source_export_inputs(current, source)?;
    encoded_bound(
        owner.source_assets(),
        8 * 1024 * 1024,
        "resident original native source",
    )?;
    encoded_bound(
        owner.binding(),
        8 * 1024 * 1024,
        "resident original native binding",
    )?;
    let packet = owner.native_packet()?;
    // The whole borrowed envelope is charged before any original is copied.
    // Native binding is kept as its actual complete owner object here.
    let original = BorrowedResidentOriginals {
        native_bundle: owner.source_assets(),
        native_preparation: &packet,
        native_basis: owner.binding().native_basis(),
        reading: owner
            .reading()
            .ok_or("resident original native reading absent")?,
    };
    encoded_bound(
        &original,
        MAX_HOST_INPUT as usize,
        "resident full selected-source originals",
    )?;
    serde_json::to_value(original).map_err(|e| e.to_string())
}
fn contact_admissions(asset: &Value) -> Result<&[Value], String> {
    match asset.get("native_contact_admission_history") {
        Some(v) => v
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| "complete native Contact admission corpus has wrong type".into()),
        None => Ok(&[]),
    }
}
fn contact_records(asset: &Value) -> Result<&[Value], String> {
    match asset["native_bundle"].get("contact_occurrence_history") {
        Some(v) => v
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| "complete native Contact source corpus has wrong type".into()),
        None => Ok(&[]),
    }
}
fn source_ordinal(record: &Value) -> Result<u64, String> {
    exact_cursor(
        record["original_native_request_id"]
            .as_str()
            .ok_or("original source Manager ordinal absent")?,
    )
}
fn source_order(asset: &Value) -> Result<Vec<Value>, String> {
    let mut all = physical_applications(asset)?
        .iter()
        .chain(acoustic_applications(asset)?)
        .map(|v| v["source"].clone())
        .chain(contact_records(asset)?.iter().cloned())
        .collect::<Vec<_>>();
    for row in &all {
        source_ordinal(row)?;
    }
    all.sort_by_key(|v| source_ordinal(v).expect("qualified original source ordinal"));
    let mut previous = 0;
    for row in &all {
        let ordinal = source_ordinal(row)?;
        if ordinal <= previous {
            return Err("actual source/contact reused an original Manager operation".into());
        }
        previous = ordinal;
    }
    Ok(all)
}
fn original_prefix(latest: &Value, asset: &Value) -> Result<(), String> {
    let full = source_order(latest)?;
    let prefix = source_order(asset)?;
    if full.get(..prefix.len()).is_none_or(|rows| {
        rows.len() != prefix.len() || rows.iter().zip(&prefix).any(|(a, b)| !exact_value(a, b))
    }) {
        return Err("native Act source lost its full interleaved original prefix".into());
    }
    for (a, b) in [
        (
            physical_applications(asset)?,
            physical_applications(latest)?,
        ),
        (
            acoustic_applications(asset)?,
            acoustic_applications(latest)?,
        ),
        (contact_admissions(asset)?, contact_admissions(latest)?),
    ] {
        if b.get(..a.len()).is_none_or(|rows| {
            rows.len() != a.len() || rows.iter().zip(a).any(|(x, y)| !exact_value(x, y))
        }) {
            return Err("native Act prefix detached its full original source ACKs".into());
        }
    }
    Ok(())
}
fn prepare_source_view(
    current: &CoupledBasis,
    instance: &str,
    source: &NativePerformanceReceivingSource,
    asset: &Value,
    lease: &NativeActSourceLease<'_>,
) -> Result<ColdSourceView, String> {
    if contact_records(asset)?.is_empty() {
        if !contact_admissions(asset)?.is_empty() {
            return Err("contact sidecar introduced an unowned occurrence".into());
        }
        Ok(ColdSourceView::Original(
            PerformanceOwner::prepare_cold_native_acoustic_source(
                current,
                instance,
                source,
                &asset["native_bundle"],
                physical_applications(asset)?,
                acoustic_applications(asset)?,
                lease,
            )?,
        ))
    } else {
        Ok(ColdSourceView::Contact(
            PerformanceOwner::prepare_cold_native_contact_source(
                current,
                instance,
                source,
                &asset["native_bundle"],
                ColdNativeContactCorpus {
                    physical_applications: physical_applications(asset)?,
                    acoustic_applications: acoustic_applications(asset)?,
                    contact_admissions: contact_admissions(asset)?,
                },
                lease,
            )?,
        ))
    }
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
    let bodies = records
        .checked_add(acoustic_records)
        .filter(|v| *v < 256)
        .ok_or("complete native source epoch bound exceeded")?;
    let contacts = contact_records(asset)?;
    let admitted = contact_admissions(asset)?;
    if contacts.len() != admitted.len()
        || contacts.iter().zip(admitted).any(|(record, row)| {
            row.as_object().is_none_or(|v| v.len() != 4)
                || row["schema"] != "ql.native-scene-contact-admission/v1"
                || !exact_value(&row["source"], record)
        })
    {
        return Err("native Contact source lacks its whole original admission corpus".into());
    }
    bodies
        .checked_add(contacts.len())
        .ok_or_else(|| "complete original source depth exhausted".into())
}
impl FieldHost {
    pub(in crate::continuous) fn prepare_cold_native_act(
        &self,
        manifest: &Value,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedColdNativeAct, String> {
        if self.performance.is_some() {
            return Err("cold continuation requires a new available actual source owner".into());
        }
        self.prepare_selected_native_act_sources(manifest, lease, false)
    }
    /// Reconstruct every original selected source from the attached owner's
    /// immutable constructor. The operative late body is retained separately;
    /// it cannot become the origin of an earlier saved performance.
    pub(in crate::continuous) fn prepare_resident_native_act(
        &self,
        manifest: &Value,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<PreparedColdNativeAct, String> {
        if self.performance.is_none() {
            return Err("selected source re-adoption requires its actual resident owner".into());
        }
        self.prepare_selected_native_act_sources(manifest, lease, true)
    }
    fn prepare_selected_native_act_sources(
        &self,
        manifest: &Value,
        lease: &NativeActSourceLease<'_>,
        resident: bool,
    ) -> Result<PreparedColdNativeAct, String> {
        if !self.available() || resident != self.performance.is_some() {
            return Err("selected source owner is unavailable or changed".into());
        }
        encoded_bound(
            manifest,
            MAX_HOST_INPUT as usize,
            "selected full Act source manifest",
        )?;
        encoded_bound(
            self.session.session().current_basis(),
            MAX_HOST_INPUT as usize,
            "selected operative complete basis",
        )?;
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
        let operative = self.session.session().current_basis();
        let resident_source = match self.performance.as_ref() {
            Some(owner) => {
                owner.validate_current(operative)?;
                lease.validate_source_assets(&self.instance_ref, owner.source_assets())?;
                Some(resident_originals(owner, operative, receiving)?)
            }
            None => None,
        };
        let current = self
            .performance
            .as_ref()
            .map(PerformanceOwner::immutable_source_origin)
            .unwrap_or(operative);
        // Every original complete source epoch is independently regenerated
        // under the same privately held lease. Contact-only epochs share a
        // physical body but retain their exact original prefix and source owner.
        let mut source_views = Vec::with_capacity(sources.len());
        for asset in sources {
            original_prefix(latest, asset)?;
            let epoch = receiving_epoch(receiving, asset)?;
            let view = prepare_source_view(current, &self.instance_ref, &epoch, asset, lease)?;
            if !exact_value(
                view.physical().final_frame().owner().source_assets(),
                &asset["native_bundle"],
            ) {
                return Err(
                    "Act original source prefix differs from its actual owner replay".into(),
                );
            }
            source_views.push(view);
        }
        // Retain all full Act source views while separately regenerating the
        // exact selected checkpoint prefix. No after/current label is an origin.
        original_prefix(latest, selected)?;
        let selected_source = receiving_epoch(receiving, selected)?;
        let selected = prepare_source_view(
            current,
            &self.instance_ref,
            &selected_source,
            selected,
            lease,
        )?;
        Ok(PreparedColdNativeAct {
            source_views,
            selected,
            contact_checkpoint: None,
            current_basis: serde_json::to_value(operative).map_err(|e| e.to_string())?,
            receiving_inputs: receiving.source_inputs()?,
            resident_source,
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
        let mut receiving_sources = Vec::with_capacity(assets.len());
        let mut returns = Vec::with_capacity(assets.len());
        for (ordinal, asset) in assets.iter().enumerate() {
            let mut matching = bases
                .iter()
                .filter(|basis| basis["content_digest"] == asset["basis_digest"]);
            let basis = matching
                .next()
                .ok_or("source epoch has no original musical basis")?;
            if matching.next().is_some() {
                return Err("source epoch musical basis ambiguous".into());
            }
            let view = candidate
                .source_views
                .get(ordinal)
                .ok_or("complete native source epoch view absent")?;
            let frame = view.physical().final_frame();
            if !exact_value(frame.owner().source_assets(), &asset["native_bundle"]) {
                return Err("native score source view lost its exact whole contact epoch".into());
            }
            let source = frame.receiving_source(&receiving)?;
            let seed = exact_cursor(
                basis["seed"]
                    .as_str()
                    .ok_or("original native Return seed absent")?,
            )?;
            returns.push(frame.prepare_return(&source, seed)?);
            receiving_sources.push(source);
        }
        let sources = candidate
            .source_views
            .iter()
            .enumerate()
            .map(|(ordinal, view)| {
                let frame = view.physical().final_frame();
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
        &mut self,
        candidate: &mut PreparedColdNativeAct,
        lease: &NativeActSourceLease<'_>,
        checkpoint_ref: &str,
        original_wire: &str,
    ) -> Result<Option<QualifiedAcousticSourceHistory>, NativeStoppedExchangeFailure> {
        let family = self.cold_act_source(candidate)?;
        let source = candidate
            .selected
            .physical()
            .final_frame()
            .receiving_source(&family)?;
        lease.validate_selected_checkpoint(&self.instance_ref, checkpoint_ref, original_wire)?;
        let saved: Value = serde_json::from_str(original_wire).map_err(|e| e.to_string())?;
        if let ColdSourceView::Contact(prepared) = &candidate.selected {
            candidate.contact_checkpoint = Some(prepared.qualify_original_native_checkpoint(
                self.session.session_mut(),
                &self.instance_ref,
                checkpoint_ref,
                original_wire,
                lease,
            )?);
        } else if saved["native_pair"]["audio"]["version"] == 3 {
            return Err(
                "contact checkpoint selected a source with no original occurrences"
                    .to_owned()
                    .into(),
            );
        }
        if saved["native_pair"]["audio"]["receiving"]["schema"]
            != "ql.performance-receiving-checkpoint/v2"
        {
            return Ok(None);
        }
        let history = qualify_saved_acoustic_physical_history(
            candidate.selected.physical(),
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
        let same_owner = match (&candidate.resident_source, self.performance.as_ref()) {
            (None, None) => true,
            (Some(original), Some(owner)) => exact_value(
                original,
                &resident_originals(
                    owner,
                    self.session.session().current_basis(),
                    self.receiving_source
                        .as_ref()
                        .ok_or("original resident receiving source absent")?,
                )?,
            ),
            _ => false,
        };
        if !same_owner
            || !self.available()
            || !exact_value(
                &serde_json::to_value(self.session.session().current_basis())
                    .map_err(|e| e.to_string())?,
                &candidate.current_basis,
            )
        {
            return Err("cold actual Field source changed during complete Act compilation".into());
        }
        let source = self
            .receiving_source
            .as_ref()
            .ok_or("cold original receiving owner disappeared")?
            .clone();
        if !exact_value(&source.source_inputs()?, &candidate.receiving_inputs) {
            return Err("cold original receiving source/context/episode changed".into());
        }
        Ok(source)
    }
    pub(in crate::continuous) fn activate_cold_native_act(
        &mut self,
        candidate: PreparedColdNativeAct,
        lease: &NativeActSourceLease<'_>,
    ) -> Result<Vec<Value>, NativeStoppedExchangeFailure> {
        if candidate.resident_source.is_some() || self.performance.is_some() {
            return Err(
                "an attached selected source requires atomic same-worker re-adoption".into(),
            );
        }
        let family = self.cold_act_source(&candidate)?;
        let source = candidate
            .selected
            .physical()
            .final_frame()
            .receiving_source(&family)?;
        if let ColdSourceView::Contact(prepared) = &candidate.selected {
            candidate
                .contact_checkpoint
                .as_ref()
                .ok_or(
                    "Contact cold activation lacks original native numeric qualification"
                        .to_owned(),
                )?
                .validate_prepared(prepared, lease)?;
        }
        let current_basis = candidate.current_basis;
        let receiving_inputs = candidate.receiving_inputs;
        // Full replay stayed alive through all-page compilation and dated
        // checkpoint qualification. Numerical activation uses the selected body.
        let (selected, contact_admissions) = candidate.selected.into_parts();
        let (mut owner, current) = selected.into_final();
        owner.restore_qualified_contact_admission_history(contact_admissions, lease)?;
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
    /// Earlier-prefix continuation on the SAME worker/Manager/Engine. Every
    /// original source view, selected Contact preflight and dated history stays
    /// alive until the actual complete native acknowledgement is qualified.
    pub(in crate::continuous) fn readopt_resident_native_act_checkpoint(
        &mut self,
        mut candidate: PreparedColdNativeAct,
        lease: &NativeActSourceLease<'_>,
        selection: NativeActCheckpointReadoption<'_>,
        history: Option<&QualifiedAcousticSourceHistory>,
        return_context: &NativeSelectedSourceReturnContext<'_>,
        retain_original: &mut impl FnMut(&str, &Value) -> Result<(), String>,
    ) -> Result<
        super::receiving_readmission::NativeReceivingReadmissionReply,
        NativeStoppedExchangeFailure,
    > {
        let NativeActCheckpointReadoption {
            checkpoint_ref,
            original_wire,
            transaction_ref,
        } = selection;
        return_context.charge_known()?;
        if candidate.resident_source.is_none() {
            return Err("selected native source re-adoption requires its retained resident".into());
        }
        let family = self.cold_act_source(&candidate)?;
        let selected_source = candidate
            .selected
            .physical()
            .final_frame()
            .receiving_source(&family)?;
        if let ColdSourceView::Contact(prepared) = &candidate.selected {
            candidate
                .contact_checkpoint
                .as_ref()
                .ok_or("selected Contact source lacks its actual original numerical preflight")?
                .validate_prepared(prepared, lease)?;
        }
        lease.validate_selected_checkpoint(&self.instance_ref, checkpoint_ref, original_wire)?;
        let selected_frame = candidate.selected.physical().final_frame();
        selected_frame.owner().precharge_selected_source_packets(
            selected_frame.current(),
            &selected_source,
            self.performance
                .as_ref()
                .ok_or("actual resident source absent before packet precharge")?,
            self.session.session().current_basis(),
            &family,
        )?;
        // Capturing the stopped BEFORE checkpoint is an actual existing owner
        // operation. It advances no P/audio cursor and retains its full pulse.
        // Preserve the earlier complete guard separately. A genuine checkpoint
        // observer may drain unread feedback, changing its copied reading while
        // leaving every complete source determinant unchanged.
        let resident_before_capture_guard = candidate
            .resident_source
            .take()
            .ok_or("original resident source guard absent before native checkpoint capture")?;
        let before_current = self.session.session().current_basis().clone();
        let before_pulse = self
            .performance
            .as_mut()
            .ok_or("selected native resident disappeared before checkpoint capture")?
            .owner_stopped_exchange(
                &before_current,
                &family,
                self.session.session_mut(),
                "checkpoint",
                &json!({}),
            )?;
        let mut originals = vec![before_pulse];
        let result = (|| -> Result<_, NativeStoppedExchangeFailure> {
            if originals[0]["accepted"] != true {
                return Err("actual resident original checkpoint capture refused".into());
            }
            // This SAME pulse drained callback/audio/input/application queues.
            // The native C owner must retain its original file and ACK before
            // source mutation; checkpoint text alone cannot preserve it.
            retain_original("before_restoration_receipt", &originals[0])?;
            let actual_before_source = resident_originals(
                self.performance
                    .as_ref()
                    .ok_or("actual resident source disappeared after checkpoint capture")?,
                self.session.session().current_basis(),
                &family,
            )?;
            if ["native_bundle", "native_preparation", "native_basis"]
                .iter()
                .any(|key| {
                    !exact_value(
                        &actual_before_source[*key],
                        &resident_before_capture_guard[*key],
                    )
                })
                || !exact_value(&actual_before_source["reading"], &originals[0]["reading"])
                || originals[0]["operation"] != "checkpoint"
                || [
                    "session_ref",
                    "transport_epoch",
                    "samples_elapsed",
                    "accepted_sequence",
                    "scope",
                ]
                .iter()
                .any(|key| {
                    !exact_value(
                        &actual_before_source["reading"][*key],
                        &resident_before_capture_guard["reading"][*key],
                    )
                })
            {
                return Err("actual checkpoint feedback changed the complete resident source or stopped source/cursor guard".into());
            }
            // This exact actual reading is now the resident guard. No field is
            // reconstructed or relabelled with the selected body's identity.
            candidate.resident_source = Some(actual_before_source);
            let original_before_wire = originals[0]["payload"]["checkpoint_after_pulse_wire"]
                .as_str()
                .ok_or("actual native feedback-complete BEFORE checkpoint text absent")?;
            encoded_bound(
                original_before_wire,
                4 * 1024 * 1024,
                "escaped original feedback-complete BEFORE checkpoint",
            )?;
            let original_before_wire = original_before_wire.to_owned();
            self.cold_act_source(&candidate)?;
            let PreparedColdNativeAct {
                source_views,
                selected,
                contact_checkpoint,
                current_basis,
                receiving_inputs,
                resident_source,
            } = candidate;
            // Retain ALL original epoch and private Contact evidence lifetimes
            // across the one native exchange, including post-reply refusal.
            let _source_views = source_views;
            let _contact_checkpoint = contact_checkpoint;
            let (selected, contact_admissions) = selected.into_parts();
            let (mut owner, current) = selected.into_final();
            owner.restore_qualified_contact_admission_history(contact_admissions, lease)?;
            let request = owner.prepare_selected_source_readoption_request(
                &current,
                &selected_source,
                self.performance
                    .as_ref()
                    .ok_or("original resident absent")?,
                self.session.session().current_basis(),
                &original_before_wire,
                original_wire,
                lease,
                checkpoint_ref,
                transaction_ref,
                history,
            )?;
            let validate_resident = |host: &FieldHost| -> Result<(), String> {
                if !host.available()
                    || !exact_value(
                        &serde_json::to_value(host.session.session().current_basis())
                            .map_err(|e| e.to_string())?,
                        &current_basis,
                    )
                    || !exact_value(
                        &host
                            .receiving_source
                            .as_ref()
                            .ok_or("original receiving source disappeared")?
                            .source_inputs()?,
                        &receiving_inputs,
                    )
                    || !exact_value(
                        &resident_originals(
                            host.performance
                                .as_ref()
                                .ok_or("original native resident disappeared")?,
                            host.session.session().current_basis(),
                            host.receiving_source
                                .as_ref()
                                .ok_or("original receiving source disappeared")?,
                        )?,
                        resident_source
                            .as_ref()
                            .ok_or("original resident source evidence absent")?,
                    )
                {
                    return Err(
                        "original native source changed during selected-source transaction".into(),
                    );
                }
                lease.validate_selected_checkpoint(
                    &host.instance_ref,
                    checkpoint_ref,
                    original_wire,
                )?;
                lease.validate_source_assets(&host.instance_ref, owner.source_assets())
            };
            request.validate_selected(&owner, &current, &selected_source, lease)?;
            validate_resident(self)?;
            let return_budget = NativeSelectedSourceReturnBudget::prepare(
                return_context,
                &self.instance_ref,
                &self.last_request.to_string(),
                self.session.session().last_field(),
                resident_source
                    .as_ref()
                    .ok_or("actual resident return source absent")?,
                &originals[0],
                request.request(),
            )?;
            // The exact privately produced request is durably retained/ACKed on
            // the existing diagnostic channel BEFORE its only worker write.
            // A transport loss cannot discard it or cause an implicit resend.
            retain_original("source_readoption_original_request", request.request())?;
            // The private source schema is accepted only through this retained
            // typed caller and the existing worker dispatcher. No new worker.
            let pulse = self
                .session
                .session_mut()
                .selected_source_exchange_retained(request.request())?;
            let checked = (|| -> Result<(), String> {
                return_budget.validate_original(&pulse)?;
                request.qualify_reply(&pulse)?;
                validate_resident(self)?;
                request.validate_selected(&owner, &current, &selected_source, lease)
            })();
            if let Err(reason) = checked {
                return Err(NativeStoppedExchangeFailure {
                    reason: self.session.session_mut().performance_invalidate(&reason),
                    native_receipts: vec![pulse],
                });
            }
            // Source publication follows the exact actual native ACK. The
            // old Rust owner remains attached until all complete originals,
            // source/episode producers and operative/AFTER checkpoints agree.
            let returned = pulse.clone();
            owner
                .retain_selected_source_readoption_pulse(pulse)
                .map_err(|(reason, pulse)| NativeStoppedExchangeFailure {
                    reason: self.session.session_mut().performance_invalidate(&reason),
                    native_receipts: vec![pulse],
                })?;
            self.session
                .session_mut()
                .adopt_physical_source_after_ack(current);
            self.performance = Some(owner);
            self.receiving_source = Some(selected_source);
            Ok(super::receiving_readmission::NativeReceivingReadmissionReply::from_source_readoption(returned)
                .with_source_readoption_before_source(resident_source
                    .ok_or("original resident source evidence disappeared after native acknowledgement")?))
        })();
        match result {
            Ok(reply) => Ok(reply.with_original_pulses(originals)),
            Err(mut failure) => {
                normalize_derived_field_error(&mut failure);
                originals.append(&mut failure.native_receipts);
                failure.native_receipts = originals;
                Err(failure)
            }
        }
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
                "native_capture":pulse["native_capture"],
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
