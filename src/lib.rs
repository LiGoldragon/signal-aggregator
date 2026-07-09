//! Ordinary Signal contract for aggregator.
//!
//! This crate carries bounded collection requests, normalized evidence
//! packages, fine-controlled metadata-first output discovery, and structured
//! transcript block search/read requests. Synthesis and review happen in agents
//! after the package, bounded output text, or bounded transcript block text is read.
//! Agent-authored output is exposed as artifact provenance, not as design
//! authority.

use nota::{NotaDecode, NotaEncode};
pub use nota_text_query::{MatchEvidence as TextMatchEvidence, Query as TextQuery};
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
string_newtype!(SessionIdentifier);
string_newtype!(TaskIdentifier);
string_newtype!(TaskTitle);
string_newtype!(ToolUseIdentifier);
string_newtype!(TaskResult);
string_newtype!(UsageSummary);
string_newtype!(RootRelativePath);
string_newtype!(ArchivePath);
string_newtype!(ArchiveRecordIdentifier);
string_newtype!(ArchiveSummaryText);
string_newtype!(ArchiveProvenanceText);
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
    /// Opaque daemon-local transcript block handle. The daemon may reject it as
    /// stale or broken when the underlying transcript files change.
    FragileTranscriptBlockReference
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
count_newtype!(TranscriptBlockIndex, u64, into_u64);

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
    ClaudeSubagentOutput,
    Codex,
    Pi,
    PiSubagentOutput,
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

/// Half-open line interval over a text projection. `start` is the first
/// included one-based line number and `end` is the first excluded one-based line
/// number. A range with `end < start` is invalid.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LineRange {
    pub start: LineNumber,
    pub end: LineNumber,
}

/// Half-open byte interval over UTF-8 text bytes. `start` is the first included
/// zero-based byte offset and `end` is the first excluded zero-based byte offset.
/// The selected byte count is `end - start` when the range is valid; a range
/// with `end < start` is invalid.
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

/// Truncation fact for a bounded text projection. `original_bytes` names the
/// full source byte count when known; `projected_bytes` names the UTF-8 bytes
/// delivered after the projection, range, and limit are applied. For an excerpt
/// carrying this record, `projected_bytes` matches the excerpt `byte_count`.
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

