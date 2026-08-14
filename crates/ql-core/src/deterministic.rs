#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicProvenance {
    pub schema_version: &'static str,
    pub kernel_version: &'static str,
    pub operation: &'static str,
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicResult<T> {
    pub value: T,
    pub provenance: DeterministicProvenance,
}
