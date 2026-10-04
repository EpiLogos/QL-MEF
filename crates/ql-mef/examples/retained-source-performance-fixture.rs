use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use ql_mef::nara_performance_receiving::{
    ContextKind, ReceivingContext, ReceivingPreparation, Reference, prepare_native_receiving,
};
use ql_mef::performance_source_context::{
    NativePublicSourceOwnership, prepare_native_source_context,
};
use serde_json::json;
#[path = "../tests/support/retained_source_performance.rs"]
mod retained_source_performance;
fn main() -> Result<(), String> {
    let (current, config) = retained_source_performance::config(true);
    let mut owner = PerformanceOwner::prepare(&current, "expression:retained/current", config)?;
    let reference = |value: &str| ReturnReference {
        reference: value.into(),
        revision: "1".into(),
    };
    let context = ReturnContext {
        context: reference("controlled:source-performance/world"),
        receiver: reference("controlled:source-performance/receiver"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    };
    let receiving_context = ReceivingContext {
        kind: ContextKind::World,
        context: Reference {
            reference: context.context.reference.clone(),
            revision: context.context.revision.clone(),
        },
        receiver: Reference {
            reference: context.receiver.reference.clone(),
            revision: context.receiver.revision.clone(),
        },
        protected_state: None,
        consent: None,
        original_occasion: None,
        private: false,
    };
    let definition = prepare_native_receiving(ReceivingPreparation {
        prepared: owner.binding(),
        context: receiving_context.clone(),
        identity: None,
        current: None,
        original_occasion: None,
        calibration: None,
    })?;
    let ownership = NativePublicSourceOwnership::reference_source(&current)?;
    let witness = prepare_native_source_context(
        &owner.source_context_basis(&current)?,
        &definition,
        ReceivingPreparation {
            prepared: owner.binding(),
            context: receiving_context,
            identity: None,
            current: None,
            original_occasion: None,
            calibration: None,
        },
        context.clone(),
        Some(&ownership),
    )?;
    owner.admit_source_context(&current, &witness)?;
    let returned = bind_performance_return(owner.binding(), None, context, 73)?;
    println!("{}",serde_json::to_string(&json!({"schema":"ql.retained-source-performance-fixture/v1","standing":"actual-native-original-source-and-canonical-form-preparation; no-runtime-device-or-installed-verdict","basis":returned.expression_basis()?,"pitches":returned.expression_pitches(0)?,"source_assets":owner.source_assets(),"native_preparation":owner.native_packet()?,"native_basis":owner.binding().native_basis(),"native_catalog":owner.native_catalog(),"pitch_scope":"Return pitches retain every genuinely available source key/register from the SAME privately native B/K binding; unavailable keys contribute no target. Prepared targets create no active voice until actual NoteOn."})).map_err(|e|e.to_string())?);
    Ok(())
}
