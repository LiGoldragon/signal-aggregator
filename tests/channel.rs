use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_aggregator::{
    AggregatorFrame, AggregatorFrameBody, AggregatorOperationKind, AggregatorReply,
    AggregatorRequest, ByteLimit, ContractName, ContractVersion, DurationAmount, DurationUnit,
    EvidencePackage, EvidenceRejected, EvidenceRequest, LimitPolicy, PackageIdentifier, Projection,
    RejectionReason, RelativeDuration, RequestIdentifier, SegmentLimit, SelectedSources,
    SourceKind, SourceSelection, TimeWindow, Timestamp, Version, VersionReport,
};
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
fn version_request_and_reply_round_trip_through_nota() {
    round_trip_nota(AggregatorRequest::Version(Version { client_name: None }));
    round_trip_nota(AggregatorReply::VersionReported(VersionReport {
        contract_name: ContractName::new("signal-aggregator"),
        contract_version: ContractVersion::new("0.1.0"),
    }));
}

#[test]
fn collect_request_round_trips_through_nota() {
    round_trip_nota(AggregatorRequest::Collect(evidence_request()));
}

#[test]
fn canonical_examples_match_file_order_and_boundaries() {
    let expected_examples = [
        CanonicalExample::Request(AggregatorRequest::Collect(canonical_evidence_request())),
        CanonicalExample::Request(AggregatorRequest::Version(Version { client_name: None })),
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
            contract_version: ContractVersion::new("0.1.0"),
        })),
        CanonicalExample::Reply(AggregatorReply::EvidenceRejected(EvidenceRejected {
            request_identifier: RequestIdentifier::new("req-20260705"),
            operation: AggregatorOperationKind::Collect,
            reason: RejectionReason::UnsupportedProjection,
        })),
    ];
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
        &["Collect", "Version"]
    );
    assert_eq!(
        AggregatorRequest::Collect(evidence_request()).operation_kind(),
        AggregatorOperationKind::Collect
    );
}

#[test]
fn schema_sketch_names_current_operation_and_reply_heads() {
    let schema = include_str!("../schema/signal.schema");
    for expected in [
        "(Collect [EvidenceRequest])",
        "(Version [Version])",
        "(EvidenceCollected [EvidencePackage])",
        "(VersionReported [VersionReport])",
        "(EvidenceRejected [EvidenceRejected])",
        "(Status Scaffold)",
    ] {
        assert!(
            schema.contains(expected),
            "schema sketch is missing expected contract surface {expected}"
        );
    }
}

#[test]
fn contract_has_no_synthesis_reply() {
    let reply_text = AggregatorReply::EvidenceRejected(EvidenceRejected {
        request_identifier: request_identifier(),
        operation: AggregatorOperationKind::Collect,
        reason: RejectionReason::CollectionUnavailable,
    })
    .to_nota();
    for forbidden in ["Summary", "Review", "Recommendation", "Score", "Judgment"] {
        assert!(!reply_text.contains(forbidden));
    }
}
