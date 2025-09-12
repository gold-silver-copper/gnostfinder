/// Top-level entity
pub trait Entity {}

/// Continuants
pub trait Continuant: Entity {}
pub trait IndependentContinuant: Continuant {}
pub trait GenericallyDependentContinuant: Continuant {}
pub trait SpecificallyDependentContinuant: Continuant {}

/// Independent continuant subtypes
pub trait MaterialEntity: IndependentContinuant {}
pub trait ImmaterialEntity: IndependentContinuant {}

/// Material entities
pub trait Object: MaterialEntity {}
pub trait FiatObjectPart: MaterialEntity {}
pub trait ObjectAggregate: MaterialEntity {}

/// Immaterial entities
pub trait Site: ImmaterialEntity {}
pub trait ContinuantFiatBoundary: ImmaterialEntity {}
pub trait FiatPointBoundary: ContinuantFiatBoundary {}
pub trait FiatLineBoundary: ContinuantFiatBoundary {}
pub trait FiatSurfaceBoundary: ContinuantFiatBoundary {}

/// Spatial regions
pub trait SpatialRegion: ImmaterialEntity {}
pub trait ZeroDimensionalSpatialRegion: SpatialRegion {}
pub trait OneDimensionalSpatialRegion: SpatialRegion {}
pub trait TwoDimensionalSpatialRegion: SpatialRegion {}
pub trait ThreeDimensionalSpatialRegion: SpatialRegion {}

/// Specifically dependent continuants
pub trait Quality: SpecificallyDependentContinuant {}
pub trait RelationalQuality: Quality {}

pub trait RealizableEntity: SpecificallyDependentContinuant {}
pub trait Role: RealizableEntity {}
pub trait Disposition: RealizableEntity {}
pub trait Function: Disposition {}

/// Occurrents
pub trait Occurrent: Entity {}
pub trait Process: Occurrent {}
pub trait ProcessBoundary: Occurrent {}
pub trait TemporalRegion: Occurrent {}
pub trait SpatiotemporalRegion: Occurrent {}

/// Temporal regions
pub trait ZeroDimensionalTemporalRegion: TemporalRegion {}
pub trait OneDimensionalTemporalRegion: TemporalRegion {}

pub trait TemporalInstant: ZeroDimensionalTemporalRegion {}
pub trait TemporalInterval: OneDimensionalTemporalRegion {}

/// BFO Relations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Relation {
    BearerOf,
    Concretizes,
    ContinuantPartOf,
    Environs,
    FirstInstantOf,
    GenericallyDependsOn,
    HasContinuantPart,
    HasFirstInstant,
    HasHistory,
    HasLastInstant,
    HasLocation,
    HasMaterialBasis,
    HasMemberPart,
    HasOccurrentPart,
    HasParticipant,
    HasProperContinuantPart,
    HasProperOccurrentPart,
    HasProperTemporalPart,
    HasRealization,
    HasTemporalPart,
    HistoryOf,
    InheresIn,
    IsCarrierOf,
    IsConcretizedBy,
    LastInstantOf,
    LocatedIn,
    MaterialBasisOf,
    MemberPartOf,
    OccupiesSpatialRegion,
    OccupiesSpatiotemporalRegion,
    OccupiesTemporalRegion,
    OccurrentPartOf,
    OccursIn,
    ParticipatesIn,
    PrecededBy,
    Precedes,
    ProperContinuantPartOf,
    ProperOccurrentPartOf,
    ProperTemporalPartOf,
    Realizes,
    SpatiallyProjectsOnto,
    SpecificallyDependedOnBy,
    SpecificallyDependsOn,
    TemporalPartOf,
    TemporallyProjectsOnto,
}
