//! Disc-over data model. Pure Rust with no Tauri dependency, so the same code
//! serves the iPhone app and the Companion, and its tests run anywhere.
//!
//! Spec: `docs/disc-over-spec.md`.

pub mod archiva;
pub mod db;
pub mod grading;
pub mod ids;
pub mod library;
pub mod pricing;
pub mod time;
