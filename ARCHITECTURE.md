# signal-aggregator — architecture

`signal-aggregator` is the ordinary public wire contract for the `aggregator`
component. It defines the peer-callable request/reply vocabulary for bounded
collection of work evidence.

## Role

The contract exposes two operations:

- `Collect(EvidenceRequest)` returns `EvidenceCollected(EvidencePackage)` when
  the daemon has collected and normalized the requested evidence.
- `Version(Version)` returns `VersionReported` so clients can identify the
  contract surface they are speaking.

Typed rejections are `EvidenceRejected` replies. They are caller-actionable
contract outcomes, not daemon logs.

## Boundary

This crate owns the wire vocabulary, frame types, NOTA examples, and round-trip
witnesses. It does not own the daemon, transcript adapters, repository reads,
configuration, durable state, policy decisions, or synthesis. The daemon lowers
these operations into its Signal/Nexus/SEMA runtime.

The evidence package is collection-only. It may include source volumes,
timestamps, paths or identifiers, repository changes, transcript segment
locators, bounded projected excerpts, truncation facts, and read-failure facts.
It has no review, summary, recommendation, score, or judgment field.

## Privacy and projection

Transcript text can be private. The contract makes projection explicit through
`Projection` and `SegmentProjection`. Metadata-only packages are first-class,
and bounded text excerpts carry truncation facts. A caller that wants synthesis
uses an agent to interpret the package after collection.

## Code map

```text
schema/signal.schema     authored schema sketch for the signal contract
generated/README.md      generation placeholder and exact follow-up command
src/lib.rs               Rust contract types plus `signal_channel!`
examples/canonical.nota  canonical NOTA request/reply examples
tests/channel.rs         operation, NOTA, frame, and boundary witnesses
```
