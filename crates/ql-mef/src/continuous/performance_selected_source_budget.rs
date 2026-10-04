//! Complete final selected-source response accounting on the stopped control
//! thread. Counts and diagnostic provenance cannot grant source/Act admission.
use super::retained_evidence::encoded_bound;
use super::{CoupledBasis, NativeStoppedExchangeFailure, PerformanceOwner};
use crate::continuous::performance_receiving::NativePerformanceReceivingSource;
use serde::Serialize;
use serde_json::Value;
use std::io::{self, Write};

const SIDECAR: usize = 4 * 1024 * 1024;
const NATIVE: usize = crate::continuous::MAX_MESSAGE;
const OUTER: usize = crate::continuous::host::MAX_HOST_OUTPUT;
// These are the EXISTING native Channel::normal_pulse_bound and the native
// source_readoption wrapper/ACK/descriptor reservations, not raised caps.
const NORMAL_PULSE: usize = SIDECAR;
const WRAPPERS: usize = 64 * 1024;
const REASON_AND_DIAGNOSTICS: usize = 512 * 1024;

/// Borrowed solely from the already qualified Bridge activity. No Clone,
/// Deserialize or serialized constructor; none of these Values is a lease.
pub(crate) struct NativeSelectedSourceReturnContext<'a> {
    score: &'a Value,
    selection: &'a Value,
    trigger: &'a Value,
    retained_source_selection: &'a Value,
    contact_evidence: &'a Option<Value>,
    diagnostics: &'a Value,
    request_id: &'a str,
}
impl<'a> NativeSelectedSourceReturnContext<'a> {
    pub(crate) fn new(
        score: &'a Value,
        selection: &'a Value,
        trigger: &'a Value,
        retained_source_selection: &'a Value,
        contact_evidence: &'a Option<Value>,
        diagnostics: &'a Value,
        request_id: &'a str,
    ) -> Self {
        Self {
            score,
            selection,
            trigger,
            retained_source_selection,
            contact_evidence,
            diagnostics,
            request_id,
        }
    }
    /// Even a refused later preparation must not clone uncharged Bridge data.
    pub(crate) fn charge_known(&self) -> Result<(), String> {
        encoded_bound(
            &(
                self.score,
                self.selection,
                self.trigger,
                self.retained_source_selection,
                self.contact_evidence,
                self.diagnostics,
                self.request_id,
            ),
            OUTER,
            "selected-source complete borrowed Bridge originals",
        )?;
        Ok(())
    }
}

#[derive(Serialize)]
struct ControllerBase<'a> {
    schema: &'static str,
    instance_ref: &'a str,
    request_id: &'a str,
    last_request_id: &'a str,
    status: &'static str,
    available: bool,
    error: Option<&'static str>,
    // The real response removes audio. Charging the complete original Field
    // here overcounts that projection without copying or changing the Field.
    field: &'a Value,
    standing: &'static str,
}
#[derive(Serialize)]
struct Success<'a> {
    score: &'a Value,
    selection: &'a Value,
    receiving_readmission: Option<&'a Value>,
    native_source_readoption: Option<&'a Value>,
    native_source_readoption_before_source: &'a Value,
    native_pulse: Option<&'a Value>,
    readmitted: bool,
    error: Option<&'static str>,
    source_requalification_trigger: &'a Value,
    retained_source_selection: &'a Value,
    native_contact_checkpoint_evidence: &'a Option<Value>,
    host_receipt: &'a ControllerBase<'a>,
}
#[derive(Serialize)]
struct Refusal<'a> {
    score: &'a Value,
    selection: &'a Value,
    receiving_readmission: Option<&'a Value>,
    native_source_readoption: Option<&'a Value>,
    native_pulse: Option<&'a Value>,
    readmitted: bool,
    error: &'static str,
    source_requalification_trigger: &'a Value,
    retained_source_selection: &'a Value,
    native_contact_checkpoint_evidence: &'a Option<Value>,
    // One authentic BEFORE pulse is known now. One future native reply is
    // separately reserved; transport loss may supply none, never a replacement.
    cold_preparation_receipts: [&'a Value; 1],
    host_receipt: &'a ControllerBase<'a>,
}
#[derive(Serialize)]
struct Envelope<'a, T: Serialize> {
    schema: &'static str,
    instance_ref: &'a str,
    request_id: &'a str,
    last_request_id: &'a str,
    available: bool,
    status: &'static str,
    result: T,
    error: Option<&'static str>,
    // The existing Bridge emits H inside result AND at this outer position.
    host_receipt: &'a ControllerBase<'a>,
    diagnostics: &'a Value,
}
#[derive(Serialize)]
struct SourceReadoption<'a> {
    schema: &'static str,
    before_checkpoint_wire: &'a str,
    original_checkpoint_wire: &'a str,
    pre_pulse_checkpoint_wire: &'static str,
    operative_checkpoint_wire: &'static str,
    after_checkpoint_wire: &'static str,
    current_source_packet: &'a Value,
    actual_native_basis: &'a Value,
    current_receiving: &'a Value,
    transport_ack: Option<&'a Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_saved_acoustic: Option<&'a Value>,
}

