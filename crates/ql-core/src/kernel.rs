use crate::{QlFace, QlFormRef, QlOperator};

pub const KERNEL_VERSION: &str = "0.1.0-q1";
pub const SCHEMA_VERSION: &str = "1.0.0";
pub const HOLOGRAPHIC_KERNEL_CONTRACT_VERSION: &str = "1.1.0";
pub const HOLOGRAPHIC_KERNEL_REFERENCE_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
pub const HOLOGRAPHIC_KERNEL_POINTER_WEB_BLOB: &str = "3eeae6f9c8cc65c5a610df1a49143b3c65bdd320";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QlFamily {
    C = 0,
    P = 1,
    L = 2,
    S = 3,
    T = 4,
    M = 5,
    None = 7,
}

impl QlFamily {
    pub const ALL: [Self; 7] = [
        Self::C,
        Self::P,
        Self::L,
        Self::S,
        Self::T,
        Self::M,
        Self::None,
    ];

    pub const fn value(self) -> u8 {
        self as u8
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::C => "C",
            Self::P => "P",
            Self::L => "L",
            Self::S => "S",
            Self::T => "T",
            Self::M => "M",
            Self::None => "NONE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelRelationId {
    PositionIdentity,
    FamilySamePosition,
    CrossSamePosition,
    PairA,
    PairB,
    PairC,
    CrossTransform,
    CrossRequire,
    CrossComplete,
    ConjugateInvarianceA,
    ConjugateInvarianceB,
    ConjugateInvarianceC,
    MirrorComplement,
    PositionSuccessor,
    MobiusReturn,
    LensAnchor,
    ContextFrame,
    VakCpf,
    VakCt,
    VakCp,
    VakCfp,
    VakCs,
    Nesting,
    Branching,
    SourceProvenance,
}

impl KernelRelationId {
    pub const ALL: [Self; 25] = [
        Self::PositionIdentity,
        Self::FamilySamePosition,
        Self::CrossSamePosition,
        Self::PairA,
        Self::PairB,
        Self::PairC,
        Self::CrossTransform,
        Self::CrossRequire,
        Self::CrossComplete,
        Self::ConjugateInvarianceA,
        Self::ConjugateInvarianceB,
        Self::ConjugateInvarianceC,
        Self::MirrorComplement,
        Self::PositionSuccessor,
        Self::MobiusReturn,
        Self::LensAnchor,
        Self::ContextFrame,
        Self::VakCpf,
        Self::VakCt,
        Self::VakCp,
        Self::VakCfp,
        Self::VakCs,
        Self::Nesting,
        Self::Branching,
        Self::SourceProvenance,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PositionIdentity => "ql.kernel.position.identity/v1",
            Self::FamilySamePosition => "ql.kernel.family.same-position/v1",
            Self::CrossSamePosition => "ql.kernel.cross.same-position/v1",
            Self::PairA => "ql.kernel.pair.A/v1",
            Self::PairB => "ql.kernel.pair.B/v1",
            Self::PairC => "ql.kernel.pair.C/v1",
            Self::CrossTransform => "ql.kernel.cross.transform/v1",
            Self::CrossRequire => "ql.kernel.cross.require/v1",
            Self::CrossComplete => "ql.kernel.cross.complete/v1",
            Self::ConjugateInvarianceA => "ql.kernel.conjugate-invariance.A/v1",
            Self::ConjugateInvarianceB => "ql.kernel.conjugate-invariance.B/v1",
            Self::ConjugateInvarianceC => "ql.kernel.conjugate-invariance.C/v1",
            Self::MirrorComplement => "ql.kernel.mirror.complement/v1",
            Self::PositionSuccessor => "ql.kernel.position.successor/v1",
            Self::MobiusReturn => "ql.kernel.return.mobius/v1",
            Self::LensAnchor => "ql.kernel.lens.anchor/v1",
            Self::ContextFrame => "ql.kernel.context-frame/v1",
            Self::VakCpf => "ql.kernel.vak.cpf/v1",
            Self::VakCt => "ql.kernel.vak.ct/v1",
            Self::VakCp => "ql.kernel.vak.cp/v1",
            Self::VakCfp => "ql.kernel.vak.cfp/v1",
            Self::VakCs => "ql.kernel.vak.cs/v1",
            Self::Nesting => "ql.kernel.nesting/v1",
            Self::Branching => "ql.kernel.branching/v1",
            Self::SourceProvenance => "ql.kernel.source.provenance/v1",
        }
    }
}

