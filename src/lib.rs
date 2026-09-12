//! Ordinary Signal contract for aggregator.
//!
//! The contract carries bounded collection requests, normalized evidence
//! packages, metadata-first output discovery, and structured transcript block
//! search and read. Synthesis and review happen in agents after the package,
//! bounded output text, or bounded transcript block text is read.
//! Agent-authored output is exposed as artifact provenance, not as design
//! authority.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, and the wire framing come from `signal`.

pub mod generated;
pub use generated::signal::*;

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
