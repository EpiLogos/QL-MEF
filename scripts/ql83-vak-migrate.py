#!/usr/bin/env python3
from pathlib import Path

# verification trigger: preserve source nomenclature while proving general O:I binding


def patch(path: str, old: str, new: str) -> None:
    target = Path(path)
    source = target.read_text()
    if old not in source:
        raise SystemExit(f"missing patch anchor in {path}: {old[:100]!r}")
    target.write_text(source.replace(old, new, 1))


vak = "crates/ql-mef/src/vak.rs"
patch(
    vak,
    '''    pub const fn name(self) -> &'static str {\n        match self {\n            Self::Potential => "Potential / In-pression",\n            Self::Distinguish => "Distinguish",\n            Self::Affirm => "Affirm",\n            Self::Relate => "Relate",\n            Self::Contextualise => "Contextualise",\n            Self::Express => "Express",\n        }\n    }\n''',
    '''    pub const fn name(self) -> &'static str {\n        match self {\n            Self::Potential => "Potential / In-pression",\n            Self::Distinguish => "Distinguish",\n            Self::Affirm => "Affirm",\n            Self::Relate => "Relate",\n            Self::Contextualise => "Contextualise",\n            Self::Express => "Express",\n        }\n    }\n\n    pub const fn source_coordinate(self) -> &'static str {\n        match self {\n            Self::Potential => "M0-5-(0/1)-0",\n            Self::Distinguish => "M0-5-(0/1)-1",\n            Self::Affirm => "M0-5-(0/1)-2",\n            Self::Relate => "M0-5-(0/1)-3",\n            Self::Contextualise => "M0-5-(0/1)-4",\n            Self::Express => "M0-5-(0/1)-5",\n        }\n    }\n''',
)
patch(
    vak,
    '''    pub const fn source_symbol(self) -> &'static str {\n        match self {\n            Self::H0 => "##",\n            Self::H1 => "O#",\n            Self::H2 => "X#",\n            Self::H3 => "N#",\n            Self::H4 => "M#",\n            Self::H5 => "R#",\n        }\n    }\n''',
    '''    pub const fn source_symbol(self) -> &'static str {\n        match self {\n            Self::H0 => "##",\n            Self::H1 => "O#",\n            Self::H2 => "X#",\n            Self::H3 => "N#",\n            Self::H4 => "M#",\n            Self::H5 => "R#",\n        }\n    }\n\n    pub const fn source_coordinate(self) -> &'static str {\n        match self {\n            Self::H0 => "M0-5-(5/0)-0",\n            Self::H1 => "M0-5-(5/0)-1",\n            Self::H2 => "M0-5-(5/0)-2",\n            Self::H3 => "M0-5-(5/0)-3",\n            Self::H4 => "M0-5-(5/0)-4",\n            Self::H5 => "M0-5-(5/0)-5",\n        }\n    }\n''',
)
patch(
    vak,
    '''    pub fn bind_operator(&self, operator: VakRelationOp) -> VakOperatorBinding {\n        let support = self\n            .entries()\n            .filter(|entry| entry.raw_source_row.contains(operator.glyph()))\n            .map(|entry| entry.vak_ref.clone())\n            .collect::<Vec<_>>();\n        VakOperatorBinding {\n            operator,\n            standing: VakStanding::DesignCommitment,\n            source_support: support,\n            evidence: vec![format!(\n                "PR #84 architecture binds Śiva position {} to `{}` / {}; source rows remain independently source-backed",\n                operator.position(),\n                operator.glyph(),\n                operator.name()\n            )],\n        }\n    }\n\n    pub fn bind_horizon(&self, horizon: VakAddressHorizon) -> VakHorizonBinding {\n        let support = self\n            .locate_symbol(horizon.source_symbol())\n            .into_iter()\n            .map(|entry| entry.vak_ref.clone())\n            .collect::<Vec<_>>();\n        VakHorizonBinding {\n            horizon,\n            standing: VakStanding::DesignCommitment,\n            source_support: support,\n            evidence: vec![format!(\n                "PR #84 architecture binds {} to source symbol `{}`; matching Vāk entries retain source provenance",\n                horizon.address(),\n                horizon.source_symbol()\n            )],\n        }\n    }\n''',
    '''    pub fn bind_operator(&self, operator: VakRelationOp) -> Result<VakOperatorBinding, VakError> {\n        let source_ref = VakRef::new(operator.source_coordinate())?;\n        let entry = self\n            .locate(&source_ref)\n            .ok_or_else(|| VakError::UnknownRef(source_ref.to_string()))?;\n        if !entry.raw_source_row.contains(operator.glyph()) {\n            return Err(VakError::InvalidRef(format!(\n                "{} does not carry Śiva operator glyph {}",\n                source_ref,\n                operator.glyph()\n            )));\n        }\n        Ok(VakOperatorBinding {\n            operator,\n            standing: VakStanding::ImplementationMapping,\n            source_support: vec![source_ref.clone()],\n            evidence: vec![format!(\n                "PR #84 maps general O:I relation position {} to exact source-backed Śiva node {}: `{}` / {}",\n                operator.position(),\n                source_ref,\n                operator.glyph(),\n                operator.name()\n            )],\n        })\n    }\n\n    pub fn bind_horizon(\n        &self,\n        horizon: VakAddressHorizon,\n    ) -> Result<VakHorizonBinding, VakError> {\n        let source_ref = VakRef::new(horizon.source_coordinate())?;\n        let entry = self\n            .locate(&source_ref)\n            .ok_or_else(|| VakError::UnknownRef(source_ref.to_string()))?;\n        let source_relation = format!("{} = {}", horizon.address(), horizon.source_symbol());\n        if !entry.raw_source_row.contains(&source_relation) {\n            return Err(VakError::InvalidRef(format!(\n                "{} does not carry Śakti horizon relation {}",\n                source_ref, source_relation\n            )));\n        }\n        Ok(VakHorizonBinding {\n            horizon,\n            standing: VakStanding::ImplementationMapping,\n            source_support: vec![source_ref.clone()],\n            evidence: vec![format!(\n                "PR #84 maps general O:I horizon {} to exact source-backed Śakti node {}: {}",\n                horizon.address(), source_ref, source_relation\n            )],\n        })\n    }\n''',
)