/// The six universal VAK reflective instruction families from the frozen C kernel.
///
/// These are language-level operations. Whether an older arena happened to materialise
/// a corresponding reflective pointer slot is separate implementation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VakFamily {
    Cpf = 0,
    Ct = 1,
    Cp = 2,
    Cf = 3,
    Cfp = 4,
    Cs = 5,
}

impl VakFamily {
    pub const ALL: [Self; 6] = [Self::Cpf, Self::Ct, Self::Cp, Self::Cf, Self::Cfp, Self::Cs];

    pub const fn value(self) -> u8 {
        self as u8
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::Cpf => "CPF",
            Self::Ct => "CT",
            Self::Cp => "CP",
            Self::Cf => "CF",
            Self::Cfp => "CFP",
            Self::Cs => "CS",
        }
    }

    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Cpf => "Category-Position-Frame",
            Self::Ct => "Context-Time / Content Types",
            Self::Cp => "Context-Position",
            Self::Cf => "Context-Frame",
            Self::Cfp => "Context-Frame-Position / Paths",
            Self::Cs => "Context-Sequence",
        }
    }

    /// Stable operator identity in the shared kernel contract.
    /// CF is the Context-Frame operator itself, rather than a second CF ontology.
    pub const fn relation_id(self) -> KernelRelationId {
        match self {
            Self::Cpf => KernelRelationId::VakCpf,
            Self::Ct => KernelRelationId::VakCt,
            Self::Cp => KernelRelationId::VakCp,
            Self::Cf => KernelRelationId::ContextFrame,
            Self::Cfp => KernelRelationId::VakCfp,
            Self::Cs => KernelRelationId::VakCs,
        }
    }

    /// Source-grounded M0 specialisation retained as provenance for the universal family.
    pub const fn m0_handler_role(self) -> &'static str {
        match self {
            Self::Cpf => "discrimination/inversion",
            Self::Ct => "QL-frame-selection",
            Self::Cp => "void-arithmetic-position-anchor",
            Self::Cf => "Context-Frame/Vimarsa-invocation",
            Self::Cfp => "R-factor-thread",
            Self::Cs => "Logos-cycle-completion",
        }
    }
}

/// Native form of the historical 5-byte VAK instruction.
/// `inverted` is the source VAK prime suffix and therefore carries direct/prime face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VakInstruction {
    pub family: VakFamily,
    pub index: u8,
    pub target_branch: u8,
    pub target_position: u8,
    pub inverted: bool,
}

impl VakInstruction {
    pub const fn new(
        family: VakFamily,
        index: u8,
        target_branch: u8,
        target_position: u8,
        inverted: bool,
    ) -> Option<Self> {
        if target_branch >= 6 || target_position >= 6 {
            return None;
        }
        Some(Self {
            family,
            index,
            target_branch,
            target_position,
            inverted,
        })
    }

    pub const fn face(self) -> QlFace {
        if self.inverted {
            QlFace::Conjugate
        } else {
            QlFace::Direct
        }
    }

    pub const fn relation_id(self) -> KernelRelationId {
        self.family.relation_id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelCapabilities {
    pub kernel_version: &'static str,
    pub schema_version: &'static str,
    pub supported_forms: [QlFormRef; 3],
    pub deterministic_operations: [QlOperator; 3],
    pub stochastic_operations: [&'static str; 0],
    pub research_operations: [&'static str; 0],
}

pub const fn kernel_capabilities() -> KernelCapabilities {
    KernelCapabilities {
        kernel_version: KERNEL_VERSION,
        schema_version: SCHEMA_VERSION,
        supported_forms: [
            QlFormRef::SIXFOLD_V1,
            QlFormRef::FOUR_PLUS_TWO_V1,
            QlFormRef::DIRECT_CONJUGATE_V1,
        ],
        deterministic_operations: [
            QlOperator::ConjugateAddress,
            QlOperator::ComplementAddress,
            QlOperator::ClassifyFourPlusTwo,
        ],
        stochastic_operations: [],
        research_operations: [],
    }
}
