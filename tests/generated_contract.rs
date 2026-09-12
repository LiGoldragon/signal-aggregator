use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_aggregator::{
    AuthoredStatusFilter, CardProjection, ListingOrder, OperationKind, OperationRejected,
    OperationRejectionReason, PageRequest, Query, RejectedFragileReference, Response, SearchPhrase,
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