tests = "crates/ql-mef/tests/vak_language.rs"
patch(
    tests,
    '''        assert!(entry.raw_source_row.contains(source_glyph));\n        assert!(entry.raw_source_row.contains(operator.name()));\n        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n''',
    '''        assert!(entry.raw_source_row.contains(source_glyph));\n        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n''',
)
patch(
    tests,
    '''        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n    }\n}\n\n#[test]\nfn shakti_sixfold_is_present_at_exact_source_coordinates() {\n''',
    '''        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n        let binding = registry.bind_operator(operator).unwrap();\n        assert_eq!(binding.standing, VakStanding::ImplementationMapping);\n        assert_eq!(binding.source_support, vec![VakRef::new(coordinate).unwrap()]);\n    }\n}\n\n#[test]\nfn shakti_sixfold_is_present_at_exact_source_coordinates() {\n''',
)
patch(
    tests,
    '''        assert!(entry.raw_source_row.contains(horizon.source_symbol()));\n        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n    }\n}\n\n#[test]\nfn self_other_language_parses_generates_and_returns_to_exact_source_nodes() {\n''',
    '''        assert!(entry.raw_source_row.contains(horizon.source_symbol()));\n        assert_eq!(entry.source.standing, VakStanding::SourceBacked);\n        let binding = registry.bind_horizon(horizon).unwrap();\n        assert_eq!(binding.standing, VakStanding::ImplementationMapping);\n        assert_eq!(binding.source_support, vec![VakRef::new(coordinate).unwrap()]);\n    }\n}\n\n#[test]\nfn self_other_language_parses_generates_and_returns_to_exact_source_nodes() {\n''',
)
patch(
    tests,
    '''    for form in SelfOtherForm::ALL {\n        assert!(children.contains(&&form.source_ref()));\n    }\n\n    let neighbourhood = registry.neighbourhood(&centre, 1).unwrap();\n    assert_eq!(neighbourhood.entries.len(), 13);\n    assert_eq!(neighbourhood.relations.len(), 13);\n''',
    '''    for form in SelfOtherForm::ALL {\n        assert!(\n            children\n                .iter()\n                .any(|child| child.as_str() == form.source_coordinate())\n        );\n    }\n\n    let neighbourhood = registry.neighbourhood(&centre, 1).unwrap();\n    // Centre + parent + twelve direct grammar children.\n    assert_eq!(neighbourhood.entries.len(), 14);\n    assert_eq!(neighbourhood.relations.len(), 13);\n''',
)
