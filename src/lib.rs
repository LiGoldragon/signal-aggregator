//! Ordinary Signal contract for aggregator.
//!
//! This crate carries bounded collection requests and normalized evidence
//! packages. Synthesis and review happen in agents after the package is read.

use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaEncode,
            NotaDecode,
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

macro_rules! count_newtype {
    ($name:ident, $inner:ty, $getter:ident) => {
        #[derive(
            Archive,
            RkyvSerialize,
            RkyvDeserialize,
            NotaEncode,
            NotaDecode,
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
        )]
        pub struct $name($inner);

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn $getter(self) -> $inner {
                self.0
            }
        }
    };
}

string_newtype!(RequestIdentifier);
string_newtype!(PackageIdentifier);
string_newtype!(Timestamp);
string_newtype!(FilesystemPath);
string_newtype!(SourceIdentifier);
string_newtype!(RepositoryIdentifier);
string_newtype!(CommitIdentifier);
string_newtype!(RepositoryPath);
string_newtype!(TranscriptSegmentIdentifier);
string_newtype!(TranscriptText);
string_newtype!(ContractName);
string_newtype!(ContractVersion);

count_newtype!(DurationAmount, u64, into_u64);
count_newtype!(ByteCount, u64, into_u64);
count_newtype!(ByteLimit, u64, into_u64);
count_newtype!(SegmentLimit, u64, into_u64);
count_newtype!(ItemCount, u64, into_u64);
count_newtype!(LineNumber, u64, into_u64);

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum DurationUnit {
    Minutes,
    Hours,
    Days,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RelativeDuration {
    pub amount: DurationAmount,
    pub unit: DurationUnit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TimeWindow {
    Recent(RelativeDuration),
    Range(TimeRange),
    Since(Timestamp),
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SourceKind {
    Claude,
    Codex,
    Pi,
    Repository,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SelectedSources {
    pub sources: Vec<SourceKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum SourceSelection {
    AllConfigured,
    Only(SelectedSources),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct BoundedTextProjection {
    pub maximum_bytes: ByteLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum Projection {
    MetadataOnly,
    IdentifiersOnly,
    BoundedText(BoundedTextProjection),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LimitPolicy {
    pub maximum_segments: SegmentLimit,
    pub maximum_bytes: ByteLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EvidenceRequest {
    pub request_identifier: RequestIdentifier,
    pub time_window: TimeWindow,
    pub source_selection: SourceSelection,
    pub projection: Projection,
    pub limit_policy: LimitPolicy,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SourceVolume {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub item_count: ItemCount,
    pub byte_count: ByteCount,
    pub earliest_timestamp: Option<Timestamp>,
    pub latest_timestamp: Option<Timestamp>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LineRange {
    pub start: LineNumber,
    pub end: LineNumber,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByteRange {
    pub start: ByteCount,
    pub end: ByteCount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptTextExcerpt {
    pub text: TranscriptText,
    pub byte_count: ByteCount,
    pub truncation: Option<Truncation>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum SegmentProjection {
    MetadataOnly,
    IdentifiersOnly,
    Text(TranscriptTextExcerpt),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptSegment {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub segment_identifier: TranscriptSegmentIdentifier,
    pub path: FilesystemPath,
    pub timestamp: Option<Timestamp>,
    pub line_range: Option<LineRange>,
    pub byte_range: Option<ByteRange>,
    pub projection: SegmentProjection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum RepositoryWorktreeState {
    Clean,
    HasChanges,
    NotObserved,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RepositoryChange {
    pub repository: RepositoryIdentifier,
    pub path: FilesystemPath,
    pub commit_identifier: Option<CommitIdentifier>,
    pub commit_timestamp: Option<Timestamp>,
    pub changed_paths: Vec<RepositoryPath>,
    pub worktree_state: RepositoryWorktreeState,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TruncationReason {
    RequestLimit,
    SourceLimit,
    ProjectionLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Truncation {
    pub source: SourceKind,
    pub path: Option<FilesystemPath>,
    pub original_bytes: Option<ByteCount>,
    pub projected_bytes: ByteCount,
    pub reason: TruncationReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ReadFailureReason {
    Missing,
    PermissionDenied,
    Malformed,
    UnsupportedFormat,
    IoFailure,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReadFailure {
    pub source: SourceKind,
    pub path: Option<FilesystemPath>,
    pub source_identifier: Option<SourceIdentifier>,
    pub reason: ReadFailureReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EvidencePackage {
    pub package_identifier: PackageIdentifier,
    pub request_identifier: RequestIdentifier,
    pub time_window: TimeWindow,
    pub collected_at: Timestamp,
    pub source_volumes: Vec<SourceVolume>,
    pub transcript_segments: Vec<TranscriptSegment>,
    pub repository_changes: Vec<RepositoryChange>,
    pub truncations: Vec<Truncation>,
    pub read_failures: Vec<ReadFailure>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Version {
    pub client_name: Option<ContractName>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct VersionReport {
    pub contract_name: ContractName,
    pub contract_version: ContractVersion,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RejectionReason {
    InvalidTimeWindow,
    UnsupportedProjection,
    LimitExceeded,
    ConfigurationUnavailable,
    CollectionUnavailable,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EvidenceRejected {
    pub request_identifier: RequestIdentifier,
    pub operation: OperationKind,
    pub reason: RejectionReason,
}

signal_channel! {
    channel Aggregator {
        operation Collect(EvidenceRequest),
        operation Version(Version),
    }
    reply AggregatorReply {
        EvidenceCollected(EvidencePackage),
        VersionReported(VersionReport),
        EvidenceRejected(EvidenceRejected),
    }
}

pub type AggregatorRequest = Operation;
pub type AggregatorOperationKind = OperationKind;
pub type AggregatorFrame = Frame;
pub type AggregatorFrameBody = FrameBody;
pub type AggregatorReplyEnvelope = ReplyEnvelope;

impl AggregatorRequest {
    pub fn operation_kind(&self) -> AggregatorOperationKind {
        self.kind()
    }
}
