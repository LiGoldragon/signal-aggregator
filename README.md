# signal-aggregator

Ordinary public Signal contract for the `aggregator` component.

The contract asks the daemon to collect and normalize recent work evidence and
to expose metadata-first session, subagent, output, and output-segment listings.
Output text is read only through explicit bounded reads using fragile daemon-local
references. The contract carries source volumes, timestamps, locators, repository
changes, transcript segments, output provenance, size metadata, truncations, and
read/rejection facts. It does not carry synthesized review or judgment.

## Range and ordering semantics

`ByteRange` is a half-open `[start, end)` interval of zero-based UTF-8 byte
offsets. `LineRange` is a half-open `[start, end)` interval of one-based line
numbers. The `end` bound is excluded; reversed ranges are `InvalidRange`
outcomes.

`OldestFirst` and `NewestFirst` use the listed card's chronology key and break
ties by fragile reference ascending. Missing timestamp keys sort after present
keys in both directions. `ReferenceAscending` ignores chronology and sorts by
fragile reference ascending.
