# signal-aggregator

Ordinary public Signal contract for the `aggregator` component.

The contract asks the daemon to collect and normalize recent work evidence and
to expose metadata-first session, subagent, output, and output-segment listings.
Output text is read only through explicit bounded reads using fragile daemon-local
references. The contract carries source volumes, timestamps, locators, repository
changes, transcript segments, output provenance, size metadata, truncations, and
read/rejection facts. It does not carry synthesized review or judgment.
