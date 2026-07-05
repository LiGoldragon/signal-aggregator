use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_aggregator::{
    AggregatorFrame, AggregatorFrameBody, AggregatorOperationKind, AggregatorReply,
    AggregatorRequest, ByteLimit, ContractName, ContractVersion, DurationAmount, DurationUnit,
    EvidencePackage, EvidenceRejected, EvidenceRequest, LimitPolicy, PackageIdentifier, Projection,
    RejectionReason, RelativeDuration, RequestIdentifier, SegmentLimit, SourceSelection,
    TimeWindow, Timestamp, Version, VersionReport,
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
