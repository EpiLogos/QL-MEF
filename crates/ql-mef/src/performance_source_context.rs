//! Original source disclosure belongs to the existing native receiving owner.
//! These private outputs are evidence, not a consent grant. Hosts call the
//! constructors only under their current Act/lease and independently replay
//! the original source on reopen; browser JSON cannot construct either token.
use crate::continuous::coupled::CoupledBasis;
use crate::m_tree::native_current_m_registry;
use crate::musical_performance_return::{ReturnContext, bind_performance_return};
use crate::nara_performance_receiving::{ContextKind, ReceivingDefinition, ReceivingPreparation};
use crate::performance_audio::PreparedPerformanceBinding;
use crate::scene::{WorldRequest, world};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub const CONTRACT: &str = "ql.retained-performance-source-context/v1";
fn hash(value: &Value) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("source context exceeds native source bound".into());
    }
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}
fn snapshot<T: Serialize>(value: &T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|e| e.to_string())
}
fn reference(reference: &str, revision: &str) -> Value {
    json!({"ref":reference,"revision":revision,"availability":"available"})
}
fn same_reference(actual: &Value, expected: &Value) -> bool {
    actual.is_object()
        && expected.is_object()
        && actual["ref"] == expected["reference"]
        && actual["revision"] == expected["revision"]
        && actual["availability"] == "available"
}
/// Privately produced public receipt ownership, never a caller list of flags.
/// Reference has no opaque receipts. World replays the EXISTING scene::world
/// constructor and owns only its exact source-generated sky receipt.
#[derive(Debug, Clone, Serialize)]
pub struct NativePublicSourceOwnership {
    original_native_input: Value,
    source_owner_ref: String,
    source_owner_revision: String,
    receipt_standing: String,
}
impl NativePublicSourceOwnership {
    /// Explicit native Reference source operation under a public World lease.
    /// The actual receiving definition determines context; zero receipt count
    /// only establishes that this operation has no opaque source to classify.
    pub fn reference_source(original: &CoupledBasis) -> Result<Self, String> {
        if !original.input.source_receipts.is_empty() {
            return Err("Reference World cannot classify an opaque original receipt".into());
        }
        let replayed = original.input.compose()?;
        if snapshot(&replayed)? != snapshot(original)? {
            return Err("original native source does not replay".into());
        }
        Ok(Self {
            original_native_input: snapshot(&original.input)?,
            source_owner_ref: "ql:continuous/coupled/reference-source".into(),
            source_owner_revision: format!(
                "sha256:{:x}",
                Sha256::digest(include_str!("continuous/coupled.rs").as_bytes())
            ),
            receipt_standing:
                "explicit native Reference source, no astronomy or protected-occasion claim".into(),
        })
    }
    /// Called at the actual public World source operation after the existing
    /// provider has validated the request. This does not upgrade astronomy or
    /// freshness: those retain their existing CLI/provider admission standing.
    /// A retained personal occurrence must use the protected receiving branch.
    pub fn world_source(request: WorldRequest) -> Result<Self, String> {
        let produced = world(request)?;
        let original = produced["event"].clone();
        let receipts = original["source_receipts"]
            .as_array()
            .ok_or("native World source receipts absent")?;
        if receipts.len() != 1 || receipts[0] != produced["sky"] {
            return Err("public World source did not own exactly its generated sky receipt".into());
        }
        let owner = &produced["native_owner_sources"]["constructor"];
        Ok(Self { original_native_input:original, source_owner_ref:owner["ref"].as_str().ok_or("native World source owner absent")?.into(),source_owner_revision:owner["revision"].as_str().ok_or("native World source revision absent")?.into(), receipt_standing:"exact receipt of existing native public World constructor; provider qualification remains separate".into() })
    }
}
/// Borrowed only from the retained PerformanceOwner after its full original
/// source/currentness check. Consumers cannot splice an unrelated original
/// input into a prepared receiving body.
pub struct NativeSourceContextBasis<'a> {
    original: &'a CoupledBasis,
    prepared: &'a PreparedPerformanceBinding,
}
impl<'a> NativeSourceContextBasis<'a> {
    pub(crate) fn from_retained_owner(
        original: &'a CoupledBasis,
        prepared: &'a PreparedPerformanceBinding,
    ) -> Self {
        Self { original, prepared }
    }
}
/// Source-bound immutable token; no Deserialize or public fields. Retention
/// cannot substitute for current native receiving/lease/consent validation.
#[derive(Debug, Clone, Serialize)]
pub struct NativePerformanceSourceContext {
    schema: &'static str,
    availability: &'static str,
    context: Value,
    original_occasion: Value,
    classified_original_input_sha256: String,
    classifications: Vec<Value>,
    currentness_refs: Vec<Value>,
    receiving_definition: Value,
    native_preparation_sha256: String,
    source_receipt_standing: String,
}
impl NativePerformanceSourceContext {
    pub fn snapshot(&self) -> Result<Value, String> {
        snapshot(self)
    }
    /// Actual source/context owner readback for the existing C readiness route.
    /// Consumers compare these with current native owner readings, never with
    /// an imported packet's own claim of currentness.
    pub fn currentness_refs(&self) -> &[Value] {
        &self.currentness_refs
    }

