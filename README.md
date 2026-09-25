# signal-aggregator

Ordinary public Signal contract for the `aggregator` component.

The contract asks the daemon to collect and normalize recent work evidence and
to expose metadata-first session, subagent, output, output-segment, and
transcript-block listings/search. Output text and transcript block text are read
only through explicit bounded reads using fragile daemon-local references. The
contract carries source volumes, timestamps, locators, repository changes,
transcript segments, transcript block cards, output provenance, size metadata,
truncations, the transcript text query and its matching evidence, and
read/rejection facts. It does not carry synthesized review or judgment.

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates the Rust
from it and refuses to build when `src/generated/signal.rs` has drifted. The
portable rkyv frame, its kinds, and the four-byte length prefix come from
`signal`; the Datom text projection is the `datom` feature.

## The text query is a recursive tree

`TranscriptBlockTextQuery` is a `TextQuery` and `TranscriptBlockSearchEvidence`
is a `MatchEvidence`: each is a recursive enum whose `AllOf` and `AnyOf` carry
their children directly and whose `Not` carries its operand (the query's) or
nothing (the evidence's). Ethos Zero 13 closes rkyv's trait bounds over the
recursion, so the tree crosses the wire as it is written, with no index to
dangle.

## Range and ordering semantics

`ByteRange` is a half-open `[start, end)` interval of zero-based UTF-8 byte
offsets. `LineRange` is a half-open `[start, end)` interval of one-based line
numbers. The `end` bound is excluded; reversed ranges are `InvalidRange`
outcomes.

`OldestFirst` and `NewestFirst` use the listed card's chronology key and break
ties by fragile reference ascending. Missing timestamp keys sort after present
keys in both directions. `ReferenceAscending` ignores chronology and sorts by
fragile reference ascending.

`ReadTranscriptBlock` reads one logical transcript block by
`FragileTranscriptBlockReference` with an explicit `maximum_bytes` bound. It may
return a truncated `TranscriptTextExcerpt`; clients do not compute byte ranges
for block reads.
