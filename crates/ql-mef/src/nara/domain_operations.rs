//! Enacted M4 operations over the source-qualified domain contracts.
//!
//! These operations continue the accepted K8 Personal receiver rather than
//! introducing a second personal runtime. Caller-supplied source evidence stays
//! explicit whenever K8 does not itself determine an M4 value.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::nara::activity::{ActivityOccurrence, NaraActivityLog, ThoughtConsumptionRef};
use crate::nara::{ConsentState, PersonalFieldState, SourceRevision};

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbodiedContinuationInput {
    pub elemental_efwa: ElementalEfwa,
    pub elemental_source: SourceRevision,
    pub elemental_standing: EvidenceStanding,
    pub nadi_refs: Vec<String>,
    pub sushumna_ref: Option<String>,
    pub temporal_astrology_refs: Vec<String>,
    pub materia_refs: Vec<String>,
    pub operation_refs: Vec<String>,
    pub safety_intensity: Option<f64>,
    pub contraindication_refs: Vec<String>,
    pub response_refs: Vec<String>,
    pub adjustment_refs: Vec<String>,
}

impl EmbodiedContinuationInput {
    pub fn validate(&self) -> Result<(), String> {
        self.elemental_efwa.validate()?;
        check_source(&self.elemental_source)?;
        check_refs(&self.nadi_refs, "nadi reference", 256)?;
        if let Some(reference) = &self.sushumna_ref {
            check_text(reference, "sushumna reference")?;
        }
        check_refs(
            &self.temporal_astrology_refs,
            "temporal astrology reference",
            256,
        )?;
        check_refs(&self.materia_refs, "materia reference", 256)?;
        check_refs(&self.operation_refs, "embodied operation reference", 256)?;
        if self
            .safety_intensity
            .is_some_and(|value| !value.is_finite() || value < 0.0)
        {
            return Err("embodied intensity must be finite and non-negative".into());
        }
        check_refs(
            &self.contraindication_refs,
            "embodied contraindication reference",
            256,
        )?;
        check_refs(&self.response_refs, "embodied response reference", 256)?;
        check_refs(&self.adjustment_refs, "embodied adjustment reference", 256)
    }
}

