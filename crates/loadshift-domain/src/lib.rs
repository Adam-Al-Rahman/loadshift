#![forbid(unsafe_code)]

//! Domain types and invariants for industrial energy optimization.
//!
//! This crate owns the core concepts used by Loadshift:
//! energy assets, production requirements, operational constraints,
//! and scheduling concepts.
//!
//! It must not depend on HTTP, database, or solver infrastructure.
