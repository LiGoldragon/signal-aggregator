use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_aggregator::{
    AuthoredStatus, AuthoredStatusFilter, ByteCount, CardProjection,
    ClaudeSubagentOutputSourcesCapability, ContainsEvidence, DiscoveredFileCount, EndWordPosition,
    FilesystemPath, FragileSessionReference, FragileTranscriptBlockReference,
    HealthObservationCapability, IndexHealth, IndexOutputCount, IndexSubagentCount, IndexedRecords,
    ItemCount, LeftOccurrence, LeftTextQueryTerm, ListingOrder, MalformedRecordCount,
    MatchEvidence, MatchEvidences, NearEvidence, NearOccurrencePair, NearOccurrencePairs,
    NearTextQuery, NextPageCursor, ObservedAt, Occurrence, OccurrenceGap, Occurrences,
    OperationKind, OperationRejected, OperationRejectionReason, PageCursor, PageLimit,
    PageMetadata, PageRequest, PiSubagentOutputSourcesCapability, Query, RejectedFragileReference,
    RequestIdentifier, Response, ReturnedItems, RightOccurrence, RightTextQueryTerm,
    RuntimeCapabilities, RuntimeCapabilityStatus, RuntimeHealthObserved, ScanLimit, ScanLimitKind,
    ScanLimitReport, ScanLimits, SearchPhrase, SearchWord, SearchWords, SegmentCount, SessionCount,
    SizeCertainty, SizeMetadata, SourceHealthCard, SourceHealthCards, SourceHealthStatus,
    SourceIdentifier, SourceKind, SourceLocator, SourceSelection, StartWordPosition, TextQueries,
    TextQuery, TextQueryTerm, TotalItems, TranscriptBlockCard, TranscriptBlockCount,
    TranscriptBlockFilter, TranscriptBlockIndex, TranscriptBlockKind, TranscriptBlockKindSelection,
    TranscriptBlockProvenance, TranscriptBlockSearchEvidence, TranscriptBlockSearchMatch,
    TranscriptBlockSearchMatches, TranscriptBlockSearchRequest, TranscriptBlockTextAvailability,
    TranscriptBlockTextQuery, TranscriptBlocksSearched, TranscriptOnlyConfigurationCapability,
    UnreadableRecords, WordDistance, WordPosition,
};

fn search_request() -> TranscriptBlockSearchRequest {
    TranscriptBlockSearchRequest {
        request_identifier: RequestIdentifier(String::from("request-1")),
        transcript_block_filter: TranscriptBlockFilter {
            source_selection: SourceSelection::AllConfigured,
            fragile_session_reference_option: None,
            fragile_subagent_reference_option: None,
            task_identifier_option: None,
            transcript_block_kind_selection: TranscriptBlockKindSelection::AllTranscriptBlockKinds,
            authored_status_filter: AuthoredStatusFilter::AnyAuthoredStatus,
            time_window_option: None,
        },
        transcript_block_text_query: TranscriptBlockTextQuery(TextQuery::AllOf(TextQueries(vec![
            TextQuery::Contains(TextQueryTerm::Word(SearchWord(String::from("alpha")))),
            TextQuery::Contains(TextQueryTerm::Phrase(SearchPhrase {
                search_words: SearchWords(vec![
                    SearchWord(String::from("bounded")),
                    SearchWord(String::from("text")),
                ]),
            })),
        ]))),
        page_request: PageRequest {
            page_limit: PageLimit(25),
            page_cursor: PageCursor(None),
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
        request_identifier: RequestIdentifier(String::from("request-1")),
        operation_kind: OperationKind::SearchTranscriptBlocks,
        operation_rejection_reason: OperationRejectionReason::FragileReferenceStale,
        rejected_fragile_reference_option: Some(RejectedFragileReference::Session(
            FragileSessionReference(String::from("session-1")),
        )),
    });
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}

fn word(text: &str) -> TextQueryTerm {
    TextQueryTerm::Word(SearchWord(String::from(text)))
}

