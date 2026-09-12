#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type RequestIdentifier = String;
#[rustfmt::skip]
pub type PackageIdentifier = String;
#[rustfmt::skip]
pub type Timestamp = String;
#[rustfmt::skip]
pub type FilesystemPath = String;
#[rustfmt::skip]
pub type SourceIdentifier = String;
#[rustfmt::skip]
pub type RepositoryIdentifier = String;
#[rustfmt::skip]
pub type CommitIdentifier = String;
#[rustfmt::skip]
pub type RepositoryPath = String;
#[rustfmt::skip]
pub type TranscriptSegmentIdentifier = String;
#[rustfmt::skip]
pub type TranscriptText = String;
#[rustfmt::skip]
pub type ContractName = String;
#[rustfmt::skip]
pub type ContractVersion = String;
#[rustfmt::skip]
pub type SubagentName = String;
#[rustfmt::skip]
pub type OutputTitle = String;
#[rustfmt::skip]
pub type OutputText = String;
#[rustfmt::skip]
pub type SessionIdentifier = String;
#[rustfmt::skip]
pub type TaskIdentifier = String;
#[rustfmt::skip]
pub type TaskTitle = String;
#[rustfmt::skip]
pub type ToolUseIdentifier = String;
#[rustfmt::skip]
pub type TaskResult = String;
#[rustfmt::skip]
pub type UsageSummary = String;
#[rustfmt::skip]
pub type RootRelativePath = String;
#[rustfmt::skip]
pub type ArchivePath = String;
#[rustfmt::skip]
pub type ArchiveRecordIdentifier = String;
#[rustfmt::skip]
pub type ArchiveSummaryText = String;
#[rustfmt::skip]
pub type ArchiveProvenanceText = String;
#[rustfmt::skip]
pub type FragileSessionReference = String;
#[rustfmt::skip]
pub type FragileSubagentReference = String;
#[rustfmt::skip]
pub type FragileOutputReference = String;
#[rustfmt::skip]
pub type FragileOutputSegmentReference = String;
#[rustfmt::skip]
pub type FragileTranscriptBlockReference = String;
#[rustfmt::skip]
pub type FragilePageCursor = String;
#[rustfmt::skip]
pub type DurationAmount = i64;
#[rustfmt::skip]
pub type ByteCount = i64;
#[rustfmt::skip]
pub type ByteLimit = i64;
#[rustfmt::skip]
pub type SegmentLimit = i64;
#[rustfmt::skip]
pub type ItemCount = i64;
#[rustfmt::skip]
pub type LineNumber = i64;
#[rustfmt::skip]
pub type LineCount = i64;
#[rustfmt::skip]
pub type PageLimit = i64;
#[rustfmt::skip]
pub type SegmentIndex = i64;
#[rustfmt::skip]
pub type TranscriptBlockIndex = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OperationKind {
    Collect,
    Version,
    ObserveHealth,
    InventorySessions,
    LookupSession,
    WriteSessionArchive,
    QuerySessionArchive,
    ReadSessionArchive,
    ListSessions,
    ListSubagents,
    ListOutputs,
    ListOutputSegments,
    EstimateOutput,
    ReadOutput,
    ListTranscriptBlocks,
    SearchTranscriptBlocks,
    EstimateTranscriptBlock,
    ReadTranscriptBlock,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DurationUnit {
    Minutes,
    Hours,
    Days,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RelativeDuration {
    pub duration_amount: DurationAmount,
    pub duration_unit: DurationUnit,
}
#[rustfmt::skip]
pub type StartTimestamp = Timestamp;
#[rustfmt::skip]
pub type EndTimestamp = Timestamp;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TimeRange {
    pub start_timestamp: StartTimestamp,
    pub end_timestamp: EndTimestamp,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TimeWindow {
    Recent(RelativeDuration),
    Range(TimeRange),
    Since(Timestamp),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SourceKind {
    Claude,
    ClaudeSubagentOutput,
    Codex,
    Pi,
    PiSubagentOutput,
    Repository,
}
#[rustfmt::skip]
pub type SourceKinds = std::vec::Vec<SourceKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SelectedSources {
    pub source_kinds: SourceKinds,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SourceSelection {
    AllConfigured,
    Only(SelectedSources),
}
#[rustfmt::skip]
pub type MaximumBytes = ByteLimit;
#[rustfmt::skip]
pub type MaximumSegments = SegmentLimit;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct BoundedTextProjection {
    pub maximum_bytes: MaximumBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Projection {
    MetadataOnly,
    IdentifiersOnly,
    BoundedText(BoundedTextProjection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LimitPolicy {
    pub maximum_segments: MaximumSegments,
    pub maximum_bytes: MaximumBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EvidenceRequest {
    pub request_identifier: RequestIdentifier,
    pub time_window: TimeWindow,
    pub source_selection: SourceSelection,
    pub projection: Projection,
    pub limit_policy: LimitPolicy,
}
#[rustfmt::skip]
pub type EarliestTimestamp = std::option::Option<Timestamp>;
#[rustfmt::skip]
pub type LatestTimestamp = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SourceVolume {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub item_count: ItemCount,
    pub byte_count: ByteCount,
    pub earliest_timestamp: EarliestTimestamp,
    pub latest_timestamp: LatestTimestamp,
}
#[rustfmt::skip]
pub type StartLineNumber = LineNumber;
#[rustfmt::skip]
pub type EndLineNumber = LineNumber;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct LineRange {
    pub start_line_number: StartLineNumber,
    pub end_line_number: EndLineNumber,
}
#[rustfmt::skip]
pub type StartByteCount = ByteCount;
#[rustfmt::skip]
pub type EndByteCount = ByteCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ByteRange {
    pub start_byte_count: StartByteCount,
    pub end_byte_count: EndByteCount,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TruncationReason {
    RequestLimit,
    SourceLimit,
    ProjectionLimit,
}
#[rustfmt::skip]
pub type OriginalBytes = std::option::Option<ByteCount>;
#[rustfmt::skip]
pub type ProjectedBytes = ByteCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Truncation {
    pub source_kind: SourceKind,
    pub filesystem_path_option: Option<FilesystemPath>,
    pub original_bytes: OriginalBytes,
    pub projected_bytes: ProjectedBytes,
    pub truncation_reason: TruncationReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptTextExcerpt {
    pub transcript_text: TranscriptText,
    pub byte_count: ByteCount,
    pub truncation_option: Option<Truncation>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SegmentProjection {
    MetadataOnly,
    IdentifiersOnly,
    Text(TranscriptTextExcerpt),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptSegment {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub transcript_segment_identifier: TranscriptSegmentIdentifier,
    pub filesystem_path: FilesystemPath,
    pub timestamp_option: Option<Timestamp>,
    pub line_range_option: Option<LineRange>,
    pub byte_range_option: Option<ByteRange>,
    pub segment_projection: SegmentProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RepositoryWorktreeState {
    Clean,
    HasChanges,
    NotObserved,
}
#[rustfmt::skip]
pub type ChangedPaths = std::vec::Vec<RepositoryPath>;
#[rustfmt::skip]
pub type CommitTimestamp = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RepositoryChange {
    pub repository_identifier: RepositoryIdentifier,
    pub filesystem_path: FilesystemPath,
    pub commit_identifier_option: Option<CommitIdentifier>,
    pub commit_timestamp: CommitTimestamp,
    pub changed_paths: ChangedPaths,
    pub repository_worktree_state: RepositoryWorktreeState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ReadFailureReason {
    Missing,
    PermissionDenied,
    Malformed,
    UnsupportedFormat,
    IoFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ReadFailure {
    pub source_kind: SourceKind,
    pub filesystem_path_option: Option<FilesystemPath>,
    pub source_identifier_option: Option<SourceIdentifier>,
    pub read_failure_reason: ReadFailureReason,
}
#[rustfmt::skip]
pub type SourceVolumes = std::vec::Vec<SourceVolume>;
#[rustfmt::skip]
pub type TranscriptSegments = std::vec::Vec<TranscriptSegment>;
#[rustfmt::skip]
pub type RepositoryChanges = std::vec::Vec<RepositoryChange>;
#[rustfmt::skip]
pub type Truncations = std::vec::Vec<Truncation>;
#[rustfmt::skip]
pub type ReadFailureRecords = std::vec::Vec<ReadFailure>;
#[rustfmt::skip]
pub type CollectedAt = Timestamp;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EvidencePackage {
    pub package_identifier: PackageIdentifier,
    pub request_identifier: RequestIdentifier,
    pub time_window: TimeWindow,
    pub collected_at: CollectedAt,
    pub source_volumes: SourceVolumes,
    pub transcript_segments: TranscriptSegments,
    pub repository_changes: RepositoryChanges,
    pub truncations: Truncations,
    pub read_failure_records: ReadFailureRecords,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SizeCertainty {
    Exact,
    Estimated,
    Unknown,
}
#[rustfmt::skip]
pub type SegmentCount = std::option::Option<ItemCount>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SizeMetadata {
    pub byte_count_option: Option<ByteCount>,
    pub line_count_option: Option<LineCount>,
    pub segment_count: SegmentCount,
    pub size_certainty: SizeCertainty,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ListingOrder {
    OldestFirst,
    NewestFirst,
    OldestModifiedFirst,
    NewestModifiedFirst,
    ReferenceAscending,
}
#[rustfmt::skip]
pub type PageCursor = std::option::Option<FragilePageCursor>;
#[rustfmt::skip]
pub type NextPageCursor = std::option::Option<FragilePageCursor>;
#[rustfmt::skip]
pub type ReturnedItems = ItemCount;
#[rustfmt::skip]
pub type TotalItems = std::option::Option<ItemCount>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PageRequest {
    pub page_limit: PageLimit,
    pub page_cursor: PageCursor,
    pub listing_order: ListingOrder,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PageMetadata {
    pub page_limit: PageLimit,
    pub returned_items: ReturnedItems,
    pub total_items: TotalItems,
    pub next_page_cursor: NextPageCursor,
    pub listing_order: ListingOrder,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CardProjection {
    MetadataOnly,
    BoundedPreview(BoundedTextProjection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AuthoredStatus {
    AgentAuthored,
    HumanAuthored,
    MixedAuthorship,
    UnknownAuthorship,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AuthoredStatusFilter {
    AnyAuthoredStatus,
    OnlyAuthoredStatus(AuthoredStatus),
}
#[rustfmt::skip]
pub type ProducedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputProvenance {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub authored_status: AuthoredStatus,
    pub produced_at: ProducedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputTextExcerpt {
    pub output_text: OutputText,
    pub byte_count: ByteCount,
    pub truncation_option: Option<Truncation>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SourceLocator {
    pub filesystem_path: FilesystemPath,
    pub root_relative_path_option: Option<RootRelativePath>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SourceHealthStatus {
    ReadableEmpty,
    ReadableIndexed,
    UnreadableRoot,
    DiscoveryTruncated,
    MalformedRecords,
    IndexStoreUnreadable,
}
#[rustfmt::skip]
pub type TaskDuration = std::option::Option<RelativeDuration>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubagentTaskMetadata {
    pub task_identifier: TaskIdentifier,
    pub task_title_option: Option<TaskTitle>,
    pub tool_use_identifier_option: Option<ToolUseIdentifier>,
    pub source_locator_option: Option<SourceLocator>,
    pub source_health_status: SourceHealthStatus,
    pub task_result_option: Option<TaskResult>,
    pub usage_summary_option: Option<UsageSummary>,
    pub task_duration: TaskDuration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionInventoryCompleteness {
    Complete,
    Resumable,
    Truncated,
    Failed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionLifecycleStatus {
    Current,
    PreviouslyObserved,
    SourceMissing,
    SourceBroken,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionArchiveStatus {
    NotArchived,
    Archived,
    ArchiveUnknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ScanLimitKind {
    ScanEntries,
    DiscoveredFiles,
    FileBytes,
    LineBytes,
    ReadFailures,
}
#[rustfmt::skip]
pub type ScanLimit = ItemCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ScanLimitReport {
    pub scan_limit_kind: ScanLimitKind,
    pub scan_limit: ScanLimit,
    pub filesystem_path_option: Option<FilesystemPath>,
}
#[rustfmt::skip]
pub type ScanLimits = std::vec::Vec<ScanLimitReport>;
#[rustfmt::skip]
pub type DiscoveredFileCount = ItemCount;
#[rustfmt::skip]
pub type IndexedSessions = ItemCount;
#[rustfmt::skip]
pub type EarliestModifiedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
pub type LatestModifiedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionInventorySourceReport {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub source_locator: SourceLocator,
    pub session_inventory_completeness: SessionInventoryCompleteness,
    pub scan_limits: ScanLimits,
    pub discovered_file_count: DiscoveredFileCount,
    pub indexed_sessions: IndexedSessions,
    pub byte_count: ByteCount,
    pub earliest_modified_at: EarliestModifiedAt,
    pub latest_modified_at: LatestModifiedAt,
}
#[rustfmt::skip]
pub type SessionInventorySourceReports = std::vec::Vec<SessionInventorySourceReport>;
#[rustfmt::skip]
pub type TotalSessions = ItemCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionInventoryScanReport {
    pub session_inventory_source_reports: SessionInventorySourceReports,
    pub total_sessions: TotalSessions,
    pub session_inventory_completeness: SessionInventoryCompleteness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionRole {
    MainSession,
    SubagentOutputSession,
    Unknown,
}
#[rustfmt::skip]
pub type FileCount = ItemCount;
#[rustfmt::skip]
pub type StartedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
pub type LastObservedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
pub type SubagentCount = std::option::Option<ItemCount>;
#[rustfmt::skip]
pub type OutputCount = std::option::Option<ItemCount>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionInventoryCard {
    pub fragile_session_reference: FragileSessionReference,
    pub session_role: SessionRole,
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub session_identifier_option: Option<SessionIdentifier>,
    pub source_locator: SourceLocator,
    pub file_count: FileCount,
    pub byte_count: ByteCount,
    pub earliest_modified_at: EarliestModifiedAt,
    pub latest_modified_at: LatestModifiedAt,
    pub started_at: StartedAt,
    pub last_observed_at: LastObservedAt,
    pub subagent_count: SubagentCount,
    pub output_count: OutputCount,
    pub session_lifecycle_status: SessionLifecycleStatus,
    pub source_health_status: SourceHealthStatus,
    pub session_archive_status: SessionArchiveStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionCard {
    pub fragile_session_reference: FragileSessionReference,
    pub session_role: SessionRole,
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub session_identifier_option: Option<SessionIdentifier>,
    pub source_locator_option: Option<SourceLocator>,
    pub started_at: StartedAt,
    pub last_observed_at: LastObservedAt,
    pub subagent_count: SubagentCount,
    pub output_count: OutputCount,
    pub size_metadata: SizeMetadata,
}
#[rustfmt::skip]
pub type FirstObservedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubagentCard {
    pub fragile_subagent_reference: FragileSubagentReference,
    pub fragile_session_reference: FragileSessionReference,
    pub subagent_name: SubagentName,
    pub subagent_task_metadata_option: Option<SubagentTaskMetadata>,
    pub authored_status: AuthoredStatus,
    pub output_count: OutputCount,
    pub size_metadata: SizeMetadata,
    pub first_observed_at: FirstObservedAt,
    pub last_observed_at: LastObservedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputCard {
    pub fragile_output_reference: FragileOutputReference,
    pub fragile_session_reference: FragileSessionReference,
    pub fragile_subagent_reference_option: Option<FragileSubagentReference>,
    pub output_title_option: Option<OutputTitle>,
    pub subagent_task_metadata_option: Option<SubagentTaskMetadata>,
    pub output_provenance: OutputProvenance,
    pub size_metadata: SizeMetadata,
    pub output_text_excerpt_option: Option<OutputTextExcerpt>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputSegmentCard {
    pub fragile_output_segment_reference: FragileOutputSegmentReference,
    pub fragile_output_reference: FragileOutputReference,
    pub segment_index: SegmentIndex,
    pub byte_range_option: Option<ByteRange>,
    pub line_range_option: Option<LineRange>,
    pub size_metadata: SizeMetadata,
    pub output_text_excerpt_option: Option<OutputTextExcerpt>,
}
#[rustfmt::skip]
pub type SearchWord = String;
#[rustfmt::skip]
pub type SearchWords = std::vec::Vec<SearchWord>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SearchPhrase {
    pub search_words: SearchWords,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TextQueryTerm {
    Word(SearchWord),
    Phrase(SearchPhrase),
}
#[rustfmt::skip]
pub type WordDistance = i64;
#[rustfmt::skip]
pub type LeftTextQueryTerm = TextQueryTerm;
#[rustfmt::skip]
pub type RightTextQueryTerm = TextQueryTerm;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct NearTextQuery {
    pub left_text_query_term: LeftTextQueryTerm,
    pub right_text_query_term: RightTextQueryTerm,
    pub word_distance: WordDistance,
}
#[rustfmt::skip]
pub type TextQueryNodeIndex = i64;
#[rustfmt::skip]
pub type TextQueryNodeIndices = std::vec::Vec<TextQueryNodeIndex>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TextQueryNode {
    Contains(TextQueryTerm),
    AllOf(TextQueryNodeIndices),
    AnyOf(TextQueryNodeIndices),
    Not(TextQueryNodeIndex),
    Near(NearTextQuery),
}
#[rustfmt::skip]
pub type TextQueryRoot = TextQueryNodeIndex;
#[rustfmt::skip]
pub type TextQueryNodes = std::vec::Vec<TextQueryNode>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockTextQuery {
    pub text_query_nodes: TextQueryNodes,
    pub text_query_root: TextQueryRoot,
}
#[rustfmt::skip]
pub type WordPosition = i64;
#[rustfmt::skip]
pub type StartWordPosition = WordPosition;
#[rustfmt::skip]
pub type EndWordPosition = WordPosition;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Occurrence {
    pub start_word_position: StartWordPosition,
    pub end_word_position: EndWordPosition,
}
#[rustfmt::skip]
pub type Occurrences = std::vec::Vec<Occurrence>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ContainsEvidence {
    pub text_query_term: TextQueryTerm,
    pub occurrences: Occurrences,
}
#[rustfmt::skip]
pub type LeftOccurrence = Occurrence;
#[rustfmt::skip]
pub type RightOccurrence = Occurrence;
#[rustfmt::skip]
pub type OccurrenceGap = WordDistance;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct NearOccurrencePair {
    pub left_occurrence: LeftOccurrence,
    pub right_occurrence: RightOccurrence,
    pub occurrence_gap: OccurrenceGap,
}
#[rustfmt::skip]
pub type NearOccurrencePairs = std::vec::Vec<NearOccurrencePair>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct NearEvidence {
    pub left_text_query_term: LeftTextQueryTerm,
    pub right_text_query_term: RightTextQueryTerm,
    pub word_distance: WordDistance,
    pub near_occurrence_pairs: NearOccurrencePairs,
}
#[rustfmt::skip]
pub type MatchEvidenceIndex = i64;
#[rustfmt::skip]
pub type MatchEvidenceIndices = std::vec::Vec<MatchEvidenceIndex>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MatchEvidenceNode {
    Contains(ContainsEvidence),
    AllOf(MatchEvidenceIndices),
    AnyOf(MatchEvidenceIndices),
    Not,
    Near(NearEvidence),
}
#[rustfmt::skip]
pub type MatchEvidenceRoot = MatchEvidenceIndex;
#[rustfmt::skip]
pub type MatchEvidenceNodes = std::vec::Vec<MatchEvidenceNode>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockSearchEvidence {
    pub match_evidence_nodes: MatchEvidenceNodes,
    pub match_evidence_root: MatchEvidenceRoot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
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
#[rustfmt::skip]
pub type TranscriptBlockKinds = std::vec::Vec<TranscriptBlockKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SelectedTranscriptBlockKinds {
    pub transcript_block_kinds: TranscriptBlockKinds,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TranscriptBlockKindSelection {
    AllTranscriptBlockKinds,
    OnlyTranscriptBlockKinds(SelectedTranscriptBlockKinds),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TranscriptBlockTextAvailability {
    ReadableText,
    UnavailableText,
    EncryptedText,
}
#[rustfmt::skip]
pub type ObservedAt = std::option::Option<Timestamp>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockProvenance {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub authored_status: AuthoredStatus,
    pub observed_at: ObservedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockCard {
    pub fragile_transcript_block_reference: FragileTranscriptBlockReference,
    pub fragile_session_reference: FragileSessionReference,
    pub fragile_subagent_reference_option: Option<FragileSubagentReference>,
    pub subagent_task_metadata_option: Option<SubagentTaskMetadata>,
    pub transcript_block_kind: TranscriptBlockKind,
    pub transcript_block_index: TranscriptBlockIndex,
    pub transcript_block_provenance: TranscriptBlockProvenance,
    pub line_range_option: Option<LineRange>,
    pub byte_range_option: Option<ByteRange>,
    pub size_metadata: SizeMetadata,
    pub transcript_block_text_availability: TranscriptBlockTextAvailability,
    pub transcript_text_excerpt_option: Option<TranscriptTextExcerpt>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionListFilter {
    pub source_selection: SourceSelection,
    pub time_window_option: Option<TimeWindow>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubagentListFilter {
    pub fragile_session_reference: FragileSessionReference,
    pub authored_status_filter: AuthoredStatusFilter,
    pub task_identifier_option: Option<TaskIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputListFilter {
    pub source_selection: SourceSelection,
    pub fragile_session_reference_option: Option<FragileSessionReference>,
    pub fragile_subagent_reference_option: Option<FragileSubagentReference>,
    pub task_identifier_option: Option<TaskIdentifier>,
    pub authored_status_filter: AuthoredStatusFilter,
    pub time_window_option: Option<TimeWindow>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputSegmentListFilter {
    pub fragile_output_reference: FragileOutputReference,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockFilter {
    pub source_selection: SourceSelection,
    pub fragile_session_reference_option: Option<FragileSessionReference>,
    pub fragile_subagent_reference_option: Option<FragileSubagentReference>,
    pub task_identifier_option: Option<TaskIdentifier>,
    pub transcript_block_kind_selection: TranscriptBlockKindSelection,
    pub authored_status_filter: AuthoredStatusFilter,
    pub time_window_option: Option<TimeWindow>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionListRequest {
    pub request_identifier: RequestIdentifier,
    pub session_list_filter: SessionListFilter,
    pub page_request: PageRequest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubagentListRequest {
    pub request_identifier: RequestIdentifier,
    pub subagent_list_filter: SubagentListFilter,
    pub page_request: PageRequest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputListRequest {
    pub request_identifier: RequestIdentifier,
    pub output_list_filter: OutputListFilter,
    pub page_request: PageRequest,
    pub card_projection: CardProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputSegmentListRequest {
    pub request_identifier: RequestIdentifier,
    pub output_segment_list_filter: OutputSegmentListFilter,
    pub page_request: PageRequest,
    pub card_projection: CardProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockListRequest {
    pub request_identifier: RequestIdentifier,
    pub transcript_block_filter: TranscriptBlockFilter,
    pub page_request: PageRequest,
    pub card_projection: CardProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockSearchRequest {
    pub request_identifier: RequestIdentifier,
    pub transcript_block_filter: TranscriptBlockFilter,
    pub transcript_block_text_query: TranscriptBlockTextQuery,
    pub page_request: PageRequest,
    pub card_projection: CardProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OutputReadRange {
    EntireOutput,
    Bytes(ByteRange),
    Lines(LineRange),
    Segment(FragileOutputSegmentReference),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputEstimateRequest {
    pub request_identifier: RequestIdentifier,
    pub fragile_output_reference: FragileOutputReference,
    pub output_read_range: OutputReadRange,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputReadRequest {
    pub request_identifier: RequestIdentifier,
    pub fragile_output_reference: FragileOutputReference,
    pub output_read_range: OutputReadRange,
    pub maximum_bytes: MaximumBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockEstimateRequest {
    pub request_identifier: RequestIdentifier,
    pub fragile_transcript_block_reference: FragileTranscriptBlockReference,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockReadRequest {
    pub request_identifier: RequestIdentifier,
    pub fragile_transcript_block_reference: FragileTranscriptBlockReference,
    pub maximum_bytes: MaximumBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RuntimeHealthRequest {
    pub request_identifier: RequestIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionInventoryRequest {
    pub request_identifier: RequestIdentifier,
    pub source_selection: SourceSelection,
    pub archive_path_option: Option<ArchivePath>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SessionLookupSelector {
    ByReference(FragileSessionReference),
    ByProducerSession(SessionIdentifier),
    BySourceLocator(SourceLocator),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionLookupRequest {
    pub request_identifier: RequestIdentifier,
    pub session_lookup_selector: SessionLookupSelector,
    pub archive_path_option: Option<ArchivePath>,
}
#[rustfmt::skip]
pub type CreatedAt = Timestamp;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveRecordDraft {
    pub session_inventory_card: SessionInventoryCard,
    pub archive_summary_text: ArchiveSummaryText,
    pub archive_provenance_text: ArchiveProvenanceText,
    pub created_at: CreatedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveWriteRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub session_archive_record_draft: SessionArchiveRecordDraft,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveQueryRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub fragile_session_reference_option: Option<FragileSessionReference>,
}
#[rustfmt::skip]
pub type MaximumSummaryBytes = ByteLimit;
#[rustfmt::skip]
pub type MaximumProvenanceBytes = ByteLimit;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveReadRequest {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub archive_record_identifier: ArchiveRecordIdentifier,
    pub maximum_summary_bytes: MaximumSummaryBytes,
    pub maximum_provenance_bytes: MaximumProvenanceBytes,
}
#[rustfmt::skip]
pub type SummaryBytes = ByteCount;
#[rustfmt::skip]
pub type ProvenanceBytes = ByteCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveRecordCard {
    pub archive_record_identifier: ArchiveRecordIdentifier,
    pub fragile_session_reference: FragileSessionReference,
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub session_identifier_option: Option<SessionIdentifier>,
    pub created_at: CreatedAt,
    pub summary_bytes: SummaryBytes,
    pub provenance_bytes: ProvenanceBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ArchiveTextCompleteness {
    Complete,
    Truncated,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveTextProjection {
    pub archive_summary_text: ArchiveSummaryText,
    pub byte_count: ByteCount,
    pub archive_text_completeness: ArchiveTextCompleteness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveProvenanceProjection {
    pub archive_provenance_text: ArchiveProvenanceText,
    pub byte_count: ByteCount,
    pub archive_text_completeness: ArchiveTextCompleteness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveRecordProjection {
    pub session_archive_record_card: SessionArchiveRecordCard,
    pub session_inventory_card: SessionInventoryCard,
    pub session_archive_text_projection: SessionArchiveTextProjection,
    pub session_archive_provenance_projection: SessionArchiveProvenanceProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RuntimeCapabilityStatus {
    Supported,
    Unsupported,
}
#[rustfmt::skip]
pub type HealthObservationCapability = RuntimeCapabilityStatus;
#[rustfmt::skip]
pub type TranscriptOnlyConfigurationCapability = RuntimeCapabilityStatus;
#[rustfmt::skip]
pub type ClaudeSubagentOutputSourcesCapability = RuntimeCapabilityStatus;
#[rustfmt::skip]
pub type PiSubagentOutputSourcesCapability = RuntimeCapabilityStatus;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RuntimeCapabilities {
    pub health_observation_capability: HealthObservationCapability,
    pub transcript_only_configuration_capability: TranscriptOnlyConfigurationCapability,
    pub claude_subagent_output_sources_capability: ClaudeSubagentOutputSourcesCapability,
    pub pi_subagent_output_sources_capability: PiSubagentOutputSourcesCapability,
}
#[rustfmt::skip]
pub type IndexedRecords = ItemCount;
#[rustfmt::skip]
pub type MalformedRecordCount = ItemCount;
#[rustfmt::skip]
pub type UnreadableRecords = ItemCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SourceHealthCard {
    pub source_kind: SourceKind,
    pub source_identifier: SourceIdentifier,
    pub source_locator: SourceLocator,
    pub source_health_status: SourceHealthStatus,
    pub scan_limits: ScanLimits,
    pub discovered_file_count: DiscoveredFileCount,
    pub indexed_records: IndexedRecords,
    pub malformed_record_count: MalformedRecordCount,
    pub unreadable_records: UnreadableRecords,
}
#[rustfmt::skip]
pub type SourceHealthCards = std::vec::Vec<SourceHealthCard>;
#[rustfmt::skip]
pub type SessionCount = ItemCount;
#[rustfmt::skip]
pub type IndexSubagentCount = ItemCount;
#[rustfmt::skip]
pub type IndexOutputCount = ItemCount;
#[rustfmt::skip]
pub type TranscriptBlockCount = ItemCount;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct IndexHealth {
    pub source_health_status: SourceHealthStatus,
    pub session_count: SessionCount,
    pub index_subagent_count: IndexSubagentCount,
    pub index_output_count: IndexOutputCount,
    pub transcript_block_count: TranscriptBlockCount,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RuntimeHealthObserved {
    pub request_identifier: RequestIdentifier,
    pub runtime_capabilities: RuntimeCapabilities,
    pub source_health_cards: SourceHealthCards,
    pub index_health: IndexHealth,
}
#[rustfmt::skip]
pub type SessionInventoryCards = std::vec::Vec<SessionInventoryCard>;
#[rustfmt::skip]
pub type SessionArchiveRecordCards = std::vec::Vec<SessionArchiveRecordCard>;
#[rustfmt::skip]
pub type SessionCards = std::vec::Vec<SessionCard>;
#[rustfmt::skip]
pub type SubagentCards = std::vec::Vec<SubagentCard>;
#[rustfmt::skip]
pub type OutputCards = std::vec::Vec<OutputCard>;
#[rustfmt::skip]
pub type OutputSegmentCards = std::vec::Vec<OutputSegmentCard>;
#[rustfmt::skip]
pub type TranscriptBlockCards = std::vec::Vec<TranscriptBlockCard>;
#[rustfmt::skip]
pub type TranscriptBlockSearchMatches = std::vec::Vec<TranscriptBlockSearchMatch>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionsInventoried {
    pub request_identifier: RequestIdentifier,
    pub session_inventory_cards: SessionInventoryCards,
    pub session_inventory_scan_report: SessionInventoryScanReport,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionLookedUp {
    pub request_identifier: RequestIdentifier,
    pub session_inventory_cards: SessionInventoryCards,
    pub session_inventory_scan_report: SessionInventoryScanReport,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveWritten {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub session_archive_record_card: SessionArchiveRecordCard,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveQueried {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub session_archive_record_cards: SessionArchiveRecordCards,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionArchiveRead {
    pub request_identifier: RequestIdentifier,
    pub archive_path: ArchivePath,
    pub session_archive_record_projection: SessionArchiveRecordProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionsListed {
    pub request_identifier: RequestIdentifier,
    pub session_cards: SessionCards,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SubagentsListed {
    pub request_identifier: RequestIdentifier,
    pub subagent_cards: SubagentCards,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputsListed {
    pub request_identifier: RequestIdentifier,
    pub output_cards: OutputCards,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputSegmentsListed {
    pub request_identifier: RequestIdentifier,
    pub output_segment_cards: OutputSegmentCards,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputEstimated {
    pub request_identifier: RequestIdentifier,
    pub fragile_output_reference: FragileOutputReference,
    pub output_read_range: OutputReadRange,
    pub size_metadata: SizeMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OutputRead {
    pub request_identifier: RequestIdentifier,
    pub fragile_output_reference: FragileOutputReference,
    pub output_read_range: OutputReadRange,
    pub size_metadata: SizeMetadata,
    pub output_text_excerpt: OutputTextExcerpt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlocksListed {
    pub request_identifier: RequestIdentifier,
    pub transcript_block_cards: TranscriptBlockCards,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockSearchMatch {
    pub transcript_block_card: TranscriptBlockCard,
    pub transcript_block_search_evidence: TranscriptBlockSearchEvidence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlocksSearched {
    pub request_identifier: RequestIdentifier,
    pub transcript_block_search_matches: TranscriptBlockSearchMatches,
    pub page_metadata: PageMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockEstimated {
    pub request_identifier: RequestIdentifier,
    pub fragile_transcript_block_reference: FragileTranscriptBlockReference,
    pub size_metadata: SizeMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TranscriptBlockRead {
    pub request_identifier: RequestIdentifier,
    pub fragile_transcript_block_reference: FragileTranscriptBlockReference,
    pub size_metadata: SizeMetadata,
    pub transcript_text_excerpt: TranscriptTextExcerpt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RejectedFragileReference {
    Session(FragileSessionReference),
    Subagent(FragileSubagentReference),
    Output(FragileOutputReference),
    OutputSegment(FragileOutputSegmentReference),
    PageCursor(FragilePageCursor),
    TranscriptBlock(FragileTranscriptBlockReference),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OperationRejected {
    pub request_identifier: RequestIdentifier,
    pub operation_kind: OperationKind,
    pub operation_rejection_reason: OperationRejectionReason,
    pub rejected_fragile_reference_option: Option<RejectedFragileReference>,
}
#[rustfmt::skip]
pub type ClientName = std::option::Option<ContractName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionQuery {
    pub client_name: ClientName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionReport {
    pub contract_name: ContractName,
    pub contract_version: ContractVersion,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RejectionReason {
    InvalidTimeWindow,
    UnsupportedProjection,
    LimitExceeded,
    ConfigurationUnavailable,
    CollectionUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct EvidenceRejected {
    pub request_identifier: RequestIdentifier,
    pub operation_kind: OperationKind,
    pub rejection_reason: RejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Collect(EvidenceRequest),
    Version(VersionQuery),
    ObserveHealth(RuntimeHealthRequest),
    InventorySessions(SessionInventoryRequest),
    LookupSession(SessionLookupRequest),
    WriteSessionArchive(SessionArchiveWriteRequest),
    QuerySessionArchive(SessionArchiveQueryRequest),
    ReadSessionArchive(SessionArchiveReadRequest),
    ListSessions(SessionListRequest),
    ListSubagents(SubagentListRequest),
    ListOutputs(OutputListRequest),
    ListOutputSegments(OutputSegmentListRequest),
    EstimateOutput(OutputEstimateRequest),
    ReadOutput(OutputReadRequest),
    ListTranscriptBlocks(TranscriptBlockListRequest),
    SearchTranscriptBlocks(TranscriptBlockSearchRequest),
    EstimateTranscriptBlock(TranscriptBlockEstimateRequest),
    ReadTranscriptBlock(TranscriptBlockReadRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
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
