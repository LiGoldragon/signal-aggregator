use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_aggregator::{
    AuthoredStatusFilter, CardProjection, IndexHealth, ListingOrder, OperationKind,
    OperationRejected, OperationRejectionReason, PageRequest, Query, RejectedFragileReference,
    Response, RuntimeCapabilities, RuntimeCapabilityStatus, RuntimeHealthObserved, ScanLimitKind,
    ScanLimitReport, SearchPhrase, SourceHealthCard, SourceHealthStatus, SourceKind, SourceLocator,
    SourceSelection, TextQueryNode, TextQueryTerm, TranscriptBlockFilter,
    TranscriptBlockKindSelection, TranscriptBlockSearchRequest, TranscriptBlockTextQuery,
};

fn search_request() -> TranscriptBlockSearchRequest {
    TranscriptBlockSearchRequest {
        request_identifier: String::from("request-1"),
        transcript_block_filter: TranscriptBlockFilter {
            source_selection: SourceSelection::AllConfigured,
            fragile_session_reference_option: None,
            fragile_subagent_reference_option: None,
            task_identifier_option: None,
            transcript_block_kind_selection: TranscriptBlockKindSelection::AllTranscriptBlockKinds,
            authored_status_filter: AuthoredStatusFilter::AnyAuthoredStatus,
            time_window_option: None,
        },
        transcript_block_text_query: TranscriptBlockTextQuery {
            text_query_nodes: vec![
                TextQueryNode::Contains(TextQueryTerm::Word(String::from("alpha"))),
                TextQueryNode::Contains(TextQueryTerm::Phrase(SearchPhrase {
                    search_words: vec![String::from("bounded"), String::from("text")],
                })),
                TextQueryNode::AllOf(vec![0, 1]),
            ],
            text_query_root: 2,
        },
        page_request: PageRequest {
            page_limit: 25,
            page_cursor: None,
            listing_order: ListingOrder::NewestFirst,
        },
        card_projection: CardProjection::MetadataOnly,
    }
}

#[test]
fn aggregator_query_restores_from_fresh_peer_bytes() {
    let query = Query::SearchTranscriptBlocks(search_request());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
}

#[test]
fn aggregator_response_restores_from_fresh_peer_bytes() {
    let response = Response::OperationRejected(OperationRejected {
        request_identifier: String::from("request-1"),
        operation_kind: OperationKind::SearchTranscriptBlocks,
        operation_rejection_reason: OperationRejectionReason::FragileReferenceStale,
        rejected_fragile_reference_option: Some(RejectedFragileReference::Session(String::from(
            "session-1",
        ))),
    });
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_the_flat_text_query() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::SearchTranscriptBlocks(search_request());
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 65536,
            reader: ReaderBudget { remaining: 65536 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, query);
}

fn health() -> RuntimeHealthObserved {
    RuntimeHealthObserved {
        request_identifier: String::from("request-2"),
        runtime_capabilities: RuntimeCapabilities {
            health_observation_capability: RuntimeCapabilityStatus::Supported,
            transcript_only_configuration_capability: RuntimeCapabilityStatus::Unsupported,
            claude_subagent_output_sources_capability: RuntimeCapabilityStatus::Supported,
            pi_subagent_output_sources_capability: RuntimeCapabilityStatus::Unsupported,
        },
        source_health_cards: vec![SourceHealthCard {
            source_kind: SourceKind::Claude,
            source_identifier: String::from("claude"),
            source_locator: SourceLocator {
                filesystem_path: String::from("/home/li/.claude/projects"),
                root_relative_path_option: None,
            },
            source_health_status: SourceHealthStatus::MalformedRecords,
            scan_limits: vec![ScanLimitReport {
                scan_limit_kind: ScanLimitKind::ReadFailures,
                scan_limit: 1024,
                filesystem_path_option: None,
            }],
            discovered_file_count: 12,
            indexed_records: 11,
            malformed_record_count: 1,
            unreadable_records: 0,
        }],
        index_health: IndexHealth {
            source_health_status: SourceHealthStatus::ReadableIndexed,
            session_count: 3,
            index_subagent_count: 4,
            index_output_count: 5,
            transcript_block_count: 6,
        },
    }
}

/// A scan limit names a kind and nothing else. When a variant head spells a
/// declared type, Ethos Zero gives the variant that type as a payload, and
/// `ScanLimitKind::ReadFailures` was generated carrying the read failures
/// themselves. Building these two by their bare heads and carrying them over
/// the wire is the witness that they stay bare.
#[test]
fn a_scan_limit_kind_and_a_health_status_cross_the_wire_as_bare_heads() {
    let response = Response::RuntimeHealthObserved(health());
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_a_bare_scan_limit_kind() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let response = Response::RuntimeHealthObserved(health());
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    assert!(rendered.contains("ReadFailures"));
    let mut pending = Potential::<Response>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 65536,
            reader: ReaderBudget { remaining: 65536 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, response);
}