/// A query tree nested four deep, every recursive position used: `AllOf` and
/// `AnyOf` holding subtrees, and `Not` boxing a `Not`.
fn nested_text_query() -> TextQuery {
    TextQuery::AnyOf(TextQueries(vec![
        TextQuery::AllOf(TextQueries(vec![
            TextQuery::Contains(word("alpha")),
            TextQuery::Not(Box::new(TextQuery::Not(Box::new(TextQuery::AnyOf(
                TextQueries(vec![
                    TextQuery::Contains(word("beta")),
                    TextQuery::AllOf(TextQueries(vec![])),
                ]),
            ))))),
        ])),
        TextQuery::Near(NearTextQuery {
            left_text_query_term: LeftTextQueryTerm(word("gamma")),
            right_text_query_term: RightTextQueryTerm(TextQueryTerm::Phrase(SearchPhrase {
                search_words: SearchWords(vec![
                    SearchWord(String::from("delta")),
                    SearchWord(String::from("epsilon")),
                ]),
            })),
            word_distance: WordDistance(3),
        }),
        TextQuery::Not(Box::new(TextQuery::AllOf(TextQueries(vec![
            TextQuery::Contains(word("zeta")),
        ])))),
    ]))
}

/// The evidence tree that answers [`nested_text_query`]'s shape.
fn nested_match_evidence() -> MatchEvidence {
    MatchEvidence::AnyOf(MatchEvidences(vec![
        MatchEvidence::AllOf(MatchEvidences(vec![
            MatchEvidence::Contains(ContainsEvidence {
                text_query_term: word("alpha"),
                occurrences: Occurrences(vec![Occurrence {
                    start_word_position: StartWordPosition(WordPosition(4)),
                    end_word_position: EndWordPosition(WordPosition(5)),
                }]),
            }),
            MatchEvidence::Not,
        ])),
        MatchEvidence::Near(NearEvidence {
            left_text_query_term: LeftTextQueryTerm(word("gamma")),
            right_text_query_term: RightTextQueryTerm(word("delta")),
            word_distance: WordDistance(3),
            near_occurrence_pairs: NearOccurrencePairs(vec![NearOccurrencePair {
                left_occurrence: LeftOccurrence(Occurrence {
                    start_word_position: StartWordPosition(WordPosition(10)),
                    end_word_position: EndWordPosition(WordPosition(11)),
                }),
                right_occurrence: RightOccurrence(Occurrence {
                    start_word_position: StartWordPosition(WordPosition(12)),
                    end_word_position: EndWordPosition(WordPosition(14)),
                }),
                occurrence_gap: OccurrenceGap(WordDistance(1)),
            }]),
        }),
        MatchEvidence::AllOf(MatchEvidences(vec![MatchEvidence::AnyOf(MatchEvidences(
            vec![MatchEvidence::Not],
        ))])),
    ]))
}

fn nested_search_response() -> Response {
    Response::TranscriptBlocksSearched(TranscriptBlocksSearched {
        request_identifier: RequestIdentifier(String::from("request-3")),
        transcript_block_search_matches: TranscriptBlockSearchMatches(vec![
            TranscriptBlockSearchMatch {
                transcript_block_card: TranscriptBlockCard {
                    fragile_transcript_block_reference: FragileTranscriptBlockReference(
                        String::from("block-1"),
                    ),
                    fragile_session_reference: FragileSessionReference(String::from("session-1")),
                    fragile_subagent_reference_option: None,
                    subagent_task_metadata_option: None,
                    transcript_block_kind: TranscriptBlockKind::AgentResponse,
                    transcript_block_index: TranscriptBlockIndex(7),
                    transcript_block_provenance: TranscriptBlockProvenance {
                        source_kind: SourceKind::Claude,
                        source_identifier: SourceIdentifier(String::from("claude")),
                        authored_status: AuthoredStatus::AgentAuthored,
                        observed_at: ObservedAt(None),
                    },
                    line_range_option: None,
                    byte_range_option: None,
                    size_metadata: SizeMetadata {
                        byte_count_option: Some(ByteCount(120)),
                        line_count_option: None,
                        segment_count: SegmentCount(None),
                        size_certainty: SizeCertainty::Exact,
                    },
                    transcript_block_text_availability:
                        TranscriptBlockTextAvailability::ReadableText,
                    transcript_text_excerpt_option: None,
                },
                transcript_block_search_evidence: TranscriptBlockSearchEvidence(
                    nested_match_evidence(),
                ),
            },
        ]),
        page_metadata: PageMetadata {
            page_limit: PageLimit(25),
            returned_items: ReturnedItems(ItemCount(1)),
            total_items: TotalItems(Some(ItemCount(1))),
            next_page_cursor: NextPageCursor(None),
            listing_order: ListingOrder::NewestFirst,
        },
    })
}

