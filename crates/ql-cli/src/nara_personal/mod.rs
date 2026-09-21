//! Native operations over accepted K10 personal records, not a journal or
//! Agent runtime. Central human/source content stays by reference.
mod identity_material;
mod operations;
mod store;
use crate::CliError;
use ql_mef::nara::domain::operations::EmbodiedContinuationInput;
use ql_mef::nara::{domain::*, PersonalFieldState};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{io::Read, path::PathBuf};
pub const CONTRACT: &str = "ql.nara-personal-operations/v1";
pub const RECORD: &str = "ql.nara-personal-record/v1";
pub const MAX_BYTES: u64 = 4 * 1024 * 1024;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub record_ref: String,
    pub nara_ref: String,
    pub subject_ref: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Consent {
    pub actor_ref: String,
    pub actor_kind: String,
    pub personal_data: bool,
}
impl Consent {
    fn validate(&self) -> Result<(), String> {
        text(&self.actor_ref)?;
        if self.actor_kind != "human" || !self.personal_data {
            return Err("explicit local human personal-data consent is required".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub nara_ref: String,
    pub personal: PersonalFieldState,
    pub embodied: EmbodiedContinuationInput,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum Request {
    // An empty struct, not a unit variant: serde must reject private extras.
    Capabilities {},
    List {
        consent: Consent,
    },
    IdentityMaterial {
        target: Target,
        consent: Consent,
        expected_revision: u64,
    },
    Read {
        target: Target,
        consent: Consent,
    },
    Create {
        request_id: String,
        consent: Consent,
        seed: Box<Seed>,
    },
    Apply {
        request_id: String,
        consent: Consent,
        target: Target,
        expected_revision: u64,
        mutation: Box<operations::Mutation>,
    },
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub request_id: String,
    pub request_digest: String,
    pub actor_ref: String,
    pub operation: String,
    pub previous_revision: u64,
    pub revision: u64,
    pub occurred_at_unix_ms: u64,
    pub source_mutated: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersonalRecord {
    pub schema: String,
    pub target: Target,
    pub revision: u64,
    pub domain: M4DomainState,
    /// Central owns the actual notes, Day, Flow and journal.
    pub journal_refs: Vec<ProtectedRef>,
    pub receipts: Vec<Receipt>,
    pub private: bool,
}
impl PersonalRecord {
    fn validate(&self) -> Result<(), String> {
        if self.schema != RECORD || !self.private || self.revision == 0 {
            return Err("invalid private personal record".into());
        }
        self.domain.validate()?;
        text(&self.target.nara_ref)?;
        if self.target.subject_ref != self.domain.subject_id
            || self.target.record_ref != record_ref(&self.target.nara_ref, &self.domain)
        {
            return Err("personal record identity does not match its native occasion".into());
        }
        for link in &self.journal_refs {
            link.validate()?;
        }
        if self.journal_refs.len() > 4096 || self.receipts.len() > 8192 {
            return Err(
                "personal working record bound exceeded; continue in a new occasion".into(),
            );
        }
        Ok(())
    }
}
fn text(s: &str) -> Result<(), String> {
    if s.trim().is_empty() || s.len() > 4096 || s.chars().any(char::is_control) {
        return Err("invalid native reference".into());
    }
    Ok(())
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn record_ref(nara: &str, domain: &M4DomainState) -> String {
    // A record address over existing identities, never a new Nara/session.
    format!(
        "ql:nara-record:{}",
        digest(
            serde_json::to_string(&(
                nara,
                &domain.subject_id,
                &domain.event.event_ref,
                domain.event.profile_generation
            ))
            .expect("strings serialize")
            .as_bytes()
        )
    )
}
fn now() -> Result<u64, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| "native clock is before epoch".to_owned())
        .and_then(|v| u64::try_from(v.as_millis()).map_err(|_| "native clock overflow".into()))
}
fn entropy(bytes: &mut [u8]) -> Result<(), String> {
    getrandom::fill(bytes).map_err(|_| "OS entropy is unavailable; no cast was performed".into())
}
fn same_target(a: &Target, b: &Target) -> bool {
    a.record_ref == b.record_ref && a.nara_ref == b.nara_ref && a.subject_ref == b.subject_ref
}
fn reply(record: PersonalRecord, receipt: Option<Receipt>, duplicate: bool) -> Value {
    json!({"schema":CONTRACT,"ok":true,"record":record,"receipt":receipt,"duplicate":duplicate,"private":true,"source_mutated":false,"automatic_agent_or_model_invocation":false})
}
/// Production and tests share the real native handler. Only the host chooses
/// the private root; request payloads cannot redirect storage or install data.
pub fn execute(root: &std::path::Path, request: Request) -> Result<Value, String> {
    execute_with(root, request, &mut entropy, &now)
}
fn execute_with(
    root: &std::path::Path,
    request: Request,
    random: &mut dyn FnMut(&mut [u8]) -> Result<(), String>,
    clock: &dyn Fn() -> Result<u64, String>,
) -> Result<Value, String> {
    if matches!(request, Request::Capabilities {}) {
        return Ok(capabilities());
    }
    let request_digest = digest(&serde_json::to_vec(&request).map_err(|_| "invalid request")?);
    match request {
        Request::Capabilities {} => unreachable!(),
        Request::List { consent } => {
            consent.validate()?;
            let records = store::list(root)?;
            Ok(
                json!({"schema":CONTRACT,"ok":true,"records":records,"private":true,"automatic_agent_or_model_invocation":false}),
            )
        }
        Request::IdentityMaterial {
            target,
            consent,
            expected_revision,
        } => {
            consent.validate()?;
            let record = store::read(root, &target.record_ref)?;
            if !same_target(&target, &record.target) {
                return Err("personal target mismatch".into());
            }
            if record.revision != expected_revision {
                return Err("personal revision changed before identity material read".into());
            }
            let material = identity_material::material(&record)?;
            Ok(
                json!({"schema":CONTRACT,"ok":true,"material":material,"private":true,
                "source_mutated":false,"automatic_agent_or_model_invocation":false}),
            )
        }
        Request::Read { target, consent } => {
            consent.validate()?;
            let record = store::read(root, &target.record_ref)?;
            if !same_target(&target, &record.target) {
                return Err("personal target mismatch".into());
            }
            Ok(reply(record, None, false))
        }
        Request::Create {
            request_id,
            consent,
            seed,
        } => {
            consent.validate()?;
            text(&request_id)?;
            text(&seed.nara_ref)?;
            let domain = operations::initialize(&seed)?;
            let target = Target {
                record_ref: record_ref(&seed.nara_ref, &domain),
                nara_ref: seed.nara_ref.clone(),
                subject_ref: domain.subject_id.clone(),
            };
            let _lock = store::lock(root, &target.record_ref)?;
            if let Some(existing) = store::read_optional(root, &target.record_ref)? {
                if let Some(r) = existing
                    .receipts
                    .iter()
                    .find(|r| r.request_id == request_id)
                {
                    if r.request_digest != request_digest {
                        return Err("request identity reused with different input".into());
                    }
                    return Ok(reply(existing.clone(), Some(r.clone()), true));
                }
                return Err("this native personal occasion already exists; read it without recreating identity".into());
            }
            let receipt = Receipt {
                request_id,
                request_digest,
                actor_ref: consent.actor_ref,
                operation: "create".into(),
                previous_revision: 0,
                revision: 1,
                occurred_at_unix_ms: clock()?,
                source_mutated: false,
            };
            let record = PersonalRecord {
                schema: RECORD.into(),
                target,
                revision: 1,
                domain,
                journal_refs: vec![],
                receipts: vec![receipt.clone()],
                private: true,
            };
            record.validate()?;
            store::write(root, &record)?;
            Ok(reply(record, Some(receipt), false))
        }
        Request::Apply {
            request_id,
            consent,
            target,
            expected_revision,
            mutation,
        } => {
            consent.validate()?;
            text(&request_id)?;
            let _lock = store::lock(root, &target.record_ref)?;
            let mut record = store::read(root, &target.record_ref)?;
            if !same_target(&target, &record.target) {
                return Err("personal target mismatch".into());
            }
            if let Some(r) = record.receipts.iter().find(|r| r.request_id == request_id) {
                if r.request_digest != request_digest {
                    return Err("request identity reused with different input".into());
                }
                return Ok(reply(record.clone(), Some(r.clone()), true));
            }
            if record.revision != expected_revision {
                return Err(
                    "personal revision conflict; re-read before another deliberate act".into(),
                );
            }
            let at = clock()?;
            let operation = mutation.name().to_owned();
            // Transform a private clone. Invalid partial fields never persist.
            operations::apply(&mut record, *mutation, &request_id, at, random)?;
            let revision = record
                .revision
                .checked_add(1)
                .ok_or("personal revision overflow")?;
            let receipt = Receipt {
                request_id,
                request_digest,
                actor_ref: consent.actor_ref,
                operation,
                previous_revision: record.revision,
                revision,
                occurred_at_unix_ms: at,
                source_mutated: false,
            };
            record.revision = revision;
            record.receipts.push(receipt.clone());
            record.validate()?;
            store::write(root, &record)?;
            Ok(reply(record, Some(receipt), false))
        }
    }
}
fn capabilities() -> Value {
    json!({"schema":CONTRACT,"ok":true,"domain_contract":M4_DOMAIN_CONTRACT,
    "private":true,"storage":"owner-private-derived-working-record; Central source remains by reference",
    "identity_material_contract":identity_material::SCHEMA,"identity_material_algorithm":identity_material::ALGORITHM,
    "operations":["list","read","create","identity_material","identity_seal","identity_replace","embodied_receive","centre_feedback","oracle_cast","oracle_interpret","practice_start","practice_hold","practice_resume","practice_close","context_record","integration_return","journal_link"],
    "identity_slots":["birthdate-name","natal-chart","jungian-assessment","gene-keys","human-design","archetypal-quintessence"],
    "oracle_systems":["tarot-rws","tarot-thoth","tarot-marseille","tarot-ql","i-ching-coins","i-ching-yarrow"],
    "context_branches":["gebser","ontological","epistemological","jungian-depth","phenomenological","trika-kashmir"],
    "integration_offices":["curriculum-map","core-epi-logos-voice","method-transparency-lab","integration-lab","pedagogy-lab","logos-cycle-engine"],
    "branches":["identity","embodied","oracle","transformation","context","integration"],"centres":7,"centres_are_cymatic_stations":false,
    "practice_extent":{"storeys":12,"decans":3,"strokes":24},"native_meaning":"crates/ql-mef/src/nara/domain; operations do not establish empirical personal-state claims","automatic_agent_or_model_invocation":false,"public_export":false})
}
fn root() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("QL_NARA_HOME") {
        return Ok(path.into());
    }
    let home =
        std::env::var_os("HOME").ok_or("HOME unavailable; select QL_NARA_HOME explicitly")?;
    #[cfg(target_os = "macos")]
    let base = PathBuf::from(home).join("Library/Application Support/QL");
    #[cfg(not(target_os = "macos"))]
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(home).join(".local/state"))
        .join("ql");
    Ok(base.join("nara-private"))
}
/// Personal requests travel on stdin, never process arguments or generic logs.
pub fn command(args: &[String]) -> Result<String, CliError> {
    let result = if args == ["capabilities"] {
        Ok(capabilities())
    } else if args == ["--request-file", "-"] {
        let mut bytes = Vec::new();
        std::io::stdin()
            .take(MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| CliError("could not read private request".into()))?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err(CliError("private request exceeds native bound".into()));
        }
        let request: Request = serde_json::from_slice(&bytes)
            .map_err(|_| CliError("invalid private Nara request; content omitted".into()))?;
        execute(&root().map_err(CliError)?, request)
    } else {
        return Err(CliError(
            "usage: ql nara capabilities --json | ql nara --request-file - --json".into(),
        ));
    };
    let document=result.unwrap_or_else(|reason|json!({"schema":CONTRACT,"ok":false,"error":{"code":"nara_refused","reason":reason},"private":true,"source_mutated":false,"automatic_agent_or_model_invocation":false}));
    serde_json::to_string(&document)
        .map_err(|_| CliError("private Nara response serialization failed".into()))
}
#[cfg(test)]
mod tests;
