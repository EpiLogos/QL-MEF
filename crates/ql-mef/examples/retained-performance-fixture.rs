//! Test material from real native producers, not an external project factory.
//! The integration lead executes this under finite compiler admission, then
//! passes this exact stdout to the OI production-operation integration tests.
mod support {
    include!("../tests/support/retained_performance.rs");
}
use ql_mef::musical_performance_return::*;
use ql_mef::performance_audio::prepare_native_performance;
use serde_json::json;
fn reference(v: &str) -> ReturnReference {
    ReturnReference {
        reference: v.into(),
        revision: "1".into(),
    }
}
fn main() -> Result<(), String> {
    let prepared = prepare_native_performance(support::preparation())?;
    let context = ReturnContext {
        context: reference("reference:receiving-context"),
        receiver: reference("reference:receiving-owner"),
        source_occasion: None,
        protected_state: None,
        consent: None,
        kind: "world".into(),
        private: false,
        required_assets: vec![],
    };
    let binding = bind_performance_return(&prepared, None, context, 73)?;
    let out = json!({"schema":"ql.retained-performance-fixture/v1","producer_contract":CONTRACT,
        "basis":binding.expression_basis()?,"pitches":binding.expression_pitches(0)?,"native_return":binding.snapshot()?,
        "native_notes":prepared.notes(),"native_preparation":serde_json::to_value(&prepared).map_err(|e| e.to_string())?});
    println!(
        "{}",
        serde_json::to_string(&out).map_err(|e| e.to_string())?
    );
    Ok(())
}
