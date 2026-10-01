//! Native semantic admission for the hosted Nara/Epii dialogue.
//! The host owns identity, Expression/profile reads and persistence. QL resolves
//! the actual current registry here; a supplied registry is never authority.
use crate::CliError;
use ql_mef::MFace;
use ql_mef::aw1_world::{RootedFace, RootedMWorld, resolve_rooted_m_world};
use ql_mef::coordinate_expression::resolve_coordinate_expression;
use ql_mef::m_tree::native_current_m_registry;
use ql_mef::nara::dialogue::{EpiiDelegation, EpiiEnrichment, MFocus, NaraDialogueContext};
use ql_mef::nara::domain::M4Branch;
use ql_mef::nara::expression::BimbaSelectionBinding;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContextRequest {
    schema: String,
    coordinate_ref: String,
    context: NaraDialogueContext,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoordinateRequest {
    coordinate_ref: String,
    face: Option<RootedFace>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentRequest {
    coordinate_ref: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InventoryRequest {
    offset: usize,
    limit: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoordinateBundleRequest {
    coordinate_refs: Vec<String>,
    #[serde(default)]
    face: Option<RootedFace>,
    #[serde(default)]
    inventory: Option<InventoryRequest>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DelegationRequest {
    context: NaraDialogueContext,
    delegation_ref: String,
    epii_session_ref: String,
    brief: String,
    scope_refs: Vec<String>,
    delegated_at_unix_ms: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnrichmentRequest {
    delegation: EpiiDelegation,
    enrichment: EpiiEnrichment,
    current: NaraDialogueContext,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReceiveRequest {
    delegation: EpiiDelegation,
    enrichment: EpiiEnrichment,
}

fn error(reason: impl std::fmt::Display) -> CliError {
    CliError(reason.to_string())
}

fn resolve(context: &mut NaraDialogueContext, verify: bool) -> Result<RootedMWorld, CliError> {
    let registry = native_current_m_registry();
    // The source spelling is #4.1; the dialogue contract uses the native M4.1
    // alias. Derive that alias through MCoordinate, preserving separators and
    // retaining the exact source spelling separately in the AW1/Bimba reading.
    let selector = context
        .coordinate_ref
        .strip_prefix("ql:m-coordinate:bimba:")
        .or_else(|| {
            context
                .coordinate_ref
                .strip_prefix("ql:m-coordinate:pratibimba:")
        })
        .unwrap_or(&context.coordinate_ref);
    let world = resolve_rooted_m_world(registry, selector).map_err(error)?;
    let canonical = registry
        .coordinate(&world.selected_source_ref, MFace::Bimba)
        .map_err(error)?
        .canonical_ref();
    let dialogue_coordinate = canonical
        .strip_prefix("ql:m-coordinate:bimba:")
        .ok_or_else(|| error("native direct coordinate has an unsupported canonical scheme"))?;
    let bimba = BimbaSelectionBinding::from_rooted_world(&world).map_err(error)?;
    let focus = match world.root_position {
        0 => MFocus::M0,
        1 => MFocus::M1,
        2 => MFocus::M2,
        3 => MFocus::M3,
        4 => MFocus::M4,
        5 => MFocus::M5,
        _ => return Err(error("native coordinate has an unsupported M root")),
    };
    let branch = [
        M4Branch::Identity,
        M4Branch::Embodied,
        M4Branch::Oracle,
        M4Branch::Transformation,
        M4Branch::Context,
        M4Branch::Integration,
    ]
    .into_iter()
    .find(|branch| {
        registry
            .resolve(branch.coordinate())
            .is_some_and(|node| world.ancestry.iter().any(|step| step.id == node.id))
    });
    if verify
        && (context.coordinate_ref != dialogue_coordinate
            || context.bimba.as_ref() != Some(&bimba)
            || context.m4_branch != branch
            || context.active_m_focus != focus)
    {
        return Err(error(
            "dialogue context does not match the current native coordinate and Bimba registry",
        ));
    }
    context.coordinate_ref = dialogue_coordinate.to_owned();
    context.bimba = Some(bimba);
    context.m4_branch = branch;
    context.active_m_focus = focus;
    context.validate().map_err(error)?;
    Ok(world)
}

pub(super) fn command(operation: &str, bytes: &[u8]) -> Result<String, CliError> {
    let result = match operation {
        "coordinate-bundle" => {
            let request: CoordinateBundleRequest = serde_json::from_slice(bytes).map_err(error)?;
            if request.coordinate_refs.is_empty()
                || request.coordinate_refs.len() > 64
                || request
                    .coordinate_refs
                    .iter()
                    .any(|r| r.is_empty() || r.len() > 4096)
            {
                return Err(error(
                    "A coordinate bundle requires 1..64 bounded source references",
                ));
            }
            let content = ql_mef::bimba_content::native_bimba_content().map_err(error)?;
            let face = match request.face {
                Some(RootedFace::Pratibimba) => MFace::Pratibimba,
                _ => MFace::Bimba,
            };
            let items = request
                .coordinate_refs
                .iter()
                .map(|reference| {
                    let binding =
                        resolve_coordinate_expression(native_current_m_registry(), reference, face)
                            .map_err(error)?;
                    let source = content.coordinate(reference).map_err(error)?;
                    Ok(json!({"binding":binding,"source_content":source}))
                })
                .collect::<Result<Vec<_>, CliError>>()?;
            let inventory = request
                .inventory
                .map(|page| content.inventory(page.offset, page.limit))
                .transpose()
                .map_err(error)?;
            json!({"schema":"ql.coordinate-content-bundle/v1","items":items,"inventory":inventory})
        }
        "coordinate-content" => {
            let request: ContentRequest = serde_json::from_slice(bytes).map_err(error)?;
            ql_mef::bimba_content::native_bimba_content()
                .map_err(error)?
                .coordinate(&request.coordinate_ref)
                .map_err(error)?
        }
        "source-inventory" => {
            let request: InventoryRequest = serde_json::from_slice(bytes).map_err(error)?;
            ql_mef::bimba_content::native_bimba_content()
                .map_err(error)?
                .inventory(request.offset, request.limit)
                .map_err(error)?
        }
        "coordinate" => {
            let request: CoordinateRequest = serde_json::from_slice(bytes).map_err(error)?;
            let face = match request.face {
                Some(RootedFace::Bimba) => MFace::Bimba,
                Some(RootedFace::Pratibimba) => MFace::Pratibimba,
                None if request
                    .coordinate_ref
                    .starts_with("ql:m-coordinate:pratibimba:") =>
                {
                    MFace::Pratibimba
                }
                None => MFace::Bimba,
            };
            serde_json::to_value(
                resolve_coordinate_expression(
                    native_current_m_registry(),
                    &request.coordinate_ref,
                    face,
                )
                .map_err(error)?,
            )
            .map_err(error)?
        }
        "context" => {
            let mut request: ContextRequest = serde_json::from_slice(bytes).map_err(error)?;
            if request.schema != "ql.nara-dialogue-context-request/v1" {
                return Err(error("unsupported native dialogue context request"));
            }
            if request.context.bimba.is_some() {
                return Err(error(
                    "context resolution takes no claimed Bimba registry; QL resolves it",
                ));
            }
            if request.coordinate_ref != request.context.coordinate_ref {
                return Err(error("coordinate selector differs from the host context"));
            }
            let face = if request
                .coordinate_ref
                .starts_with("ql:m-coordinate:pratibimba:")
            {
                MFace::Pratibimba
            } else {
                MFace::Bimba
            };
            let world = resolve(&mut request.context, false)?;
            let coordinate_binding = resolve_coordinate_expression(
                native_current_m_registry(),
                &world.selected_source_ref,
                face,
            )
            .map_err(error)?;
            json!({"schema":"ql.nara-dialogue-context-result/v1", "context":request.context,
                "world":world,"coordinate_binding":coordinate_binding,
                "host_basis_owner":"oi","profile_resolution":"host-owned"})
        }
        "delegate" => {
            let mut request: DelegationRequest = serde_json::from_slice(bytes).map_err(error)?;
            resolve(&mut request.context, true)?;
            let delegation = EpiiDelegation::from_context(
                request.delegation_ref,
                &request.context,
                request.epii_session_ref,
                request.brief,
                request.scope_refs,
                request.delegated_at_unix_ms,
            )
            .map_err(error)?;
            serde_json::to_value(delegation).map_err(error)?
        }
        "receive" => {
            let mut request: ReceiveRequest = serde_json::from_slice(bytes).map_err(error)?;
            request
                .delegation
                .receive_enrichment(&request.enrichment)
                .map_err(error)?;
            json!({"schema":"ql.nara-enrichment-receipt/v1", "delegation":request.delegation,
                "enrichment":request.enrichment,"apply_allowed":false,"applied":false,
                "reason":"Returned material only; a current native context and apply gate are required before proposal acceptance"})
        }
        "enrichment" => {
            let mut request: EnrichmentRequest = serde_json::from_slice(bytes).map_err(error)?;
            resolve(&mut request.current, true)?;
            request
                .delegation
                .receive_enrichment(&request.enrichment)
                .map_err(error)?;
            let gate = request
                .delegation
                .apply_gate(&request.enrichment, &request.current);
            json!({"schema":"ql.nara-enrichment-validation/v1",
                "delegation":request.delegation,"enrichment":request.enrichment,
                "apply_allowed":gate.is_ok(),"reason":gate.err(),"applied":false})
        }
        _ => return Err(error("unknown Nara dialogue operation")),
    };
    serde_json::to_string_pretty(&result).map_err(error)
}
