//! Native N and A/P source admission under the existing serial control owner.
//! Serialized bytes retain evidence; they grant neither personal consent nor a
//! lease. Only actual native constructors can produce this immutable output.
use crate::continuous::coupled::CoupledBasis;
use crate::m_tree::native_current_m_registry;
use crate::nara::{EventBasisRefs, NativeEventGenerations};
use crate::nara_performance_receiving::{ReceivingDefinition, ReceivingPreparation};
use serde::Serialize;
use serde_json::Value;

pub const CONTRACT: &str = "ql.performance-receiving-admission/v1";
#[derive(Debug, Clone, Serialize)]
pub struct NativeReceivingAdmission {
    schema: &'static str,
    native_basis: Value,
    native_preparation: Value,
    native_generations: NativeEventGenerations,
    receiving_definition: Value,
    operation: Value,
}
impl NativeReceivingAdmission {
    pub fn snapshot(&self) -> Result<Value, String> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
    pub fn validate_current(
        &self,
        definition: &ReceivingDefinition,
        input: ReceivingPreparation<'_>,
        actual_current_basis: &CoupledBasis,
        native_admitted_cursor: u64,
    ) -> Result<(), String> {
        let rebuilt = prepare_native_receiving_admission(
            definition,
            input,
            actual_current_basis,
            native_admitted_cursor,
        )?;
        if self.snapshot()? != rebuilt.snapshot()? {
            return Err(
                "receiving admission differs from complete current native source replay".into(),
            );
        }
        Ok(())
    }
}
/// The existing lease/context owner supplies the actual current native inputs,
/// including original occasion/grants. A browser cannot call this with a
/// deserialized prepared binding or definition: both have private constructors.
pub fn prepare_native_receiving_admission(
    definition: &ReceivingDefinition,
    input: ReceivingPreparation<'_>,
    actual_current_basis: &CoupledBasis,
    native_admitted_cursor: u64,
) -> Result<NativeReceivingAdmission, String> {
    let prepared = input.prepared;
    prepared.validate_native_consumers(actual_current_basis, prepared.physical_body())?;
    definition.validate_source_registry(native_current_m_registry())?;
    definition.validate_sources(input)?;
    let native_generations = EventBasisRefs::native_generations(actual_current_basis)?;
    let operation = serde_json::to_value(definition.native_operation(native_admitted_cursor)?)
        .map_err(|e| e.to_string())?;
    if operation["native_generations"]
        != serde_json::to_value(&native_generations).map_err(|e| e.to_string())?
    {
        return Err("receiving operation belongs to another actual native basis".into());
    }
    let out = NativeReceivingAdmission {
        schema: CONTRACT,
        native_basis: serde_json::to_value(actual_current_basis).map_err(|e| e.to_string())?,
        native_preparation: serde_json::to_value(prepared).map_err(|e| e.to_string())?,
        native_generations,
        receiving_definition: definition.snapshot()?,
        operation,
    };
    if serde_json::to_vec(&out).map_err(|e| e.to_string())?.len() > 16 * 1024 * 1024 {
        return Err("receiving admission exceeds bounded native control message".into());
    }
    Ok(out)
}
