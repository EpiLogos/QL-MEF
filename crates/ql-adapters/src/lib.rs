//! Optional client adapters that preserve client-owned identity while attaching QL/MEF readings.
//!
//! Factory and AIKit remain semantic owners of their own refs. QL-MEF refracts those refs; it does
//! not rename them or make QL a hidden prerequisite for ordinary client behaviour.

mod aikit;
mod attachment;
mod client;
mod core;
mod error;
mod factory;
mod runtime;
mod techne;

pub use aikit::AiKitAdapter;
pub use attachment::{AdapterResult, QlAttachment, QlMode};
pub use client::{AdapterSubject, AiKitSubject, ClientRecord, ClientSubject, FactorySubject};
pub use error::AdapterError;
pub use factory::FactoryAdapter;
pub use runtime::{
    RuntimeEnvelope, RuntimeRefractionAdapter, RuntimeSelection, RuntimeStatus, RuntimeSubject,
};
pub use techne::{
    AgencyRole, AgencyRoleKind, ApplicationCutDisclosure, DegradedDisclosure, DisclosureNavigation,
    DisclosureSelection, DisclosureSession, DisclosureSuggestion, DisclosureTimeWindow,
    ExpressionBinding, InstrumentDisclosure, NativeActionRef, PlaceFacet, PlaceGeometry,
    PlaceGeometryType, PlaceHierarchyMember, PlaceIdentity, PlaceName, PlacePrecision,
    QlResultClass, QlWarrant, ReadingSnapshot, SourceProvenance, SourceSelector, SubjectReading,
    TECHNE_CONTRACT, TechneActionRoute, TechneActionRouteReceipt, TechneAdapter, TechneDisclosure,
    TechneInstrument, TechneReading, TechneReadingKind, TechneSubject, TechneWhole, TemporalFacet,
    TemporalInterval, TemporalKind, TemporalPrecision, WarrantedQlReading, WholeRelation,
};

pub(crate) use core::AdapterCore;
