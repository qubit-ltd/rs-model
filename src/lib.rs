//! Domain objects migrated to the Qubit Rust platform.
//!
//! This crate begins with the shared dependencies and error boundary used by
//! the domain models migrated from Java.

pub mod error;

pub use error::{ModelError, ValidationViolation};