    pub fn validate_binding(
        &self,
        original: &CoupledBasis,
        prepared: &PreparedPerformanceBinding,
    ) -> Result<(), String> {
        if self.classified_original_input_sha256 != hash(&snapshot(&original.input)?)?
            || self.native_preparation_sha256 != hash(&snapshot(prepared)?)?
        {
            return Err(
                "source context token belongs to another original/prepared native owner".into(),
            );
        }
        Ok(())
    }
    pub fn validate_current(
        &self,
        origin: &NativeSourceContextBasis<'_>,
        definition: &ReceivingDefinition,
        input: ReceivingPreparation<'_>,
        context: ReturnContext,
        public: Option<&NativePublicSourceOwnership>,
    ) -> Result<(), String> {
        let actual = prepare_native_source_context(origin, definition, input, context, public)?;
        if self.snapshot()? != actual.snapshot()? {
            return Err("source context/occasion/consent/current native owner changed".into());
        }
        Ok(())
    }
}
/// Actual native receiving definition plus exact original input and source
/// ownership. No label, missing field, digest, or browser packet grants scope.
pub fn prepare_native_source_context(
    origin: &NativeSourceContextBasis<'_>,
    definition: &ReceivingDefinition,
    input: ReceivingPreparation<'_>,
    context: ReturnContext,
    public: Option<&NativePublicSourceOwnership>,
) -> Result<NativePerformanceSourceContext, String> {
    let original = origin.original;
    let prepared = origin.prepared;
    if snapshot(input.prepared)? != snapshot(prepared)? {
        return Err("receiving context detached from actual retained preparation".into());
    }
    prepared.validate_native_consumers(prepared.native_basis(), prepared.physical_body())?;
    definition.validate_source_registry(native_current_m_registry())?;
    let receipt_context = input.context.clone();
    let occasion = input.original_occasion.cloned();
    definition.validate_sources(input)?;
    let retained_return = bind_performance_return(prepared, occasion.clone(), context, 0)?;
    let returned = retained_return.expression_basis()?;
    let actual_context = &returned["context"];
    let kind = match receipt_context.kind {
        ContextKind::World => "world",
        ContextKind::Personal => "personal",
        ContextKind::Shared => "shared",
    };
    if actual_context["kind"] != kind
        || actual_context["private"] != receipt_context.private
        || !same_reference(
            &actual_context["context"],
            &snapshot(&receipt_context.context)?,
        )
        || !same_reference(
            &actual_context["receiver"],
            &snapshot(&receipt_context.receiver)?,
        )
    {
        return Err("Return context differs from actual receiving owner".into());
    }
    for (field, receipt) in [
        ("source_occasion", &receipt_context.original_occasion),
        ("protected_state", &receipt_context.protected_state),
        ("consent", &receipt_context.consent),
    ] {
        match receipt {
            Some(value) if same_reference(&actual_context[field], &snapshot(value)?) => {}
            None if actual_context[field].is_null() => {}
            _ => {
                return Err(
                    "Return original occasion/protected/consent detached from native receiving"
                        .into(),
                );
            }
        }
    }
    let original_value = snapshot(&original.input)?;
    let mut currentness = vec![
        reference(
            "ql:m-registry",
            &native_current_m_registry().manifest().registry_revision,
        ),
        reference(
            &receipt_context.context.reference,
            &receipt_context.context.revision,
        ),
        reference(
            &receipt_context.receiver.reference,
            &receipt_context.receiver.revision,
        ),
    ];
    let (owner_ref, owner_revision, standing) = if receipt_context.kind == ContextKind::World {
        let ownership = public.ok_or("public World requires native original source ownership")?;
        if ownership.original_native_input != original_value {
            return Err("public ownership belongs to another complete original source".into());
        }
        (
            ownership.source_owner_ref.clone(),
            ownership.source_owner_revision.clone(),
            ownership.receipt_standing.clone(),
        )
    } else {
        if public.is_some() || !receipt_context.private || occasion.is_none() {
            return Err("protected receiving cannot borrow public original ownership".into());
        }
        for item in [
            &receipt_context.protected_state,
            &receipt_context.original_occasion,
            &receipt_context.consent,
        ]
        .into_iter()
        .flatten()
        {
            currentness.push(reference(&item.reference, &item.revision));
        }
        ("ql:nara/native-protected-receiving".into(),definition.content_digest().into(),"conservative private retention of every original receipt under exact native occasion/protected/consent owner".into())
    };
    currentness.push(reference(&owner_ref, &owner_revision));
    let classifications = original.input.source_receipts.iter().enumerate().map(|(index,receipt)|Ok(json!({"receipt_index":index,"receipt_sha256":hash(receipt)?,"private":receipt_context.private,"source_owner_ref":owner_ref,"source_owner_revision":owner_revision}))).collect::<Result<Vec<_>,String>>()?;
    let out = NativePerformanceSourceContext {
        schema: CONTRACT,
        availability: "available",
        context: actual_context.clone(),
        original_occasion: snapshot(&occasion)?,
        classified_original_input_sha256: hash(&original_value)?,
        classifications,
        currentness_refs: currentness,
        receiving_definition: definition.snapshot()?,
        native_preparation_sha256: hash(&snapshot(prepared)?)?,
        source_receipt_standing: standing,
    };
    out.validate_binding(original, prepared)?;
    Ok(out)
}