fn add(a: usize, b: usize) -> Result<usize, String> {
    a.checked_add(b)
        .ok_or_else(|| "selected-source whole response size overflow".into())
}
fn text<'a>(request: &'a Value, key: &str) -> Result<&'a str, String> {
    request[key]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("actual selected-source {key} text absent"))
}

/// A checked size reservation, never a native source witness. The known
/// original envelope and all future native projections were charged BEFORE
/// the only source exchange. One complete raw refusal remains reserved.
pub(crate) struct NativeSelectedSourceReturnBudget {
    success_pulse: usize,
    source_readoption: usize,
}
impl NativeSelectedSourceReturnBudget {
    pub(crate) fn prepare(
        context: &NativeSelectedSourceReturnContext<'_>,
        instance: &str,
        last_request_id: &str,
        original_field: &Value,
        old_source: &Value,
        before_pulse: &Value,
        request: &Value,
    ) -> Result<Self, String> {
        context.charge_known()?;
        encoded_bound(
            request,
            NATIVE,
            "full original selected-source worker request",
        )?;
        encoded_bound(old_source, NATIVE, "full original selected-source resident")?;
        encoded_bound(
            &old_source["native_bundle"],
            8 * 1024 * 1024,
            "selected-source original bundle",
        )?;
        encoded_bound(before_pulse, NATIVE, "actual selected-source BEFORE pulse")?;
        let before_wire = text(request, "original_before_checkpoint_wire")?;
        let saved_wire = text(request, "original_checkpoint_wire")?;
        // Charge escaped original strings individually, exactly as the native
        // sidecar guard does. Do not charge their unescaped .len() instead.
        encoded_bound(before_wire, SIDECAR, "escaped original BEFORE checkpoint")?;
        encoded_bound(saved_wire, SIDECAR, "escaped original saved checkpoint")?;
        let source = SourceReadoption {
            schema: "ql.native-selected-source-readoption/v1",
            before_checkpoint_wire: before_wire,
            original_checkpoint_wire: saved_wire,
            pre_pulse_checkpoint_wire: "",
            operative_checkpoint_wire: "",
            after_checkpoint_wire: "",
            current_source_packet: &request["current_source_packet"],
            actual_native_basis: &request["actual_native_basis"],
            current_receiving: &request["current_receiving"],
            transport_ack: None,
            original_saved_acoustic: request.get("original_saved_acoustic"),
        };
        let mut source_readoption = encoded_bound(
            &source,
            NATIVE,
            "complete prospective selected-source projection",
        )?;
        // R10's real numerical preview is guarded at4MiB before commit.
        // It supplies THREE full CP strings: pre-pulse, operative and after.
        // Its established same-pulse flag/counter growth allowance is64 bytes
        // for EACH post-pulse copy. All source/trajectory/ring data remain whole.
        for bytes in [SIDECAR, SIDECAR + 64, SIDECAR + 64, WRAPPERS] {
            source_readoption = add(source_readoption, bytes)?;
        }
        source_readoption = source_readoption.min(NATIVE);
        let physical = encoded_bound(
            &request["packet"]["physical_body"],
            8 * 1024 * 1024,
            "actual selected physical descriptor",
        )?;
        // Native normal_pulse_bound already enforces≤4MiB for the complete
        // normal reading, original journals, device/catalog and fixed native
        // observations. Capture was drained by BEFORE; no callback adds one.
        // The descriptor physical body is copied once beside S, with at most
        // 96 real mode frequencies and the original64KiB wrapper allowance.
        let success_pulse = add(
            add(add(source_readoption, NORMAL_PULSE)?, physical)?,
            WRAPPERS,
        )?
        .min(NATIVE);
        let base = ControllerBase {
            schema: crate::continuous::host::HOST_RECEIPT,
            instance_ref: instance,
            request_id: context.request_id,
            last_request_id,
            status: "unavailable",
            available: false,
            error: Some(""),
            field: original_field,
            standing: "local single-owner control; caller-supplied pipe authority; last acknowledged native state",
        };
        let success = Envelope {
            schema: "ql.native-act-owner-result/v1",
            instance_ref: instance,
            request_id: context.request_id,
            last_request_id,
            available: false,
            status: "refused",
            error: None,
            host_receipt: &base,
            diagnostics: context.diagnostics,
            result: Success {
                score: context.score,
                selection: context.selection,
                receiving_readmission: None,
                native_source_readoption: None,
                native_source_readoption_before_source: old_source,
                native_pulse: None,
                readmitted: false,
                error: None,
                source_requalification_trigger: context.trigger,
                retained_source_selection: context.retained_source_selection,
                native_contact_checkpoint_evidence: context.contact_evidence,
                host_receipt: &base,
            },
        };
        let known_success = encoded_bound(
            &success,
            OUTER,
            "literal borrowed selected-source success envelope",
        )?;
        let mut total = add(known_success, success_pulse)?;
        total = add(total, source_readoption)?; // S is ALSO emitted outside P.
        for _ in 0..2 {
            // TWO H performance projections, beside their charged Field.
            total = add(total, add(NORMAL_PULSE, WRAPPERS)?)?;
        }
        total = add(total, REASON_AND_DIAGNOSTICS)?;
        if total > OUTER {
            return Err(
                "complete selected-source success response exceeds64MiB before native exchange"
                    .into(),
            );
        }
        let refusal = Envelope {
            schema: "ql.native-act-owner-result/v1",
            instance_ref: instance,
            request_id: context.request_id,
            last_request_id,
            available: false,
            status: "refused",
            error: None,
            host_receipt: &base,
            diagnostics: context.diagnostics,
            result: Refusal {
                score: context.score,
                selection: context.selection,
                receiving_readmission: None,
                native_source_readoption: None,
                native_pulse: None,
                readmitted: false,
                error: "",
                source_requalification_trigger: context.trigger,
                retained_source_selection: context.retained_source_selection,
                native_contact_checkpoint_evidence: context.contact_evidence,
                cold_preparation_receipts: [before_pulse],
                host_receipt: &base,
            },
        };
        let known_refusal = encoded_bound(
            &refusal,
            OUTER,
            "literal borrowed selected-source refusal envelope",
        )?;
        if add(add(known_refusal, NATIVE)?, REASON_AND_DIAGNOSTICS)? > OUTER {
            return Err(
                "complete selected-source refusal response exceeds64MiB before native exchange"
                    .into(),
            );
        }
        Ok(Self {
            success_pulse,
            source_readoption,
        })
    }
    /// Check real returned originals before ANY publication/projection clone.
    /// A violated bound is retained as a genuine postcommit failure, not trimmed.
    pub(crate) fn validate_original(&self, pulse: &Value) -> Result<(), String> {
        encoded_bound(
            pulse,
            self.success_pulse,
            "actual selected-source same pulse",
        )?;
        encoded_bound(
            &pulse["payload"]["source_readoption"],
            self.source_readoption,
            "actual selected-source projection",
        )?;
        Ok(())
    }
}

