//! Ordinary Signal contract for aggregator.
//!
//! This crate carries bounded collection requests, normalized evidence
//! packages, and fine-controlled metadata-first output discovery. Synthesis and
//! review happen in agents after the package or bounded output text is read.
//! Agent-authored output is exposed as artifact provenance, not as design
//! authority.

use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

macro_rules! string_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
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
    ($(#[$meta:meta])* $name:ident, $inner:ty, $getter:ident) => {
        $(#[$meta])*
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
string_newtype!(SubagentName);
string_newtype!(OutputTitle);
string_newtype!(OutputText);
string_newtype!(
    /// Opaque daemon-local session handle. The daemon may reject it as stale or
    /// broken when the underlying transcript or artifact files change.
    FragileSessionReference
);
string_newtype!(
    /// Opaque daemon-local subagent handle. The daemon may reject it as stale or
    /// broken when the underlying transcript or artifact files change.
    FragileSubagentReference
);
string_newtype!(
    /// Opaque daemon-local output handle. The daemon may reject it as stale or
    /// broken when the underlying transcript or artifact files change.
    FragileOutputReference
);
string_newtype!(
    /// Opaque daemon-local output segment handle. The daemon may reject it as
    /// stale or broken when the underlying transcript or artifact files change.
    FragileOutputSegmentReference
);
string_newtype!(
    /// Opaque daemon-local pagination handle. The daemon may reject it as stale
    /// when the listed collection changes.
    FragilePageCursor
);

count_newtype!(DurationAmount, u64, into_u64);
count_newtype!(ByteCount, u64, into_u64);
count_newtype!(ByteLimit, u64, into_u64);
count_newtype!(SegmentLimit, u64, into_u64);
count_newtype!(ItemCount, u64, into_u64);
count_newtype!(LineNumber, u64, into_u64);
count_newtype!(LineCount, u64, into_u64);
count_newtype!(PageLimit, u64, into_u64);
count_newtype!(SegmentIndex, u64, into_u64);

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
pub enum SizeCertainty {
    Exact,
    Estimated,
    Unknown,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SizeMetadata {
    pub byte_count: Option<ByteCount>,
    pub line_count: Option<LineCount>,
    pub segment_count: Option<ItemCount>,
    pub certainty: SizeCertainty,
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
pub enum ListingOrder {
    OldestFirst,
    NewestFirst,
    ReferenceAscending,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PageRequest {
    pub limit: PageLimit,
    pub cursor: Option<FragilePageCursor>,
    pub order: ListingOrder,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PageMetadata {
    pub limit: PageLimit,
    pub returned_items: ItemCount,
    pub total_items: Option<ItemCount>,
    pub next_cursor: Option<FragilePageCursor>,
    pub order: ListingOrder,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum CardProjection {
    MetadataOnly,
    BoundedPreview(BoundedTextProjection),
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
pub enum AuthoredStatus {
    AgentAuthored,
    HumanAuthored,
    MixedAuthorship,
    UnknownAuthorship,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum AuthoredStatusFilter {
    AnyAuthoredStatus,
    OnlyAuthoredStatus(AuthoredStatus),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputProvenance {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub authored_status: AuthoredStatus,
    pub produced_at: Option<Timestamp>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputTextExcerpt {
    pub text: OutputText,
    pub byte_count: ByteCount,
    pub truncation: Option<Truncation>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionCard {
    pub reference: FragileSessionReference,
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub started_at: Option<Timestamp>,
    pub last_observed_at: Option<Timestamp>,
    pub subagent_count: Option<ItemCount>,
    pub output_count: Option<ItemCount>,
    pub size: SizeMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubagentCard {
    pub reference: FragileSubagentReference,
    pub session_reference: FragileSessionReference,
    pub name: SubagentName,
    pub authored_status: AuthoredStatus,
    pub output_count: Option<ItemCount>,
    pub size: SizeMetadata,
    pub first_observed_at: Option<Timestamp>,
    pub last_observed_at: Option<Timestamp>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputCard {
    pub reference: FragileOutputReference,
    pub session_reference: FragileSessionReference,
    pub subagent_reference: Option<FragileSubagentReference>,
    pub title: Option<OutputTitle>,
    pub provenance: OutputProvenance,
    pub size: SizeMetadata,
    pub preview: Option<OutputTextExcerpt>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputSegmentCard {
    pub reference: FragileOutputSegmentReference,
    pub output_reference: FragileOutputReference,
    pub segment_index: SegmentIndex,
    pub byte_range: Option<ByteRange>,
    pub line_range: Option<LineRange>,
    pub size: SizeMetadata,
    pub preview: Option<OutputTextExcerpt>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionListFilter {
    pub source_selection: SourceSelection,
    pub time_window: Option<TimeWindow>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubagentListFilter {
    pub session_reference: FragileSessionReference,
    pub authored_status: AuthoredStatusFilter,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputListFilter {
    pub source_selection: SourceSelection,
    pub session_reference: Option<FragileSessionReference>,
    pub subagent_reference: Option<FragileSubagentReference>,
    pub authored_status: AuthoredStatusFilter,
    pub time_window: Option<TimeWindow>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputSegmentListFilter {
    pub output_reference: FragileOutputReference,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionListRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: SessionListFilter,
    pub page: PageRequest,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubagentListRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: SubagentListFilter,
    pub page: PageRequest,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputListRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: OutputListFilter,
    pub page: PageRequest,
    pub projection: CardProjection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputSegmentListRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: OutputSegmentListFilter,
    pub page: PageRequest,
    pub projection: CardProjection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum OutputReadRange {
    EntireOutput,
    Bytes(ByteRange),
    Lines(LineRange),
    Segment(FragileOutputSegmentReference),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputEstimateRequest {
    pub request_identifier: RequestIdentifier,
    pub output_reference: FragileOutputReference,
    pub range: OutputReadRange,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputReadRequest {
    pub request_identifier: RequestIdentifier,
    pub output_reference: FragileOutputReference,
    pub range: OutputReadRange,
    pub maximum_bytes: ByteLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionsListed {
    pub request_identifier: RequestIdentifier,
    pub sessions: Vec<SessionCard>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubagentsListed {
    pub request_identifier: RequestIdentifier,
    pub subagents: Vec<SubagentCard>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputsListed {
    pub request_identifier: RequestIdentifier,
    pub outputs: Vec<OutputCard>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputSegmentsListed {
    pub request_identifier: RequestIdentifier,
    pub segments: Vec<OutputSegmentCard>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputEstimated {
    pub request_identifier: RequestIdentifier,
    pub output_reference: FragileOutputReference,
    pub range: OutputReadRange,
    pub size: SizeMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputRead {
    pub request_identifier: RequestIdentifier,
    pub output_reference: FragileOutputReference,
    pub range: OutputReadRange,
    pub size: SizeMetadata,
    pub excerpt: OutputTextExcerpt,
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
pub enum OperationRejectionReason {
    Missing,
    FragileReferenceStale,
    FragileReferenceBroken,
    Oversized,
    Unsupported,
    Unauthorized,
    InvalidRequest,
    InvalidRange,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum RejectedFragileReference {
    Session(FragileSessionReference),
    Subagent(FragileSubagentReference),
    Output(FragileOutputReference),
    OutputSegment(FragileOutputSegmentReference),
    PageCursor(FragilePageCursor),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OperationRejected {
    pub request_identifier: RequestIdentifier,
    pub operation: OperationKind,
    pub reason: OperationRejectionReason,
    pub reference: Option<RejectedFragileReference>,
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
        operation ListSessions(SessionListRequest),
        operation ListSubagents(SubagentListRequest),
        operation ListOutputs(OutputListRequest),
        operation ListOutputSegments(OutputSegmentListRequest),
        operation EstimateOutput(OutputEstimateRequest),
        operation ReadOutput(OutputReadRequest),
    }
    reply AggregatorReply {
        EvidenceCollected(EvidencePackage),
        VersionReported(VersionReport),
        EvidenceRejected(EvidenceRejected),
        SessionsListed(SessionsListed),
        SubagentsListed(SubagentsListed),
        OutputsListed(OutputsListed),
        OutputSegmentsListed(OutputSegmentsListed),
        OutputEstimated(OutputEstimated),
        OutputRead(OutputRead),
        OperationRejected(OperationRejected),
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
