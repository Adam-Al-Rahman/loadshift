#![forbid(unsafe_code)]

//! Mathematical optimization engine for Loadshift.
//!
//! Responsibilities:
//! - Build optimization models.
//! - Formulate objectives and constraints.
//! - Execute MILP models through HiGHS.
//! - Interpret solver termination and solution status.
//! - Independently verify returned schedules.
//!
//! This crate must not depend on the HTTP API or Tokio.