/// The recursive query and evidence trees archive through rkyv and restore
/// equal, both through rkyv directly (which validates the archive) and
/// through the Signal frame a peer receives.
#[test]
fn a_nested_query_and_its_nested_evidence_round_trip_through_rkyv() {
    let mut request = search_request();
    request.transcript_block_text_query = TranscriptBlockTextQuery(nested_text_query());
    let query = Query::SearchTranscriptBlocks(request);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).expect("archive query");
    let restored = rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).expect("restore query");
    assert_eq!(restored, query);
    let incoming = Signal::<Query>::from(query.signalize().expect("frame").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore framed query"), query);

    let response = nested_search_response();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&response).expect("archive response");
    let restored =
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).expect("restore response");
    assert_eq!(restored, response);
    let incoming = Signal::<Response>::from(response.signalize().expect("frame").bytes().to_vec());
    assert_eq!(
        incoming.restore().expect("restore framed response"),
        response
    );
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_the_text_query_tree() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::SearchTranscriptBlocks(search_request());
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 65536,
            reader: ReaderBudget::try_from(65536).unwrap(),
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, query);
}

fn health() -> RuntimeHealthObserved {
    RuntimeHealthObserved {
        request_identifier: RequestIdentifier(String::from("request-2")),
        runtime_capabilities: RuntimeCapabilities {
            health_observation_capability: HealthObservationCapability(
                RuntimeCapabilityStatus::Supported,
            ),
            transcript_only_configuration_capability: TranscriptOnlyConfigurationCapability(
                RuntimeCapabilityStatus::Unsupported,
            ),
            claude_subagent_output_sources_capability: ClaudeSubagentOutputSourcesCapability(
                RuntimeCapabilityStatus::Supported,
            ),
            pi_subagent_output_sources_capability: PiSubagentOutputSourcesCapability(
                RuntimeCapabilityStatus::Unsupported,
            ),
        },
        source_health_cards: SourceHealthCards(vec![SourceHealthCard {
            source_kind: SourceKind::Claude,
            source_identifier: SourceIdentifier(String::from("claude")),
            source_locator: SourceLocator {
                filesystem_path: FilesystemPath(String::from("/home/li/.claude/projects")),
                root_relative_path_option: None,
            },
            source_health_status: SourceHealthStatus::MalformedRecords,
            scan_limits: ScanLimits(vec![ScanLimitReport {
                scan_limit_kind: ScanLimitKind::ReadFailures,
                scan_limit: ScanLimit(ItemCount(1024)),
                filesystem_path_option: None,
            }]),
            discovered_file_count: DiscoveredFileCount(ItemCount(12)),
            indexed_records: IndexedRecords(ItemCount(11)),
            malformed_record_count: MalformedRecordCount(ItemCount(1)),
            unreadable_records: UnreadableRecords(ItemCount(0)),
        }]),
        index_health: IndexHealth {
            source_health_status: SourceHealthStatus::ReadableIndexed,
            session_count: SessionCount(ItemCount(3)),
            index_subagent_count: IndexSubagentCount(ItemCount(4)),
            index_output_count: IndexOutputCount(ItemCount(5)),
            transcript_block_count: TranscriptBlockCount(ItemCount(6)),
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
            reader: ReaderBudget::try_from(65536).unwrap(),
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, response);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_a_nested_query_and_its_nested_evidence() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let budget = || Budget {
        remaining: 65536,
        reader: ReaderBudget::try_from(65536).unwrap(),
        depth: 0,
        maximum_depth: 256,
    };
    let mut request = search_request();
    request.transcript_block_text_query = TranscriptBlockTextQuery(nested_text_query());
    let query = Query::SearchTranscriptBlocks(request);
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(rendered)
        .actualize(&mut budget())
        .expect("actualize query");
    assert_eq!(restored, query);
    let response = nested_search_response();
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Response>::from(rendered)
        .actualize(&mut budget())
        .expect("actualize response");
    assert_eq!(restored, response);
}
