//! Orchestration of existing typed K10 operations, not redefinitions of them.
use super::*;
use ql_mef::nara::domain::operations::{M4AssemblyInput, OracleCastContext};
use ql_mef::nara::{ConsentState, LifecycleState};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cast {
    pub system: OracleSystem,
    pub tradition_ref: String,
    pub deck_or_method_ref: String,
    pub spread_ref: Option<String>,
    pub query_ref: ProtectedRef,
    pub draw_count: u8,
    pub source_revisions: Vec<ql_mef::nara::SourceRevision>,
    pub hygiene: OracleHygiene,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Mutation {
    IdentityReplace {
        slot: IdentityEvidenceSlot,
        identity_revision: String,
    },
    EmbodiedReceive {
        personal: Box<PersonalFieldState>,
        input: EmbodiedContinuationInput,
    },
    CentreFeedback {
        ordinal: u8,
        feedback_ref: String,
    },
    OracleCast {
        cast: Cast,
    },
    OracleInterpret {
        packet_ref: String,
        interpretation: OracleInterpretation,
    },
    PracticeStart {
        phase: TransformationPhase,
    },
    PracticeHold {
        phase_ref: String,
        feedback_ref: String,
    },
    PracticeResume {
        phase_ref: String,
        safety_review_ref: String,
    },
    PracticeClose {
        phase_ref: String,
        feedback_refs: Vec<String>,
    },
    ContextRecord {
        branch: ContextBranchKind,
        reading: ContextReading,
    },
    IntegrationReturn {
        returned: IntegrationReturn,
    },
    JournalLink {
        source: ProtectedRef,
    },
}
impl Mutation {
    pub(super) fn name(&self) -> &'static str {
        match self {
            Self::IdentityReplace { .. } => "identity_replace",
            Self::EmbodiedReceive { .. } => "embodied_receive",
            Self::CentreFeedback { .. } => "centre_feedback",
            Self::OracleCast { .. } => "oracle_cast",
            Self::OracleInterpret { .. } => "oracle_interpret",
            Self::PracticeStart { .. } => "practice_start",
            Self::PracticeHold { .. } => "practice_hold",
            Self::PracticeResume { .. } => "practice_resume",
            Self::PracticeClose { .. } => "practice_close",
            Self::ContextRecord { .. } => "context_record",
            Self::IntegrationReturn { .. } => "integration_return",
            Self::JournalLink { .. } => "journal_link",
        }
    }
}
fn present(reference: &str) -> Result<CapabilityRefs, String> {
    CapabilityRefs::present(vec![reference.into()])
}
fn absent() -> Result<CapabilityRefs, String> {
    CapabilityRefs::absent("No source has been selected for this office in this personal occasion")
}
pub(super) fn initialize(seed: &Seed) -> Result<M4DomainState, String> {
    let p = &seed.personal;
    validate_personal(p)?;
    let slots = [
        IdentitySlotKind::BirthdateName,
        IdentitySlotKind::NatalChart,
        IdentitySlotKind::JungianAssessment,
        IdentitySlotKind::GeneKeys,
        IdentitySlotKind::HumanDesign,
        IdentitySlotKind::ArchetypalQuintessence,
    ]
    .into_iter()
    .map(|kind| IdentityEvidenceSlot {
        kind,
        coordinate_ref: kind.coordinate().into(),
        source: None,
        protected_value_ref: None,
        evidence_refs: vec![],
        tensions: vec![],
        absence_reason: Some("Not supplied by the person".into()),
        standing: EvidenceStanding::Unavailable,
    })
    .collect();
    let branches = [
        ContextBranchKind::Gebser,
        ContextBranchKind::Ontological,
        ContextBranchKind::Epistemological,
        ContextBranchKind::JungianDepth,
        ContextBranchKind::Phenomenological,
        ContextBranchKind::TrikaKashmir,
    ]
    .into_iter()
    .map(|branch| ContextBranchState {
        branch,
        readings: vec![],
        absence_reason: Some("No interpretation has been accepted".into()),
    })
    .collect();
    let offices = [
        IntegrationOffice::CurriculumMap,
        IntegrationOffice::CoreEpiLogosVoice,
        IntegrationOffice::MethodTransparencyLab,
        IntegrationOffice::IntegrationLab,
        IntegrationOffice::PedagogyLab,
        IntegrationOffice::LogosCycleEngine,
    ]
    .into_iter()
    .map(|office| IntegrationOfficeState {
        office,
        available: true,
        unavailable_reason: None,
        returns: vec![],
    })
    .collect();
    // Empty content remains absent. An available integration office can record
    // an actual owner Return; it does not fabricate a teaching result.
    M4DomainState::from_personal_state(p,M4AssemblyInput{
        identity:IdentityField{identity_revision:"unprovided".into(),slots,identity_hash_ref:None,identity_quaternion_ref:None,m3_form_address_ref:None,derivation_refs:vec![]},
        embodied:EmbodiedField::from_personal_state(p,seed.embodied.clone())?,
        oracle:OracleField{common_symbolic_substrate:absent()?,cosmic_oracle_invocation:absent()?,tarot_engines:present("ql:nara:oracle:tarot-permutation")?,iching_integration:present("ql:nara:oracle:coins-yarrow")?,casting_interpretation:present("ql:nara:oracle:os-entropy-original-and-reinterpretation")?,hygiene_pedagogy:absent()?,records:vec![],standing:EvidenceStanding::Implementation},
        transformation:TransformationField{cycle_engine:present("ql:nara:transformation:12x3x24")?,operational_grammar:absent()?,dialogical_containers:absent()?,control_safety:present("ql:nara:practice:explicit-hold-resume")?,protocol_library:absent()?,phase_history:TransformationHistory{history_ref:format!("{}:practice",seed.nara_ref),subject_id:p.subject_id.clone(),phases:vec![],standing:EvidenceStanding::Reported},standing:EvidenceStanding::Implementation},
        context:ContextField{branches,personal_pratibimba_ref:None,recognition_refs:vec![]},
        integration:IntegrationField{offices,operative_return_refs:vec![]},source_revisions:p.source_revisions.clone(),
        standing:"Native K10 continuation of the supplied Personal reception. Identity and interpretations remain unprovided until authored/accepted; not a diagnosis.".into(),
    })
}
fn validate_personal(p: &PersonalFieldState) -> Result<(), String> {
    if p.schema != "ql.nara-personal-field/v1"
        || p.subject_id != p.event.subject_ref
        || p.consent != ConsentState::Granted
        || p.lifecycle != LifecycleState::Active
        || p.reception_generation == 0
    {
        return Err(
            "an active consented source-qualified native Personal reception is required".into(),
        );
    }
    text(&p.subject_id)?;
    text(&p.constitution_ref)?;
    text(&p.event.event_ref)?;
    for r in [
        &p.event.registry_revision,
        &p.event.m1_revision,
        &p.event.m2_source_ref,
        &p.event.m2_contract_ref,
        &p.event.m3_source_ref,
        &p.event.m3_contract_ref,
    ] {
        text(r)?;
    }
    if p.source_revisions.is_empty() {
        return Err("native Personal reception has no source basis".into());
    }
    p.q_composed.normalized()?;
    Ok(())
}
fn current_phase<'a>(
    record: &'a mut PersonalRecord,
    reference: &str,
) -> Result<&'a mut TransformationPhase, String> {
    let phase = record
        .domain
        .transformation
        .phase_history
        .phases
        .last_mut()
        .ok_or("no practice phase is open")?;
    if phase.phase_ref != reference || phase.closed_at_unix_ms.is_some() {
        return Err("the addressed practice phase is not open".into());
    }
    Ok(phase)
}
fn derived(standing: EvidenceStanding) -> Result<(), String> {
    if !matches!(
        standing,
        EvidenceStanding::Derived | EvidenceStanding::Reported | EvidenceStanding::Proposed
    ) {
        return Err("interpretation must remain derived, reported or proposed, never original source/observation".into());
    }
    Ok(())
}
pub(super) fn apply(
    record: &mut PersonalRecord,
    mutation: Mutation,
    request_id: &str,
    at: u64,
    random: &mut dyn FnMut(&mut [u8]) -> Result<(), String>,
) -> Result<(), String> {
    match mutation {
        Mutation::IdentityReplace {
            slot,
            identity_revision,
        } => record
            .domain
            .identity
            .replace_slot(slot, identity_revision)?,
        Mutation::EmbodiedReceive { personal, input } => {
            validate_personal(&personal)?;
            if personal.event != record.domain.event
                || personal.subject_id != record.domain.subject_id
                || personal.reception_generation <= record.domain.embodied.reception_generation
            {
                return Err("reception is stale or belongs to another personal occasion; do not relabel historical records".into());
            }
            record.domain.embodied = EmbodiedField::from_personal_state(&personal, input)?;
            record.domain.q_composed = personal.q_composed;
        }
        Mutation::CentreFeedback {
            ordinal,
            feedback_ref,
        } => {
            text(&feedback_ref)?;
            let centre = record
                .domain
                .embodied
                .centres
                .iter_mut()
                .find(|c| c.ordinal == ordinal)
                .ok_or("no such receiving centre")?;
            if centre.feedback_refs.contains(&feedback_ref) {
                return Err("centre feedback reference already recorded".into());
            }
            centre.feedback_refs.push(feedback_ref);
            centre.validate()?;
        }
        Mutation::OracleCast { cast } => {
            text(&cast.tradition_ref)?;
            text(&cast.deck_or_method_ref)?;
            cast.query_ref.validate()?;
            if cast.hygiene == OracleHygiene::Blocked {
                return Err("oracle hygiene blocks this cast".into());
            }
            if cast.system == OracleSystem::Custom {
                return Err("custom oracle requires a native provider, not fallback Tarot".into());
            }
            if !(1..=78).contains(&cast.draw_count) || cast.source_revisions.is_empty() {
                return Err("cast count/source basis unavailable".into());
            }
            if record.domain.oracle.records.len() >= 4096 {
                return Err("oracle record bound exceeded".into());
            }
            let packet_ref = format!(
                "{}:oracle:{}",
                record.target.record_ref,
                digest(request_id.as_bytes())
            );
            if record
                .domain
                .oracle
                .records
                .iter()
                .any(|r| r.original.packet_ref == packet_ref)
            {
                return Err("original cast already retained; replay it without recasting".into());
            }
            let mut bytes = [0_u8; 1024];
            random(&mut bytes)?;
            let entropy_digest = digest(&bytes);
            let context = OracleCastContext {
                system: cast.system,
                packet_ref: packet_ref.clone(),
                subject_id: record.domain.subject_id.clone(),
                event_ref: record.domain.event.event_ref.clone(),
                profile_generation: record.domain.event.profile_generation,
                tradition_ref: cast.tradition_ref,
                deck_or_method_ref: cast.deck_or_method_ref,
                spread_ref: cast.spread_ref,
                query_ref: cast.query_ref,
                entropy: OracleEntropyReceipt {
                    entropy_ref: ProtectedRef {
                        ref_id: format!("{packet_ref}:entropy"),
                        revision: entropy_digest.clone(),
                        owner_ref: "quaternal-logic".into(),
                    },
                    method_ref: "os-csprng/v1".into(),
                    provider_ref: "getrandom/os".into(),
                    observed_at_unix_ms: at,
                    evidence_refs: vec![format!("sha256:{entropy_digest}")],
                },
                cast_degree: None,
                cast_at_unix_ms: at,
                vak_ref: None,
                original_payload_ref: ProtectedRef {
                    ref_id: packet_ref,
                    revision: (record.revision + 1).to_string(),
                    owner_ref: "quaternal-logic".into(),
                },
                source_revisions: cast.source_revisions,
                consent: ConsentState::Granted,
                hygiene: cast.hygiene,
            };
            let packet = if matches!(
                cast.system,
                OracleSystem::IChingCoins | OracleSystem::IChingYarrow
            ) {
                OracleOriginalPacket::cast_iching(context, &bytes)?
            } else {
                OracleOriginalPacket::draw_tarot(context, cast.draw_count, &bytes)?
            };
            record.domain.oracle.records.push(OracleRecord {
                original: packet,
                interpretations: vec![],
            });
        }
        Mutation::OracleInterpret {
            packet_ref,
            interpretation,
        } => {
            derived(interpretation.standing)?;
            let entry = record
                .domain
                .oracle
                .records
                .iter_mut()
                .find(|r| r.original.packet_ref == packet_ref)
                .ok_or("original oracle packet not found")?;
            if interpretation.interpreted_at_unix_ms < entry.original.cast_at_unix_ms
                || interpretation.interpreted_at_unix_ms > at
            {
                return Err("interpretation time must follow the original cast and not claim a future observation".into());
            }
            entry.add_reinterpretation(interpretation)?;
        }
        Mutation::PracticeStart { phase } => {
            phase.validate()?;
            if phase.safety != TransformationSafety::Clear
                || phase.closed_at_unix_ms.is_some()
                || phase.opened_at_unix_ms > at
            {
                return Err("practice requires an explicit clear, open, non-future phase with source protocol".into());
            }
            if let Some(last) = record.domain.transformation.phase_history.phases.last() {
                if last.closed_at_unix_ms.is_none() {
                    return Err("close the current practice phase before advancing".into());
                }
                if (phase.storey, phase.decan, phase.stroke) != last.next_coordinate() {
                    return Err(
                        "practice phase does not follow the native 12x3x24 coordinate".into(),
                    );
                }
                if phase.opened_at_unix_ms < last.closed_at_unix_ms.unwrap_or(0) {
                    return Err("practice continuation predates the completed phase".into());
                }
            }
            record.domain.transformation.append_phase(phase)?;
        }
        Mutation::PracticeHold {
            phase_ref,
            feedback_ref,
        } => {
            text(&feedback_ref)?;
            let phase = current_phase(record, &phase_ref)?;
            // Explicit hold is consent-required, not a guessed arousal diagnosis.
            phase.safety = TransformationSafety::ConsentRequired;
            phase.feedback_refs.push(feedback_ref);
        }
        Mutation::PracticeResume {
            phase_ref,
            safety_review_ref,
        } => {
            text(&safety_review_ref)?;
            let phase = current_phase(record, &phase_ref)?;
            if phase.safety != TransformationSafety::ConsentRequired {
                return Err("only the explicit personal hold may resume here; unsafe/unavailable protocols require new review".into());
            }
            phase.safety = TransformationSafety::Clear;
            phase.feedback_refs.push(safety_review_ref);
        }
        Mutation::PracticeClose {
            phase_ref,
            feedback_refs,
        } => {
            for r in &feedback_refs {
                text(r)?;
            }
            let phase = current_phase(record, &phase_ref)?;
            if at < phase.opened_at_unix_ms {
                return Err("practice cannot close before opening".into());
            }
            phase.closed_at_unix_ms = Some(at);
            phase.feedback_refs.extend(feedback_refs);
        }
        Mutation::ContextRecord { branch, reading } => {
            derived(reading.standing)?;
            record.domain.context.record_reading(branch, reading)?;
        }
        Mutation::IntegrationReturn { returned } => {
            record.domain.integration.record_return(returned)?
        }
        Mutation::JournalLink { source } => {
            source.validate()?;
            if source.owner_ref != "central" {
                return Err("journal/notes remain Central-owned references".into());
            }
            if record
                .journal_refs
                .iter()
                .any(|r| r.ref_id == source.ref_id && r.revision == source.revision)
            {
                return Err("journal source revision already linked".into());
            }
            record.journal_refs.push(source);
        }
    }
    record.domain.validate()
}
