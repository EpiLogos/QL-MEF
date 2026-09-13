//! Versioned S5 -> QL-MEF self-reference required by the agent-world lock.
//!
//! Same-index S5 affinity is not exclusive ownership. This is an explicit
//! reference to the product/revision implementing and describing the selected
//! kernel, not a copied kernel or an inferred capability assignment.

use serde::{Deserialize, Serialize};

use crate::m_tree::MRegistry;
use crate::property::SourcePin;

pub const S5_SELF_REFERENCE_CONTRACT: &str = "ql.aw1-s5-self-reference/v1";
pub const S5_ORGAN_REF: &str = "S5";
pub const QL_MEF_PRODUCT_REF: &str = "EpiLogos/QL-MEF";

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5ProductSelfReference {
    pub contract: String,
    pub organ_ref: String,
    pub product_ref: String,
    pub product_revision: String,
    pub registry_revision: String,
    pub source: SourcePin,
    pub affinity_only: bool,
}

fn nonempty(value: &str, what: &str) -> Result<()> {
    if value.is_empty() || value != value.trim() || value.contains('\0') {
        Err(what.into())
    } else {
        Ok(())
    }
}

pub fn s5_ql_mef_self_reference(
    registry: &MRegistry,
    product_revision: impl Into<String>,
    source: SourcePin,
) -> Result<S5ProductSelfReference> {
    let product_revision = product_revision.into();
    nonempty(&product_revision, "S5 self-reference requires a product revision")?;
    source.validate()?;
    Ok(S5ProductSelfReference {
        contract: S5_SELF_REFERENCE_CONTRACT.into(),
        organ_ref: S5_ORGAN_REF.into(),
        product_ref: QL_MEF_PRODUCT_REF.into(),
        product_revision,
        registry_revision: registry.manifest().registry_revision.clone(),
        source,
        affinity_only: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::m_tree::native_m_registry;
    use crate::property::SourceUse;

    fn source() -> SourcePin {
        SourcePin {
            reference: "docs/kernel-rebuild/PRE-K8-AGENT-WORLD-LOCK.md".into(),
            revision: "owner-ratified-2026-09-12".into(),
            use_mode: SourceUse::Current,
        }
    }

    #[test]
    fn s5_is_a_versioned_affinity_not_an_exclusive_owner_claim() {
        let registry = native_m_registry();
        let reference = s5_ql_mef_self_reference(registry, "commit:abc123", source()).unwrap();
        assert_eq!(reference.organ_ref, "S5");
        assert_eq!(reference.product_ref, "EpiLogos/QL-MEF");
        assert_eq!(reference.product_revision, "commit:abc123");
        assert_eq!(
            reference.registry_revision,
            registry.manifest().registry_revision
        );
        assert!(reference.affinity_only);
    }

    #[test]
    fn unversioned_or_unsourced_self_reference_is_rejected() {
        let registry = native_m_registry();
        assert!(s5_ql_mef_self_reference(registry, "", source()).is_err());
        let mut missing = source();
        missing.reference.clear();
        assert!(s5_ql_mef_self_reference(registry, "commit:abc123", missing).is_err());
    }
}
