# signal-aggregator skills

This is a contract repo. Keep daemon runtime, adapter implementation, storage,
and policy out of this crate. Public operations are contract-local verbs; SEMA
classification stays in the `aggregator` runtime.
