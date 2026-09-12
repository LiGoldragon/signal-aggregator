# signal-aggregator — architecture

`signal-aggregator` is the ordinary public wire contract for the `aggregator`
component. It defines the peer-callable request/reply vocabulary for bounded
collection of work evidence and metadata-first discovery of session, subagent,
and output artifacts.

## Role

The contract exposes the backward-compatible collection spine plus output-facing
interfaces:

- `Collect(EvidenceRequest)` returns `EvidenceCollected(EvidencePackage)` when
  the daemon has collected and normalized the requested evidence.
- `Version(VersionQuery)` returns `VersionReported` so clients can identify the
  contract surface they are speaking.
- `ObserveHealth(RuntimeHealthRequest)` returns `RuntimeHealthObserved` with
  runtime capabilities, source health cards, and fragile-index health.
- `InventorySessions` returns metadata-only complete session inventory cards and per-source scan completeness.
- `LookupSession` resolves by fragile session reference, producer session identifier, or source locator.
- `WriteSessionArchive`, `QuerySessionArchive`, and `ReadSessionArchive` let agents store and read agent-authored session summaries in an explicit aggregator-local archive path.
- `ListSessions`, `ListSubagents`, `ListOutputs`, and `ListOutputSegments`
  return metadata cards with deterministic ordering, pagination metadata, opaque
  fragile references, size metadata, provenance, and at most bounded previews.
- `ListTranscriptBlocks` and `SearchTranscriptBlocks` expose whole logical
  transcript blocks with `TranscriptBlock` vocabulary, kind selection, bounded
  previews, and the flat transcript text query and its matching evidence.
- `EstimateOutput` returns size metadata for a referenced output range.
- `ReadOutput` is the explicit bounded output text read path.
- `EstimateTranscriptBlock` and `ReadTranscriptBlock` are whole-block estimate
  and bounded text read paths; callers do not compute byte ranges for block
  reads.

Typed rejections are caller-actionable contract outcomes, not daemon logs.
`EvidenceRejected` remains the collection rejection reply. Output-interface
operations use `OperationRejected` with reasons for missing, stale, broken,
oversized, unsupported, unauthorized, invalid, and invalid-range requests.

## Boundary

This crate owns the wire vocabulary as one Ethos Signal declaration, its
generated Rust projection, and the round-trip witnesses. It does not own the daemon, transcript adapters, repository reads,
configuration, durable state, policy decisions, or synthesis. The daemon lowers
these operations into its Signal/Nexus/SEMA runtime.

The evidence package is collection-only. It may include source volumes,
timestamps, paths or identifiers, repository changes, transcript segment
locators, bounded projected excerpts, truncation facts, and read-failure facts.
It has no review, summary, recommendation, score, or judgment field.

## Privacy and projection

Transcript text can be private. The contract makes projection explicit through
`Projection`, `SegmentProjection`, `CardProjection`, `OutputReadRange`, and
whole-block `ReadTranscriptBlock` byte bounds. Metadata-only packages and
metadata-first output and transcript-block lists are first-class. List and health cards can carry typed locators, source status, and bounded
previews, but full text content requires an explicit read request with a byte
bound. Bounded text excerpts carry truncation facts. A caller that
wants synthesis uses an agent to interpret the package, output read, or
transcript block read after collection. Session archive summaries are supplied
by agents; the contract stores and bounds them without quote/paraphrase policy.

The output references, segment references, transcript block references, session
references, subagent references, and page cursors are named `Fragile*` because
they are daemon-local opaque handles over transcript/artifact files that can
change. Stale or broken references are normal `OperationRejected` outcomes.

Agent-authored output appears only as artifact provenance and authored status;
it is not a design-authority surface.

## Code map

```text
ethos/signal.ethos          the schema authority: queries, responses, and types
build.rs                    regenerates from the ethos and refuses drift
src/generated/signal.rs     the checked-in Rust projection
src/lib.rs                  the module surface and the two source constants
tests/generated_contract.rs rkyv frame and Datom text round-trip witnesses
```

## A bare variant may not spell a declared type

Ethos Zero resolves a bare enum variant head against the type table, so a
variant whose name is also a declared type silently becomes a
payload-carrying variant. `ScanLimitKind::ReadFailures` was generated
carrying a `Vec<ReadFailure>` — a limit kind holding the failures — and
`ScanLimitKind::DiscoveredFiles` and `SourceHealthStatus::MalformedRecords`
each carried a duplicate count. The count and vector aliases are therefore
named `DiscoveredFileCount`, `MalformedRecordCount` and
`ReadFailureRecords`, distinct from every variant head in this file. Anyone
adding a variant here must check it against the type declarations.

## Recursion and the wire

Ethos declares recursive types and boxes the position that reaches back, but a
Signal root also derives rkyv, whose derive cannot close the trait bounds of a
self-reaching type: `Vec<Self>` and `Box<Self>` both overflow. The transcript
text query and its matching evidence are therefore declared as flat node arenas
— a vector of nodes plus a root index — with children named by index. This is
recorded here because it is a property of the generator, not a choice this
contract would otherwise have made.
