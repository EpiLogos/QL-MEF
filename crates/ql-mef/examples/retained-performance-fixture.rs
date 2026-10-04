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
    let mut arguments = std::env::args().skip(1);
    let opposite_zero = match arguments.next().as_deref() {
        None => false,
        Some("--native-opposite-zero") => true,
        _ => return Err("unknown retained native fixture mode".into()),
    };
    if arguments.next().is_some() {
        return Err("retained native fixture accepts only one optional mode".into());
    }
    let mut input = support::preparation();
    if opposite_zero {
        input.coupled.m1.cycle = "0".into();
        input.coupled.m1.tick12 = 0;
        input.source_face = ql_mef::MFace::Pratibimba;
    }
    let prepared = prepare_native_performance(input)?;
    if opposite_zero
        && (prepared.notes().is_empty()
            || prepared.notes().iter().any(|note| {
                note["phase_sin"].as_f64().map(f64::to_bits) != Some((-0.0_f64).to_bits())
                    || note["phase_cos"].as_f64() != Some(-1.0)
                    || note["source_face"] != json!(1)
            }))
    {
        return Err("native M1 opposite-zero quadrature was not produced".into());
    }
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
