use ql_core::{KERNEL_VERSION, SCHEMA_VERSION, kernel_capabilities};

#[test]
fn q1_capabilities_advertise_only_deterministic_surface() {
    let capabilities = kernel_capabilities();
    assert_eq!(capabilities.kernel_version, KERNEL_VERSION);
    assert_eq!(capabilities.schema_version, SCHEMA_VERSION);
    assert_eq!(capabilities.supported_forms.len(), 3);
    assert_eq!(capabilities.deterministic_operations.len(), 3);
    assert!(capabilities.stochastic_operations.is_empty());
    assert!(capabilities.research_operations.is_empty());
}
