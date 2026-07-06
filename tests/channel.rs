use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_aggregator::*;
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};

fn request_identifier() -> RequestIdentifier {
    RequestIdentifier::new("req-test")
}

fn recent_window() -> TimeWindow {
    TimeWindow::Recent(RelativeDuration {
        amount: DurationAmount::new(6),
        unit: DurationUnit::Hours,
    })
}

fn evidence_request() -> EvidenceRequest {
    EvidenceRequest {
        request_identifier: request_identifier(),
        time_window: recent_window(),
        source_selection: SourceSelection::AllConfigured,
        projection: Projection::MetadataOnly,
        limit_policy: LimitPolicy {
            maximum_segments: SegmentLimit::new(32),
            maximum_bytes: ByteLimit::new(4096),
        },
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(0),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: AggregatorRequest) -> AggregatorRequest {
    let frame = AggregatorFrame::new(AggregatorFrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = AggregatorFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        AggregatorFrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply_payload: AggregatorReply) -> AggregatorReply {
    let frame = AggregatorFrame::new(AggregatorFrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply_payload.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = AggregatorFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        AggregatorFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected ok reply, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let decoded = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(decoded, value);
}

fn session_reference() -> FragileSessionReference {
    FragileSessionReference::new("fragile-session-1")
}

fn subagent_reference() -> FragileSubagentReference {
    FragileSubagentReference::new("fragile-subagent-1")
}

fn output_reference() -> FragileOutputReference {
    FragileOutputReference::new("fragile-output-1")
}

fn output_segment_reference() -> FragileOutputSegmentReference {
    FragileOutputSegmentReference::new("fragile-segment-1")
}

fn page_request() -> PageRequest {
    PageRequest {
        limit: PageLimit::new(2),
        cursor: None,
        order: ListingOrder::NewestFirst,
    }
}

fn page_metadata() -> PageMetadata {
    PageMetadata {
        limit: PageLimit::new(2),
        returned_items: ItemCount::new(1),
        total_items: Some(ItemCount::new(4)),
        next_cursor: Some(FragilePageCursor::new("fragile-page-2")),
        order: ListingOrder::NewestFirst,
    }
}

fn exact_size() -> SizeMetadata {
    SizeMetadata {
        byte_count: Some(ByteCount::new(256)),
        line_count: Some(LineCount::new(8)),
        segment_count: Some(ItemCount::new(2)),
        certainty: SizeCertainty::Exact,
    }
}

fn estimated_size() -> SizeMetadata {
    SizeMetadata {
        byte_count: Some(ByteCount::new(512)),
        line_count: Some(LineCount::new(16)),
        segment_count: Some(ItemCount::new(4)),
        certainty: SizeCertainty::Estimated,
    }
}

fn output_provenance() -> OutputProvenance {
    OutputProvenance {
        source: SourceKind::Claude,
        source_identifier: SourceIdentifier::new("claude-transcript-1"),
        authored_status: AuthoredStatus::AgentAuthored,
        produced_at: Some(Timestamp::new("20260705T130000Z")),
    }
}

fn output_text_preview() -> OutputTextExcerpt {
    OutputTextExcerpt {
        text: OutputText::new("preview-text"),
        byte_count: ByteCount::new(12),
        truncation: None,
    }
}

fn output_text_excerpt() -> OutputTextExcerpt {
    OutputTextExcerpt {
        text: OutputText::new("bounded-output-text"),
        byte_count: ByteCount::new(19),
        truncation: Some(Truncation {
            source: SourceKind::Claude,
            path: None,
            original_bytes: Some(ByteCount::new(512)),
            projected_bytes: ByteCount::new(64),
            reason: TruncationReason::RequestLimit,
        }),
    }
}

fn session_card() -> SessionCard {
    SessionCard {
        reference: session_reference(),
        source: SourceKind::Claude,
        source_identifier: SourceIdentifier::new("claude-session-1"),
        started_at: Some(Timestamp::new("20260705T120000Z")),
        last_observed_at: Some(Timestamp::new("20260705T130000Z")),
        subagent_count: Some(ItemCount::new(1)),
        output_count: Some(ItemCount::new(2)),
        size: exact_size(),
    }
}

fn subagent_card() -> SubagentCard {
    SubagentCard {
        reference: subagent_reference(),
        session_reference: session_reference(),
        name: SubagentName::new("ContractSurface"),
        authored_status: AuthoredStatus::AgentAuthored,
        output_count: Some(ItemCount::new(2)),
        size: exact_size(),
        first_observed_at: Some(Timestamp::new("20260705T121000Z")),
        last_observed_at: Some(Timestamp::new("20260705T130000Z")),
    }
}

fn output_card() -> OutputCard {
    OutputCard {
        reference: output_reference(),
        session_reference: session_reference(),
        subagent_reference: Some(subagent_reference()),
        title: Some(OutputTitle::new("worker-output")),
        provenance: output_provenance(),
        size: exact_size(),
        preview: Some(output_text_preview()),
    }
}

fn output_segment_card() -> OutputSegmentCard {
    OutputSegmentCard {
        reference: output_segment_reference(),
        output_reference: output_reference(),
        segment_index: SegmentIndex::new(0),
        byte_range: Some(ByteRange {
            start: ByteCount::new(0),
            end: ByteCount::new(64),
        }),
        line_range: Some(LineRange {
            start: LineNumber::new(1),
            end: LineNumber::new(4),
        }),
        size: exact_size(),
        preview: Some(output_text_preview()),
    }
}

fn session_list_request() -> SessionListRequest {
    SessionListRequest {
        request_identifier: RequestIdentifier::new("req-sessions"),
        filter: SessionListFilter {
            source_selection: SourceSelection::AllConfigured,
            time_window: None,
        },
        page: page_request(),
    }
}

fn subagent_list_request() -> SubagentListRequest {
    SubagentListRequest {
        request_identifier: RequestIdentifier::new("req-subagents"),
        filter: SubagentListFilter {
            session_reference: session_reference(),
            authored_status: AuthoredStatusFilter::AnyAuthoredStatus,
        },
        page: page_request(),
    }
}

fn output_list_request() -> OutputListRequest {
    OutputListRequest {
        request_identifier: RequestIdentifier::new("req-outputs"),
        filter: OutputListFilter {
            source_selection: SourceSelection::AllConfigured,
            session_reference: Some(session_reference()),
            subagent_reference: Some(subagent_reference()),
            authored_status: AuthoredStatusFilter::OnlyAuthoredStatus(
                AuthoredStatus::AgentAuthored,
            ),
            time_window: None,
        },
        page: page_request(),
        projection: CardProjection::BoundedPreview(BoundedTextProjection {
            maximum_bytes: ByteLimit::new(128),
        }),
    }
}

fn output_segment_list_request() -> OutputSegmentListRequest {
    OutputSegmentListRequest {
        request_identifier: RequestIdentifier::new("req-segments"),
        filter: OutputSegmentListFilter {
            output_reference: output_reference(),
        },
        page: page_request(),
        projection: CardProjection::MetadataOnly,
    }
}

fn output_estimate_request() -> OutputEstimateRequest {
    OutputEstimateRequest {
        request_identifier: RequestIdentifier::new("req-estimate"),
        output_reference: output_reference(),
        range: OutputReadRange::EntireOutput,
    }
}

fn output_read_range() -> OutputReadRange {
    OutputReadRange::Bytes(ByteRange {
        start: ByteCount::new(0),
        end: ByteCount::new(64),
    })
}

fn output_read_request() -> OutputReadRequest {
    OutputReadRequest {
        request_identifier: RequestIdentifier::new("req-read"),
        output_reference: output_reference(),
        range: output_read_range(),
        maximum_bytes: ByteLimit::new(64),
    }
}

fn sessions_listed() -> SessionsListed {
    SessionsListed {
        request_identifier: RequestIdentifier::new("req-sessions"),
        sessions: vec![session_card()],
        page: page_metadata(),
    }
}

fn subagents_listed() -> SubagentsListed {
    SubagentsListed {
        request_identifier: RequestIdentifier::new("req-subagents"),
        subagents: vec![subagent_card()],
        page: page_metadata(),
    }
}

fn outputs_listed() -> OutputsListed {
    OutputsListed {
        request_identifier: RequestIdentifier::new("req-outputs"),
        outputs: vec![output_card()],
        page: page_metadata(),
    }
}

fn output_segments_listed() -> OutputSegmentsListed {
    OutputSegmentsListed {
        request_identifier: RequestIdentifier::new("req-segments"),
        segments: vec![output_segment_card()],
        page: page_metadata(),
    }
}

fn output_estimated() -> OutputEstimated {
    OutputEstimated {
        request_identifier: RequestIdentifier::new("req-estimate"),
        output_reference: output_reference(),
        range: OutputReadRange::EntireOutput,
        size: estimated_size(),
    }
}

fn output_read() -> OutputRead {
    OutputRead {
        request_identifier: RequestIdentifier::new("req-read"),
        output_reference: output_reference(),
        range: output_read_range(),
        size: estimated_size(),
        excerpt: output_text_excerpt(),
    }
}

fn output_interface_requests() -> Vec<AggregatorRequest> {
    vec![
        AggregatorRequest::ListSessions(session_list_request()),
        AggregatorRequest::ListSubagents(subagent_list_request()),
        AggregatorRequest::ListOutputs(output_list_request()),
        AggregatorRequest::ListOutputSegments(output_segment_list_request()),
        AggregatorRequest::EstimateOutput(output_estimate_request()),
        AggregatorRequest::ReadOutput(output_read_request()),
    ]
}

fn output_interface_replies() -> Vec<AggregatorReply> {
    vec![
        AggregatorReply::SessionsListed(sessions_listed()),
        AggregatorReply::SubagentsListed(subagents_listed()),
        AggregatorReply::OutputsListed(outputs_listed()),
        AggregatorReply::OutputSegmentsListed(output_segments_listed()),
        AggregatorReply::OutputEstimated(output_estimated()),
        AggregatorReply::OutputRead(output_read()),
    ]
}

enum CanonicalExample {
    Request(AggregatorRequest),
    Reply(AggregatorReply),
}

impl CanonicalExample {
    fn assert_matches_line(&self, line: &str) {
        match self {
            Self::Request(expected) => {
                let decoded = NotaSource::new(line)
                    .parse::<AggregatorRequest>()
                    .expect("canonical request decode");
                assert_eq!(&decoded, expected, "canonical request decode for {line}");
                assert_eq!(decoded.to_nota(), line, "canonical request encode");
            }
            Self::Reply(expected) => {
                let decoded = NotaSource::new(line)
                    .parse::<AggregatorReply>()
                    .expect("canonical reply decode");
                assert_eq!(&decoded, expected, "canonical reply decode for {line}");
                assert_eq!(decoded.to_nota(), line, "canonical reply encode");
            }
        }
    }
}

fn canonical_example_lines() -> Vec<&'static str> {
    include_str!("../examples/canonical.nota")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

fn canonical_evidence_request() -> EvidenceRequest {
    EvidenceRequest {
        request_identifier: RequestIdentifier::new("req-20260705"),
        time_window: TimeWindow::Recent(RelativeDuration {
            amount: DurationAmount::new(6),
            unit: DurationUnit::Hours,
        }),
        source_selection: SourceSelection::Only(SelectedSources {
            sources: vec![
                SourceKind::Claude,
                SourceKind::Codex,
                SourceKind::Repository,
            ],
        }),
        projection: Projection::MetadataOnly,
        limit_policy: LimitPolicy {
            maximum_segments: SegmentLimit::new(32),
            maximum_bytes: ByteLimit::new(4096),
        },
    }
}

fn canonical_examples() -> Vec<CanonicalExample> {
    let mut examples = vec![
        CanonicalExample::Request(AggregatorRequest::Collect(canonical_evidence_request())),
        CanonicalExample::Request(AggregatorRequest::Version(Version { client_name: None })),
    ];
    examples.extend(
        output_interface_requests()
            .into_iter()
            .map(CanonicalExample::Request),
    );
    examples.extend([
        CanonicalExample::Reply(AggregatorReply::EvidenceCollected(EvidencePackage {
            package_identifier: PackageIdentifier::new("pkg-20260705"),
            request_identifier: RequestIdentifier::new("req-20260705"),
            time_window: TimeWindow::Recent(RelativeDuration {
                amount: DurationAmount::new(6),
                unit: DurationUnit::Hours,
            }),
            collected_at: Timestamp::new("20260705T130000Z"),
            source_volumes: vec![],
            transcript_segments: vec![],
            repository_changes: vec![],
            truncations: vec![],
            read_failures: vec![],
        })),
        CanonicalExample::Reply(AggregatorReply::VersionReported(VersionReport {
            contract_name: ContractName::new("signal-aggregator"),
            contract_version: ContractVersion::new("0.2.0"),
        })),
        CanonicalExample::Reply(AggregatorReply::EvidenceRejected(EvidenceRejected {
            request_identifier: RequestIdentifier::new("req-20260705"),
            operation: AggregatorOperationKind::Collect,
            reason: RejectionReason::UnsupportedProjection,
        })),
    ]);
    examples.extend(
        output_interface_replies()
            .into_iter()
            .map(CanonicalExample::Reply),
    );
    examples.push(CanonicalExample::Reply(AggregatorReply::OperationRejected(
        OperationRejected {
            request_identifier: RequestIdentifier::new("req-read"),
            operation: AggregatorOperationKind::ReadOutput,
            reason: OperationRejectionReason::FragileReferenceStale,
            reference: Some(RejectedFragileReference::Output(output_reference())),
        },
    )));
    examples
}

#[test]
fn collect_request_round_trips_through_frame() {
    let request = AggregatorRequest::Collect(evidence_request());
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn evidence_collected_round_trips_through_frame() {
    let reply = AggregatorReply::EvidenceCollected(EvidencePackage {
        package_identifier: PackageIdentifier::new("pkg-test"),
        request_identifier: request_identifier(),
        time_window: recent_window(),
        collected_at: Timestamp::new("20260705T130000Z"),
        source_volumes: vec![],
        transcript_segments: vec![],
        repository_changes: vec![],
        truncations: vec![],
        read_failures: vec![],
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn typed_rejection_round_trips_through_frame() {
    let reply = AggregatorReply::EvidenceRejected(EvidenceRejected {
        request_identifier: request_identifier(),
        operation: AggregatorOperationKind::Collect,
        reason: RejectionReason::ConfigurationUnavailable,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn output_interface_requests_round_trip_through_frame() {
    for request in output_interface_requests() {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn output_interface_replies_round_trip_through_frame() {
    for reply in output_interface_replies() {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn fragile_reference_rejections_round_trip_through_frame() {
    let stale_reply = AggregatorReply::OperationRejected(OperationRejected {
        request_identifier: RequestIdentifier::new("req-read"),
        operation: AggregatorOperationKind::ReadOutput,
        reason: OperationRejectionReason::FragileReferenceStale,
        reference: Some(RejectedFragileReference::Output(output_reference())),
    });
    let broken_reply = AggregatorReply::OperationRejected(OperationRejected {
        request_identifier: RequestIdentifier::new("req-segments"),
        operation: AggregatorOperationKind::ListOutputSegments,
        reason: OperationRejectionReason::FragileReferenceBroken,
        reference: Some(RejectedFragileReference::OutputSegment(
            output_segment_reference(),
        )),
    });
    assert_eq!(round_trip_reply(stale_reply.clone()), stale_reply);
    assert_eq!(round_trip_reply(broken_reply.clone()), broken_reply);
}

#[test]
fn version_request_and_reply_round_trip_through_nota() {
    round_trip_nota(AggregatorRequest::Version(Version { client_name: None }));
    round_trip_nota(AggregatorReply::VersionReported(VersionReport {
        contract_name: ContractName::new("signal-aggregator"),
        contract_version: ContractVersion::new("0.2.0"),
    }));
}

#[test]
fn collect_request_round_trips_through_nota() {
    round_trip_nota(AggregatorRequest::Collect(evidence_request()));
}

#[test]
fn output_interface_requests_and_replies_round_trip_through_nota() {
    for request in output_interface_requests() {
        round_trip_nota(request);
    }
    for reply in output_interface_replies() {
        round_trip_nota(reply);
    }
    round_trip_nota(AggregatorReply::OperationRejected(OperationRejected {
        request_identifier: RequestIdentifier::new("req-read"),
        operation: AggregatorOperationKind::ReadOutput,
        reason: OperationRejectionReason::InvalidRange,
        reference: Some(RejectedFragileReference::Output(output_reference())),
    }));
}

#[test]
fn output_lists_are_metadata_first_with_bounded_preview() {
    let request = output_list_request();
    match request.projection {
        CardProjection::BoundedPreview(projection) => {
            assert_eq!(projection.maximum_bytes.into_u64(), 128);
        }
        other => panic!("expected bounded preview projection, got {other:?}"),
    }

    let reply = outputs_listed();
    let preview = reply.outputs[0]
        .preview
        .as_ref()
        .expect("output card carries bounded preview only");
    assert_eq!(preview.byte_count.into_u64(), 12);
    assert_eq!(preview.text.as_str(), "preview-text");
}

#[test]
fn read_output_requires_explicit_bound_and_range() {
    let request = output_read_request();
    assert_eq!(request.maximum_bytes.into_u64(), 64);
    assert!(matches!(request.range, OutputReadRange::Bytes(_)));

    let reply = output_read();
    assert_eq!(reply.excerpt.text.as_str(), "bounded-output-text");
    assert_eq!(reply.excerpt.byte_count.into_u64(), 19);
    assert!(reply.excerpt.truncation.is_some());
}

#[test]
fn canonical_examples_match_file_order_and_boundaries() {
    let expected_examples = canonical_examples();
    let actual_lines = canonical_example_lines();
    assert_eq!(
        actual_lines.len(),
        expected_examples.len(),
        "canonical example count changed"
    );
    for (expected, line) in expected_examples.iter().zip(actual_lines) {
        expected.assert_matches_line(line);
    }
}

#[test]
fn operation_heads_are_contract_local() {
    assert_eq!(
        <AggregatorRequest as SignalOperationHeads>::HEADS,
        &[
            "Collect",
            "Version",
            "ListSessions",
            "ListSubagents",
            "ListOutputs",
            "ListOutputSegments",
            "EstimateOutput",
            "ReadOutput",
        ]
    );
    assert_eq!(
        AggregatorRequest::Collect(evidence_request()).operation_kind(),
        AggregatorOperationKind::Collect
    );
    assert_eq!(
        AggregatorRequest::ReadOutput(output_read_request()).operation_kind(),
        AggregatorOperationKind::ReadOutput
    );
}

const EXPECTED_SCHEMA_SKETCH: &str = r#"{}

[
  (Collect [EvidenceRequest])
  (Version [Version])
  (ListSessions [SessionListRequest])
  (ListSubagents [SubagentListRequest])
  (ListOutputs [OutputListRequest])
  (ListOutputSegments [OutputSegmentListRequest])
  (EstimateOutput [OutputEstimateRequest])
  (ReadOutput [OutputReadRequest])
]

[
  (EvidenceCollected [EvidencePackage])
  (VersionReported [VersionReport])
  (EvidenceRejected [EvidenceRejected])
  (SessionsListed [SessionsListed])
  (SubagentsListed [SubagentsListed])
  (OutputsListed [OutputsListed])
  (OutputSegmentsListed [OutputSegmentsListed])
  (OutputEstimated [OutputEstimated])
  (OutputRead [OutputRead])
  (OperationRejected [OperationRejected])
]

[]

{
  EvidenceRequest (RequestIdentifier TimeWindow SourceSelection Projection LimitPolicy)
  TimeWindow [Recent Range Since]
  Recent (RelativeDuration)
  Range (TimeRange)
  Since (Timestamp)
  RelativeDuration (DurationAmount DurationUnit)
  DurationUnit [Minutes Hours Days]
  TimeRange (Timestamp Timestamp)
  SourceSelection [AllConfigured Only]
  AllConfigured
  Only ([SourceKind])
  SourceKind [Claude Codex Pi Repository]
  Projection [MetadataOnly IdentifiersOnly BoundedText]
  BoundedText (ByteLimit)
  LimitPolicy (SegmentLimit ByteLimit)
  EvidencePackage (PackageIdentifier RequestIdentifier TimeWindow Timestamp [SourceVolume] [TranscriptSegment] [RepositoryChange] [Truncation] [ReadFailure])
  SourceVolume (SourceKind SourceIdentifier ItemCount ByteCount ?Timestamp ?Timestamp)
  LineRange (LineNumber LineNumber)
  ByteRange (ByteCount ByteCount)
  TranscriptTextExcerpt (TranscriptText ByteCount ?Truncation)
  SegmentProjection [MetadataOnly IdentifiersOnly Text]
  Text (TranscriptTextExcerpt)
  TranscriptSegment (SourceKind SourceIdentifier TranscriptSegmentIdentifier FilesystemPath ?Timestamp ?LineRange ?ByteRange SegmentProjection)
  RepositoryWorktreeState [Clean HasChanges NotObserved]
  RepositoryChange (RepositoryIdentifier FilesystemPath ?CommitIdentifier ?Timestamp [RepositoryPath] RepositoryWorktreeState)
  TruncationReason [RequestLimit SourceLimit ProjectionLimit]
  Truncation (SourceKind ?FilesystemPath ?ByteCount ByteCount TruncationReason)
  ReadFailureReason [Missing PermissionDenied Malformed UnsupportedFormat IoFailure]
  ReadFailure (SourceKind ?FilesystemPath ?SourceIdentifier ReadFailureReason)
  SizeMetadata (?ByteCount ?LineCount ?ItemCount SizeCertainty)
  SizeCertainty [Exact Estimated Unknown]
  ListingOrder [OldestFirst NewestFirst ReferenceAscending]
  PageRequest (PageLimit ?FragilePageCursor ListingOrder)
  PageMetadata (PageLimit ItemCount ?ItemCount ?FragilePageCursor ListingOrder)
  CardProjection [MetadataOnly BoundedPreview]
  BoundedPreview (BoundedTextProjection)
  AuthoredStatus [AgentAuthored HumanAuthored MixedAuthorship UnknownAuthorship]
  AuthoredStatusFilter [AnyAuthoredStatus OnlyAuthoredStatus]
  OnlyAuthoredStatus (AuthoredStatus)
  OutputProvenance (SourceKind SourceIdentifier AuthoredStatus ?Timestamp)
  OutputTextExcerpt (OutputText ByteCount ?Truncation)
  SessionCard (FragileSessionReference SourceKind SourceIdentifier ?Timestamp ?Timestamp ?ItemCount ?ItemCount SizeMetadata)
  SubagentCard (FragileSubagentReference FragileSessionReference SubagentName AuthoredStatus ?ItemCount SizeMetadata ?Timestamp ?Timestamp)
  OutputCard (FragileOutputReference FragileSessionReference ?FragileSubagentReference ?OutputTitle OutputProvenance SizeMetadata ?OutputTextExcerpt)
  OutputSegmentCard (FragileOutputSegmentReference FragileOutputReference SegmentIndex ?ByteRange ?LineRange SizeMetadata ?OutputTextExcerpt)
  SessionListFilter (SourceSelection ?TimeWindow)
  SubagentListFilter (FragileSessionReference AuthoredStatusFilter)
  OutputListFilter (SourceSelection ?FragileSessionReference ?FragileSubagentReference AuthoredStatusFilter ?TimeWindow)
  OutputSegmentListFilter (FragileOutputReference)
  SessionListRequest (RequestIdentifier SessionListFilter PageRequest)
  SubagentListRequest (RequestIdentifier SubagentListFilter PageRequest)
  OutputListRequest (RequestIdentifier OutputListFilter PageRequest CardProjection)
  OutputSegmentListRequest (RequestIdentifier OutputSegmentListFilter PageRequest CardProjection)
  OutputReadRange [EntireOutput Bytes Lines Segment]
  EntireOutput
  Bytes (ByteRange)
  Lines (LineRange)
  Segment (FragileOutputSegmentReference)
  OutputEstimateRequest (RequestIdentifier FragileOutputReference OutputReadRange)
  OutputReadRequest (RequestIdentifier FragileOutputReference OutputReadRange ByteLimit)
  SessionsListed (RequestIdentifier [SessionCard] PageMetadata)
  SubagentsListed (RequestIdentifier [SubagentCard] PageMetadata)
  OutputsListed (RequestIdentifier [OutputCard] PageMetadata)
  OutputSegmentsListed (RequestIdentifier [OutputSegmentCard] PageMetadata)
  OutputEstimated (RequestIdentifier FragileOutputReference OutputReadRange SizeMetadata)
  OutputRead (RequestIdentifier FragileOutputReference OutputReadRange SizeMetadata OutputTextExcerpt)
  OperationRejectionReason [Missing FragileReferenceStale FragileReferenceBroken Oversized Unsupported Unauthorized InvalidRequest InvalidRange]
  RejectedFragileReference [Session Subagent Output OutputSegment PageCursor]
  Session (FragileSessionReference)
  Subagent (FragileSubagentReference)
  Output (FragileOutputReference)
  OutputSegment (FragileOutputSegmentReference)
  PageCursor (FragilePageCursor)
  OperationRejected (RequestIdentifier OperationKind OperationRejectionReason ?RejectedFragileReference)
  Version (?ContractName)
  VersionReport (ContractName ContractVersion)
  EvidenceRejected (RequestIdentifier OperationKind RejectionReason)
}

[
  (Version 0 2)
  (Status Scaffold)
]
"#;

struct SchemaSketchWitness {
    full_text: &'static str,
    expected_operation_heads: &'static [&'static str],
    expected_reply_heads: &'static [&'static str],
    expected_data_heads: &'static [&'static str],
}

impl SchemaSketchWitness {
    fn assert_matches_contract(self) {
        assert_eq!(
            self.full_text, EXPECTED_SCHEMA_SKETCH,
            "schema sketch drifted; update the complete manual witness with any intentional schema change"
        );
        assert_eq!(
            <AggregatorRequest as SignalOperationHeads>::HEADS,
            self.expected_operation_heads,
            "exported operation heads drifted from the schema sketch"
        );
        for head in self.expected_reply_heads {
            assert!(
                self.full_text.contains(&format!("  ({head} [")),
                "schema sketch is missing reply head {head}"
            );
        }
        for head in self.expected_data_heads {
            assert!(
                self.full_text.contains(&format!("  {head} ")),
                "schema sketch is missing data/config/evidence head {head}"
            );
        }
        assert!(
            self.full_text.ends_with("  (Status Scaffold)\n]\n"),
            "schema sketch scaffold status drifted"
        );
    }
}

#[test]
fn schema_sketch_matches_complete_manual_contract_witness() {
    SchemaSketchWitness {
        full_text: include_str!("../schema/signal.schema"),
        expected_operation_heads: &[
            "Collect",
            "Version",
            "ListSessions",
            "ListSubagents",
            "ListOutputs",
            "ListOutputSegments",
            "EstimateOutput",
            "ReadOutput",
        ],
        expected_reply_heads: &[
            "EvidenceCollected",
            "VersionReported",
            "EvidenceRejected",
            "SessionsListed",
            "SubagentsListed",
            "OutputsListed",
            "OutputSegmentsListed",
            "OutputEstimated",
            "OutputRead",
            "OperationRejected",
        ],
        expected_data_heads: &[
            "EvidenceRequest",
            "EvidencePackage",
            "SessionListRequest",
            "SubagentListRequest",
            "OutputListRequest",
            "OutputSegmentListRequest",
            "OutputEstimateRequest",
            "OutputReadRequest",
            "SessionsListed",
            "SubagentsListed",
            "OutputsListed",
            "OutputSegmentsListed",
            "OutputEstimated",
            "OutputRead",
            "OperationRejected",
            "Version",
            "EvidenceRejected",
        ],
    }
    .assert_matches_contract();
}

#[test]
fn contract_has_no_synthesis_reply() {
    let reply_text = AggregatorReply::OperationRejected(OperationRejected {
        request_identifier: request_identifier(),
        operation: AggregatorOperationKind::ReadOutput,
        reason: OperationRejectionReason::Unsupported,
        reference: Some(RejectedFragileReference::Output(output_reference())),
    })
    .to_nota();
    for forbidden in ["Summary", "Review", "Recommendation", "Score", "Judgment"] {
        assert!(!reply_text.contains(forbidden));
    }
}