impl PerformanceOwner {
    /// Count exactly the sparse admission added by packet(), before it copies
    /// preparation or per-key source collections. The twelve-key preparation
    /// appears ONCE. Every available address contributes its own selected-key
    /// receipt, not another whole twelve-key preparation.
    pub(crate) fn sparse_packet_extra_bound(&self) -> Result<usize, String> {
        let Some(source) = &self.sparse else {
            return Ok(0);
        };
        source.validate_current(self.binding(), source.targets().collection())?;
        let preparation = &self.source_assets()["source_key_preparation"];
        let mut bytes = encoded_bound(
            preparation,
            8 * 1024 * 1024,
            "actual sparse twelve-key preparation",
        )?;
        let keys = preparation["key_availability"]
            .as_array()
            .filter(|keys| keys.len() == 12)
            .ok_or("actual sparse twelve-key preparation receipts absent")?;
        // This bounded, stack-owned address set follows the exact producer's
        // unique (key,register) selection without constructing native notes or
        // copying any receipt. SOURCE_REGISTER_MIN/MAX are -32/+32.
        let mut seen = [[false; 65]; 12];
        let mut touches = 0usize;
        for cell in &self.cells {
            if cell["available"] != true {
                continue;
            }
            let key = cell["key"]
                .as_u64()
                .filter(|key| *key < 12)
                .ok_or("actual sparse available key absent")? as usize;
            let register = cell["register_octave"]
                .as_i64()
                .filter(|register| (-32..=32).contains(register))
                .ok_or("actual sparse available register outside native source bound")?;
            let register_slot = (register + 32) as usize;
            if seen[key][register_slot] {
                continue;
            }
            seen[key][register_slot] = true;
            touches += 1;
            if touches > crate::performance_audio::MAX_TOUCHES {
                return Err("actual sparse available touches exceed96 native capacity".into());
            }
            let receipt = &keys[key];
            if receipt["key"].as_u64() != Some(key as u64) || receipt["available"] != true {
                return Err("actual sparse selected key lost its full native receipt".into());
            }
            bytes = add(
                bytes,
                encoded_bound(
                    receipt,
                    8 * 1024 * 1024,
                    "actual sparse selected-key receipt",
                )?,
            )?;
            // SparseNoteTarget::receipt only changes register, finite hertz,
            // applied octave ratio, two exact-ratio copies and touch_ref from
            // this key's native register0 receipt. All six exact integers are
            // u64 (≤20 digits each); three JSON ratio wrappers, a finite f64
            // (≤32 bytes), register (-32..32) and the private ASCII prepared-key
            // reference (<64 bytes) together grow by <512 encoded bytes. No
            // source collection/entry/provenance/identity field changes here.
            bytes = add(bytes, 512)?;
        }
        if touches == 0 {
            return Err("actual sparse available source target catalog is empty".into());
        }
        // Literal source_key_admission keys, the schema/two standing strings,
        // array separators and insertion punctuation are fixed and <1024.
        add(bytes, 1024)
    }