/// Deterministic order for a paged listing.
///
/// The chronology key is session `last_observed_at` falling back to
/// `started_at`, subagent `last_observed_at` falling back to `first_observed_at`,
/// output `provenance.produced_at`, and output segment `segment_index`. Missing
/// timestamp keys sort after present timestamp keys in both chronological
/// directions.
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
    /// Ascending chronology key; equal keys break by fragile reference ascending.
    OldestFirst,
    /// Descending chronology key; equal keys break by fragile reference ascending.
    NewestFirst,
    /// Ascending filesystem last-modified key; equal keys break by fragile reference ascending.
    OldestModifiedFirst,
    /// Descending filesystem last-modified key; equal keys break by fragile reference ascending.
    NewestModifiedFirst,
    /// Fragile reference ascending, with no chronology key in the sort.
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
pub enum SessionRole {
    MainSession,
    SubagentOutputSession,
    Unknown,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionCard {
    pub reference: FragileSessionReference,
    pub role: SessionRole,
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub producer_session_identifier: Option<SessionIdentifier>,
    pub transcript_locator: Option<SourceLocator>,
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
    pub task: Option<SubagentTaskMetadata>,
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
    pub task: Option<SubagentTaskMetadata>,
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
pub struct TranscriptBlockTextQuery(TextQuery);

impl TranscriptBlockTextQuery {
    pub fn new(query: TextQuery) -> Self {
        Self(query)
    }

    pub fn as_query(&self) -> &TextQuery {
        &self.0
    }

    pub fn into_query(self) -> TextQuery {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockSearchEvidence(TextMatchEvidence);

impl TranscriptBlockSearchEvidence {
    pub fn new(evidence: TextMatchEvidence) -> Self {
        Self(evidence)
    }

    pub fn as_evidence(&self) -> &TextMatchEvidence {
        &self.0
    }

    pub fn into_evidence(self) -> TextMatchEvidence {
        self.0
    }
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
pub enum TranscriptBlockKind {
    UserPrompt,
    AgentResponse,
    ToolCall,
    ToolResult,
    Inference,
    SystemInstruction,
    Attachment,
    SessionEvent,
    Unclassified,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SelectedTranscriptBlockKinds {
    pub kinds: Vec<TranscriptBlockKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TranscriptBlockKindSelection {
    AllTranscriptBlockKinds,
    OnlyTranscriptBlockKinds(SelectedTranscriptBlockKinds),
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
pub enum TranscriptBlockTextAvailability {
    ReadableText,
    UnavailableText,
    EncryptedText,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SourceLocator {
    pub root: FilesystemPath,
    pub relative_path: Option<RootRelativePath>,
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
pub enum SourceHealthStatus {
    ReadableEmpty,
    ReadableIndexed,
    UnreadableRoot,
    DiscoveryTruncated,
    MalformedRecords,
    IndexStoreUnreadable,
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
pub enum SessionInventoryCompleteness {
    Complete,
    Resumable,
    Truncated,
    Failed,
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
pub enum SessionLifecycleStatus {
    Current,
    PreviouslyObserved,
    SourceMissing,
    SourceBroken,
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
pub enum SessionArchiveStatus {
    NotArchived,
    Archived,
    ArchiveUnknown,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ScanLimitKind {
    ScanEntries,
    DiscoveredFiles,
    FileBytes,
    LineBytes,
    ReadFailures,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ScanLimitReport {
    pub kind: ScanLimitKind,
    pub limit: ItemCount,
    pub path: Option<FilesystemPath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionInventorySourceReport {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub locator: SourceLocator,
    pub completeness: SessionInventoryCompleteness,
    pub scan_limits: Vec<ScanLimitReport>,
    pub discovered_files: ItemCount,
    pub indexed_sessions: ItemCount,
    pub byte_count: ByteCount,
    pub earliest_modified_at: Option<Timestamp>,
    pub latest_modified_at: Option<Timestamp>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionInventoryScanReport {
    pub sources: Vec<SessionInventorySourceReport>,
    pub total_sessions: ItemCount,
    pub completeness: SessionInventoryCompleteness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionInventoryCard {
    pub reference: FragileSessionReference,
    pub role: SessionRole,
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub producer_session_identifier: Option<SessionIdentifier>,
    pub locator: SourceLocator,
    pub file_count: ItemCount,
    pub byte_count: ByteCount,
    pub earliest_modified_at: Option<Timestamp>,
    pub latest_modified_at: Option<Timestamp>,
    pub started_at: Option<Timestamp>,
    pub last_observed_at: Option<Timestamp>,
    pub subagent_count: Option<ItemCount>,
    pub output_count: Option<ItemCount>,
    pub lifecycle_status: SessionLifecycleStatus,
    pub source_status: SourceHealthStatus,
    pub archive_status: SessionArchiveStatus,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubagentTaskMetadata {
    pub task_identifier: TaskIdentifier,
    pub title: Option<TaskTitle>,
    pub tool_use_identifier: Option<ToolUseIdentifier>,
    pub output_locator: Option<SourceLocator>,
    pub source_status: SourceHealthStatus,
    pub result: Option<TaskResult>,
    pub usage: Option<UsageSummary>,
    pub duration: Option<RelativeDuration>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockProvenance {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub authored_status: AuthoredStatus,
    pub observed_at: Option<Timestamp>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockCard {
    pub reference: FragileTranscriptBlockReference,
    pub session_reference: FragileSessionReference,
    pub subagent_reference: Option<FragileSubagentReference>,
    pub task: Option<SubagentTaskMetadata>,
    pub kind: TranscriptBlockKind,
    pub block_index: TranscriptBlockIndex,
    pub provenance: TranscriptBlockProvenance,
    pub line_range: Option<LineRange>,
    pub byte_range: Option<ByteRange>,
    pub size: SizeMetadata,
    pub text_availability: TranscriptBlockTextAvailability,
    pub preview: Option<TranscriptTextExcerpt>,
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
    pub task_identifier: Option<TaskIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OutputListFilter {
    pub source_selection: SourceSelection,
    pub session_reference: Option<FragileSessionReference>,
    pub subagent_reference: Option<FragileSubagentReference>,
    pub task_identifier: Option<TaskIdentifier>,
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
pub struct TranscriptBlockFilter {
    pub source_selection: SourceSelection,
    pub session_reference: Option<FragileSessionReference>,
    pub subagent_reference: Option<FragileSubagentReference>,
    pub task_identifier: Option<TaskIdentifier>,
    pub kind_selection: TranscriptBlockKindSelection,
    pub authored_status: AuthoredStatusFilter,
    pub time_window: Option<TimeWindow>,
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
pub struct TranscriptBlockListRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: TranscriptBlockFilter,
    pub page: PageRequest,
    pub projection: CardProjection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockSearchRequest {
    pub request_identifier: RequestIdentifier,
    pub filter: TranscriptBlockFilter,
    pub query: TranscriptBlockTextQuery,
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
pub struct TranscriptBlockEstimateRequest {
    pub request_identifier: RequestIdentifier,
    pub block_reference: FragileTranscriptBlockReference,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockReadRequest {
    pub request_identifier: RequestIdentifier,
    pub block_reference: FragileTranscriptBlockReference,
    pub maximum_bytes: ByteLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RuntimeHealthRequest {
    pub request_identifier: RequestIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionInventoryRequest {
    pub request_identifier: RequestIdentifier,
    pub source_selection: SourceSelection,
    pub archive_path: Option<ArchivePath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum SessionLookupSelector {
    ByReference(FragileSessionReference),
    ByProducerSession(SessionIdentifier),
    BySourceLocator(SourceLocator),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionLookupRequest {
    pub request_identifier: RequestIdentifier,
    pub selector: SessionLookupSelector,
    pub archive_path: Option<ArchivePath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveRecordDraft {
    pub session: SessionInventoryCard,
    pub summary: ArchiveSummaryText,
    pub provenance: ArchiveProvenanceText,
    pub created_at: Timestamp,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveWriteRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub record: SessionArchiveRecordDraft,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveQueryRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub session_reference: Option<FragileSessionReference>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveReadRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub record_identifier: ArchiveRecordIdentifier,
    pub maximum_summary_bytes: ByteLimit,
    pub maximum_provenance_bytes: ByteLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveRecordCard {
    pub record_identifier: ArchiveRecordIdentifier,
    pub session_reference: FragileSessionReference,
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub producer_session_identifier: Option<SessionIdentifier>,
    pub created_at: Timestamp,
    pub summary_bytes: ByteCount,
    pub provenance_bytes: ByteCount,
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
pub enum ArchiveTextCompleteness {
    Complete,
    Truncated,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveTextProjection {
    pub text: ArchiveSummaryText,
    pub byte_count: ByteCount,
    pub completeness: ArchiveTextCompleteness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveProvenanceProjection {
    pub text: ArchiveProvenanceText,
    pub byte_count: ByteCount,
    pub completeness: ArchiveTextCompleteness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveRecordProjection {
    pub card: SessionArchiveRecordCard,
    pub session: SessionInventoryCard,
    pub summary: SessionArchiveTextProjection,
    pub provenance: SessionArchiveProvenanceProjection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionsInventoried {
    pub request_identifier: RequestIdentifier,
    pub sessions: Vec<SessionInventoryCard>,
    pub scan_report: SessionInventoryScanReport,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionLookedUp {
    pub request_identifier: RequestIdentifier,
    pub sessions: Vec<SessionInventoryCard>,
    pub scan_report: SessionInventoryScanReport,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveWritten {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub card: SessionArchiveRecordCard,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveQueried {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub records: Vec<SessionArchiveRecordCard>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionArchiveRead {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub record: SessionArchiveRecordProjection,
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
pub enum RuntimeCapabilityStatus {
    Supported,
    Unsupported,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RuntimeCapabilities {
    pub health_observation: RuntimeCapabilityStatus,
    pub transcript_only_configuration: RuntimeCapabilityStatus,
    pub claude_subagent_output_sources: RuntimeCapabilityStatus,
    pub pi_subagent_output_sources: RuntimeCapabilityStatus,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SourceHealthCard {
    pub source: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub locator: SourceLocator,
    pub status: SourceHealthStatus,
    pub scan_limits: Vec<ScanLimitReport>,
    pub discovered_files: ItemCount,
    pub indexed_records: ItemCount,
    pub malformed_records: ItemCount,
    pub unreadable_records: ItemCount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct IndexHealth {
    pub status: SourceHealthStatus,
    pub session_count: ItemCount,
    pub subagent_count: ItemCount,
    pub output_count: ItemCount,
    pub transcript_block_count: ItemCount,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RuntimeHealthObserved {
    pub request_identifier: RequestIdentifier,
    pub capabilities: RuntimeCapabilities,
    pub sources: Vec<SourceHealthCard>,
    pub index: IndexHealth,
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
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlocksListed {
    pub request_identifier: RequestIdentifier,
    pub blocks: Vec<TranscriptBlockCard>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockSearchMatch {
    pub card: TranscriptBlockCard,
    pub evidence: TranscriptBlockSearchEvidence,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlocksSearched {
    pub request_identifier: RequestIdentifier,
    pub matches: Vec<TranscriptBlockSearchMatch>,
    pub page: PageMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockEstimated {
    pub request_identifier: RequestIdentifier,
    pub block_reference: FragileTranscriptBlockReference,
    pub size: SizeMetadata,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptBlockRead {
    pub request_identifier: RequestIdentifier,
    pub block_reference: FragileTranscriptBlockReference,
    pub size: SizeMetadata,
    pub excerpt: TranscriptTextExcerpt,
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
    InvalidQuery,
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
    TranscriptBlock(FragileTranscriptBlockReference),
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
        operation ObserveHealth(RuntimeHealthRequest),
        operation InventorySessions(SessionInventoryRequest),
        operation LookupSession(SessionLookupRequest),
        operation WriteSessionArchive(SessionArchiveWriteRequest),
        operation QuerySessionArchive(SessionArchiveQueryRequest),
        operation ReadSessionArchive(SessionArchiveReadRequest),
        operation ListSessions(SessionListRequest),
        operation ListSubagents(SubagentListRequest),
        operation ListOutputs(OutputListRequest),
        operation ListOutputSegments(OutputSegmentListRequest),
        operation EstimateOutput(OutputEstimateRequest),
        operation ReadOutput(OutputReadRequest),
        operation ListTranscriptBlocks(TranscriptBlockListRequest),
        operation SearchTranscriptBlocks(TranscriptBlockSearchRequest),
        operation EstimateTranscriptBlock(TranscriptBlockEstimateRequest),
        operation ReadTranscriptBlock(TranscriptBlockReadRequest),
    }
    reply AggregatorReply {
        EvidenceCollected(EvidencePackage),
        VersionReported(VersionReport),
        RuntimeHealthObserved(RuntimeHealthObserved),
        SessionsInventoried(SessionsInventoried),
        SessionLookedUp(SessionLookedUp),
        SessionArchiveWritten(SessionArchiveWritten),
        SessionArchiveQueried(SessionArchiveQueried),
        SessionArchiveRead(SessionArchiveRead),
        EvidenceRejected(EvidenceRejected),
        SessionsListed(SessionsListed),
        SubagentsListed(SubagentsListed),
        OutputsListed(OutputsListed),
        OutputSegmentsListed(OutputSegmentsListed),
        OutputEstimated(OutputEstimated),
        OutputRead(OutputRead),
        OperationRejected(OperationRejected),
        TranscriptBlocksListed(TranscriptBlocksListed),
        TranscriptBlocksSearched(TranscriptBlocksSearched),
        TranscriptBlockEstimated(TranscriptBlockEstimated),
        TranscriptBlockRead(TranscriptBlockRead),
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