impl EmbodiedField {
    /// Continue the accepted K8 seven-receiver state into M4.1 without inventing
    /// phase, centre couplings or EarthBody amplitude that K8 did not determine.
    /// The exact M1/M2/M3 contribution triple for every centre is retained.
    pub fn from_personal_state(
        personal: &PersonalFieldState,
        input: EmbodiedContinuationInput,
    ) -> Result<Self, String> {
        input.validate()?;
        if personal.receivers.len() != CENTRE_COUNT {
            return Err("accepted Personal state does not contain seven centres".into());
        }
        if personal.consent != ConsentState::Granted {
            return Err("M4.1 continuation requires granted personal consent".into());
        }

        let mut seen = BTreeSet::new();
        let mut centres = Vec::with_capacity(CENTRE_COUNT);
        for receiver in &personal.receivers {
            if !seen.insert(receiver.ordinal) {
                return Err("accepted Personal state contains duplicate centre ordinals".into());
            }
            centres.push(CentreEmbodiment {
                ordinal: receiver.ordinal,
                label: receiver.label.clone(),
                source: receiver.source.clone(),
                world_inputs: CentreWorldInputs {
                    m1: receiver.input_basis[0].clone(),
                    m2: receiver.input_basis[1].clone(),
                    m3: receiver.input_basis[2].clone(),
                },
                amplitude: Some(receiver.resonance),
                phase_radians: None,
                modes: Vec::new(),
                couplings: Vec::new(),
                body_zone_refs: Vec::new(),
                sense_refs: Vec::new(),
                action_refs: Vec::new(),
                feedback_refs: Vec::new(),
                standing: EvidenceStanding::Derived,
            });
        }
        centres.sort_by_key(|centre| centre.ordinal);

        let field = Self {
            reception_generation: personal.reception_generation,
            elemental_efwa: input.elemental_efwa,
            elemental_source: input.elemental_source,
            elemental_standing: input.elemental_standing,
            centres,
            earth_body: EarthBodyEmbodiment {
                source: personal.earth_body.source.clone(),
                frame_ref: personal.earth_body.frame_ref.clone(),
                amplitude: None,
                phase_radians: None,
                relation_refs: vec![personal.event.event_ref.clone()],
                standing: EvidenceStanding::Derived,
            },
            nadi_refs: input.nadi_refs,
            sushumna_ref: input.sushumna_ref,
            temporal_astrology_refs: input.temporal_astrology_refs,
            materia_refs: input.materia_refs,
            operation_refs: input.operation_refs,
            safety: EmbodiedSafetyState {
                consent: personal.consent.clone(),
                intensity: input.safety_intensity,
                contraindication_refs: input.contraindication_refs,
                response_refs: input.response_refs,
                adjustment_refs: input.adjustment_refs,
                standing: EvidenceStanding::Derived,
            },
        };
        field.validate()?;
        Ok(field)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M4AssemblyInput {
    pub identity: IdentityField,
    pub embodied: EmbodiedField,
    pub oracle: OracleField,
    pub transformation: TransformationField,
    pub context: ContextField,
    pub integration: IntegrationField,
    pub source_revisions: Vec<SourceRevision>,
    pub standing: String,
}

impl M4DomainState {
    /// Assemble one complete M4 state over the exact accepted Personal occasion.
    pub fn from_personal_state(
        personal: &PersonalFieldState,
        input: M4AssemblyInput,
    ) -> Result<Self, String> {
        if input.embodied.reception_generation != personal.reception_generation {
            return Err("M4.1 reception generation does not match Personal state".into());
        }
        if input.transformation.phase_history.subject_id != personal.subject_id {
            return Err("M4.3 transformation history belongs to another Nara".into());
        }
        let state = Self {
            schema: M4_DOMAIN_CONTRACT.into(),
            subject_id: personal.subject_id.clone(),
            event: personal.event.clone(),
            identity: input.identity,
            embodied: input.embodied,
            oracle: input.oracle,
            transformation: input.transformation,
            context: input.context,
            integration: input.integration,
            q_composed: personal.q_composed,
            source_revisions: input.source_revisions,
            standing: input.standing,
        };
        state.validate()?;
        Ok(state)
    }
}

impl IdentityField {
    /// Replace one source-defined identity office without mutating another.
    pub fn replace_slot(
        &mut self,
        slot: IdentityEvidenceSlot,
        identity_revision: String,
    ) -> Result<(), String> {
        slot.validate()?;
        check_text(&identity_revision, "identity revision")?;
        let existing = self
            .slots
            .iter_mut()
            .find(|candidate| candidate.kind == slot.kind)
            .ok_or("identity office is not present in the six-office matrix")?;
        *existing = slot;
        self.identity_revision = identity_revision;
        self.validate()
    }
}

impl TransformationPhase {
    /// Advance the historical 12 × 3 / 24-stroke coordinate as an odometer.
    /// Protocol and operation meaning are never generated by this arithmetic.
    pub const fn next_coordinate(&self) -> (u8, u8, u8) {
        if self.stroke < 23 {
            return (self.storey, self.decan, self.stroke + 1);
        }
        if self.decan < 2 {
            return (self.storey, self.decan + 1, 0);
        }
        ((self.storey + 1) % 12, 0, 0)
    }
}

impl ContextField {
    pub fn record_reading(
        &mut self,
        branch: ContextBranchKind,
        reading: ContextReading,
    ) -> Result<(), String> {
        reading.validate(branch)?;
        let state = self
            .branches
            .iter_mut()
            .find(|candidate| candidate.branch == branch)
            .ok_or("context branch is not present in the six-branch field")?;
        if state
            .readings
            .iter()
            .any(|existing| existing.reading_ref == reading.reading_ref)
        {
            return Err("context reading reference already exists".into());
        }
        state.absence_reason = None;
        state.readings.push(reading);
        self.validate()
    }
}

impl IntegrationField {
    pub fn record_return(&mut self, returned: IntegrationReturn) -> Result<(), String> {
        let office = self
            .offices
            .iter_mut()
            .find(|candidate| candidate.office == returned.office)
            .ok_or("integration office is not present in the six-office field")?;
        if !office.available {
            return Err("cannot record a Return into an unavailable M4.5 office".into());
        }
        returned.validate(office.office)?;
        if office
            .returns
            .iter()
            .any(|existing| existing.return_ref == returned.return_ref)
        {
            return Err("integration Return reference already exists".into());
        }
        office.returns.push(returned);
        self.validate()
    }
}

impl NaraActivityLog {
    pub fn record_occurrence(&mut self, occurrence: ActivityOccurrence) -> Result<(), String> {
        occurrence.validate()?;
        if occurrence.subject_id != self.subject_id {
            return Err("activity occurrence belongs to another Nara".into());
        }
        if self
            .occurrences
            .iter()
            .any(|existing| existing.activity_ref == occurrence.activity_ref)
        {
            return Err("activity reference already exists".into());
        }
        self.occurrences.push(occurrence);
        self.validate()
    }

    pub fn record_thought_consumption(
        &mut self,
        thought: ThoughtConsumptionRef,
    ) -> Result<(), String> {
        thought.validate()?;
        if self
            .thought_consumptions
            .iter()
            .any(|existing| existing.thought_ref == thought.thought_ref)
        {
            return Err("consumed thought reference already exists".into());
        }
        self.thought_consumptions.push(thought);
        self.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OracleCastContext {
    pub system: OracleSystem,
    pub packet_ref: String,
    pub subject_id: String,
    pub event_ref: String,
    pub profile_generation: u64,
    pub tradition_ref: String,
    pub deck_or_method_ref: String,
    pub spread_ref: Option<String>,
    pub query_ref: ProtectedRef,
    pub entropy: OracleEntropyReceipt,
    pub cast_degree: Option<u16>,
    pub cast_at_unix_ms: u64,
    pub vak_ref: Option<String>,
    pub original_payload_ref: ProtectedRef,
    pub source_revisions: Vec<SourceRevision>,
    pub consent: ConsentState,
    pub hygiene: OracleHygiene,
}

impl OracleCastContext {
    fn packet(self, system: OracleSystem, tokens: Vec<OracleToken>) -> OracleOriginalPacket {
        OracleOriginalPacket {
            packet_ref: self.packet_ref,
            subject_id: self.subject_id,
            event_ref: self.event_ref,
            profile_generation: self.profile_generation,
            system,
            tradition_ref: self.tradition_ref,
            deck_or_method_ref: self.deck_or_method_ref,
            spread_ref: self.spread_ref,
            query_ref: self.query_ref,
            entropy: self.entropy,
            tokens,
            cast_degree: self.cast_degree,
            cast_at_unix_ms: self.cast_at_unix_ms,
            vak_ref: self.vak_ref,
            original_payload_ref: self.original_payload_ref,
            source_revisions: self.source_revisions,
            consent: self.consent,
            hygiene: self.hygiene,
        }
    }

    fn token_sources(&self) -> Vec<String> {
        self.source_revisions
            .iter()
            .map(|source| source.source_ref.clone())
            .collect()
    }
}

struct EntropyCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> EntropyCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn byte(&mut self) -> Result<u8, String> {
        let value = self
            .bytes
            .get(self.offset)
            .copied()
            .ok_or("insufficient entropy bytes for requested oracle cast")?;
        self.offset += 1;
        Ok(value)
    }

    fn unbiased_index(&mut self, upper_exclusive: u16) -> Result<usize, String> {
        if upper_exclusive == 0 || upper_exclusive > 256 {
            return Err("oracle entropy index bound must be within 1..=256".into());
        }
        let range = u16::from(u8::MAX) + 1;
        let limit = range - range % upper_exclusive;
        loop {
            let byte = u16::from(self.byte()?);
            if byte < limit {
                return Ok(usize::from(byte % upper_exclusive));
            }
        }
    }
}

impl OracleOriginalPacket {
    /// Three-coin casting preserves the 1:3:3:1 line distribution; yarrow uses
    /// the traditional 1:5:7:3 distribution. Raw entropy is never serialized.
    pub fn cast_iching(context: OracleCastContext, entropy_bytes: &[u8]) -> Result<Self, String> {
        if !matches!(
            context.system,
            OracleSystem::IChingCoins | OracleSystem::IChingYarrow
        ) {
            return Err("I-Ching cast requires a coins or yarrow oracle system".into());
        }
        let system = context.system;
        let token_sources = context.token_sources();
        let mut cursor = EntropyCursor::new(entropy_bytes);
        let mut tokens = Vec::with_capacity(6);
        for position in 0..6_u16 {
            let value = match system {
                OracleSystem::IChingCoins => {
                    let mut sum = 6_u8;
                    for _ in 0..3 {
                        sum += cursor.unbiased_index(2)? as u8;
                    }
                    sum
                }
                OracleSystem::IChingYarrow => match cursor.unbiased_index(16)? {
                    0 => 6,
                    1..=5 => 7,
                    6..=12 => 8,
                    _ => 9,
                },
                _ => unreachable!(),
            };
            tokens.push(OracleToken {
                token_ref: format!("iching-line:{value}"),
                position,
                reversed: false,
                changing: matches!(value, 6 | 9),
                source_refs: token_sources.clone(),
            });
        }
        let packet = context.packet(system, tokens);
        packet.validate()?;
        Ok(packet)
    }

    /// Draw an unbiased permutation prefix from a 78-card Tarot deck. Numeric
    /// card identity remains separate from deck-specific source meanings.
    pub fn draw_tarot(
        context: OracleCastContext,
        draw_count: u8,
        entropy_bytes: &[u8],
    ) -> Result<Self, String> {
        if !matches!(
            context.system,
            OracleSystem::TarotRws
                | OracleSystem::TarotThoth
                | OracleSystem::TarotMarseille
                | OracleSystem::TarotQl
        ) {
            return Err("Tarot draw requires a Tarot oracle system".into());
        }
        if draw_count == 0 || draw_count > 78 {
            return Err("Tarot draw count must be within 1..=78".into());
        }
        let system = context.system;
        let token_sources = context.token_sources();
        let mut cursor = EntropyCursor::new(entropy_bytes);
        let mut deck = (0_u8..78).collect::<Vec<_>>();
        for index in (1..deck.len()).rev() {
            let selected = cursor.unbiased_index((index + 1) as u16)?;
            deck.swap(index, selected);
        }
        let mut tokens = Vec::with_capacity(usize::from(draw_count));
        for (position, card) in deck.into_iter().take(usize::from(draw_count)).enumerate() {
            tokens.push(OracleToken {
                token_ref: format!("tarot-card:{card}"),
                position: position as u16,
                reversed: cursor.unbiased_index(2)? == 1,
                changing: false,
                source_refs: token_sources.clone(),
            });
        }
        let packet = context.packet(system, tokens);
        packet.validate()?;
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nara::{
        BioQuaternion, EarthBodyState, EventBasisRefs, LifecycleState, ReceiverState,
        WorldContribution,
    };

    fn source(name: &str) -> SourceRevision {
        SourceRevision {
            source_ref: format!("source:{name}"),
            revision: "r1".into(),
            standing_ref: "controlled-test".into(),
        }
    }

    fn protected(name: &str) -> ProtectedRef {
        ProtectedRef {
            ref_id: format!("protected:{name}"),
            revision: "r1".into(),
            owner_ref: "central".into(),
        }
    }

    fn contribution(name: &str, value: f64) -> WorldContribution {
        WorldContribution {
            basis_ref: format!("basis:{name}"),
            source_ref: format!("source:{name}"),
            value,
        }
    }

    fn personal() -> PersonalFieldState {
        let receivers = (0..CENTRE_COUNT)
            .map(|ordinal| ReceiverState {
                ordinal: ordinal as u8,
                label: format!("centre-{ordinal}"),
                source: source(&format!("centre-{ordinal}")),
                input_basis: [
                    contribution("m1", 1.0),
                    contribution("m2", 2.0),
                    contribution("m3", 3.0),
                ],
                bioquaternion: BioQuaternion::IDENTITY,
                receiver_orientation: BioQuaternion::IDENTITY,
                composed_orientation: BioQuaternion::IDENTITY,
                orientation_alignment: 1.0,
                drive: ordinal as f64,
                resonance: ordinal as f64 + 0.5,
                reradiation: ordinal as f64 / 2.0,
            })
            .collect();
        PersonalFieldState {
            schema: "ql.nara-personal-field/v1".into(),
            subject_id: "nara-a".into(),
            constitution_ref: "constitution:a".into(),
            reception_generation: 4,
            event: EventBasisRefs {
                event_ref: "event:1".into(),
                subject_ref: "nara-a".into(),
                profile_generation: 1,
                registry_revision: "registry:r1".into(),
                m1_revision: "basis:m1".into(),
                m2_source_ref: "basis:m2".into(),
                m2_contract_ref: "m2:contract".into(),
                m3_source_ref: "basis:m3".into(),
                m3_contract_ref: "m3:contract".into(),
            },
            observed_at_unix_ms: 10,
            consent: ConsentState::Granted,
            lifecycle: LifecycleState::Active,
            q_identity: BioQuaternion::IDENTITY,
            q_transit: BioQuaternion::IDENTITY,
            q_activity: BioQuaternion::IDENTITY,
            q_composed: BioQuaternion::IDENTITY,
            ephemeral_source: None,
            receivers,
            earth_body: EarthBodyState {
                source: source("earth-body"),
                frame_ref: "earth-fixed".into(),
                orientation: BioQuaternion::IDENTITY,
                relation_alignment: 1.0,
            },
            aggregate_resonance: 24.5,
            aggregate_reradiation: 10.5,
            source_revisions: vec![source("personal")],
            standing: "controlled test".into(),
        }
    }

    fn cast_context(system: OracleSystem) -> OracleCastContext {
        OracleCastContext {
            system,
            packet_ref: "oracle:1".into(),
            subject_id: "nara-a".into(),
            event_ref: "event:1".into(),
            profile_generation: 1,
            tradition_ref: "tradition:test".into(),
            deck_or_method_ref: "method:test".into(),
            spread_ref: Some("spread:test".into()),
            query_ref: protected("query"),
            entropy: OracleEntropyReceipt {
                entropy_ref: protected("entropy"),
                method_ref: "explicit-test-bytes".into(),
                provider_ref: "provider:test".into(),
                observed_at_unix_ms: 10,
                evidence_refs: vec!["evidence:entropy".into()],
            },
            cast_degree: Some(100),
            cast_at_unix_ms: 11,
            vak_ref: None,
            original_payload_ref: protected("original-packet"),
            source_revisions: vec![source("oracle")],
            consent: ConsentState::Granted,
            hygiene: OracleHygiene::Clear,
        }
    }

    #[test]
    fn personal_receiver_continues_exact_contribution_paths_without_invented_phase() {
        let field = EmbodiedField::from_personal_state(
            &personal(),
            EmbodiedContinuationInput {
                elemental_efwa: ElementalEfwa {
                    earth: 0.1,
                    fire: 0.2,
                    water: 0.3,
                    air: 0.4,
                },
                elemental_source: source("elemental"),
                elemental_standing: EvidenceStanding::Observed,
                nadi_refs: vec!["nadi:ida".into(), "nadi:pingala".into()],
                sushumna_ref: Some("nadi:sushumna".into()),
                temporal_astrology_refs: Vec::new(),
                materia_refs: Vec::new(),
                operation_refs: Vec::new(),
                safety_intensity: None,
                contraindication_refs: Vec::new(),
                response_refs: Vec::new(),
                adjustment_refs: Vec::new(),
            },
        )
        .unwrap();
        assert_eq!(field.centres.len(), CENTRE_COUNT);
        assert!(
            field
                .centres
                .iter()
                .all(|centre| centre.phase_radians.is_none())
        );
        assert_eq!(field.centres[6].amplitude, Some(6.5));
        assert_eq!(field.centres[6].world_inputs.m2.basis_ref, "basis:m2");
        assert_eq!(field.earth_body.frame_ref, "earth-fixed");
    }

    #[test]
    fn transformation_coordinate_cascades_across_stroke_decan_and_storey() {
        let phase = TransformationPhase {
            phase_ref: "phase:last".into(),
            storey: 11,
            decan: 2,
            stroke: 23,
            operation_refs: vec!["operation:test".into()],
            protocol_ref: "protocol:test".into(),
            container_ref: "container:test".into(),
            source_revisions: vec![source("transformation")],
            opened_at_unix_ms: 1,
            closed_at_unix_ms: Some(2),
            safety: TransformationSafety::Clear,
            feedback_refs: Vec::new(),
        };
        assert_eq!(phase.next_coordinate(), (0, 0, 0));
    }

    #[test]
    fn i_ching_three_coin_cast_uses_explicit_entropy() {
        let packet =
            OracleOriginalPacket::cast_iching(cast_context(OracleSystem::IChingCoins), &[0_u8; 18])
                .unwrap();
        assert_eq!(packet.tokens.len(), 6);
        assert!(
            packet
                .tokens
                .iter()
                .all(|token| token.token_ref == "iching-line:6" && token.changing)
        );
    }

    #[test]
    fn tarot_draw_is_unique_and_raw_entropy_is_not_serialized() {
        let entropy = (0..=255_u8).cycle().take(256).collect::<Vec<_>>();
        let packet =
            OracleOriginalPacket::draw_tarot(cast_context(OracleSystem::TarotThoth), 5, &entropy)
                .unwrap();
        let unique = packet
            .tokens
            .iter()
            .map(|token| token.token_ref.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(unique.len(), 5);
        let encoded = serde_json::to_string(&packet).unwrap();
        assert!(!encoded.contains("entropy_bytes"));
    }
}