    pub(crate) fn precharge_selected_source_packets(
        &self,
        current: &CoupledBasis,
        source: &NativePerformanceReceivingSource,
        before: &PerformanceOwner,
        before_current: &CoupledBasis,
        before_source: &NativePerformanceReceivingSource,
    ) -> Result<(), String> {
        self.validate_current(current)?;
        before.validate_current(before_current)?;
        if self.config.transpose != before.config.transpose {
            return Err("selected-source transpose differs from the actual resident; native readoption has no transpose operand".into());
        }
        // The shared export producer uses sparse_packet_extra_bound above:
        // one whole preparation and the actual per-key receipt multiplicity.
        self.precharge_source_export_inputs(current, source)?;
        before.precharge_source_export_inputs(before_current, before_source)
    }
}

struct EqualSerialized<'a> {
    original: &'a [u8],
    offset: usize,
}
impl Write for EqualSerialized<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let end = self
            .offset
            .checked_add(bytes.len())
            .ok_or_else(|| io::Error::other("native error comparison overflow"))?;
        if self.original.get(self.offset..end) != Some(bytes) {
            return Err(io::Error::other(
                "native reason is not the whole original JSON",
            ));
        }
        self.offset = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
/// Normalize ONLY Worker::exchange_contract_retained's derived Value.to_string
/// reason, comparing streamed bytes without allocating another whole JSON.
/// The complete original field-error and every preceding native pulse stay put.
pub(crate) fn normalize_derived_field_error(failure: &mut NativeStoppedExchangeFailure) {
    for original in &failure.native_receipts {
        if original["schema"] != "ql.field-error/v1" {
            continue;
        }
        let mut comparison = EqualSerialized {
            original: failure.reason.as_bytes(),
            offset: 0,
        };
        if serde_json::to_writer(&mut comparison, original).is_ok()
            && comparison.offset == comparison.original.len()
        {
            failure.reason = "actual selected-source operation refused; complete original field-error is retained".into();
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::native_source_support;
    use super::super::retained_evidence::exact_value;
    use super::*;
    use serde_json::json;

    fn admission_bound(owner: &PerformanceOwner) -> usize {
        owner.sparse_packet_extra_bound().unwrap()
    }
    fn repeat_packet(owner: &PerformanceOwner) {
        let first = owner.native_packet().unwrap();
        let second = owner.native_packet().unwrap();
        assert!(exact_value(&first, &second));
        let extra = admission_bound(owner);
        let admission = first.get("source_key_admission").unwrap();
        assert!(encoded_bound(admission, extra, "real sparse admission").is_ok());
        let whole = add(
            encoded_bound(owner.binding(), NATIVE, "real native binding").unwrap(),
            extra,
        )
        .unwrap();
        assert!(encoded_bound(&first, whole, "real complete sparse packet").is_ok());
    }

    #[test]
    fn original_native_world_sparse_packet_fits_before_export_copy() {
        // The immutable original703 corpus is a numerical counterexample to
        //97 copies of the WHOLE preparation. Re-enter the real native World
        // and source/body constructors; the fixture issues no Act/Scene lease.
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/native-receiving-703/world.source-performance.json"
        ))
        .unwrap();
        let assets = &fixture["source_assets"];
        let input: crate::continuous::coupled::CoupledInput =
            serde_json::from_value(assets["original_native_input"].clone()).unwrap();
        let current = input.compose().unwrap();
        let inputs = &assets["receiving_source_inputs"];
        let source = NativePerformanceReceivingSource::world_source(
            serde_json::from_value(inputs["world_request"].clone()).unwrap(),
            serde_json::from_value(inputs["return_context"].clone()).unwrap(),
        )
        .unwrap();
        let instance = fixture["native_preparation"]["determination"]["identity"]["instance"]
            .as_str()
            .unwrap();
        let owner =
            PerformanceOwner::prepare_cold_act_source(&current, instance, &source, assets).unwrap();
        let original = owner.source_assets().clone();
        let preparation = encoded_bound(
            &assets["source_key_preparation"],
            NATIVE,
            "original preparation",
        )
        .unwrap();
        assert!(
            preparation
                .checked_mul(crate::performance_audio::MAX_TOUCHES + 1)
                .unwrap()
                > NATIVE
        );
        owner
            .precharge_source_export_inputs(&current, &source)
            .unwrap();
        repeat_packet(&owner);
        assert!(exact_value(
            &owner.native_packet().unwrap(),
            &fixture["native_preparation"]
        ));
        assert!(exact_value(owner.source_assets(), &original));
        let mut changed_config: super::super::PerformanceConfig =
            serde_json::from_value(assets["configuration"].clone()).unwrap();
        // A real independently prepared transposed source has no operand in
        // the selected native transaction. Reject before packet construction;
        // do not relabel the existing native keyboard after restoring a body.
        changed_config.transpose = (owner.config.transpose + 1) % 12;
        let changed = PerformanceOwner::prepare(&current, instance, changed_config).unwrap();
        let refusal = match changed
            .precharge_selected_source_packets(&current, &source, &owner, &current, &source)
        {
            Ok(()) => panic!("actual changed transpose was admitted without a native operand"),
            Err(refusal) => refusal,
        };
        assert!(refusal.contains("transpose"));
        owner
            .precharge_source_export_inputs(&current, &source)
            .unwrap();
        repeat_packet(&owner);
    }

    #[test]
    fn full_actual_twelve_key_receipts_and96_native_addresses_are_counted() {
        let (current, mut config) = native_source_support::config(true);
        config.columns = 32;
        config.base_register = -4;
        config.fundamental_hertz = 110.0;
        config.use_native_m1_harmonic_ratio = false;
        // Actual typed source assignments may repeat an existing degree. All
        //12 canonical keys now have an explicit source reduction; no filler.
        let reduction = &mut config.sparse_condition.as_mut().unwrap().reduction;
        reduction.assignments = (0..12u8)
            .map(|key| crate::source_key_determination::KeyDegreeAssignment {
                key,
                source_degree: u16::from(key % 7),
                octave: 0,
            })
            .collect();
        let mut owner =
            PerformanceOwner::prepare(&current, "actual:source-budget/96", config).unwrap();
        let source = owner.sparse.as_ref().unwrap();
        // The SAME actual source catalog emits the next32 distinct addresses.
        // This tests the native packet's96-target ceiling numerically; it is
        // not an imported Scene definition or permission to enqueue a gesture.
        let extra_cells = source.catalog(16, 2, 0).unwrap();
        let over_cells = source.catalog(17, 2, 0).unwrap();
        owner.cells.extend(
            extra_cells
                .into_iter()
                .map(|cell| serde_json::to_value(cell).unwrap()),
        );
        assert_eq!(
            owner.available_source_preparation_touches().unwrap().len(),
            96
        );
        assert_eq!(
            owner.source_assets()["source_key_preparation"]["key_availability"]
                .as_array()
                .unwrap()
                .len(),
            12
        );
        repeat_packet(&owner);
        let full_cells = owner.cells.clone();
        let actual_full = owner.native_packet().unwrap();
        owner.cells.extend(
            over_cells
                .into_iter()
                .map(|cell| serde_json::to_value(cell).unwrap()),
        );
        assert!(owner.sparse_packet_extra_bound().is_err());
        assert!(owner.native_packet().is_err());
        owner.cells = full_cells.clone();
        repeat_packet(&owner);
        assert!(exact_value(&owner.native_packet().unwrap(), &actual_full));
        owner.cells.clear();
        assert!(owner.sparse_packet_extra_bound().is_err());
        assert!(owner.native_packet().is_err());
        owner.cells = full_cells;
        repeat_packet(&owner);
        assert!(exact_value(&owner.native_packet().unwrap(), &actual_full));
        let original = owner.source_assets.clone();
        owner.source_assets["source_key_preparation"]["key_availability"][0]["key"] = json!(12);
        assert!(owner.sparse_packet_extra_bound().is_err());
        owner.source_assets = original;
        repeat_packet(&owner);
        assert!(exact_value(&owner.native_packet().unwrap(), &actual_full));
    }

    #[test]
    fn scalar_native_packet_has_no_sparse_copy_and_empty_source_stays_refused() {
        let (current, config) = native_source_support::config(false);
        let owner =
            PerformanceOwner::prepare(&current, "actual:source-budget/scalar", config).unwrap();
        assert_eq!(owner.sparse_packet_extra_bound().unwrap(), 0);
        let first = owner.native_packet().unwrap();
        assert!(first.get("source_key_admission").is_none());
        assert!(exact_value(
            &first,
            &serde_json::to_value(owner.binding()).unwrap()
        ));
        assert!(exact_value(&first, &owner.native_packet().unwrap()));
        let (current, mut config) = native_source_support::config(true);
        config
            .sparse_condition
            .as_mut()
            .unwrap()
            .reduction
            .assignments
            .clear();
        assert!(PerformanceOwner::prepare(&current, "actual:source-budget/empty", config).is_err());
        let (current, config) = native_source_support::config(true);
        let original =
            PerformanceOwner::prepare(&current, "actual:source-budget/repeated", config).unwrap();
        repeat_packet(&original);
    }
}
