use ql_mef::continuous::performance::PerformanceOwner;
use ql_mef::musical_performance_return::{ReturnContext, ReturnReference, bind_performance_return};
use serde_json::json;
#[path = "../tests/support/retained_source_performance.rs"]
mod retained_source_performance;
fn main() -> Result<(), String> {
    let (current, config) = retained_source_performance::config(true);
    let owner = PerformanceOwner::prepare(&current, "expression:retained/current", config)?;
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
    let returned = bind_performance_return(owner.binding(), None, context, 73)?;
    println!("{}",serde_json::to_string(&json!({"schema":"ql.retained-source-performance-fixture/v1","standing":"actual-native-original-source-and-canonical-form-preparation; no-runtime-device-or-installed-verdict","basis":returned.expression_basis()?,"pitches":returned.expression_pitches(0)?,"source_assets":owner.source_assets(),"native_preparation":owner.native_packet()?,"native_basis":owner.binding().native_basis(),"native_catalog":owner.native_catalog(),"pitch_scope":"Return pitches retain every genuinely available source key/register from the SAME privately native B/K binding; unavailable keys contribute no target. Prepared targets create no active voice until actual NoteOn."})).map_err(|e|e.to_string())?);
    Ok(())
}
