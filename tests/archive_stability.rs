//! The rkyv archive of the contract is byte-identical to 1.0.0 (d5ce226).
//!
//! Each constant is the hex of a fixture archived by the 1.0.0 build, where
//! the fields were plain aliases. The newtype wrappers of this build must
//! archive to the same bytes, both bare and inside the Signal frame.

use signal::{ByteViewable, Signalizable};
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

const QUERY_SEARCH: &str = "726571756573742d31626f756e646564ff74657874ffffffff0000616c706861ffffff00000000000000000000000000000000000001d3ffffff0200000000000000000000000000000000000000000f89000000b0ffffff0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000017fffffff02000000000000000000000000000000000000000000190000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000";

const QUERY_NESTED: &str = "726571756573742d31000062657461ffffffff000000000000000000000000000000000001e4ffffff0000000000000000000000000000000000000000000002c9ffffff0200000000000000000000000000000000000000000003e4ffffff000000000000000000000000000000000000000000000000616c706861ffffff000000000000000000000000000000000003c9ffffff0000000000000000000000000000000000000000000064656c7461ffffff657073696c6f6eff00007a657461ffffffff000000000000000000000000000000000001e4ffffff010000000000000000000000000000000000000000000183ffffff02000000000000000000000000000000000000000000040067616d6d61ffffff0194ffffff02000000030000000000000003aeffffff000000000000000000000000000000000000000000000f89000000bdfeffff00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000264ffffff03000000000000000000000000000000000000000000190000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000";

const RESPONSE_REJECTED: &str = "726571756573742d3173657373696f6e2d310f89000000edffffff0f01010089000000eaffffff00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

const RESPONSE_NESTED: &str = "726571756573742d3373657373696f6e2d31040000000000000005000000000000000000616c706861ffffffe6ffffff01000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000a000000000000000b000000000000000c000000000000000e000000000000000100000000000000030000000000000000000000000000000000000000000000000000000000000000000002dcffffff010000000000000000000000000000000000000000000000000000000000014bffffff020000000000000000000000000000000000000000000000000000000000040067616d6d61ffffff0064656c7461ffffff030000000000000054ffffff010000000196ffffff010000000000000000000000000000000000000000000000000000000000626c6f636b2d31ff89000000c2feffff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001070000000000000000636c61756465ffff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001780000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002aafeffff0300000000000000000000000000000000000000000000000000000000001189000000b1fdffffe8feffff010000001900000000000000010000000000000001010000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

const RESPONSE_HEALTH: &str = "726571756573742d322f686f6d652f6c692f2e636c617564652f70726f6a6563747304000400000000000000000000000000000000636c61756465ffff99000000ccffffff00000000000000000004d3ffffff010000000c000000000000000b0000000000000001000000000000000000000000000000028900000088ffffff00010001b0ffffff0100000001030000000000000004000000000000000500000000000000060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

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

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn assert_archives_as<T>(value: &T, expected: &str)
where
    T: Signalizable
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        >,
{
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(value).expect("archive");
    assert_eq!(hex(&archive), expected);
    let frame = value.signalize().expect("frame");
    assert_eq!(hex(frame.bytes()), expected);
}

#[test]
fn a_search_query_archives_as_it_did_in_1_0_0() {
    let query = Query::SearchTranscriptBlocks(search_request());
    assert_archives_as(&query, QUERY_SEARCH);
}

#[test]
fn a_nested_query_archives_as_it_did_in_1_0_0() {
    let mut request = search_request();
    request.transcript_block_text_query = TranscriptBlockTextQuery(nested_text_query());
    assert_archives_as(&Query::SearchTranscriptBlocks(request), QUERY_NESTED);
}

#[test]
fn a_rejection_archives_as_it_did_in_1_0_0() {
    let response = Response::OperationRejected(OperationRejected {
        request_identifier: RequestIdentifier(String::from("request-1")),
        operation_kind: OperationKind::SearchTranscriptBlocks,
        operation_rejection_reason: OperationRejectionReason::FragileReferenceStale,
        rejected_fragile_reference_option: Some(RejectedFragileReference::Session(
            FragileSessionReference(String::from("session-1")),
        )),
    });
    assert_archives_as(&response, RESPONSE_REJECTED);
}

#[test]
fn a_nested_evidence_response_archives_as_it_did_in_1_0_0() {
    assert_archives_as(&nested_search_response(), RESPONSE_NESTED);
}

#[test]
fn a_health_response_archives_as_it_did_in_1_0_0() {
    let response = Response::RuntimeHealthObserved(health());
    assert_archives_as(&response, RESPONSE_HEALTH);
}
