# Generated schema artifacts

The local contract repos currently keep the schema sketch and the checked-in Rust
contract side by side. There is not yet a repo-local contract-emission command
wired for `schema/signal.schema`.

TODO when schema-rust grows contract emission for signal repos:

```sh
SCHEMA_RUST_UPDATE_SIGNAL_ARTIFACTS=1 cargo check
```

The expected generated destination for this repo is `src/generated.rs`, with
`src/lib.rs` reduced to hand-written documentation, re-exports, and boundary
helpers.
