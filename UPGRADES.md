# Upgrades

## 1.1.0 — repin to the 0.33 cohort

Schema and archive are unchanged; the minor version marks the text and API adoption.

- Pins: signal 7afb68b (10.3.0), datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0),
  build-dependency ethos-zero 5a5daca (19.2.0). One revision of each in the lock.
- Archive: five fixtures (two queries, three responses) archive to the same bytes as 1.0.0
  (d5ce226), as bare rkyv and as the Signal frame; `tests/archive_stability.rs` holds the 1.0.0 bytes.
- Rust API: `src/generated/signal.rs` is regenerated with ethos-zero 19.2.0, so every scalar,
  vector and option alias is a tuple struct. A consumer wraps a value with the constructor and
  reads it as `.0`.
- `ReaderBudget` fields are private: build one with `ReaderBudget::try_from(n).unwrap()`.
